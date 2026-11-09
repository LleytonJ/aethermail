## ADDED Requirements

### Requirement: AI Provider configuration storage

The system SHALL provide JSON file storage for AI Provider configuration in the same directory as accounts.json.

#### Scenario: Store AI configuration
- **WHEN** user configures AI Provider settings
- **THEN** system stores configuration in `ai-provider.json`
- **AND** file is located in same directory as `accounts.json`

#### Scenario: Load AI configuration
- **WHEN** application needs to call AI service
- **THEN** system loads configuration from `ai-provider.json`
- **AND** system initializes OpenAI SDK with loaded settings

#### Scenario: Default AI configuration
- **WHEN** application starts without existing AI configuration
- **THEN** system creates default config with `enabled: false`
- **AND** system does not require API Key until user enables AI

#### Scenario: Missing configuration file
- **WHEN** `ai-provider.json` does not exist
- **THEN** system treats AI as not configured
- **AND** system shows prompt to configure AI in settings

### Requirement: Configuration file path

The system SHALL store AI configuration in platform-specific config directory.

#### Scenario: Windows configuration path
- **WHEN** running on Windows
- **THEN** system stores config at `%APPDATA%/aethermail/ai-provider.json`

#### Scenario: macOS configuration path
- **WHEN** running on macOS
- **THEN** system stores config at `~/Library/Application Support/aethermail/ai-provider.json`

#### Scenario: Linux configuration path
- **WHEN** running on Linux
- **THEN** system stores config at `~/.config/aethermail/ai-provider.json`

### Requirement: Configuration file structure

The system SHALL use defined JSON structure for AI Provider configuration.

#### Scenario: Valid JSON structure
- **WHEN** reading or writing `ai-provider.json`
- **THEN** system uses the following structure:
```json
{
  "provider": "openai",
  "base_url": "https://api.openai.com/v1",
  "api_key": "sk-xxx",
  "model": "gpt-4o-mini",
  "enabled": true
}
```

#### Scenario: Handle corrupted config
- **WHEN** `ai-provider.json` is corrupted or invalid JSON
- **THEN** system creates new default configuration
- **AND** system logs error message

### Requirement: OpenAI protocol compatibility

The system SHALL use OpenAI SDK with configurable baseURL to support multiple AI providers.

#### Scenario: Use OpenAI official API
- **WHEN** user sets baseURL to `https://api.openai.com/v1`
- **THEN** system calls OpenAI official API
- **AND** system uses OpenAI models (gpt-4o-mini, gpt-4o, etc.)

#### Scenario: Use Chinese provider (百炼)
- **WHEN** user sets baseURL to `https://dashscope.aliyuncs.com/compatible-mode/v1`
- **THEN** system calls Alibaba DashScope API
- **AND** system uses Qwen models (qwen-turbo, qwen-plus, etc.)

#### Scenario: Use Chinese provider (DeepSeek)
- **WHEN** user sets baseURL to `https://api.deepseek.com/v1`
- **THEN** system calls DeepSeek API
- **AND** system uses DeepSeek models (deepseek-chat, deepseek-reasoner)

#### Scenario: Use local Ollama
- **WHEN** user sets baseURL to `http://localhost:11434/v1`
- **THEN** system calls local Ollama server
- **AND** system uses local models configured in Ollama

#### Scenario: Custom baseURL
- **WHEN** user enters custom baseURL
- **THEN** system validates URL format
- **AND** system uses custom baseURL for API calls

### Requirement: AI Provider settings UI

The system SHALL provide UI for configuring AI Provider in settings page.

#### Scenario: Display AI settings section
- **WHEN** user navigates to settings page
- **THEN** system displays "AI 设置" section
- **AND** section shows baseURL input, API Key input, model selector

#### Scenario: Save AI configuration
- **WHEN** user clicks save button after filling configuration
- **THEN** system validates baseURL format
- **AND** system saves configuration to `ai-provider.json`
- **AND** system shows success notification

#### Scenario: Test AI connection
- **WHEN** user clicks test connection button
- **THEN** system calls AI with simple test prompt
- **AND** system shows success or failure result
- **AND** test timeout is 10 seconds

#### Scenario: Show preset provider options
- **WHEN** user views baseURL input
- **THEN** system shows dropdown with preset providers:
  - OpenAI (官方)
  - 百炼 (阿里云)
  - DeepSeek
  - Moonshot (Kimi)
  - 智谱 AI
  - Ollama (本地)
- **AND** selecting preset fills corresponding baseURL

#### Scenario: Enable/disable AI
- **WHEN** user toggles AI enabled switch
- **THEN** system updates `enabled` field in config
- **AND** system enables/disables AI features accordingly

### Requirement: AI service interface

The system SHALL provide a unified AI service interface for calling AI completion.

#### Scenario: Call AI completion
- **WHEN** code calls AIService.complete(systemPrompt, userContent)
- **THEN** system loads AI configuration from file
- **AND** system creates OpenAI client with baseURL and API Key
- **AND** system sends chat completion request with specified model
- **AND** system returns completion text

#### Scenario: Handle AI call timeout
- **WHEN** AI call exceeds configured timeout (default 5 seconds)
- **THEN** system aborts the request
- **AND** system throws AiTimeoutError
- **AND** system logs timeout event

#### Scenario: Handle AI call error
- **WHEN** AI call fails with error (401, 429, network)
- **THEN** system throws appropriate AiError with error type
- **AND** error types include: AUTH_ERROR, RATE_LIMITED, NETWORK_ERROR

#### Scenario: Check AI availability
- **WHEN** code calls AIService.isAvailable()
- **THEN** system checks if AI is enabled in config
- **AND** system checks if API Key is configured
- **AND** system returns boolean availability status