use arcsys::ggst::pac::GGSTPac;
use eframe::egui;
use poll_promise::Promise;
use self::boxes::BoxesWindow;

mod open;
mod boxes;

#[derive(Default)]
pub struct MyApp {
    promise: Option<Promise<Result<GGSTPac, String>>>,
    boxes_window: BoxesWindow,
    loaded: bool,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let promise = self.promise.get_or_insert_with(|| {
            // Begin download.
            // We download the image using `ehttp`, a library that works both in WASM and on native.
            // We use the `poll-promise` library to communicate with the UI thread.
            let ctx = ctx.clone();
            let (sender, promise) = Promise::new();
            let request = ehttp::Request::get("https://wistfulhopes.neocities.org/pacs/COL_SOL.pac");
            ehttp::fetch(request, move |response| {
                let pac = response.and_then(parse_response);
                sender.send(pac); // send the results back to the UI thread.
                ctx.request_repaint(); // wake up UI thread
            });
            self.loaded = false;
            promise
        });

        egui::CentralPanel::default().show(ctx, |ui| match promise.ready() {
            None => ui.label("Loading..."),
            Some(Err(e)) => ui.label(format!("Failed to read pac! Error: {}", e)),
            Some(Ok(pac)) => {
                if !self.loaded {
                    self.boxes_window.open_file(&pac);
                }
                ui.horizontal(|ui| {
                    ui.menu_button("Settings", |ui| {
                        ui.checkbox(&mut self.boxes_window.is_gbvs, "Granblue Fantasy Versus");
                    });
                    let mut visuals = ui.ctx().style().visuals.clone();
                    visuals.light_dark_radio_buttons(ui);
                    ui.ctx().set_visuals(visuals);
                });
                self.loaded = true;
                self.boxes_window.ui(ui)
            }
        });
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