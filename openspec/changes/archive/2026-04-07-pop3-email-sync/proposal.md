## Why

当前应用仅使用 Mock 数据模拟邮件显示，无法实际连接 POP3 服务器获取真实邮件。用户添加账户后，无法接收和查看真实的邮件内容。实现 POP3 邮件接收功能是应用核心功能之一，使应用能够实际使用。

## What Changes

- **新增** Tauri 后端 POP3 客户端模块（Rust 实现）
- **新增** SQLite 本地邮件存储（替代 Mock 数据）
- **新增** Tauri Commands 用于前后端通信
- **新增** 邮件同步服务，支持增量同步
- **修改** 前端邮件服务层，替换 Mock 调用为 Tauri Commands
- **修改** 账户配置存储，添加 POP3 连接参数持久化
- **新增** 密码配置文件存储（明文 JSON，后续迭代升级为安全存储）

## Capabilities

### New Capabilities

- `pop3-connection`: POP3 服务器连接能力 - 建立 TCP/TLS 连接，认证用户身份
- `email-sync`: 邮件同步能力 - 从 POP3 服务器获取邮件并存储到本地
- `email-storage`: 邮件本地存储能力 - 使用 SQLite 持久化邮件数据
- `account-persistence`: 账户持久化能力 - 将账户配置（含密码）存储到配置文件

### Modified Capabilities

（无现有 capability 需要修改，当前邮件相关功能均为 Mock 实现）

## Impact

### 后端影响 (Rust/Tauri)

| 文件/目录 | 变更类型 |
|----------|---------|
| `src-tauri/src/pop3/mod.rs` | 新增 |
| `src-tauri/src/pop3/client.rs` | 新增 |
| `src-tauri/src/pop3/types.rs` | 新增 |
| `src-tauri/src/storage/mod.rs` | 新增 |
| `src-tauri/src/storage/database.rs` | 新增 |
| `src-tauri/src/storage/models.rs` | 新增 |
| `src-tauri/src/commands/account.rs` | 新增 |
| `src-tauri/src/commands/email.rs` | 新增 |
| `src-tauri/src/commands/mod.rs` | 新增 |
| `src-tauri/src/lib.rs` | 修改 |
| `src-tauri/Cargo.toml` | 修改 |

### 前端影响 (Vue/TypeScript)

| 文件/目录 | 变更类型 |
|----------|---------|
| `src/services/email/emailService.ts` | 新增（替代 Mock） |
| `src/services/email/accountService.ts` | 新增（替代 Mock） |
| `src/services/mock/` | 废弃 |
| `src/stores/mailbox.ts` | 修改 |
| `src/stores/account.ts` | 修改 |

### 依赖影响

**新增 Rust 依赖：**
- `async-pop3` - 异步 POP3 客户端库
- `mail-parser` - 邮件解析库
- `rusqlite` (通过 tauri-plugin-sql) - SQLite 数据库

### 配置文件

- `~/.aethermail/accounts.json` - 账户配置文件（含明文密码）