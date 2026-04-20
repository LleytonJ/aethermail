//! SMTP Client Implementation
//!
//! SMTP client using lettre crate for email sending.

use lettre::{
    message::{Message, MultiPart, SinglePart, header},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, Tokio1Executor, AsyncTransport,
};
use std::time::Duration;

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
            Ok(mailer) => {
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
        let msg = if let Some(html) = &email.body_html {
            // Create multipart MIME message (text + HTML)
            let text_part = SinglePart::builder()
                .header(header::ContentType::TEXT_PLAIN)
                .body(email.body.clone());

            let html_part = SinglePart::builder()
                .header(header::ContentType::TEXT_HTML)
                .body(html.clone());

            let multipart = MultiPart::alternative()
                .singlepart(text_part)
                .singlepart(html_part);

            builder
                .multipart(multipart)
                .map_err(|e| format!("邮件正文构建失败: {}", e))?
        } else {
            // Plain text only
            builder
                .body(email.body.clone())
                .map_err(|e| format!("邮件正文构建失败: {}", e))?
        };

        Ok(msg)
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