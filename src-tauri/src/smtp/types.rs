//! SMTP Types
//!
//! Data types for SMTP email sending.

use serde::{Deserialize, Serialize};

/// SMTP server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmtpConfig {
    /// SMTP server host
    pub host: String,
    /// SMTP server port (default: 465 for SSL)
    pub port: u16,
    /// Whether to use SSL/TLS
    pub use_ssl: bool,
}

impl Default for SmtpConfig {
    fn default() -> Self {
        Self {
            host: String::new(),
            port: 465,
            use_ssl: true,
        }
    }
}

/// SMTP credentials for authentication
#[derive(Debug, Clone)]
pub struct SmtpCredentials {
    /// Username (typically email address)
    pub username: String,
    /// Password
    pub password: String,
}

/// SMTP connection test result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmtpTestResult {
    /// Whether the test succeeded
    pub success: bool,
    /// Result message
    pub message: String,
}

/// Email message for sending
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailMessage {
    /// Sender email address
    pub from_address: String,
    /// Sender display name (optional)
    pub from_name: Option<String>,
    /// Recipient email addresses
    pub to_addresses: Vec<String>,
    /// CC recipients (optional)
    pub cc_addresses: Option<Vec<String>>,
    /// Email subject
    pub subject: String,
    /// Email body (plain text)
    pub body: String,
}

/// Email send result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendEmailResult {
    /// Whether the send succeeded
    pub success: bool,
    /// Result message
    pub message: String,
}