## Context

AetherMail 是一个基于 Tauri v2 + Vue 3 的桌面邮件客户端。当前实现了完整的 UI 框架，但邮件数据使用 Mock 数据，无法实际连接 POP3 服务器获取真实邮件。

### 当前架构

```
前端 (Vue 3)
├── stores/account.ts ──────► localStorage (账户 Mock 数据)
├── stores/mailbox.ts ──────► 内存 (邮件 Mock 数据)
└── services/mock/ ─────────► 硬编码测试数据

后端 (Rust/Tauri)
└── 仅托盘功能，无邮件处理逻辑
```

### 约束

- 使用成熟 Rust crate，不自研协议实现
- 当前迭代密码明文存储，后续升级为 keyring
- POP3 协议限制：仅支持收件箱，无文件夹管理
- 需要支持增量同步，避免重复下载

## Goals / Non-Goals

**Goals:**

- 实现 POP3 服务器连接和认证
- 实现邮件下载和本地存储
- 实现增量同步机制
- 前端集成真实邮件数据
- 账户配置持久化（含密码）

**Non-Goals:**

- 不实现 SMTP 发送邮件
- 不实现 IMAP 协议
- 不实现密码加密存储（后续迭代）
- 不实现邮件服务器端管理
- 不实现多文件夹支持（POP3 限制）

## Decisions

### 1. POP3 客户端库选择

**Decision:** 使用 `async-pop3` crate

**Rationale:**
- 异步实现，与 Tokio 生态兼容
- 支持 TLS/SSL 连接
- 活跃维护，文档完善
- API 设计简洁

**Alternatives considered:**
- `pop3-rs` - 同步实现，不适用于异步 Tauri
- 手写 POP3 协议 - 不必要，增加维护负担

### 2. 邮件解析库选择

**Decision:** 使用 `mail-parser` crate

**Rationale:**
- 纯 Rust 实现，无外部依赖
- 支持 MIME 解析
- 提取正文、附件、头部信息
- velo 项目验证可行

**Alternatives considered:**
- `lettre` - 主要用于发送，解析功能较弱

### 3. 本地存储方案

**Decision:** 使用 SQLite (通过 tauri-plugin-sql)

**Rationale:**
- Tauri 官方插件，集成简单
- 项目已配置 tauri-plugin-sql
- 支持复杂查询、事务
- 单文件数据库，便于备份

**Alternatives considered:**
- 文件系统 JSON - 查询效率低，无事务
- sled/lmdb - 过于底层，需要手动管理

### 4. 增量同步策略

**Decision:** 使用 POP3 UIDL 命令

**Rationale:**
- UIDL 返回邮件唯一标识符
- 对比本地已存储的 UIDL 列表
- 仅下载新邮件，节省带宽

**同步流程:**
```
1. 连接服务器
2. 执行 UIDL 获取所有邮件 ID
3. 对比本地已知 UIDL
4. 对新邮件执行 RETR 下载
5. 解析并存储到 SQLite
6. 更新本地 UIDL 列表
```

### 5. 密码存储方案

**Decision:** 明文 JSON 配置文件

**文件位置:** `~/.aethermail/accounts.json`

**结构:**
```json
{
  "accounts": [
    {
      "id": "account-123",
      "email": "user@example.com",
      "password": "plaintext-password",
      "host": "pop.example.com",
      "port": 995,
      "use_ssl": true
    }
  ]
}
```

**后续升级路径:**
- 引入 `keyring` crate
- 密码存储到系统密钥库
- 配置文件只存储非敏感信息

### 6. 前后端通信架构

**Decision:** Tauri Commands

```
┌──────────────────────────────────────────────────────┐
│                     前端                              │
├──────────────────────────────────────────────────────┤
│  invoke('sync_emails', { accountId })                │
│  invoke('get_emails', { accountId, folder })         │
│  invoke('test_connection', { config })               │
│  invoke('add_account', { account })                  │
└─────────────────────┬────────────────────────────────┘
                      │
┌─────────────────────▼────────────────────────────────┐
│                   Tauri Backend                       │
├──────────────────────────────────────────────────────┤
│  commands/account.rs - 账户管理命令                   │
│  commands/email.rs - 邮件操作命令                     │
│  pop3/client.rs - POP3 连接实现                       │
│  storage/database.rs - SQLite 操作                    │
└──────────────────────────────────────────────────────┘
```

## Risks / Trade-offs

### Risk 1: 明文密码安全风险

**Risk:** 配置文件中密码明文存储，存在泄露风险

**Mitigation:**
- 配置文件权限限制为用户可读
- 文档中明确提示风险
- 后续迭代优先升级为 keyring

### Risk 2: POP3 协议限制

**Risk:** POP3 不支持文件夹、已读状态同步

**Mitigation:**
- 已读状态仅在本地管理
- 文档说明 POP3 限制
- 未来考虑支持 IMAP

### Risk 3: 大量邮件性能

**Risk:** 首次同步大量邮件可能耗时较长

**Mitigation:**
- 后台异步同步
- 前端显示进度
- 分批下载，避免内存溢出

### Trade-off: 同步策略

**Trade-off:** UIDL 增量同步 vs 全量同步

**Reason:** 
- 全量同步简单但带宽消耗大
- UIDL 增量需要额外存储，但效率高

**Resolution:** 采用 UIDL 增量同步

## Migration Plan

### 阶段 1: 后端基础 (本次迭代)

1. 添加 Rust 依赖
2. 实现 POP3 客户端
3. 实现 SQLite 存储
4. 实现 Tauri Commands
5. 实现账户配置文件

### 阶段 2: 前端集成 (本次迭代)

1. 创建新服务层
2. 替换 Mock 调用
3. 更新 Store
4. 测试完整流程

### 阶段 3: 优化 (后续迭代)

1. 密码安全存储
2. 后台同步调度
3. 离线支持
4. 错误恢复机制