use chrono::Local;
use egui::Layout;
use uuid::Uuid;

use crate::Matters;

use super::Show;

impl Show for Matters {
    fn show(&mut self, kind: &super::ShowKind, ui: &mut egui::Ui) -> anyhow::Result<()> {
        match kind {
            super::ShowKind::ShortWithoutId => {
                ui.label(self.name.to_string());
            }
            super::ShowKind::Short => {
                ui.label(self.name.to_string());
            }
            super::ShowKind::Normal => {
                egui::Grid::new(ui.next_auto_id())
                    .num_columns(2)
                    .spacing([30.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("Name: ");
                        ui.label(self.name.to_string());
                        ui.end_row();
                        ui.label("Des: ");
                        let des = match &self.des {
                            Some(d) => d.to_string(),
                            None => "NO DES".into(),
                        };
                        ui.label(des);
                        ui.end_row();
                        ui.label("Start: ");
                        ui.label(self.start_time.format("%Y/%m/%d").to_string());
                        ui.end_row();
                        ui.label("Finish: ");
                        let finish = match &self.finish_time {
                            Some(f) => f.format("%Y/%m/%d").to_string(),
                            None => "BE QUICK".into(),
                        };
                        ui.label(finish);
                        ui.end_row();
                        ui.label("Final: ");
                        let ffinal = match &self.final_time {
                            Some(f) => f.format("%Y/%m/%d").to_string(),
                            None => "TAKE YOU TIME !".into(),
                        };
                        ui.label(ffinal);
                        ui.end_row();
                        ui.label("Magnitude: ");
                        ui.label(self.magnitude.to_string());
                        ui.end_row();
                        ui.label("Urgency: ");
                        ui.label(self.urgency.to_string());
                        ui.end_row();
                        ui.label("Sub: ");
                        let count = self.sub_matters.len();
                        let text = match count == 0 {
                            true => "NO SUB MATTERS".into(),
                            false => count.to_string(),
                        };
                        ui.label(text);
                        ui.end_row();
                    });
                for (index, sub_matters) in self.sub_matters.iter_mut().enumerate() {
                    ui.collapsing(
                        format!("SUB MATTERS {}", index),
                        |ui| -> anyhow::Result<()> {
                            sub_matters.show(kind, ui)?;
                            Ok(())
                        },
                    )
                    .body_returned
                    .unwrap_or(Ok(()))?;
                    ui.separator();
                }
            }
            super::ShowKind::Edit => {
                egui::Grid::new(ui.next_auto_id())
                    .num_columns(2)
                    .spacing([40.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("Name: ");
                        ui.vertical_centered_justified(|ui| {
                            ui.text_edit_singleline(&mut self.name)
                        });
                        ui.end_row();
                        ui.label("Des: ");
                        ui.vertical_centered_justified(|ui| {
                            let mut old_des = match &self.des {
                                Some(d) => d.to_string(),
                                None => "".to_string(),
                            };
                            if ui.text_edit_multiline(&mut old_des).changed() {
                                if old_des == "None" {
                                    self.des = None;
                                } else {
                                    self.des = Some(old_des);
                                }
                            }
                        });
                        ui.end_row();
                        ui.label("Start: ");
                        ui.label(self.start_time.format("%Y/%m/%d").to_string());
                        ui.end_row();
                        ui.label("Finish: ");
                        let finish = match &self.finish_time {
                            Some(f) => f.format("%Y/%m/%d").to_string(),
                            None => "BE QUICK".into(),
                        };
                        ui.label(finish);
                        ui.end_row();
                        ui.label("Final: ");
                        let now_date = Local::now().date_naive();
                        let mut ffinal = match &self.final_time {
                            Some(f) => f.clone(),
                            None => now_date,
                        };
                        ui.add(egui_extras::DatePickerButton::new(&mut ffinal));
                        if ffinal > now_date {
                            if Some(ffinal) != self.final_time {
                                self.final_time = Some(ffinal);
                            }
                        } else {
                            self.final_time = None;
                        }
                        ui.end_row();
                        ui.label("Magnitude: ");
                        ui.add(egui::Slider::new(&mut self.magnitude, -127..=127));
                        ui.end_row();
                        ui.end_row();
                        ui.label("Urgency: ");
                        ui.add(egui::Slider::new(&mut self.urgency, -127..=127));
                        ui.end_row();
                        ui.label("Sub: ");
                        ui.horizontal_centered(|ui| {
                            let count = self.sub_matters.len();
                            let text = match count == 0 {
                                true => "NO SUB MATTERS".into(),
                                false => count.to_string(),
                            };
                            ui.label(text);
                            if ui.button("＋").clicked() {
                                self.sub_matters.push(Default::default());
                            }
                        });
                        ui.end_row();
                    });
                for (index, sub_matters) in self.sub_matters.iter_mut().enumerate() {
                    ui.collapsing(
                        format!("SUB MATTERS {}", index),
                        |ui| -> anyhow::Result<()> {
                            sub_matters.show(kind, ui)?;
                            Ok(())
                        },
                    )
                    .body_returned
                    .unwrap_or(Ok(()))?;
                    ui.separator();
                }
            }
        }
        Ok(())
    }
}
