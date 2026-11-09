# IMAP Email Sync Capability

## Summary

从 IMAP 服务器同步 INBOX 邮件到本地，支持增量同步。

## Interface

### 输入

```typescript
interface SyncRequest {
  account_id: string;  // 账户 ID
}
```

### 输出

```typescript
interface SyncResult {
  total: number;      // 服务器邮件总数
  new: number;        // 新同步邮件数
  success: boolean;   // 是否成功
  message: string;    // 结果消息
}
```

### 命令

| 命令 | 描述 |
|-----|------|
| `sync_emails(account_id)` | 同步指定账户的邮件（根据账户协议类型选择 POP3 或 IMAP） |

内部使用 `sync_via_imap()` 处理 IMAP 账户。

## Behavior

### 同步流程

```
1. 获取账户配置（验证 protocol_type = "imap"）
2. 创建 IMAP 客户端
3. 连接并认证
4. SELECT INBOX（进入收件箱）
5. SEARCH UID *（获取所有邮件 UID）
6. 对比本地已存储的 UID 列表
7. 对新 UID 执行 UID FETCH（获取邮件内容）
8. 使用 mail-parser 解析邮件
9. 存储到 SQLite（emails 表）
10. LOGOUT 断开连接
11. 返回同步结果
```

### 增量同步

- 使用 UID（Unique ID）作为邮件唯一标识
- 本地存储已同步的 UID 列表
- 仅下载 UID 列表中不存在的邮件
- UID 在同一 UIDVALIDITY 下唯一且持久

### 数据存储

邮件存储到 SQLite emails 表：

| 字段 | 来源 |
|-----|------|
| id | 生成唯一 ID：`imap-{account_id}-{uid}` |
| account_id | 账户 ID |
| uidl | IMAP UID（数字）转换为字符串 |
| from_address | mail-parser 解析 |
| from_name | mail-parser 解析 |
| to_addresses | mail-parser 解析（JSON 数组） |
| cc_addresses | mail-parser 解析（JSON 数组） |
| subject | mail-parser 解析 |
| body_text | mail-parser 解析（纯文本正文） |
| body_html | mail-parser 解析（HTML 正文） |
| date | mail-parser 解析（时间戳） |
| is_read | 默认 false（IMAP FLAGS 未实现） |
| is_starred | 默认 false（IMAP FLAGS 未实现） |
| has_attachment | mail-parser 判断 |

## Implementation Notes

### IMAP 命令序列

```rust
// SELECT INBOX
session.select("INBOX").await?;

// 获取 UID 列表
let uids = session.uid_search("ALL").await?;

// 获取邮件内容
for uid in new_uids {
    let fetch = session.uid_fetch(uid, "BODY[]").await?;
    let raw_content = fetch.first()?.body()?;
    // 解析并存储
}

// 断开
session.logout().await?;
```

### UIDVALIDITY 处理

- 精简版暂不处理 UIDVALIDITY 变化
- 后续迭代可增加：UIDVALIDITY 变化时重新映射所有 UID

## Limitations

- 仅同步 INBOX，不支持其他文件夹
- 不同步 FLAGS（已读、星标等状态）
- 不支持 CONDSTORE 高级增量同步
- 本地操作不同步回服务器

## Dependencies

- imap-connection capability
- mail-parser (邮件解析)
- SQLite (本地存储)