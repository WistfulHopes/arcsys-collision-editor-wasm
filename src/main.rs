mod app;
pub use app::MyApp;
use eframe::emath::Vec2;

fn main() {
    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        initial_window_size: Some(Vec2{x: 1280.0, y: 720.0}),
        ..Default::default()
    };
    eframe::run_native(
        "GGST Collision Editor Rust v3.4",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}
