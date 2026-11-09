## 1. 项目依赖配置

- [x] 1.1 在 `Cargo.toml` 添加 POP3 相关依赖（tokio-native-tls, native-tls）
- [x] 1.2 在 `Cargo.toml` 添加 `mail-parser` 依赖
- [x] 1.3 在 `Cargo.toml` 添加 `serde` 和 `serde_json` 依赖（已有）
- [x] 1.4 验证依赖编译通过

## 2. 后端模块结构

- [x] 2.1 创建 `src-tauri/src/pop3/mod.rs` 模块入口
- [x] 2.2 创建 `src-tauri/src/pop3/types.rs` 定义数据类型
- [x] 2.3 创建 `src-tauri/src/pop3/client.rs` POP3 客户端实现
- [x] 2.4 创建 `src-tauri/src/storage/mod.rs` 存储模块入口
- [x] 2.5 创建 `src-tauri/src/storage/database.rs` SQLite 数据库操作
- [x] 2.6 创建 `src-tauri/src/storage/models.rs` 数据模型
- [x] 2.7 创建 `src-tauri/src/commands/mod.rs` 命令模块入口
- [x] 2.8 创建 `src-tauri/src/commands/account.rs` 账户命令
- [x] 2.9 创建 `src-tauri/src/commands/email.rs` 邮件命令

## 3. POP3 连接实现

- [x] 3.1 实现 `Pop3Client` 结构体和构造函数
- [x] 3.2 实现 `connect` 方法建立 TCP/TLS 连接
- [x] 3.3 实现 `authenticate` 方法进行用户认证
- [x] 3.4 实现 `disconnect` 方法断开连接
- [x] 3.5 实现 `test_connection` 命令测试连接

## 4. POP3 邮件操作

- [x] 4.1 实现 `get_uidl_list` 方法获取邮件 UIDL 列表
- [x] 4.2 实现 `fetch_email` 方法下载单封邮件
- [x] 4.3 实现 `fetch_all_emails` 方法下载所有邮件（通过 sync_emails）
- [x] 4.4 实现邮件解析逻辑（提取 subject, from, to, body 等）
- [x] 4.5 实现附件提取逻辑

## 5. SQLite 数据库实现

- [x] 5.1 定义数据库表结构（accounts, emails, attachments）
- [x] 5.2 实现 `init_database` 函数创建表
- [x] 5.3 实现 `save_email` 函数存储邮件
- [x] 5.4 实现 `get_emails` 函数获取邮件列表
- [x] 5.5 实现 `get_email_by_id` 函数获取单封邮件
- [x] 5.6 实现 `update_email_state` 函数更新已读/星标状态
- [x] 5.7 实现 `get_existing_uidls` 函数获取已存储的 UIDL
- [x] 5.8 实现 `delete_emails_by_account` 函数删除账户邮件

## 6. 账户配置文件实现

- [x] 6.1 定义 `AccountConfig` 结构体
- [x] 6.2 实现 `get_config_path` 获取配置文件路径
- [x] 6.3 实现 `load_accounts` 从文件加载账户
- [x] 6.4 实现 `save_accounts` 保存账户到文件
- [x] 6.5 实现 `add_account` 添加新账户
- [x] 6.6 实现 `update_account` 更新账户配置
- [x] 6.7 实现 `remove_account` 删除账户

## 7. Tauri Commands 实现

- [x] 7.1 实现 `test_pop3_connection` 命令
- [x] 7.2 实现 `add_pop3_account` 命令
- [x] 7.3 实现 `sync_emails` 命令
- [x] 7.4 实现 `get_emails` 命令
- [x] 7.5 实现 `get_email_detail` 命令
- [x] 7.6 实现 `mark_email_read` 命令
- [x] 7.7 实现 `toggle_email_star` 命令
- [x] 7.8 在 `lib.rs` 注册所有命令

## 8. 前端服务层

- [x] 8.1 创建 `src/services/email/emailService.ts` 邮件服务
- [x] 8.2 创建 `src/services/email/accountService.ts` 账户服务
- [x] 8.3 实现 `testConnection` 调用后端命令
- [x] 8.4 实现 `addAccount` 调用后端命令
- [x] 8.5 实现 `syncEmails` 调用后端命令
- [x] 8.6 实现 `getEmails` 调用后端命令
- [x] 8.7 实现 `markAsRead` 调用后端命令
- [x] 8.8 实现 `toggleStar` 调用后端命令

## 9. 前端 Store 集成

- [x] 9.1 修改 `mailbox.ts` 使用新的 emailService
- [x] 9.2 修改 `account.ts` 使用新的 accountService
- [x] 9.3 实现同步状态管理（loading, progress, error）
- [x] 9.4 实现首次打开自动同步

## 10. 添加账户流程集成

- [x] 10.1 修改 `AddPop3Account.vue` 使用真实连接测试
- [x] 10.2 实现账户添加成功后自动同步
- [x] 10.3 实现同步进度显示

## 11. 错误处理和用户反馈

- [x] 11.1 实现连接错误提示
- [x] 11.2 实现认证错误提示
- [x] 11.3 实现同步错误提示
- [x] 11.4 实现离线模式支持（读取本地数据）

## 12. 测试和验证

- [ ] 12.1 测试 POP3 连接（Gmail, 163, QQ 等常见邮箱）
- [ ] 12.2 测试邮件同步流程
- [ ] 12.3 测试增量同步（UIDL 对比）
- [ ] 12.4 测试邮件显示（HTML 正文、附件）
- [ ] 12.5 测试已读/星标状态持久化
- [ ] 12.6 测试账户删除时清理数据