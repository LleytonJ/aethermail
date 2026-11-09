## Why

当前钓鱼检测在前端执行，导致两个严重问题：

1. **用户体验问题**：邮件同步后先显示在收件箱，几秒后钓鱼检测完成才被移除，用户已经看到了钓鱼邮件
2. **重复检测问题**：用户手动将钓鱼邮件移回收件箱后，下次同步又被检测为钓鱼邮件再次移走，无法保留用户决策

根本原因是检测和保存不是原子操作，前端和后端分离导致数据一致性难以保证。

## What Changes

- 将钓鱼邮件检测从前端移到 Rust 后端执行
- 使用 `async-openai` crate 调用兼容 OpenAI 协议的 AI API
- 邮件在保存数据库前完成钓鱼检测，根据检测结果直接保存到正确文件夹
- 新增 `user_overridden` 字段标记用户手动处理的邮件，避免重复检测
- 移除前端 `aiService.ts` 的 API 调用逻辑，仅保留配置管理

## Capabilities

### New Capabilities

- `phishing-detection-backend`: 后端钓鱼邮件检测能力，在邮件同步流程中执行 AI 分析，原子性地完成检测和保存

### Modified Capabilities

- `email-sync`: 同步流程需在保存前执行钓鱼检测，返回检测结果信息
- `phishing-management`: 新增 `user_overridden` 字段支持用户决策持久化

## Impact

### 后端改动

- **新增依赖**：`async-openai = "0.27"` (Cargo.toml)
- **新增模块**：`src-tauri/src/ai/client.rs` - AI 客户端封装
- **重构**：`src-tauri/src/commands/email.rs` - `sync_emails` 函数重构
- **数据库**：新增 `user_overridden` 字段 (emails 表)

### 前端改动

- **简化**：`src/services/ai/aiService.ts` - 移除 OpenAI SDK 调用，保留配置读取
- **简化**：`src/services/phishing/phishingDetectionService.ts` - 移除检测逻辑
- **简化**：`src/stores/mailbox.ts` - 移除前端钓鱼检测流程

### API Key 安全

- API Key 从浏览器环境移到后端处理，安全性提升