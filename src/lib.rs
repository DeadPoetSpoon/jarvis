/// view and view  model
#[cfg(feature = "ui")]
mod ui;
#[cfg(feature = "ui")]
pub use ui::JarvisUI;

/// all model and porter to handle resource
mod data;
pub use data::*;
mod porter;
pub use porter::*;

/// server to handle all resource and heavy work
#[cfg(feature = "server")]
mod server;
#[cfg(feature = "server")]
pub use server::serve_jarvis_rocket;
