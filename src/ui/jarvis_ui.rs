use std::{
    collections::VecDeque,
    path::PathBuf,
    str::FromStr,
    sync::{Arc, Mutex},
};

use crate::ui::schedule;
use crate::{
    data::{Resource, ResourceData, ResourceId},
    InnerJobKind, Job, JobKind, LaborHall,
};
use chrono::{Datelike, Local, Timelike};
use log::debug;
use poll_promise::Promise;

use super::Show;

#[derive(Copy, Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Anchor {
    Day,
    Schedule,
}
impl Anchor {
    pub fn all_apps() -> Vec<(String, Anchor)> {
        vec![
            ("  Day".to_owned(), Anchor::Day),
            ("  Schedule".to_owned(), Anchor::Schedule),
        ]
    }
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
    del_all_msg: bool,
    #[serde(skip)]
    del_msg_index: Option<usize>,
    #[serde(skip)]
    res_queue: Arc<Mutex<VecDeque<Promise<Resource>>>>,
}

impl Default for JarvisUI {
    fn default() -> Self {
        let res_queue = Arc::new(Mutex::new(VecDeque::new()));
        Self {
            name: "Jarvis".to_owned(),
            anchor: Anchor::Schedule,
            show_msg_panel: false,
            labor_hall: Default::default(),
            del_all_msg: false,
            del_msg_index: None,
            res_queue,
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
                    for (name, anchor) in Anchor::all_apps() {
                        if ui
                            .selectable_label(selected_anchor == anchor, name)
                            .clicked()
                        {
                            selected_anchor = anchor;
                        }
                    }
                    self.anchor = selected_anchor;
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let msg_icon = if self.has_msg_or_err() {
                            "󰍡"
                        } else {
                            "󰍥"
                        };
                        if ui.selectable_label(self.show_msg_panel, msg_icon).clicked() {
                            self.show_msg_panel = !self.show_msg_panel;
                        }
                        ui.separator();
                        let dt = Local::now();
                        ui.label(format!(
                            "{}/{:02}/{:02} {:02}:{:02}",
                            dt.year(),
                            dt.month(),
                            dt.day(),
                            dt.hour(),
                            dt.minute()
                        ));
                    });
                });
            });
        if self.show_msg_panel {
            egui::SidePanel::right("msg_panel")
                .min_width(400.0)
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        if ui.button("Add").clicked() {
                            let msg = Resource::pkg_error(format!("err:{}",Local::now()));
                            let mut job: Job = Default::default();
                            job.kind(JobKind::Inner(InnerJobKind::AddMsg(msg)));
                            self.labor_hall.push_job(job);
                        }

                        if let Ok(Some(mut res)) = self.labor_hall.get_all_inner_msg() {
                            if let Err(err) = res.show(&super::ShowKind::ShortWithoutId, ui) {
                                ui.label(format!("{}",err));
                            };
                        }
                    });
                });
        }
        match self.anchor {
            Anchor::Day => {}
            Anchor::Schedule => schedule::ui(self, ctx, frame),
        }
        if let Some(promise) = self.pop_promise() {
            if let Some(resource) = promise.ready() {
                self.push_resrouse(resource.to_owned());
            } else {
                self.push_promise(promise);
            }
        }
        if let Err(err) = self.labor_hall.do_job() {
            debug!("LaborHall Handle Job Err: {}", err);
        };
        ctx.request_repaint();
    }
}
impl JarvisUI {
    pub fn remove_resource(&mut self, index: usize) {}
    pub fn has_msg_or_err(&self) -> bool {
        true
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
    fn push_promise(&mut self, promise: Promise<Resource>) {
        let mut queue = self.res_queue.lock().unwrap();
        queue.push_back(promise);
    }
    fn pop_promise(&mut self) -> Option<Promise<Resource>> {
        let mut queue = self.res_queue.lock().unwrap();
        queue.pop_front()
    }
    fn push_resrouse(&mut self, resource: Resource) {}
    fn pop_resrouse(&mut self) -> Option<Resource> {
        None
    }
}
