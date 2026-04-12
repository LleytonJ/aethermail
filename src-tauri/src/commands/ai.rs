//! AI Provider configuration commands
//!
//! Provides commands for managing AI provider configuration and phishing prompt.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;
use std::io::{Read, Write};

/// AI Provider configuration stored in JSON file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    /// Provider name (e.g., "openai", "bailian", "deepseek")
    pub provider: String,
    /// Base URL for API calls
    pub base_url: String,
    /// API key for authentication
    pub api_key: String,
    /// Model name to use
    pub model: String,
    /// Whether AI features are enabled
    pub enabled: bool,
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            provider: "openai".to_string(),
            base_url: "https://api.openai.com/v1".to_string(),
            api_key: "".to_string(),
            model: "gpt-4o-mini".to_string(),
            enabled: false,
        }
    }
}

/// Default phishing detection prompt
const DEFAULT_PHISHING_PROMPT: &str = include_str!("../../../src/assets/phishing-prompt-default.md");

/// Get the configuration directory path
fn get_config_dir_path() -> Result<PathBuf, String> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| "无法确定配置目录".to_string())?
        .join("aethermail");

    // Create directory if it doesn't exist
    fs::create_dir_all(&config_dir)
        .map_err(|e| format!("创建配置目录失败: {}", e))?;

    Ok(config_dir)
}

/// Get the AI configuration file path (same directory as accounts.json)
fn get_ai_config_path() -> Result<PathBuf, String> {
    Ok(get_config_dir_path()?.join("ai-provider.json"))
}

/// Get the phishing prompt file path
fn get_phishing_prompt_path() -> Result<PathBuf, String> {
    Ok(get_config_dir_path()?.join("phishing-prompt.md"))
}

/// Load AI configuration from file
fn load_ai_config_file() -> Result<AiConfig, String> {
    let path = get_ai_config_path()?;

    if !path.exists() {
        return Ok(AiConfig::default());
    }

    let mut file = fs::File::open(&path)
        .map_err(|e| format!("打开 AI 配置文件失败: {}", e))?;

    let mut content = String::new();
    file.read_to_string(&mut content)
        .map_err(|e| format!("读取 AI 配置文件失败: {}", e))?;

    serde_json::from_str(&content)
        .map_err(|e| format!("解析 AI 配置文件失败: {}", e))
}

/// Save AI configuration to file
fn save_ai_config_file(config: &AiConfig) -> Result<(), String> {
    let path = get_ai_config_path()?;

    let content = serde_json::to_string_pretty(config)
        .map_err(|e| format!("序列化 AI 配置失败: {}", e))?;

    let mut file = fs::File::create(&path)
        .map_err(|e| format!("创建 AI 配置文件失败: {}", e))?;

    file.write_all(content.as_bytes())
        .map_err(|e| format!("写入 AI 配置文件失败: {}", e))?;

    Ok(())
}

/// Get AI configuration
#[tauri::command]
pub fn get_ai_config() -> Result<AiConfig, String> {
    load_ai_config_file()
}

/// Save AI configuration
#[tauri::command]
pub fn save_ai_config(
    provider: String,
    base_url: String,
    api_key: String,
    model: String,
    enabled: bool,
) -> Result<AiConfig, String> {
    let config = AiConfig {
        provider,
        base_url,
        api_key,
        model,
        enabled,
    };

    save_ai_config_file(&config)?;
    Ok(config)
}

/// Update AI configuration (partial update)
#[tauri::command]
pub fn update_ai_config(
    provider: Option<String>,
    base_url: Option<String>,
    api_key: Option<String>,
    model: Option<String>,
    enabled: Option<bool>,
) -> Result<AiConfig, String> {
    let mut config = load_ai_config_file()?;

    if let Some(p) = provider {
        config.provider = p;
    }
    if let Some(b) = base_url {
        config.base_url = b;
    }
    if let Some(k) = api_key {
        config.api_key = k;
    }
    if let Some(m) = model {
        config.model = m;
    }
    if let Some(e) = enabled {
        config.enabled = e;
    }

    save_ai_config_file(&config)?;
    Ok(config)
}

/// Get the configuration directory path (for frontend)
#[tauri::command]
pub fn get_config_dir() -> Result<String, String> {
    Ok(get_config_dir_path()?.to_string_lossy().into_owned())
}

/// Check if AI provider is configured and enabled
#[tauri::command]
pub fn is_ai_available() -> Result<bool, String> {
    let config = load_ai_config_file()?;
    Ok(config.enabled && !config.api_key.is_empty() && !config.base_url.is_empty())
}

/// Load phishing detection prompt from file
#[tauri::command]
pub fn load_phishing_prompt() -> Result<String, String> {
    let path = get_phishing_prompt_path()?;

    // If file doesn't exist, create with default content
    if !path.exists() {
        log::info!("钓鱼提示词文件不存在，创建默认文件: {:?}", path);
        save_phishing_prompt_file(DEFAULT_PHISHING_PROMPT)?;
        return Ok(DEFAULT_PHISHING_PROMPT.to_string());
    }

    let mut file = fs::File::open(&path)
        .map_err(|e| format!("打开钓鱼提示词文件失败: {}", e))?;

    let mut content = String::new();
    file.read_to_string(&mut content)
        .map_err(|e| format!("读取钓鱼提示词文件失败: {}", e))?;

    log::info!("已加载钓鱼提示词文件，长度: {} 字符", content.len());
    Ok(content)
}

/// Save phishing detection prompt to file
fn save_phishing_prompt_file(content: &str) -> Result<(), String> {
    let path = get_phishing_prompt_path()?;

    let mut file = fs::File::create(&path)
        .map_err(|e| format!("创建钓鱼提示词文件失败: {}", e))?;

    file.write_all(content.as_bytes())
        .map_err(|e| format!("写入钓鱼提示词文件失败: {}", e))?;

    Ok(())
}

/// Reset phishing prompt to default content
#[tauri::command]
pub fn reset_phishing_prompt() -> Result<String, String> {
    save_phishing_prompt_file(DEFAULT_PHISHING_PROMPT)?;
    log::info!("已重置钓鱼提示词为默认内容");
    Ok(DEFAULT_PHISHING_PROMPT.to_string())
}

/// Get phishing prompt file path (for frontend display)
#[tauri::command]
pub fn get_phishing_prompt_path_str() -> Result<String, String> {
    Ok(get_phishing_prompt_path()?.to_string_lossy().into_owned())
}