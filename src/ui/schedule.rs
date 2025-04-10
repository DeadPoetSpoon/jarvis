use super::JarvisUI;
use std::{
    collections::VecDeque,
    path::PathBuf,
    str::FromStr,
    sync::{Arc, Mutex},
};

use crate::{
    data::{Resource, ResourceData, ResourceId},
    Job,
};
use egui::{RichText, Ui};
use egui_extras::{Size, StripBuilder};
use poll_promise::Promise;

pub fn job_ui(jarvis: &mut JarvisUI, ctx: &egui::Context, ui: &mut Ui) {}

pub fn ui(jarvis: &mut JarvisUI, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
                                ui.text_edit_singleline(&mut jarvis.name);
                            });
                        });
                        strip.cell(|ui| {
                            ui.painter().rect_filled(
                                ui.available_rect_before_wrap(),
                                0.0,
                                ui.visuals().error_fg_color,
                            );

                            ui.horizontal(|ui| {
                                ui.label("Hello~ I'm ");
                                ui.label(RichText::new(jarvis.name.clone()).font(egui::FontId {
                                    size: 18f32,
                                    family: egui::FontFamily::Monospace,
                                }));
                            });

                            for i in jarvis.resources.clone() {
                                match i.data {
                                    ResourceData::Jobs(a) => {
                                        for n in a {
                                            ui.label(n.name);
                                        }
                                    }
                                    _ => {}
                                }
                            }
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

                            if ui.button("get job").clicked() {
                                let id = ResourceId {
                                    place: "schedule".to_owned(),
                                    path: PathBuf::from_str(
                                        "work/job/2024/12/54e34ac5-3e0b-4b30-94d5-309f31fd2367.job",
                                    )
                                    .unwrap(),
                                };
                            }
                            if ui.button("fetch all jobs").clicked() {
                                let id = ResourceId {
                                    place: "schedule".to_owned(),
                                    path: PathBuf::from_str("work/job/2024/12").unwrap(),
                                };
                            }
                        });
                        strip.cell(|ui| {
                            ui.painter().rect_filled(
                                ui.available_rect_before_wrap(),
                                0.0,
                                ui.visuals().warn_fg_color,
                            );

                            if ui.button("Add file to Minio").clicked() {
                                // let path = Path::new("/home/Jarvis/test");
                                // let mut file = File::create(path).unwrap();
                                // write!(file, "asd").unwrap();
                                let id = ResourceId {
                                    place: "schedule".to_owned(),
                                    path: PathBuf::from_str("work").unwrap(),
                                };
                                let mut job = Job::new();
                                job.set_name("test".to_owned());
                                let resource = Resource {
                                    id,
                                    data: ResourceData::Job(job),
                                };
                            }
                        });
                    });
                });
            });
    });
    egui::TopBottomPanel::bottom("schedule_bottom_panel")
        .exact_height(30f32)
        .show(ctx, |ui| {
            ui.label("sss");
        });
}
