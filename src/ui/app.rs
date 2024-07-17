use std::{
    any,
    sync::{Arc, Mutex},
};

use chrono::{DateTime, Utc};
use eframe::egui::{self, style::Spacing, FontFamily, FontId, TextStyle, Widget};
use egui_extras::{Column, TableBuilder};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::{
    data::{
        char::{Loot, PlayerData},
        combat_tabs::{CombatTabs, Tab as CombatTab},
    },
    //server::ServerMessage,
    network::Message as ServerMessage,
    prelude::{Equipment, Invenotry, Item as InvItem},
};

use super::{character::show_char_ui, View, ViewMut};

pub struct App {
    state: State,
    receiver: UnboundedReceiver<ServerMessage>,
    combat_tabs: CombatTabs,
    sender: Arc<Mutex<UnboundedSender<ServerMessage>>>,
}
#[derive(Debug, Default)]
pub struct State {
    dt: DateTime<Utc>,
    player: PlayerData,
    equipment: Equipment,
    intentory: Invenotry,
    loot: Loot,
    loot_opened: bool,
    current_tab: Tab,
    cols_num: usize,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum Tab {
    #[default]
    Combat,
    Character,
    Skills,
}
impl State {
    pub fn set_dt(&mut self, dt: u64) {
        self.dt = DateTime::<Utc>::from_timestamp_millis(dt as i64).unwrap();
    }
}
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.request_repaint();
        self.handle_keyboard(ctx, frame);
        self.receive_messages();
        self.draw_top_bar(ctx, frame);
        self.draw_left_bar(ctx, frame);
        self.draw_footer(ctx, frame);
        self.draw_center(ctx, frame);
        if self.state.loot_opened {
            let window = egui::Window::new("Inventory")
                .default_width(320.0)
                .default_height(480.0)
                .open(&mut self.state.loot_opened)
                .resizable([true, true]);
            window.show(ctx, |ui| {
                self.state.loot.ui(ui);
            });
        }
    }
    fn raw_input_hook(&mut self, ctx: &egui::Context, _raw_input: &mut egui::RawInput) {
        use FontFamily::{Monospace, Proportional};

        let mut style = (*ctx.style()).clone();
        style.text_styles = [
            (TextStyle::Heading, FontId::new(25.0, Proportional)),
            //(heading2(), FontId::new(22.0, Proportional)),
            //(heading3(), FontId::new(19.0, Proportional)),
            (TextStyle::Body, FontId::new(16.0, Proportional)),
            (TextStyle::Monospace, FontId::new(12.0, Monospace)),
            (TextStyle::Button, FontId::new(16.0, Proportional)),
            (TextStyle::Small, FontId::new(8.0, Proportional)),
        ]
        .into();
        ctx.set_style(style);
    }
}
impl App {
    pub fn new(
        sender: UnboundedSender<ServerMessage>,
        receiver: UnboundedReceiver<ServerMessage>,
    ) -> Self {
        let sender = Arc::new(Mutex::new(sender));
        let mut state = State::default();
        state.cols_num = 2;
        let sender_clone = sender.clone();
        let combat_tabs = CombatTabs {
            current_tab: Default::default(),
            current_zone: None,
            on_change: None,
            on_zone_change: Some(Box::new(move |zone| {
                log::debug!("Sending data from zone change callback: {:?}", zone);
                //let data = format!("Zone is changed to  {:?}", zone);
                sender_clone
                    .lock()
                    .unwrap()
                    .send(ServerMessage::EnterZone(zone))
                    .unwrap();
            })),
        };
        Self {
            receiver,
            sender,
            state,
            combat_tabs,
        }
    }
    pub fn connect(&mut self) -> anyhow::Result<()> {
        self.sender
            .lock()
            .unwrap()
            .send(ServerMessage::Connect(0))?;
        Ok(())
    }
    pub fn handle_keyboard(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        #[cfg(not(target_arch = "wasm32"))]
        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape)) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }

    fn receive_messages(&mut self) {
        while let Ok(message) = self.receiver.try_recv() {
            match message {
                ServerMessage::SystemTime(dt) => self.state.set_dt(dt),
                ServerMessage::PlayerData(data) => {
                    self.state.player = data;
                }
                _ => (),
            }
        }
    }
    fn draw_left_bar(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("left_panel")
            .resizable(false)
            .default_width(150.0)
            //.width_range(80.0..=200.0)
            .show(ctx, |ui| {
                let available_width = ui.available_width();
                let width_vec = (available_width, 0f32).into();
                let selected = self.state.current_tab == Tab::Combat;
                let btn_combat = egui::Button::new("Combat")
                    .wrap(true)
                    .min_size(width_vec)
                    .selected(selected);

                if btn_combat.ui(ui).clicked() {
                    log::debug!("{:?} button is clicked", Tab::Combat);
                    self.state.current_tab = Tab::Combat;
                }
                let selected = self.state.current_tab == Tab::Character;
                let btn_char = egui::Button::new("Character")
                    .wrap(true)
                    .min_size(width_vec)
                    .selected(selected);
                if btn_char.ui(ui).clicked() {
                    log::debug!("{:?} button is clicked", Tab::Character);
                    self.state.current_tab = Tab::Character;
                }
                let selected = self.state.current_tab == Tab::Skills;
                let btn_skills = egui::Button::new("Skills")
                    .wrap(true)
                    .min_size(width_vec)
                    .selected(selected);
                if btn_skills.ui(ui).clicked() {
                    log::debug!("{:?} button is clicked", Tab::Skills);
                    self.state.current_tab = Tab::Skills;
                }
            });
    }
    fn draw_center(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.state.current_tab {
                Tab::Combat => self.draw_combat_tab(ui),
                Tab::Character => self.draw_character_tab(ui),
                Tab::Skills => self.draw_skill_tab(ui),
            };
        });
    }

    fn draw_footer(&self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::bottom("footer").show(ctx, |ui| {
            ui.label(self.state.dt.format("%Y-%m-%d %H:%M:%S").to_string());
        });
    }
    fn draw_top_bar(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.horizontal(|ui| {
                self.state.player.health.ui(ui);
                self.state.player.level.ui(ui);
            });
        });
    }
    fn draw_combat_tab(&mut self, ui: &mut egui::Ui) {
        let Spacing {
            item_spacing,
            indent,
            ..
        } = ui.spacing();
        let min_height = ui.available_height() / 2.0 - item_spacing.y * 2.0 - indent;
        ui.vertical_centered(|ui| {
            ui.set_height(min_height);
            let player = &self.state.player;
            egui::CentralPanel::default().show_inside(ui, |ui| {
                ui.columns(2, |cols| {
                    show_char_ui(&mut cols[0], player);
                    if let Some(target) = player.target.as_ref() {
                        show_char_ui(&mut cols[1], target);
                    }
                });
            });
        });
        if let Some(_) = self.state.player.target {
            ui.vertical_centered(|ui| {
                if ui.button("Leave").clicked() {
                    if let Err(e) = self.sender.lock().unwrap().send(ServerMessage::LeaveZone) {
                        log::error!("{}", e);
                    }
                    self.combat_tabs.current_zone = None;
                }
            });
        }
        ui.separator();
        ui.vertical_centered(|ui| {
            ui.set_height(min_height);
            //ui.set_height(min_height);
            self.combat_tabs.ui_mut(ui);
        });
    }
    fn draw_character_tab(&mut self, ui: &mut egui::Ui) {
        ui.columns(2, |cols| {
            self.draw_inventory(&mut cols[0]);
            self.draw_equipment(&mut cols[1]);
        });
    }
    fn draw_skill_tab(&self, ui: &mut egui::Ui) {
        ui.heading("Skill tab");
    }
    fn draw_equipment(&mut self, ui: &mut egui::Ui) {
        ui.heading("Equipment");
        let table = TableBuilder::new(ui)
            .striped(true)
            //.resizable(self.resizable)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())
            .column(Column::remainder())
            .min_scrolled_height(0.0);
        table.body(|mut body| {
            let height = 30.0;
            let equipment = &mut self.state.equipment;
            body.row(height, |mut row| {
                row.col(|ui| {
                    ui.label("Helmet");
                });
                row.col(|ui| match equipment.helmet.as_ref() {
                    Some(h) => {
                        if ui.button(h.name.to_string()).double_clicked() {
                            equipment.helmet = None;
                            //self.move_equipment_to_inventory(h);
                        }
                    }
                    None => {
                        ui.label("<Empty>");
                    }
                });
            });
            body.row(height, |mut row| {
                row.col(|ui| {
                    ui.label("Upper Body");
                });
                row.col(|ui| match equipment.upper_body.as_ref() {
                    Some(h) => {
                        if ui.button(h.name.to_string()).double_clicked() {
                            equipment.upper_body = None;
                        }
                    }
                    None => {
                        ui.label("<Empty>");
                    }
                });
            });
            body.row(height, |mut row| {
                row.col(|ui| {
                    ui.label("Lower Body");
                });
                row.col(|ui| match equipment.lower_body.as_ref() {
                    Some(h) => {
                        if ui.button(h.name.to_string()).double_clicked() {
                            equipment.lower_body = None;
                        }
                    }
                    None => {
                        ui.label("<Empty>");
                    }
                });
            });
            body.row(height, |mut row| {
                row.col(|ui| {
                    ui.label("Gloves");
                });
                row.col(|ui| match equipment.gloves.as_ref() {
                    Some(h) => {
                        if ui.button(h.name.to_string()).double_clicked() {
                            equipment.gloves = None;
                        }
                    }
                    None => {
                        ui.label("<Empty>");
                    }
                });
            });
            body.row(height, |mut row| {
                row.col(|ui| {
                    ui.label("Boots");
                });
                row.col(|ui| match equipment.boots.as_ref() {
                    Some(h) => {
                        if ui.button(h.name.to_string()).double_clicked() {
                            equipment.boots = None;
                        }
                    }
                    None => {
                        ui.label("<Empty>");
                    }
                });
            });
            body.row(height, |mut row| {
                row.col(|ui| {
                    ui.label("Left Hand");
                });
                row.col(|ui| match equipment.left_hand.as_ref() {
                    Some(h) => {
                        if ui.button(h.name.to_string()).double_clicked() {
                            equipment.left_hand = None;
                            if let Ok(sender) = self.sender.lock() {
                                sender
                                    .send(ServerMessage::RemoveWeapon { left_hand: true })
                                    .unwrap();
                            }
                        }
                    }
                    None => {
                        ui.label("<Empty>");
                    }
                });
            });
            body.row(height, |mut row| {
                row.col(|ui| {
                    ui.label("Right Hand");
                });
                row.col(|ui| match equipment.right_hand.as_ref() {
                    Some(h) => {
                        if ui.button(h.name.to_string()).double_clicked() {
                            equipment.right_hand = None;
                        }
                    }
                    None => {
                        ui.label("<Empty>");
                    }
                });
            });
        });
    }

    fn draw_inventory(&mut self, ui: &mut egui::Ui) {
        ui.heading("Inventory");
        egui::TopBottomPanel::bottom("inventory summury").show_inside(ui, |ui| {
            ui.add_space(10.0);
            let max_size = self.state.intentory.max_size;
            let cur_size = self.state.intentory.curent_size();
            ui.horizontal(|ui| {
                ui.label(format!("{} / {}", cur_size, max_size));
                ui.add(egui::Slider::new(&mut self.state.cols_num, 1..=4));
            });
        });
        let cols_num = self.state.cols_num;
        egui::ScrollArea::vertical()
            .id_source("intentory")
            .show(ui, |ui| {
                ui.columns(cols_num, |cols| {
                    for (i, item) in self.state.intentory.items.iter().enumerate() {
                        let item_name = match item {
                            InvItem::Shit => "shit".to_string(),
                            InvItem::Helmet(i) => i.name.to_string(),
                            InvItem::UpperBody(i) => i.name.to_owned(),
                            InvItem::LowerBody(i) => i.name.to_owned(),
                            InvItem::Gloves(i) => i.name.to_owned(),
                            InvItem::Boots(i) => i.name.to_owned(),
                            InvItem::Weapon(i) => i.name.to_owned(),
                        };
                        let ui = &mut cols[i % cols_num];
                        let available_width = ui.available_width();
                        let width_vec = (available_width, 0f32).into();
                        let btn = egui::Button::new(item_name).wrap(true).min_size(width_vec);
                        if btn.ui(ui).double_clicked() {
                            match item {
                                InvItem::Shit => log::debug!("shit"),
                                InvItem::Helmet(i) => {
                                    self.state.equipment.helmet = Some(i.clone());
                                }
                                InvItem::UpperBody(i) => {
                                    self.state.equipment.upper_body = Some(i.clone());
                                }
                                InvItem::LowerBody(i) => {
                                    self.state.equipment.lower_body = Some(i.clone());
                                }
                                InvItem::Gloves(i) => {
                                    self.state.equipment.gloves = Some(i.clone());
                                }
                                InvItem::Boots(i) => {
                                    self.state.equipment.boots = Some(i.clone());
                                }
                                InvItem::Weapon(i) => {
                                    self.state.equipment.left_hand = Some(i.clone());
                                    if let Ok(sender) = self.sender.lock() {
                                        sender
                                            .send(ServerMessage::UseWeapon {
                                                left_hand: true,
                                                weapon: i.clone(),
                                            })
                                            .unwrap();
                                    }
                                }
                            };
                        }
                    }
                });
            });
    }
}
