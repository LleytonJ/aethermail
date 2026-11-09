## 1. 项目依赖配置

- [x] 1.1 在 `Cargo.toml` 添加 `lettre` 依赖
- [x] 1.2 验证 lettre 编译通过
- [x] 1.3 查阅 lettre 文档和示例

## 2. 后端模块结构

- [x] 2.1 创建 `src-tauri/src/smtp/mod.rs` 模块入口
- [x] 2.2 创建 `src-tauri/src/smtp/types.rs` 定义数据类型
- [x] 2.3 创建 `src-tauri/src/smtp/client.rs` SMTP 客户端实现

## 3. SMTP 连接实现

- [x] 3.1 实现 `SmtpConfig` 结构体（host, port, use_ssl）
- [x] 3.2 实现 `SmtpClient::new()` 构造函数
- [x] 3.3 实现 `SmtpClient::test_connection()` 测试连接
- [x] 3.4 实现 `SmtpClient::send_email()` 发送邮件

## 4. 数据库扩展

- [x] 4.1 修改 AccountConfig 支持 SMTP 字段（账户存储在 JSON 文件，非数据库）
- [x] 4.2 AccountConfig 默认值兼容现有账户
- [x] 4.3 实现 AccountConfig 序列化兼容

## 5. Tauri Commands 扩展

- [x] 5.1 修改 `commands/account.rs` 支持 SMTP 配置
- [x] 5.2 新增 `test_smtp_connection` 命令
- [x] 5.3 新增 `send_email` 命令
- [x] 5.4 在 `lib.rs` 注册 SMTP 相关命令

## 6. 前端服务层

- [x] 6.1 扩展 `providerPresets.ts` 添加 SMTP 预设
  - Gmail SMTP (smtp.gmail.com:465)
  - 163 SMTP (smtp.163.com:25)
  - QQ SMTP (smtp.qq.com:465)
  - Outlook SMTP (smtp-mail.outlook.com:587)
- [x] 6.2 扩展 `accountService.ts` 支持 SMTP 配置参数
- [x] 6.3 新增 `testSmtpConnection` 函数
- [x] 6.4 新增 `sendEmail` 函数

## 7. 前端 Store 扩展

- [x] 7.1 修改 `stores/account.ts` Account 类型
  - 新增 `smtpHost: string`
  - 新增 `smtpPort: number`
  - 新增 `smtpUseSsl: boolean`
- [x] 7.2 修改账户添加逻辑支持 SMTP 参数

## 8. 前端 UI - 添加账户流程

- [x] 8.1 修改 `AddPop3Account.vue` 增加 SMTP 配置步骤
  - 步骤指示器新增 "发件" 步骤
  - SMTP 服务器配置表单
  - 自动填充 SMTP 预设
- [x] 8.2 修改 `AddImapAccount.vue` 增加 SMTP 配置步骤
  - 步骤指示器新增 "发件" 步骤
  - SMTP 服务器配置表单
  - 自动填充 SMTP 预设
- [x] 8.3 修改测试连接步骤测试 SMTP 连接
  - 同时测试收件和发件服务器
  - 显示各自测试结果
- [x] 8.4 修改账户保存逻辑包含 SMTP 配置

## 9. 前端 UI - 邮件撰写

- [x] 9.1 创建 `ComposeEmail.vue` 邮件撰写组件
  - 收件人输入框（支持验证）
  - 主题输入框
  - 正文编辑区
  - 发送按钮
- [x] 9.2 创建 `stores/compose.ts` 撰写邮件状态管理
  - 管理当前撰写邮件
  - 发送状态跟踪
  - 错误处理
- [x] 9.3 集成撰写组件到主布局
  - Sidebar 撰写按钮
  - 点击后打开撰写界面

## 10. 前端 UI - 回复转发

- [x] 10.1 修改 `ReadingPane.vue` 回复按钮功能
  - 点击回复打开撰写界面
  - 预填充收件人、主题、引用正文
- [x] 10.2 修改 `ReadingPane.vue` 转发按钮功能
  - 点击转发打开撰写界面
  - 预填充主题、转发正文
  - 收件人留空待填写

## 11. 测试和验证

- [ ] 11.1 测试 SMTP 连接
  - Gmail SMTP (smtp.gmail.com:465)
  - 163 SMTP (smtp.163.com:465)
  - QQ SMTP (smtp.qq.com:465)
- [ ] 11.2 测试邮件发送
  - 发送纯文本邮件
  - 发送带显示名称邮件
  - 回复邮件
  - 转发邮件
- [ ] 11.3 测试 SMTP 配置 UI
  - 预填充功能正常
  - 手动配置功能正常
  - 测试连接显示正确结果

---

## 任务依赖关系

```
Phase 1 (Rust 客户端)
├── 1.x (依赖配置)
│   └── 2.x (模块结构)
│       └── 3.x (连接实现)
│
Phase 2 (数据库 + 命令)
├── 4.x (数据库扩展) ─────┬── 5.x (Tauri 命令)
│                        │
Phase 3 (前端)           │
├── 6.x (服务层) ────────┤
├── 7.x (Store) ─────────┤
├── 8.x (添加账户 UI) ───┤
├── 9.x (撰写 UI) ───────┤
├── 10.x (回复转发) ─────┘
│
Phase 4 (测试)
└── 11.x (测试验证)
```

## 工时估算

| 阶段 | 预计工时 |
|-----|---------|
| 1. 依赖配置 | 0.5 小时 |
| 2. 模块结构 | 0.5 小时 |
| 3. SMTP 实现 | 2 小时 |
| 4. 数据库扩展 | 1.5 小时 |
| 5. Tauri Commands | 1.5 小时 |
| 6. 服务层 | 1 小时 |
| 7. Store 扩展 | 0.5 小时 |
| 8. 添加账户 UI | 2 小时 |
| 9. 撰写 UI | 2.5 小时 |
| 10. 回复转发 | 1 小时 |
| 11. 测试验证 | 2 小时 |
| **总计** | **13 小时** |

考虑缓冲和调试时间，预计 **3-4 天** 完成全部任务。