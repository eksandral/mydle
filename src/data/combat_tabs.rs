use eframe::egui::{self, Button, Id, Widget};

use crate::{prelude::Zone, ui::ViewMut};

#[derive(Debug, Default)]
pub struct CombatTabs {
    pub current_tab: Tab,
}
impl ViewMut for CombatTabs {
    fn ui_mut(&mut self, ui: &mut egui::Ui) {
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
                Tab::Zones => self.draw_zonez(ui),
                Tab::Dangeouns => self.draw_dangeouns(ui),
            }
        });
        ui.separator();
    }
}
impl CombatTabs {
    fn draw_combat(&self, ui: &mut egui::Ui) {}
    fn draw_quests(&self, ui: &mut egui::Ui) {}
    fn draw_zonez(&self, ui: &mut egui::Ui) {
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
                    let btn = egui::Button::new(btn_name).wrap(true).min_size(width_vec);
                    if btn.ui(ui).double_clicked() {}
                    if i > 0 && i % 5 == 0 {
                        ui.end_row();
                    }
                }
            });
    }
    fn draw_dangeouns(&self, ui: &mut egui::Ui) {}
}
#[derive(Debug, Default, PartialEq, Eq)]
pub enum Tab {
    Combat,
    Quests,
    #[default]
    Zones,
    Dangeouns,
}
