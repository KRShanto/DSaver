//! Common structs and enums for the application.
//!
//! Both backend and frontend will use these types for sharing data between them.
pub(crate) mod browser;
pub(crate) mod error_reporter;
pub(crate) mod link;

pub use browser::*;
pub use error_reporter::*;
pub use link::*;
