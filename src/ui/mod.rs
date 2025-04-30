mod jarvis_ui;

use egui::Ui;
pub use jarvis_ui::*;

mod resource_ui;

mod message_ui;

mod matters_ui;

mod schedule;
pub use schedule::*;
use thiserror::Error;

#[derive(Default, Debug)]
pub enum ShowKind {
    #[default]
    ShortWithoutId,
    Short,
    Normal,
    Edit,
    EditData,
}

pub trait Show {
    fn show(&mut self, kind: &ShowKind, ui: &mut Ui) -> anyhow::Result<()>;
}

#[derive(Error, Debug)]
pub enum ShowError {
    #[error("Need passing `{0}` to show")]
    NeedPassing(String),
}

pub trait AppUI {
    fn ui(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame);
}
