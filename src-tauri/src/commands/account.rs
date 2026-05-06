//! Account management commands

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;
use std::io::{Read, Write};

use crate::pop3::{Pop3Client, Pop3Config, Pop3Credentials, ConnectionTestResult};
use crate::imap::{ImapClient, ImapConfig, ImapCredentials};
use crate::smtp::{SmtpClient, SmtpConfig, SmtpCredentials, SmtpTestResult};

/// Protocol type for email account
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ProtocolType {
    Pop3,
    Imap,
}

impl Default for ProtocolType {
    fn default() -> Self {
        ProtocolType::Pop3
    }
}

/// Account configuration stored in JSON file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountConfig {
    pub id: String,
    pub email: String,
    pub display_name: Option<String>,
    pub password: String,
    /// Protocol type (pop3 or imap)
    #[serde(default)]
    pub protocol: ProtocolType,
    /// POP3 server configuration
    pub pop3_host: String,
    pub pop3_port: u16,
    pub use_ssl: bool,
    /// IMAP server configuration
    #[serde(default)]
    pub imap_host: String,
    #[serde(default = "default_imap_port")]
    pub imap_port: u16,
    #[serde(default = "default_imap_ssl")]
    pub imap_use_ssl: bool,
    /// SMTP server configuration
    #[serde(default)]
    pub smtp_host: String,
    #[serde(default = "default_smtp_port")]
    pub smtp_port: u16,
    #[serde(default = "default_smtp_ssl")]
    pub smtp_use_ssl: bool,
    pub is_active: bool,
    /// 是否启用钓鱼邮件检测
    #[serde(default)]
    pub phishing_detection_enabled: bool,
    /// 签名 HTML 内容
    #[serde(default)]
    pub signature_html: String,
}

fn default_imap_port() -> u16 { 993 }
fn default_imap_ssl() -> bool { true }
fn default_smtp_port() -> u16 { 465 }
fn default_smtp_ssl() -> bool { true }

impl Default for AccountConfig {
    fn default() -> Self {
        Self {
            id: String::new(),
            email: String::new(),
            display_name: None,
            password: String::new(),
            protocol: ProtocolType::Pop3,
            pop3_host: String::new(),
            pop3_port: 995,
            use_ssl: true,
            imap_host: String::new(),
            imap_port: 993,
            imap_use_ssl: true,
            smtp_host: String::new(),
            smtp_port: 465,
            smtp_use_ssl: true,
            is_active: false,
            phishing_detection_enabled: false,
            signature_html: String::new(),
        }
    }
}

/// Accounts configuration file structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountsFile {
    pub accounts: Vec<AccountConfig>,
}

/// Get the configuration file path
fn get_config_path() -> Result<PathBuf, String> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| "无法确定配置目录".to_string())?
        .join("aethermail");

    // Create directory if it doesn't exist
    fs::create_dir_all(&config_dir)
        .map_err(|e| format!("创建配置目录失败: {}", e))?;

    Ok(config_dir.join("accounts.json"))
}

/// Load accounts from configuration file
fn load_accounts_file() -> Result<AccountsFile, String> {
    let path = get_config_path()?;

    if !path.exists() {
        return Ok(AccountsFile::default());
    }

    let mut file = fs::File::open(&path)
        .map_err(|e| format!("打开配置文件失败: {}", e))?;

    let mut content = String::new();
    file.read_to_string(&mut content)
        .map_err(|e| format!("读取配置文件失败: {}", e))?;

    let mut accounts_file: AccountsFile = serde_json::from_str(&content)
        .map_err(|e| format!("解析配置文件失败: {}", e))?;

    // 迁移旧账户：根据服务器配置检测协议类型
    // 旧账户可能没有 protocol 字段，serde 会使用默认值 Pop3
    // 但如果 imap_host 有值，说明实际上是 IMAP 账户
    let needs_save = migrate_account_protocols(&mut accounts_file.accounts);
    if needs_save {
        save_accounts_file(&accounts_file)?;
    }

    Ok(accounts_file)
}

/// 迁移账户协议类型（修复旧账户的 protocol 字段）
fn migrate_account_protocols(accounts: &mut [AccountConfig]) -> bool {
    let mut changed = false;
    for account in accounts {
        // 如果 imap_host 有值但 protocol 是 Pop3，修正为 Imap
        if !account.imap_host.is_empty() && account.protocol == ProtocolType::Pop3 {
            account.protocol = ProtocolType::Imap;
            changed = true;
            println!("[AccountMigration] 账户 {} 迁移为 IMAP (imap_host={})", account.email, account.imap_host);
        }
        // 如果 pop3_host 有值但 protocol 是 Imap，修正为 Pop3
        if !account.pop3_host.is_empty() && account.protocol == ProtocolType::Imap && account.imap_host.is_empty() {
            account.protocol = ProtocolType::Pop3;
            changed = true;
            println!("[AccountMigration] 账户 {} 迁移为 POP3 (pop3_host={})", account.email, account.pop3_host);
        }
    }
    changed
}

/// Save accounts to configuration file
fn save_accounts_file(accounts: &AccountsFile) -> Result<(), String> {
    let path = get_config_path()?;

    let content = serde_json::to_string_pretty(accounts)
        .map_err(|e| format!("序列化配置失败: {}", e))?;

    let mut file = fs::File::create(&path)
        .map_err(|e| format!("创建配置文件失败: {}", e))?;

    file.write_all(content.as_bytes())
        .map_err(|e| format!("写入配置文件失败: {}", e))?;

    Ok(())
}

/// Test POP3 connection
#[tauri::command]
pub async fn test_pop3_connection(
    host: String,
    port: u16,
    use_ssl: bool,
    email: String,
    password: String,
) -> Result<ConnectionTestResult, String> {
    let config = Pop3Config { host, port, use_ssl };
    let credentials = Pop3Credentials {
        username: email,
        password,
    };

    let client = Pop3Client::new(config, credentials);
    client.test_connection().await
}

/// Test IMAP connection
#[tauri::command]
pub async fn test_imap_connection(
    host: String,
    port: u16,
    use_ssl: bool,
    email: String,
    password: String,
) -> Result<ConnectionTestResult, String> {
    let config = ImapConfig { host, port, use_ssl };
    let credentials = ImapCredentials {
        username: email,
        password,
    };

    let client = ImapClient::new(config, credentials);
    client.test_connection().await
}

/// Test SMTP connection
#[tauri::command]
pub async fn test_smtp_connection(
    host: String,
    port: u16,
    use_ssl: bool,
    email: String,
    password: String,
) -> Result<SmtpTestResult, String> {
    let config = SmtpConfig { host, port, use_ssl };
    let credentials = SmtpCredentials {
        username: email,
        password,
    };

    let client = SmtpClient::new(config, credentials);
    Ok(client.test_connection().await)
}

/// Add a new POP3 account
#[tauri::command]
pub async fn add_pop3_account(
    email: String,
    password: String,
    display_name: Option<String>,
    host: String,
    port: u16,
    use_ssl: bool,
    smtp_host: String,
    smtp_port: u16,
    smtp_use_ssl: bool,
) -> Result<AccountConfig, String> {
    let id = format!("account-{}", chrono::Utc::now().timestamp_millis());

    let account = AccountConfig {
        id: id.clone(),
        email,
        display_name,
        password,
        protocol: ProtocolType::Pop3,
        pop3_host: host,
        pop3_port: port,
        use_ssl,
        imap_host: String::new(),
        imap_port: 993,
        imap_use_ssl: true,
        smtp_host,
        smtp_port,
        smtp_use_ssl,
        is_active: true,
        phishing_detection_enabled: false,
        signature_html: String::new(),
    };

    // Load existing accounts
    let mut accounts_file = load_accounts_file()?;

    // Deactivate other accounts if this one is active
    if account.is_active {
        for acc in &mut accounts_file.accounts {
            acc.is_active = false;
        }
    }

    // Add new account
    accounts_file.accounts.push(account.clone());

    // Save to file
    save_accounts_file(&accounts_file)?;

    Ok(account)
}

/// Add a new IMAP account
#[tauri::command]
pub async fn add_imap_account(
    email: String,
    password: String,
    display_name: Option<String>,
    imap_host: String,
    imap_port: u16,
    imap_use_ssl: bool,
    smtp_host: String,
    smtp_port: u16,
    smtp_use_ssl: bool,
) -> Result<AccountConfig, String> {
    let id = format!("account-{}", chrono::Utc::now().timestamp_millis());

    let account = AccountConfig {
        id: id.clone(),
        email,
        display_name,
        password,
        protocol: ProtocolType::Imap,
        pop3_host: String::new(),
        pop3_port: 995,
        use_ssl: true,
        imap_host,
        imap_port,
        imap_use_ssl,
        smtp_host,
        smtp_port,
        smtp_use_ssl,
        is_active: true,
        phishing_detection_enabled: false,
        signature_html: String::new(),
    };

    // Load existing accounts
    let mut accounts_file = load_accounts_file()?;

    // Deactivate other accounts if this one is active
    if account.is_active {
        for acc in &mut accounts_file.accounts {
            acc.is_active = false;
        }
    }

    // Add new account
    accounts_file.accounts.push(account.clone());

    // Save to file
    save_accounts_file(&accounts_file)?;

    Ok(account)
}

/// Get all accounts
#[tauri::command]
pub fn get_accounts() -> Result<Vec<AccountConfig>, String> {
    let accounts_file = load_accounts_file()?;
    Ok(accounts_file.accounts)
}

/// Get active account
#[tauri::command]
pub fn get_active_account() -> Result<Option<AccountConfig>, String> {
    let accounts_file = load_accounts_file()?;
    Ok(accounts_file.accounts.into_iter().find(|a| a.is_active))
}

/// Set active account
#[tauri::command]
pub fn set_active_account(id: String) -> Result<(), String> {
    let mut accounts_file = load_accounts_file()?;

    for account in &mut accounts_file.accounts {
        account.is_active = account.id == id;
    }

    save_accounts_file(&accounts_file)?;
    Ok(())
}

/// Update account
#[tauri::command]
pub fn update_account(
    id: String,
    display_name: Option<String>,
    password: Option<String>,
    host: Option<String>,
    port: Option<u16>,
    use_ssl: Option<bool>,
    smtp_host: Option<String>,
    smtp_port: Option<u16>,
    smtp_use_ssl: Option<bool>,
) -> Result<AccountConfig, String> {
    let mut accounts_file = load_accounts_file()?;

    let account = accounts_file
        .accounts
        .iter_mut()
        .find(|a| a.id == id)
        .ok_or_else(|| "账户不存在".to_string())?;

    if let Some(d) = display_name {
        account.display_name = Some(d);
    }
    if let Some(p) = password {
        // Only update password if not empty
        if !p.is_empty() {
            account.password = p;
        }
    }

    // Update incoming server based on protocol type
    if let Some(h) = host {
        match account.protocol {
            ProtocolType::Pop3 => account.pop3_host = h,
            ProtocolType::Imap => account.imap_host = h,
        }
    }
    if let Some(p) = port {
        match account.protocol {
            ProtocolType::Pop3 => account.pop3_port = p,
            ProtocolType::Imap => account.imap_port = p,
        }
    }
    if let Some(s) = use_ssl {
        match account.protocol {
            ProtocolType::Pop3 => account.use_ssl = s,
            ProtocolType::Imap => account.imap_use_ssl = s,
        }
    }

    // Update SMTP settings
    if let Some(h) = smtp_host {
        account.smtp_host = h;
    }
    if let Some(p) = smtp_port {
        account.smtp_port = p;
    }
    if let Some(s) = smtp_use_ssl {
        account.smtp_use_ssl = s;
    }

    let updated = account.clone();
    save_accounts_file(&accounts_file)?;

    Ok(updated)
}

/// Delete account
#[tauri::command]
pub fn delete_account(id: String) -> Result<(), String> {
    let mut accounts_file = load_accounts_file()?;

    let was_active = accounts_file
        .accounts
        .iter()
        .find(|a| a.id == id)
        .map(|a| a.is_active)
        .unwrap_or(false);

    accounts_file.accounts.retain(|a| a.id != id);

    // If deleted account was active, activate the first remaining account
    if was_active && !accounts_file.accounts.is_empty() {
        accounts_file.accounts[0].is_active = true;
    }

    save_accounts_file(&accounts_file)?;
    Ok(())
}

/// Update account phishing detection setting
#[tauri::command]
pub fn update_account_phishing_detection(
    id: String,
    phishing_detection_enabled: bool,
) -> Result<AccountConfig, String> {
    let mut accounts_file = load_accounts_file()?;

    let account = accounts_file
        .accounts
        .iter_mut()
        .find(|a| a.id == id)
        .ok_or_else(|| "账户不存在".to_string())?;

    account.phishing_detection_enabled = phishing_detection_enabled;

    let updated = account.clone();
    save_accounts_file(&accounts_file)?;

    Ok(updated)
}

/// Update account signature HTML
#[tauri::command]
pub fn update_account_signature(
    id: String,
    signature_html: String,
) -> Result<AccountConfig, String> {
    let mut accounts_file = load_accounts_file()?;

    let account = accounts_file
        .accounts
        .iter_mut()
        .find(|a| a.id == id)
        .ok_or_else(|| "账户不存在".to_string())?;

    account.signature_html = signature_html;

    let updated = account.clone();
    save_accounts_file(&accounts_file)?;

    Ok(updated)
}