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

/// MIME 消息结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailStructure {
    /// MIME 部分列表
    pub parts: Vec<MimePart>,
}

/// MIME 部分
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MimePart {
    /// Section 编号 (如 "1", "2", "3.1")
    pub section: String,
    /// MIME 类型 (如 "application/pdf")
    pub mime_type: String,
    /// 文件名
    pub filename: Option<String>,
    /// 大小（字节）
    pub size: u64,
    /// Content-ID（内嵌图片）
    pub content_id: Option<String>,
    /// 是否为附件
    pub is_attachment: bool,
    /// 子部分（multipart 时）
    pub children: Vec<MimePart>,
}

/// 附件元信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachmentMeta {
    /// Section 编号
    pub section: String,
    /// 文件名
    pub filename: String,
    /// MIME 类型
    pub mime_type: String,
    /// 大小
    pub size: u64,
    /// Content-ID
    pub content_id: Option<String>,
}