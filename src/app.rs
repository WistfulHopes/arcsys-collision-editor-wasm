use arcsys::ggst::pac::GGSTPac;
use bevy::{prelude::{ResMut, AssetServer, Res, Handle, Assets}, reflect::TypeUuid, asset::{AssetLoader, LoadContext, BoxedFuture, LoadedAsset}};
use bevy_egui::{egui::{self, ComboBox}, EguiContext};
use serde::Deserialize;
use self::boxes::BoxesWindow;
use bbscript::{command_db::{GameDB}, run_parser};

mod boxes;

#[derive(Deserialize, TypeUuid, Default)]
#[uuid = "39cadc56-aa9c-4543-8640-a018b74b5052"]
pub struct PacAsset {
    pub value: Option<GGSTPac>,
}

#[derive(Default)]
pub struct PacLoader;

impl AssetLoader for PacLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let custom_asset = PacAsset { value: Some(open_file(bytes.to_vec())?)};
            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["pac"]
    }
}

#[derive(Deserialize, TypeUuid, Default)]
#[uuid = "f0e5c7ff-743d-4049-b095-d4a26fbc8905"]
pub struct BBSAsset {
    pub value: Vec<u8>,
}

#[derive(Default)]
pub struct BBSLoader;

impl AssetLoader for BBSLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let custom_asset = BBSAsset {value: bytes.to_vec()};
            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["bbscript"]
    }
}

#[derive(Deserialize, TypeUuid, Default)]
#[uuid = "5c0609a2-ce4c-4a4c-9652-33eda6aadf28"]
pub struct RonAsset {
    pub value: Option<GameDB>,
}

#[derive(Default)]
pub struct RonLoader;

impl AssetLoader for RonLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let custom_asset = RonAsset { value: Some(GameDB::load(bytes.to_vec())?)};
            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}
#[derive(Default)]
pub struct MyApp {
    boxes_window: BoxesWindow,
    loaded: bool,
    selected: String,
    ggst_file_list: Vec<String>,
    file_changed: bool,
    pac: Option<GGSTPac>,
    char_script: Option<Vec<u8>>,
    ef_script: Option<Vec<u8>>,
    ron: Option<GameDB>,
}
pub fn update(
    mut egui_ctx: ResMut<EguiContext>,
    mut ui_state: ResMut<MyApp>,
    server: Res<AssetServer>,
    pacs: ResMut<Assets<PacAsset>>,
    scripts: ResMut<Assets<BBSAsset>>,
    rons: ResMut<Assets<RonAsset>>,
    ) {
    if ui_state.selected == "".to_string() {
        ui_state.selected = "SOL".to_string();
    }
    if ui_state.ggst_file_list.len() == 0 {
        ui_state.ggst_file_list.push("SOL".to_string());
        ui_state.ggst_file_list.push("KYK".to_string());
        ui_state.ggst_file_list.push("MAY".to_string());
        ui_state.ggst_file_list.push("AXL".to_string());
        ui_state.ggst_file_list.push("CHP".to_string());
        ui_state.ggst_file_list.push("POT".to_string());
        ui_state.ggst_file_list.push("FAU".to_string());
        ui_state.ggst_file_list.push("MLL".to_string());
        ui_state.ggst_file_list.push("ZAT".to_string());
        ui_state.ggst_file_list.push("RAM".to_string());
        ui_state.ggst_file_list.push("LEO".to_string());
        ui_state.ggst_file_list.push("NAG".to_string());
        ui_state.ggst_file_list.push("GIO".to_string());
        ui_state.ggst_file_list.push("ANJ".to_string());
        ui_state.ggst_file_list.push("INO".to_string());
        ui_state.ggst_file_list.push("GLD".to_string());
        ui_state.ggst_file_list.push("JKO".to_string());
        ui_state.ggst_file_list.push("COS".to_string());
        ui_state.ggst_file_list.push("BKN".to_string());
        ui_state.ggst_file_list.push("TST".to_string());
    }
    if !ui_state.loaded {
        ui_state.boxes_window.reset();
    }
    ui_state.file_changed = false;

    let pac_asset: Handle<PacAsset> = server.load(&format!("pacs/COL_{}.pac", ui_state.selected));
    let char_script_asset: Handle<BBSAsset> = server.load(&format!("scripts/BBS_{}.bbscript", ui_state.selected));
    let ef_script_asset: Handle<BBSAsset> = server.load(&format!("scripts/BBS_{}EF.bbscript", ui_state.selected));
    let ron_asset: Handle<RonAsset> = server.load("rons/ggst.ron");

    if ui_state.pac.is_none() {
        ui_state.pac = match pacs.get(pac_asset){
            Some(pac) => pac.value.clone(),
            None => None,
        };
    }

    if ui_state.char_script.is_none() {
        ui_state.char_script = match scripts.get(char_script_asset){
            Some(script) => Some(script.value.clone()),
            None => None,
        };
    }

    if ui_state.ef_script.is_none() {
        ui_state.ef_script = match scripts.get(ef_script_asset){
            Some(script) => Some(script.value.clone()),
            None => None,
        };
    }

    if ui_state.ron.is_none() {
        ui_state.ron = match rons.get(ron_asset){
            Some(ron) => ron.value.clone(),
            None => None,
        };
    }

    egui::CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ComboBox::from_label("Strive")
            .selected_text(format!("{:?}", ui_state.selected))
            .width(150.0)
            .show_ui(ui, |ui| {
                for name in ui_state.ggst_file_list.clone() {
                    if ui.selectable_label(true, name.as_str())
                    .clicked()
                    {
                        ui_state.selected = name.clone();
                        ui_state.loaded = false;
                        ui_state.file_changed = true;
                    };
                }
            });
            let mut visuals = ui.ctx().style().visuals.clone();
            visuals.light_dark_radio_buttons(ui);
            ui.ctx().set_visuals(visuals);
        });

        match ui_state.ron.clone() {
            None => {
                ui.label("Loading game functions...");
                ()
            }
            Some(ron) => {
                ui.label("Game functions loaded!");
                match ui_state.char_script.clone() {
                    None => {
                        ui.label("Loading character script...");
                        ()
                    }
                    Some(bytes) => {
                        if ui_state.boxes_window.char_script == "" {
                            ui_state.boxes_window.char_script = match run_parser(&ron,&bytes, Some(0 as usize), Some(0 as usize), false) {
                                Ok(script) => {script},
                                Err(_) => "Error".to_string()
                            };
                            ui_state.boxes_window.collect_states();
                        }
                        if ui_state.boxes_window.char_script == "Error".to_string() {
                            ui.label(format!("Failed to load character script!!"));
                        }
                        ()
                    }
                };

                match ui_state.ef_script.clone() {
                    None => {
                        ui.label("Loading effect script...");
                        ()
                    }
                    Some(bytes) => {
                        if ui_state.boxes_window.ef_script == "" {
                            ui_state.boxes_window.ef_script = match run_parser(&ron,&bytes, Some(0 as usize), Some(0 as usize), false) {
                                Ok(script) => script,
                                Err(_) => "Error".to_string()
                            };
                            ui_state.boxes_window.collect_ef_states();
                        };
                        if ui_state.boxes_window.ef_script == "Error".to_string() {
                            ui.label("Failed to load effect script!!");
                        }
                        ()
                    }
                };
            }
        };
        match ui_state.pac.clone() {
            None => {
                ui.label("Loading collision data...");
                ()
            }
            Some(pac) => {
                if !ui_state.loaded {
                    ui_state.boxes_window.open_file(&pac);
                }
                ui_state.loaded = true;
                ui_state.boxes_window.ui(ui);
            }
        }
    });
    if ui_state.file_changed {
        ui_state.pac = None;
        ui_state.char_script = None;
        ui_state.ef_script = None;
    }
}

pub fn open_file(file_buf: Vec<u8>) -> Result<GGSTPac, arcsys::Error> {
    match GGSTPac::parse(&file_buf)
    {
        Ok(file) => return Ok(file),
        Err(e) => return Err(e),
    };
}