//! IMAP Email Client Module
//!
//! Provides IMAP protocol implementation for email retrieval.
//! Uses async-imap crate for IMAP protocol handling.

mod types;
mod client;

pub use types::*;
pub use client::ImapClient;
pub use client::ImapSession;