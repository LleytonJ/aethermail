## Why

当前 AetherMail 邮件客户端在撰写邮件时不支持添加附件，用户无法在发送邮件时附带文件。这是一个基础邮件客户端功能的缺失，严重影响用户体验。同时，用户期望能够通过拖拽文件直接添加附件，这是一种更直观、高效的交互方式。

## What Changes

- 添加邮件撰写时的附件上传功能
- 支持拖拽文件到撰写区域添加附件
- 支持点击按钮选择文件添加附件
- 附件列表显示在撰写界面中，可删除已添加的附件
- SMTP 发送支持 multipart/mixed MIME 格式携带附件
- 草稿保存/编辑时保留附件信息

## Capabilities

### New Capabilities

- `compose-attachments`: 撰写邮件时的附件管理功能，包括拖拽添加、按钮添加、附件列表展示、删除附件

### Modified Capabilities

- `email-sending`: 修改 SMTP 发送逻辑以支持携带附件（从 multipart/alternative 改为 multipart/mixed）

## Impact

### 前端
- `src/components/email/ComposeEmail.vue` - 添加拖拽区域、附件按钮、附件列表
- `src/components/email/ComposeAttachmentList.vue` - 新组件，展示撰写时的附件列表
- `src/stores/compose.ts` - 添加附件状态管理
- `src/services/email/emailService.ts` - sendEmail() 增加 attachments 参数

### 后端 (Rust/Tauri)
- `src-tauri/src/commands/email.rs` - send_email/send_draft 增加附件处理
- `src-tauri/src/smtp/client.rs` - build_message() 支持 multipart/mixed + 附件
- `src-tauri/src/smtp/types.rs` - EmailMessage 结构增加 attachments 字段

### 数据库
- drafts 表需支持存储附件信息（可能需要新建 draft_attachments 表）