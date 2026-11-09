## Context

AetherMail 目前仅支持邮件接收功能（POP3 和 IMAP），缺少邮件发送能力。完整的邮件客户端需要支持 SMTP 协议发送邮件。

### 当前架构
- 后端：Rust + Tauri，使用 async-imap 和 pop3-rs 接收邮件
- 前端：Vue 3 + Pinia，已实现账户管理和邮件列表
- 数据库：SQLite 存储账户和邮件数据

### SMTP 协议特点
- SMTP 是邮件发送标准协议
- 默认端口：25（无加密）、465（SSL）、587（TLS/STARTTLS）
- 认证方式：PLAIN、LOGIN 等
- 与 POP3/IMAP 独立，需要单独配置

## Goals / Non-Goals

**Goals:**
- 实现 SMTP 连接和认证
- 支持邮件发送、回复、转发
- 在添加账户时配置 SMTP 服务器
- 支持主流邮箱服务商 SMTP 预设
- SMTP 连接测试功能

**Non-Goals:**
- 不实现 OAuth2 认证（后续迭代）
- 不实现邮件附件发送（后续迭代）
- 不实现 HTML 邮件编辑器（先支持纯文本）
- 不实现已发送邮件同步到服务器

## Decisions

### 1. SMTP 库选择

**选择：使用 `lettre` crate**

理由：
- lettre 是 Rust 最成熟的 SMTP 库
- 支持 SSL/TLS 加密
- 支持多种认证方式
- 异步支持（tokio）
- 活跃维护，文档完善

备选方案：
- `async-smtp`：较新，文档较少
- 手动实现 SMTP 协议：过于复杂

### 2. SMTP 配置存储

**选择：扩展现有 Account 模型**

在 Account 模型新增字段：
- `smtp_host: String`
- `smtp_port: u16`（默认 465）
- `smtp_use_ssl: bool`（默认 true）

理由：
- 与现有 POP3/IMAP 配置保持一致
- 数据库迁移简单
- 前端 UI 统一

### 3. 添加账户流程

**选择：在收件服务器配置后增加 SMTP 配置步骤**

流程变更：
- 原流程：基本信息 → 收件服务器 → 测试连接 → 完成
- 新流程：基本信息 → 收件服务器 → 发件服务器 → 测试连接 → 完成

理由：
- SMTP 和收件服务器是独立配置
- 用户可能使用不同服务商的收件和发件服务器
- 分步配置减少认知负担

### 4. SMTP 预设策略

**选择：与收件服务器预设合并**

当用户输入邮箱地址时，自动填充：
- POP3/IMAP 服务器配置
- SMTP 服务器配置

理由：
- 大多数邮箱服务商的收件和发件服务器来自同一服务商
- 减少用户手动输入
- Gmail、163、QQ、Outlook 等主流服务商均有公开 SMTP 配置

## Risks / Trade-offs

### 风险：SMTP 服务器认证失败
- 部分邮箱服务商需要应用专用密码（如 Gmail）
- 缓解：提示用户启用 SMTP 服务或使用应用专用密码

### 风险：邮件发送失败处理
- 网络、认证、服务器拒绝等错误
- 缓解：详细错误信息提示，支持重试

### 风险：数据库迁移兼容性
- 现有账户无 SMTP 配置
- 缓解：SMTP 字段设置默认值，用户可后续编辑添加

## Migration Plan

### 数据库迁移
```sql
ALTER TABLE accounts ADD COLUMN smtp_host TEXT DEFAULT '';
ALTER TABLE accounts ADD COLUMN smtp_port INTEGER DEFAULT 465;
ALTER TABLE accounts ADD COLUMN smtp_use_ssl INTEGER DEFAULT 1;
```

### 回滚策略
- 删除新增字段
- 前端代码回退到不显示 SMTP 配置