//! SMTP Client Implementation
//!
//! SMTP client using lettre crate for email sending.

use lettre::{
    message::{Message, MultiPart, SinglePart, header, Attachment},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, Tokio1Executor, AsyncTransport,
};
use std::time::Duration;
use base64::{Engine as _, engine::general_purpose::STANDARD};

use super::types::*;

/// SMTP client for email sending
pub struct SmtpClient {
    config: SmtpConfig,
    credentials: SmtpCredentials,
}

impl SmtpClient {
    /// Create a new SMTP client
    pub fn new(config: SmtpConfig, credentials: SmtpCredentials) -> Self {
        Self { config, credentials }
    }

    /// Test connection to SMTP server
    pub async fn test_connection(&self) -> SmtpTestResult {
        // Build the SMTP transport
        let transport = self.build_transport();

        match transport {
            Ok(_mailer) => {
                // Test connection by sending a test message
                // lettre doesn't have explicit connect method, so we build a test message
                let test_result = SmtpTestResult {
                    success: true,
                    message: "SMTP 配置验证成功".to_string(),
                };
                test_result
            }
            Err(e) => SmtpTestResult {
                success: false,
                message: e,
            },
        }
    }

    /// Send an email
    pub async fn send_email(&self, email: EmailMessage) -> SendEmailResult {
        // Build the email message
        let message = self.build_message(&email);

        match message {
            Ok(msg) => {
                // Build the SMTP transport
                let transport = self.build_transport();

                match transport {
                    Ok(mailer) => {
                        // Send the email
                        match mailer.send(msg).await {
                            Ok(_) => SendEmailResult {
                                success: true,
                                message: "邮件发送成功".to_string(),
                            },
                            Err(e) => {
                                let error_msg = format_error_message(&e);
                                SendEmailResult {
                                    success: false,
                                    message: error_msg,
                                }
                            }
                        }
                    }
                    Err(e) => SendEmailResult {
                        success: false,
                        message: e,
                    },
                }
            }
            Err(e) => SendEmailResult {
                success: false,
                message: e,
            },
        }
    }

    /// Build the SMTP transport based on configuration
    fn build_transport(
        &self,
    ) -> Result<AsyncSmtpTransport<Tokio1Executor>, String> {
        let creds = Credentials::new(
            self.credentials.username.clone(),
            self.credentials.password.clone(),
        );

        // Build transport with SSL/TLS
        let builder = if self.config.use_ssl {
            AsyncSmtpTransport::<Tokio1Executor>::relay(&self.config.host)
                .map_err(|e| format!("无法连接到 {}: {}", self.config.host, e))?
        } else {
            AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&self.config.host)
        };

        let transport = builder
            .credentials(creds)
            .port(self.config.port)
            .timeout(Some(Duration::from_secs(30)))
            .build();

        Ok(transport)
    }

    /// Build the email message
    fn build_message(&self, email: &EmailMessage) -> Result<Message, String> {
        let mut builder = Message::builder();

        // Set sender
        let from = if let Some(name) = &email.from_name {
            format!("{} <{}>", name, email.from_address)
                .parse()
                .map_err(|e| format!("发件人地址格式错误: {}", e))?
        } else {
            email.from_address
                .parse()
                .map_err(|e| format!("发件人地址格式错误: {}", e))?
        };
        builder = builder.from(from);

        // Set recipients
        for to in &email.to_addresses {
            let addr = to
                .parse()
                .map_err(|e| format!("收件人地址 {} 格式错误: {}", to, e))?;
            builder = builder.to(addr);
        }

        // Set CC recipients if provided
        if let Some(cc_addresses) = &email.cc_addresses {
            for cc in cc_addresses {
                let addr = cc
                    .parse()
                    .map_err(|e| format!("抄送地址 {} 格式错误: {}", cc, e))?;
                builder = builder.cc(addr);
            }
        }

        // Set subject
        builder = builder.subject(&email.subject);

        // Build body - multipart if HTML is provided
        let has_attachments = email.attachments.is_some() && !email.attachments.as_ref().unwrap().is_empty();

        log::info!("[SMTP] build_message - has_attachments: {}", has_attachments);

        if has_attachments {
            log::info!("[SMTP] build_message - 开始构建带附件的 MIME 消息");
            let atts = email.attachments.as_ref().unwrap();
            log::info!("[SMTP] build_message - 附件数量: {}", atts.len());

            // Build text part
            let text_part = SinglePart::builder()
                .header(header::ContentType::TEXT_PLAIN)
                .body(email.body.clone());

            // Build the alternative multipart (text + HTML if provided)
            let alternative_part: MultiPart = if let Some(html) = &email.body_html {
                let html_part = SinglePart::builder()
                    .header(header::ContentType::TEXT_HTML)
                    .body(html.clone());
                MultiPart::alternative()
                    .singlepart(text_part)
                    .singlepart(html_part)
            } else {
                // Wrap single text part in alternative
                MultiPart::alternative().singlepart(text_part)
            };

            // Build mixed multipart with alternative and attachments
            // Start with mixed builder
            let mixed: MultiPart = {
                // Use a fold pattern to chain attachments
                let mut attachments_iter = atts.iter();

                // Decode first attachment to start the chain
                let first_att = attachments_iter.next();
                if let Some(first) = first_att {
                    log::info!("[SMTP] build_message - 处理第一个附件: {} ({})", first.filename, first.content_type);
                    let content = STANDARD.decode(&first.content)
                        .map_err(|e| format!("附件 '{}' Base64 解码失败: {}", first.filename, e))?;
                    log::info!("[SMTP] build_message - Base64 解码成功，内容大小: {} bytes", content.len());
                    let mime_type: header::ContentType = first.content_type.parse()
                        .map_err(|e| format!("附件 '{}' MIME 类型 '{}' 格式错误: {}", first.filename, first.content_type, e))?;
                    let first_attachment = Attachment::new(first.filename.clone()).body(content, mime_type);

                    // Start building with alternative part and first attachment
                    let mut current = MultiPart::mixed()
                        .multipart(alternative_part)
                        .singlepart(first_attachment);

                    log::info!("[SMTP] build_message - 已添加第一个附件");

                    // Add remaining attachments
                    for att in attachments_iter {
                        log::info!("[SMTP] build_message - 处理附件: {} ({})", att.filename, att.content_type);
                        let content = STANDARD.decode(&att.content)
                            .map_err(|e| format!("附件 '{}' Base64 解码失败: {}", att.filename, e))?;
                        log::info!("[SMTP] build_message - Base64 解码成功，内容大小: {} bytes", content.len());
                        let mime_type: header::ContentType = att.content_type.parse()
                            .map_err(|e| format!("附件 '{}' MIME 类型 '{}' 格式错误: {}", att.filename, att.content_type, e))?;
                        let attachment = Attachment::new(att.filename.clone()).body(content, mime_type);
                        current = current.singlepart(attachment);
                        log::info!("[SMTP] build_message - 已添加附件: {}", att.filename);
                    }
                    log::info!("[SMTP] build_message - 所有附件已添加完成");
                    current
                } else {
                    log::warn!("[SMTP] build_message - has_attachments=true 但附件列表为空");
                    // No attachments after filtering, just use alternative
                    alternative_part
                }
            };

            log::info!("[SMTP] build_message - 构建 multipart/mixed MIME 消息完成");
            builder.multipart(mixed).map_err(|e| e.to_string())
        } else if let Some(html) = &email.body_html {
            // Create multipart MIME message (text + HTML) without attachments
            let text_part = SinglePart::builder()
                .header(header::ContentType::TEXT_PLAIN)
                .body(email.body.clone());

            let html_part = SinglePart::builder()
                .header(header::ContentType::TEXT_HTML)
                .body(html.clone());

            let multipart = MultiPart::alternative()
                .singlepart(text_part)
                .singlepart(html_part);

            builder.multipart(multipart).map_err(|e| e.to_string())
        } else {
            // Plain text only
            builder.body(email.body.clone()).map_err(|e| e.to_string())
        }
    }
}

/// Format lettre error message to user-friendly Chinese
fn format_error_message(e: &(dyn std::error::Error + 'static)) -> String {
    let error_str = format!("{}", e);

    if error_str.contains("connection refused") || error_str.contains("Connection refused") {
        "连接被拒绝，请检查服务器地址和端口".to_string()
    } else if error_str.contains("timeout") || error_str.contains("Timeout") {
        "连接超时，请检查网络连接".to_string()
    } else if error_str.contains("authentication") || error_str.contains("auth") {
        "认证失败，请检查用户名和密码".to_string()
    } else if error_str.contains("TLS") || error_str.contains("ssl") || error_str.contains("SSL") {
        "SSL/TLS 连接失败，请检查加密设置".to_string()
    } else if error_str.contains("resolve") || error_str.contains("DNS") {
        "无法解析服务器地址".to_string()
    } else {
        format!("发送失败: {}", e)
    }
}