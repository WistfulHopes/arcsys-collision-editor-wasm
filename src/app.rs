use arcsys::ggst::pac::GGSTPac;
use eframe::egui::{self, ComboBox};
use poll_promise::Promise;
use self::boxes::BoxesWindow;
use bbscript::{command_db::{GameDB}, error::BBScriptError, run_parser};

mod open;
mod boxes;

#[derive(Default)]
pub struct MyApp {
    col_promise: Option<Promise<Result<GGSTPac, String>>>,
    char_promise: Option<Promise<Vec<u8>>>,
    ef_promise: Option<Promise<Vec<u8>>>,
    ron_promise: Option<Promise<Result<GameDB, BBScriptError>>>,
    boxes_window: BoxesWindow,
    loaded: bool,
    selected: String,
    ggst_file_list: Vec<String>,
    file_changed: bool,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.selected == "".to_string() {
            self.selected = "SOL".to_string();
        }
        if self.ggst_file_list.len() == 0 {
            self.ggst_file_list.push("SOL".to_string());
            self.ggst_file_list.push("KYK".to_string());
            self.ggst_file_list.push("MAY".to_string());
            self.ggst_file_list.push("AXL".to_string());
            self.ggst_file_list.push("CHP".to_string());
            self.ggst_file_list.push("POT".to_string());
            self.ggst_file_list.push("FAU".to_string());
            self.ggst_file_list.push("MLL".to_string());
            self.ggst_file_list.push("ZAT".to_string());
            self.ggst_file_list.push("RAM".to_string());
            self.ggst_file_list.push("LEO".to_string());
            self.ggst_file_list.push("NAG".to_string());
            self.ggst_file_list.push("GIO".to_string());
            self.ggst_file_list.push("ANJ".to_string());
            self.ggst_file_list.push("INO".to_string());
            self.ggst_file_list.push("GLD".to_string());
            self.ggst_file_list.push("JKO".to_string());
            self.ggst_file_list.push("COS".to_string());
            self.ggst_file_list.push("BKN".to_string());
            self.ggst_file_list.push("TST".to_string());
        }
        let col_promise = self.col_promise.get_or_insert_with(|| {
            // Begin download.
            // We download the image using `ehttp`, a library that works both in WASM and on native.
            // We use the `poll-promise` library to communicate with the UI thread.
            let ctx = ctx.clone();
            let (sender, promise) = Promise::new();
            let request = ehttp::Request::get(format!("https://wistfulhopes.neocities.org/pacs/COL_{}.pac", self.selected));
            ehttp::fetch(request, move |response| {
                let pac = response.and_then(parse_col);
                sender.send(pac); // send the results back to the UI thread.
                ctx.request_repaint(); // wake up UI thread
            });
            self.loaded = false;
            promise
        });
        
        let char_promise = self.char_promise.get_or_insert_with(|| {
            // Begin download.
            // We download the image using `ehttp`, a library that works both in WASM and on native.
            // We use the `poll-promise` library to communicate with the UI thread.
            let ctx = ctx.clone();
            let (sender, promise) = Promise::new();
            let request = ehttp::Request::get(format!("https://wistfulhopes.neocities.org/scripts/BBS_{}.bbscript", self.selected));
            ehttp::fetch(request, move |response| {
                let charscript = response_to_bytes(response.unwrap());
                sender.send(charscript); // send the results back to the UI thread.
                ctx.request_repaint(); // wake up UI thread
            });
            self.loaded = false;
            promise
        });
        
        let ef_promise = self.ef_promise.get_or_insert_with(|| {
            // Begin download.
            // We download the image using `ehttp`, a library that works both in WASM and on native.
            // We use the `poll-promise` library to communicate with the UI thread.
            let ctx = ctx.clone();
            let (sender, promise) = Promise::new();
            let request = ehttp::Request::get(format!("https://wistfulhopes.neocities.org/scripts/BBS_{}EF.bbscript", self.selected));
            ehttp::fetch(request, move |response| {
                let efscript = response_to_bytes(response.unwrap());
                sender.send(efscript); // send the results back to the UI thread.
                ctx.request_repaint(); // wake up UI thread
            });
            self.loaded = false;
            promise
        });
        
        let ron_promise = self.ron_promise.get_or_insert_with(|| {
            // Begin download.
            // We download the image using `ehttp`, a library that works both in WASM and on native.
            // We use the `poll-promise` library to communicate with the UI thread.
            let ctx = ctx.clone();
            let (sender, promise) = Promise::new();
            let request = ehttp::Request::get(format!("https://wistfulhopes.neocities.org/rons/ggst.ron"));
            ehttp::fetch(request, move |response| {
                let ron = parse_ron(response.unwrap());
                sender.send(ron); // send the results back to the UI thread.
                ctx.request_repaint(); // wake up UI thread
            });
            self.loaded = false;
            promise
        });
        if !self.loaded {
            self.boxes_window.reset();
        }
        self.file_changed = false;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
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
                let mut visuals = ui.ctx().style().visuals.clone();
                visuals.light_dark_radio_buttons(ui);
                ui.ctx().set_visuals(visuals);
            });
            match ron_promise.ready() {
                None => ui.label("Loading game functions..."),
                Some(Err(e)) => ui.label(format!("Failed to load game functions! {}", e)),
                Some(Ok(ron)) => {
                    ui.label("Game functions loaded!");
                            
                    match char_promise.ready() {
                        None => ui.label("Loading character script..."),
                        Some(bytes) => {
                            if self.boxes_window.char_script == "" {
                                self.boxes_window.char_script = match run_parser(ron,bytes, Some(0 as usize), Some(0 as usize), false) {
                                    Ok(script) => {script},
                                    Err(_) => "Error".to_string()
                                };
                                self.boxes_window.collect_states();
                            }
                            if self.boxes_window.char_script == "Error".to_string() {
                                ui.label(format!("Failed to load character script!!"))
                            }
                            else {
                                ui.horizontal(|_ui| {
                                }).response
                            }
                        }
                    };

                    match ef_promise.ready() {
                        None => ui.label("Loading effect script..."),
                        Some(bytes) => {
                            if self.boxes_window.ef_script == "" {
                                self.boxes_window.ef_script = match run_parser(ron,bytes, Some(0 as usize), Some(0 as usize), false) {
                                    Ok(script) => script,
                                    Err(_) => "Error".to_string()
                                };
                                self.boxes_window.collect_ef_states();
                            };
                            if self.boxes_window.ef_script == "Error".to_string() {
                                ui.label("Failed to load effect script!!")
                            }
                            else {
                                ui.horizontal(|_ui| {
                                }).response
                            }
                        }
                    };
                    ui.horizontal(|_ui|{
                    }).response
                }
            };
            match col_promise.ready() {
                None => ui.label("Loading..."),
                Some(Err(e)) => {
                    ui.label(format!("Failed to read pac! Error: {}", e))
                },
                Some(Ok(pac)) => {
                    if !self.loaded {
                        self.boxes_window.open_file(&pac);
                    }
                    self.loaded = true;
                    self.boxes_window.ui(ui);
                    ui.horizontal(|_ui|{

                    }).response
                }
            }
        });
        if self.file_changed {
            self.col_promise = None;
            self.char_promise = None;
            self.ef_promise = None;
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

fn parse_col(response: ehttp::Response) -> Result<GGSTPac, String> {
    let pac = open::open_file(response.bytes);
    pac
}
fn parse_ron(response: ehttp::Response) -> Result<GameDB, BBScriptError> {
    let ron = GameDB::load(response.bytes);
    ron
}
fn response_to_bytes(response: ehttp::Response) -> Vec<u8> {
    response.bytes
}
