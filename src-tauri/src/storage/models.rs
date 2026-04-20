//! Database models

use serde::{Deserialize, Serialize};

/// Contact stored in database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbContact {
    pub id: String,
    pub email: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub frequency: i32,
    pub last_contacted_at: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

/// Account stored in database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbAccount {
    pub id: String,
    pub email: String,
    pub display_name: Option<String>,
    pub pop3_host: String,
    pub pop3_port: u16,
    pub use_ssl: bool,
    pub is_active: bool,
}

/// Email stored in database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbEmail {
    pub id: String,
    pub account_id: String,
    pub uidl: String,
    /// 邮件文件夹: inbox, sent, drafts, phishing
    pub folder: String,
    /// 草稿 ID（仅草稿邮件有值，发送后清空）
    pub draft_id: Option<String>,
    pub from_address: Option<String>,
    pub from_name: Option<String>,
    pub to_addresses: String, // JSON array
    pub cc_addresses: String, // JSON array
    pub subject: Option<String>,
    pub body_text: Option<String>,
    pub body_html: Option<String>,
    pub date: i64,
    pub is_read: bool,
    pub is_starred: bool,
    pub has_attachment: bool,
    /// 是否为钓鱼邮件
    pub is_phishing: bool,
    /// 钓鱼检测原因 (JSON array)
    pub phishing_reasons: Option<String>,
    /// 用户是否手动覆盖了钓鱼判定（移回收件箱后不再重复检测）
    #[serde(default)]
    pub user_overridden: bool,
    /// 删除时间戳（回收站功能，NULL 表示未删除）
    #[serde(default)]
    pub deleted_at: Option<i64>,
}

/// Email folder enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EmailFolder {
    Inbox,
    Sent,
    Drafts,
    Phishing,
}

impl EmailFolder {
    pub fn as_str(&self) -> &'static str {
        match self {
            EmailFolder::Inbox => "inbox",
            EmailFolder::Sent => "sent",
            EmailFolder::Drafts => "drafts",
            EmailFolder::Phishing => "phishing",
        }
    }
}

/// Attachment stored in database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbAttachment {
    pub id: String,
    pub email_id: String,
    pub filename: String,
    pub mime_type: String,
    pub size: i64,
    pub content_id: Option<String>,
    /// IMAP MIME section 编号 (如 "2", "3", "2.1")
    pub imap_section: Option<String>,
    /// 是否已下载到本地缓存
    #[serde(default)]
    pub is_downloaded: bool,
    /// 本地缓存文件路径
    pub local_path: Option<String>,
}

/// Draft email update fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DraftUpdates {
    pub to_addresses: String,  // JSON array
    pub cc_addresses: String,  // JSON array
    pub subject: Option<String>,
    pub body_text: Option<String>,
    pub body_html: Option<String>,
}