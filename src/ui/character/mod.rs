use eframe::egui::{self, Color32, Ui};

use crate::{data::char::CharData, prelude::BasicStats};

use super::{render_health_bar, render_progress_bar, show_grid};

pub fn show_char_ui(ui: &mut Ui, data: impl CharData) {
    ui.heading(data.name());
    ui.columns(2, |cols| {
        let combat_data = vec![
            ("P.Atk", data.combat().p_attack.to_string()),
            ("P.Def", data.combat().p_defense.to_string()),
        ];
        show_grid(
            &mut cols[0],
            "Combat Stats",
            format!("combat_stats_{}", data.id()),
            combat_data,
        );
        show_stats(
            &mut cols[0],
            format!("basic_stats_{}", data.id()),
            data.stats(),
        );

        egui::Grid::new(format!("CharData_{}", data.id()))
            .num_columns(2)
            .show(&mut cols[1], |ui| {
                ui.end_row();
                //
                // Health
                //
                render_health_bar(ui, &data.health());
                ui.end_row();
                //
                // Attack
                //
                let attack = data.attack();
                if attack.timer.running {
                    let progress = attack.timer.progress();
                    let text = format!("Next Attack in {:.2}s", attack.timer.remains(),);
                    let color = Color32::from_rgb(147, 130, 0);
                    let pb = render_progress_bar(1.0 - progress, color, text);

                    ui.add(pb);
                }
            });
    });
}
fn show_stats(ui: &mut Ui, id: impl std::hash::Hash, stats: BasicStats) {
    let data = vec![
        ("STR:", stats.strength.to_string()),
        ("CON:", stats.constitution.to_string()),
        ("DEX:", stats.dexterity.to_string()),
        ("INT:", stats.intelligence.to_string()),
        ("WIT:", stats.wisdom.to_string()),
        ("MEN:", stats.mental.to_string()),
    ];
    show_grid(ui, "Basic Stats", id, data);
}
