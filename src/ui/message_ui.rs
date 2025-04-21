use egui::RichText;

use crate::Message;

use super::Show;

impl Show for Message {
    fn show(
        &mut self,
        _kind: &super::ShowKind,
        ui: &mut egui::Ui
    ) -> anyhow::Result<()> {
        match self {
            Message::SimpleMessage(msg) => {
                let text = RichText::new(msg.to_string());
                ui.label(text);
            },
            Message::Error(msg) => {
                let text = RichText::new(msg.to_string())
                                    .color(ui.visuals().error_fg_color);
                ui.label(text);
            },
        }
        Ok(())
    }
}