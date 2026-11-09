# Email Storage 规范（扩展）

## 背景

本文档是对现有 email-storage 规范的扩展，新增 folder 和 draft_id 字段。

## 数据模型扩展

### DbEmail 新增字段

```rust
pub struct DbEmail {
    pub id: String,
    pub account_id: String,
    pub uidl: String,
    
    // 新增字段 ↓
    pub folder: String,           // "inbox" | "sent" | "drafts" | "phishing"
    pub draft_id: Option<String>, // 仅草稿有值，发送后清空
    
    // 现有字段
    pub from_address: Option<String>,
    pub from_name: Option<String>,
    pub to_addresses: String,
    pub cc_addresses: String,
    pub subject: Option<String>,
    pub body_text: Option<String>,
    pub body_html: Option<String>,
    pub date: i64,
    pub is_read: bool,
    pub is_starred: bool,
    pub has_attachment: bool,
    pub is_phishing: bool,
    pub phishing_reasons: Option<String>,
    pub user_overridden: bool,
    pub deleted_at: Option<i64>,
}
```

### folder 字段取值

| 值 | 来源 | uidl 格式 | draft_id |
|---|------|----------|----------|
| inbox | POP3/IMAP 同步 | 服务器 UID | NULL |
| sent | SMTP 发送成功 | sent-{timestamp} | NULL |
| drafts | 用户保存草稿 | draft-{uuid} | {uuid} |
| phishing | AI 检测标记 | 服务器 UID | NULL |

### 回收站处理

回收站不作为独立 folder 值：
- `deleted_at IS NOT NULL` 表示在回收站
- 原 folder 值保留，便于恢复时知道放回哪里

## 数据库迁移

### 迁移脚本

```sql
-- 1. 添加 folder 字段（默认 inbox）
ALTER TABLE emails ADD COLUMN folder TEXT NOT NULL DEFAULT 'inbox';

-- 2. 存量钓鱼邮件迁移
UPDATE emails SET folder = 'phishing' WHERE is_phishing = 1;

-- 3. 创建索引
CREATE INDEX idx_emails_folder ON emails(account_id, folder);

-- 4. 添加 draft_id 字段
ALTER TABLE emails ADD COLUMN draft_id TEXT;
```

### 迁移时机

- 应用启动时检测字段是否存在
- 存量数据自动迁移
- 兼容旧版本数据

## 查询方法变更

### 现有方法修改

```rust
// get_emails: 只查 inbox
WHERE folder = 'inbox' AND deleted_at IS NULL

// get_phishing_emails: 只查 phishing
WHERE folder = 'phishing' AND deleted_at IS NULL

// get_starred_emails: 跨 inbox 和 sent
WHERE is_starred = 1 AND folder IN ('inbox', 'sent') AND deleted_at IS NULL
```

### 新增方法

```rust
// 已发送
get_sent_emails(account_id) -> WHERE folder = 'sent' AND deleted_at IS NULL
get_sent_count(account_id) -> SELECT COUNT(*) WHERE folder = 'sent' AND deleted_at IS NULL

// 草稿
get_draft_emails(account_id) -> WHERE folder = 'drafts' AND deleted_at IS NULL
get_draft_count(account_id) -> SELECT COUNT(*) WHERE folder = 'drafts' AND deleted_at IS NULL

// 草稿操作
create_draft(draft) -> INSERT with folder = 'drafts', draft_id = uuid
update_draft(draft_id, updates) -> UPDATE WHERE draft_id = ?
delete_draft(draft_id) -> DELETE WHERE draft_id = ? AND folder = 'drafts'
mark_draft_as_sent(draft_id, sent_uidl) -> UPDATE folder = 'sent', draft_id = NULL, uidl = ?
```

## save_email 方法扩展

```rust
pub async fn save_email(&self, email: &DbEmail) -> Result<(), String> {
    sqlx::query(
        r#"
        INSERT OR REPLACE INTO emails
        (id, account_id, uidl, folder, draft_id, from_address, from_name, 
         to_addresses, cc_addresses, subject, body_text, body_html, date, 
         is_read, is_starred, has_attachment, is_phishing, phishing_reasons, 
         user_overridden, deleted_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&email.id)
    .bind(&email.account_id)
    .bind(&email.uidl)
    .bind(&email.folder)         // 新增
    .bind(&email.draft_id)       // 新增
    // ... 其他字段
    .execute(&self.pool)
    .await?;
}
```

## 唯一约束调整

当前约束：`UNIQUE(account_id, uidl)`

需要确认不同 folder 的 uidl 不会冲突：
- inbox: 服务器 UID（数字）
- sent: sent-{timestamp}（字符串）
- drafts: draft-{uuid}（字符串）

格式不同，不会冲突。

## 索引优化

```sql
-- 主索引（已有）
CREATE INDEX idx_emails_account ON emails(account_id);
CREATE INDEX idx_emails_date ON emails(date);
CREATE INDEX idx_emails_uidl ON emails(account_id, uidl);

-- 新增索引
CREATE INDEX idx_emails_folder ON emails(account_id, folder);
CREATE INDEX idx_emails_draft_id ON emails(draft_id);
```