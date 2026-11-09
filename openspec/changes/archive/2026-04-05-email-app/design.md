# AetherMail 技术设计文档

## 1. 系统架构

### 1.1 整体架构

```
┌─────────────────────────────────────────────────────────────────┐
│                    AetherMail Architecture                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │                  UI Layer (Vue 3 + TypeScript)             │ │
│  │  ┌──────────────┐ ┌──────────────┐ ┌──────────────────┐   │ │
│  │  │   Sidebar    │ │  Email List  │ │  Reading Pane    │   │ │
│  │  │  Component   │ │  Component   │ │   Component      │   │ │
│  │  └──────────────┘ └──────────────┘ └──────────────────┘   │ │
│  │         │                │                  │               │ │
│  │         └──────────────────────────────────┘               │ │
│  │                          ▼                                  │ │
│  │              ┌───────────────────────┐                      │ │
│  │              │   Pinia Store         │                      │ │
│  │              │   (State Management)  │                      │ │
│  │              └───────────────────────┘                      │ │
│  └────────────────────────────────────────────────────────────┘ │
│                              │ IPC (Tauri)                      │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │               Service Layer (TypeScript)                   │ │
│  │  ┌──────────────────────────────────────────────────────┐  │ │
│  │  │   EmailProvider 抽象层 (统一接口)                     │  │ │
│  │  └──────────────────────────────────────────────────────┘  │ │
│  └────────────────────────────────────────────────────────────┘ │
│                              │ IPC (Tauri)                      │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │               Backend Layer (Rust + Tauri v2)              │ │
│  │  ┌─────────────┐  ┌─────────────┐  ┌────────────────────┐  │ │
│  │  │  POP3 Client│  │  SMTP Client│  │  SQLite Database   │  │ │
│  │  │  (async)    │  │  (lettre)   │  │  (FTS5 Search)     │  │ │
│  │  └─────────────┘  └─────────────┘  └────────────────────┘  │ │
│  │  ┌─────────────┐  ┌─────────────┐  ┌────────────────────┐  │ │
│  │  │ System Tray │  │ Notifications│  │  File/Attachment   │  │ │
│  │  └─────────────┘  └─────────────┘  └────────────────────┘  │ │
│  └────────────────────────────────────────────────────────────┘ │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### 1.2 技术栈选型

| 层级 | 技术 | 版本 | 说明 |
|-----|------|------|------|
| **框架** | Tauri | v2.x | 跨平台桌面框架 |
| **前端** | Vue | v3.4+ | Composition API |
| **语言** | TypeScript | v5.3+ | 严格模式 |
| **状态管理** | Pinia | v2.1+ | Vue 官方推荐 |
| **路由** | Vue Router | v4.2+ | 单页面路由 |
| **样式** | Tailwind CSS | v4.x | 原子化 CSS |
| **UI 组件** | Lucide Vue | latest | 图标库 |
| **编辑器** | TipTap | v3.x | 富文本编辑 |
| **后端** | Rust | 1.77+ | 系统编程 |
| **数据库** | SQLite | v3.x | 本地存储 |
| **POP3** | 待定 | - | 收件协议 |
| **SMTP** | lettre | v0.11+ | 邮件发送 |

### 1.3 项目结构

```
aethermail/
├── src/                          # 前端源代码
│   ├── components/               # Vue 组件
│   │   ├── layout/               # 布局组件
│   │   │   ├── MailLayout.vue    # 三栏主布局
│   │   │   ├── Sidebar.vue       # 侧边导航
│   │   │   ├── EmailList.vue     # 邮件列表
│   │   │   ├── ReadingPane.vue   # 阅读面板
│   │   │   └── TitleBar.vue      # 自定义标题栏
│   │   ├── email/                # 邮件相关组件
│   │   │   ├── ThreadView.vue    # 会话视图
│   │   │   ├── MessageItem.vue   # 邮件项
│   │   │   ├── EmailRenderer.vue # HTML 渲染
│   │   │   └── AttachmentList.vue# 附件列表
│   │   ├── composer/             # 撰写邮件
│   │   │   ├── Composer.vue      # 撰写窗口
│   │   │   ├── AddressInput.vue  # 收件人输入
│   │   │   ├── EditorToolbar.vue # 工具栏
│   │   │   └── AttachmentPicker.vue
│   │   ├── accounts/             # 账户管理
│   │   │   ├── AccountList.vue
│   │   │   ├── AddAccount.vue
│   │   │   └── AccountSettings.vue
│   │   ├── search/               # 搜索
│   │   │   ├── SearchBar.vue
│   │   │   └── CommandPalette.vue
│   │   ├── settings/             # 设置
│   │   │   ├── SettingsPage.vue
│   │   │   ├── AccountEditor.vue
│   │   │   └── SignatureEditor.vue
│   │   └── ui/                   # 基础 UI
│   │       ├── Button.vue
│   │       ├── Input.vue
│   │       ├── Modal.vue
│   │       ├── ContextMenu.vue
│   │       └── Skeleton.vue
│   ├── stores/                   # Pinia 状态管理
│   │   ├── account.ts            # 账户状态
│   │   ├── mailbox.ts            # 邮箱状态
│   │   ├── email.ts              # 邮件状态
│   │   ├── composer.ts           # 撰写状态
│   │   └── ui.ts                 # UI 设置
│   ├── services/                 # 业务逻辑层
│   │   ├── db/                   # 数据库操作
│   │   │   ├── connection.ts     # 数据库连接
│   │   │   ├── migrations.ts     # 数据库迁移
│   │   │   ├── accounts.ts       # 账户 CRUD
│   │   │   ├── messages.ts       # 邮件 CRUD
│   │   │   ├── threads.ts        # 会话 CRUD
│   │   │   └── attachments.ts    # 附件 CRUD
│   │   ├── email/                # 邮件服务
│   │   │   ├── types.ts          # 类型定义
│   │   │   ├── provider.ts       # Provider 抽象
│   │   │   └── pop3SmtpProvider.ts
│   │   ├── sync/                 # 同步服务
│   │   │   ├── syncManager.ts    # 同步调度
│   │   │   └── pop3Sync.ts       # POP3 同步逻辑
│   │   └── search/               # 搜索服务
│   │       ├── queryParser.ts    # 查询解析
│   │       └── searchBuilder.ts  # SQL 构建
│   ├── hooks/                    # Composables
│   │   ├── useKeyboardShortcuts.ts
│   │   ├── useContextMenu.ts
│   │   └── useClickOutside.ts
│   ├── utils/                    # 工具函数
│   │   ├── date.ts
│   │   ├── email.ts
│   │   └── sanitize.ts
│   ├── constants/                # 常量
│   │   ├── shortcuts.ts
│   │   └── themes.ts
│   ├── styles/                   # 全局样式
│   │   └── globals.css
│   ├── router/                   # 路由配置
│   │   └── index.ts
│   ├── App.vue
│   └── main.ts
├── src-tauri/                    # Rust 后端
│   ├── src/
│   │   ├── lib.rs                # Tauri 入口
│   │   ├── main.rs               # 程序入口
│   │   ├── commands.rs           # Tauri 命令
│   │   ├── pop3/                 # POP3 模块
│   │   │   ├── mod.rs
│   │   │   ├── client.rs         # POP3 客户端
│   │   │   └── types.rs          # 类型定义
│   │   ├── smtp/                 # SMTP 模块
│   │   │   ├── mod.rs
│   │   │   ├── client.rs         # SMTP 客户端
│   │   │   └── types.rs
│   │   ├── db/                   # 数据库模块
│   │   │   └── mod.rs
│   │   └── util/                 # 工具模块
│   │       ├── crypto.rs         # 加密工具
│   │       └── error.rs          # 错误处理
│   ├── capabilities/             # Tauri 权限配置
│   │   └── default.json
│   ├── icons/                    # 应用图标
│   ├── Cargo.toml                # Rust 依赖
│   └── tauri.conf.json           # Tauri 配置
├── package.json
├── tsconfig.json
├── vite.config.ts
├── tailwind.config.ts
└── README.md
```

---

## 2. 数据库设计

### 2.1 ER 图

```
┌─────────────────┐       ┌─────────────────┐       ┌─────────────────┐
│    accounts     │       │     threads     │       │     labels      │
├─────────────────┤       ├─────────────────┤       ├─────────────────┤
│ id (PK)         │──┐    │ id (PK)         │    ┌──│ id (PK)         │
│ email           │  │    │ account_id (FK) │◀───┘  │ account_id (FK) │
│ display_name    │  └───▶│ subject         │       │ name            │
│ avatar_url      │       │ last_message_at │       │ type            │
│ pop3_host       │       │ message_count   │       │ color           │
│ pop3_port       │       │ is_read         │       │ imap_path       │
│ pop3_security   │       │ is_starred      │       └─────────────────┘
│ pop3_username   │       │ is_pinned       │
│ pop3_password   │       │ is_muted        │       ┌─────────────────┐
│ smtp_host       │       └─────────────────┘       │   messages      │
│ smtp_port       │                │                ├─────────────────┤
│ smtp_security   │                │                │ id (PK)         │
│ smtp_username   │                ▼                │ account_id (FK) │
│ smtp_password   │       ┌─────────────────┐       │ thread_id (FK)  │
│ created_at      │       │   attachments   │       │ from_address    │
└─────────────────┘       ├─────────────────┤       │ from_name       │
                          │ id (PK)         │       │ to_addresses    │
                          │ message_id (FK) │◀──────│ subject         │
                          │ filename        │       │ date            │
                          │ mime_type       │       │ is_read         │
                          │ size            │       │ body_html       │
                          │ content_id      │       │ body_text       │
                          └─────────────────┘       └─────────────────┘
```

### 2.2 表结构

#### accounts - 账户表

```sql
CREATE TABLE accounts (
  id TEXT PRIMARY KEY,                    -- UUID
  email TEXT NOT NULL UNIQUE,
  display_name TEXT,
  avatar_url TEXT,
  
  -- POP3 配置
  pop3_host TEXT,
  pop3_port INTEGER DEFAULT 995,
  pop3_security TEXT DEFAULT 'ssl',       -- 'ssl' | 'starttls' | 'none'
  pop3_username TEXT,
  pop3_password TEXT,                     -- AES-256-GCM 加密存储
  
  -- SMTP 配置
  smtp_host TEXT,
  smtp_port INTEGER DEFAULT 587,
  smtp_security TEXT DEFAULT 'starttls',
  smtp_username TEXT,
  smtp_password TEXT,                     -- AES-256-GCM 加密存储
  
  -- 状态
  is_active INTEGER DEFAULT 1,
  last_sync_at INTEGER,
  sync_interval INTEGER DEFAULT 300,      -- 同步间隔 (秒)
  
  -- 元数据
  created_at INTEGER DEFAULT (unixepoch()),
  updated_at INTEGER DEFAULT (unixepoch())
);

CREATE INDEX idx_accounts_email ON accounts(email);
CREATE INDEX idx_accounts_active ON accounts(is_active);
```

#### threads - 会话表

```sql
CREATE TABLE threads (
  id TEXT PRIMARY KEY,
  account_id TEXT NOT NULL,
  subject TEXT,
  snippet TEXT,                           -- 邮件摘要
  last_message_at INTEGER NOT NULL,
  message_count INTEGER DEFAULT 1,
  
  -- 状态
  is_read INTEGER DEFAULT 0,
  is_starred INTEGER DEFAULT 0,
  is_pinned INTEGER DEFAULT 0,
  is_muted INTEGER DEFAULT 0,
  has_attachments INTEGER DEFAULT 0,
  
  -- 发件人信息 (用于列表显示)
  from_address TEXT,
  from_name TEXT,
  
  FOREIGN KEY (account_id) REFERENCES accounts(id) ON DELETE CASCADE
);

CREATE INDEX idx_threads_account ON threads(account_id);
CREATE INDEX idx_threads_date ON threads(account_id, last_message_at DESC);
CREATE INDEX idx_threads_starred ON threads(is_starred);
CREATE INDEX idx_threads_unread ON threads(is_read);
```

#### messages - 邮件表

```sql
CREATE TABLE messages (
  id TEXT PRIMARY KEY,                    -- 格式：pop3-{account_id}-{uid}
  account_id TEXT NOT NULL,
  thread_id TEXT NOT NULL,
  
  -- 发件人
  from_address TEXT,
  from_name TEXT,
  
  -- 收件人
  to_addresses TEXT,                      -- JSON 数组
  cc_addresses TEXT,                      -- JSON 数组
  reply_to TEXT,                          -- JSON 数组
  
  -- 内容
  subject TEXT,
  snippet TEXT,
  body_html TEXT,
  body_text TEXT,
  
  -- 时间
  date INTEGER NOT NULL,                  -- 邮件发送时间戳
  internal_date INTEGER,                  -- 服务器接收时间
  
  -- 状态
  is_read INTEGER DEFAULT 0,
  is_starred INTEGER DEFAULT 0,
  has_attachments INTEGER DEFAULT 0,
  
  -- POP3 专用
  pop3_uid INTEGER NOT NULL,
  pop3_folder TEXT DEFAULT 'INBOX',
  
  FOREIGN KEY (account_id) REFERENCES accounts(id) ON DELETE CASCADE,
  FOREIGN KEY (thread_id) REFERENCES threads(id) ON DELETE CASCADE
);

CREATE INDEX idx_messages_account ON messages(account_id);
CREATE INDEX idx_messages_thread ON messages(account_id, thread_id);
CREATE INDEX idx_messages_date ON messages(account_id, date DESC);
CREATE INDEX idx_messages_from ON messages(from_address);

-- FTS5 全文索引
CREATE VIRTUAL TABLE messages_fts USING fts5(
  subject,
  from_name,
  from_address,
  body_text,
  snippet,
  content='messages',
  content_rowid='rowid',
  tokenize='trigram'
);

-- 触发器：自动维护 FTS 索引
CREATE TRIGGER messages_ai AFTER INSERT ON messages BEGIN
  INSERT INTO messages_fts(rowid, subject, from_name, from_address, body_text, snippet)
  VALUES (new.rowid, new.subject, new.from_name, new.from_address, new.body_text, new.snippet);
END;

CREATE TRIGGER messages_ad AFTER DELETE ON messages BEGIN
  INSERT INTO messages_fts(messages_fts, rowid, subject, from_name, from_address, body_text, snippet)
  VALUES('delete', old.rowid, old.subject, old.from_name, old.from_address, old.body_text, old.snippet);
END;

CREATE TRIGGER messages_au AFTER UPDATE ON messages BEGIN
  INSERT INTO messages_fts(messages_fts, rowid, subject, from_name, from_address, body_text, snippet)
  VALUES('delete', old.rowid, old.subject, old.from_name, old.from_address, old.body_text, old.snippet);
  INSERT INTO messages_fts(rowid, subject, from_name, from_address, body_text, snippet)
  VALUES (new.rowid, new.subject, new.from_name, new.from_address, new.body_text, new.snippet);
END;
```

#### labels - 标签表

```sql
CREATE TABLE labels (
  id TEXT NOT NULL,
  account_id TEXT NOT NULL,
  name TEXT NOT NULL,
  type TEXT NOT NULL,                      -- 'system' | 'user' | 'folder'
  color_bg TEXT,
  color_fg TEXT,
  parent_id TEXT,                          -- 父标签 ID (用于嵌套)
  sort_order INTEGER DEFAULT 0,
  is_visible INTEGER DEFAULT 1,
  
  -- POP3 文件夹映射
  pop3_folder_path TEXT,
  
  PRIMARY KEY (account_id, id),
  FOREIGN KEY (account_id) REFERENCES accounts(id) ON DELETE CASCADE
);

CREATE INDEX idx_labels_account ON labels(account_id);
```

#### attachments - 附件表

```sql
CREATE TABLE attachments (
  id TEXT PRIMARY KEY,
  message_id TEXT NOT NULL,
  account_id TEXT NOT NULL,
  filename TEXT NOT NULL,
  mime_type TEXT,
  size INTEGER,
  content_id TEXT,                         -- 用于 CID 内联图片
  local_path TEXT,                         -- 本地存储路径
  is_inline INTEGER DEFAULT 0,
  
  FOREIGN KEY (message_id) REFERENCES messages(id) ON DELETE CASCADE,
  FOREIGN KEY (account_id) REFERENCES accounts(id) ON DELETE CASCADE
);

CREATE INDEX idx_attachments_message ON attachments(message_id);
CREATE INDEX idx_attachments_account ON attachments(account_id);
```

#### settings - 设置表

```sql
CREATE TABLE settings (
  key TEXT PRIMARY KEY,
  value TEXT NOT NULL
);

-- 默认设置
INSERT OR IGNORE INTO settings (key, value) VALUES
  ('theme', 'system'),                      -- 'light' | 'dark' | 'system'
  ('sidebar_collapsed', 'false'),
  ('reading_pane_position', 'right'),       -- 'right' | 'bottom' | 'hidden'
  ('email_density', 'comfortable'),         -- 'comfortable' | 'compact'
  ('font_scale', '1.0'),
  ('sync_interval', '300'),
  ('notifications_enabled', 'true'),
  ('play_sound', 'true'),
  ('show_preview', 'true'),
  ('preview_lines', '2');
```

#### folder_sync_state - 文件夹同步状态 (POP3 专用)

```sql
CREATE TABLE folder_sync_state (
  account_id TEXT NOT NULL,
  folder_name TEXT NOT NULL,
  last_uid INTEGER DEFAULT 0,              -- 上次同步的最大 UID
  last_synced_at INTEGER,
  
  PRIMARY KEY (account_id, folder_name),
  FOREIGN KEY (account_id) REFERENCES accounts(id) ON DELETE CASCADE
);
```

---

## 3. 后端设计 (Rust)

### 3.1 POP3 客户端设计

```rust
// src-tauri/src/pop3/client.rs

use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use native_tls::TlsConnector;

pub struct Pop3Client {
    stream: Option<BufReader<TcpStream>>,
    host: String,
    port: u16,
}

pub struct Pop3Config {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub use_ssl: bool,
    pub use_starttls: bool,
}

pub struct Pop3Message {
    pub uid: u32,
    pub size: u32,
    pub raw_content: Vec<u8>,
}

impl Pop3Client {
    /// 连接到 POP3 服务器
    pub async fn connect(config: &Pop3Config) -> Result<Self, Pop3Error>;
    
    /// 认证 (USER/PASS 或 APOP)
    pub async fn authenticate(&mut self) -> Result<(), Pop3Error>;
    
    /// 获取邮件列表 (UIDL)
    pub async fn list_messages(&mut self) -> Result<Vec<Pop3MessageMeta>, Pop3Error>;
    
    /// 获取单封邮件内容 (RETR)
    pub async fn retrieve_message(&mut self, uid: u32) -> Result<Pop3Message, Pop3Error>;
    
    /// 标记删除 (DELE)
    pub async fn delete_message(&mut self, uid: u32) -> Result<(), Pop3Error>;
    
    /// 提交变更并断开 (QUIT)
    pub async fn quit(&mut self) -> Result<(), Pop3Error>;
    
    /// 测试连接
    pub async fn test_connection(config: &Pop3Config) -> Result<String, String>;
}
```

### 3.2 SMTP 客户端设计

```rust
// src-tauri/src/smtp/client.rs

use lettre::{
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
};

pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub use_ssl: bool,
    pub use_starttls: bool,
}

pub struct EmailContent {
    pub from: String,
    pub to: Vec<String>,
    pub cc: Vec<String>,
    pub bcc: Vec<String>,
    pub subject: String,
    pub body_html: String,
    pub body_text: String,
    pub attachments: Vec<AttachmentInfo>,
}

pub struct SmtpSendResult {
    pub message_id: String,
    pub status: String,
}

pub async fn send_email(config: &SmtpConfig, email: &EmailContent) -> Result<SmtpSendResult, SmtpError>;

pub async fn test_connection(config: &SmtpConfig) -> Result<String, String>;
```

### 3.3 Tauri 命令

```rust
// src-tauri/src/commands.rs

// ========== POP3 命令 ==========

#[tauri::command]
pub async fn pop3_test_connection(config: Pop3Config) -> Result<String, String>;

#[tauri::command]
pub async fn pop3_list_messages(config: Pop3Config) -> Result<Vec<Pop3MessageMeta>, String>;

#[tauri::command]
pub async fn pop3_fetch_message(config: Pop3Config, uid: u32) -> Result<Pop3Message, String>;

#[tauri::command]
pub async fn pop3_delete_messages(config: Pop3Config, uids: Vec<u32>) -> Result<(), String>;

// ========== SMTP 命令 ==========

#[tauri::command]
pub async fn smtp_send_email(config: SmtpConfig, email: EmailContent) -> Result<SmtpSendResult, String>;

#[tauri::command]
pub async fn smtp_test_connection(config: SmtpConfig) -> Result<String, String>;

// ========== 账户管理命令 ==========

#[tauri::command]
pub async fn add_account(
    email: String,
    display_name: String,
    pop3_config: Pop3Config,
    smtp_config: SmtpConfig,
) -> Result<String, String>;

#[tauri::command]
pub async fn remove_account(account_id: String) -> Result<(), String>;

#[tauri::command]
pub async fn sync_account(account_id: String) -> Result<SyncResult, String>;
```

### 3.4 依赖配置 (Cargo.toml)

```toml
[package]
name = "aethermail"
version = "0.1.0"
edition = "2021"

[lib]
name = "app_lib"
crate-type = ["staticlib", "cdylib", "rlib"]

[dependencies]
# Tauri 核心
tauri = { version = "2.0", features = ["tray-icon"] }
tauri-plugin-sql = { version = "2", features = ["sqlite"] }
tauri-plugin-notification = "2"
tauri-plugin-dialog = "2"
tauri-plugin-fs = "2"
tauri-plugin-http = "2"

# 异步运行时
tokio = { version = "1", features = ["full"] }

# 序列化
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# 网络
native-tls = "0.2"
tokio-native-tls = "0.3"

# 邮件处理
mail-parser = "0.10"
lettre = { version = "0.11", features = [
    "smtp-transport",
    "builder",
    "tokio1-native-tls",
    "hostname",
] }

# 加密
aes-gcm = "0.10"
base64 = "0.22"
ring = "0.17"

# 错误处理
thiserror = "1"
anyhow = "1"

# 日志
log = "0.4"
tauri-plugin-log = "2"
```

---

## 4. 前端设计 (Vue 3)

### 4.1 组件层级

```
┌─────────────────────────────────────────────────────────────┐
│  App.vue                                                     │
│  ├── TitleBar.vue                                           │
│  │    └── [窗口控制 + 全局搜索]                              │
│  ├── MailLayout.vue (三栏布局)                               │
│  │    ├── Sidebar.vue                                       │
│  │    │    ├── AccountSwitcher                              │
│  │    │    ├── ComposeButton                                │
│  │    │    ├── FolderList                                   │
│  │    │    └── TagList                                      │
│  │    ├── EmailList.vue                                     │
│  │    │    ├── SearchBar                                    │
│  │    │    ├── EmailListItem (v-for)                        │
│  │    │    └── VirtualScroll                                │
│  │    └── ReadingPane.vue                                   │
│  │         ├── EmailHeader                                  │
│  │         ├── ActionBar (回复/转发/删除...)                 │
│  │         ├── EmailRenderer                                │
│  │         └── AttachmentList                               │
│  └── ComposerWindow.vue (弹出)                               │
└─────────────────────────────────────────────────────────────┘
```

### 4.2 Pinia Store 设计

```typescript
// stores/account.ts
interface Account {
  id: string;
  email: string;
  displayName: string | null;
  avatarUrl: string | null;
  isActive: boolean;
}

interface AccountState {
  accounts: Account[];
  activeAccountId: string | null;
}

interface AccountActions {
  setAccounts: (accounts: Account[]) => void;
  setActiveAccount: (id: string) => void;
  addAccount: (account: Account) => void;
  removeAccount: (id: string) => void;
  syncAccount: (accountId: string) => Promise<void>;
}

// stores/mailbox.ts
interface Folder {
  id: string;
  name: string;
  type: 'system' | 'user';
  unreadCount: number;
  totalCount: number;
}

interface MailboxState {
  folders: Folder[];
  threads: Thread[];
  selectedThreadId: string | null;
  loading: boolean;
}

interface MailboxActions {
  loadFolders: (accountId: string) => Promise<void>;
  loadThreads: (folderId: string) => Promise<void>;
  selectThread: (id: string) => Promise<void>;
  markAsRead: (threadId: string) => Promise<void>;
  starThread: (threadId: string) => Promise<void>;
  deleteThread: (threadId: string) => Promise<void>;
}

// stores/composer.ts
interface ComposerState {
  isOpen: boolean;
  to: string[];
  cc: string[];
  bcc: string[];
  subject: string;
  body: string;
  attachments: Attachment[];
  isDraft: boolean;
}

// stores/ui.ts
interface UiState {
  sidebarCollapsed: boolean;
  readingPanePosition: 'right' | 'bottom' | 'hidden';
  theme: 'light' | 'dark' | 'system';
  density: 'comfortable' | 'compact';
  fontScale: number;
  isOnline: boolean;
}
```

### 4.3 键盘快捷键

```typescript
// constants/shortcuts.ts
export const SHORTCUTS = {
  // 导航
  'j': 'selectNext',
  'k': 'selectPrevious',
  'o': 'openThread',
  'Enter': 'openThread',
  
  // 操作
  'e': 'archive',
  's': 'toggleStar',
  'r': 'reply',
  'a': 'replyAll',
  'f': 'forward',
  '#': 'delete',
  'v': 'moveToFolder',
  'c': 'compose',
  
  // 跳转
  'g i': 'goToInbox',
  'g s': 'goToStarred',
  'g d': 'goToDrafts',
  
  // 搜索
  '/': 'focusSearch',
  'Escape': 'closePanel',
};
```

---

## 5. 同步策略

### 5.1 POP3 同步特点

```
┌─────────────────────────────────────────────────────────┐
│  POP3 vs IMAP 对比                                       │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  IMAP:                    │  POP3:                       │
│  - 双向同步               │  - 单向同步 (下载)           │
│  - 服务器保留状态         │  - 本地管理状态              │
│  - 支持增量 (UID + MODSEQ)│  - 仅支持 UID 跟踪            │
│  - 多文件夹               │  - 通常仅 INBOX              │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

### 5.2 POP3 同步流程

```
┌─────────────────────────────────────────────────────────┐
│  初始同步                                                 │
├─────────────────────────────────────────────────────────┤
│  1. 连接 POP3 服务器                                      │
│  2. UIDL 获取所有邮件 UID 列表                              │
│  3. 对比本地 folder_sync_state，确定新邮件                 │
│  4. 批量 RETR 下载新邮件 (每批 50 封)                        │
│  5. 解析邮件内容 (mail-parser)                            │
│  6. 构建会话树 (JWZ 算法，可选)                            │
│  7. 存储到 SQLite                                         │
│  8. 更新 folder_sync_state.last_uid                       │
│  9. QUIT 断开连接                                          │
└─────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────┐
│  增量同步 (定时轮询)                                      │
├─────────────────────────────────────────────────────────┤
│  1. 读取 folder_sync_state.last_uid                       │
│  2. UIDL 获取服务器所有 UID                                │
│  3. 过滤出 last_uid 之后的新 UID                           │
│  4. RETR 下载新邮件                                        │
│  5. 更新 last_uid                                         │
└─────────────────────────────────────────────────────────┘

同步间隔：默认 300 秒 (5 分钟)，用户可配置
后台服务：7 个检查器 (同步/定时发送/提醒等)
```

### 5.3 同步代码结构

```typescript
// services/sync/pop3Sync.ts
interface Pop3SyncProgress {
  phase: 'connect' | 'list' | 'fetch' | 'parse' | 'store' | 'done';
  current: number;
  total: number;
}

export async function syncAccount(
  accountId: string,
  onProgress?: (progress: Pop3SyncProgress) => void
): Promise<SyncResult> {
  // 1. 获取账户配置
  const account = await getAccount(accountId);
  const pop3Config = buildPop3Config(account);
  
  // 2. 获取同步状态
  const syncState = await getFolderSyncState(accountId, 'INBOX');
  const sinceUid = syncState?.lastUid ?? 0;
  
  // 3. 连接并获取邮件列表
  onProgress?.({ phase: 'connect', current: 0, total: 100 });
  const client = await pop3Connect(pop3Config);
  
  onProgress?.({ phase: 'list', current: 10, total: 100 });
  const allMessages = await pop3ListMessages(client);
  
  // 4. 过滤新邮件
  const newMessages = allMessages.filter(m => m.uid > sinceUid);
  
  // 5. 批量获取
  const batchSize = 50;
  const fetchedMessages: Pop3Message[] = [];
  for (let i = 0; i < newMessages.length; i += batchSize) {
    const batch = newMessages.slice(i, i + batchSize);
    const batchResults = await Promise.all(
      batch.map(m => pop3Retrieve(client, m.uid))
    );
    fetchedMessages.push(...batchResults);
    onProgress?.({ 
      phase: 'fetch', 
      current: i + batch.length, 
      total: newMessages.length 
    });
  }
  
  // 6. 解析并存储
  onProgress?.({ phase: 'parse', current: 0, total: fetchedMessages.length });
  const parsedMessages = fetchedMessages.map(m => parseMessage(m, accountId));
  
  // 7. 构建会话 (可选)
  const threadGroups = buildThreads(parsedMessages);
  
  // 8. 事务存储
  await withTransaction(async (db) => {
    for (const thread of threadGroups) {
      await upsertThread(db, thread);
      for (const msg of thread.messages) {
        await upsertMessage(db, msg);
      }
    }
  });
  
  // 9. 更新同步状态
  const maxUid = Math.max(...newMessages.map(m => m.uid), sinceUid);
  await updateFolderSyncState(accountId, 'INBOX', { lastUid: maxUid });
  
  // 10. 断开
  await pop3Quit(client);
  
  onProgress?.({ phase: 'done', current: 100, total: 100 });
  
  return {
    syncedCount: fetchedMessages.length,
    threadCount: threadGroups.length,
  };
}
```

---

## 6. 安全设计

### 6.1 密码加密存储

```rust
// src-tauri/src/util/crypto.rs

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use ring::rand::{SecureRandom, SystemRandom};

const KEY_SIZE: usize = 32;  // AES-256

/// 生成随机加密密钥
pub fn generate_key() -> Result<[u8; KEY_SIZE], CryptoError> {
    let rng = SystemRandom::new();
    let mut key = [0u8; KEY_SIZE];
    rng.fill(&mut key)?;
    Ok(key)
}

/// 加密密码
pub fn encrypt_password(key: &[u8], password: &str) -> Result<String, CryptoError> {
    let cipher = Aes256Gcm::new_from_slice(key)?;
    let mut nonce_bytes = [0u8; 12];
    SystemRandom::new().fill(&mut nonce_bytes)?;
    let nonce = Nonce::from_slice(&nonce_bytes);
    
    let ciphertext = cipher.encrypt(nonce, password.as_bytes())?;
    
    // 拼接 nonce + ciphertext 并 Base64 编码
    let mut combined = nonce_bytes.to_vec();
    combined.extend(ciphertext);
    Ok(base64::encode(combined))
}

/// 解密密码
pub fn decrypt_password(key: &[u8], encrypted: &str) -> Result<String, CryptoError> {
    let data = base64::decode(encrypted)?;
    if data.len() < 12 {
        return Err(CryptoError::InvalidData);
    }
    
    let nonce_bytes = &data[..12];
    let ciphertext = &data[12..];
    
    let cipher = Aes256Gcm::new_from_slice(key)?;
    let nonce = Nonce::from_slice(nonce_bytes);
    
    let plaintext = cipher.decrypt(nonce, ciphertext)?;
    Ok(String::from_utf8(plaintext)?)
}
```

### 6.2 HTML 邮件安全

```typescript
// utils/sanitize.ts
import DOMPurify from 'dompurify';

export const sanitizeHtml = (html: string): string => {
  return DOMPurify.sanitize(html, {
    // 允许的标签
    ALLOWED_TAGS: [
      'p', 'br', 'div', 'span',
      'b', 'i', 'strong', 'em', 'u',
      'ul', 'ol', 'li',
      'h1', 'h2', 'h3', 'h4', 'h5', 'h6',
      'blockquote', 'pre', 'code',
      'table', 'thead', 'tbody', 'tr', 'th', 'td',
      'a', 'img',
    ],
    // 允许的属性
    ALLOWED_ATTR: [
      'href', 'src', 'alt', 'title',
      'style', 'class', 'id',
      'colspan', 'rowspan',
    ],
    // 允许的协议
    ALLOWED_URI_REGEXP: /^(?:(?:(?:f|ht)tps?|mailto):|[^a-z]|[a-z+.\-]+(?:[^a-z+.\-:]|$))/i,
    // 阻止远程图片
    SANITIZE_DOMPURIFY_SANDBOX: true,
  });
};

// 渲染组件
// components/email/EmailRenderer.vue
<template>
  <iframe
    ref="iframeRef"
    sandbox="allow-same-origin"
    :srcdoc="sanitizedContent"
    class="email-content-frame"
  />
</template>

<script setup lang="ts">
import { sanitizeHtml } from '@/utils/sanitize';

const props = defineProps<{ htmlContent: string }>();
const sanitizedContent = computed(() => sanitizeHtml(props.htmlContent));
</script>
```

---

## 7. UI/UX 设计

### 7.1 三栏布局尺寸

```
默认窗口尺寸：1200 x 800

┌─────────────────────────────────────────────────────────────┐
│  TitleBar (32px)                                            │
├──────────┬───────────────────┬──────────────────────────────┤
│          │                   │                              │
│ Sidebar  │    Email List     │      Reading Pane            │
│ (240px)  │    (350px)        │      (flex-1)                │
│          │                   │                              │
│ [折叠]   │  [搜索框]         │  [邮件头部]                  │
│ ──────── │  ──────────────── │  ──────────────────────────  │
│ 账户切换 │  邮件项 1         │  [邮件正文]                  │
│ Compose  │  邮件项 2         │                              │
│ ──────── │  邮件项 3 (选中)  │  [附件列表]                  │
│ 文件夹   │  邮件项 4         │                              │
│ 收件箱   │  ...              │                              │
│ 已发送   │                   │                              │
│ 草稿箱   │  [虚拟滚动]       │                              │
│ 垃圾邮件 │                   │                              │
│ ──────── │                   │                              │
│ 标签     │                   │                              │
│ 工作     │                   │                              │
│ 个人     │                   │                              │
│          │                   │                              │
└──────────┴───────────────────┴──────────────────────────────┘
```

### 7.2 颜色主题

```css
/* styles/globals.css */
:root {
  /* 基础色板 - 深色主题 */
  --bg-primary: #1e1e2e;
  --bg-secondary: #252537;
  --bg-tertiary: #2d2d44;
  --bg-hover: #353550;
  --bg-selected: #404060;
  
  --text-primary: #ffffff;
  --text-secondary: #a0a0b0;
  --text-tertiary: #606070;
  
  --border-primary: #353550;
  --border-secondary: #2d2d44;
  
  /* 强调色 - 可切换 */
  --accent: #7c3aed;
  --accent-hover: #6d28d9;
  --accent-light: #8b5cf633;
  
  /* 功能色 */
  --danger: #ef4444;
  --warning: #f59e0b;
  --success: #10b981;
}

.dark {
  /* 深色主题变量 */
}

.light {
  /* 浅色主题变量 */
}
```

### 7.3 密度配置

```css
/* 舒适模式 */
.density-comfortable {
  --email-item-height: 72px;
  --email-item-padding: 16px;
  --sidebar-item-height: 40px;
}

/* 紧凑模式 */
.density-compact {
  --email-item-height: 56px;
  --email-item-padding: 12px;
  --sidebar-item-height: 32px;
}
```

---

## 8. 背景服务

### 8.1 服务架构

```typescript
// services/backgroundCheckers.ts
interface BackgroundService {
  name: string;
  interval: number;  // 毫秒
  handler: () => Promise<void>;
}

const services: BackgroundService[] = [
  {
    name: 'sync',
    interval: 300_000,  // 5 分钟
    handler: checkNewEmails,
  },
  {
    name: 'scheduledSend',
    interval: 60_000,   // 1 分钟
    handler: processScheduledEmails,
  },
  {
    name: 'queueProcessor',
    interval: 30_000,   // 30 秒
    handler: processOfflineQueue,
  },
];

export function startBackgroundServices() {
  services.forEach(service => {
    setInterval(service.handler, service.interval);
  });
}
```

### 8.2 系统通知

```typescript
// services/notifications/notificationManager.ts
import { sendMessage } from '@tauri-apps/api/core';
import { requestPermission } from '@tauri-apps/plugin-notification';

export async function initNotifications() {
  const permission = await requestPermission();
  if (permission !== 'granted') {
    console.warn('Notification permission not granted');
  }
}

export async function showNewEmailNotification(
  accountId: string,
  senderName: string,
  subject: string
) {
  await sendMessage('show_notification', {
    title: `新邮件来自 ${senderName}`,
    body: subject,
    icon: 'mail-unread',
  });
}
```

---

## 9. 错误处理

### 9.1 Rust 错误类型

```rust
// src-tauri/src/util/error.rs

#[derive(Debug, thiserror::Error)]
pub enum Pop3Error {
    #[error("Connection failed: {0}")]
    Connection(String),
    
    #[error("Authentication failed: {0}")]
    Auth(String),
    
    #[error("Protocol error: {0}")]
    Protocol(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("TLS error: {0}")]
    Tls(#[from] native_tls::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum SmtpError {
    #[error("Connection failed: {0}")]
    Connection(String),
    
    #[error("Authentication failed: {0}")]
    Auth(String),
    
    #[error("Send failed: {0}")]
    Send(String),
    
    #[error("Lettre error: {0}")]
    Lettre(#[from] lettre::error::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum CryptoError {
    #[error("Key generation failed")]
    KeyGeneration,
    
    #[error("Encryption failed: {0}")]
    Encryption(String),
    
    #[error("Decryption failed: {0}")]
    Decryption(String),
    
    #[error("Invalid data")]
    InvalidData,
}
```

### 9.2 TypeScript 错误处理

```typescript
// utils/error.ts
export class EmailError extends Error {
  constructor(
    message: string,
    public code: string,
    public details?: Record<string, unknown>
  ) {
    super(message);
    this.name = 'EmailError';
  }
}

export function isConnectionError(error: unknown): boolean {
  if (error instanceof Error) {
    const msg = error.message.toLowerCase();
    return (
      msg.includes('network') ||
      msg.includes('connection') ||
      msg.includes('timeout') ||
      msg.includes('econnrefused')
    );
  }
  return false;
}

// 错误边界处理
export async function withErrorHandling<T>(
  fn: () => Promise<T>,
  context: string
): Promise<T | null> {
  try {
    return await fn();
  } catch (error) {
    console.error(`[${context}]`, error);
    
    if (isConnectionError(error)) {
      // 触发离线模式
      useUiStore.getState().setOnline(false);
      // 加入队列
      await queueForRetry(fn, context);
    }
    
    return null;
  }
}
```

---

## 10. 开发里程碑

### Phase 0: 基础搭建 (第 1 周)

| 任务 | 描述 | 产出 |
|-----|------|------|
| 项目初始化 | Tauri + Vue 3 + TypeScript | 可运行的空壳项目 |
| 依赖配置 | Rust crate + npm package | package.json, Cargo.toml |
| 基础布局 | 三栏布局框架 | MailLayout.vue |
| 数据库连接 | SQLite + 迁移系统 | connection.ts, migrations.ts |

### Phase 1: 核心功能 (第 2-4 周)

| 任务 | 描述 | 产出 |
|-----|------|------|
| POP3 客户端 | Rust 实现连接/认证/获取 | pop3/client.rs |
| SMTP 客户端 | Rust 实现邮件发送 | smtp/client.rs |
| Tauri 命令 | 前后端通信接口 | commands.rs |
| 账户管理 | 添加/删除/切换账户 | stores/account.ts |
| 同步引擎 | POP3 同步逻辑 | services/sync/pop3Sync.ts |
| 邮件列表 | 加载/显示/选择 | EmailList.vue |
| 邮件阅读 | HTML 渲染/附件 | ReadingPane.vue |

### Phase 2: 增强功能 (第 5-6 周)

| 任务 | 描述 | 产出 |
|-----|------|------|
| 邮件撰写 | 富文本编辑器 | Composer.vue |
| 搜索功能 | FTS5 + 查询解析 | SearchBar.vue |
| 键盘快捷键 | Superhuman 风格 | useKeyboardShortcuts.ts |
| 系统通知 | 新邮件提醒 | notificationManager.ts |
| 离线支持 | 操作队列 | services/queue/ |

### Phase 3: 打磨发布 (第 7-8 周)

| 任务 | 描述 | 产出 |
|-----|------|------|
| 主题切换 | 深色/浅色模式 | themes.ts |
| 密度配置 | 舒适/紧凑模式 | globals.css |
| 性能优化 | 虚拟滚动/懒加载 | EmailList.vue |
| 测试 | 单元测试 + E2E | tests/ |
| 打包发布 | Windows/macOS/Linux | .exe, .dmg, .deb |

---

## 11. 风险与缓解

| 风险 | 影响 | 缓解措施 |
|-----|------|---------|
| POP3 无标准增量 | 高 | 本地 UID 跟踪，避免重复下载 |
| 邮件编码复杂 | 中 | 使用 mail-parser crate |
| HTML 邮件 XSS | 高 | DOMPurify + 沙盒 iframe |
| 密码存储安全 | 高 | AES-256-GCM 加密 |
| 大附件处理 | 中 | 流式处理，限制 25MB |
| 跨平台兼容性 | 中 | Tauri 抽象，充分测试 |

---

## 12. 参考资料

- [Velo 邮箱项目](https://github.com/avihaymenahem/velo) - Apache-2.0 许可
- [Tauri v2 文档](https://v2.tauri.app/)
- [lettre SMTP 库](https://github.com/lettre/lettre)
- [mail-parser](https://github.com/stalwartlabs/mail-parser)
- [DOMPurify](https://github.com/cure53/DOMPurify)
