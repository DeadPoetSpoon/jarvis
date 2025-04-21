mod jarvis_ui;

use egui::Ui;
pub use jarvis_ui::*;

mod resource_ui;
pub use resource_ui::*;

mod schedule;
pub use schedule::*;

#[derive(Default, Debug)]
pub enum ShowKind {
    #[default]
    Short,
    Normal,
    Window,
}

pub trait Show {
    fn show(
        &mut self,
        kind: &ShowKind,
        ui: Option<&mut Ui>,
        ctx: Option<&egui::Context>,
        frame: Option<&mut eframe::Frame>,
    ) -> anyhow::Result<()>;
}

#[derive(Debug)]
pub enum ShowError {
    NeedUI,
}
