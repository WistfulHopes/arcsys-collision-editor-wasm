use std::{path::{PathBuf}};
use arcsys::{ggst::{pac::{GGSTPac}, jonbin::{GGSTJonBin}}};
use bevy_egui::{egui::{self, Sense, Frame, emath::{Rect, Pos2, Vec2}, epaint::{Color32, Stroke, ColorImage, Mesh, TextureId, Shape}}};
use serde::{Serialize, Deserialize};
use std::collections::{BTreeMap};
use substring::Substring;

#[derive(Serialize, Deserialize)]
enum MetaKind {
    Pac(GGSTPac),
}

#[derive(Copy, Clone)]
enum BoxType {
    Hurtbox = 0,
    Hitbox = 1,
    ExPoint = 2,
    ExRect = 3,
    ExVector = 4,
    Push = 5,
    TempCenter = 6,
    Neck = 7,
    Abdominal = 8,
    AttackVsPush = 9,
    SpGuard = 10,
    RLeg = 11,
    LLeg = 12,
    Private0 = 13,
    Private1 = 14,
    Private2 = 15,
    Private3 = 16,
    ExtendJon = 17,
}

impl TryFrom<u32> for BoxType {
    type Error = ();

    fn try_from(v: u32) -> Result<Self, Self::Error> {
        match v {
            x if x == BoxType::Hurtbox as u32 => Ok(BoxType::Hurtbox),
            x if x == BoxType::Hitbox as u32 => Ok(BoxType::Hitbox),
            x if x == BoxType::ExPoint as u32 => Ok(BoxType::ExPoint),
            x if x == BoxType::ExRect as u32 => Ok(BoxType::ExRect),
            x if x == BoxType::ExVector as u32 => Ok(BoxType::ExVector),
            x if x == BoxType::Push as u32 => Ok(BoxType::Push),
            x if x == BoxType::TempCenter as u32 => Ok(BoxType::TempCenter),
            x if x == BoxType::Neck as u32 => Ok(BoxType::Neck),
            x if x == BoxType::Abdominal as u32 => Ok(BoxType::Abdominal),
            x if x == BoxType::AttackVsPush as u32 => Ok(BoxType::AttackVsPush),
            x if x == BoxType::SpGuard as u32 => Ok(BoxType::SpGuard),
            x if x == BoxType::RLeg as u32 => Ok(BoxType::RLeg),
            x if x == BoxType::LLeg as u32 => Ok(BoxType::LLeg),
            x if x == BoxType::Private0 as u32 => Ok(BoxType::Private0),
            x if x == BoxType::Private1 as u32 => Ok(BoxType::Private1),
            x if x == BoxType::Private2 as u32 => Ok(BoxType::Private2),
            x if x == BoxType::Private3 as u32 => Ok(BoxType::Private3),
            x if x == BoxType::ExtendJon as u32 => Ok(BoxType::ExtendJon),
            _ => Err(()),
        }
    }
}

#[derive(Default)]
pub struct BoxesWindow {
    path: PathBuf,
    pub jonbins: BTreeMap<String, GGSTJonBin>,
    pub selected: String,
    offset_x: f32,
    offset_y: f32,
    last_cursor_pos: Pos2,
    current_name: String,
    pub is_gbvs: bool,
    pub char_script: String,
    pub ef_script: String,
    states: BTreeMap<String, String>,
    ef_states: BTreeMap<String, String>,
    current_state: (String, String),
    is_ef: bool,
    show_state_list: bool,
    show_state: bool,
    pub box_changed: bool,
    pub image: Option<ColorImage>,
    pub texture: Option<egui::TextureHandle>,
    pub reset_image: bool,
}

impl BoxesWindow {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        self.reset_image = false;
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.is_ef, "Effect States");
            ui.checkbox(&mut self.show_state_list, "Show state list");
            ui.checkbox(&mut self.show_state, "Show state info");
        });
        let height = ui.available_height();
        ui.horizontal(|ui| {
            ui.set_height(height);
            if self.show_state_list {
                ui.vertical(|ui| {
                    ui.push_id(23561, |ui|{
                        egui::ScrollArea::vertical()
                        .max_width(250.0)
                        .show(ui, |ui| {
                            if !self.is_ef {
                                for (name, state) in &self.states {
                                    if ui.selectable_label(true, name)
                                    .clicked()
                                    {
                                        self.current_state = (name.clone(), state.clone());
                                        self.selected = "".to_string();
                                        self.current_name = "".to_string();    
                                    };
                                }
                            }
                            else {
                                for (name, state) in &self.ef_states {
                                    if ui.selectable_label(true, name)
                                    .clicked()
                                    {
                                        self.current_state = (name.clone(), state.clone());
                                        self.selected = "".to_string();
                                        self.current_name = "".to_string();                        
                                    };
                                }
                            }
                        });
                    });
                });    
            }
            if self.show_state {
                ui.vertical(|ui|{
                    if self.current_state.0 != "" {
                        self.display_state(ui);
                    }
                });
            };
            ui.vertical(|ui|{
                if self.selected != ""{
                    ui.label(format!("Selected sprite: {}", self.selected));
                    ui.label("You can click and drag the canvas to move around!
Double click to reset to the original position.");
                    Frame::canvas(ui.style()).show(ui, |ui| {
                        self.render_boxes(ui);
                    });
                }
                else {
                    ui.horizontal(|ui| {
                        ui.label("Select a sprite to view its hitboxes!");
                    });
                }
            });
        });
    }

    fn render_boxes(&mut self, ui: &mut egui::Ui) {
        let test = self.jonbins.get_mut(&self.selected);
        if test.is_some() {
            let width = ui.available_width();
            let jonb = self.jonbins.get_mut(&self.selected).unwrap();
            let (mut response, painter) = ui.allocate_painter(
                Vec2 {
                    x: width,
                    y: ui.available_height()
                },
                Sense::click_and_drag()
            );
            if let Some(pointer_pos) = response.interact_pointer_pos() {
                if self.last_cursor_pos != Default::default()
                {
                    let pointer_delta = pointer_pos - self.last_cursor_pos;
                    self.offset_x += pointer_delta.x;
                    self.offset_y += pointer_delta.y;
                    response.mark_changed();
                }
                self.last_cursor_pos = pointer_pos;
            }
            else {
                self.last_cursor_pos = Default::default();
            }
            if self.box_changed {
                self.offset_x = width * 0.8;
                self.offset_y = 802.0;
                self.box_changed = false;
            }
            if response.double_clicked()
            {
                self.offset_x = width * 0.8;
                self.offset_y = 802.0;
            }

            if self.image.is_some() {
                let image = self.image.as_ref().unwrap();
                if image.width() != 0 && image.height() != 0 {
                    let texture: &egui::TextureHandle = self.texture.get_or_insert_with(|| {
                        // Load the texture only once.
                        ui.ctx().load_texture("render", image.clone())
                    });
                    let mut mesh = Mesh::with_texture(TextureId::from(texture));
                    let pos = Pos2{x: 0.0, y: 0.0};
                    let uv = Pos2{x: 0.0, y: 0.0};
                    let max = Vec2{x: 1920.0, y: 1080.0};
                    let rect = Rect::from_min_size(pos, max);
                    let uv = Rect::from_min_size(uv, max);
                    mesh.add_rect_with_uv(rect, uv, Color32::WHITE);
                    painter.add(Shape::mesh(mesh));
                }
            }

            for boxgroup in &mut jonb.boxes {
                for hitbox in boxgroup {
                    let mut color = Color32::GREEN;
                    match hitbox.kind.try_into(){
                        Ok(BoxType::Hurtbox) => {
                            color = Color32::GREEN;
                            "Hurtbox"},
                        Ok(BoxType::Hitbox) => {
                            color = Color32::RED;
                            "Hitbox"},
                        Ok(BoxType::ExPoint) => {
                            color = Color32::BLUE;
                            "Hitbox"},
                        Ok(BoxType::ExRect) => {
                            color = Color32::GOLD;
                            "ExRect"},
                        Ok(BoxType::ExVector) => {
                            color = Color32::YELLOW;
                            "ExVector"},
                        Ok(BoxType::Push) => {
                            color = Color32::DARK_BLUE;
                            "Push"},
                        Ok(BoxType::TempCenter) => {
                            color = Color32::LIGHT_GREEN;
                            "TempCenter"},
                        Ok(BoxType::Neck) => {
                            color = Color32::LIGHT_RED;
                            "Neck"},
                        Ok(BoxType::Abdominal) => {
                            color = Color32::LIGHT_BLUE;
                            "Abdominal"},
                        Ok(BoxType::AttackVsPush) => {
                            color = Color32::LIGHT_YELLOW;
                            "AttackVsPush"},
                        Ok(BoxType::SpGuard) => {
                            color = Color32::DEBUG_COLOR;
                            "SpGuard"},
                        Ok(BoxType::RLeg) => {
                            color = Color32::KHAKI;
                            "RLeg"},
                        Ok(BoxType::LLeg) => {
                            color = Color32::BROWN;
                            "LLeg"},
                        Ok(BoxType::Private0) => {
                            color = Color32::GRAY;
                            "LLeg"},
                        Ok(BoxType::Private1) => {
                            color = Color32::BLACK;
                            "LLeg"},
                        Ok(BoxType::Private2) => {
                            color = Color32::LIGHT_GRAY;
                            "LLeg"},
                        Ok(BoxType::Private3) => {
                            color = Color32::DARK_GRAY;
                            "SpGuard"},
                        Ok(BoxType::ExtendJon)  => {
                            color = Color32::DARK_RED;
                            "ExtendJon"},
                        Err(_) => ""
                    };
                    painter.rect_stroke(
                        Rect { min: Pos2{x: (hitbox.rect.x_offset + self.offset_x ), 
                            y: (hitbox.rect.y_offset + self.offset_y)}, 
                            max: Pos2{x: (hitbox.rect.x_offset + hitbox.rect.width + self.offset_x), 
                            y: (hitbox.rect.y_offset + hitbox.rect.height + self.offset_y)} },
                        0.0, 
                        Stroke{width: 3.0, color},
                    );
                }
            }
        }
    }

    pub fn reset(&mut self)
    {
        self.path = Default::default();
        self.jonbins = Default::default();
        self.selected = "".to_string();
        self.offset_x = 480.0;
        self.offset_y = 802.0;
        self.last_cursor_pos = Default::default();
        self.current_name = Default::default();
        self.char_script = Default::default();
        self.ef_script = Default::default();
        self.states = Default::default();
        self.ef_states = Default::default();
        self.current_state = Default::default();
        self.show_state_list = true;
        self.show_state = true;
        self.box_changed = true;
    }

    pub fn open_file(&mut self, pac: &GGSTPac) -> bool {
        self.read_pac(pac);
        return true;
    }

    fn read_pac(&mut self, pac: &GGSTPac) {
        for i in &pac.files {
            match GGSTJonBin::parse(&i.contents, self.is_gbvs){
                Ok(jonb) => {
                    self.jonbins.insert(i.name.clone(),
                jonb);
                },
                Err(e) => {
                    println!("Error reading file {}: {}", i.name, e);
                    continue},
            };
        }
    }

    pub fn collect_states(&mut self) {
        let begin_state: Vec<_> = self.char_script.match_indices("beginState").collect();
        let end_state: Vec<_> = self.char_script.match_indices("endState").collect();
        
        for (index, state_pos) in begin_state.iter().enumerate() {
            let state = self.char_script.substring(state_pos.0, end_state[index].0).to_string();
            let state_end = self.char_script[state_pos.0..].find(0xa as char).map(|i| i + state_pos.0);
            let state_name = self.char_script.substring(state_pos.0, state_end.unwrap()).to_string();
            self.states.insert(state_name, state);
        }
    }

    pub fn collect_ef_states(&mut self) {
        let begin_state: Vec<_> = self.ef_script.match_indices("beginState").collect();
        let end_state: Vec<_> = self.ef_script.match_indices("endState").collect();
        
        for (index, state_pos) in begin_state.iter().enumerate() {
            let state = self.ef_script.substring(state_pos.0, end_state[index].0).to_string();
            let state_end = self.ef_script[state_pos.0..].find(0xa as char).map(|i| i + state_pos.0);
            let state_name = self.ef_script.substring(state_pos.0, state_end.unwrap()).to_string();
            self.ef_states.insert(state_name, state);
        }
    }

    fn display_state(&mut self, ui: &mut egui::Ui)
    {
        egui::ScrollArea::vertical()
        .max_width(250.0)
        .show(ui, |ui| {
            let line_breaks: Vec<_> = self.current_state.1.match_indices(0xa as char).collect();
            let mut prev_index: usize = 0;
    
            for line_index in line_breaks {
                let line = self.current_state.1.substring(prev_index, line_index.0).to_string();
                if line.contains("sprite: ") {
                    if ui.selectable_label(true, &line)
                    .clicked()
                    {
                        let quotes: Vec<_> = line.match_indices("'").collect();
                        let name = line.substring(quotes[0].0 + 1, quotes[1].0).to_string();
                        self.selected = name.to_string();
                        self.current_name = "".to_string();
                        self.reset_image = true;
                        self.image = None;
                    };
                }
                else if line.contains("hit:")
                {
                    ui.colored_label(egui::Color32::RED, &line);
                }
                else if line.contains("grabOrRelease:")
                {
                    ui.colored_label(egui::Color32::RED, &line);
                }
                else {
                    ui.label(&line);
                }
                prev_index = line_index.0 + 1;
            }
        });
    }
}