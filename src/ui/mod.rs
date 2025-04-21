mod jarvis_ui;

use egui::Ui;
pub use jarvis_ui::*;

mod resource_ui;
pub use resource_ui::*;

mod message_ui;
pub use message_ui::*;

mod schedule;
pub use schedule::*;
use thiserror::Error;

#[derive(Default, Debug)]
pub enum ShowKind {
    #[default]
    ShortWithoutId,
    Short,
    Normal,
}

pub trait Show {
    fn show(
        &mut self,
        kind: &ShowKind,
        ui: &mut Ui,
    ) -> anyhow::Result<()>;
}

#[derive(Error,Debug)]
pub enum ShowError {
    #[error("Need passing `{0}` to show")]
    NeedPassing(String),
}
