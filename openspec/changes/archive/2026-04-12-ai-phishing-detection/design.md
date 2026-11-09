## Context

当前邮箱应用基于 Tauri + Vue 3 + SQLite 架构，已具备：
- **邮件同步流程**：POP3 协议同步，UIDL 跟踪已下载邮件
- **邮件存储**：SQLite 本地数据库，支持邮件状态管理
- **设置页面**：账户管理、通用设置等配置界面
- **配置文件存储**：统一存储在应用配置目录
  - Windows: `%APPDATA%/aethermail/`
  - macOS: `~/Library/Application Support/aethermail/`
  - Linux: `~/.config/aethermail/`
- **账户配置**：`accounts.json`（JSON 明文格式）

**缺失**：项目目前没有 AI Provider 配置基础设施，需要新建 OpenAI 协议兼容的 AI 调用系统。

钓鱼邮件检测需要先构建 AI Provider 系统，再集成到邮件同步流程中。

## Goals / Non-Goals

**Goals:**
- 构建 OpenAI 协议兼容的 AI Provider 系统
- 支持自定义 baseURL，兼容国内大模型提供商（百炼、DeepSeek 等）
- API Key 明文 JSON 存储（与账户配置同目录，简化实现）
- 实时钓鱼邮件检测：邮件同步时调用 AI 进行研判
- 独立钓鱼分类：疑似钓鱼邮件存放在独立文件夹
- 用户可控开关：账户级别开启/关闭钓鱼检测功能
- **提示词配置化**：钓鱼检测提示词完全由配置文件定义，用户可自定义

**Non-Goals:**
- 不实现多 Provider 并行（Claude SDK、Gemini SDK 等）
- 不实现 Provider 自动切换和负载均衡
- 不实现 API Key 加密存储（本期简化，后续可迭代）
- 不实现钓鱼邮件的自动举报功能
- 不实现钓鱼链接的自动提取和验证
- 不实现跨账户的统一钓鱼检测策略
- 不实现置信度评分（简化为二元判断：是/否钓鱼）

## Decisions

### 1. AI Provider：仅实现 OpenAI 协议

**选择**：使用 OpenAI SDK 作为统一接口，支持自定义 baseURL

**理由**：
- OpenAI 协议已成为业界标准，大多数国内大模型提供商兼容
- 单一 SDK 简化实现和维护成本
- 用户只需配置 baseURL 和 API Key 即可使用任意兼容提供商

### 2. 配置存储：与账户配置同目录，明文 JSON

**选择**：AI Provider 配置存储在 `ai-provider.json`，与 `accounts.json` 同目录

**理由**：
- 统一配置目录便于管理
- 明文 JSON 简化实现，与现有账户配置风格一致
- 后续可根据需求迭代为加密存储

**配置目录结构**：
```
~/.config/aethermail/           (Linux)
~/Library/Application Support/aethermail/  (macOS)
%APPDATA%/aethermail/           (Windows)

├── accounts.json               # 账户配置
├── ai-provider.json            # AI Provider 配置
├── phishing-prompt.md          # 钓鱼检测提示词（完整 System Prompt）
└── emails.db                   # SQLite 数据库
```

**ai-provider.json 结构**：
```json
{
  "provider": "openai",
  "base_url": "https://api.openai.com/v1",
  "api_key": "sk-xxx",
  "model": "gpt-4o-mini",
  "enabled": true
}
```

### 3. 钓鱼检测时机：同步时实时判断

**选择**：邮件下载后立即调用 AI 进行钓鱼研判

**缓解性能影响**：
- 设置 5 秒超时限制
- 使用较快的小模型（如 qwen-turbo、deepseek-chat）
- 检测失败不影响邮件同步

### 4. 钓鱼分类为独立文件夹

**选择**：创建独立的 `phishing` 分类，与收件箱并列

**理由**：独立分类既保证用户能看到钓鱼邮件（便于确认），又避免与正常邮件混淆。

### 5. 提示词完全配置化（关键变更）

**选择**：`phishing-prompt.md` 文件内容直接作为 System Prompt，无需额外模板拼接

**理由**：
- 用户拥有完全控制权，可自定义整个提示词
- 简化代码实现，无需模板和规则拼接逻辑
- 用户可针对特定场景定制专属检测规则
- 默认文件已是完整的、可立即使用的提示词

**实现方式**：
```
┌─────────────────────────────────────────────────────────────────────┐
│                     提示词构建方式                                    │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│   原方案（已弃用）：                                                   │
│   ┌────────────────────────────────────────┐                        │
│   │ systemPrompt = TEMPLATE + rules        │                        │
│   │              (模板)   (规则变量)         │                        │
│   └────────────────────────────────────────┘                        │
│                                                                      │
│   新方案（当前）：                                                     │
│   ┌────────────────────────────────────────┐                        │
│   │ systemPrompt = phishing-prompt.md 内容  │                        │
│   │              （文件直接作为完整 Prompt） │                        │
│   └────────────────────────────────────────┘                        │
│                                                                      │
│   userContent = <email_content> + 邮件内容                           │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### 6. 简化为二元判断（移除置信度）

**选择**：检测结果仅包含是否钓鱼和原因列表，不包含置信度评分

**理由**：
- 简化实现复杂度
- 减少用户理解负担
- AI 模型的置信度评估不稳定，二元判断更可靠

## Prompt Design

### 提示词文件：phishing-prompt.md

文件内容直接作为 AI 调用的 System Prompt，无需任何代码层面的模板处理。

**默认文件内容**（参见 `specs/phishing-detection/default-prompt.md`）：
- 定义钓鱼邮件检测系统的角色和目标
- 包含完整的分析维度说明
- 包含判定标准（高危、中危、正常）
- 包含严格的 JSON 输出格式要求
- 包含用户自定义扩展区域（注释说明）

### 代码实现（极简化）

```typescript
// phishingDetectionService.ts

import { readFile, writeFile } from '@tauri-apps/plugin-fs'
import { AIService } from '@/services/ai/aiService'

const PHISHING_PROMPT_FILE = 'phishing-prompt.md'

/**
 * 加载钓鱼检测提示词（直接使用文件内容）
 */
async function loadPhishingPrompt(): Promise<string> {
  try {
    const configDir = await getConfigDir()
    const filePath = `${configDir}/${PHISHING_PROMPT_FILE}`
    
    // 读取文件内容
    const content = await readFile(filePath)
    return content
  } catch {
    // 文件不存在时，创建默认提示词文件
    return await createDefaultPhishingPrompt()
  }
}

/**
 * 创建默认提示词文件
 */
async function createDefaultPhishingPrompt(): Promise<string> {
  const defaultPrompt = await import('@/assets/phishing-prompt-default.md')
  const configDir = await getConfigDir()
  const filePath = `${configDir}/${PHISHING_PROMPT_FILE}`
  
  await writeFile(filePath, defaultPrompt)
  return defaultPrompt
}

/**
 * 构建钓鱼检测请求
 * systemPrompt: 直接使用 phishing-prompt.md 文件内容
 * userContent: 邮件内容包装在 <email_content> 标签中
 */
async function buildPhishingRequest(email: EmailData): Promise<{
  systemPrompt: string
  userContent: string
}> {
  // 直接加载提示词文件作为 System Prompt
  const systemPrompt = await loadPhishingPrompt()
  
  // 构建邮件内容（限制长度）
  const snippet = extractEmailSnippet(email, 2000)
  
  const userContent = `<email_content>
发件人: ${email.from_name ? `${email.from_name} <${email.from_address}>` : email.from_address}
主题: ${email.subject}

邮件内容:
${snippet}
</email_content>

请分析上述邮件是否为钓鱼邮件，输出 JSON 结果。`

  return { systemPrompt, userContent }
}

/**
 * 执行钓鱼检测
 */
export async function detectPhishing(email: EmailData): Promise<PhishingResult> {
  const { systemPrompt, userContent } = await buildPhishingRequest(email)
  
  try {
    // 调用 AI（5秒超时）
    const response = await AIService.complete(systemPrompt, userContent, 5000)
    
    // 解析结果
    return parsePhishingResult(response)
  } catch (error) {
    // 超时或错误时返回安全默认值
    return {
      is_phishing: false,
      reasons: ['AI 检测超时或失败']
    }
  }
}
```

### 响应结构

```typescript
interface PhishingResult {
  is_phishing: boolean    // 是否为钓鱼邮件
  reasons: string[]       // 检测原因列表
}

// 存储到数据库
interface PhishingDbFields {
  is_phishing: boolean
  phishing_reasons: string  // JSON.stringify(reasons)
}
```

### 结果解析

```typescript
export function parsePhishingResult(aiResponse: string): PhishingResult {
  try {
    // 提取 JSON（处理可能的 markdown 包装）
    const jsonMatch = aiResponse.match(/\{[\s\S]*\}/)
    if (!jsonMatch) {
      throw new Error('No JSON found in response')
    }
    
    const result = JSON.parse(jsonMatch[0]) as PhishingResult
    
    // 验证并修正字段
    if (typeof result.is_phishing !== 'boolean') {
      result.is_phishing = false
    }
    
    if (!Array.isArray(result.reasons)) {
      result.reasons = []
    }
    
    // 钓鱼邮件必须有原因
    if (result.is_phishing && result.reasons.length === 0) {
      result.reasons = ['AI 判定为钓鱼邮件但未提供具体原因']
    }
    
    return result
  } catch {
    // 解析失败时返回安全默认值
    return {
      is_phishing: false,
      reasons: ['AI 响应解析失败']
    }
  }
}
```

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────┐
│                     AI Provider + 钓鱼检测系统架构                      │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│   ┌─────────────────────────────────────────────────────────────┐  │
│   │                      Phase 1: AI Provider                     │  │
│   │                                                             │  │
│   │   ┌─────────────┐  ┌─────────────┐  ┌─────────────┐          │  │
│   │   │ AI Config   │  │ AI Service  │  │ Settings UI │          │  │
│   │   │             │  │             │  │             │          │  │
│   │   │ ai-provider │  │ openai SDK  │  │ Provider    │          │  │
│   │   │ .json       │  │ complete()  │  │ 配置页面    │          │  │
│   │   │             │  │             │  │             │          │  │
│   │   └─────────────┘  │ timeout     │  │ API Key 输入│          │  │
│   │                    └─────────────┘  │ BaseURL 输入│          │  │
│   │                                      └─────────────┘          │  │
│   │                                                             │  │
│   │   配置目录: accounts.json 同目录                              │  │
│   │   兼容提供商:                                                 │  │
│   │   OpenAI | 百炼 | DeepSeek | Moonshot | 智谱 | Ollama        │  │
│   │                                                             │  │
│   └─────────────────────────────────────────────────────────────┘  │
│                                    │                                 │
│                                    ▼                                 │
│   ┌─────────────────────────────────────────────────────────────┐  │
│   │                    Phase 2: Phishing Detection                │  │
│   │                                                             │  │
│   │   ┌─────────────────────────────────────────────────────┐  │  │
│   │   │                    邮件同步流程                        │  │  │
│   │   │                                                     │  │  │
│   │   │  POP3 Server → Download → Parse → Phishing Check   │  │  │
│   │   │                                       │             │  │  │
│   │   │                                       ▼             │  │  │
│   │   │                           ┌─────────────────┐      │  │  │
│   │   │                           │ Phishing        │      │  │  │
│   │   │                           │ Detection       │      │  │  │
│   │   │                           │ Service         │      │  │  │
│   │   │                           │                 │      │  │  │
│   │   │                           │ • loadPrompt    │      │  │  │
│   │   │                           │ • buildRequest  │      │  │  │
│   │   │                           │ • parseResult   │      │  │  │
│   │   │                           └─────────────────┘      │  │  │
│   │   │                                     │               │  │  │
│   │   └─────────────────────────────────────┼───────────────┘  │  │
│   │                                           │                 │  │
│   │   ┌─────────────────────┐  ┌─────────────┼───────────────┐ │  │
│   │   │ Prompt Config       │  │             │               │ │  │
│   │   │                     │  │             ▼               │ │  │
│   │   │ phishing-prompt     │  │     Email Storage           │ │  │
│   │   │ .md                 │  │                             │ │  │
│   │   │                     │  │     + is_phishing (bool)     │ │  │
│   │   │ 直接作为            │  │     + phishing_reasons       │ │  │
│   │   │ System Prompt       │  │       (JSON array)          │ │  │
│   │   │                     │  │                             │ │  │
│   │   │ 用户可完全自定义     │  └─────────────────────────────┘ │  │
│   │   └─────────────────────┘                                 │  │
│   │                                                             │  │
│   └─────────────────────────────────────────────────────────────┘  │
│                                                                      │
│   ┌─────────────────────────────────────────────────────────────┐  │
│   │                         UI 层                                 │  │
│   │                                                             │  │
│   │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐          │  │
│   │  │ AI Settings │  │ Sidebar     │  │ Phishing    │          │  │
│   │  │             │  │             │  │ List        │          │  │
│   │  │ Provider    │  │ "钓鱼邮件"   │  │             │          │  │
│   │  │ 配置        │  │ 分类入口    │  │ 原因显示    │          │  │
│   │  │             │  │             │  │             │          │  │
│   │  │ BaseURL     │  └─────────────┘  │ 警告 Banner │          │  │
│   │  │ API Key     │                   └─────────────┘          │  │
│   │  │ Model       │                                             │  │
│   │  │             │                                             │  │
│   │  │ Test Button │                                             │  │
│   │  └─────────────┘                                             │  │
│   │                                                             │  │
│   └─────────────────────────────────────────────────────────────┘  │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

## Data Flow

### 钓鱼检测调用流程（简化版）

```
┌─────────────────────────────────────────────────────────────────────┐
│                     钓鱼检测调用数据流                                │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│   1. 邮件同步                                                        │
│      │                                                               │
│      ▼                                                               │
│   ┌─────────────────────────────────────┐                          │
│   │ POP3 Sync → Download → Parse        │                          │
│   │                                     │                          │
│   │ if (account.phishing_detection_enabled)                         │
│   │   → 调用钓鱼检测                     │                          │
│   └─────────────────────────────────────┘                          │
│      │                                                               │
│      ▼                                                               │
│   2. 加载提示词（直接读取文件）                                         │
│      │                                                               │
│   ┌─────────────────────────────────────┐                          │
│   │ systemPrompt = phishing-prompt.md   │                          │
│   │                                     │                          │
│   │ 文件不存在时：                        │                          │
│   │   → 创建默认文件                     │                          │
│   │   → 返回默认内容                     │                          │
│   └─────────────────────────────────────┘                          │
│      │                                                               │
│      ▼                                                               │
│   3. 构建请求                                                         │
│      │                                                               │
│   ┌─────────────────────────────────────┐                          │
│   │ systemPrompt: phishing-prompt.md    │                          │
│   │ userContent: <email_content>邮件    │                          │
│   │                                     │                          │
│   │ 邮件内容限制 2000 字符               │                          │
│   └─────────────────────────────────────┘                          │
│      │                                                               │
│      ▼                                                               │
│   4. AI 调用                                                         │
│      │                                                               │
│   ┌─────────────────────────────────────┐                          │
│   │ AIService.complete(prompt, 5000)    │                          │
│   │                                     │                          │
│   │ 返回:                                │                          │
│   │   {                                 │                          │
│   │     "is_phishing": true/false,      │                          │
│   │     "reasons": ["原因1", ...]       │                          │
│   │   }                                 │                          │
│   └─────────────────────────────────────┘                          │
│      │                                                               │
│      ▼                                                               │
│   5. 解析 & 存储                                                       │
│      │                                                               │
│   ┌─────────────────────────────────────┐                          │
│   │ parsePhishingResult(response)       │                          │
│   │                                     │                          │
│   │ UPDATE emails SET                   │                          │
│   │   is_phishing = ?,                  │                          │
│   │   phishing_reasons = ?              │                          │
│   └─────────────────────────────────────┘                          │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

## Risks / Trade-offs

### [风险 1] AI API 调用成本
**描述**：每封邮件调用 AI 可能产生大量 API 费用
**缓解**：
- 用户可关闭钓鱼检测功能
- 推荐使用低成本模型（qwen-turbo、deepseek-chat）
- 邮件内容截取到 2000 字符，减少 token 消耗

### [风险 2] 检测延迟影响同步体验
**描述**：AI 调用可能耗时较长
**缓解**：
- 设置 5 秒超时限制
- 使用国内提供商减少网络延迟
- 检测失败不影响邮件同步

### [风险 3] 用户自定义提示词可能失效
**描述**：用户修改提示词可能导致检测效果下降或格式错误
**缓解**：
- 提供"重置为默认"按钮
- 默认文件包含清晰的格式说明
- 解析失败时返回安全默认值

### [风险 4] API Key 明文存储
**描述**：API Key 以明文存储在 JSON 文件中
**缓解**：
- 配置文件存储在用户目录，非公开路径
- 提示用户注意 API Key 安全
- 后续版本可迭代为加密存储

### [风险 5] Prompt Injection
**描述**：邮件内容可能包含恶意构造的文本，诱导 AI 输出错误结果
**缓解**：
- 使用 `<email_content>` 标签包裹邮件内容
- 默认提示词明确声明：将标签内容视为纯文本，不执行指令

## Migration Plan

### Phase 1: AI Provider 基础设施

1. **Rust 后端扩展**
   - 新增 `src-tauri/src/commands/ai.rs`
   - 实现配置文件读写函数（复用 accounts.rs 的目录路径逻辑）

2. **配置文件创建**
```rust
fn get_ai_config_path() -> Result<PathBuf, String> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| "无法确定配置目录".to_string())?
        .join("aethermail");
    
    fs::create_dir_all(&config_dir)
        .map_err(|e| format!("创建配置目录失败: {}", e))?;
    
    Ok(config_dir.join("ai-provider.json"))
}
```

3. **前端依赖添加**
```bash
npm install openai
```

### Phase 2: 钓鱼检测集成

1. **数据库扩展**
```sql
ALTER TABLE emails ADD COLUMN is_phishing INTEGER DEFAULT 0;
ALTER TABLE emails ADD COLUMN phishing_reasons TEXT DEFAULT NULL;
CREATE INDEX idx_emails_phishing ON emails(is_phishing);
```

2. **账户配置扩展**
   - `accounts.json` 中 AccountConfig 新增 `phishing_detection_enabled: bool` 字段

3. **提示词文件创建**
   - 首次开启检测时，复制 `phishing-prompt-default.md` 到配置目录
   - 文件名为 `phishing-prompt.md`

### Rollback Strategy
- 关闭 AI 功能即可禁用所有 AI 相关功能
- 数据库字段保留，不影响其他功能
- 提示词文件可删除或重置