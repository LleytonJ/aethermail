//! SMTP Module
//!
//! SMTP email sending functionality using lettre crate.

mod types;
mod client;

pub use types::*;
pub use client::SmtpClient;