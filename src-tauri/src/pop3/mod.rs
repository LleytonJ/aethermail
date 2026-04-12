//! POP3 Email Client Module
//!
//! Provides POP3 protocol implementation for email retrieval.

mod types;
mod client;

pub use types::*;
pub use client::Pop3Client;