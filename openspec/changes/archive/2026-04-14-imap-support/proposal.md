## Why

当前应用仅支持 POP3 协议收件，POP3 协议有以下限制：

1. **仅支持收件箱** - 无法访问其他文件夹（已发送、草稿、垃圾邮件等）
2. **单向同步** - 本地已读状态无法同步到服务器
3. **下载即删除** - 许多 POP3 配置会从服务器删除已下载邮件
4. **无法增量更新** - 只能通过 UIDL 判断新邮件，无法获取已读状态变化

IMAP 协议提供双向同步能力，是现代邮箱客户端的标准选择。支持 IMAP 收件可以让用户：
- 保持多设备邮件状态一致
- 不丢失服务器上的邮件
- 为后续实现文件夹管理、双向同步奠定基础

## What Changes

- **新增** Rust IMAP 客户端模块（使用 `async-imap` crate）
- **新增** 账户协议类型字段，支持 POP3 和 IMAP 切换
- **新增** 常见邮箱服务器预设配置（Gmail、163、QQ 等）
- **修改** 账户添加 UI，提供协议选择和动态配置字段
- **修改** 邮件同步逻辑，根据协议类型选择 POP3 或 IMAP 同步
- **扩展** 数据库 accounts 表，新增 IMAP 配置字段

## Capabilities

### New Capabilities

- `imap-connection`: IMAP 服务器连接能力 - 建立 TLS 连接，认证用户身份
- `imap-email-sync`: IMAP 邮件同步能力 - 使用 UID 增量同步 INBOX 邮件

### Modified Capabilities

- `account-persistence`: 扩展账户模型，支持协议类型和 IMAP 配置
- `email-sync`: 扩展同步逻辑，根据协议类型选择 POP3 或 IMAP

## Impact

### 后端影响 (Rust/Tauri)

| 文件/目录 | 变更类型 |
|----------|---------|
| `src-tauri/src/imap/mod.rs` | 新增 |
| `src-tauri/src/imap/types.rs` | 新增 |
| `src-tauri/src/imap/client.rs` | 新增 |
| `src-tauri/src/storage/models.rs` | 修改 (新增 IMAP 字段) |
| `src-tauri/src/storage/database.rs` | 修改 (数据库迁移) |
| `src-tauri/src/commands/account.rs` | 修改 (支持 IMAP 配置) |
| `src-tauri/src/commands/email.rs` | 修改 (同步协议分支) |
| `src-tauri/src/lib.rs` | 修改 (注册 IMAP 命令) |
| `src-tauri/Cargo.toml` | 修改 (添加 async-imap) |

### 前端影响 (Vue/TypeScript)

| 文件/目录 | 变更类型 |
|----------|---------|
| `src/components/accounts/AddAccount.vue` | 修改 (协议切换 UI) |
| `src/components/accounts/AccountList.vue` | 修改 (显示协议类型) |
| `src/stores/account.ts` | 修改 (新增 protocol 字段) |
| `src/services/email/serverPresets.ts` | 新增 (服务器预设) |

### 依赖影响

**新增 Rust 依赖：**
- `async-imap` - 异步 IMAP 客户端库

### 数据库影响

**accounts 表新增字段：**
- `protocol_type` - 协议类型 (pop3/imap)
- `imap_host` - IMAP 服务器地址
- `imap_port` - IMAP 端口
- `imap_use_ssl` - IMAP SSL 开关

## Scope

### In Scope

- IMAP 连接和认证 (LOGIN 命令)
- IMAP 选择 INBOX 文件夹 (SELECT 命令)
- IMAP 获取邮件 UID 列表 (SEARCH UID * 命令)
- IMAP 获取邮件内容 (UID FETCH 命令)
- IMAP 增量同步 (基于 UID)
- 账户添加时的协议选择 (POP3/IMAP Radio)
- 根据邮箱地址预填充服务器配置
- POP3 和 IMAP 账户共存

### Out of Scope

- IMAP 文件夹管理（LIST、CREATE、DELETE 文件夹）
- IMAP 双向同步（本地状态同步到服务器）
- IMAP 邮件移动/复制
- IMAP 邮件删除
- SMTP 发件（单独提案）
- 回复/转发功能（单独提案）

## Timeline Estimate

| 阶段 | 工时 | 天数 |
|-----|------|------|
| Rust IMAP 客户端 | 8 小时 | 1.3 天 |
| 数据库扩展 | 2.5 小时 | 0.5 天 |
| Tauri 命令层 | 4 小时 | 0.7 天 |
| 前端 UI | 4 小时 | 0.7 天 |
| 测试 | 3 小时 | 0.5 天 |
| **总计** | **21.5 小时** | **4-5 天** |

## Success Criteria

1. 可以添加 IMAP 类型账户
2. IMAP 账户能成功连接 Gmail/163/QQ 邮箱
3. IMAP 账户能同步 INBOX 邅件到本地
4. IMAP 增量同步正常工作（基于 UID）
5. POP3 账户功能不受影响
6. 同一邮箱可分别用 POP3 和 IMAP 添加
7. 协议选择 UI 正常切换
8. 邮箱地址能预填充正确的服务器配置