## Why

当前邮件撰写界面的地址输入组件只能逐个输入邮箱地址。当用户粘贴或输入多个用逗号、分号或换行分隔的地址时，整个字符串被当作单个地址处理，导致验证失败。此外，系统不支持带显示名称的邮箱格式（如 `"张三" <zhangsan@example.com>`），限制了用户的使用场景。

这是一个影响用户体验的关键问题，需要优先解决。

## What Changes

- 支持多地址批量输入，自动识别逗号（`,`）、分号（`;`）、换行符作为分隔符
- 支持带显示名称的邮箱格式解析（`"显示名称" <email@example.com>` 或 `显示名称 <email@example.com>`）
- 支持纯邮箱地址格式（`email@example.com`）
- 在粘贴、输入分隔符、Enter/Tab、失焦时自动解析并验证地址
- 解析结果包含无效地址时，拒绝全部添加并显示错误提示
- 地址标签显示完整格式（带名称时显示 `"名称" <email>`，否则显示纯邮箱）
- 数据结构从 `string[]` 升级为 `EmailAddress[]`（包含 `email` 和可选的 `name` 字段）
- 发送邮件时将 `EmailAddress` 格式化为 SMTP 兼容的字符串格式

## Capabilities

### New Capabilities

- `address-parsing`: 多地址解析能力，支持分隔符分割、显示名称提取、格式验证
- `address-display`: 地址显示能力，支持带名称和不带名称的格式化展示

### Modified Capabilities

- `email-compose`: 邮件撰写功能的收件人/抄送输入逻辑变更，数据结构升级

## Impact

### 前端代码

- `src/components/ui/AddressInput.vue` - 主要改动，新增解析逻辑、数据结构变更
- `src/components/email/ComposeEmail.vue` - 数据结构适配、发送逻辑调整
- `src/services/email/emailService.ts` - 类型定义新增

### 后端代码

无需改动。后端 SMTP 客户端（lettre 库）已支持带显示名称的地址格式，前端只需正确传递格式化字符串即可。

### 数据兼容性

- 草稿数据：现有草稿存储的是 `string[]`（纯邮箱数组），需要兼容处理
- 联系人数据：联系人服务可能需要适配新的数据结构
