//! Email management commands

use mail_parser::{MessageParser, MimeHeaders};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::pop3::{Pop3Client, Pop3Config, Pop3Credentials, ParsedEmail, EmailAttachment};
use crate::imap::{ImapClient, ImapConfig, ImapCredentials};
use crate::storage::{Database, DbEmail, DbAttachment};
use crate::commands::account::{get_accounts, ProtocolType};

/// Global database instance
static DB: std::sync::OnceLock<Arc<RwLock<Option<Database>>>> = std::sync::OnceLock::new();

/// Initialize the database
pub async fn init_database(app_data_dir: PathBuf) -> Result<(), String> {
    // Ensure directory exists
    if let Some(parent) = app_data_dir.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("创建目录失败: {}", e))?;
    }

    let db_path = app_data_dir.join("aethermail.db");
    let database = Database::new(&db_path).await?;

    let db_cell = DB.get_or_init(|| Arc::new(RwLock::new(None)));
    let mut db_guard = db_cell.write().await;
    *db_guard = Some(database);

    Ok(())
}

/// Get database instance
async fn get_db() -> Result<Database, String> {
    let db_cell = DB.get_or_init(|| Arc::new(RwLock::new(None)));
    let db_guard = db_cell.read().await;

    db_guard
        .as_ref()
        .ok_or_else(|| "数据库未初始化".to_string())
        .map(|db| (*db).clone())
}

/// Email data for frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Email {
    pub id: String,
    pub account_id: String,
    pub from_address: Option<String>,
    pub from_name: Option<String>,
    pub to_addresses: Vec<String>,
    pub cc_addresses: Vec<String>,
    pub subject: Option<String>,
    pub snippet: String,
    pub date: i64,
    pub is_read: bool,
    pub is_starred: bool,
    pub has_attachment: bool,
    /// 是否为钓鱼邮件
    pub is_phishing: bool,
    /// 钓鱼检测原因
    pub phishing_reasons: Option<Vec<String>>,
}

/// Email detail with full body
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailDetail {
    pub id: String,
    pub account_id: String,
    pub from_address: Option<String>,
    pub from_name: Option<String>,
    pub to_addresses: Vec<String>,
    pub cc_addresses: Vec<String>,
    pub subject: Option<String>,
    pub body_text: Option<String>,
    pub body_html: Option<String>,
    pub date: i64,
    pub is_read: bool,
    pub is_starred: bool,
    pub has_attachment: bool,
    pub attachments: Vec<AttachmentInfo>,
    /// 是否为钓鱼邮件
    pub is_phishing: bool,
    /// 钓鱼检测原因
    pub phishing_reasons: Option<Vec<String>>,
}

/// Attachment info for frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachmentInfo {
    pub id: String,
    pub filename: String,
    pub mime_type: String,
    pub size: i64,
}

/// Sync result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResult {
    pub total: u32,
    pub new: u32,
    pub success: bool,
    pub message: String,
}

/// Sync emails for an account
#[tauri::command]
pub async fn sync_emails(account_id: String) -> Result<SyncResult, String> {
    // Get account config
    let accounts = get_accounts()?;
    let account = accounts
        .iter()
        .find(|a| a.id == account_id)
        .ok_or_else(|| "账户不存在".to_string())?;

    // Branch based on protocol type
    match account.protocol {
        ProtocolType::Pop3 => sync_via_pop3(account).await,
        ProtocolType::Imap => sync_via_imap(account).await,
    }
}

/// Sync emails via POP3 protocol
async fn sync_via_pop3(account: &crate::commands::account::AccountConfig) -> Result<SyncResult, String> {
    let account_id = account.id.clone();

    // Create POP3 client
    let config = Pop3Config {
        host: account.pop3_host.clone(),
        port: account.pop3_port,
        use_ssl: account.use_ssl,
    };
    let credentials = Pop3Credentials {
        username: account.email.clone(),
        password: account.password.clone(),
    };

    let client = Pop3Client::new(config, credentials);

    // Connect to server
    let mut session = client.connect().await?;

    // Get UIDL list from server
    let server_uidls = session.get_uidl_list().await?;
    let total = server_uidls.len() as u32;

    // Get existing UIDLs from database
    let db = get_db().await?;
    let existing_uidls = db.get_existing_uidls(&account_id).await?;

    // Filter new emails
    let new_uidls: Vec<(u32, String)> = server_uidls
        .into_iter()
        .filter(|(_, uidl)| !existing_uidls.contains(uidl))
        .collect();

    let total_new = new_uidls.len() as u32;

    // 暂时只获取最新的一封邮件（消息编号最大的）
    let emails_to_download: Vec<(u32, String)> = if !new_uidls.is_empty() {
        // 找到消息编号最大的邮件
        let latest = new_uidls.iter().max_by_key(|(num, _)| *num).cloned();
        match latest {
            Some(email) => vec![email],
            None => vec![],
        }
    } else {
        vec![]
    };

    // Download and save new emails
    let mut saved = 0u32;
    for (_msg_num, uidl) in emails_to_download.iter() {
        match session.fetch_email(*_msg_num).await {
            Ok(raw_email) => {
                // Parse email
                let parsed = parse_email(&raw_email.raw_content, &uidl);

                // Create database record
                let db_email = DbEmail {
                    id: format!("email-{}", chrono::Utc::now().timestamp_millis()),
                    account_id: account_id.clone(),
                    uidl: parsed.uidl,
                    from_address: parsed.from_address,
                    from_name: parsed.from_name,
                    to_addresses: serde_json::to_string(&parsed.to_addresses).unwrap_or_default(),
                    cc_addresses: serde_json::to_string(&parsed.cc_addresses).unwrap_or_default(),
                    subject: parsed.subject,
                    body_text: parsed.body_text,
                    body_html: parsed.body_html,
                    date: parsed.date.unwrap_or(0),
                    is_read: false,
                    is_starred: false,
                    has_attachment: parsed.has_attachments,
                    is_phishing: false,
                    phishing_reasons: None,
                };

                let email_id = db_email.id.clone();

                // Save email
                if db.save_email(&db_email).await.is_ok() {
                    // Save attachments
                    if !parsed.attachments.is_empty() {
                        for att in parsed.attachments {
                            let db_att = DbAttachment {
                                id: format!("att-{}", uuid::Uuid::new_v4()),
                                email_id: email_id.clone(),
                                filename: att.filename,
                                mime_type: att.mime_type,
                                size: att.size as i64,
                                content_id: att.content_id,
                                content: att.content.map(|c| c.into_bytes()),
                            };
                            let _ = db.save_attachment(&db_att).await;
                        }
                    }
                    saved += 1;
                }
            }
            Err(e) => {
                // Log error but continue
                eprintln!("下载邮件 {} 失败: {}", uidl, e);
            }
        }
    }

    // Disconnect
    session.disconnect().await?;

    Ok(SyncResult {
        total,
        new: saved,
        success: true,
        message: format!("同步完成，新增 {} 封邮件（共发现 {} 封新邮件）", saved, total_new),
    })
}

/// Sync emails via IMAP protocol
async fn sync_via_imap(account: &crate::commands::account::AccountConfig) -> Result<SyncResult, String> {
    let account_id = account.id.clone();

    // Create IMAP client
    let config = ImapConfig {
        host: account.imap_host.clone(),
        port: account.imap_port,
        use_ssl: account.imap_use_ssl,
    };
    let credentials = ImapCredentials {
        username: account.email.clone(),
        password: account.password.clone(),
    };

    let client = ImapClient::new(config, credentials);

    // Connect to server
    let mut session = client.connect().await?;

    // Select INBOX
    let mailbox_status = session.select_inbox().await?;
    let total = mailbox_status.messages;

    // Get UID list from server
    let server_uids = session.get_uid_list().await?;

    // Get existing UIDLs from database
    let db = get_db().await?;
    let existing_uidls = db.get_existing_uidls(&account_id).await?;

    // Filter new UIDs (convert existing_uidls from String to u32 for comparison)
    let existing_uids: Vec<u32> = existing_uidls
        .iter()
        .filter_map(|uidl| uidl.parse::<u32>().ok())
        .collect();

    let new_uids: Vec<u32> = server_uids
        .into_iter()
        .filter(|uid| !existing_uids.contains(uid))
        .collect();

    let total_new = new_uids.len() as u32;

    // Download and save new emails (fetch the latest one for now)
    let mut saved = 0u32;
    if let Some(&latest_uid) = new_uids.last() {
        match session.fetch_email(latest_uid).await {
            Ok(imap_message) => {
                // Parse email using mail-parser
                let uidl = format!("{}", imap_message.uid);
                let parsed = parse_email(&imap_message.raw_content, &uidl);

                // Create database record (use imap-{uid} as id)
                let db_email = DbEmail {
                    id: format!("imap-{}-{}", account_id, imap_message.uid),
                    account_id: account_id.clone(),
                    uidl: uidl.clone(),
                    from_address: parsed.from_address,
                    from_name: parsed.from_name,
                    to_addresses: serde_json::to_string(&parsed.to_addresses).unwrap_or_default(),
                    cc_addresses: serde_json::to_string(&parsed.cc_addresses).unwrap_or_default(),
                    subject: parsed.subject,
                    body_text: parsed.body_text,
                    body_html: parsed.body_html,
                    date: parsed.date.unwrap_or(0),
                    is_read: false,
                    is_starred: false,
                    has_attachment: parsed.has_attachments,
                    is_phishing: false,
                    phishing_reasons: None,
                };

                let email_id = db_email.id.clone();

                // Save email
                if db.save_email(&db_email).await.is_ok() {
                    // Save attachments
                    if !parsed.attachments.is_empty() {
                        for att in parsed.attachments {
                            let db_att = DbAttachment {
                                id: format!("att-{}", uuid::Uuid::new_v4()),
                                email_id: email_id.clone(),
                                filename: att.filename,
                                mime_type: att.mime_type,
                                size: att.size as i64,
                                content_id: att.content_id,
                                content: att.content.map(|c| c.into_bytes()),
                            };
                            let _ = db.save_attachment(&db_att).await;
                        }
                    }
                    saved += 1;
                }
            }
            Err(e) => {
                eprintln!("下载邮件 UID {} 失败: {}", latest_uid, e);
            }
        }
    }

    // Logout
    session.logout().await?;

    Ok(SyncResult {
        total,
        new: saved,
        success: true,
        message: format!("IMAP 同步完成，新增 {} 封邮件（共发现 {} 封新邮件）", saved, total_new),
    })
}

/// Parse raw email content
fn parse_email(raw_content: &str, uidl: &str) -> ParsedEmail {
    let parser = MessageParser::new();
    let message = parser.parse(raw_content.as_bytes());

    let mut result = ParsedEmail {
        uidl: uidl.to_string(),
        ..Default::default()
    };

    if let Some(msg) = message {
        // Subject
        result.subject = msg.subject().map(|s| s.to_string());

        // From
        if let Some(from) = msg.from() {
            if let Some(addr) = from.first() {
                result.from_address = addr.address().map(|s| s.to_string());
                result.from_name = addr.name().map(|s| s.to_string());
            }
        }

        // To
        if let Some(to) = msg.to() {
            result.to_addresses = to
                .iter()
                .filter_map(|addr| addr.address().map(|s| s.to_string()))
                .collect();
        }

        // CC
        if let Some(cc) = msg.cc() {
            result.cc_addresses = cc
                .iter()
                .filter_map(|addr| addr.address().map(|s| s.to_string()))
                .collect();
        }

        // Date
        result.date = msg.date().map(|d| d.to_timestamp());

        // Body
        result.body_text = msg.body_text(0).map(|s| s.to_string());
        result.body_html = msg.body_html(0).map(|s| s.to_string());

        // Attachments
        let attachments: Vec<EmailAttachment> = msg
            .attachments()
            .filter_map(|att| {
                let mime_type = att.content_type()
                    .map(|ct| {
                        let type_str = ct.ctype();
                        let subtype_str = ct.subtype().unwrap_or("octet-stream");
                        format!("{}/{}", type_str, subtype_str)
                    })
                    .unwrap_or_else(|| "application/octet-stream".to_string());

                Some(EmailAttachment {
                    filename: att.attachment_name()?.to_string(),
                    mime_type,
                    size: att.contents().len(),
                    content_id: att.content_id().map(|s| s.to_string()),
                    content: Some(base64::Engine::encode(
                        &base64::engine::general_purpose::STANDARD,
                        att.contents(),
                    )),
                })
            })
            .collect();

        result.has_attachments = !attachments.is_empty();
        result.attachments = attachments;
    }

    result
}

/// Get emails for an account
#[tauri::command]
pub async fn get_emails(account_id: String) -> Result<Vec<Email>, String> {
    let db = get_db().await?;
    let db_emails = db.get_emails(&account_id).await?;

    let emails: Vec<Email> = db_emails
        .into_iter()
        .map(|e| {
            let to_addresses: Vec<String> = serde_json::from_str(&e.to_addresses).unwrap_or_default();
            let snippet = e.body_text
                .as_ref()
                .map(|t| t.chars().take(100).collect())
                .unwrap_or_default();

            // 解析钓鱼检测原因
            let phishing_reasons: Option<Vec<String>> = e.phishing_reasons
                .as_ref()
                .and_then(|r| serde_json::from_str(r).ok());

            Email {
                id: e.id,
                account_id: e.account_id,
                from_address: e.from_address,
                from_name: e.from_name,
                to_addresses,
                cc_addresses: serde_json::from_str(&e.cc_addresses).unwrap_or_default(),
                subject: e.subject,
                snippet,
                date: e.date,
                is_read: e.is_read,
                is_starred: e.is_starred,
                has_attachment: e.has_attachment,
                is_phishing: e.is_phishing,
                phishing_reasons,
            }
        })
        .collect();

    Ok(emails)
}

/// Get email detail
#[tauri::command]
pub async fn get_email_detail(id: String) -> Result<Option<EmailDetail>, String> {
    let db = get_db().await?;
    let db_email = db.get_email_by_id(&id).await?;

    if let Some(e) = db_email {
        let attachments = db.get_attachments(&id).await?;

        // 解析钓鱼检测原因
        let phishing_reasons: Option<Vec<String>> = e.phishing_reasons
            .as_ref()
            .and_then(|r| serde_json::from_str(r).ok());

        Ok(Some(EmailDetail {
            id: e.id,
            account_id: e.account_id,
            from_address: e.from_address,
            from_name: e.from_name,
            to_addresses: serde_json::from_str(&e.to_addresses).unwrap_or_default(),
            cc_addresses: serde_json::from_str(&e.cc_addresses).unwrap_or_default(),
            subject: e.subject,
            body_text: e.body_text,
            body_html: e.body_html,
            date: e.date,
            is_read: e.is_read,
            is_starred: e.is_starred,
            has_attachment: e.has_attachment,
            attachments: attachments
                .into_iter()
                .map(|a| AttachmentInfo {
                    id: a.id,
                    filename: a.filename,
                    mime_type: a.mime_type,
                    size: a.size,
                })
                .collect(),
            is_phishing: e.is_phishing,
            phishing_reasons,
        }))
    } else {
        Ok(None)
    }
}

/// Mark email as read
#[tauri::command]
pub async fn mark_email_read(id: String, is_read: bool) -> Result<(), String> {
    let db = get_db().await?;
    db.mark_email_read(&id, is_read).await
}

/// Toggle email star
#[tauri::command]
pub async fn toggle_email_star(id: String, is_starred: bool) -> Result<(), String> {
    let db = get_db().await?;
    db.toggle_email_star(&id, is_starred).await
}

/// Delete emails for an account (when account is deleted)
#[tauri::command]
pub async fn delete_account_emails(account_id: String) -> Result<(), String> {
    let db = get_db().await?;
    db.delete_emails_by_account(&account_id).await
}

/// Get unread count
#[tauri::command]
pub async fn get_unread_count(account_id: String) -> Result<i32, String> {
    let db = get_db().await?;
    db.get_unread_count(&account_id).await
}

/// Update phishing status for an email
#[tauri::command]
pub async fn update_phishing_status(
    id: String,
    is_phishing: bool,
    phishing_reasons: Option<Vec<String>>,
) -> Result<(), String> {
    let db = get_db().await?;

    // 将原因数组转换为 JSON 字符串存储
    let reasons_json = phishing_reasons
        .map(|r| serde_json::to_string(&r).unwrap_or_default());

    db.update_phishing_status(&id, is_phishing, reasons_json).await
}

/// Get phishing emails for an account
#[tauri::command]
pub async fn get_phishing_emails(account_id: String) -> Result<Vec<Email>, String> {
    let db = get_db().await?;
    let db_emails = db.get_phishing_emails(&account_id).await?;

    let emails: Vec<Email> = db_emails
        .into_iter()
        .map(|e| {
            let to_addresses: Vec<String> = serde_json::from_str(&e.to_addresses).unwrap_or_default();
            let snippet = e.body_text
                .as_ref()
                .map(|t| t.chars().take(100).collect())
                .unwrap_or_default();

            // 解析钓鱼检测原因
            let phishing_reasons: Option<Vec<String>> = e.phishing_reasons
                .as_ref()
                .and_then(|r| serde_json::from_str(r).ok());

            Email {
                id: e.id,
                account_id: e.account_id,
                from_address: e.from_address,
                from_name: e.from_name,
                to_addresses,
                cc_addresses: serde_json::from_str(&e.cc_addresses).unwrap_or_default(),
                subject: e.subject,
                snippet,
                date: e.date,
                is_read: e.is_read,
                is_starred: e.is_starred,
                has_attachment: e.has_attachment,
                is_phishing: e.is_phishing,
                phishing_reasons,
            }
        })
        .collect();

    Ok(emails)
}

/// Get phishing email count for an account
#[tauri::command]
pub async fn get_phishing_count(account_id: String) -> Result<i32, String> {
    let db = get_db().await?;
    db.get_phishing_count(&account_id).await
}

/// Get unread phishing email count for an account
#[tauri::command]
pub async fn get_phishing_unread_count(account_id: String) -> Result<i32, String> {
    let db = get_db().await?;
    db.get_phishing_unread_count(&account_id).await
}