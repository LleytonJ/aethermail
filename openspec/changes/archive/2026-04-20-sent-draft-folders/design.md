## Context

AetherMail 目前实现了邮件接收（POP3/IMAP）和发送（SMTP），但发送的邮件没有保存记录。用户发送邮件后无法查看"已发送"列表，也无法在撰写过程中保存草稿。

### 当前数据模型

```
emails 表（无 folder 字段）
├── inbox: WHERE is_phishing=0 AND deleted_at IS NULL
├── phishing: WHERE is_phishing=1 AND deleted_at IS NULL
├── trash: WHERE deleted_at IS NOT NULL
```

当前通过 SQL WHERE 条件隐式区分"文件夹"，但没有区分收件和发件。

## Goals / Non-Goals

**Goals:**
- 数据模型增加 folder 字段，隔离收件和发件
- 发送邮件后保存到本地数据库
- 草稿创建、更新、删除功能
- 已发送和草稿列表 UI
- POP3 和 IMAP 账户统一本地处理

**Non-Goals:**
- 不实现服务器同步（IMAP APPEND）→ 后续迭代
- 不实现草稿自动保存 → 后续迭代
- 不实现邮件附件发送 → 后续迭代

## Decisions

### 1. 数据模型设计

**选择：单表 + folder 字段**

```
folder 取值：
- 'inbox'   - 收件箱（POP3/IMAP 同步）
- 'sent'    - 已发送（SMTP 发送成功，本地保存）
- 'drafts'  - 草稿箱（用户保存未发送，本地保存）
- 'phishing' - 钓鱼邮件（AI 检测标记）

trash 不作为 folder 值：
- deleted_at IS NOT NULL 表示回收站
- 原 folder 值保留，便于恢复时知道放回哪里
```

理由：
- 收件和发件本质不同，需要逻辑隔离
- 草稿和已发送是发件的两种状态，发送后 folder 从 'drafts' 改为 'sent'
- 单表减少 JOIN 复杂度，查询简单

### 2. folder 字段迁移策略

**选择：存量数据默认 'inbox'，钓鱼邮件迁移 'phishing'**

```sql
ALTER TABLE emails ADD COLUMN folder TEXT NOT NULL DEFAULT 'inbox';
UPDATE emails SET folder = 'phishing' WHERE is_phishing = 1;
```

理由：
- 存量邮件都是收件，默认 inbox 正确
- 钓鱼邮件已在 UI 有独立入口，需要迁移
- 新字段有默认值，无需处理 NULL

### 3. 草稿发送流程

**选择：发送后 UPDATE folder 为 'sent'**

流程：
1. 用户发送草稿
2. SMTP 发送邮件
3. UPDATE emails SET folder='sent', draft_id=NULL WHERE draft_id=?

理由：
- 草稿发送是状态变更，改变 folder 符合语义
- 避免创建新记录，保留原始撰写时间等信息

### 4. 本期不做服务器同步

**选择：本期仅本地保存，后续迭代再考虑 IMAP APPEND**

理由：
- 简化本期实现范围
- POP3 账户无法服务器同步，统一处理避免复杂分支
- 本地保存已满足基本需求
- 后续可单独迭代 IMAP 服务器同步功能

## Risks / Trade-offs

### 风险：多设备无法同步已发送/草稿
- 本期仅本地保存，换设备后看不到已发送邮件
- 缓解：后续迭代增加 IMAP 账户服务器同步

### 风险：草稿 ID 冲突
- 多设备同步可能导致草稿 ID 冲突（后期服务器同步时）
- 缓解：使用 UUID 生成 draft_id，概率极低

## Migration Plan

### 数据库迁移
```sql
ALTER TABLE emails ADD COLUMN folder TEXT NOT NULL DEFAULT 'inbox';
ALTER TABLE emails ADD COLUMN draft_id TEXT;
UPDATE emails SET folder = 'phishing' WHERE is_phishing = 1;
CREATE INDEX idx_emails_folder ON emails(account_id, folder);
```

### 回滚策略
- 删除 folder 和 draft_id 字段（SQLite 不支持 DROP COLUMN，保留字段）
- 前端代码回退到不显示已发送/草稿入口