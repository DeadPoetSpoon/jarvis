use crate::ui::schedule;
use crate::{data::Resource, LaborHall};
use chrono::{Datelike, Local, Timelike};
use log::error;

use super::{AppUI, ScheduleUI, Show};

#[derive(Copy, Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Anchor {
    Day,
    Schedule,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct JarvisUI {
    pub name: String,
    pub anchor: Anchor,
    show_msg_panel: bool,
    #[serde(skip)]
    labor_hall: LaborHall,
    #[serde(skip)]
    apps: Vec<(String, Anchor,Box<dyn AppUI>)>
}

impl Default for JarvisUI {
    fn default() -> Self {
        Self {
            name: "Jarvis".to_owned(),
            anchor: Anchor::Schedule,
            show_msg_panel: false,
            labor_hall: Default::default(),
            apps: vec![
                ("  Day".to_owned(), Anchor::Day, Box::new(ScheduleUI::default())),
                ("  Schedule".to_owned(), Anchor::Schedule, Box::new(ScheduleUI::default())),
            ]
        }
    }
}

impl eframe::App for JarvisUI {
    /// save app state
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
    /// main
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("app_tool_bar")
            .exact_height(26.0)
            .show(ctx, |ui| {
                ui.horizontal_wrapped(|ui| {
                    egui::global_theme_preference_switch(ui);
                    ui.add_space(ui.available_size().x / 2.0 - 150.0);
                    ui.visuals_mut().button_frame = false;
                    let mut selected_anchor = self.anchor;
                    for (name, anchor,_ui) in &self.apps {
                        if ui
                            .selectable_label(selected_anchor == *anchor, name)
                            .clicked()
                        {
                            selected_anchor = *anchor;
                        }
                    }
                    self.anchor = selected_anchor;
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let msg_icon = if self.has_msg() { "󰍡" } else { "󰍥" };
                        if ui.selectable_label(self.show_msg_panel, msg_icon).clicked() {
                            self.show_msg_panel = !self.show_msg_panel;
                        }
                        ui.separator();
                        let dt = Local::now();
                        ui.label(dt.format("%Y/%m/%d %H:%M:%S").to_string());
                    });
                });
            });
        if self.show_msg_panel {
            egui::SidePanel::right("msg_panel")
                .min_width(400.0)
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        if ui.button("clear").clicked() {
                            self.clear_all_msg();
                        }
                        if let Ok(Some(mut res)) = self.labor_hall.get_all_inner_msg() {
                            if let Err(err) = res.show(&super::ShowKind::ShortWithoutId, ui) {
                                ui.label(format!("{}", err));
                            };
                        }
                    });
                });
        }
        for (_,archor, app_ui) in self.apps.iter_mut() {
            if *archor == self.anchor {
                app_ui.ui(ctx, frame);
            }
        }
        if let Err(err) = self.labor_hall.do_job() {
            error!("LaborHall Handle Job Err: {}", err);
        };
        ctx.request_repaint();
    }
}
impl JarvisUI {
    pub fn clear_all_msg(&mut self) {
        match self.labor_hall.clear_all_inner_msg() {
            Ok(_) => {}
            Err(err) => {
                error!("From labor hall: {}", err);
            }
        }
    }
    pub fn has_msg(&mut self) -> bool {
        match self.labor_hall.has_inner_msg() {
            Ok(r) => r,
            Err(err) => {
                error!("From labor hall: {}", err);
                false
            }
        }
    }
    pub fn get_all_msg(&mut self) -> Option<Resource> {
        match self.labor_hall.get_all_inner_msg() {
            Ok(res) => res,
            Err(err) => {
                error!("From labor hall: {}", err);
                None
            }
        }
    }
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // set fonts
        let font_definitiona = JarvisUI::get_font();
        cc.egui_ctx.set_fonts(font_definitiona);
        // load app state
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
    fn get_font() -> egui::FontDefinitions {
        let mut font_definitiona = egui::FontDefinitions::default();
        font_definitiona.font_data.insert(
            "0x Regular".to_owned(),
            std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
                "../../assets/fonts/0xProto/0xProtoNerdFontPropo-Regular.ttf"
            ))),
        );
        font_definitiona.font_data.insert(
            "0x Mono Regular".to_owned(),
            std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
                "../../assets/fonts/0xProto/0xProtoNerdFontMono-Regular.ttf"
            ))),
        );
        font_definitiona.font_data.insert(
            "cn Regular".to_owned(),
            std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
                "../../assets/fonts/SourceHanSans/SourceHanSansCN-Regular.otf"
            ))),
        );

        font_definitiona
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "0x Regular".to_owned());

        font_definitiona
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(1, "cn Regular".to_owned());
        font_definitiona
            .families
            .get_mut(&egui::FontFamily::Monospace)
            .unwrap()
            .insert(0, "0x Mono Regular".to_owned());
        font_definitiona
            .families
            .get_mut(&egui::FontFamily::Monospace)
            .unwrap()
            .insert(1, "cn Regular".to_owned());
        font_definitiona
    }
}
