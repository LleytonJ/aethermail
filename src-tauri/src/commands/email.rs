//! Email management commands

use mail_parser::{MessageParser, MimeHeaders};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tauri::Emitter;

use crate::pop3::{Pop3Client, Pop3Config, Pop3Credentials, ParsedEmail, EmailAttachment};
use crate::imap::{ImapClient, ImapConfig, ImapCredentials};
use crate::smtp::{SmtpClient, SmtpConfig, SmtpCredentials, EmailMessage, SendEmailResult, AttachmentData};
use crate::storage::{Database, DbEmail, DbAttachment, DbDraftAttachment, DbThread, DraftUpdates};
use crate::commands::account::{get_accounts, ProtocolType};
use crate::ai::{AiClient, is_ai_available};
use crate::utils::html_extractor::HtmlExtractor;

/// Global database instance
static DB: std::sync::OnceLock<Arc<RwLock<Option<Database>>>> = std::sync::OnceLock::new();

/// Global app data directory for cache
static APP_DATA_DIR: std::sync::OnceLock<PathBuf> = std::sync::OnceLock::new();

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

    // Store app data directory for cache
    APP_DATA_DIR.get_or_init(|| app_data_dir.clone());

    Ok(())
}

/// Get app data directory
fn get_app_data_dir() -> Result<PathBuf, String> {
    APP_DATA_DIR
        .get()
        .cloned()
        .ok_or_else(|| "应用数据目录未初始化".to_string())
}

/// Get cache directory path
fn get_cache_directory() -> Result<PathBuf, String> {
    let app_dir = get_app_data_dir()?;
    let cache_dir = app_dir.join("attachments");

    // Ensure directory exists
    std::fs::create_dir_all(&cache_dir)
        .map_err(|e| format!("创建缓存目录失败: {}", e))?;

    Ok(cache_dir)
}

/// Get attachment cache file path
fn get_attachment_cache_path(attachment_id: &str) -> Result<PathBuf, String> {
    let cache_dir = get_cache_directory()?;
    Ok(cache_dir.join(format!("{}.dat", attachment_id)))
}

/// Get database instance
pub async fn get_db() -> Result<Database, String> {
    let db_cell = DB.get_or_init(|| Arc::new(RwLock::new(None)));
    let db_guard = db_cell.read().await;

    db_guard
        .as_ref()
        .ok_or_else(|| "数据库未初始化".to_string())
        .map(|db| (*db).clone())
}

/// Check if database is ready
#[tauri::command]
pub async fn is_database_ready() -> Result<bool, String> {
    let db_cell = DB.get_or_init(|| Arc::new(RwLock::new(None)));
    let db_guard = db_cell.read().await;
    Ok(db_guard.is_some())
}

/// Wait for database to be ready (with timeout in ms)
#[tauri::command]
pub async fn wait_for_database(timeout_ms: u64) -> Result<bool, String> {
    let db_cell = DB.get_or_init(|| Arc::new(RwLock::new(None)));

    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_millis(timeout_ms);

    while start.elapsed() < timeout {
        let db_guard = db_cell.read().await;
        if db_guard.is_some() {
            return Ok(true);
        }
        drop(db_guard);
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }

    Ok(false)
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

/// 检测邮件是否为钓鱼邮件
/// 返回 (is_phishing, phishing_reasons)
async fn detect_phishing_for_email(parsed: &ParsedEmail) -> (bool, Option<Vec<String>>) {
    log::info!("[Sync] ========== 开始钓鱼检测流程 ==========");
    log::info!("[Sync] 邮件信息:");
    log::info!("[Sync]   UIDL: {}", parsed.uidl);
    log::info!("[Sync]   发件人: {} ({})", parsed.from_name.as_deref().unwrap_or("未知"), parsed.from_address.as_deref().unwrap_or("未知"));
    log::info!("[Sync]   主题: {}", parsed.subject.as_deref().unwrap_or("无主题"));
    log::info!("[Sync]   正文长度: {} 字符", parsed.body_text.as_deref().map(|t| t.len()).unwrap_or(0));

    // 检查 AI 是否可用（包含 mock_phishing 模式检查）
    log::info!("[Sync] 步骤1: 检查 AI 服务可用性");
    if !is_ai_available() {
        log::warn!("[Sync] AI 未启用或配置不完整，跳过钓鱼检测");
        log::info!("[Sync] ========== 钓鱼检测跳过 (AI未启用) ==========");
        return (false, None);
    }
    log::info!("[Sync] AI 服务可用");

    // 创建 AI 客户端（mock_phishing 模式下会自动启用 mock）
    log::info!("[Sync] 步骤2: 创建 AI 客户端");
    let ai_client = match AiClient::new() {
        Ok(client) => {
            log::info!("[Sync] AI 客户端创建成功");
            client
        }
        Err(e) => {
            log::error!("[Sync] 创建 AI 客户端失败: {}", e);
            log::info!("[Sync] ========== 钓鱼检测失败 (客户端创建失败) ==========");
            return (false, None);
        }
    };

    // 检查客户端是否可用（mock 模式下也会返回 true）
    log::info!("[Sync] 步骤3: 检查客户端可用性");
    if !ai_client.is_available() {
        log::warn!("[Sync] AI 客户端不可用，跳过钓鱼检测");
        log::info!("[Sync] ========== 钓鱼检测跳过 (客户端不可用) ==========");
        return (false, None);
    }
    log::info!("[Sync] 客户端可用");

    // 构建检测内容
    log::info!("[Sync] 步骤4: 构建检测内容");
    let email_content = format!(
        "<email_content>
发件人: {} ({})
收件人: {}
主题: {}

邮件内容:
{}
</email_content>

请分析上述邮件是否为钓鱼邮件，输出 JSON 结果。",
        parsed.from_name.as_deref().unwrap_or("未知"),
        parsed.from_address.as_deref().unwrap_or("未知"),
        parsed.to_addresses.join(", "),
        parsed.subject.as_deref().unwrap_or("无主题"),
        parsed.body_text.as_deref().unwrap_or("无内容")
    );

    log::info!("[Sync] 检测内容长度: {} 字符", email_content.len());
    log::debug!("[Sync] 检测内容前500字符: {}", if email_content.len() > 500 { &email_content[..500] } else { &email_content });

    // 调用 AI 检测（10秒超时）
    // mock_phishing 模式下会返回模拟的钓鱼检测结果
    log::info!("[Sync] 步骤5: 调用 AI API 进行检测");
    log::info!("[Sync] 超时设置: 10000ms");

    match ai_client.detect_phishing(&email_content, 10000).await {
        Ok(result) => {
            log::info!("[Sync] 钓鱼检测完成:");
            log::info!("[Sync]   is_phishing: {}", result.is_phishing);
            log::info!("[Sync]   reasons数量: {}", result.reasons.len());
            for (i, reason) in result.reasons.iter().enumerate() {
                log::info!("[Sync]   reason[{}]: {}", i, reason);
            }
            log::info!("[Sync] ========== 钓鱼检测完成 ==========");

            if result.is_phishing {
                log::warn!("[Sync] ⚠️ 发现钓鱼邮件! UIDL: {}", parsed.uidl);
            } else {
                log::info!("[Sync] ✓ 邮件安全，非钓鱼邮件");
            }

            (result.is_phishing, Some(result.reasons))
        }
        Err(e) => {
            // 检测失败时保守处理：不标记为钓鱼邮件
            log::error!("[Sync] 钓鱼检测失败: {}", e);
            log::warn!("[Sync] 检测失败，保守处理：不标记为钓鱼邮件");
            log::info!("[Sync] ========== 钓鱼检测失败 (保守处理) ==========");
            (false, None)
        }
    }
}

/// Sync emails via POP3 protocol (attachments still downloaded, but stored with new structure)
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

    // 获取最新的5封邮件（消息编号最大的）
    let emails_to_download: Vec<(u32, String)> = if !new_uidls.is_empty() {
        // 按消息编号排序，取最大的5封
        let mut sorted: Vec<(u32, String)> = new_uidls.into_iter().collect();
        sorted.sort_by_key(|(num, _)| std::cmp::Reverse(*num));
        sorted.into_iter().take(5).collect()
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

                // 钓鱼检测（在保存前执行）
                let (is_phishing, phishing_reasons) = detect_phishing_for_email(&parsed).await;
                let phishing_reasons_json = phishing_reasons
                    .map(|r| serde_json::to_string(&r).unwrap_or_default());

                // Create database record
                // 钓鱼邮件放入 phishing folder，其他邮件放入 inbox
                let folder = if is_phishing { "phishing" } else { "inbox" };
                let db_email = DbEmail {
                    id: format!("email-{}", chrono::Utc::now().timestamp_millis()),
                    account_id: account_id.clone(),
                    uidl: parsed.uidl,
                    folder: folder.to_string(),
                    draft_id: None,
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
                    is_phishing,
                    phishing_reasons: phishing_reasons_json,
                    user_overridden: false,
                    deleted_at: None,
                    // POP3 无法获取 threading headers
                    message_id_header: None,
                    in_reply_to_header: None,
                    references_header: None,
                    thread_id: None,
                };

                let email_id = db_email.id.clone();

                // Save email
                if db.save_email(&db_email).await.is_ok() {
                    // Save attachments (POP3 全量下载，但使用新结构)
                    // 注意：POP3 无法获取 IMAP section，所以 imap_section = None
                    // 附件内容暂时不保存到文件（需要 app_handle），标记为未下载
                    if !parsed.attachments.is_empty() {
                        for att in parsed.attachments {
                            let db_att = DbAttachment {
                                id: format!("att-{}", uuid::Uuid::new_v4()),
                                email_id: email_id.clone(),
                                filename: att.filename,
                                mime_type: att.mime_type,
                                size: att.size as i64,
                                content_id: att.content_id,
                                imap_section: None, // POP3 无 section 信息
                                is_downloaded: false, // 暂时标记为未下载
                                local_path: None,
                            };
                            let _ = db.save_attachment(&db_att).await;
                        }
                    }
                    saved += 1;
                }
            }
            Err(e) => {
                // Log error but continue
                log::error!("下载邮件 {} 失败: {}", uidl, e);
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

/// Sync emails via IMAP protocol (simplified for reliability)
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

    // 获取最新的5封邮件（UID最大的）
    let emails_to_download: Vec<u32> = new_uids
        .into_iter()
        .rev()  // UID列表已排序，reverse后最大的在前面
        .take(5)
        .collect();

    let total_new = emails_to_download.len() as u32;

    // Download and save new emails
    let mut saved = 0u32;
    for &uid in emails_to_download.iter() {
        // 简化流程：直接获取完整邮件，避免多次 fetch 导致连接断开
        match session.fetch_email(uid).await {
            Ok(imap_message) => {
                let uidl = format!("{}", imap_message.uid);
                let parsed = parse_email(&imap_message.raw_content, &uidl);

                // 钓鱼检测
                let (is_phishing, phishing_reasons) = detect_phishing_for_email(&parsed).await;
                let phishing_reasons_json = phishing_reasons
                    .map(|r| serde_json::to_string(&r).unwrap_or_default());

                // Create database record
                // 钓鱼邮件放入 phishing folder，其他邮件放入 inbox
                let folder = if is_phishing { "phishing" } else { "inbox" };

                // Threading headers: generate synthetic Message-ID if missing
                let message_id_header = imap_message.message_id.clone()
                    .unwrap_or_else(|| format!("imap-{}-{}@synthetic", account_id, imap_message.uid));

                let db_email = DbEmail {
                    id: format!("imap-{}-{}", account_id, imap_message.uid),
                    account_id: account_id.clone(),
                    uidl: uidl.clone(),
                    folder: folder.to_string(),
                    draft_id: None,
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
                    is_phishing,
                    phishing_reasons: phishing_reasons_json,
                    user_overridden: false,
                    deleted_at: None,
                    // Threading header fields
                    message_id_header: Some(message_id_header),
                    in_reply_to_header: imap_message.in_reply_to.clone(),
                    references_header: imap_message.references.clone(),
                    thread_id: None,  // Will be assigned by frontend threading computation
                };

                let email_id = db_email.id.clone();

                // Save email
                if db.save_email(&db_email).await.is_ok() {
                    // Save attachments (标记为未下载，用户点击时重新 fetch)
                    // 附件没有 imap_section 信息，需要在下载时重新获取邮件结构
                    if parsed.has_attachments {
                        // 从解析结果中获取附件元信息
                        for att in parsed.attachments {
                            let db_att = DbAttachment {
                                id: format!("att-{}", uuid::Uuid::new_v4()),
                                email_id: email_id.clone(),
                                filename: att.filename,
                                mime_type: att.mime_type,
                                size: att.size as i64,
                                content_id: att.content_id,
                                // 不存储 section，下载时需要重新解析邮件结构
                                imap_section: None,
                                is_downloaded: false,
                                local_path: None,
                            };
                            let _ = db.save_attachment(&db_att).await;
                        }
                    }
                    saved += 1;
                }
            }
            Err(e) => {
                log::error!("下载邮件 UID {} 失败: {}", uid, e);
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

        // Body - 智能文本提取
        // 1. 先获取 mail-parser 提供的 body_text
        let raw_body_text = msg.body_text(0).map(|s| s.to_string());
        let raw_body_html = msg.body_html(0).map(|s| s.to_string());

        // 2. 检查 body_text 是否干净（无 CSS/JS 干扰）
        result.body_text = match raw_body_text {
            Some(text) if HtmlExtractor::is_text_clean(&text) => {
                // body_text 干净，直接使用
                Some(text)
            }
            Some(text) if text.trim().is_empty() && raw_body_html.is_some() => {
                // body_text 为空但有 HTML，从 HTML 提取
                raw_body_html.as_ref().map(|html| HtmlExtractor::extract_clean_text(html))
            }
            Some(_) if raw_body_html.is_some() => {
                // body_text 不干净（含 CSS/JS），从 HTML 提取干净文本
                log::info!("[EmailParser] body_text 包含 CSS/JS 干扰，使用 scraper 提取干净文本");
                raw_body_html.as_ref().map(|html| HtmlExtractor::extract_clean_text(html))
            }
            _ => raw_body_text, // 其他情况保持原样
        };

        // Attachments - 收集所有附件（包括内嵌图片）
        let all_attachments: Vec<EmailAttachment> = msg
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

        result.has_attachments = !all_attachments.is_empty();

        // 先收集 HTML 中被引用的 cid（在替换前检查）
        let referenced_cids: Vec<String> = if let Some(html) = &raw_body_html {
            all_attachments
                .iter()
                .filter_map(|att| att.content_id.clone())
                .filter(|cid| {
                    // 检查原始 HTML 中是否有 cid:xxx 或 cid=<xxx> 引用
                    html.contains(&format!("cid:{}", cid)) ||
                    html.contains(&format!("cid:<{}>", cid)) ||
                    // 也检查不带 cid: 前缀的直接引用（某些邮件格式）
                    html.contains(&format!("src=\"{}\"", cid)) ||
                    html.contains(&format!("src='{}'", cid))
                })
                .collect()
        } else {
            vec![]
        };

        log::info!("[EmailParser] HTML 中引用的内嵌图片 cid: {:?}", referenced_cids);

        // 处理 body_html 中的 cid: 内嵌图片引用
        // 将 cid:xxx 替换为 data:image/xxx;base64,... 格式
        result.body_html = raw_body_html.map(|html| {
            replace_cid_images_with_base64(&html, &all_attachments)
        });

        // 分离内嵌附件和普通附件
        // 内嵌附件（有 content_id 且已在 HTML 中被引用）不作为独立附件显示
        result.attachments = all_attachments
            .into_iter()
            .filter(|att| {
                // 没有 content_id 的附件作为普通附件显示
                if att.content_id.is_none() {
                    return true;
                }
                // 有 content_id 但未被 HTML 引用的也显示
                // 已被引用的不显示（已在正文中展示）
                let cid = att.content_id.as_ref().unwrap();
                !referenced_cids.contains(cid)
            })
            .collect();

        log::info!("[EmailParser] 最终附件数量: {} (过滤了 {} 个内嵌图片)",
            result.attachments.len(), referenced_cids.len());
    }

    result
}

/// 替换 HTML 中的 cid: 图片引用为 base64 data URI
fn replace_cid_images_with_base64(html: &str, attachments: &[EmailAttachment]) -> String {
    let mut result = html.to_string();

    for att in attachments {
        if let Some(content_id) = &att.content_id {
            if let Some(content) = &att.content {
                // 构建 data URI
                let data_uri = format!("data:{};base64,{}", att.mime_type, content);

                // 替换 src="cid:xxx" 和 src='cid:xxx' 格式
                // 同时处理带 <> 的 content-id 格式（如 cid:<xxx>）
                let cid_patterns = [
                    format!("src=\"cid:{}\"", content_id),
                    format!("src='cid:{}'", content_id),
                    format!("src=\"cid:<{}>\"", content_id),
                    format!("src='cid:<{}>'", content_id),
                ];

                for pattern in cid_patterns {
                    let replacement = format!("src=\"{}\"", data_uri);
                    result = result.replace(&pattern, &replacement);
                }
            }
        }
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

/// Delete a single email by ID
#[tauri::command]
pub async fn delete_email(id: String) -> Result<(), String> {
    let db = get_db().await?;
    db.delete_email(&id).await
}

// ==================== 回收站功能命令 ====================

/// Move email to trash (soft delete)
#[tauri::command]
pub async fn move_to_trash(id: String) -> Result<(), String> {
    let db = get_db().await?;
    db.move_to_trash(&id).await
}

/// Get trash emails for an account
#[tauri::command]
pub async fn get_trash_emails(account_id: String) -> Result<Vec<Email>, String> {
    let db = get_db().await?;
    let db_emails = db.get_trash_emails(&account_id).await?;

    let emails: Vec<Email> = db_emails
        .into_iter()
        .map(|e| {
            let to_addresses: Vec<String> = serde_json::from_str(&e.to_addresses).unwrap_or_default();
            let snippet = e.body_text
                .as_ref()
                .map(|t| t.chars().take(100).collect())
                .unwrap_or_default();

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

/// Get trash count for an account
#[tauri::command]
pub async fn get_trash_count(account_id: String) -> Result<i32, String> {
    let db = get_db().await?;
    db.get_trash_count(&account_id).await
}

/// Restore email from trash to inbox or phishing folder
#[tauri::command]
pub async fn restore_email(id: String, target_folder: String) -> Result<(), String> {
    let db = get_db().await?;
    let is_phishing = target_folder == "phishing";
    db.restore_email(&id, is_phishing).await
}

/// Permanently delete email from trash
#[tauri::command]
pub async fn permanent_delete(id: String) -> Result<(), String> {
    let db = get_db().await?;
    db.permanent_delete(&id).await
}

/// Cleanup expired trash emails (called on app startup)
#[tauri::command]
pub async fn cleanup_expired_trash() -> Result<u64, String> {
    let db = get_db().await?;
    db.cleanup_expired_trash().await
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
    user_overridden: Option<bool>,
) -> Result<(), String> {
    let db = get_db().await?;

    // 将原因数组转换为 JSON 字符串存储
    let reasons_json = phishing_reasons
        .map(|r| serde_json::to_string(&r).unwrap_or_default());

    db.update_phishing_status(&id, is_phishing, reasons_json, user_overridden).await
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

/// Get starred emails for an account
#[tauri::command]
pub async fn get_starred_emails(account_id: String) -> Result<Vec<Email>, String> {
    let db = get_db().await?;
    let db_emails = db.get_starred_emails(&account_id).await?;

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

/// Get starred email count for an account
#[tauri::command]
pub async fn get_starred_count(account_id: String) -> Result<i32, String> {
    let db = get_db().await?;
    db.get_starred_count(&account_id).await
}

// ==================== 已发送/草稿功能命令 ====================

/// Get sent emails for an account
#[tauri::command]
pub async fn get_sent_emails(account_id: String) -> Result<Vec<Email>, String> {
    let db = get_db().await?;
    let db_emails = db.get_sent_emails(&account_id).await?;

    let emails: Vec<Email> = db_emails
        .into_iter()
        .map(|e| {
            let to_addresses: Vec<String> = serde_json::from_str(&e.to_addresses).unwrap_or_default();
            let snippet = e.body_text
                .as_ref()
                .map(|t| t.chars().take(100).collect())
                .unwrap_or_default();

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
                phishing_reasons: None, // 已发送邮件无钓鱼检测
            }
        })
        .collect();

    Ok(emails)
}

/// Get sent email count for an account
#[tauri::command]
pub async fn get_sent_count(account_id: String) -> Result<i32, String> {
    let db = get_db().await?;
    db.get_sent_count(&account_id).await
}

/// Get draft emails for an account
#[tauri::command]
pub async fn get_draft_emails(account_id: String) -> Result<Vec<Email>, String> {
    let db = get_db().await?;
    let db_emails = db.get_draft_emails(&account_id).await?;

    let emails: Vec<Email> = db_emails
        .into_iter()
        .map(|e| {
            let to_addresses: Vec<String> = serde_json::from_str(&e.to_addresses).unwrap_or_default();
            let snippet = e.body_text
                .as_ref()
                .map(|t| t.chars().take(100).collect())
                .unwrap_or_default();

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
                is_phishing: false,
                phishing_reasons: None,
            }
        })
        .collect();

    Ok(emails)
}

/// Get draft email count for an account
#[tauri::command]
pub async fn get_draft_count(account_id: String) -> Result<i32, String> {
    let db = get_db().await?;
    db.get_draft_count(&account_id).await
}

/// Draft attachment data from frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DraftAttachmentData {
    pub id: String,
    pub filename: String,
    pub content_type: Option<String>,
    pub file_path: Option<String>,
    /// Base64 编码的附件内容（直接存储）
    pub content: Option<String>,
    pub file_size: i64,
}

/// Draft data for creating/updating drafts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DraftData {
    pub account_id: String,
    pub to_addresses: Vec<String>,
    pub cc_addresses: Vec<String>,
    pub subject: Option<String>,
    pub body_text: Option<String>,
    pub body_html: Option<String>,
    pub attachments: Option<Vec<DraftAttachmentData>>,
}

/// Create a draft email
#[tauri::command]
pub async fn create_draft(draft: DraftData) -> Result<String, String> {
    let db = get_db().await?;

    // 调试日志：记录附件数据
    log::info!("[create_draft] 开始创建草稿");
    if let Some(atts) = &draft.attachments {
        log::info!("[create_draft] 附件数量: {}", atts.len());
        for att in atts {
            log::info!("[create_draft]   - id: {}, filename: {}, file_path: {:?}, file_size: {}",
                att.id, att.filename, att.file_path, att.file_size);
        }
    } else {
        log::info!("[create_draft] 无附件");
    }

    // 获取账户信息以设置 from_address
    let accounts = get_accounts()?;
    let account = accounts
        .iter()
        .find(|a| a.id == draft.account_id)
        .ok_or_else(|| "账户不存在".to_string())?;

    let draft_id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().timestamp();

    // 判断是否有附件
    let has_attachment = draft.attachments.is_some() && !draft.attachments.as_ref().unwrap().is_empty();

    let db_draft = DbEmail {
        id: format!("draft-{}", draft_id),
        account_id: draft.account_id.clone(),
        uidl: format!("draft-{}", draft_id),
        folder: "drafts".to_string(),
        draft_id: Some(draft_id.clone()),
        from_address: Some(account.email.clone()),
        from_name: account.display_name.clone(),
        to_addresses: serde_json::to_string(&draft.to_addresses).unwrap_or_default(),
        cc_addresses: serde_json::to_string(&draft.cc_addresses).unwrap_or_default(),
        subject: draft.subject,
        body_text: draft.body_text,
        body_html: draft.body_html,
        date: now,
        is_read: true,
        is_starred: false,
        has_attachment,
        is_phishing: false,
        phishing_reasons: None,
        user_overridden: false,
        deleted_at: None,
        // 草稿邮件无 threading headers
        message_id_header: None,
        in_reply_to_header: None,
        references_header: None,
        thread_id: None,
    };

    db.create_draft(&db_draft).await?;
    log::info!("[create_draft] 草稿邮件已保存, id: {}", db_draft.id);

    // 保存草稿附件（使用草稿邮件的 id）
    if let Some(attachments) = &draft.attachments {
        for att in attachments {
            log::info!("[create_draft] 保存附件到数据库: id={}, filename={}, content长度={}",
                att.id, att.filename, att.content.as_ref().map(|c| c.len()).unwrap_or(0));
            let db_att = DbDraftAttachment {
                id: att.id.clone(),
                email_id: db_draft.id.clone(),  // 使用草稿邮件的 id
                filename: att.filename.clone(),
                content_type: att.content_type.clone(),
                file_path: att.file_path.clone(),
                content: att.content.clone(),
                file_size: att.file_size,
                created_at: now,
            };
            db.save_draft_attachment(&db_att).await?;
            log::info!("[create_draft] 附件保存成功: {}", att.filename);
        }
    }

    log::info!("[create_draft] 草稿创建完成, draft_id: {}", draft_id);
    Ok(draft_id)
}

/// Update a draft email
#[tauri::command]
pub async fn update_draft(draft_id: String, draft: DraftData) -> Result<(), String> {
    let db = get_db().await?;

    // 调试日志
    log::info!("[update_draft] 开始更新草稿, draft_id: {}", draft_id);
    if let Some(atts) = &draft.attachments {
        log::info!("[update_draft] 附件数量: {}", atts.len());
        for att in atts {
            log::info!("[update_draft]   - id: {}, filename: {}, file_path: {:?}, file_size: {}",
                att.id, att.filename, att.file_path, att.file_size);
        }
    } else {
        log::info!("[update_draft] 无附件");
    }

    // 判断是否有附件
    let has_attachment = draft.attachments.is_some() && !draft.attachments.as_ref().unwrap().is_empty();

    let updates = DraftUpdates {
        to_addresses: serde_json::to_string(&draft.to_addresses).unwrap_or_default(),
        cc_addresses: serde_json::to_string(&draft.cc_addresses).unwrap_or_default(),
        subject: draft.subject,
        body_text: draft.body_text,
        body_html: draft.body_html,
        has_attachment,
    };

    db.update_draft(&draft_id, &updates).await?;

    // 获取草稿邮件的 id（用于附件查询）
    let email_id = format!("draft-{}", draft_id);

    // 删除旧附件并保存新附件
    db.delete_draft_attachments(&email_id).await?;
    if let Some(attachments) = &draft.attachments {
        let now = chrono::Utc::now().timestamp();
        for att in attachments {
            log::info!("[update_draft] 保存附件到数据库: id={}, filename={}, content长度={}",
                att.id, att.filename, att.content.as_ref().map(|c| c.len()).unwrap_or(0));
            let db_att = DbDraftAttachment {
                id: att.id.clone(),
                email_id: email_id.clone(),  // 使用草稿邮件的 id
                filename: att.filename.clone(),
                content_type: att.content_type.clone(),
                file_path: att.file_path.clone(),
                content: att.content.clone(),
                file_size: att.file_size,
                created_at: now,
            };
            db.save_draft_attachment(&db_att).await?;
        }
    }

    log::info!("[update_draft] 草稿更新完成");
    Ok(())
}

/// Delete a draft email
#[tauri::command]
pub async fn delete_draft(draft_id: String) -> Result<(), String> {
    let db = get_db().await?;
    db.delete_draft(&draft_id).await
}

/// Send a draft email
#[tauri::command]
pub async fn send_draft(
    draft_id: String,
    signature_html: Option<String>,
    attachments: Option<Vec<AttachmentData>>,
) -> Result<SendEmailResult, String> {
    let db = get_db().await?;

    // 调试日志
    log::info!("[send_draft] 开始发送草稿, draft_id: {}", draft_id);
    if let Some(atts) = &attachments {
        log::info!("[send_draft] 前端传递的附件数量: {}", atts.len());
        for att in atts {
            log::info!("[send_draft]   - 附件: {} ({}), content长度: {} bytes",
                att.filename, att.content_type, att.content.len());
        }
    } else {
        log::info!("[send_draft] 前端未传递附件");
    }

    // 获取草稿内容
    let draft = db.get_draft_by_id(&draft_id).await?
        .ok_or_else(|| "草稿不存在".to_string())?;

    // 获取账户配置
    let accounts = get_accounts()?;
    let account = accounts
        .iter()
        .find(|a| a.id == draft.account_id)
        .ok_or_else(|| "账户不存在".to_string())?;

    // 检查 SMTP 配置
    if account.smtp_host.is_empty() {
        return Err("账户未配置 SMTP 服务器".to_string());
    }

    // 创建 SMTP 客户端
    let config = SmtpConfig {
        host: account.smtp_host.clone(),
        port: account.smtp_port,
        use_ssl: account.smtp_use_ssl,
    };
    let credentials = SmtpCredentials {
        username: account.email.clone(),
        password: account.password.clone(),
    };

    let client = SmtpClient::new(config, credentials);

    // 解析收件人
    let to_addresses: Vec<String> = serde_json::from_str(&draft.to_addresses).unwrap_or_default();
    let cc_addresses: Vec<String> = serde_json::from_str(&draft.cc_addresses).unwrap_or_default();

    // 处理签名
    let final_body_html = if let Some(html) = draft.body_html.clone() {
        if let Some(sig) = signature_html {
            if !sig.is_empty() {
                Some(format!(
                    "{}<div style=\"margin-top:16px;border-top:1px solid #e5e5e5;padding-top:12px\">{}</div>",
                    html, sig
                ))
            } else {
                Some(html)
            }
        } else {
            Some(html)
        }
    } else {
        None
    };

    // 使用前端传递的附件（优先），如果没有则从数据库读取草稿附件
    let final_attachments: Option<Vec<AttachmentData>> = if attachments.is_some() && !attachments.as_ref().unwrap().is_empty() {
        log::info!("[send_draft] 使用前端传递的附件");
        attachments
    } else {
        // 读取草稿附件（使用草稿邮件的 id）
        log::info!("[send_draft] 从数据库读取草稿附件, draft.id: {}", draft.id);
        let draft_attachments = db.get_draft_attachments(&draft.id).await?;
        log::info!("[send_draft] 数据库中草稿附件数量: {}", draft_attachments.len());

        let atts: Vec<AttachmentData> = draft_attachments
            .into_iter()
            .filter_map(|att| {
                let filename = att.filename.clone();
                let content_type = att.content_type.clone().unwrap_or_else(|| {
                    infer_mime_type(&filename)
                });

                // 如果数据库中已存储 content，直接使用
                if let Some(content) = &att.content {
                    if !content.is_empty() {
                        log::info!("[send_draft]   - 使用数据库中的 content: {} (长度: {})", filename, content.len());
                        return Some(AttachmentData {
                            filename,
                            content_type,
                            content: content.clone(),
                        });
                    }
                }

                // 否则尝试从 file_path 读取
                if let Some(file_path) = &att.file_path {
                    if !file_path.is_empty() && std::path::Path::new(file_path).exists() {
                        match std::fs::read(file_path) {
                            Ok(content) => {
                                use base64::{Engine as _, engine::general_purpose::STANDARD};
                                let encoded = STANDARD.encode(&content);
                                log::info!("[send_draft]   - 从文件读取: {} (长度: {})", filename, encoded.len());
                                return Some(AttachmentData {
                                    filename,
                                    content_type,
                                    content: encoded,
                                });
                            }
                            Err(e) => {
                                log::warn!("读取草稿附件文件失败: {} - {}", file_path, e);
                                return None;
                            }
                        }
                    }
                }

                log::warn!("草稿附件无有效 content 或 file_path: {}", filename);
                None
            })
            .collect();

        if atts.is_empty() {
            None
        } else {
            Some(atts)
        }
    };

    // 判断是否有附件
    let has_attachment = final_attachments.is_some() && !final_attachments.as_ref().unwrap().is_empty();

    // 提前保存附件信息（用于发送成功后保存到数据库）
    let attachments_info: Vec<(String, String, String, i64)> = if has_attachment {
        final_attachments.as_ref().unwrap().iter().map(|att| {
            let base64_len = att.content.len();
            let padding = if att.content.ends_with("==") { 2 } else if att.content.ends_with("=") { 1 } else { 0 };
            let size = ((base64_len - padding) * 3 / 4) as i64;
            (att.filename.clone(), att.content_type.clone(), att.content.clone(), size)
        }).collect()
    } else {
        vec![]
    };

    // 创建邮件消息
    let email = EmailMessage {
        from_address: account.email.clone(),
        from_name: account.display_name.clone(),
        to_addresses,
        cc_addresses: Some(cc_addresses),
        subject: draft.subject.clone().unwrap_or_default(),
        body: draft.body_text.clone().unwrap_or_default(),
        body_html: final_body_html,
        attachments: final_attachments,
    };

    // 发送邮件
    let result = client.send_email(email).await;

    // 发送成功后标记草稿为已发送并保存附件
    if result.success {
        let sent_uidl = format!("sent-{}", chrono::Utc::now().timestamp_millis());
        db.mark_draft_as_sent(&draft_id, &sent_uidl).await?;

        // 保存附件信息到数据库并缓存附件内容
        if has_attachment {
            let sent_id = format!("draft-{}", draft_id); // 使用草稿邮件的 id
            let cache_dir = get_cache_directory()?;
            for (filename, content_type, content_base64, size) in attachments_info {
                let att_id = format!("att-{}", uuid::Uuid::new_v4());

                // 将附件内容解码并保存到缓存目录
                let cache_path = cache_dir.join(format!("{}.dat", att_id));
                let is_downloaded = match base64::Engine::decode(&base64::engine::general_purpose::STANDARD, &content_base64) {
                    Ok(decoded) => {
                        match std::fs::write(&cache_path, &decoded) {
                            Ok(_) => true,
                            Err(e) => {
                                log::warn!("保存附件缓存失败: {} - {}", filename, e);
                                false
                            }
                        }
                    }
                    Err(e) => {
                        log::warn!("Base64 解码失败: {} - {}", filename, e);
                        false
                    }
                };

                let local_path = if is_downloaded {
                    Some(cache_path.to_string_lossy().to_string())
                } else {
                    None
                };

                let db_att = DbAttachment {
                    id: att_id,
                    email_id: sent_id.clone(),
                    filename,
                    mime_type: content_type,
                    size,
                    content_id: None,
                    imap_section: None,
                    is_downloaded,
                    local_path,
                };

                if let Err(e) = db.save_attachment(&db_att).await {
                    log::warn!("保存附件记录失败: {}", e);
                }
            }
        }
    }

    Ok(result)
}

/// Infer MIME type from filename
fn infer_mime_type(filename: &str) -> String {
    let ext = filename.rsplit('.').next().unwrap_or("").to_lowercase();
    match ext.as_str() {
        "pdf" => "application/pdf",
        "doc" => "application/msword",
        "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        "xls" => "application/vnd.ms-excel",
        "xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        "ppt" => "application/vnd.ms-powerpoint",
        "pptx" => "application/vnd.openxmlformats-officedocument.presentationml.presentation",
        "zip" => "application/zip",
        "rar" => "application/vnd.rar",
        "7z" => "application/x-7z-compressed",
        "tar" => "application/x-tar",
        "gz" => "application/gzip",
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "gif" => "image/gif",
        "bmp" => "image/bmp",
        "webp" => "image/webp",
        "svg" => "image/svg+xml",
        "txt" => "text/plain",
        "html" | "htm" => "text/html",
        "csv" => "text/csv",
        "json" => "application/json",
        "xml" => "application/xml",
        "mp3" => "audio/mpeg",
        "mp4" => "video/mp4",
        "avi" => "video/x-msvideo",
        "mov" => "video/quicktime",
        "wav" => "audio/wav",
        _ => "application/octet-stream",
    }.to_string()
}

/// Get draft detail by draft_id
#[tauri::command]
pub async fn get_draft_detail(draft_id: String) -> Result<Option<EmailDetail>, String> {
    let db = get_db().await?;
    let draft = db.get_draft_by_id(&draft_id).await?;

    if let Some(e) = draft {
        // 获取草稿附件（使用草稿邮件的 id）
        let draft_attachments = db.get_draft_attachments(&e.id).await?;
        let attachments: Vec<AttachmentInfo> = draft_attachments
            .into_iter()
            .map(|att| {
                let filename = att.filename.clone();
                let mime_type = att.content_type.clone().unwrap_or_else(|| infer_mime_type(&filename));
                AttachmentInfo {
                    id: att.id,
                    filename,
                    mime_type,
                    size: att.file_size,
                }
            })
            .collect();

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
            attachments,
            is_phishing: false,
            phishing_reasons: None,
        }))
    } else {
        Ok(None)
    }
}

/// Get draft attachments for frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DraftAttachmentInfo {
    pub id: String,
    pub filename: String,
    pub content_type: Option<String>,
    pub file_path: Option<String>,
    /// Base64 编码的附件内容
    pub content: Option<String>,
    pub file_size: i64,
}

/// Get draft attachments by draft_id
#[tauri::command]
pub async fn get_draft_attachments(draft_id: String) -> Result<Vec<DraftAttachmentInfo>, String> {
    let db = get_db().await?;

    // 转换 draft_id 为草稿邮件的 id
    let email_id = format!("draft-{}", draft_id);
    log::info!("[get_draft_attachments] draft_id={}, email_id={}", draft_id, email_id);

    let attachments = db.get_draft_attachments(&email_id).await?;
    log::info!("[get_draft_attachments] 找到 {} 个附件", attachments.len());

    for att in &attachments {
        log::info!("[get_draft_attachments]   - id: {}, filename: {}, content长度: {}, file_size: {}",
            att.id, att.filename, att.content.as_ref().map(|c| c.len()).unwrap_or(0), att.file_size);
    }

    Ok(attachments
        .into_iter()
        .map(|att| DraftAttachmentInfo {
            id: att.id,
            filename: att.filename,
            content_type: att.content_type,
            file_path: att.file_path,
            content: att.content,
            file_size: att.file_size,
        })
        .collect())
}

/// Debug: 查看数据库中所有草稿附件
#[tauri::command]
pub async fn debug_list_all_draft_attachments() -> Result<Vec<(String, String, String)>, String> {
    let db = get_db().await?;

    // 直接使用数据库的公开方法来查询
    // 先获取所有草稿邮件的 id，然后查询每个的附件
    let rows = sqlx::query_as::<_, (String, String, String)>(
        "SELECT id, email_id, filename FROM draft_attachments ORDER BY created_at DESC LIMIT 20"
    )
    .fetch_all(db.get_pool())
    .await
    .map_err(|e| format!("查询失败: {}", e))?;

    Ok(rows)
}

/// Debug: 查看草稿邮件的 id 和 draft_id
#[tauri::command]
pub async fn debug_list_draft_email_ids() -> Result<Vec<(String, Option<String>)>, String> {
    let db = get_db().await?;

    let rows = sqlx::query_as::<_, (String, Option<String>)>(
        "SELECT id, draft_id FROM emails WHERE folder = 'drafts' ORDER BY date DESC LIMIT 10"
    )
    .fetch_all(db.get_pool())
    .await
    .map_err(|e| format!("查询失败: {}", e))?;

    Ok(rows)
}
#[tauri::command]
pub async fn read_file_for_attachment(file_path: String) -> Result<AttachmentData, String> {
    // 检查文件是否存在
    if !std::path::Path::new(&file_path).exists() {
        return Err(format!("文件不存在: {}", file_path));
    }

    // 获取文件名
    let filename = std::path::Path::new(&file_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    // 读取文件内容
    let content = std::fs::read(&file_path)
        .map_err(|e| format!("读取文件失败: {}", e))?;

    // 编码为 Base64
    use base64::{Engine as _, engine::general_purpose::STANDARD};
    let encoded = STANDARD.encode(&content);

    // 推断 MIME 类型
    let content_type = infer_mime_type(&filename);

    Ok(AttachmentData {
        filename,
        content_type,
        content: encoded,
    })
}

/// Validate attachment file path (check if file exists)
#[tauri::command]
pub async fn validate_attachment_path(file_path: String) -> Result<bool, String> {
    Ok(std::path::Path::new(&file_path).exists())
}

/// Send an email
#[tauri::command]
pub async fn send_email(
    account_id: String,
    to_addresses: Vec<String>,
    cc_addresses: Option<Vec<String>>,
    subject: String,
    body: String,
    body_html: Option<String>,
    signature_html: Option<String>,
    attachments: Option<Vec<AttachmentData>>,
) -> Result<SendEmailResult, String> {
    // 调试日志：记录附件数据
    log::info!("[send_email] 开始发送邮件");
    log::info!("[send_email] subject: {}", subject);
    log::info!("[send_email] to_addresses: {:?}", to_addresses);
    if let Some(atts) = &attachments {
        log::info!("[send_email] 附件数量: {}", atts.len());
        for att in atts {
            log::info!("[send_email]   - 附件: {} ({}), content长度: {} bytes",
                att.filename, att.content_type, att.content.len());
        }
    } else {
        log::info!("[send_email] 无附件");
    }

    // Get account configuration
    let accounts = get_accounts()?;
    let account = accounts
        .iter()
        .find(|a| a.id == account_id)
        .ok_or_else(|| "账户不存在".to_string())?;

    // Check SMTP configuration
    if account.smtp_host.is_empty() {
        return Err("账户未配置 SMTP 服务器".to_string());
    }

    // Create SMTP client
    let config = SmtpConfig {
        host: account.smtp_host.clone(),
        port: account.smtp_port,
        use_ssl: account.smtp_use_ssl,
    };
    let credentials = SmtpCredentials {
        username: account.email.clone(),
        password: account.password.clone(),
    };

    let client = SmtpClient::new(config, credentials);

    // Process body HTML with signature
    let final_body_html = if let Some(html) = body_html {
        // Append signature if provided
        if let Some(sig) = signature_html {
            if !sig.is_empty() {
                Some(format!(
                    "{}<div style=\"margin-top:16px;border-top:1px solid #e5e5e5;padding-top:12px\">{}</div>",
                    html, sig
                ))
            } else {
                Some(html)
            }
        } else {
            Some(html)
        }
    } else if let Some(sig) = signature_html {
        // If no HTML body but signature exists, create HTML with signature
        if !sig.is_empty() {
            Some(format!(
                "<p>{}</p><div style=\"margin-top:16px;border-top:1px solid #e5e5e5;padding-top:12px\">{}</div>",
                body.replace("\n", "<br/>"), sig
            ))
        } else {
            None
        }
    } else {
        None
    };

    // 判断是否有附件
    let has_attachment = attachments.is_some() && !attachments.as_ref().unwrap().is_empty();

    // 提前保存附件信息（用于发送成功后保存到数据库）
    let attachments_info: Vec<(String, String, String, i64)> = if has_attachment {
        attachments.as_ref().unwrap().iter().map(|att| {
            let base64_len = att.content.len();
            let padding = if att.content.ends_with("==") { 2 } else if att.content.ends_with("=") { 1 } else { 0 };
            let size = ((base64_len - padding) * 3 / 4) as i64;
            (att.filename.clone(), att.content_type.clone(), att.content.clone(), size)
        }).collect()
    } else {
        vec![]
    };

    // Create email message
    let email = EmailMessage {
        from_address: account.email.clone(),
        from_name: account.display_name.clone(),
        to_addresses: to_addresses.clone(),
        cc_addresses: cc_addresses.clone(),
        subject: subject.clone(),
        body: body.clone(),
        body_html: final_body_html.clone(),
        attachments,
    };

    // Send email
    let result = client.send_email(email).await;

    // 发送成功后保存到本地数据库（已发送文件夹）
    if result.success {
        let db = get_db().await?;
        let now = chrono::Utc::now().timestamp();
        let sent_id = format!("sent-{}", now);

        let sent_email = DbEmail {
            id: sent_id.clone(),
            account_id: account_id.clone(),
            uidl: format!("sent-{}", now),
            folder: "sent".to_string(),
            draft_id: None,
            from_address: Some(account.email.clone()),
            from_name: account.display_name.clone(),
            to_addresses: serde_json::to_string(&to_addresses).unwrap_or_default(),
            cc_addresses: serde_json::to_string(&cc_addresses.unwrap_or_default()).unwrap_or_default(),
            subject: Some(subject),
            body_text: Some(body),
            body_html: final_body_html,
            date: now,
            is_read: true,
            is_starred: false,
            has_attachment,
            is_phishing: false,
            phishing_reasons: None,
            user_overridden: false,
            deleted_at: None,
            // 已发送邮件无 threading headers
            message_id_header: None,
            in_reply_to_header: None,
            references_header: None,
            thread_id: None,
        };

        if let Err(e) = db.save_email(&sent_email).await {
            log::warn!("保存已发送邮件到本地数据库失败: {}", e);
        }

        // 保存附件信息到数据库并缓存附件内容
        if has_attachment {
            let cache_dir = get_cache_directory()?;
            for (filename, content_type, content_base64, size) in attachments_info {
                let att_id = format!("att-{}", uuid::Uuid::new_v4());

                // 将附件内容解码并保存到缓存目录
                let cache_path = cache_dir.join(format!("{}.dat", att_id));
                let is_downloaded = match base64::Engine::decode(&base64::engine::general_purpose::STANDARD, &content_base64) {
                    Ok(decoded) => {
                        match std::fs::write(&cache_path, &decoded) {
                            Ok(_) => true,
                            Err(e) => {
                                log::warn!("保存附件缓存失败: {} - {}", filename, e);
                                false
                            }
                        }
                    }
                    Err(e) => {
                        log::warn!("Base64 解码失败: {} - {}", filename, e);
                        false
                    }
                };

                let local_path = if is_downloaded {
                    Some(cache_path.to_string_lossy().to_string())
                } else {
                    None
                };

                let db_att = DbAttachment {
                    id: att_id,
                    email_id: sent_id.clone(),
                    filename,
                    mime_type: content_type,
                    size,
                    content_id: None,
                    imap_section: None,
                    is_downloaded,
                    local_path,
                };

                if let Err(e) = db.save_attachment(&db_att).await {
                    log::warn!("保存附件记录失败: {}", e);
                }
            }
        }
    }

    Ok(result)
}

// ==================== 附件懒加载相关命令 ====================

/// 下载进度事件 payload
#[derive(Clone, Serialize)]
pub struct AttachmentProgress {
    pub attachment_id: String,
    pub downloaded: u64,
    pub total: u64,
    pub percent: u8,
}

/// 下载完成事件 payload
#[derive(Clone, Serialize)]
pub struct AttachmentComplete {
    pub attachment_id: String,
    pub local_path: String,
}

/// 缓存统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    pub file_count: u32,
    pub total_size: u64,
    pub cache_path: String,
}

/// 附件信息（扩展版，包含下载状态）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachmentInfoWithStatus {
    pub id: String,
    pub filename: String,
    pub mime_type: String,
    pub size: i64,
    pub is_downloaded: bool,
    pub local_path: Option<String>,
}

/// Download a single attachment using velo's approach:
/// Fetch full message, parse with mail-parser, extract decoded attachment directly.
/// This avoids IMAP section complexity and ensures content-transfer-encoding is decoded.
#[tauri::command]
pub async fn download_attachment(
    attachment_id: String,
    app_handle: tauri::AppHandle,
) -> Result<String, String> {
    let db = get_db().await?;
    let att = db.get_attachment_by_id(&attachment_id).await?
        .ok_or_else(|| "附件不存在".to_string())?;

    // 已下载 → 直接返回路径
    if att.is_downloaded {
        return Ok(att.local_path.unwrap_or_default());
    }

    // 获取邮件和账户信息
    let email = db.get_email_by_id(&att.email_id).await?
        .ok_or_else(|| "邮件不存在".to_string())?;

    let accounts = get_accounts()?;
    let account = accounts
        .iter()
        .find(|a| a.id == email.account_id)
        .ok_or_else(|| "账户不存在".to_string())?;

    // 连接 IMAP
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
    let mut session = client.connect().await?;
    session.select_inbox().await?;

    // 获取 UID
    let uid: u32 = email.uidl.parse()
        .map_err(|_| "邮件 UID 格式错误".to_string())?;

    // 获取完整邮件（velo 方法）
    log::info!("[Attachment] 获取完整邮件以解析附件 UID={}", uid);
    let full_email = session.fetch_email(uid).await?;

    // Logout - 尽早关闭连接
    session.logout().await?;

    // 使用 mail-parser 解析完整邮件
    let parser = mail_parser::MessageParser::new();
    let message = parser.parse(full_email.raw_content.as_bytes())
        .ok_or_else(|| "无法解析邮件内容".to_string())?;

    // 直接从 mail-parser 解析结果中提取附件（无需 section 计算）
    // mail-parser 已完成 content-transfer-encoding 解码
    let content = extract_attachment_from_parsed_message(
        &message,
        &att.filename,
        att.content_id.as_deref(),
    )?;

    log::info!("[Attachment] 附件 '{}' 内容提取成功，大小 {} bytes", att.filename, content.len());

    // 创建缓存文件
    let cache_path = get_attachment_cache_path(&attachment_id)?;
    let mut file = std::fs::File::create(&cache_path)
        .map_err(|e| format!("创建缓存文件失败: {}", e))?;

    // 写入文件并发送进度事件
    let total = content.len() as u64;
    let chunk_size = 64 * 1024; // 64KB
    let mut downloaded = 0u64;

    for chunk in content.chunks(chunk_size) {
        use std::io::Write;
        file.write_all(chunk)
            .map_err(|e| format!("写入文件失败: {}", e))?;
        downloaded += chunk.len() as u64;

        let percent = if total > 0 {
            ((downloaded * 100 / total).min(100)) as u8
        } else {
            100
        };

        // 发送进度事件
        app_handle.emit("attachment-progress", AttachmentProgress {
            attachment_id: attachment_id.clone(),
            downloaded,
            total,
            percent,
        }).ok();
    }

    // 更新数据库
    let local_path_str = cache_path.to_string_lossy().to_string();
    db.update_attachment_downloaded(&attachment_id, &local_path_str).await?;

    // 发送完成事件
    let _ = app_handle.emit("attachment-complete", AttachmentComplete {
        attachment_id,
        local_path: local_path_str.clone(),
    });

    Ok(local_path_str)
}

/// 从已解析的 mail-parser Message 中提取指定附件的内容
/// mail-parser 已完成 content-transfer-encoding 解码，直接返回原始二进制数据
fn extract_attachment_from_parsed_message(
    message: &mail_parser::Message<'_>,
    target_filename: &str,
    target_content_id: Option<&str>,
) -> Result<Vec<u8>, String> {
    use mail_parser::PartType;

    // 遍历所有 parts 找到匹配的附件
    for part in message.parts.iter() {
        // 只处理 leaf parts（非 multipart）
        match &part.body {
            PartType::Multipart(_) => continue, // 跳过 multipart 容器
            PartType::Binary(data) | PartType::InlineBinary(data) => {
                // 检查是否匹配目标附件
                let filename = part.attachment_name().map(|s| s.to_string());
                let content_id = part.content_id().map(|s| s.to_string());

                // 精确匹配 filename 或 content_id
                let filename_match = filename.as_ref() == Some(&target_filename.to_string());
                let content_id_match = target_content_id.is_some() &&
                    content_id.as_ref() == Some(&target_content_id.unwrap().to_string());

                if filename_match || content_id_match {
                    log::info!("[Attachment] 精确匹配附件: filename={}, cid={}",
                        filename.as_deref().unwrap_or("无"),
                        content_id.as_deref().unwrap_or("无"));
                    return Ok(data.as_ref().to_vec());
                }

                // 模糊匹配 filename
                if let Some(fn_name) = &filename {
                    if fn_name.contains(target_filename) || target_filename.contains(fn_name) {
                        log::info!("[Attachment] 模糊匹配附件: {} ~= {}", fn_name, target_filename);
                        return Ok(data.as_ref().to_vec());
                    }
                }
            }
            PartType::Text(_) | PartType::Html(_) => {
                // 文本内容可能是附件（如 .txt 文件）
                let filename = part.attachment_name().map(|s| s.to_string());
                let content_id = part.content_id().map(|s| s.to_string());

                let filename_match = filename.as_ref() == Some(&target_filename.to_string());
                let content_id_match = target_content_id.is_some() &&
                    content_id.as_ref() == Some(&target_content_id.unwrap().to_string());

                if filename_match || content_id_match {
                    log::info!("[Attachment] 匹配文本附件: filename={}", filename.as_deref().unwrap_or("无"));
                    // 根据实际类型获取内容
                    let bytes = match &part.body {
                        PartType::Text(t) => t.as_bytes().to_vec(),
                        PartType::Html(h) => h.as_bytes().to_vec(),
                        _ => unreachable!(),
                    };
                    return Ok(bytes);
                }
            }
            PartType::Message(msg) => {
                // 嵌套邮件 - 通常不是我们要找的附件
                let filename = part.attachment_name().map(|s| s.to_string());
                if filename.as_ref() == Some(&target_filename.to_string()) {
                    return Ok(msg.raw_message.as_ref().to_vec());
                }
            }
        }
    }

    // 如果精确匹配失败，尝试使用 attachments() 方法
    // 这是 mail-parser 提供的便捷方法，返回所有附件
    log::info!("[Attachment] 使用 attachments() 方法查找...");
    for att in message.attachments() {
        let filename = att.attachment_name().map(|s| s.to_string());
        let content_id = att.content_id().map(|s| s.to_string());

        let filename_match = filename.as_ref() == Some(&target_filename.to_string());
        let content_id_match = target_content_id.is_some() &&
            content_id.as_ref() == Some(&target_content_id.unwrap().to_string());

        if filename_match || content_id_match {
            log::info!("[Attachment] 通过 attachments() 找到: {}", filename.as_deref().unwrap_or("无"));
            return Ok(att.contents().to_vec());
        }

        // 模糊匹配
        if let Some(fn_name) = &filename {
            if fn_name.contains(target_filename) || target_filename.contains(fn_name) {
                log::info!("[Attachment] 通过 attachments() 模糊匹配: {} ~= {}", fn_name, target_filename);
                return Ok(att.contents().to_vec());
            }
        }
    }

    Err(format!("未找到附件: {}", target_filename))
}

/// Save attachment to a user-selected path
#[tauri::command]
pub async fn save_attachment_to_path(
    attachment_id: String,
    target_path: String,
) -> Result<(), String> {
    let db = get_db().await?;
    let att = db.get_attachment_by_id(&attachment_id).await?
        .ok_or_else(|| "附件不存在".to_string())?;

    if !att.is_downloaded {
        return Err("附件未下载，请先下载附件".to_string());
    }

    let source_path = att.local_path
        .ok_or_else(|| "附件缺少本地路径信息".to_string())?;

    std::fs::copy(&source_path, &target_path)
        .map_err(|e| format!("复制文件失败: {}", e))?;

    Ok(())
}

/// Get attachment info with download status
#[tauri::command]
pub async fn get_attachment_info(attachment_id: String) -> Result<AttachmentInfoWithStatus, String> {
    let db = get_db().await?;
    let att = db.get_attachment_by_id(&attachment_id).await?
        .ok_or_else(|| "附件不存在".to_string())?;

    Ok(AttachmentInfoWithStatus {
        id: att.id,
        filename: att.filename,
        mime_type: att.mime_type,
        size: att.size,
        is_downloaded: att.is_downloaded,
        local_path: att.local_path,
    })
}

/// Get attachments for an email with download status
#[tauri::command]
pub async fn get_attachments_with_status(email_id: String) -> Result<Vec<AttachmentInfoWithStatus>, String> {
    let db = get_db().await?;
    let attachments = db.get_attachments(&email_id).await?;

    Ok(attachments
        .into_iter()
        .map(|a| AttachmentInfoWithStatus {
            id: a.id,
            filename: a.filename,
            mime_type: a.mime_type,
            size: a.size,
            is_downloaded: a.is_downloaded,
            local_path: a.local_path,
        })
        .collect())
}

/// Get cache statistics
#[tauri::command]
pub async fn get_attachment_cache_stats() -> Result<CacheStats, String> {
    let cache_dir = get_cache_directory()?;

    let mut file_count = 0u32;
    let mut total_size = 0u64;

    for entry in std::fs::read_dir(&cache_dir)
        .map_err(|e| format!("读取缓存目录失败: {}", e))?
    {
        let entry = entry.map_err(|e| format!("读取目录项失败: {}", e))?;
        let path = entry.path();

        if path.extension().map(|e| e == "dat").unwrap_or(false) {
            file_count += 1;
            total_size += entry.metadata()
                .map_err(|e| format!("读取文件元数据失败: {}", e))?
                .len();
        }
    }

    Ok(CacheStats {
        file_count,
        total_size,
        cache_path: cache_dir.to_string_lossy().to_string(),
    })
}

/// Clear attachment cache
#[tauri::command]
pub async fn clear_attachment_cache() -> Result<(), String> {
    let db = get_db().await?;
    let cache_dir = get_cache_directory()?;

    // 删除所有缓存文件
    for entry in std::fs::read_dir(&cache_dir)
        .map_err(|e| format!("读取缓存目录失败: {}", e))?
    {
        let entry = entry.map_err(|e| format!("读取目录项失败: {}", e))?;
        let path = entry.path();

        if path.is_file() {
            std::fs::remove_file(&path)
                .map_err(|e| format!("删除文件失败: {}", e))?;
        }
    }

    // 更新数据库状态
    db.mark_all_attachments_not_downloaded().await?;

    Ok(())
}

/// Open cache directory in system file explorer
#[tauri::command]
pub async fn open_attachment_cache_directory() -> Result<(), String> {
    let cache_dir = get_cache_directory()?;

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(&cache_dir)
            .spawn()
            .map_err(|e| format!("打开目录失败: {}", e))?;
    }

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&cache_dir)
            .spawn()
            .map_err(|e| format!("打开目录失败: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&cache_dir)
            .spawn()
            .map_err(|e| format!("打开目录失败: {}", e))?;
    }

    Ok(())
}

// ==================== 邮件会话功能命令 ====================

/// Thread data for frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thread {
    pub id: String,
    pub account_id: String,
    pub subject_base: Option<String>,
    pub last_message_at: i64,
    pub message_count: i32,
    pub has_unread: bool,
}

/// Thread email data (includes threading headers)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadEmail {
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
    pub is_phishing: bool,
    pub phishing_reasons: Option<Vec<String>>,
    /// RFC 2822 Message-ID header
    pub message_id_header: Option<String>,
    /// In-Reply-To header
    pub in_reply_to_header: Option<String>,
    /// References header
    pub references_header: Option<String>,
    /// Thread ID
    pub thread_id: Option<String>,
}

/// Thread assignment for frontend to backend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadAssignment {
    pub email_id: String,
    pub thread_id: String,
}

/// Get all threads for an account
#[tauri::command]
pub async fn get_threads(account_id: String) -> Result<Vec<Thread>, String> {
    let db = get_db().await?;
    let db_threads = db.get_threads_for_account(&account_id).await?;

    let threads: Vec<Thread> = db_threads
        .into_iter()
        .map(|t| Thread {
            id: t.id,
            account_id: t.account_id,
            subject_base: t.subject_base,
            last_message_at: t.last_message_at,
            message_count: t.message_count,
            has_unread: t.has_unread,
        })
        .collect();

    Ok(threads)
}

/// Get all emails in a thread
#[tauri::command]
pub async fn get_thread_emails(thread_id: String) -> Result<Vec<ThreadEmail>, String> {
    let db = get_db().await?;
    let db_emails = db.get_thread_emails(&thread_id).await?;

    let emails: Vec<ThreadEmail> = db_emails
        .into_iter()
        .map(|e| {
            let snippet = e.body_text
                .as_ref()
                .map(|t| t.chars().take(100).collect())
                .unwrap_or_default();

            let phishing_reasons: Option<Vec<String>> = e.phishing_reasons
                .as_ref()
                .and_then(|r| serde_json::from_str(r).ok());

            ThreadEmail {
                id: e.id,
                account_id: e.account_id,
                from_address: e.from_address,
                from_name: e.from_name,
                to_addresses: serde_json::from_str(&e.to_addresses).unwrap_or_default(),
                cc_addresses: serde_json::from_str(&e.cc_addresses).unwrap_or_default(),
                subject: e.subject,
                snippet,
                date: e.date,
                is_read: e.is_read,
                is_starred: e.is_starred,
                has_attachment: e.has_attachment,
                is_phishing: e.is_phishing,
                phishing_reasons,
                message_id_header: e.message_id_header,
                in_reply_to_header: e.in_reply_to_header,
                references_header: e.references_header,
                thread_id: e.thread_id,
            }
        })
        .collect();

    Ok(emails)
}

/// Assign thread IDs to emails (called after frontend computes threads)
#[tauri::command]
pub async fn assign_threads(assignments: Vec<ThreadAssignment>) -> Result<(), String> {
    let db = get_db().await?;

    // Convert to slice of tuples
    let tuples: Vec<(&str, &str)> = assignments
        .iter()
        .map(|a| (a.email_id.as_str(), a.thread_id.as_str()))
        .collect();

    db.assign_thread_ids(&tuples).await?;

    Ok(())
}

/// Get emails with threading headers for frontend computation
#[tauri::command]
pub async fn get_emails_for_threading(account_id: String) -> Result<Vec<ThreadEmail>, String> {
    let db = get_db().await?;
    let db_emails = db.get_emails_for_threading(&account_id).await?;

    let emails: Vec<ThreadEmail> = db_emails
        .into_iter()
        .map(|e| {
            let snippet = e.body_text
                .as_ref()
                .map(|t| t.chars().take(100).collect())
                .unwrap_or_default();

            let phishing_reasons: Option<Vec<String>> = e.phishing_reasons
                .as_ref()
                .and_then(|r| serde_json::from_str(r).ok());

            ThreadEmail {
                id: e.id,
                account_id: e.account_id,
                from_address: e.from_address,
                from_name: e.from_name,
                to_addresses: serde_json::from_str(&e.to_addresses).unwrap_or_default(),
                cc_addresses: serde_json::from_str(&e.cc_addresses).unwrap_or_default(),
                subject: e.subject,
                snippet,
                date: e.date,
                is_read: e.is_read,
                is_starred: e.is_starred,
                has_attachment: e.has_attachment,
                is_phishing: e.is_phishing,
                phishing_reasons,
                message_id_header: e.message_id_header,
                in_reply_to_header: e.in_reply_to_header,
                references_header: e.references_header,
                thread_id: e.thread_id,
            }
        })
        .collect();

    Ok(emails)
}