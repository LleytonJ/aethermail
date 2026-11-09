# Draft Folder 规范

## 功能概述

草稿文件夹保存用户撰写但未发送的邮件，支持创建、编辑、删除（仅本地保存）。

## 数据模型

```rust
// DbEmail 中 folder='drafts' 的记录
pub struct DraftEmail {
    pub id: String,           // 格式: "draft-{uuid}"
    pub account_id: String,
    pub folder: String,       // 值: "drafts"
    pub uidl: String,         // 格式: "draft-{uuid}"
    pub draft_id: String,     // UUID，用于更新/删除
    pub from_address: String, // 用户自己的邮箱
    pub to_addresses: String, // JSON array
    pub cc_addresses: String, // JSON array
    pub subject: Option<String>,
    pub body_text: Option<String>,
    pub body_html: Option<String>,
    pub date: i64,            // 最后保存时间戳
    pub is_read: bool,        // 默认 true
}
```

## API

### 后端命令

```rust
#[tauri::command]
pub async fn get_draft_emails(account_id: String) -> Result<Vec<Email>, String>

#[tauri::command]
pub async fn get_draft_count(account_id: String) -> Result<i32, String>

#[tauri::command]
pub async fn create_draft(
    account_id: String,
    to_addresses: Vec<String>,
    cc_addresses: Option<Vec<String>>,
    subject: Option<String>,
    body: Option<String>,
    body_html: Option<String>,
) -> Result<String, String>  // 返回 draft_id

#[tauri::command]
pub async fn update_draft(
    draft_id: String,
    to_addresses: Vec<String>,
    cc_addresses: Option<Vec<String>>,
    subject: Option<String>,
    body: Option<String>,
    body_html: Option<String>,
) -> Result<(), String>

#[tauri::command]
pub async fn delete_draft(draft_id: String) -> Result<(), String>

#[tauri::command]
pub async fn send_draft(draft_id: String) -> Result<SendEmailResult, String>
```

### 前端服务

```typescript
export async function getDraftEmails(accountId: string): Promise<Email[]>
export async function getDraftCount(accountId: string): Promise<number>
export async function createDraft(draft: DraftData): Promise<string>
export async function updateDraft(draftId: string, draft: DraftData): Promise<void>
export async function deleteDraft(draftId: string): Promise<void>
export async function sendDraft(draftId: string): Promise<SendResult>
```

## 流程

### 创建草稿流程

```
1. 用户撰写邮件
2. 点击"保存草稿"
3. 生成 draft_id (UUID)
4. 创建 DbEmail:
   - id = "draft-{draft_id}"
   - folder = "drafts"
   - uidl = "draft-{draft_id}"
   - draft_id = UUID
5. 保存到本地数据库
```

### 更新草稿流程

```
1. 用户编辑草稿
2. 点击"保存"
3. UPDATE emails WHERE draft_id=?
```

### 删除草稿流程

```
1. 用户点击删除
2. DELETE emails WHERE draft_id=? AND folder='drafts'
```

### 发送草稿流程

```
1. 用户点击发送
2. 获取草稿内容
3. 构建 RFC 2822 邮件
4. SMTP 发送
5. 发送成功：
   - UPDATE emails SET folder='sent', draft_id=NULL, uidl='sent-{ts}'
```

## UI

### Sidebar 入口

- 图标：文件图标（file-text）
- 标签："草稿箱"
- 计数：draftCount

### 列表页面

- 显示字段：收件人（可能空）、主题、保存时间
- 点击：打开撰写界面，加载草稿内容
- 右键菜单：删除草稿

### 撰写界面

- "保存草稿"按钮
- 编辑草稿时显示"草稿"标签

## POP3 和 IMAP 统一处理

本期 POP3 和 IMAP 账户统一处理，草稿仅保存在本地：

| 功能 | POP3 | IMAP |
|------|------|------|
| 本地保存 | ✅ | ✅ |
| 更新草稿 | ✅ 本地 UPDATE | ✅ 本地 UPDATE |
| 删除草稿 | ✅ 本地 DELETE | ✅ 本地 DELETE |
| 发送草稿 | ✅ 本地状态变更 | ✅ 本地状态变更 |

后续迭代可考虑为 IMAP 账户增加服务器同步功能。

## draft_id 设计

- 格式：UUID v4
- 生成时机：创建草稿时
- 用途：
  - 更新草稿定位记录
  - 删除草稿定位记录
  - 发送草稿定位记录
- 清空时机：发送成功后（draft_id=NULL）