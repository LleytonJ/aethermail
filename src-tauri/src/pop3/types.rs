//! POP3 data types

use serde::{Deserialize, Serialize};

/// POP3 server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pop3Config {
    /// Server hostname
    pub host: String,
    /// Server port (typically 110 for plain, 995 for SSL)
    pub port: u16,
    /// Whether to use SSL/TLS
    pub use_ssl: bool,
}

/// POP3 authentication credentials
#[derive(Debug, Clone)]
pub struct Pop3Credentials {
    /// Email address (used as username)
    pub username: String,
    /// Password
    pub password: String,
}

/// Result of a connection test
#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionTestResult {
    /// Whether the connection was successful
    pub success: bool,
    /// Human-readable message
    pub message: String,
    /// Server greeting (if available)
    pub server_greeting: Option<String>,
}

/// Email retrieved from POP3 server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawEmail {
    /// Message number on the server
    pub message_number: u32,
    /// Unique identifier (from UIDL command)
    pub uidl: String,
    /// Raw email content
    pub raw_content: String,
}

/// Parsed email data
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ParsedEmail {
    /// Unique identifier
    pub uidl: String,
    /// Subject line
    pub subject: Option<String>,
    /// Sender address
    pub from_address: Option<String>,
    /// Sender display name
    pub from_name: Option<String>,
    /// Recipient addresses
    pub to_addresses: Vec<String>,
    /// CC addresses
    pub cc_addresses: Vec<String>,
    /// Date timestamp
    pub date: Option<i64>,
    /// Plain text body
    pub body_text: Option<String>,
    /// HTML body
    pub body_html: Option<String>,
    /// Attachments
    pub attachments: Vec<EmailAttachment>,
    /// Has attachments flag
    pub has_attachments: bool,
}

/// Email attachment metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailAttachment {
    /// Attachment filename
    pub filename: String,
    /// MIME type
    pub mime_type: String,
    /// Size in bytes
    pub size: usize,
    /// Content ID (for inline images)
    pub content_id: Option<String>,
    /// Raw content (base64 encoded)
    pub content: Option<String>,
}

/// Sync progress information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncProgress {
    /// Total emails to sync
    pub total: u32,
    /// Emails processed so far
    pub processed: u32,
    /// Current email being processed
    pub current_uidl: Option<String>,
    /// Whether sync is complete
    pub complete: bool,
    /// Error message if any
    pub error: Option<String>,
}