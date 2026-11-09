## Why

当前 aethermail 项目的邮件撰写功能使用简单的 textarea，仅支持纯文本编辑，无法满足用户对专业邮件沟通的需求（如格式化文本、图片签名、链接等）。竞品 velo 项目已基于 TipTap 实现了完整的富文本签名功能，验证了该方案的可行性。引入富文本编辑器和签名功能将显著提升产品的专业性和用户体验。

## What Changes

- **撰写编辑器改造**：将 ComposeEmail.vue 中的 textarea 替换为 TipTap 富文本编辑器，支持加粗、斜体、列表、链接、图片等格式
- **签名功能**：在账户设置下新增签名编辑区域，每个账户可设置一个富文本签名
- **签名开关**：撰写邮件时可通过开关控制是否附加签名（默认开启）
- **签名存储**：新增数据库字段存储账户的签名 HTML 内容
- **自动附加签名**：发送邮件时自动将签名 HTML 附加到正文底部

## Capabilities

### New Capabilities

- `rich-text-editor`: TipTap 富文本编辑器集成，用于邮件正文和签名编辑
- `account-signature`: 账户签名设置功能（每个账户一个签名，位于账户设置下）

### Modified Capabilities

- `account-editing`: 账户编辑页面新增签名编辑区域
- `email-sending`: 发送邮件时需支持 HTML 正文格式，并在正文末尾附加签名 HTML（可开关控制）

## Impact

- **新增组件**：
  - `RichTextEditor.vue` - 可复用的 TipTap 编辑器组件
- **修改组件**：
  - `ComposeEmail.vue` - 集成富文本编辑器和签名开关
  - `EditAccountForm.vue` - 新增签名编辑区域
- **新增服务**：
  - `signatureService.ts` - 签名数据读写操作
- **数据库变更**：
  - `accounts` 表新增 `signature_html` 字段（TEXT类型，存储签名HTML）
- **新增依赖**：
  - `@tiptap/vue-3`
  - `@tiptap/starter-kit`
  - `@tiptap/extension-image`
  - `@tiptap/extension-placeholder`
- **状态管理**：
  - `composeStore.ts` 新增 includeSignature 开关状态