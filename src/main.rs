use eframe::egui::{self, ahash::HashMap, FontFamily, FontId, RichText, TextStyle, WidgetText};

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        default_theme: eframe::Theme::Dark,
        ..Default::default()
    };

    // Our application state:
    //let mut name = "Arthur".to_owned();V
    //let mut age = 42;
    let mut player = Player::new(170, 50); // orc fighter
    let mut boolean = false;
    eframe::run_native(
        "My App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            //egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<MyApp>::default()
        }),
    )

    //eframe::run_simple_native("My egui App", options, move |ctx, _frame| {
    //    egui::CentralPanel::default().show(ctx, |ui| {
    //        player.ui(ui);
    //        let btn_text = if boolean { "Stop Fight" } else { "Start Fight" };
    //        if ui.button(btn_text).clicked() {
    //            boolean = !boolean;
    //        }
    //        //ui.heading("My egui Application");
    //        //ui.horizontal(|ui| {
    //        //    let name_label = ui.label("Your name: ");
    //        //    ui.text_edit_singleline(&mut name)
    //        //        .labelled_by(name_label.id);
    //        //});
    //        //ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
    //        //if ui.button("Increment").clicked() {
    //        //    age += 1;
    //        //}
    //        //ui.label(format!("Hello '{name}', age {age}"));
    //    });
    //})
}
#[derive(PartialEq, Default)]
struct Player {
    health: u32,
    max_health: u32,
    damage: u32,
}
impl Player {
    pub fn new(health: u32, damage: u32) -> Self {
        Self {
            health,
            damage,
            max_health: health,
        }
    }
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Player Stats");
        let _ = egui::Grid::new("Player Stats")
            .num_columns(2)
            .show(ui, |ui| {
                let progress = self.health * 100 / self.max_health;
                let pb_text = format!("{} / {}", self.health, self.max_health);
                let pb = egui::ProgressBar::new(progress as f32)
                    .show_percentage()
                    .text(pb_text);

                ui.label("Health:");
                ui.add(pb);
                //
                //
                //
                ui.end_row();
                ui.label("Damage:");
                ui.label(self.damage.to_string());
                //ui.add(egui::DragValue::new(&mut self.damage));
                ui.end_row();
            });
    }
}
#[derive(Debug)]
struct MyApp {
    copper: f32,
    silver: f32,
    gold: f32,
    mines: HashMap<Mine, usize>,
    max_mine_size: usize,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            copper: 10.0,
            silver: 0.0,
            gold: 0.0,
            mines: Default::default(),
            max_mine_size: 10,
        }
    }
}
impl eframe::App for MyApp {
    fn raw_input_hook(&mut self, ctx: &egui::Context, raw_input: &mut egui::RawInput) {
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
        let dt = ctx.input(|x| (x.time % 2.0 / 60.0)) as f32;
        ctx.request_repaint();
        egui::TopBottomPanel::top("Money").show(ctx, |ui| {
            //ui.horizontal_centered(1)
            ui.horizontal(|ui| {
                ui.label(format!("Copper: {:.0}", self.copper));
                ui.label(format!("Silver: {:.0}", self.silver));
                ui.label(format!("Gold: {:.0}", self.gold));
            });
        });
        egui::SidePanel::left("left_panel")
            .resizable(false)
            .default_width(150.0)
            //.width_range(80.0..=200.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    //ui.heading("Left Panel");
                });
                egui::ScrollArea::vertical().show(ui, |ui| {
                    if ui.button("Copper Mine").clicked() && self.copper >= 10.0 {
                        self.mines
                            .entry(Mine::Copper)
                            .and_modify(|x| {
                                if *x < self.max_mine_size {
                                    *x += 1;
                                    self.copper -= 10.0;
                                }
                            })
                            .or_insert_with(|| {
                                self.copper -= 10.0;
                                1
                            });
                    }
                    if ui.button("Silver Mine").clicked() && self.silver >= 10.0 {
                        self.mines
                            .entry(Mine::Silver)
                            .and_modify(|x| {
                                if *x < self.max_mine_size {
                                    self.silver -= 10.0;
                                    *x += 1
                                }
                            })
                            .or_insert_with(|| {
                                self.silver -= 10.0;
                                1
                            });
                    }
                    if ui.button("Gold Mine").clicked() && self.gold >= 10.0 {
                        self.mines
                            .entry(Mine::Gold)
                            .and_modify(|x| {
                                if *x < self.max_mine_size {
                                    *x += 1;
                                    self.gold -= 10.0;
                                }
                            })
                            .or_insert_with(|| {
                                self.gold -= 10.0;
                                1
                            });
                    }
                    //lorem_ipsum(ui);
                });
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("TextLayoutDemo")
                .num_columns(2)
                .show(ui, |ui| {
                    for (mine, lvl) in &self.mines {
                        ui.label(mine.to_widget(*lvl));
                        ui.end_row();
                        let num = *lvl as f32;
                        match mine {
                            Mine::Copper => {
                                self.copper += num * dt;
                            }
                            Mine::Silver => {
                                self.silver += num * dt;
                            }
                            Mine::Gold => {
                                self.gold += num * dt;
                            }
                        }
                    }
                });
        });
        if self.copper > 100.0 {
            self.silver += 1.0;
            self.copper -= 100.0;
        }
        if self.silver > 100.0 {
            self.gold += 1.0;
            self.silver -= 100.0;
        }
    }
}
#[derive(Debug, PartialEq, Hash, Eq)]
#[repr(u8)]
enum Mine {
    Copper,
    Silver,
    Gold,
}

impl Mine {
    pub fn to_widget(&self, num: usize) -> WidgetText {
        WidgetText::RichText(RichText::new(format!("{:?} x {}lvl", self, num)))
    }
}
