use std::any::Any;
use std::hash::Hash;
use std::sync::{Arc, Mutex};

use eframe::egui::{self, Color32, Rounding, Ui};
use eframe::egui::{FontFamily, FontId, TextStyle};
use futures::{SinkExt, StreamExt};
use my_idle::data::char::{CharData, PlayerData};
use my_idle::network;
use my_idle::prelude::*;
use tokio::sync::mpsc;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
const PLAYER_ID: u32 = 0;
#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let (gui_tx, gui_rx) = mpsc::unbounded_channel();
    let (ws_tx, ws_rx) = mpsc::unbounded_channel();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        default_theme: eframe::Theme::Dark,
        ..Default::default()
    };
    let gui_rx = Arc::new(Mutex::new(gui_rx));
    let ws_tx = Arc::new(Mutex::new(ws_tx));
    //
    // Spawn websocket handler
    //
    tokio::spawn(run_websocket(ws_rx, gui_tx));
    let app = IdleClientApp::new(ws_tx, gui_rx);

    eframe::run_native(
        "My App",
        options,
        Box::new(|_cc| {
            // This gives us image support:
            //egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::new(app)
        }),
    )?;
    Ok(())
}
#[derive(Debug)]
struct IdleClientApp {
    pub player: Option<PlayerData>,
    pub connect: bool,
    ws_tx: Arc<Mutex<mpsc::UnboundedSender<network::Message>>>,
    gui_rx: Arc<Mutex<mpsc::UnboundedReceiver<PlayerData>>>,
    open_loot_window: bool,
    loot_items: Vec<(String, usize)>,
}
impl IdleClientApp {
    pub fn new(
        ws_tx: Arc<Mutex<mpsc::UnboundedSender<network::Message>>>,
        gui_rx: Arc<Mutex<mpsc::UnboundedReceiver<PlayerData>>>,
    ) -> Self {
        Self {
            player: None,
            connect: false,
            ws_tx,
            gui_rx,
            open_loot_window: true,
            loot_items: vec![("100c".to_string(), 100), ("200c".to_string(), 200)],
        }
    }
    fn handle_rx(&mut self) {
        if let Ok(mut receiver) = self.gui_rx.lock() {
            while let Ok(message) = receiver.try_recv() {
                self.player = Some(message);
            }
        }
    }
}
impl eframe::App for IdleClientApp {
    fn raw_input_hook(&mut self, ctx: &egui::Context, _raw_input: &mut egui::RawInput) {
        //raw_input.time = Some(self.time.elapsed().as_secs_f64());
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
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_rx();
        ctx.request_repaint();
        if let Some(player) = self.player.as_ref() {
            egui::TopBottomPanel::top("Money").show(ctx, |ui| {
                //ui.horizontal_centered(1)
                ui.horizontal(|ui| {
                    render_health_bar(ui, &player.health);
                    render_experience_bar(ui, &player.level);
                    let mut ws_tx = self.ws_tx.lock().unwrap();
                    self.connect = show_connect_button(ui, &mut ws_tx, self.connect);
                });
            });
            egui::SidePanel::left("left_panel")
                .resizable(false)
                .default_width(150.0)
                //.width_range(80.0..=200.0)
                .show(ctx, |ui| {
                    if ui.button("Combat").clicked() {
                        log::debug!("Combat button is clicked");
                    }
                    if ui.button("Inventory").clicked() {
                        log::debug!("Inventory button is clicked");
                    }
                });
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.columns(2, |cols| {
                    show_char_ui(&mut cols[0], player);
                    if let Some(target) = player.target.as_ref() {
                        show_char_ui(&mut cols[1], target);
                    }
                });
            });
            egui::TopBottomPanel::bottom("bottom_bar").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // render_experience_bar(ui, &player.level());
                    if ui.button("Zone 1").clicked() {
                        let ws_tx = self.ws_tx.lock().unwrap();
                        let message = network::Message::EnterZone(Zone::Zone1);
                        ws_tx.send(message).unwrap();
                    }
                    if ui.button("Zone 2").clicked() {
                        let ws_tx = self.ws_tx.lock().unwrap();
                        let message = network::Message::EnterZone(Zone::Zone2);
                        ws_tx.send(message).unwrap();
                    }
                    if ui.button("Zone 3").clicked() {
                        let ws_tx = self.ws_tx.lock().unwrap();
                        let message = network::Message::EnterZone(Zone::Zone3);
                        ws_tx.send(message).unwrap();
                    }
                    if ui.button("Leave").clicked() {
                        let ws_tx = self.ws_tx.lock().unwrap();
                        let message = network::Message::LeaveZone;
                        ws_tx.send(message).unwrap();
                    }
                });
            });
        } else {
            egui::TopBottomPanel::top("Money").show(ctx, |ui| {
                //ui.horizontal_centered(1)
                ui.horizontal(|ui| {
                    let mut ws_tx = self.ws_tx.lock().unwrap();
                    self.connect = show_connect_button(ui, &mut ws_tx, self.connect);
                });
            });
        }
        egui::TopBottomPanel::bottom("command bar").show(ctx, |ui| {
            if ui.button("Loot Window").clicked() {
                self.open_loot_window = !self.open_loot_window;
            }
        });

        egui::Window::new("Loot")
            .default_width(320.0)
            .default_height(480.0)
            .open(&mut self.open_loot_window)
            .resizable([true, false])
            .show(ctx, |ui| {
                //use crate::View as _;
                let mut clicked = vec![];
                for (i, (title, value)) in self.loot_items.iter().enumerate() {
                    let btn = ui.button(title);
                    if btn.clicked() {
                        log::debug!("Collected {}", value);
                        clicked.push(i);
                    }
                }
                for i in clicked {
                    self.loot_items.remove(i);
                }
            });
    }
}
fn show_char_ui(ui: &mut Ui, data: impl CharData) {
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
fn show_grid(
    ui: &mut Ui,
    header: impl Into<egui::RichText>,
    id: impl Hash,
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
fn show_stats(ui: &mut Ui, id: impl Hash, stats: BasicStats) {
    let data = vec![
        ("STR:", stats.strength.to_string()),
        ("CON:", stats.constitution.to_string()),
        ("DEX:", stats.dexterity.to_string()),
        ("INT:", stats.intelligence.to_string()),
        ("WIS:", stats.wisdom.to_string()),
        ("MEN:", stats.mental.to_string()),
    ];
    show_grid(ui, "Basic Stats", id, data);
}
fn render_health_bar(ui: &mut Ui, health: &Health) -> egui::Response {
    let Health { value, max_value } = health;
    let progress = *value as f32 / *max_value as f32;
    let text = format!("Health {} / {}", value, max_value);
    let color = Color32::from_rgb(0, 127, 0);
    let pb = render_progress_bar(progress, color, text);
    ui.add(pb)
}
fn render_experience_bar(ui: &mut Ui, level: &Level) -> egui::Response {
    let progress = level.progress();
    let text = format!("Level {} ({:.2}%)", level.value, progress * 100.0);
    let color = Color32::from_rgb(128, 128, 128);
    let pb = render_progress_bar(progress, color, text);
    ui.add(pb)
}
fn render_progress_bar(
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
async fn run_websocket(
    mut ws_rx: mpsc::UnboundedReceiver<network::Message>,
    gui_tx: mpsc::UnboundedSender<PlayerData>,
) -> anyhow::Result<()> {
    if let Ok((ws_stream, _)) = connect_async("ws://127.0.0.1:9031").await {
        let (mut write, mut read) = ws_stream.split();
        let gui_tx_clonned = gui_tx.clone();
        //
        // Spawn Websocket Reader
        //
        tokio::spawn(async move {
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Binary(data)) => {
                        if let Ok(message) = rmp_serde::from_slice::<network::Message>(&data) {
                            match message {
                                network::Message::PlayerData(pd) => {
                                    gui_tx_clonned.send(pd).unwrap();
                                }
                                _ => {
                                    log::error!("unsuported message for GUI");
                                }
                            }
                        }
                    }
                    Ok(Message::Ping(src)) => {
                        let mut buf = [0u8; 8];
                        buf.copy_from_slice(&src[..]);
                        let server_time = u64::from_be_bytes(buf);
                        //println!("Server time PING: {}", server_time);

                        //let _ = write.send(Message::Pong(src));
                    }
                    _ => {}
                }
            }
        });
        while let Some(message) = ws_rx.recv().await {
            let b = rmp_serde::to_vec(&message).unwrap();
            let _ = write.send(Message::Binary(b)).await;
        }
    }
    Ok(())
}
fn show_connect_button(
    ui: &mut Ui,
    ws_tx: &mut mpsc::UnboundedSender<network::Message>,
    connected: bool,
) -> bool {
    let mut connected = connected;
    let btn_txt = if connected { "Disconnect" } else { "Connect" };
    if ui.button(btn_txt).clicked() {
        connected = !connected;
        if connected {
            let _ = ws_tx.send(network::Message::Connect(PLAYER_ID));
        } else {
            let _ = ws_tx.send(network::Message::Disconnect(PLAYER_ID));
        }
    }
    connected
}
