//! IMAP data types

use serde::{Deserialize, Serialize};

/// IMAP server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImapConfig {
    /// Server hostname
    pub host: String,
    /// Server port (typically 143 for plain, 993 for SSL)
    pub port: u16,
    /// Whether to use SSL/TLS
    pub use_ssl: bool,
}

/// IMAP authentication credentials
#[derive(Debug, Clone)]
pub struct ImapCredentials {
    /// Email address (used as username)
    pub username: String,
    /// Password
    pub password: String,
}

/// Result of a connection test (re-export from POP3 module)
pub use crate::pop3::ConnectionTestResult;

/// Email retrieved from IMAP server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImapMessage {
    /// Unique identifier from IMAP UID
    pub uid: u32,
    /// Raw email content
    pub raw_content: String,
}

/// IMAP sync result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImapSyncResult {
    /// Total emails on server
    pub total: u32,
    /// New emails synced
    pub new: u32,
    /// Whether sync was successful
    pub success: bool,
    /// Human-readable message
    pub message: String,
}

/// Mailbox status information
#[derive(Debug, Clone)]
pub struct MailboxStatus {
    /// Number of messages in mailbox
    pub messages: u32,
    /// UID validity value
    pub uid_validity: u32,
    /// Next UID value
    pub uid_next: u32,
}