## Why

钓鱼邮件是网络安全的主要威胁之一，用户难以识别复杂的钓鱼攻击。现有邮箱客户端依赖基础的规则过滤，缺乏智能化的钓鱼邮件研判能力。通过集成 AI 大模型，可以基于邮件内容、发送方特征、链接等多维度因素进行智能判断，有效提升钓鱼邮件识别率，保护用户安全。

当前项目缺少 AI Provider 配置基础设施，需要先构建支持 OpenAI 协议的通用 AI 调用接口，以兼容百炼、DeepSeek、智谱 AI 等国内大模型提供商。

## What Changes

### Phase 1: AI Provider 基础设施

- **新增功能**：通用 AI Provider 配置系统
  - 实现 OpenAI 协议接口（支持自定义 baseURL）
  - 支持配置 API Key（明文 JSON 存储，与账户配置同目录）
  - 兼容国内大模型提供商（百炼、DeepSeek、智谱 AI、Moonshot 等）
  - 设置页新增 AI Provider 配置区域

### Phase 2: 钓鱼邮件检测功能

- **新增功能**：AI 钓鱼邮件实时研判功能
  - 邮件同步时实时调用 AI 进行钓鱼判断
  - 疑似钓鱼邮件自动移入独立的"钓鱼邮件"分类
  - 用户可在账户管理中开启/关闭该功能

- **配置扩展**：
  - 新增钓鱼邮件规则配置文件（`phishing-rules.md`）
  - 提供默认规则模板，用户可自定义修改

- **UI 新增**：
  - 设置页新增 AI 钓鱼检测开关（账户级别）
  - 侧边栏新增"钓鱼邮件"分类入口
  - 钓鱼邮件列表显示研判置信度

## Capabilities

### New Capabilities

- `ai-provider`: OpenAI 协议兼容的 AI Provider 配置系统，支持自定义 baseURL 和 API Key，兼容国内外大模型提供商
- `phishing-detection`: AI 钓鱼邮件检测功能，包括实时研判、规则配置、分类管理
- `phishing-folder`: 钓鱼邮件独立分类文件夹，包括列表展示、置信度显示、邮件移动操作

### Modified Capabilities

- `account-persistence`: 账户配置新增钓鱼检测开关字段（`phishing_detection_enabled`）
- `settings-page`: 设置页新增 AI Provider 配置区域和钓鱼检测配置区域
- `email-sync`: 同步流程中集成钓鱼邮件实时检测逻辑
- `email-storage`: 邮件存储新增钓鱼标记字段和钓鱼分类

## Impact

### 配置文件变更

配置文件统一存储在应用配置目录：

| 操作系统 | 配置目录 |
|----------|----------|
| Windows | `%APPDATA%/aethermail/` |
| macOS | `~/Library/Application Support/aethermail/` |
| Linux | `~/.config/aethermail/` |

配置文件列表：

| 文件 | 内容 | 格式 |
|------|------|------|
| `accounts.json` | 账户配置（新增钓鱼检测开关） | JSON 明文 |
| `ai-provider.json` | AI Provider 配置 | JSON 明文 |
| `phishing-rules.md` | 钓鱼邮件规则配置 | Markdown |

### 数据库变更

- SQLite `emails` 表: 新增 `is_phishing` 和 `phishing_confidence` 字段

### AI Provider 配置结构

```json
{
  "provider": "openai",
  "base_url": "https://api.openai.com/v1",
  "api_key": "sk-xxx",
  "model": "gpt-4o-mini",
  "enabled": true
}
```

### 新增文件

- `src-tauri/src/commands/ai.rs`: AI Provider Rust 命令
- `src/services/ai/`: AI Provider 前端服务
- `src/services/phishing/`: 钓鱼检测服务模块
- `src/components/settings/AiSettings.vue`: AI 配置 UI 组件
- `src/components/phishing/`: 钓鱼邮件 UI 组件

### 新增依赖

- `openai`: OpenAI SDK（兼容 OpenAI 协议的各家提供商）

### 兼容的大模型提供商

| Provider | baseURL | 说明 |
|----------|---------|------|
| OpenAI | `https://api.openai.com/v1` | 默认 OpenAI API |
| 百炼 (阿里云) | `https://dashscope.aliyuncs.com/compatible-mode/v1` | Qwen 系列 |
| DeepSeek | `https://api.deepseek.com/v1` | DeepSeek 系列 |
| Moonshot | `https://api.moonshot.cn/v1` | Kimi 系列 |
| 智谱 AI | `https://open.bigmodel.cn/api/paas/v4` | GLM 系列 |
| SiliconFlow | `https://api.siliconflow.cn/v1` | 多模型聚合 |
| 本地 Ollama | `http://localhost:11434/v1` | 本地运行 |