//! POP3 Client Implementation
//!
//! Hand-written POP3 client using tokio and native-tls.

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio_native_tls::TlsConnector;
use std::time::Duration;

use super::types::*;

/// POP3 client for email retrieval
pub struct Pop3Client {
    config: Pop3Config,
    credentials: Pop3Credentials,
}

impl Pop3Client {
    /// Create a new POP3 client
    pub fn new(config: Pop3Config, credentials: Pop3Credentials) -> Self {
        Self { config, credentials }
    }

    /// Test connection to POP3 server
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

            // Read server greeting
            let mut reader = BufReader::new(tls_stream);
            let mut greeting = String::new();
            reader.read_line(&mut greeting)
                .await
                .map_err(|e| format!("读取服务器响应失败: {}", e))?;

            if !greeting.starts_with("+OK") {
                return Err(format!("服务器返回错误: {}", greeting.trim()));
            }

            Ok(ConnectionTestResult {
                success: true,
                message: "连接成功".to_string(),
                server_greeting: Some(greeting.trim().to_string()),
            })
        } else {
            let mut reader = BufReader::new(stream);
            let mut greeting = String::new();
            reader.read_line(&mut greeting)
                .await
                .map_err(|e| format!("读取服务器响应失败: {}", e))?;

            if !greeting.starts_with("+OK") {
                return Err(format!("服务器返回错误: {}", greeting.trim()));
            }

            Ok(ConnectionTestResult {
                success: true,
                message: "连接成功".to_string(),
                server_greeting: Some(greeting.trim().to_string()),
            })
        }
    }

    /// Connect and authenticate to POP3 server
    pub async fn connect(&self) -> Result<Pop3Session, String> {
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

            let session = Pop3Session::new_tls(tls_stream).await?;
            session.authenticate(&self.credentials).await
        } else {
            let session = Pop3Session::new_plain(stream).await?;
            session.authenticate(&self.credentials).await
        }
    }
}

/// Reader type for POP3 session - stores BufReader to preserve buffer
enum Pop3Reader {
    Plain(BufReader<TcpStream>),
    Tls(BufReader<tokio_native_tls::TlsStream<TcpStream>>),
}

/// POP3 session (can be TLS or plain)
pub struct Pop3Session {
    reader: Pop3Reader,
}

impl Pop3Session {
    /// Create a new TLS session
    async fn new_tls(stream: tokio_native_tls::TlsStream<TcpStream>) -> Result<Self, String> {
        let reader = BufReader::new(stream);
        let mut session = Self { reader: Pop3Reader::Tls(reader) };
        session.read_greeting().await?;
        Ok(session)
    }

    /// Create a new plain TCP session
    async fn new_plain(stream: TcpStream) -> Result<Self, String> {
        let reader = BufReader::new(stream);
        let mut session = Self { reader: Pop3Reader::Plain(reader) };
        session.read_greeting().await?;
        Ok(session)
    }

    /// Read a line from the stream with timeout
    async fn read_line(&mut self) -> Result<String, String> {
        match &mut self.reader {
            Pop3Reader::Plain(reader) => {
                let mut line = String::new();
                tokio::time::timeout(
                    Duration::from_secs(60),
                    reader.read_line(&mut line)
                )
                .await
                .map_err(|_| "读取超时".to_string())?
                .map_err(|e| format!("读取响应失败: {}", e))?;
                Ok(line)
            }
            Pop3Reader::Tls(reader) => {
                let mut line = String::new();
                tokio::time::timeout(
                    Duration::from_secs(60),
                    reader.read_line(&mut line)
                )
                .await
                .map_err(|_| "读取超时".to_string())?
                .map_err(|e| format!("读取响应失败: {}", e))?;
                Ok(line)
            }
        }
    }

    /// Write data to the stream
    async fn write(&mut self, data: &[u8]) -> Result<(), String> {
        match &mut self.reader {
            Pop3Reader::Plain(reader) => {
                reader.get_mut().write_all(data).await
                    .map_err(|e| format!("发送数据失败: {}", e))?;
                reader.get_mut().flush().await
                    .map_err(|e| format!("刷新缓冲区失败: {}", e))?;
            }
            Pop3Reader::Tls(reader) => {
                reader.get_mut().write_all(data).await
                    .map_err(|e| format!("发送数据失败: {}", e))?;
                reader.get_mut().flush().await
                    .map_err(|e| format!("刷新缓冲区失败: {}", e))?;
            }
        }
        Ok(())
    }

    /// Read server greeting
    async fn read_greeting(&mut self) -> Result<String, String> {
        let line = self.read_line().await?;

        if line.starts_with("+OK") {
            Ok(line.trim().to_string())
        } else {
            Err(format!("服务器返回错误: {}", line))
        }
    }

    /// Send command and read response
    async fn send_command(&mut self, cmd: &str) -> Result<String, String> {
        self.write(cmd.as_bytes()).await?;
        self.write(b"\r\n").await?;
        self.read_line().await.map(|l| l.trim().to_string())
    }

    /// Authenticate with USER and PASS commands
    async fn authenticate(mut self, credentials: &Pop3Credentials) -> Result<Self, String> {
        // USER command
        let response = self.send_command(&format!("USER {}", credentials.username)).await?;
        if !response.starts_with("+OK") {
            return Err(format!("USER 命令失败: {}", response));
        }

        // PASS command
        let response = self.send_command(&format!("PASS {}", credentials.password)).await?;
        if !response.starts_with("+OK") {
            return Err(format!("认证失败: {}", response));
        }

        Ok(self)
    }

    /// Get message count and total size (STAT command)
    pub async fn stat(&mut self) -> Result<(u32, u32), String> {
        let response = self.send_command("STAT").await?;

        if response.starts_with("+OK") {
            // Format: +OK <count> <size>
            let parts: Vec<&str> = response.split_whitespace().collect();
            if parts.len() >= 3 {
                let count: u32 = parts[1].parse().unwrap_or(0);
                let size: u32 = parts[2].parse().unwrap_or(0);
                return Ok((count, size));
            }
        }

        Err(format!("STAT 命令失败: {}", response))
    }

    /// Get UIDL list for all messages
    pub async fn get_uidl_list(&mut self) -> Result<Vec<(u32, String)>, String> {
        let response = self.send_command("UIDL").await?;

        if !response.starts_with("+OK") {
            return Err(format!("UIDL 命令失败: {}", response));
        }

        let mut uidls = Vec::new();
        loop {
            let line = self.read_line().await?;
            let line = line.trim();

            if line == "." {
                break;
            }

            // Format: <message_number> <uidl>
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                if let Ok(num) = parts[0].parse::<u32>() {
                    uidls.push((num, parts[1].to_string()));
                }
            }
        }

        Ok(uidls)
    }

    /// Retrieve a specific email by message number
    pub async fn fetch_email(&mut self, message_number: u32) -> Result<RawEmail, String> {
        // First get the UIDL for this message
        let uidl_response = self.send_command(&format!("UIDL {}", message_number)).await?;

        let uidl = if uidl_response.starts_with("+OK") {
            let parts: Vec<&str> = uidl_response.split_whitespace().collect();
            if parts.len() >= 3 {
                parts[2].to_string()
            } else {
                format!("msg-{}", message_number)
            }
        } else {
            format!("msg-{}", message_number)
        };

        // Retrieve the email content
        let retr_response = self.send_command(&format!("RETR {}", message_number)).await?;

        if !retr_response.starts_with("+OK") {
            return Err(format!("RETR 命令失败: {}", retr_response));
        }

        // Parse reported size from response (+OK <size>)
        let reported_size: usize = retr_response
            .split_whitespace()
            .nth(1)
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        // Read email content until terminating "."
        let mut content = String::new();

        // Set max read limit (10x reported size, min 1MB, max 50MB)
        let max_size = std::cmp::min(
            std::cmp::max(reported_size * 10, 1_000_000),
            50_000_000
        );

        loop {
            let line = self.read_line().await?;
            let line_trimmed = line.trim_end_matches('\n').trim_end_matches('\r');

            if line_trimmed == "." {
                break;
            }

            // POP3 uses byte-stuffing: lines starting with "." have an extra "." prepended
            if line_trimmed.starts_with("..") {
                content.push_str(&line_trimmed[1..]);
            } else {
                content.push_str(line_trimmed);
            }
            content.push('\n');

            // Check if we exceed max limit
            if content.len() > max_size {
                return Err(format!(
                    "邮件大小超过限制 (已读 {} 字节，限制 {} 字节)",
                    content.len(), max_size
                ));
            }
        }

        Ok(RawEmail {
            message_number,
            uidl,
            raw_content: content,
        })
    }

    /// Disconnect from server (QUIT command)
    pub async fn disconnect(&mut self) -> Result<(), String> {
        let _ = self.send_command("QUIT").await;
        Ok(())
    }
}