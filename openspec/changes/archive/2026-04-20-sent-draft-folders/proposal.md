## Why

用户目前发送的邮件没有在应用中保存记录，发送后无法查看已发送的邮件列表。同时用户在撰写邮件时，无法保存草稿，导致无法中途离开再继续编辑。这是邮件客户端的核心功能缺失。

本期目标：仅实现本地邮箱功能，已发送和草稿只保存在本地数据库，不同步到远程服务器。

## What Changes

- 数据模型增加 `folder` 字段，区分收件（inbox）和发件（sent/drafts）
- 数据模型增加 `draft_id` 字段，用于草稿更新和删除
- 发送邮件后保存到本地数据库（folder='sent'）
- 实现草稿创建、更新、删除功能
- 实现已发送和草稿列表查看功能
- 前端增加已发送和草稿箱入口

## Capabilities

### New Capabilities

- `sent-folder`: 已发送邮件列表查看、计数（仅本地）
- `draft-folder`: 草稿创建、更新、删除、列表查看（仅本地）

### Modified Capabilities

- `email-storage`: 数据模型增加 folder 和 draft_id 字段
- `email-sync`: 同步邮件时指定 folder='inbox'
- `email-sending`: 发送成功后保存到本地数据库

## Impact

### 后端影响
- `src-tauri/src/storage/models.rs` - DbEmail 新增 folder、draft_id 字段
- `src-tauri/src/storage/database.rs` - 迁移脚本、新增查询方法
- `src-tauri/src/commands/email.rs` - 新增命令、修改 send_email
- `src-tauri/src/lib.rs` - 注册新命令

### 前端影响
- `src/stores/mailbox.ts` - 新增 sent/drafts 状态
- `src/stores/compose.ts` - 新增 draftId、isSaving 状态
- `src/router/index.ts` - 新增 /sent、/drafts 路由
- `src/components/sidebar/` - 新增已发送、草稿箱入口
- 新增已发送列表组件
- 新增草稿列表组件

### 数据库迁移
- emails 表新增 folder 字段（默认 'inbox'）
- emails 表新增 draft_id 字段
- 存量钓鱼邮件迁移 folder='phishing'
- 创建 folder 索引

## POP3 和 IMAP 统一处理

本期 POP3 和 IMAP 账户统一处理，已发送和草稿都只保存在本地数据库：

| 功能 | POP3 账户 | IMAP 账户 |
|------|-----------|-----------|
| 发送邮件 | SMTP 发送 | SMTP 发送 |
| 本地保存已发送 | ✅ DB 保存 | ✅ DB 保存 |
| 服务器同步已发送 | ❌ 本期不做 | ❌ 本期不做 |
| 草稿创建 | ✅ 仅本地 DB | ✅ 仅本地 DB |
| 草稿更新 | ✅ 本地更新 | ✅ 本地更新 |
| 草稿删除 | ✅ 本地删除 | ✅ 本地删除 |

后续迭代可考虑为 IMAP 账户增加服务器同步功能。