# Sent Folder 规范

## 功能概述

已发送文件夹展示用户通过 SMTP 发送成功的邮件列表（仅本地保存）。

## 数据模型

```rust
// DbEmail 中 folder='sent' 的记录
pub struct SentEmail {
    pub id: String,           // 格式: "sent-{timestamp}"
    pub account_id: String,
    pub folder: String,       // 值: "sent"
    pub uidl: String,         // 格式: "sent-{timestamp}"
    pub from_address: String, // 用户自己的邮箱
    pub to_addresses: String, // JSON array
    pub cc_addresses: String, // JSON array
    pub subject: String,
    pub body_text: String,
    pub body_html: Option<String>,
    pub date: i64,            // 发送时间戳
    pub is_read: bool,        // 默认 true
}
```

## API

### 后端命令

```rust
#[tauri::command]
pub async fn get_sent_emails(account_id: String) -> Result<Vec<Email>, String>

#[tauri::command]
pub async fn get_sent_count(account_id: String) -> Result<i32, String>
```

### 前端服务

```typescript
export async function getSentEmails(accountId: string): Promise<Email[]>
export async function getSentCount(accountId: string): Promise<number>
```

## 流程

### 发送邮件保存流程

```
1. 用户点击发送
2. 构建 RFC 2822 邮件（含 Message-ID, Date）
3. SMTP 发送
4. 发送成功：
   - 解析邮件提取字段
   - 创建 DbEmail (folder='sent', uidl='sent-{ts}')
   - 保存到本地数据库
```

## UI

### Sidebar 入口

- 图标：发送图标（paper-plane）
- 标签："已发送"
- 计数：sentCount（可选显示）

### 列表页面

- 显示字段：收件人、主题、发送时间
- 点击：打开邮件详情
- 排序：发送时间倒序

## POP3 和 IMAP 统一处理

本期 POP3 和 IMAP 账户统一处理，已发送仅保存在本地：

| 功能 | POP3 | IMAP |
|------|------|------|
| 本地保存 | ✅ | ✅ |
| 服务器同步 | ❌ 本期不做 | ❌ 本期不做 |

后续迭代可考虑为 IMAP 账户增加服务器同步功能。