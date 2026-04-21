//! AI client for phishing detection using async-openai
//!
//! Provides async OpenAI-compatible API client for email phishing analysis.

use async_openai::{
    Client,
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestMessage,
        CreateChatCompletionRequestArgs,
    },
};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use tokio::time::timeout;
use std::path::PathBuf;
use std::fs;
use std::io::Read;

/// Phishing detection result from AI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhishingResult {
    pub is_phishing: bool,
    pub reasons: Vec<String>,
}

/// AI configuration for phishing detection
#[derive(Debug, Clone, Deserialize)]
pub struct AiConfig {
    pub provider: String,
    pub base_url: String,
    pub api_key: String,
    pub model: String,
    pub enabled: bool,
    /// Mock 模式：强制返回钓鱼检测结果用于测试
    #[serde(default)]
    pub mock_phishing: bool,
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            provider: "openai".to_string(),
            base_url: "https://api.openai.com/v1".to_string(),
            api_key: "".to_string(),
            model: "gpt-4o-mini".to_string(),
            enabled: false,
            mock_phishing: false,
        }
    }
}

/// Default phishing detection prompt
const DEFAULT_PHISHING_PROMPT: &str = include_str!("../../assets/phishing-prompt-default.md");

/// Get the configuration directory path
fn get_config_dir() -> Result<PathBuf, String> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| "无法确定配置目录".to_string())?
        .join("aethermail");

    fs::create_dir_all(&config_dir)
        .map_err(|e| format!("创建配置目录失败: {}", e))?;

    log::debug!("[AI] 配置目录路径: {:?}", config_dir);
    Ok(config_dir)
}

/// Get the AI configuration file path
fn get_ai_config_path() -> Result<PathBuf, String> {
    let path = get_config_dir()?.join("ai-provider.json");
    log::debug!("[AI] AI 配置文件路径: {:?}", path);
    Ok(path)
}

/// Get the phishing prompt file path
fn get_phishing_prompt_path() -> Result<PathBuf, String> {
    let path = get_config_dir()?.join("phishing-prompt.md");
    log::debug!("[AI] 钓鱼提示词文件路径: {:?}", path);
    Ok(path)
}

/// Load AI configuration from file
pub fn load_ai_config() -> Result<AiConfig, String> {
    log::info!("[AI] ========== 加载 AI 配置 ==========");

    let path = get_ai_config_path()?;
    log::info!("[AI] 配置文件路径: {:?}", path);

    if !path.exists() {
        log::info!("[AI] 配置文件不存在，使用默认配置");
        log::info!("[AI] 默认配置: provider=openai, base_url=https://api.openai.com/v1, model=gpt-4o-mini, enabled=false");
        return Ok(AiConfig::default());
    }

    let mut file = fs::File::open(&path)
        .map_err(|e| {
            log::error!("[AI] 打开配置文件失败: {}", e);
            format!("打开 AI 配置文件失败: {}", e)
        })?;

    let mut content = String::new();
    file.read_to_string(&mut content)
        .map_err(|e| {
            log::error!("[AI] 读取配置文件失败: {}", e);
            format!("读取 AI 配置文件失败: {}", e)
        })?;

    log::info!("[AI] 配置文件内容长度: {} 字符", content.len());
    log::debug!("[AI] 配置文件原始内容: {}", content);

    let config: AiConfig = serde_json::from_str(&content)
        .map_err(|e| {
            log::error!("[AI] 解析配置文件失败: {}", e);
            log::error!("[AI] 配置文件内容: {}", content);
            format!("解析 AI 配置文件失败: {}", e)
        })?;

    log::info!("[AI] 配置解析成功:");
    log::info!("[AI]   provider: {}", config.provider);
    log::info!("[AI]   base_url: {}", config.base_url);
    log::info!("[AI]   model: {}", config.model);
    log::info!("[AI]   enabled: {}", config.enabled);
    log::info!("[AI]   api_key长度: {} 字符", config.api_key.len());
    log::info!("[AI]   api_key前缀: {}...", if config.api_key.len() > 8 { &config.api_key[..8] } else { "(空或太短)" });
    log::info!("[AI] ========== AI 配置加载完成 ==========");

    Ok(config)
}

/// Load phishing detection prompt from file
pub fn load_phishing_prompt() -> Result<String, String> {
    log::info!("[AI] ========== 加载钓鱼提示词 ==========");

    let path = get_phishing_prompt_path()?;

    // If file doesn't exist, create with default content
    if !path.exists() {
        log::info!("[AI] 钓鱼提示词文件不存在，创建默认文件: {:?}", path);
        fs::write(&path, DEFAULT_PHISHING_PROMPT)
            .map_err(|e| {
                log::error!("[AI] 创建钓鱼提示词文件失败: {}", e);
                format!("创建钓鱼提示词文件失败: {}", e)
            })?;
        log::info!("[AI] 已创建默认提示词文件，长度: {} 字符", DEFAULT_PHISHING_PROMPT.len());
        return Ok(DEFAULT_PHISHING_PROMPT.to_string());
    }

    let mut file = fs::File::open(&path)
        .map_err(|e| {
            log::error!("[AI] 打开钓鱼提示词文件失败: {}", e);
            format!("打开钓鱼提示词文件失败: {}", e)
        })?;

    let mut content = String::new();
    file.read_to_string(&mut content)
        .map_err(|e| {
            log::error!("[AI] 读取钓鱼提示词文件失败: {}", e);
            format!("读取钓鱼提示词文件失败: {}", e)
        })?;

    log::info!("[AI] 已加载钓鱼提示词文件，长度: {} 字符", content.len());
    log::debug!("[AI] 提示词内容前500字符: {}", if content.len() > 500 { &content[..500] } else { &content });
    log::info!("[AI] ========== 钓鱼提示词加载完成 ==========");

    Ok(content)
}

/// Check if AI is available and enabled (or mock mode)
pub fn is_ai_available() -> bool {
    log::info!("[AI] 检查 AI 可用性...");

    match load_ai_config() {
        Ok(config) => {
            // Mock 模式直接返回可用
            if config.mock_phishing {
                log::info!("[AI] ⚠️ Mock 钓鱼模式已启用，AI 可用");
                log::info!("[AI] AI 可用性检查结果: true (mock_phishing=true)");
                return true;
            }

            let available = config.enabled && !config.api_key.is_empty() && !config.base_url.is_empty();
            log::info!("[AI] AI 可用性检查结果: {}", available);
            log::info!("[AI]   enabled={}, api_key非空={}, base_url非空={}",
                config.enabled,
                !config.api_key.is_empty(),
                !config.base_url.is_empty()
            );
            available
        }
        Err(e) => {
            log::error!("[AI] 加载配置失败: {}", e);
            log::info!("[AI] AI 可用性检查结果: false");
            false
        }
    }
}

/// AI client for phishing detection
pub struct AiClient {
    config: AiConfig,
    client: Option<Client<OpenAIConfig>>,
    mock_mode: bool,
}

impl AiClient {
    /// Create a new AI client from configuration
    pub fn new() -> Result<Self, String> {
        log::info!("[AI] ========== 创建 AI 客户端 ==========");

        let config = load_ai_config()?;

        // Mock 模式：不需要真实配置，直接返回 mock 客户端
        if config.mock_phishing {
            log::info!("[AI] ⚠️ Mock 钓鱼模式已启用，返回 Mock 客户端");
            return Ok(Self {
                config,
                client: None,
                mock_mode: true,
            });
        }

        // Check if AI is enabled and has valid configuration
        if !config.enabled {
            log::warn!("[AI] AI 未启用 (enabled=false)");
            log::info!("[AI] 返回空客户端（不可用）");
            return Ok(Self {
                config,
                client: None,
                mock_mode: false,
            });
        }

        if config.api_key.is_empty() {
            log::warn!("[AI] API Key 为空");
            log::info!("[AI] 返回空客户端（不可用）");
            return Ok(Self {
                config,
                client: None,
                mock_mode: false,
            });
        }

        if config.base_url.is_empty() {
            log::warn!("[AI] Base URL 为空");
            log::info!("[AI] 返回空客户端（不可用）");
            return Ok(Self {
                config,
                client: None,
                mock_mode: false,
            });
        }

        log::info!("[AI] 配置完整，创建 OpenAI 客户端...");
        log::info!("[AI]   API Base: {}", config.base_url);
        log::info!("[AI]   API Key: {}...{}",
            if config.api_key.len() > 8 { &config.api_key[..8] } else { "" },
            if config.api_key.len() > 4 { &config.api_key[config.api_key.len()-4..] } else { "" }
        );
        log::info!("[AI]   Model: {}", config.model);

        // Create OpenAI config with custom base URL
        let openai_config = OpenAIConfig::new()
            .with_api_key(&config.api_key)
            .with_api_base(&config.base_url);

        let client = Client::with_config(openai_config);

        log::info!("[AI] OpenAI 客户端创建成功");
        log::info!("[AI] provider={}, model={}", config.provider, config.model);
        log::info!("[AI] ========== AI 客户端创建完成 ==========");

        Ok(Self {
            config,
            client: Some(client),
            mock_mode: false,
        })
    }

    /// Enable mock mode for testing
    pub fn with_mock_mode(mut self, enabled: bool) -> Self {
        log::info!("[AI] 设置 Mock 模式: {}", enabled);
        self.mock_mode = enabled;
        self
    }

    /// Check if client is available
    pub fn is_available(&self) -> bool {
        let available = self.mock_mode || (self.client.is_some() && self.config.enabled);
        log::info!("[AI] 客户端可用性: {} (mock={}, client={}, enabled={})",
            available, self.mock_mode, self.client.is_some(), self.config.enabled);
        available
    }

    /// Detect phishing in email content
    pub async fn detect_phishing(
        &self,
        email_content: &str,
        timeout_ms: u64,
    ) -> Result<PhishingResult, String> {
        log::info!("[AI] ========== 开始钓鱼检测 ==========");
        log::info!("[AI] 邮件内容长度: {} 字符", email_content.len());
        log::debug!("[AI] 邮件内容: {}", if email_content.len() > 1000 { &email_content[..1000] } else { email_content });
        log::info!("[AI] 超时设置: {}ms", timeout_ms);

        // Mock mode: return simulated result
        if self.mock_mode {
            log::info!("[AI] ⚠️ Mock 模式已启用，返回模拟结果");
            let result = mock_phishing_response();
            log::info!("[AI] Mock 结果: is_phishing={}, reasons={:?}", result.is_phishing, result.reasons);
            log::info!("[AI] ========== 钓鱼检测完成(Mock) ==========");
            return Ok(result);
        }

        // Check if client is available
        let client = self.client.as_ref()
            .ok_or_else(|| {
                log::error!("[AI] 客户端未初始化");
                "AI 客户端未初始化".to_string()
            })?;

        // Load system prompt
        log::info!("[AI] 加载系统提示词...");
        let system_prompt = load_phishing_prompt()?;
        log::info!("[AI] ========== SYSTEM PROMPT (完整内容) ==========");
        log::info!("{}", system_prompt);
        log::info!("[AI] ========== SYSTEM PROMPT END ==========");
        log::info!("[AI] 系统提示词长度: {} 字符", system_prompt.len());

        // 打印用户消息
        log::info!("[AI] ========== USER MESSAGE (完整内容) ==========");
        log::info!("{}", email_content);
        log::info!("[AI] ========== USER MESSAGE END ==========");
        log::info!("[AI] 用户消息长度: {} 字符", email_content.len());

        // Build chat completion request
        log::info!("[AI] 构建 API 请求...");
        log::info!("[AI] 请求参数:");
        log::info!("[AI]   model: {}", self.config.model);
        log::info!("[AI]   temperature: 0.1");
        log::info!("[AI]   max_tokens: 500");
        log::info!("[AI]   system_prompt长度: {}", system_prompt.len());
        log::info!("[AI]   user_message长度: {}", email_content.len());

        let request = CreateChatCompletionRequestArgs::default()
            .model(&self.config.model)
            .messages(vec![
                ChatCompletionRequestMessage::System(system_prompt.clone().into()),
                ChatCompletionRequestMessage::User(email_content.into()),
            ])
            .temperature(0.1)
            .max_tokens(500u16)
            .build()
            .map_err(|e| {
                log::error!("[AI] 构建请求失败: {}", e);
                format!("构建请求失败: {}", e)
            })?;

        log::info!("[AI] 请求构建成功");

        // Call API with timeout
        log::info!("[AI] 发送 API 请求到: {}", self.config.base_url);
        log::info!("[AI] 端点: /chat/completions");

        let start_time = Instant::now();
        log::info!("[AI] 请求开始时间: {:?}", start_time);

        let api_call = async {
            client.chat().create(request).await
        };

        log::info!("[AI] 等待响应（最长 {}ms）...", timeout_ms);

        let response = timeout(Duration::from_millis(timeout_ms), api_call)
            .await
            .map_err(|_| {
                let elapsed = start_time.elapsed();
                log::error!("[AI] ⏱️ 请求超时! 已等待 {}ms (设置超时 {}ms)", elapsed.as_millis(), timeout_ms);
                format!("AI 检测超时 ({}ms)", timeout_ms)
            })?;

        let elapsed = start_time.elapsed();
        log::info!("[AI] ⏱️ API 响应耗时: {}ms", elapsed.as_millis());

        let response = response
            .map_err(|e| {
                log::error!("[AI] API 调用失败: {}", e);
                log::error!("[AI] 错误详情: {:?}", e);
                format!("AI API 调用失败: {}", e)
            })?;

        log::info!("[AI] API 响应接收成功");
        log::info!("[AI] 响应 ID: {}", response.id);
        log::info!("[AI] 响应 model: {}", response.model);
        log::info!("[AI] choices 数量: {}", response.choices.len());

        if let Some(usage) = response.usage {
            log::info!("[AI] Token 使用:");
            log::info!("[AI]   prompt_tokens: {}", usage.prompt_tokens);
            log::info!("[AI]   completion_tokens: {}", usage.completion_tokens);
            log::info!("[AI]   total_tokens: {}", usage.total_tokens);
        }

        // Extract content from response
        let content = response
            .choices
            .first()
            .and_then(|choice| {
                log::info!("[AI] choice index: {}", choice.index);
                log::info!("[AI] choice finish_reason: {:?}", choice.finish_reason);
                log::debug!("[AI] choice message role: {:?}", choice.message.role);
                choice.message.content.clone()
            })
            .ok_or_else(|| {
                log::error!("[AI] 响应内容为空");
                "AI 响应内容为空".to_string()
            })?;

        log::info!("[AI] 响应内容长度: {} 字符", content.len());
        log::info!("[AI] ========== AI 响应 (完整内容) ==========");
        log::info!("{}", content);
        log::info!("[AI] ========== AI 响应 END ==========");

        // Parse JSON response
        log::info!("[AI] 解析 JSON 响应...");
        let result = self.parse_response(&content)?;

        log::info!("[AI] 解析结果:");
        log::info!("[AI]   is_phishing: {}", result.is_phishing);
        log::info!("[AI]   reasons数量: {}", result.reasons.len());
        for (i, reason) in result.reasons.iter().enumerate() {
            log::info!("[AI]   reason[{}]: {}", i, reason);
        }
        log::info!("[AI] ========== 钓鱼检测完成 ==========");
        log::info!("[AI] 总耗时: {}ms", start_time.elapsed().as_millis());

        Ok(result)
    }

    /// Parse AI response JSON
    fn parse_response(&self, content: &str) -> Result<PhishingResult, String> {
        log::debug!("[AI] 开始解析响应...");

        // Try to extract JSON from response (handle markdown code blocks)
        let binding = content
            .trim()
            .replace("```json", "")
            .replace("```", "");
        let json_str = binding.trim();

        log::debug!("[AI] 清理后的 JSON 字符串长度: {} 字符", json_str.len());
        log::debug!("[AI] 清理后的 JSON: {}", json_str);

        // Parse JSON
        let result: PhishingResult = serde_json::from_str(json_str)
            .map_err(|e| {
                log::error!("[AI] JSON 解析失败: {}", e);
                log::error!("[AI] 待解析内容: {}", json_str);
                format!("解析 AI 响应失败: {}", e)
            })?;

        log::info!("[AI] JSON 解析成功");

        // Validate: if phishing, must have reasons
        if result.is_phishing && result.reasons.is_empty() {
            log::warn!("[AI] 钓鱼邮件判定但无原因，添加默认原因");
            return Ok(PhishingResult {
                is_phishing: true,
                reasons: vec!["AI 判定为钓鱼邮件但未提供具体原因".to_string()],
            });
        }

        Ok(result)
    }
}

/// Default mock phishing response for testing
pub fn mock_phishing_response() -> PhishingResult {
    log::info!("[AI] 生成 Mock 钓鱼检测结果");
    PhishingResult {
        is_phishing: true,
        reasons: vec![
            "Mock: 发件人地址可疑".to_string(),
            "Mock: 链接域名与官方不符".to_string(),
        ],
    }
}