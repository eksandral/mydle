use eframe::egui::{self, Widget};

use crate::{prelude::Zone, server::ServerMessage, ui::ViewMut};

pub struct CombatTabs {
    pub current_tab: Tab,
    pub current_zone: Option<Zone>,
    pub on_change: Option<fn(old: Tab, new: Tab)>,
    pub on_zone_change: Option<Box<dyn FnMut(Zone) + 'static>>,
}
impl ViewMut for CombatTabs {
    fn ui_mut(&mut self, ui: &mut egui::Ui) {
        let old_tab = self.current_tab.clone();

        ui.vertical(|ui| {
            //ui.set_height(2.9);
            ui.columns(4, |cols| {
                let btn = egui::Button::new("Combat Controll")
                    .wrap(true)
                    .selected(self.current_tab == Tab::Combat);
                if btn.ui(&mut cols[0]).clicked() {
                    self.current_tab = Tab::Combat;
                };
                let btn = egui::Button::new("Quests")
                    .wrap(true)
                    .selected(self.current_tab == Tab::Quests);
                if btn.ui(&mut cols[1]).clicked() {
                    self.current_tab = Tab::Quests;
                };

                let btn = egui::Button::new("Zones")
                    .wrap(true)
                    .selected(self.current_tab == Tab::Zones);
                if btn.ui(&mut cols[2]).clicked() {
                    self.current_tab = Tab::Zones;
                };
                let btn = egui::Button::new("Dangeons")
                    .wrap(true)
                    .selected(self.current_tab == Tab::Dangeouns);
                if btn.ui(&mut cols[3]).clicked() {
                    self.current_tab = Tab::Dangeouns;
                };
            });
            ui.separator();
            match self.current_tab {
                Tab::Combat => self.draw_combat(ui),
                Tab::Quests => self.draw_quests(ui),
                Tab::Zones => self.draw_zones(ui),
                Tab::Dangeouns => self.draw_dangeouns(ui),
            }
        });
        ui.separator();
        if old_tab != self.current_tab {
            if let Some(handler) = self.on_change {
                handler(old_tab, self.current_tab.clone());
            }
        }
    }
}
impl CombatTabs {
    fn draw_combat(&self, ui: &mut egui::Ui) {}
    fn draw_quests(&self, ui: &mut egui::Ui) {}
    fn draw_zones(&mut self, ui: &mut egui::Ui) {
        let old_zone = self.current_zone.clone();
        egui::Grid::new("zones grid")
            .num_columns(5)
            .spacing([10.0, 10.0])
            .striped(true)
            .show(ui, |ui| {
                for (i, zone) in Zone::VALUES.iter().enumerate() {
                    let zone_range = zone.mosnter_level_range();
                    let btn_name =
                        format!("{:?}\n{} - {} lvl", zone, zone_range.start, zone_range.end);
                    let width_vec = (120.0, 80f32).into();
                    let selected = self
                        .current_zone
                        .as_ref()
                        .map(|x| x == zone)
                        .unwrap_or(false);
                    let btn = egui::Button::new(btn_name)
                        .wrap(true)
                        .min_size(width_vec)
                        .selected(selected);
                    if btn.ui(ui).double_clicked() && !selected {
                        self.current_zone = Some(zone.clone());
                        if let Some(handler) = &mut self.on_zone_change {
                            handler(zone.clone());
                        }
                    }
                    if i > 0 && i % 5 == 0 {
                        ui.end_row();
                    }
                }
            });
    }
    fn draw_dangeouns(&self, ui: &mut egui::Ui) {}
}
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub enum Tab {
    Combat,
    Quests,
    #[default]
    Zones,
    Dangeouns,
}
