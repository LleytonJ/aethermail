## 1. 项目依赖配置

- [x] 1.1 在 `Cargo.toml` 添加 `async-imap` 依赖
- [x] 1.2 验证 async-imap 编译通过
- [x] 1.3 查阅 async-imap 文档和示例

## 2. 后端模块结构

- [x] 2.1 创建 `src-tauri/src/imap/mod.rs` 模块入口
- [x] 2.2 创建 `src-tauri/src/imap/types.rs` 定义数据类型
- [x] 2.3 创建 `src-tauri/src/imap/client.rs` IMAP 客户端实现

## 3. IMAP 连接实现

- [x] 3.1 实现 `ImapConfig` 和 `ImapCredentials` 结构体
- [x] 3.2 实现 `ImapClient::new()` 构造函数
- [x] 3.3 实现 `ImapClient::test_connection()` 测试连接
- [x] 3.4 实现 `ImapClient::connect()` 建立 TLS 连接并认证
- [x] 3.5 实现 `ImapSession` 结构体封装 async-imap Client

## 4. IMAP 邮件操作

- [x] 4.1 实现 `ImapSession::select_inbox()` 选择 INBOX
- [x] 4.2 实现 `ImapSession::get_uid_list()` 获取 UID 列表
- [x] 4.3 实现 `ImapSession::fetch_email()` 下载单封邮件
- [x] 4.4 实现 `ImapSession::logout()` 断开连接
- [x] 4.5 验证邮件解析使用现有 mail-parser

## 5. 数据库扩展

- [x] 5.1 修改 `models.rs` 扩展 DbAccount 模型
  - 新增 `protocol_type: String`
  - 新增 `imap_host: String`
  - 新增 `imap_port: u16`
  - 新增 `imap_use_ssl: bool`
- [x] 5.2 修改 `database.rs` 实现数据库迁移
  - accounts 表添加 protocol_type 字段（默认 'pop3'）
  - accounts 表添加 imap_host 字段
  - accounts 表添加 imap_port 字段（默认 993）
  - accounts 表添加 imap_use_ssl 字段（默认 1）
- [x] 5.3 修改账户配置文件加载/保存逻辑（accounts.rs）
- [x] 5.4 实现数据库兼容现有 POP3 账户

## 6. Tauri Commands 扩展

- [x] 6.1 修改 `commands/account.rs` 支持 IMAP 配置
  - 扩展 `add_account` 接收协议类型参数
  - 扩展 `add_account` 接收 IMAP 配置参数
- [x] 6.2 新增 `imap_test_connection` 命令
- [x] 6.3 修改 `commands/email.rs` 同步逻辑分支
  - 扩展 `sync_emails` 根据协议类型分支
  - 新增 `sync_via_imap` 内部函数
- [x] 6.4 在 `lib.rs` 注册 IMAP 相关命令

## 7. 前端服务层

- [x] 7.1 创建 `src/services/email/serverPresets.ts` 服务器预设
  - 定义 Gmail、163、QQ、Outlook 预设配置
  - 实现 `autoFillServer()` 函数
- [x] 7.2 修改 `src/services/email/accountService.ts` 支持协议参数
- [x] 7.3 扩展 `testConnection` 支持 IMAP 测试

## 8. 前端 Store 扩展

- [x] 8.1 修改 `src/stores/account.ts` Account 类型
  - 新增 `protocol: 'pop3' | 'imap'`
  - 新增 `imapHost`, `imapPort`, `imapUseSSL` 字段
- [x] 8.2 修改账户添加逻辑支持协议参数

## 9. 前端 UI 实现

- [x] 9.1 修改 `AddAccount.vue` 添加协议选择
  - Radio 切换 POP3/IMAP
  - 默认选中 POP3
- [x] 9.2 修改 `AddAccount.vue` 动态显示配置字段
  - POP3 时显示 POP3 服务器配置
  - IMAP 时显示 IMAP 服务器配置
- [x] 9.3 实现邮箱地址输入自动预填充服务器
  - 监听邮箱输入变化
  - 根据域名查询 serverPresets
  - 自动填充对应协议的服务器配置
- [x] 9.4 修改 `AddAccount.vue` 添加账户逻辑
  - 提交时包含协议类型
  - 提交时包含 IMAP 配置（如选择 IMAP）
- [x] 9.5 修改 `AccountList.vue` 显示协议类型标签
  - POP3 账户显示 "POP3" 标签
  - IMAP 账户显示 "IMAP" 标签

## 10. 测试和验证

- [ ] 10.1 测试 IMAP 连接
  - Gmail IMAP (imap.gmail.com:993)
  - 163 IMAP (imap.163.com:993)
  - QQ IMAP (imap.qq.com:993)
- [ ] 10.2 测试 IMAP 邮件同步
  - 首次同步（下载所有邮件）
  - 增量同步（仅下载新邮件）
- [ ] 10.3 测试 POP3 账户兼容性
  - 确认 POP3 账户功能正常
  - 确认数据库迁移不影响现有账户
- [ ] 10.4 测试协议切换 UI
  - Radio 切换正常
  - 配置字段正确显示/隐藏
  - 预填充功能正常
- [ ] 10.5 测试同一邮箱分别用 POP3 和 IMAP 添加

## 11. 文档更新

- [ ] 11.1 更新 README 说明支持 IMAP
- [ ] 11.2 更新 design.md（如需）

---

## 任务依赖关系

```
Phase 1 (Rust 客户端)
├── 1.x (依赖配置)
│   └── 2.x (模块结构)
│       └── 3.x (连接实现)
│           └── 4.x (邮件操作)
│
Phase 2 (数据库 + 命令)
├── 5.x (数据库扩展) ─────┬── 6.x (Tauri 命令)
│                        │
Phase 3 (前端)           │
├── 7.x (服务层) ────────┤
├── 8.x (Store) ─────────┤
├── 9.x (UI) ────────────┘
│
Phase 4 (测试)
└── 10.x (测试验证)
```

## 工时估算

| 阶段 | 预计工时 |
|-----|---------|
| 1. 依赖配置 | 0.5 小时 |
| 2. 模块结构 | 0.5 小时 |
| 3. 连接实现 | 2 小时 |
| 4. 邮件操作 | 3 小时 |
| 5. 数据库扩展 | 2.5 小时 |
| 6. Tauri Commands | 2 小时 |
| 7. 服务层 | 1 小时 |
| 8. Store 扩展 | 0.5 小时 |
| 9. UI 实现 | 2.5 小时 |
| 10. 测试验证 | 3 小时 |
| **总计** | **16 小时** |

考虑缓冲和调试时间，预计 **4-5 天** 完成全部任务。