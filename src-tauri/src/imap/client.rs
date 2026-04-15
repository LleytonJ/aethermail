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

        Ok(ImapMessage {
            uid,
            raw_content,
        })
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