## Why

用户目前只能接收邮件，无法发送邮件。完整的邮件客户端需要具备发送邮件的能力，这是核心功能缺失。SMTP 是邮件发送的标准协议，所有主流邮箱服务商都支持。

## What Changes

- 新增 SMTP 发件服务器配置，与收件服务器（POP3/IMAP）配置并行
- 在添加账户流程中增加 SMTP 服务器配置步骤
- 实现邮件撰写界面，支持发送新邮件
- 实现邮件回复和转发功能
- 支持主流邮箱服务商的 SMTP 预设配置自动填充
- SMTP 连接支持 SSL/TLS 加密

## Capabilities

### New Capabilities

- `smtp-connection`: SMTP 服务器连接、认证、测试连接功能
- `email-sending`: 邮件撰写、发送、回复、转发功能

### Modified Capabilities

- `account-ui`: 添加账户流程需要增加 SMTP 服务器配置步骤
- `account-persistence`: Account 模型需要新增 SMTP 相关字段（smtp_host, smtp_port, smtp_use_ssl）

## Impact

### 后端影响
- `src-tauri/src/smtp/` - 新增 SMTP 客户端模块
- `src-tauri/src/commands/account.rs` - 扩展账户配置支持 SMTP
- `src-tauri/src/commands/email.rs` - 新增发送邮件命令
- `src-tauri/src/models.rs` - DbAccount 新增 SMTP 字段
- `src-tauri/src/database.rs` - 数据库迁移添加 SMTP 字段

### 前端影响
- `src/services/email/accountService.ts` - 支持 SMTP 配置参数
- `src/services/email/emailService.ts` - 新增发送邮件接口
- `src/services/email/providerPresets.ts` - 新增 SMTP 预设配置
- `src/stores/account.ts` - Account 类型扩展
- `src/components/accounts/AddAccount.vue` - 增加 SMTP 配置 UI
- `src/components/accounts/AddPop3Account.vue` - 增加 SMTP 配置
- `src/components/accounts/AddImapAccount.vue` - 增加 SMTP 配置
- 新增邮件撰写组件

### 数据库迁移
- accounts 表新增 smtp_host、smtp_port、smtp_use_ssl 字段