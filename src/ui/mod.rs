pub mod app;
pub mod character;

use eframe::egui::{self, Color32, Rounding};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::{
    data::char::Loot,
    network::Message as ServerMessage,
    prelude::{Health, Level},
};

pub trait View {
    fn ui(&self, ui: &mut egui::Ui);
}
pub trait ViewMut {
    fn ui_mut(&mut self, ui: &mut egui::Ui);
}
pub async fn run_ui_app(
    sender: UnboundedSender<ServerMessage>,
    receiver: UnboundedReceiver<ServerMessage>,
) -> eframe::Result<()> {
    let mut app = app::App::new(sender, receiver);

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Confirm exit",
        options,
        Box::new(|_cc| {
            app.connect().unwrap();
            Box::new(app)
        }),
    )
}
impl View for Level {
    fn ui(&self, ui: &mut egui::Ui) {
        let progress = self.progress();
        let text = format!("Level {} ({:.2}%)", self.value, progress * 100.0);
        let color = Color32::from_rgb(128, 128, 128);
        let pb = render_progress_bar(progress, color, text);
        ui.add(pb);
    }
}
impl View for Health {
    fn ui(&self, ui: &mut egui::Ui) {
        let Health { value, max_value } = self;
        let progress = *value as f32 / *max_value as f32;
        let text = format!("Health {} / {}", value, max_value);
        let color = Color32::from_rgb(0, 127, 0);
        let pb = render_progress_bar(progress, color, text);

        ui.add(pb);
    }
}
pub fn render_progress_bar(
    progress: f32,
    color: Color32,
    text: impl Into<egui::WidgetText>,
) -> egui::ProgressBar {
    egui::ProgressBar::new(progress)
        .show_percentage()
        .desired_width(200.0)
        .fill(color)
        .rounding(Rounding::from(2f32))
        .text(text)
}

impl View for Loot {
    fn ui(&self, ui: &mut egui::Ui) {
        egui::Grid::new("Character Inventory")
            .num_columns(2)
            .show(ui, |ui| {
                for item in self.items.iter() {
                    ui.label(item.name.clone());
                    ui.label(item.weight.to_string());
                    ui.end_row();
                }
            });
        ui.horizontal(|ui| {
            ui.label(format!("Total Weight: {}", self.total_weight()));
        });
    }
}
pub fn show_grid(
    ui: &mut egui::Ui,
    header: impl Into<egui::RichText>,
    id: impl std::hash::Hash,
    data: Vec<(impl Into<egui::WidgetText>, impl Into<egui::WidgetText>)>,
) {
    ui.label(header.into().size(18.0));
    egui::Grid::new(id).num_columns(2).show(ui, |ui| {
        for (label, value) in data.into_iter() {
            ui.label(label);
            ui.label(value);
            ui.end_row();
        }
    });
}
pub fn render_health_bar(ui: &mut egui::Ui, health: &Health) -> egui::Response {
    let Health { value, max_value } = health;
    let progress = *value as f32 / *max_value as f32;
    let text = format!("Health {} / {}", value, max_value);
    let color = Color32::from_rgb(0, 127, 0);
    let pb = render_progress_bar(progress, color, text);
    ui.add(pb)
}
pub fn render_experience_bar(ui: &mut egui::Ui, level: &Level) -> egui::Response {
    let progress = level.progress();
    let text = format!("Level {} ({:.2}%)", level.value, progress * 100.0);
    let color = Color32::from_rgb(128, 128, 128);
    let pb = render_progress_bar(progress, color, text);
    ui.add(pb)
}
