//! Useful tauri commands for building the application.

#[cfg(debug_assertions)]
mod generate;
mod open_browser;
mod validate_link;

#[cfg(debug_assertions)]
pub use generate::*;
pub use open_browser::*;
pub use validate_link::*;
