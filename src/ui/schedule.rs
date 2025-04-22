use crate::JarvisUI;

use egui::RichText;
use egui_extras::{Size, StripBuilder};

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
    egui::TopBottomPanel::bottom("schedule_bottom_panel")
        .exact_height(30f32)
        .show(ctx, |ui| {
            ui.label("sss");
        });
}
