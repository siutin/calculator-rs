/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    #[serde(skip)] // This how you opt-out of serialization of a field
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    num_cols: usize,

    #[serde(skip)] // This how you opt-out of serialization of a field
    num_rows: usize,

    #[serde(skip)] // This how you opt-out of serialization of a field
    min_col_width: f32,

    #[serde(skip)] // This how you opt-out of serialization of a field
    max_col_width: f32,

    #[serde(skip)] // This how you opt-out of serialization of a field
    accumulator: i64,

    #[serde(skip)] // This how you opt-out of serialization of a field
    input: i64,

    #[serde(skip)] // This how you opt-out of serialization of a field
    acc_op: Option<Operation>,

    #[serde(skip)] // This how you opt-out of serialization of a field
    display: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    reason: Option<&'static str>
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello calculator".to_owned(),

            num_cols: 4,
            num_rows: 4,
            min_col_width: 50.0,
            max_col_width: 200.0,

            accumulator: 0,
            input: 0,
            acc_op: None,
            display: "0".to_owned(),
            reason: None
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {

            ctx.set_pixels_per_point(3.0);

            ui.heading(&mut self.label);

            ui.add(egui::TextEdit::singleline(&mut format!("{}", self.input)).hint_text("input"));
            ui.add(egui::TextEdit::singleline(&mut format!("{}", self.accumulator)).hint_text("accumulator"));
            ui.add(egui::TextEdit::singleline(&mut self.display).hint_text("display"));
            ui.colored_label(egui::Color32::RED,  self.reason.unwrap_or(""));

            ui.horizontal(|ui| {
              if ui.add( egui::Button::new("<").min_size(egui::Vec2 { x: self.min_col_width, y: 20.0 }) ).clicked() {
                self.input = self.input / 10;
                self.display = format!("{}", self.input);
                self.reason = None;
              }
              if ui.add( egui::Button::new("AC").min_size(egui::Vec2 { x: self.min_col_width, y: 20.0 }) ).clicked() {
                self.input = 0;
                self.accumulator = 0;
                self.acc_op = None;
                self.display = "0".to_owned();
                self.reason = None;
              }
            });

            egui::Grid::new("num_grid")
            .striped(true)
            .min_col_width(self.min_col_width)
            .max_col_width(self.max_col_width)
            .show(ui, |ui| {
                for row in 0..self.num_rows {
                    for col in 0..self.num_cols {

                        if row < 3 && col < 3 {
                          let num = row * (self.num_cols - 1) + col + 1;
                          if ui.add( egui::Button::new(num.to_string()).min_size(egui::Vec2 { x: self.min_col_width, y: 20.0 })  ).clicked() {
                            self.input = self.input * 10 + num as i64;
                            self.reason = None;
                            self.display = format!("{}", self.input);
                          }
                        } else if row == 0 {
                          if ui.add( egui::Button::new("+").min_size(egui::Vec2 { x: self.min_col_width, y: 20.0 }) ).clicked() {
                            match handle_operation(self.acc_op, self.accumulator, self.input) {
                              Ok(Some(value)) => {
                                self.accumulator = value;
                                self.input = 0;
                                self.display = format!("{}", self.accumulator);
                                self.reason = None;
                              },
                              Err(reason) => self.reason = Some(reason),
                              _ => {},
                            }
                            self.acc_op = Some(Operation::Add);
                          }
                        } else if row == 1 {
                          if ui.add( egui::Button::new("-").min_size(egui::Vec2 { x: self.min_col_width, y: 20.0 }) ).clicked() {
                            match handle_operation(self.acc_op, self.accumulator, self.input) {
                              Ok(Some(value)) => {
                                self.accumulator = value;
                                self.input = 0;
                                self.display = format!("{}", self.accumulator);
                                self.reason = None;
                              },
                              Err(reason) => self.reason = Some(reason),
                              _ => {},
                            }
                            self.acc_op = Some(Operation::Subtract);
                          }
                        } else if row == 2 {
                          if ui.add( egui::Button::new("*").min_size(egui::Vec2 { x: self.min_col_width, y: 20.0 }) ).clicked() {
                            match handle_operation(self.acc_op, self.accumulator, self.input) {
                              Ok(Some(value)) => {
                                self.accumulator = value;
                                self.input = 0;
                                self.display = format!("{}", self.accumulator);
                                self.reason = None;
                              },
                              Err(reason) => self.reason = Some(reason),
                              _ => {},
                            }
                            self.acc_op = Some(Operation::Multiply);
                          }
                        } else if row == 3 && col == 3 {
                          if ui.add( egui::Button::new("0").min_size(egui::Vec2 { x: self.min_col_width, y: 20.0 }) ).clicked() {
                            self.input = self.input * 10;
                            self.display = format!("{}", self.input);
                            self.reason = None;
                          }
                          ui.add( egui::Button::new(".").min_size(egui::Vec2 { x: self.min_col_width, y: 20.0 }) );

                          if ui.add( egui::Button::new("=").min_size(egui::Vec2 { x: self.min_col_width, y: 20.0 }) ).clicked() {
                            match handle_operation(self.acc_op, self.accumulator, self.input) {
                              Ok(Some(value)) => {
                                self.accumulator = value;
                                self.input = 0;
                                self.display = format!("{}", self.accumulator);
                                self.reason = None;
                              },
                              Err(reason) => self.reason = Some(reason),
                              _ => {},
                            }
                            self.acc_op = None;
                            self.input = 0;
                          }
                          if ui.add( egui::Button::new("/").min_size(egui::Vec2 { x: self.min_col_width, y: 20.0 }) ).clicked() {
                            match handle_operation(self.acc_op, self.accumulator, self.input) {
                              Ok(Some(value)) => {
                                self.accumulator = value;
                                self.input = 0;
                                self.display = format!("{}", self.accumulator);
                                self.reason = None;
                              },
                              Err(reason) => self.reason = Some(reason),
                              _ => {},
                            }
                            self.acc_op = Some(Operation::Divide);
                          }
                        }

                    }

                    ui.end_row();
                }
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                // powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide
}

fn handle_operation (acc_op: Option<Operation>, accumulator: i64, input: i64) -> Result<Option<i64>, &'static str> {
  match acc_op {
    Some(Operation::Add) => Ok(Some(accumulator + input)),
    Some(Operation::Subtract) => Ok(Some(accumulator - input)),
    Some(Operation::Multiply) => Ok(Some(accumulator * input)),
    Some(Operation::Divide) => {
      if input == 0 {
        return Err("divide by zero")
      }
      Ok(Some(accumulator / input))
    },
    None => {
      if accumulator == 0 {
        return Ok(Some(input))
      } 
      Ok(None)
    }
  }
}