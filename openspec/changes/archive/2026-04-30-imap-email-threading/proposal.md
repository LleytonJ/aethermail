## Why

当前 aethermail 的邮件列表是扁平展示，同一主题的往来邮件（如回复链、讨论对话）散落在列表中，用户难以追踪完整对话脉络。

邮件的 RFC 标准头（Message-ID、In-Reply-To、References）天然建立了邮件之间的父子关系。通过 JWZ threading 算法可以自动将这些关联邮件合并为"会话"视图，显著提升邮件浏览体验。

**只针对 IMAP 协议的邮件实现**，因为 IMAP 协议能可靠获取这些 threading 头，POP3 协议通常不提供完整的邮件头信息。

## What Changes

- **IMAP 同步流程**：提取并存储 Message-ID、In-Reply-To、References 邮件头
- **数据库结构**：新增 `threads` 表，`emails` 表新增 `thread_id` 字段
- **前端计算**：引入 JWZ threading 算法，将邮件分组成会话
- **UI 视图**：邮件列表改为"会话卡片"展示，点击展开会话内所有邮件

## Capabilities

### New Capabilities

- `email-threading`: IMAP 邮件会话合并能力，包括 threading 头解析、会话计算、会话存储和展示

### Modified Capabilities

- `imap-email-sync`: 新增需求：同步时提取并存储 Message-ID、In-Reply-To、References 邮件头
- `email-storage`: 新增需求：支持 threads 表和 emails.thread_id 字段

## Impact

### Rust 后端 (src-tauri)

- `imap/types.rs`: ImapMessage 结构体新增 3 个字段
- `imap/client.rs`: fetch_email 方法需解析 threading 头
- `storage/models.rs`: 新增 DbThread 结构体，DbEmail 新增 thread_id
- `storage/database.rs`: 新增 threads 表迁移，新增线程相关方法
- `commands/email.rs`: 同步流程需传递 threading 头

### TypeScript 前端 (src)

- 新增 `services/threading/threadBuilder.ts`: JWZ 算法实现（可复用 velo 项目代码）
- 新增 `services/email/threadService.ts`: 会话相关 API 调用
- `stores/mailbox.ts`: 新增 threads 状态和操作方法
- 新增 `components/email/ThreadListItem.vue`: 会话卡片组件
- 新增 `components/email/ThreadView.vue`: 会话详情组件