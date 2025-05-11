use std::path::PathBuf;

use crate::{JarvisUI, Matters, Resource, ResourceId};

use egui::{Align2, Id, Layout, RichText};
use egui_extras::{Size, StripBuilder};

use super::{AppUI, Show};

#[derive(Debug)]
pub struct ScheduleUI {
    show_add_window: bool,
    wait_to_add_matters: Resource,
}

impl Default for ScheduleUI {
    fn default() -> Self {
        Self {
            show_add_window: false,
            wait_to_add_matters: Default::default(),
        }
    }
}

impl AppUI for ScheduleUI {
    fn ui(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top(Id::new("schedule_top_panel"))
            .exact_height(32f32)
            .show(ctx, |ui| {
                ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button(RichText::new("").size(18f32)).clicked() {
                        self.show_add_window = true;
                        self.wait_to_add_matters = Resource {
                            data: crate::ResourceData::Matters(Matters::default()),
                            id: ResourceId {
                                place:Some("schedule".to_string()),
                                path:Some(PathBuf::from("matters")),
                                ..Default::default()
                            }
                        }
                    };
                });
            });
        let name = match &self.wait_to_add_matters.data {
            crate::ResourceData::Matters(matters)=>{
                matters.name.to_string()
            }
            _ => "".to_string(),
        };
        // let window_name = format!("Add Matters: {}",name);
        egui::Window::new("Add Matters")
            .open(&mut self.show_add_window)
            .fade_in(true)
            .fade_out(true)
            .scroll(true)
            .pivot(Align2::CENTER_CENTER)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("").clicked() {}
                        if ui.button("").clicked() {}
                    });
                });
                ui.add_space(5f32);
                if let Err(err) = self.wait_to_add_matters.show(&super::ShowKind::EditData, ui) {
                    ui.label(format!("{}", err));
                };
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            StripBuilder::new(ui)
                .sizes(Size::remainder(), 2)
                .vertical(|mut strip| {
                    strip.strip(|builder| {
                        builder.sizes(Size::remainder(), 2).horizontal(|mut strip| {
                            strip.cell(|ui| {
                                ui.painter().rect_filled(
                                    ui.available_rect_before_wrap(),
                                    0.0,
                                    ui.visuals().hyperlink_color,
                                );
                                // low urgen high import
                                ui.horizontal(|ui| {
                                    ui.label("change name: ");
                                    // ui.text_edit_singleline(&mut jarvis.name);
                                });
                            });
                            strip.cell(|ui| {
                                ui.painter().rect_filled(
                                    ui.available_rect_before_wrap(),
                                    0.0,
                                    ui.visuals().error_fg_color,
                                );
                                if let Err(err) =
                                    self.wait_to_add_matters.show(&super::ShowKind::Normal, ui)
                                {
                                    ui.label(err.to_string());
                                }
                                ui.horizontal(|ui| {
                                    ui.label("Hello~ I'm ");
                                    // ui.label(RichText::new(jarvis.name.clone()).font(egui::FontId {
                                    //     size: 18f32,
                                    //     family: egui::FontFamily::Monospace,
                                    // }));
                                });
                            });
                        });
                    });
                    strip.strip(|builder| {
                        builder.sizes(Size::remainder(), 2).horizontal(|mut strip| {
                            strip.cell(|ui| {
                                ui.painter().rect_filled(
                                    ui.available_rect_before_wrap(),
                                    0.0,
                                    ui.visuals().faint_bg_color,
                                );
                                if ui.button("get job").clicked() {}
                                if ui.button("fetch all jobs").clicked() {}
                            });
                            strip.cell(|ui| {
                                ui.painter().rect_filled(
                                    ui.available_rect_before_wrap(),
                                    0.0,
                                    ui.visuals().warn_fg_color,
                                );

                                if ui.button("Add file to Minio").clicked() {}
                            });
                        });
                    });
                });
        });
        // egui::TopBottomPanel::bottom("schedule_bottom_panel")
        //     .exact_height(30f32)
        //     .show(ctx, |ui| {
        //         ui.label("sss");
        //     });
    }
}
