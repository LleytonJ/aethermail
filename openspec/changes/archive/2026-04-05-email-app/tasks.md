# AetherMail 开发任务列表

> MVP 版本：仅收件 + 查看 | 预计工期：7 天 | 开发模式：全职

---

## Phase 0: 项目搭建 (Day 1)

### Task 0.1: 初始化 Tauri + Vue 3 项目

**描述：** 创建项目脚手架

**子任务：**
- [ ] 创建项目目录 `aethermail`
- [ ] 运行 `npm create tauri-app@latest` 初始化
- [ ] 选择 Vue + TypeScript 模板
- [ ] 验证基础项目可运行

**验收标准：**
```bash
cd aethermail
npm install
npm run tauri dev
# 能看到空白窗口
```

**预计时间：** 1 小时

---

### Task 0.2: 配置 Rust 依赖

**描述：** 添加 Cargo.toml 依赖

**子任务：**
- [ ] 编辑 `src-tauri/Cargo.toml`
- [ ] 添加 Tauri 插件：
  - `tauri = "2.0"` (core)
  - `tauri-plugin-sql = "2"` (SQLite)
  - `tauri-plugin-dialog = "2"` (文件对话框)
  - `tauri-plugin-notification = "2"` (通知)
  - `tauri-plugin-log = "2"` (日志)
- [ ] 添加网络库：
  - `tokio = { version = "1", features = ["full"] }`
  - `native-tls = "0.2"`
  - `tokio-native-tls = "0.3"`
- [ ] 添加邮件处理：
  - `mail-parser = "0.10"`
  - `lettre = { version = "0.11", features = ["smtp-transport", "builder", "tokio1-native-tls"] }`
- [ ] 添加工具库：
  - `serde = { version = "1", features = ["derive"] }`
  - `serde_json = "1"`
  - `thiserror = "1"`
  - `aes-gcm = "0.10"`
  - `base64 = "0.22"`

**验收标准：** `cargo build` 无错误

**预计时间：** 30 分钟

---

### Task 0.3: 配置前端依赖

**描述：** 添加 npm 包

**子任务：**
- [ ] 编辑 `package.json`
- [ ] 添加核心依赖：
  - `vue = "^3.4"`
  - `vue-router = "^4.2"`
  - `pinia = "^2.1"`
  - `@tauri-apps/api = "^2.0"`
  - `@tauri-apps/plugin-sql = "^2"`
  - `@tauri-apps/plugin-dialog = "^2"`
  - `@tauri-apps/plugin-notification = "^2"`
- [ ] 添加样式：
  - `tailwindcss = "^4.x"`
  - `@tailwindcss/vite = "^4.x"`
- [ ] 添加 UI 库：
  - `lucide-vue-next = "latest"` (图标)
- [ ] 添加工具：
  - `dompurify = "^3.x"`
  - `typescript = "^5.3"`

**验收标准：** `npm install` 无错误

**预计时间：** 30 分钟

---

### Task 0.4: 创建基础三栏布局

**描述：** 实现 MailLayout 组件

**文件：**
```
src/
├── components/
│   └── layout/
│       ├── MailLayout.vue    # 主布局
│       ├── Sidebar.vue       # 侧边栏 (空壳)
│       ├── EmailList.vue     # 邮件列表 (空壳)
│       └── ReadingPane.vue   # 阅读面板 (空壳)
├── styles/
│   └── globals.css           # Tailwind 引入
└── App.vue                   # 根组件
```

**MailLayout.vue 结构：**
```vue
<template>
  <div class="flex h-screen bg-bg-primary">
    <!-- 左侧边栏 -->
    <aside class="w-60 bg-bg-secondary border-r border-border-primary">
      <Sidebar />
    </aside>
    
    <!-- 中间邮件列表 -->
    <div class="w-96 bg-bg-primary border-r border-border-primary">
      <EmailList />
    </div>
    
    <!-- 右侧阅读面板 -->
    <main class="flex-1 bg-bg-primary">
      <ReadingPane />
    </main>
  </div>
</template>
```

**验收标准：** 能看到三栏布局，有背景色区分

**预计时间：** 2 小时

---

### Task 0.5: 配置 Tailwind CSS

**描述：** 设置主题颜色和深色模式

**文件：** `src/styles/globals.css`

**内容：**
```css
@import "tailwindcss";

@theme {
  /* 深色主题 - 默认 */
  --color-bg-primary: #1e1e2e;
  --color-bg-secondary: #252537;
  --color-bg-tertiary: #2d2d44;
  --color-bg-hover: #353550;
  --color-bg-selected: #404060;
  
  --color-text-primary: #ffffff;
  --color-text-secondary: #a0a0b0;
  --color-text-tertiary: #606070;
  
  --color-border-primary: #353550;
  --color-border-secondary: #2d2d44;
  
  --color-accent: #7c3aed;
  --color-accent-hover: #6d28d9;
  
  --color-danger: #ef4444;
  --color-success: #10b981;
}

/* 基础样式 */
body {
  @apply bg-bg-primary text-text-primary;
  margin: 0;
  padding: 0;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
}
```

**验收标准：** Tailwind 类可用，颜色正确

**预计时间：** 1 小时

---

### Task 0.6: 验证开发环境

**描述：** 完整测试开发流程

**检查清单：**
- [ ] `npm run tauri dev` 正常启动
- [ ] 热重载工作正常
- [ ] Rust 代码修改后重新编译
- [ ] Vue 组件修改后热更新
- [ ] 控制台无错误

**验收标准：** 开发环境完全可用

**预计时间：** 30 分钟

---

## Phase 1: POP3 客户端 + 数据库 (Day 2-3)

### Task 1.1: 实现 POP3 连接

**描述：** Rust 实现 POP3 基础连接

**文件：** `src-tauri/src/pop3/client.rs`

**功能：**
- [ ] TCP 连接到 POP3 服务器
- [ ] SSL/TLS 封装
- [ ] 读取服务器响应
- [ ] 错误处理

**代码结构：**
```rust
pub struct Pop3Client {
    stream: Option<BufReader<TcpStream>>,
}

pub struct Pop3Config {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub use_ssl: bool,
}

impl Pop3Client {
    pub async fn connect(config: &Pop3Config) -> Result<Self, Pop3Error>;
    pub async fn disconnect(&mut self) -> Result<(), Pop3Error>;
}
```

**验收标准：** 能连接到 POP3 服务器（如 `pop.gmail.com:995`）

**预计时间：** 3 小时

---

### Task 1.2: 实现 POP3 认证

**描述：** USER/PASS 认证

**文件：** `src-tauri/src/pop3/client.rs`

**功能：**
- [ ] 发送 USER 命令
- [ ] 发送 PASS 命令
- [ ] 解析 +OK / -ERR 响应
- [ ] 认证错误处理

**验收标准：** 认证成功/失败正确返回

**预计时间：** 2 小时

---

### Task 1.3: 实现 LIST 命令

**描述：** 获取邮件列表

**文件：** `src-tauri/src/pop3/client.rs`

**功能：**
- [ ] 发送 LIST 命令
- [ ] 解析邮件 UID 和大小
- [ ] 返回 Vec<Pop3MessageMeta>

**数据结构：**
```rust
pub struct Pop3MessageMeta {
    pub uid: u32,
    pub size: u32,
}
```

**验收标准：** 能获取邮箱中的邮件列表

**预计时间：** 2 小时

---

### Task 1.4: 实现 RETR 命令

**描述：** 获取邮件内容

**文件：** `src-tauri/src/pop3/client.rs`

**功能：**
- [ ] 发送 RETR <uid> 命令
- [ ] 读取完整邮件内容（含头部和正文）
- [ ] 处理多行响应
- [ ] 返回原始邮件字符串

**验收标准：** 能获取单封邮件的原始内容（RFC 2822 格式）

**预计时间：** 2 小时

---

### Task 1.5: 实现 QUIT 命令

**描述：** 断开连接

**文件：** `src-tauri/src/pop3/client.rs`

**功能：**
- [ ] 发送 QUIT 命令
- [ ] 关闭连接
- [ ] 清理资源

**验收标准：** 正常断开连接

**预计时间：** 30 分钟

---

### Task 1.6: 封装 Tauri 命令

**描述：** 暴露 POP3 功能给前端

**文件：** `src-tauri/src/commands.rs`

**命令列表：**
```rust
#[tauri::command]
pub async fn pop3_test_connection(config: Pop3Config) -> Result<String, String>;

#[tauri::command]
pub async fn pop3_list_messages(config: Pop3Config) -> Result<Vec<Pop3MessageMeta>, String>;

#[tauri::command]
pub async fn pop3_fetch_message(config: Pop3Config, uid: u32) -> Result<String, String>;
```

**注册命令：** `src-tauri/src/lib.rs`
```rust
.invoke_handler(tauri::generate_handler![
    pop3_test_connection,
    pop3_list_messages,
    pop3_fetch_message,
])
```

**验收标准：** 前端能调用这些命令

**预计时间：** 2 小时

---

### Task 1.7: 测试 POP3 连接

**描述：** 用真实邮箱测试

**测试账户：**
- [ ] Gmail: `pop.gmail.com:995` (需要开启 POP3)
- [ ] 163: `pop.163.com:110`
- [ ] QQ: `pop.qq.com:110`

**测试流程：**
```
1. 构建测试配置
2. 调用 pop3_test_connection
3. 调用 pop3_list_messages
4. 调用 pop3_fetch_message 获取 1 封
5. 打印原始邮件内容
```

**验收标准：** 3 个邮箱都能成功连接并获取邮件

**预计时间：** 2 小时

---

### Task 1.8: 创建数据库迁移

**描述：** 设计并创建 SQLite 表

**文件：** `src/services/db/migrations.ts`

**表结构：**
```sql
-- 账户表
CREATE TABLE accounts (
  id TEXT PRIMARY KEY,
  email TEXT NOT NULL UNIQUE,
  display_name TEXT,
  pop3_host TEXT,
  pop3_port INTEGER,
  pop3_security TEXT,
  pop3_username TEXT,
  pop3_password TEXT,  -- 加密存储
  smtp_host TEXT,
  smtp_port INTEGER,
  smtp_security TEXT,
  smtp_username TEXT,
  smtp_password TEXT,
  last_sync_at INTEGER,
  created_at INTEGER DEFAULT (unixepoch())
);

-- 邮件表
CREATE TABLE messages (
  id TEXT PRIMARY KEY,  -- pop3-{account_id}-{uid}
  account_id TEXT NOT NULL,
  from_address TEXT,
  from_name TEXT,
  to_addresses TEXT,
  subject TEXT,
  date INTEGER NOT NULL,
  is_read INTEGER DEFAULT 0,
  body_html TEXT,
  body_text TEXT,
  pop3_uid INTEGER NOT NULL,
  raw_content TEXT,
  created_at INTEGER DEFAULT (unixepoch())
);

-- 同步状态表
CREATE TABLE folder_sync_state (
  account_id TEXT NOT NULL,
  folder_name TEXT NOT NULL,
  last_uid INTEGER DEFAULT 0,
  PRIMARY KEY (account_id, folder_name)
);

-- 设置表
CREATE TABLE settings (
  key TEXT PRIMARY KEY,
  value TEXT NOT NULL
);
```

**验收标准：** 数据库表创建成功

**预计时间：** 2 小时

---

### Task 1.9: 实现账户 CRUD

**描述：** 账户的增删改查

**文件：** `src/services/db/accounts.ts`

**函数：**
```typescript
async function createAccount(account: Account): Promise<void>;
async function getAccount(id: string): Promise<Account | null>;
async function getAllAccounts(): Promise<Account[]>;
async function updateAccount(account: Account): Promise<void>;
async function deleteAccount(id: string): Promise<void>;
```

**验收标准：** 账户数据能正确存取

**预计时间：** 2 小时

---

### Task 1.10: 实现邮件 CRUD

**描述：** 邮件的增删改查

**文件：** `src/services/db/messages.ts`

**函数：**
```typescript
async function createMessage(message: Message): Promise<void>;
async function getMessage(id: string): Promise<Message | null>;
async function getMessagesByAccount(accountId: string, limit: number): Promise<Message[]>;
async function markAsRead(id: string): Promise<void>;
async function deleteMessage(id: string): Promise<void>;
```

**验收标准：** 邮件数据能正确存取

**预计时间：** 2 小时

---

### Task 1.11: 实现同步逻辑

**描述：** POP3 同步到本地

**文件：** `src/services/sync/pop3Sync.ts`

**流程：**
```typescript
async function syncAccount(accountId: string): Promise<SyncResult> {
  // 1. 获取账户配置
  const account = await getAccount(accountId);
  
  // 2. 获取同步状态 (last_uid)
  const syncState = await getFolderSyncState(accountId, 'INBOX');
  
  // 3. 连接 POP3
  const messages = await pop3ListMessages(config);
  
  // 4. 过滤新邮件
  const newMessages = messages.filter(m => m.uid > syncState.lastUid);
  
  // 5. 批量获取
  for (const msg of newMessages) {
    const raw = await pop3FetchMessage(config, msg.uid);
    const parsed = parseMessage(raw, accountId);
    await createMessage(parsed);
  }
  
  // 6. 更新同步状态
  await updateFolderSyncState(accountId, 'INBOX', { lastUid: maxUid });
  
  return { syncedCount: newMessages.length };
}
```

**验收标准：** 能同步新邮件到本地数据库

**预计时间：** 4 小时

---

### Task 1.12: 邮件解析

**描述：** 解析 RFC 2822 邮件

**文件：** `src/utils/emailParser.ts`

**功能：**
- [ ] 解析邮件头部 (From, To, Subject, Date)
- [ ] 解析 MIME 多部分
- [ ] 提取 HTML 正文
- [ ] 提取纯文本正文
- [ ] 提取附件信息

**验收标准：** 能正确解析邮件各部分

**预计时间：** 3 小时

---

## Phase 2: UI 实现 (Day 4-5)

### Task 2.1: 实现账户添加表单

**描述：** 添加 POP3 账户 UI

**文件：** `src/components/accounts/AddAccount.vue`

**字段：**
- [ ] 邮箱地址
- [ ] 显示名称（可选）
- [ ] POP3 服务器
- [ ] POP3 端口
- [ ] 用户名
- [ ] 密码
- [ ] SSL 复选框

**验收标准：** 表单能提交并调用后端添加账户

**预计时间：** 3 小时

---

### Task 2.2: 实现侧边栏

**描述：** 左侧导航栏

**文件：** `src/components/layout/Sidebar.vue`

**结构：**
```vue
<template>
  <div class="sidebar">
    <!-- 账户切换 -->
    <div class="account-section">
      <AccountSwitcher />
    </div>
    
    <!-- 文件夹列表 -->
    <nav class="folder-list">
      <FolderItem icon="inbox" label="收件箱" :count="unreadCount" />
      <FolderItem icon="sent" label="已发送" />
      <FolderItem icon="drafts" label="草稿箱" />
      <FolderItem icon="trash" label="已删除" />
    </nav>
    
    <!-- 标签列表 -->
    <div class="tags-section">
      <h3>标签</h3>
      <TagItem v-for="tag in tags" :key="tag.id" :tag="tag" />
    </div>
  </div>
</template>
```

**验收标准：** 侧边栏显示正确，文件夹可点击

**预计时间：** 3 小时

---

### Task 2.3: 实现邮件列表

**描述：** 中间邮件列表

**文件：** `src/components/layout/EmailList.vue`

**功能：**
- [ ] 显示发件人、主题、时间、摘要
- [ ] 已读/未读状态（背景色区分）
- [ ] 选中高亮
- [ ] 星标标识
- [ ] 附件图标

**数据结构：**
```typescript
interface EmailListItem {
  id: string;
  fromName: string;
  fromAddress: string;
  subject: string;
  snippet: string;
  date: number;
  isRead: boolean;
  isStarred: boolean;
  hasAttachment: boolean;
}
```

**验收标准：** 列表能显示邮件，样式正确

**预计时间：** 4 小时

---

### Task 2.4: 实现虚拟滚动

**描述：** 优化大列表性能

**文件：** `src/components/layout/EmailList.vue`

**方案：** 使用 `vue-virtual-scroller` 或自实现

```vue
<template>
  <RecycleScroller
    :items="emails"
    :item-size="72"
    key-field="id"
  >
    <template #default="{ item }">
      <EmailListItem :email="item" />
    </template>
  </RecycleScroller>
</template>
```

**验收标准：** 1000+ 邮件滚动流畅

**预计时间：** 2 小时

---

### Task 2.5: 实现阅读面板

**描述：** 右侧邮件阅读区

**文件：** `src/components/layout/ReadingPane.vue`

**结构：**
```vue
<template>
  <div class="reading-pane">
    <!-- 邮件头部 -->
    <div class="email-header">
      <div class="sender">
        <Avatar :email="email.fromAddress" />
        <div>
          <div class="name">{{ email.fromName }}</div>
          <div class="address">{{ email.fromAddress }}</div>
        </div>
      </div>
      <div class="meta">
        <span class="date">{{ formatDate(email.date) }}</span>
        <button class="star" :class="{ active: email.isStarred }" />
      </div>
    </div>
    
    <!-- 邮件正文 -->
    <EmailRenderer :html="email.bodyHtml" />
    
    <!-- 附件列表 -->
    <AttachmentList v-if="email.attachments?.length" :attachments="email.attachments" />
  </div>
</template>
```

**验收标准：** 能显示选中邮件的完整内容

**预计时间：** 3 小时

---

### Task 2.6: 实现 HTML 邮件渲染

**描述：** 安全渲染 HTML 邮件

**文件：** `src/components/email/EmailRenderer.vue`

**方案：** iframe 沙盒 + DOMPurify

```vue
<template>
  <iframe
    ref="iframeRef"
    sandbox="allow-same-origin"
    :srcdoc="sanitizedHtml"
    class="email-frame"
  />
</template>

<script setup lang="ts">
import DOMPurify from 'dompurify';

const props = defineProps<{ html: string }>();

const sanitizedHtml = computed(() => {
  return DOMPurify.sanitize(props.html, {
    ALLOWED_TAGS: ['p', 'br', 'div', 'span', 'b', 'i', 'strong', 'em', 'u', 'ul', 'ol', 'li', 'h1', 'h2', 'h3', 'a', 'img', 'table', 'tr', 'td'],
    ALLOWED_ATTR: ['href', 'src', 'alt', 'style', 'class'],
  });
});
</script>

<style scoped>
.email-frame {
  width: 100%;
  height: 100%;
  border: none;
  background: white;
}
</style>
```

**验收标准：** HTML 邮件正确显示，无 XSS 风险

**预计时间：** 2 小时

---

### Task 2.7: 实现 Pinia Store

**描述：** 状态管理

**文件：** `src/stores/mailbox.ts`

```typescript
import { defineStore } from 'pinia';

interface MailboxState {
  emails: EmailListItem[];
  selectedEmailId: string | null;
  loading: boolean;
  error: string | null;
}

interface MailboxActions {
  loadEmails: (accountId: string) => Promise<void>;
  selectEmail: (id: string) => Promise<void>;
  markAsRead: (id: string) => Promise<void>;
}

export const useMailboxStore = defineStore<'mailbox', MailboxState, MailboxActions>('mailbox', {
  state: () => ({
    emails: [],
    selectedEmailId: null,
    loading: false,
    error: null,
  }),
  actions: {
    async loadEmails(accountId: string) {
      this.loading = true;
      try {
        const emails = await invoke<EmailListItem[]>('get_messages', { accountId });
        this.emails = emails;
      } catch (e) {
        this.error = String(e);
      } finally {
        this.loading = false;
      }
    },
    async selectEmail(id: string) {
      this.selectedEmailId = id;
      // 加载完整邮件内容...
    },
  },
});
```

**验收标准：** 状态管理正常工作

**预计时间：** 2 小时

---

### Task 2.8: 实现搜索框

**描述：** 顶部搜索功能

**文件：** `src/components/search/SearchBar.vue`

**MVP 简化：** 仅前端过滤（后续版本用 FTS5）

```vue
<template>
  <div class="search-bar">
    <SearchIcon class="icon" />
    <input
      v-model="query"
      type="text"
      placeholder="搜索邮件..."
      class="search-input"
    />
  </div>
</template>

<script setup>
const query = ref('');
const filteredEmails = computed(() => {
  if (!query.value) return props.emails;
  const q = query.value.toLowerCase();
  return props.emails.filter(e =>
    e.subject.toLowerCase().includes(q) ||
    e.fromName.toLowerCase().includes(q)
  );
});
</script>
```

**验收标准：** 搜索框能过滤列表

**预计时间：** 1 小时

---

## Phase 3: 联调 + 测试 (Day 6-7)

### Task 3.1: 端到端测试

**描述：** 完整流程测试

**测试流程：**
```
1. 打开应用
2. 点击"添加账户"
3. 填写邮箱信息
4. 点击"测试连接"
5. 点击"保存"
6. 自动同步邮件
7. 查看邮件列表
8. 点击邮件阅读
```

**验收标准：** 流程顺畅无报错

**预计时间：** 3 小时

---

### Task 3.2: 错误处理优化

**描述：** 完善错误提示

**错误类型：**
- [ ] 网络连接失败
- [ ] 认证失败（密码错误）
- [ ] 服务器无响应
- [ ] 同步中断

**UI 反馈：**
```vue
<template>
  <div class="error-banner" v-if="error">
    <ErrorIcon />
    <span>{{ errorMessage }}</span>
    <button @click="retry">重试</button>
  </div>
</template>
```

**验收标准：** 错误有明确提示和重试选项

**预计时间：** 2 小时

---

### Task 3.3: 加载状态优化

**描述：** Skeleton 占位符

**文件：** `src/components/ui/Skeleton.vue`

```vue
<template>
  <div class="skeleton" :style="{ width, height }" />
</template>

<style scoped>
.skeleton {
  background: linear-gradient(90deg, #2d2d44 25%, #353550 50%, #2d2d44 75%);
  background-size: 200% 100%;
  animation: shimmer 1.5s infinite;
  border-radius: 4px;
}

@keyframes shimmer {
  0% { background-position: 200% 0; }
  100% { background-position: -200% 0; }
}
</style>
```

**验收标准：** 加载时有骨架屏

**预计时间：** 1 小时

---

### Task 3.4: 真实邮箱测试

**描述：** 多邮箱服务测试

**测试矩阵：**

| 邮箱 | POP3 服务器 | 端口 | SSL | 状态 |
|-----|-----------|------|-----|------|
| Gmail | pop.gmail.com | 995 | ✅ | ⬜ |
| 163 | pop.163.com | 110 | ❌ |  |
| QQ | pop.qq.com | 110 | ❌ | ⬜ |
| Outlook | pop-mail.outlook.com | 995 | ✅ | ⬜ |

**测试项：**
- [ ] 连接成功
- [ ] 认证成功
- [ ] 获取邮件列表
- [ ] 获取邮件内容
- [ ] 中文编码正确
- [ ] 附件解析正确

**验收标准：** 4 个邮箱服务都能正常工作

**预计时间：** 3 小时

---

### Task 3.5: 性能测试

**描述：** 大邮件量测试

**测试场景：**
- [ ] 100 封邮件 - 列表加载时间
- [ ] 1000 封邮件 - 滚动帧率
- [ ] 10000 封邮件 - 数据库查询时间

**目标指标：**
| 场景 | 目标 |
|-----|------|
| 100 封加载 | < 200ms |
| 1000 封滚动 | 60 FPS |
| 10000 封查询 | < 500ms |

**验收标准：** 达到性能指标

**预计时间：** 2 小时

---

### Task 3.6: 生产构建

**描述：** 打包发布

**命令：**
```bash
npm run tauri build
```

**输出：**
- Windows: `.msi` / `.exe`
- macOS: `.dmg` / `.app`
- Linux: `.deb` / `.AppImage`

**配置检查：**
- [ ] 应用图标
- [ ] 应用名称
- [ ] 版本号
- [ ] 版权信息

**验收标准：** 生成的安装包可正常安装运行

**预计时间：** 2 小时

---

### Task 3.7: 编写 README

**描述：** 项目文档

**文件：** `README.md`

**内容：**
```markdown
# AetherMail

现代化的个人桌面邮箱客户端

## 功能

- ✅ POP3 收件
- ✅ 邮件列表
- ✅ 邮件阅读
- ❌ SMTP 发件 (v0.2)

## 安装

[下载链接]

## 开发

```bash
npm install
npm run tauri dev
```

## 技术栈

- Tauri v2
- Vue 3
- Rust
- SQLite

## License

MIT
```

**验收标准：** README 完整清晰

**预计时间：** 1 小时

---

## 任务统计

| 阶段 | 任务数 | 预计时间 |
|-----|--------|---------|
| Phase 0: 项目搭建 | 6 | 7.5 小时 |
| Phase 1: POP3+ 数据库 | 12 | 30 小时 |
| Phase 2: UI 实现 | 8 | 20 小时 |
| Phase 3: 联调测试 | 7 | 14.5 小时 |
| **总计** | **33** | **72 小时** |

**按 7 天计算：** 约 10 小时/天（含休息和缓冲）

---

## 任务依赖图

```
Task 0.1 ─┬─ Task 0.2 ─┬─ Task 1.1 ── Task 1.6 ─┬─ Task 1.11 ─┬─ Task 3.1
          │           │           │           │            │
Task 0.3 ─           │           │           │            │
          │           │           │           │            │
Task 0.4 ─┤           │           │           │            │
          │           │           │           │            │
Task 0.5 ─           │           │           │            │
          │           │           │           │            │
Task 0.6 ─┘           │           │           │            │
                      │           │           │            │
                    Task 0.4 ─── Task 2.1 ─── Task 2.5 ───┘
                      │           │           │
                      │           │           │
                    Task 0.5 ─── Task 2.3 ─── Task 2.6 ─── Task 3.3
                      │           │
                      │           │
                    Task 0.6 ─── Task 2.7
```

---

**文档版本：** 0.1  
**最后更新：** 2026-04-05  
**状态：** 🟡 待审批
