use std::{path::{PathBuf}};
use arcsys::{ggst::{pac::{GGSTPac}, jonbin::{GGSTJonBin, HitBox}}};
use eframe::{egui::{self, Response, ComboBox, Sense, Frame}, emath::{Rect, Pos2}, epaint::{Color32, Stroke}};
use serde::{Serialize, Deserialize};
use std::collections::{BTreeMap};

#[derive(serde::Deserialize, serde::Serialize)]
struct Box {
    x: String,
    y: String,
    w: String,
    h: String,
}

#[derive(Serialize, Deserialize)]
enum MetaKind {
    Pac(GGSTPac),
}

impl Default for Box {
    fn default() -> Self {
        Self {
            x: "0.0".to_owned(),
            y: "0.0".to_owned(),
            w: "0.0".to_owned(),
            h: "0.0".to_owned(),
        }
    }
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
#[derive(serde::Deserialize, serde::Serialize)]
pub struct BoxesWindow {
    path: PathBuf,
    pub jonbins: BTreeMap<String, GGSTJonBin>,
    selected: String,
    boxtype: String,
    offset_x: f32,
    offset_y: f32,
    last_cursor_pos: Pos2,
    current_box: Option<HitBox>,
    box_info: Box,
    box_index: u32,
    current_name: String,
    new_name: String,
    jonb_name: String,
    image_index: usize,
    pub is_gbvs: bool,
}

impl BoxesWindow {
    pub fn ui(&mut self, ui: &mut egui::Ui) -> Response {
        ComboBox::from_label("File list")
        .selected_text(format!("{:?}", self.selected))
        .width(150.0)
        .show_ui(ui, |ui| {
            for (name, _jonbin) in &self.jonbins {
                if ui.selectable_label(true, name)
                .clicked()
                {
                    self.current_box = None;
                    self.box_index = 0;
                    self.boxtype = "".to_string();
                    self.selected = name.to_string();
                    self.current_name = "".to_string();
                };
            }
        });
        if self.selected != ""{
            self.box_list(ui);
            ui.label("You can click and drag the canvas to move around!
Right click to reset to the original position.");
            Frame::canvas(ui.style()).show(ui, |ui| {
                self.render_boxes(ui);
            });
        }
        else {
            ui.horizontal(|ui| {
                ui.label("Select a file from the file list!");
            });
        }
        ui.horizontal(|_ui| {
        }).response
    }

    fn box_list(&mut self, ui: &mut egui::Ui) {
        let jonb = self.jonbins.get(&self.selected).unwrap();
        ui.horizontal(|ui| {
            ComboBox::from_label("Box list")
            .selected_text(format!("{} #{}", self.boxtype, self.box_index))
            .width(150.0)
            .show_ui(ui, |ui| {
                for boxgroup in &jonb.boxes {
                    for (index, hitbox) in boxgroup.iter().enumerate() {
                        let kind = match hitbox.kind.try_into(){
                            Ok(BoxType::Hurtbox) => "Hurtbox",
                            Ok(BoxType::Hitbox) => "Hitbox",
                            Ok(BoxType::ExPoint) => "ExPoint",
                            Ok(BoxType::ExRect) => "ExRect",
                            Ok(BoxType::ExVector) => "ExVector",
                            Ok(BoxType::Push) => "Push",
                            Ok(BoxType::TempCenter) => "TempCenter",
                            Ok(BoxType::Neck) => "Neck",
                            Ok(BoxType::Abdominal) => "Abdominal",
                            Ok(BoxType::AttackVsPush) => "AttackVsPush",
                            Ok(BoxType::SpGuard) => "SpGuard",
                            Ok(BoxType::RLeg) => "RLeg",
                            Ok(BoxType::LLeg) => "LLeg",
                            Ok(BoxType::Private0) => "Private0",
                            Ok(BoxType::Private1) => "Private1",
                            Ok(BoxType::Private2) => "Private2",
                            Ok(BoxType::Private3) => "Private3",
                            Ok(BoxType::ExtendJon) => "ExtendJon",
                            Err(_) => ""
                        };
                        if ui.selectable_label(true, format!("{} #{}", kind, index))
                        .clicked()
                        {
                            self.box_index = index as u32;
                            self.boxtype = kind.to_string();
                            self.box_info.x = format!("{}", hitbox.rect.x_offset);
                            self.box_info.y = format!("{}", hitbox.rect.y_offset);
                            self.box_info.w = format!("{}", hitbox.rect.width);
                            self.box_info.h = format!("{}", hitbox.rect.height);
                            self.current_box = Some(*hitbox);
                        };
                    }
                }
            });
        });
    }
    fn render_boxes(&mut self, ui: &mut egui::Ui) -> Response {
        let jonb = self.jonbins.get_mut(&self.selected).unwrap();
        let (mut response, painter) = ui.allocate_painter(
            eframe::emath::Vec2 {
                x: (ui.available_width()),
                y: (ui.available_height())
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
        if response.clicked_by(egui::PointerButton::Secondary)
        {
            self.offset_x = 640.0;
            self.offset_y = 802.0;
        }
        
        for boxgroup in &mut jonb.boxes {
            for (index, hitbox) in boxgroup.iter_mut().enumerate() {
                let mut color = Color32::GREEN;
                let kind = match hitbox.kind.try_into(){
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
                if self.box_index == index as u32 && self.boxtype == kind
                {
                    hitbox.rect.x_offset = self.current_box.unwrap().rect.x_offset;
                    hitbox.rect.y_offset = self.current_box.unwrap().rect.y_offset;
                    hitbox.rect.width = self.current_box.unwrap().rect.width;
                    hitbox.rect.height = self.current_box.unwrap().rect.height;
                }
                painter.rect_stroke(
                    Rect { min: Pos2{x: (hitbox.rect.x_offset + self.offset_x - 1.5), 
                        y: (hitbox.rect.y_offset + self.offset_y - 1.5)}, 
                        max: Pos2{x: (hitbox.rect.x_offset + hitbox.rect.width + self.offset_x + 1.5 ), 
                        y: (hitbox.rect.y_offset + hitbox.rect.height + self.offset_y + 1.5)} },
                    0.0, 
                    Stroke{width: 3.0, color},
                );
            }
        }
        response
    }

    fn reset(&mut self)
    {
        self.path = Default::default();
        self.jonbins = Default::default();
        self.selected = "".to_string();
        self.boxtype = "".to_string();
        self.offset_x = 640.0;
        self.offset_y = 802.0;
        self.last_cursor_pos = Default::default();
        self.current_box = Default::default();
        self.box_info = Default::default();
        self.box_index = 0;
    }

    pub fn open_file(&mut self, pac: &GGSTPac) -> bool {
        self.reset();
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
}