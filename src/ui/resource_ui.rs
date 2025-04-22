
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
            ShowKind::Normal => {
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
            _ => {
                let text = format!("UID:{}", self.uid.to_string());
                ui.label(text);
                if let (Some(place), Some(path)) = (&self.place, &self.path) {
                    let text = format!("Place:{}", place);
                    ui.label(text);
                    let text = format!("Path:{}", path.to_str().unwrap_or_default());
                    ui.label(text);
                }
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
            ResourceData::Matters(_matters) => todo!(),
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
            },
        }
        Ok(())
    }
}
