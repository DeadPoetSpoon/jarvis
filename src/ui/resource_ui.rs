use std::path::PathBuf;

use uuid::Uuid;

use crate::{Resource, ResourceData, ResourceId};

use super::{Show, ShowKind};

impl Show for Resource {
    fn show(&mut self, kind: &ShowKind, ui: &mut egui::Ui) -> anyhow::Result<()> {
        match kind {
            ShowKind::ShortWithoutId => {
                self.data.show(kind, ui)?;
            }
            ShowKind::Short => {
                ui.horizontal(|ui| -> anyhow::Result<()> {
                    self.data.show(kind, ui)?;
                    self.id.show(&ShowKind::Short, ui)?;
                    Ok(())
                })
                .inner?;
            }
            _ => {
                self.id.show(kind, ui)?;
                ui.separator();
                self.data.show(kind, ui)?;
            }
        }
        Ok(())
    }
}

impl Show for ResourceId {
    fn show(&mut self, kind: &ShowKind, ui: &mut egui::Ui) -> anyhow::Result<()> {
        match kind {
            ShowKind::Short => {
                ui.label("")
                    .on_hover_cursor(egui::CursorIcon::Help)
                    .on_hover_ui(|x| {
                        x.style_mut().interaction.selectable_labels = true;
                        if let Err(err) = self.show(&ShowKind::Normal, x) {
                            x.label(format!("{}", err));
                        };
                    });
            }
            ShowKind::Edit => {
                egui::Grid::new(self.uid).num_columns(2).spacing([30.0, 4.0]).show(ui, |ui| {
                    ui.label("UID: ");

                    ui.vertical_centered_justified(|ui| {
                        let mut old_id = self.uid.to_string();
                        if ui.text_edit_singleline(&mut old_id).changed() {
                            if let Ok(nid) = Uuid::try_parse(&old_id) {
                                self.uid = nid;
                            };
                        };
                    });

                    ui.end_row();
                    ui.label("Place: ");
                    ui.vertical_centered_justified(|ui| {
                        let mut old_place = match &self.place {
                            Some(p) => p.to_string(),
                            None => "".to_string(),
                        };
                        if ui.text_edit_singleline(&mut old_place).changed() {
                            self.place = Some(old_place)
                        };
                    });

                    ui.end_row();
                    ui.label("Path: ");
                    ui.vertical_centered_justified(|ui| {
                        let mut old_path: String = match &self.path {
                            Some(p) => p.to_str().unwrap().to_string(),
                            None => "".to_string(),
                        };
                        if ui.text_edit_singleline(&mut old_path).changed() {
                            self.path = Some(PathBuf::from(old_path))
                        };
                    });
                });
            }
            _ => {
                egui::Grid::new(self.uid).num_columns(2).show(ui,|ui|{
                    ui.label("UID: ");
                    ui.label(self.uid.to_string());
                    ui.end_row();
                    if let (Some(place), Some(path)) = (&self.place, &self.path) {
                        ui.label("Place: ");
                        ui.label(place);
                        ui.end_row();
                        ui.label("Path: ");
                        ui.label(path.to_str().unwrap_or_default());
                    }
                });
            }
        }
        Ok(())
    }
}

impl Show for ResourceData {
    fn show(&mut self, kind: &ShowKind, ui: &mut egui::Ui) -> anyhow::Result<()> {
        match self {
            ResourceData::NoData => {
                ui.label("NO DATA");
            }
            ResourceData::Message(message) => {
                message.show(kind, ui)?;
            }
            ResourceData::Matters(matters) => {
                matters.show(kind, ui)?;
            }
            ResourceData::Mutli(resources) => {
                ui.vertical(|ui| -> anyhow::Result<()> {
                    for resource in resources {
                        resource.show(kind, ui)?;
                    }
                    Ok(())
                });
            }
            ResourceData::WithData => {
                ui.label("WITH INVISIABLE DATA");
            }
        }
        Ok(())
    }
}
