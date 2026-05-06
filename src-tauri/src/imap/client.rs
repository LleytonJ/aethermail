//! IMAP Client Implementation
//!
//! IMAP client using async-imap crate for email retrieval.
//! Uses tokio-util::compat to bridge tokio and async-std traits.

use async_imap::Client;
use tokio::net::TcpStream;
use tokio_native_tls::{TlsConnector, TlsStream};
use tokio_util::compat::TokioAsyncReadCompatExt;
use futures::StreamExt;
use std::time::Duration;
use mail_parser::{MessageParser, HeaderValue};

use super::types::*;

/// IMAP client for email retrieval
pub struct ImapClient {
    config: ImapConfig,
    credentials: ImapCredentials,
}

impl ImapClient {
    /// Create a new IMAP client
    pub fn new(config: ImapConfig, credentials: ImapCredentials) -> Self {
        Self { config, credentials }
    }

    /// Test connection to IMAP server (includes authentication)
    pub async fn test_connection(&self) -> Result<ConnectionTestResult, String> {
        let addr = format!("{}:{}", self.config.host, self.config.port);

        // Attempt connection with timeout
        let stream = tokio::time::timeout(
            Duration::from_secs(30),
            TcpStream::connect(&addr)
        )
        .await
        .map_err(|_| "连接超时".to_string())?
        .map_err(|e| format!("连接失败: {}", e))?;

        // Wrap with TLS if needed
        if self.config.use_ssl {
            let connector = native_tls::TlsConnector::builder()
                .build()
                .map_err(|e| format!("TLS 配置失败: {}", e))?;
            let connector = TlsConnector::from(connector);

            let tls_stream = connector
                .connect(&self.config.host, stream)
                .await
                .map_err(|e| format!("TLS 握手失败: {}", e))?;

            // Convert to async-std compatible stream
            let compat_stream = tls_stream.compat();

            // Connect via async-imap
            let client = Client::new(compat_stream);

            // Perform LOGIN authentication to verify credentials
            let result = client
                .login(&self.credentials.username, &self.credentials.password)
                .await;

            match result {
                Ok(mut session) => {
                    // Logout after successful test
                    let _ = session.logout().await;
                    Ok(ConnectionTestResult {
                        success: true,
                        message: "连接并认证成功".to_string(),
                        server_greeting: None,
                    })
                },
                Err((error, _client)) => Err(format!("认证失败: {:?}", error)),
            }
        } else {
            // Plain connection (not recommended for production)
            let compat_stream = stream.compat();
            let client = Client::new(compat_stream);

            // Perform LOGIN authentication
            let result = client
                .login(&self.credentials.username, &self.credentials.password)
                .await;

            match result {
                Ok(mut session) => {
                    let _ = session.logout().await;
                    Ok(ConnectionTestResult {
                        success: true,
                        message: "连接并认证成功".to_string(),
                        server_greeting: None,
                    })
                },
                Err((error, _client)) => Err(format!("认证失败: {:?}", error)),
            }
        }
    }

    /// Connect and authenticate to IMAP server
    pub async fn connect(&self) -> Result<ImapSession, String> {
        let addr = format!("{}:{}", self.config.host, self.config.port);

        // Connect with timeout
        let stream = tokio::time::timeout(
            Duration::from_secs(30),
            TcpStream::connect(&addr)
        )
        .await
        .map_err(|_| "连接超时".to_string())?
        .map_err(|e| format!("连接失败: {}", e))?;

        // Wrap with TLS if needed
        if self.config.use_ssl {
            let connector = native_tls::TlsConnector::builder()
                .build()
                .map_err(|e| format!("TLS 配置失败: {}", e))?;
            let connector = TlsConnector::from(connector);

            let tls_stream = connector
                .connect(&self.config.host, stream)
                .await
                .map_err(|e| format!("TLS 握手失败: {}", e))?;

            // Convert to async-std compatible stream
            let compat_stream = tls_stream.compat();

            // Connect via async-imap
            let client = Client::new(compat_stream);

            // Perform LOGIN authentication
            let result = client
                .login(&self.credentials.username, &self.credentials.password)
                .await;

            match result {
                Ok(session) => Ok(ImapSession::Tls(session)),
                Err((error, _client)) => Err(format!("认证失败: {:?}", error)),
            }
        } else {
            // Plain connection (not recommended for production)
            let compat_stream = stream.compat();
            let client = Client::new(compat_stream);

            let result = client
                .login(&self.credentials.username, &self.credentials.password)
                .await;

            match result {
                Ok(session) => Ok(ImapSession::Plain(session)),
                Err((error, _client)) => Err(format!("认证失败: {:?}", error)),
            }
        }
    }
}

/// Type alias for TLS compat stream
type CompatTlsStream = tokio_util::compat::Compat<TlsStream<TcpStream>>;

/// Type alias for plain compat stream
type CompatPlainStream = tokio_util::compat::Compat<TcpStream>;

/// IMAP session wrapper - supports both TLS and plain connections
pub enum ImapSession {
    Tls(async_imap::Session<CompatTlsStream>),
    Plain(async_imap::Session<CompatPlainStream>),
}

impl ImapSession {
    /// Select INBOX folder
    pub async fn select_inbox(&mut self) -> Result<MailboxStatus, String> {
        let mailbox = match self {
            ImapSession::Tls(session) => session
                .select("INBOX")
                .await
                .map_err(|e| format!("SELECT INBOX 失败: {:?}", e))?,
            ImapSession::Plain(session) => session
                .select("INBOX")
                .await
                .map_err(|e| format!("SELECT INBOX 失败: {:?}", e))?,
        };

        Ok(MailboxStatus {
            messages: mailbox.exists,
            uid_validity: mailbox.uid_validity.unwrap_or(0),
            uid_next: mailbox.uid_next.unwrap_or(0),
        })
    }

    /// Get list of all UIDs in INBOX
    pub async fn get_uid_list(&mut self) -> Result<Vec<u32>, String> {
        let uids = match self {
            ImapSession::Tls(session) => session
                .uid_search("ALL")
                .await
                .map_err(|e| format!("UID SEARCH 失败: {:?}", e))?,
            ImapSession::Plain(session) => session
                .uid_search("ALL")
                .await
                .map_err(|e| format!("UID SEARCH 失败: {:?}", e))?,
        };

        // Convert HashSet to Vec and sort
        let mut uid_vec: Vec<u32> = uids.into_iter().collect();
        uid_vec.sort();

        Ok(uid_vec)
    }

    /// Fetch a single email by UID
    pub async fn fetch_email(&mut self, uid: u32) -> Result<ImapMessage, String> {
        // Collect fetch results inside match to avoid impl Stream type mismatch
        let body_bytes: Vec<u8> = match self {
            ImapSession::Tls(session) => {
                let fetch_stream = session
                    .uid_fetch(format!("{}", uid), "BODY[]")
                    .await
                    .map_err(|e| format!("UID FETCH 失败: {:?}", e))?;

                let fetches: Vec<_> = fetch_stream
                    .collect::<Vec<Result<_, _>>>()
                    .await;

                let fetch = fetches
                    .into_iter()
                    .next()
                    .ok_or_else(|| "未找到邮件".to_string())?
                    .map_err(|e| format!("FETCH 错误: {:?}", e))?;

                fetch
                    .body()
                    .ok_or_else(|| "邮件内容为空".to_string())?
                    .to_vec()
            }
            ImapSession::Plain(session) => {
                let fetch_stream = session
                    .uid_fetch(format!("{}", uid), "BODY[]")
                    .await
                    .map_err(|e| format!("UID FETCH 失败: {:?}", e))?;

                let fetches: Vec<_> = fetch_stream
                    .collect::<Vec<Result<_, _>>>()
                    .await;

                let fetch = fetches
                    .into_iter()
                    .next()
                    .ok_or_else(|| "未找到邮件".to_string())?
                    .map_err(|e| format!("FETCH 错误: {:?}", e))?;

                fetch
                    .body()
                    .ok_or_else(|| "邮件内容为空".to_string())?
                    .to_vec()
            }
        };

        // Convert bytes to string
        let raw_content = String::from_utf8_lossy(&body_bytes).to_string();

        // Parse threading headers using mail-parser
        let parser = MessageParser::new();
        let message = parser.parse(&body_bytes);

        let (message_id, in_reply_to, references) = if let Some(msg) = message {
            // Message-ID
            let msg_id = msg.message_id().map(|s| s.to_string());

            // In-Reply-To (can be Text or TextList)
            let in_reply_to_val = msg.in_reply_to();
            let in_reply = match in_reply_to_val {
                HeaderValue::Text(t) => Some(t.to_string()),
                HeaderValue::TextList(list) => list.first().map(|s| s.to_string()),
                _ => {
                    // 尝试从 raw headers 中手动查找 In-Reply-To
                    if raw_content.contains("In-Reply-To:") {
                        // 打印调试信息
                        println!("[IMAP] mail-parser in_reply_to 返回非 Text/TextList: {:?}", in_reply_to_val);
                        println!("[IMAP] raw_content 包含 In-Reply-To header");
                    }
                    None
                }
            };

            // References (can be Text or TextList)
            let refs_val = msg.references();
            let refs = match refs_val {
                HeaderValue::Text(t) => Some(t.to_string()),
                HeaderValue::TextList(list) => {
                    if list.is_empty() {
                        None
                    } else {
                        Some(list.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(" "))
                    }
                }
                _ => {
                    if raw_content.contains("References:") {
                        println!("[IMAP] mail-parser references 返回非 Text/TextList: {:?}", refs_val);
                        println!("[IMAP] raw_content 包含 References header");
                    }
                    None
                }
            };

            // 调试：打印解析结果
            println!("[IMAP] Threading headers: message_id={:?}, in_reply_to={:?}, references={:?}", msg_id, in_reply, refs);

            (msg_id, in_reply, refs)
        } else {
            println!("[IMAP] mail-parser 解析失败");
            (None, None, None)
        };

        Ok(ImapMessage {
            uid,
            raw_content,
            message_id,
            in_reply_to,
            references,
        })
    }

    /// Fetch MIME structure (BODYSTRUCTURE) for an email by UID
    /// Returns the MIME tree structure with section numbers
    pub async fn fetch_structure(&mut self, uid: u32) -> Result<EmailStructure, String> {
        // Fetch BODYSTRUCTURE
        let body_structure_data = match self {
            ImapSession::Tls(session) => {
                let fetch_stream = session
                    .uid_fetch(format!("{}", uid), "BODYSTRUCTURE")
                    .await
                    .map_err(|e| format!("UID FETCH BODYSTRUCTURE 失败: {:?}", e))?;

                let fetches: Vec<_> = fetch_stream
                    .collect::<Vec<Result<_, _>>>()
                    .await;

                fetches
                    .into_iter()
                    .next()
                    .ok_or_else(|| "未找到邮件".to_string())?
                    .map_err(|e| format!("FETCH 错误: {:?}", e))?
            }
            ImapSession::Plain(session) => {
                let fetch_stream = session
                    .uid_fetch(format!("{}", uid), "BODYSTRUCTURE")
                    .await
                    .map_err(|e| format!("UID FETCH BODYSTRUCTURE 失败: {:?}", e))?;

                let fetches: Vec<_> = fetch_stream
                    .collect::<Vec<Result<_, _>>>()
                    .await;

                fetches
                    .into_iter()
                    .next()
                    .ok_or_else(|| "未找到邮件".to_string())?
                    .map_err(|e| format!("FETCH 错误: {:?}", e))?
            }
        };

        // Parse BODYSTRUCTURE into EmailStructure
        let structure = parse_bodystructure(&body_structure_data);

        Ok(structure)
    }

    /// Fetch only the body text (first MIME part) without attachments
    /// Uses BODY.PEEK[1] to avoid setting Seen flag
    pub async fn fetch_body_only(&mut self, uid: u32) -> Result<(Option<String>, Option<String>), String> {
        // First get structure to find body sections
        let structure = self.fetch_structure(uid).await?;

        // Find text/plain and text/html parts
        let text_section = structure.parts.iter()
            .find(|p| p.mime_type == "text/plain" && !p.is_attachment)
            .map(|p| p.section.clone());

        let html_section = structure.parts.iter()
            .find(|p| p.mime_type == "text/html" && !p.is_attachment)
            .map(|p| p.section.clone());

        let mut body_text: Option<String> = None;
        let mut body_html: Option<String> = None;

        // Fetch text body if section exists
        if let Some(section) = text_section {
            let content = self.fetch_section(uid, &section, true).await?;
            body_text = Some(String::from_utf8_lossy(&content).to_string());
        }

        // Fetch HTML body if section exists
        if let Some(section) = html_section {
            let content = self.fetch_section(uid, &section, true).await?;
            body_html = Some(String::from_utf8_lossy(&content).to_string());
        }

        // If no specific body sections found, use section 1 as fallback
        if body_text.is_none() && body_html.is_none() {
            let content = self.fetch_section(uid, "1", true).await?;
            body_text = Some(String::from_utf8_lossy(&content).to_string());
        }

        Ok((body_text, body_html))
    }

    /// Fetch a specific MIME section
    /// Use PEEK to avoid setting Seen flag if peek=true
    pub async fn fetch_section(&mut self, uid: u32, section: &str, peek: bool) -> Result<Vec<u8>, String> {
        let fetch_command = if peek {
            format!("BODY.PEEK[{}]", section)
        } else {
            format!("BODY[{}]", section)
        };

        let body_bytes = match self {
            ImapSession::Tls(session) => {
                let fetch_stream = session
                    .uid_fetch(format!("{}", uid), &fetch_command)
                    .await
                    .map_err(|e| format!("UID FETCH {} 失败: {:?}", fetch_command, e))?;

                let fetches: Vec<_> = fetch_stream
                    .collect::<Vec<Result<_, _>>>()
                    .await;

                let fetch = fetches
                    .into_iter()
                    .next()
                    .ok_or_else(|| "未找到邮件部分".to_string())?
                    .map_err(|e| format!("FETCH 错误: {:?}", e))?;

                fetch
                    .body()
                    .ok_or_else(|| "邮件部分内容为空".to_string())?
                    .to_vec()
            }
            ImapSession::Plain(session) => {
                let fetch_stream = session
                    .uid_fetch(format!("{}", uid), &fetch_command)
                    .await
                    .map_err(|e| format!("UID FETCH {} 失败: {:?}", fetch_command, e))?;

                let fetches: Vec<_> = fetch_stream
                    .collect::<Vec<Result<_, _>>>()
                    .await;

                let fetch = fetches
                    .into_iter()
                    .next()
                    .ok_or_else(|| "未找到邮件部分".to_string())?
                    .map_err(|e| format!("FETCH 错误: {:?}", e))?;

                fetch
                    .body()
                    .ok_or_else(|| "邮件部分内容为空".to_string())?
                    .to_vec()
            }
        };

        Ok(body_bytes)
    }

    /// Fetch attachment content by section
    /// This downloads the actual attachment data
    pub async fn fetch_attachment_section(&mut self, uid: u32, section: &str) -> Result<Vec<u8>, String> {
        self.fetch_section(uid, section, false).await
    }

    /// Logout and close connection
    pub async fn logout(&mut self) -> Result<(), String> {
        match self {
            ImapSession::Tls(session) => session
                .logout()
                .await
                .map_err(|e| format!("LOGOUT 失败: {:?}", e))?,
            ImapSession::Plain(session) => session
                .logout()
                .await
                .map_err(|e| format!("LOGOUT 失败: {:?}", e))?,
        };

        Ok(())
    }
}

/// Parse BODYSTRUCTURE response into EmailStructure
fn parse_bodystructure(fetch: &async_imap::types::Fetch) -> EmailStructure {
    let mut parts = Vec::new();

    if let Some(bs) = fetch.bodystructure() {
        parse_mime_parts_recursive(bs, "", &mut parts);
    }

    EmailStructure { parts }
}

/// Recursively parse MIME parts from BODYSTRUCTURE
fn parse_mime_parts_recursive(bs: &imap_proto::types::BodyStructure<'_>, parent_section: &str, parts: &mut Vec<MimePart>) {
    match bs {
        imap_proto::types::BodyStructure::Multipart { common: _, bodies, .. } => {
            // multipart: recursively process child parts
            for (i, child) in bodies.iter().enumerate() {
                let section = if parent_section.is_empty() {
                    format!("{}", i + 1)
                } else {
                    format!("{}.{}", parent_section, i + 1)
                };
                parse_mime_parts_recursive(child, &section, parts);
            }
        }
        // Single part types: Basic, Text, Message
        imap_proto::types::BodyStructure::Basic { common, other, .. }
        | imap_proto::types::BodyStructure::Text { common, other, .. }
        | imap_proto::types::BodyStructure::Message { common, other, .. } => {
            // Get MIME type from common.ty
            let mime_type = format!("{}/{}", common.ty.ty.as_ref(), common.ty.subtype.as_ref());

            // Determine if this is an attachment from disposition
            let is_attachment = common.disposition.as_ref()
                .map(|d: &imap_proto::types::ContentDisposition| d.ty.as_ref() == "attachment")
                .unwrap_or(false);

            // Check for filename in params (BodyParams is Option<Vec<(Cow, Cow)>)
            let filename_from_params = common.ty.params.as_ref()
                .and_then(|params| params.iter()
                    .find(|(key, _)| key.as_ref() == "filename")
                    .map(|(_, value)| value.as_ref().to_string()));

            // Or in disposition params
            let filename_from_disposition = common.disposition.as_ref()
                .and_then(|d| d.params.as_ref()
                    .and_then(|params| params.iter()
                        .find(|(key, _)| key.as_ref() == "filename")
                        .map(|(_, value)| value.as_ref().to_string())));

            // Get filename
            let filename = filename_from_params.or(filename_from_disposition);

            // If has filename and not text body, consider as attachment
            let has_filename = filename.is_some();
            let content_id = other.id.as_ref().map(|s| s.as_ref().to_string());
            let final_is_attachment = is_attachment ||
                (has_filename && !mime_type.starts_with("text/") && content_id.is_none());

            parts.push(MimePart {
                section: parent_section.to_string(),
                mime_type,
                filename,
                size: other.octets as u64,
                content_id,
                is_attachment: final_is_attachment,
                children: Vec::new(),
            });
        }
    }
}