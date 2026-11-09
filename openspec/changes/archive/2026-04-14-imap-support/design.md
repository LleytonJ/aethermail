## Context

AetherMail 当前实现了 POP3 协议收件，使用自研 POP3 客户端（约 450 行 Rust）。POP3 客户端直接使用 TCP/TLS socket，手动解析 POP3 响应。

### 当前 POP3 实现模式

```rust
// src-tauri/src/pop3/client.rs
pub struct Pop3Client {
    config: Pop3Config,
    credentials: Pop3Credentials,
}

impl Pop3Client {
    pub async fn test_connection(&self) -> Result<ConnectionTestResult, String>;
    pub async fn connect(&self) -> Result<Pop3Session, String>;
}

pub struct Pop3Session {
    reader: Pop3Reader,  // BufReader<TcpStream> 或 BufReader<TlsStream>
}

impl Pop3Session {
    pub async fn stat(&mut self) -> Result<(u32, u32), String>;
    pub async fn get_uidl_list(&mut self) -> Result<Vec<(u32, String)>, String>;
    pub async fn fetch_email(&mut self, message_number: u32) -> Result<RawEmail, String>;
    pub async fn disconnect(&mut self) -> Result<(), String>;
}
```

### 当前账户模型

```rust
// src-tauri/src/storage/models.rs
pub struct DbAccount {
    pub id: String,
    pub email: String,
    pub display_name: Option<String>,
    pub pop3_host: String,
    pub pop3_port: u16,
    pub use_ssl: bool,
    pub is_active: bool,
}
```

### 约束

- 使用成熟 Rust crate，不自研 IMAP 协议解析
- IMAP 响应格式比 POP3 复杂得多（括号、标记、嵌套结构）
- 精简版 IMAP 仅支持 INBOX，不支持文件夹管理
- 与 POP3 客户端并存，不替换 POP3
- 前端账户添加 UI 需要协议切换

## Goals / Non-Goals

**Goals:**

- 实现 IMAP 连接和认证
- 实现 IMAP INBOX 邮件获取
- 实现 IMAP UID 增量同步
- 账户添加支持协议选择
- POP3 和 IMAP 账户共存
- 预填充常见邮箱服务器配置

**Non-Goals:**

- 不实现文件夹管理（LIST、CREATE、DELETE）
- 不实现双向同步（STORE 命令）
- 不实现邮件移动/复制（COPY/MOVE 命令）
- 不实现邮件删除（EXPUNGE 命令）
- 不实现 CONDSTORE 高级增量同步

## Decisions

### 1. IMAP 客户端库选择

**Decision:** 使用 `async-imap` crate

**Rationale:**
- 异步实现，与 Tokio/Tauri 架构一致
- 支持 TLS/SSL 连接
- 响应解析已处理好（这是 IMAP 最大难点）
- 项目活跃，有示例代码可参考

**Alternatives considered:**
- `imap` crate (jonhoo/rust-imap) - 同步实现，需要额外包装为异步，架构不匹配
- 自研 IMAP 客户端 - IMAP 响应解析复杂，工作量增加约 150 行

**Cargo.toml 添加:**
```toml
async-imap = "0.9"
```

### 2. IMAP 客户端设计

**Decision:** 参考 POP3 模式，但简化结构

由于 async-imap 已经处理了响应解析，IMAP 客户端结构比 POP3 更简单：

```rust
// src-tauri/src/imap/types.rs
pub struct ImapConfig {
    pub host: String,
    pub port: u16,
    pub use_ssl: bool,
}

pub struct ImapCredentials {
    pub username: String,
    pub password: String,
}

pub struct ImapMessage {
    pub uid: u32,
    pub raw_content: String,
}

pub struct ImapSyncResult {
    pub total: u32,
    pub new: u32,
    pub success: bool,
    pub message: String,
}
```

```rust
// src-tauri/src/imap/client.rs
use async_imap::Client;

pub struct ImapClient {
    config: ImapConfig,
    credentials: ImapCredentials,
}

impl ImapClient {
    pub async fn test_connection(&self) -> Result<ConnectionTestResult, String>;
    pub async fn connect(&self) -> Result<ImapSession, String>;
}

pub struct ImapSession {
    client: Client<TlsStream<TcpStream>>,
}

impl ImapSession {
    pub async fn select_inbox(&mut self) -> Result<MailboxStatus, String>;
    pub async fn get_uid_list(&mut self) -> Result<Vec<u32>, String>;
    pub async fn fetch_email(&mut self, uid: u32) -> Result<ImapMessage, String>;
    pub async fn logout(&mut self) -> Result<(), String>;
}
```

### 3. 账户模型扩展

**Decision:** 扩展 DbAccount 支持 IMAP 配置

**新增字段:**
```rust
pub struct DbAccount {
    // 现有字段
    pub id: String,
    pub email: String,
    pub display_name: Option<String>,
    pub is_active: bool,
    
    // 协议类型
    pub protocol_type: String,  // "pop3" | "imap"
    
    // POP3 配置 (protocol_type = "pop3" 时使用)
    pub pop3_host: String,
    pub pop3_port: u16,
    pub use_ssl: bool,
    
    // IMAP 配置 (protocol_type = "imap" 时使用)
    pub imap_host: String,
    pub imap_port: u16,
    pub imap_use_ssl: bool,
    
    // 认证 (两种协议共用)
    pub password: String,  // 现有字段，存储位置需确认
}
```

**数据库迁移:**
```sql
ALTER TABLE accounts ADD COLUMN protocol_type TEXT DEFAULT 'pop3';
ALTER TABLE accounts ADD COLUMN imap_host TEXT;
ALTER TABLE accounts ADD COLUMN imap_port INTEGER DEFAULT 993;
ALTER TABLE accounts ADD COLUMN imap_use_ssl INTEGER DEFAULT 1;
```

### 4. 同步逻辑分支

**Decision:** 在 sync_emails 命令中根据协议类型分支

```rust
// src-tauri/src/commands/email.rs
#[tauri::command]
pub async fn sync_emails(account_id: String) -> Result<SyncResult, String> {
    let account = get_account(account_id)?;
    
    match account.protocol_type.as_str() {
        "pop3" => sync_via_pop3(account),
        "imap" => sync_via_imap(account),
        _ => Err("未知协议类型".to_string()),
    }
}

async fn sync_via_imap(account: DbAccount) -> Result<SyncResult, String> {
    // 1. 创建 IMAP 客户端
    let config = ImapConfig {
        host: account.imap_host,
        port: account.imap_port,
        use_ssl: account.imap_use_ssl,
    };
    let client = ImapClient::new(config, credentials);
    
    // 2. 连接并选择 INBOX
    let mut session = client.connect().await?;
    session.select_inbox().await?;
    
    // 3. 获取 UID 列表
    let server_uids = session.get_uid_list().await?;
    
    // 4. 对比本地已存储 UID
    let existing_uids = db.get_existing_uids(&account_id).await?;
    let new_uids = server_uids.into_iter()
        .filter(|uid| !existing_uids.contains(uid))
        .collect::<Vec<_>>();
    
    // 5. 下载新邮件
    for uid in new_uids {
        let email = session.fetch_email(uid).await?;
        // 解析并存储...
    }
    
    // 6. 断开
    session.logout().await?;
    
    Ok(SyncResult { ... })
}
```

### 5. 前端协议切换 UI

**Decision:** Radio 切换 POP3/IMAP，动态显示配置字段

**AddAccount.vue 结构:**
```vue
<template>
  <!-- 协议选择 -->
  <div class="protocol-select">
    <label>
      <input type="radio" value="pop3" v-model="protocol" />
      POP3
    </label>
    <label>
      <input type="radio" value="imap" v-model="protocol" />
      IMAP
    </label>
  </div>
  
  <!-- POP3 配置 -->
  <div v-if="protocol === 'pop3'" class="server-config">
    <input v-model="pop3Host" placeholder="POP3 服务器" />
    <input v-model="pop3Port" placeholder="端口" />
    <checkbox v-model="pop3SSL">使用 SSL</checkbox>
  </div>
  
  <!-- IMAP 配置 -->
  <div v-if="protocol === 'imap'" class="server-config">
    <input v-model="imapHost" placeholder="IMAP 服务器" />
    <input v-model="imapPort" placeholder="端口" />
    <checkbox v-model="imapSSL">使用 SSL</checkbox>
  </div>
  
  <!-- 公共字段 -->
  <input v-model="email" placeholder="邮箱地址" @change="autoFillServer" />
  <input v-model="password" type="password" placeholder="密码" />
</template>
```

### 6. 服务器预设配置

**Decision:** 根据邮箱域名自动预填充服务器配置

```typescript
// src/services/email/serverPresets.ts
export const SERVER_PRESETS: Record<string, ServerPreset> = {
  // Gmail
  'gmail.com': {
    pop3: { host: 'pop.gmail.com', port: 995, ssl: true },
    imap: { host: 'imap.gmail.com', port: 993, ssl: true },
  },
  // 163
  '163.com': {
    pop3: { host: 'pop.163.com', port: 995, ssl: true },
    imap: { host: 'imap.163.com', port: 993, ssl: true },
  },
  // QQ
  'qq.com': {
    pop3: { host: 'pop.qq.com', port: 995, ssl: true },
    imap: { host: 'imap.qq.com', port: 993, ssl: true },
  },
  // Outlook
  'outlook.com': {
    pop3: { host: 'pop-mail.outlook.com', port: 995, ssl: true },
    imap: { host: 'imap-mail.outlook.com', port: 993, ssl: true },
  },
  // Hotmail
  'hotmail.com': {
    pop3: { host: 'pop-mail.outlook.com', port: 995, ssl: true },
    imap: { host: 'imap-mail.outlook.com', port: 993, ssl: true },
  },
};

export function autoFillServer(email: string, protocol: 'pop3' | 'imap'): ServerConfig | null {
  const domain = email.split('@')[1]?.toLowerCase();
  if (!domain) return null;
  
  const preset = SERVER_PRESETS[domain];
  if (!preset) return null;
  
  return preset[protocol];
}
```

### 7. UID 增量同步策略

**Decision:** 基于 UID 增量同步，类似 POP3 的 UIDL

**同步流程:**
```
1. 连接 IMAP 服务器
2. SELECT INBOX
3. 获取 UIDVALIDITY（验证 UID 有效）
4. SEARCH UID * 获取所有邮件 UID
5. 对比本地已存储 UID
6. 对新邮件执行 UID FETCH
7. 解析并存储到 SQLite
8. LOGOUT
```

**UIDVALIDITY 处理:**
- 如果 UIDVALIDITY 变化，所有 UID 需要重新映射
- 精简版可暂不处理，后续迭代优化

## Risks / Trade-offs

### Risk 1: async-imap 库稳定性

**Risk:** async-imap 相较于 imap crate 社区使用较少

**Mitigation:**
- 先实现基础功能，验证可行性
- 如果遇到问题，考虑 fallback 到自研
- 参考 async-imap 示例代码

### Risk 2: IMAP 响应解析

**Risk:** IMAP 响应格式复杂，手动解析容易出错

**Mitigation:**
- 使用 async-imap 避免手动解析
- mail-parser 用于邮件内容解析（已有）

### Risk 3: POP3 和 IMAP 共存

**Risk:** 两种协议共存可能增加代码复杂度

**Mitigation:**
- 清晰的协议分支逻辑
- 共用邮件存储模型
- 共用 mail-parser 解析

### Trade-off: 精简版 IMAP

**Trade-off:** 精简版 vs 完整版 IMAP

**Reason:**
- 完整版需要文件夹管理、双向同步，工作量翻倍
- 精简版满足基础收件需求
- 用户可先添加 IMAP 账户体验，后续迭代增强

**Resolution:** 采用精简版 IMAP，后续迭代增加高级功能

## Migration Plan

### 阶段 1: Rust IMAP 客户端 (Day 1)

1. 添加 async-imap 依赖
2. 创建 imap/mod.rs, types.rs
3. 实现 imap/client.rs (连接/认证)
4. 实现 SELECT INBOX + UID FETCH

### 阶段 2: 数据库 + 命令层 (Day 2)

1. 数据库迁移 (accounts 表扩展)
2. DbAccount 模型扩展
3. imap_test_connection 命令
4. sync_emails 协议分支

### 阶段 3: 前端 UI (Day 3)

1. AddAccount.vue 协议选择 Radio
2. 动态显示 IMAP/POP3 配置字段
3. serverPresets.ts 服务器预设
4. AccountList 显示协议类型

### 阶段 4: 测试 (Day 4)

1. Gmail/163/QQ IMAP 连接测试
2. POP3 账户兼容性测试
3. IMAP 增量同步测试
4. 协议切换 UI 测试