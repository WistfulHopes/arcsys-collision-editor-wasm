mod app;
mod render;

pub use app::MyApp;
use bevy::{core_pipeline::ClearColor, prelude::{Msaa, Color, App, ResMut, AddAsset}, DefaultPlugins, log::LogPlugin};
use bevy_egui::{EguiPlugin, EguiContext, egui};
use render::CapturePlugin;
use wasm_bindgen::prelude::*;

use crate::app::{PacAsset, PacLoader, BBSAsset, BBSLoader, RonAsset, RonLoader};

/// This is the entry-point for all the web-assembly.
/// This is called once from the HTML.
/// It loads the app, installs some callbacks, then returns.
/// You can add more callbacks like this if you want to call in to your code.
#[wasm_bindgen]
pub fn start(_canvas_id: &str) {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(Msaa { samples: 4 })
        // Optimal power saving and present mode settings for desktop apps.
        .add_plugins_with(DefaultPlugins, |plugins| {
            plugins.disable::<LogPlugin>()
        })
        .add_plugin(EguiPlugin)
        .add_plugin(CapturePlugin)
        .init_resource::<MyApp>()
        .init_resource::<app::AssetPack>()
        .add_asset::<PacAsset>()
        .add_asset::<BBSAsset>()
        .add_asset::<RonAsset>()
        .init_asset_loader::<PacLoader>()
        .init_asset_loader::<BBSLoader>()
        .init_asset_loader::<RonLoader>()
        .add_startup_system(configure_visuals)
        .add_system(app::update)
        .add_system(app::load_gltf)
        .add_system(app::load_animation)
        .add_system(app::spawn_scene);

    #[cfg(target_arch = "wasm32")]
    {
        app.add_plugin(bevy_web_resizer::Plugin);
    }
    app.run()   
}

fn configure_visuals(mut egui_ctx: ResMut<EguiContext>) {
    egui_ctx.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: 0.0.into(),
        ..Default::default()
    });
}