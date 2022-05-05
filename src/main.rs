mod app;
pub use app::MyApp;
use bevy::{core_pipeline::ClearColor, prelude::{Msaa, Color, App, ResMut, AddAsset}, DefaultPlugins, log::LogPlugin};
use bevy_egui::{EguiPlugin, EguiContext, egui};

use crate::app::{PacAsset, PacLoader, BBSAsset, BBSLoader, RonAsset, RonLoader};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(Msaa { samples: 4 })
        // Optimal power saving and present mode settings for desktop apps.
        .add_plugins_with(DefaultPlugins, |plugins| {
            plugins.disable::<LogPlugin>()
        })
        .add_plugin(EguiPlugin)
        .init_resource::<MyApp>()
        .add_asset::<PacAsset>()
        .add_asset::<BBSAsset>()
        .add_asset::<RonAsset>()
        .init_asset_loader::<PacLoader>()
        .init_asset_loader::<BBSLoader>()
        .init_asset_loader::<RonLoader>()
        .add_startup_system(configure_visuals)
        .add_system(app::update)
        .run();
}

fn configure_visuals(mut egui_ctx: ResMut<EguiContext>) {
    egui_ctx.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: 0.0.into(),
        ..Default::default()
    });
}