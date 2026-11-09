## Why

当前邮件详情页面的删除按钮仅为前端展示，没有绑定实际功能。用户无法删除单封邮件，只能通过删除整个账户来清除邮件。这是一个基本且必要的邮件管理功能缺失。

## What Changes

- 后端新增单封邮件删除功能（数据库方法 + Tauri 命令）
- 前端添加删除邮件的服务函数和 store action
- 将删除按钮绑定实际功能，点击后从数据库移除邮件及其附件
- 删除后自动更新邮件列表并清空选中状态

## Capabilities

### New Capabilities
- `email-deletion`: 邮件删除功能，包括数据库删除方法、后端命令和前端调用

### Modified Capabilities
- `email-storage`: 新增单封邮件删除方法（已有批量删除，需扩展）
- `mailbox`: 新增 deleteEmail action 和相关状态管理

## Impact

- 后端：`src-tauri/src/storage/database.rs` - 新增 `delete_email` 方法
- 后端：`src-tauri/src/commands/email.rs` - 新增 `delete_email` 命令
- 前端：`src/services/email/emailService.ts` - 新增 `deleteEmail` 函数
- 前端：`src/stores/mailbox.ts` - 新增 `deleteEmail` action
- 前端：`src/components/layout/ReadingPane.vue` - 绑定删除按钮点击事件