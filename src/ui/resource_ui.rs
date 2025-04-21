use crate::{Resource, ResourceData, ResourceId};

use super::{Show, ShowError, ShowKind};

impl Show for Resource {
    fn show(
        &mut self,
        kind: &ShowKind,
        ui: Option<&mut egui::Ui>,
        ctx: Option<&egui::Context>,
        frame: Option<&mut eframe::Frame>,
    ) -> anyhow::Result<()> {
        match kind {
            ShowKind::Short => {
                self.data.show(kind, ui, ctx, frame)?;
            }
            ShowKind::Normal => {
                self.data.show(kind, ui, ctx, frame)?;
                let tui = ui.unwrap();
            }
            ShowKind::Window => {}
        }
        Ok(())
    }
}

impl Show for ResourceId {
    fn show(
        &mut self,
        kind: &ShowKind,
        ui: Option<&mut egui::Ui>,
        ctx: Option<&egui::Context>,
        frame: Option<&mut eframe::Frame>,
    ) -> anyhow::Result<()> {
        todo!()
    }
}

impl Show for ResourceData {
    fn show(
        &mut self,
        kind: &ShowKind,
        ui: Option<&mut egui::Ui>,
        ctx: Option<&egui::Context>,
        frame: Option<&mut eframe::Frame>,
    ) -> anyhow::Result<()> {
        todo!()
    }
}
