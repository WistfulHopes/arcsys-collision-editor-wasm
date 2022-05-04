use arcsys::ggst::pac::GGSTPac;
use eframe::egui::{self, ComboBox};
use poll_promise::Promise;
use self::boxes::BoxesWindow;

mod open;
mod boxes;

#[derive(Default)]
pub struct MyApp {
    promise: Option<Promise<Result<GGSTPac, String>>>,
    boxes_window: BoxesWindow,
    loaded: bool,
    selected: String,
    ggst_file_list: Vec<String>,
    file_changed: bool,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.selected == "".to_string() {
            self.selected = "COL_SOL".to_string();
        }
        if self.ggst_file_list.len() == 0 {
            self.ggst_file_list.push("COL_SOL".to_string());
            self.ggst_file_list.push("COL_KYK".to_string());
            self.ggst_file_list.push("COL_MAY".to_string());
            self.ggst_file_list.push("COL_AXL".to_string());
            self.ggst_file_list.push("COL_CHP".to_string());
            self.ggst_file_list.push("COL_POT".to_string());
        }
        let promise = self.promise.get_or_insert_with(|| {
            // Begin download.
            // We download the image using `ehttp`, a library that works both in WASM and on native.
            // We use the `poll-promise` library to communicate with the UI thread.
            let ctx = ctx.clone();
            let (sender, promise) = Promise::new();
            let request = ehttp::Request::get(format!("https://wistfulhopes.neocities.org/pacs/{}.pac", self.selected));
            ehttp::fetch(request, move |response| {
                let pac = response.and_then(parse_response);
                sender.send(pac); // send the results back to the UI thread.
                ctx.request_repaint(); // wake up UI thread
            });
            self.loaded = false;
            promise
        });
        self.file_changed = false;

        egui::CentralPanel::default().show(ctx, |ui| match promise.ready() {
            None => ui.label("Loading..."),
            Some(Err(e)) => {
                ui.horizontal(|ui| {
                    ui.menu_button("Settings", |ui| {
                        let mut visuals = ui.ctx().style().visuals.clone();
                        visuals.light_dark_radio_buttons(ui);
                        ui.ctx().set_visuals(visuals);
                    });
                    ComboBox::from_label("Strive")
                    .selected_text(format!("{:?}", self.selected))
                    .width(150.0)
                    .show_ui(ui, |ui| {
                        for name in &self.ggst_file_list {
                            if ui.selectable_label(true, name)
                            .clicked()
                            {
                                self.selected = name.clone();
                                self.loaded = false;
                                self.file_changed = true;
                            };
                        }
                    });
                });
                ui.label(format!("Failed to read pac! Error: {}", e))
            },
            Some(Ok(pac)) => {
                if !self.loaded {
                    self.boxes_window.open_file(&pac);
                }
                ui.horizontal(|ui| {
                    ui.menu_button("Settings", |ui| {
                        let mut visuals = ui.ctx().style().visuals.clone();
                        visuals.light_dark_radio_buttons(ui);
                        ui.ctx().set_visuals(visuals);
                    });
                    ComboBox::from_label("Strive")
                    .selected_text(format!("{:?}", self.selected))
                    .width(150.0)
                    .show_ui(ui, |ui| {
                        for name in &self.ggst_file_list {
                            if ui.selectable_label(true, name)
                            .clicked()
                            {
                                self.selected = name.clone();
                                self.loaded = false;
                                self.file_changed = true;
                            };
                        }
                    });
                });
                self.loaded = true;
                self.boxes_window.ui(ui)
            }
        });
        if self.file_changed {
            self.promise = None;
        }
    }
}

impl MyApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        Default::default()
    }
}

fn parse_response(response: ehttp::Response) -> Result<GGSTPac, String> {
    let pac = open::open_file(response.bytes);
    pac
}