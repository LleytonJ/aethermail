## Context

AetherMail 是一个基于 Tauri + Vue 3 的桌面邮件客户端。当前邮件撰写组件 `ComposeEmail.vue` 只支持文本和 HTML 内容，不支持附件。SMTP 客户端使用 `lettre` crate，目前只构建 `multipart/alternative` 格式的邮件。

### 现有架构

**前端撰写流程**:
- `ComposeEmail.vue` → `emailService.sendEmail()` → Tauri command `send_email` → `SmtpClient.send_email()`

**SMTP 消息构建**:
- 使用 `MultiPart::alternative()` 包含 text + HTML 两部分
- 不支持附件 MIME part

**附件现有功能**（仅接收端）:
- IMAP/POP3 接收邮件时解析附件信息存入数据库
- 用户点击附件时懒加载下载
- 缓存管理在 `~/.aethermail/attachments/`

## Goals / Non-Goals

**Goals:**
- 实现拖拽文件到撰写区域添加附件
- 实现点击按钮选择文件添加附件
- 附件列表在撰写界面展示，支持删除
- SMTP 发送邮件携带附件（multipart/mixed）
- 草稿保存时保留附件信息

**Non-Goals:**
- 不支持内嵌图片（CID 引用）作为附件显示
- 不支持拖拽整个邮件作为附件
- 不支持超大附件警告/限制（后续可扩展）

## Decisions

### 1. 附件存储策略

**选择**: 撰写时附件暂存内存，发送前转为 Base64 或临时文件

**原因**:
- 小附件（<5MB）可直接 Base64 编码传递给后端
- 大附件需要 Tauri 文件系统 API 读取路径传递
- 不预先写入缓存目录，避免发送失败后残留垃圾文件

**替代方案**:
- 预写入缓存目录 → 增加清理复杂度，放弃

### 2. 前端文件读取方式

**选择**: 使用 Tauri `fs` API 读取文件内容

**原因**:
- Web 标准 FileReader 在 Tauri 桌面环境可用
- 拖拽获取的是文件路径，需要通过 Tauri 读取实际内容
- 大文件需要分块读取避免内存溢出

**实现方式**:
```typescript
// 小文件直接读取
const content = await invoke('read_file', { path: filePath })

// 大文件传递路径，后端直接读取
await invoke('send_email', { ..., attachments: [{ path: filePath }] })
```

### 3. SMTP MIME 结构

**选择**: 使用 `multipart/mixed` + `multipart/alternative` 结构

**MIME 结构**:
```
multipart/mixed
├── multipart/alternative
│   ├── text/plain
│   └── text/html
├── attachment 1 (application/octet-stream 或实际类型)
├── attachment 2
└── ...
```

**原因**:
- 这是标准的带附件邮件格式
- lettre 支持通过 `MultiPart::mixed()` 构建

### 4. 草稿附件存储

**选择**: 新建 `draft_attachments` 表存储草稿附件

**表结构**:
```sql
CREATE TABLE draft_attachments (
  id TEXT PRIMARY KEY,
  draft_id TEXT NOT NULL,
  filename TEXT NOT NULL,
  content_type TEXT,
  file_path TEXT,        -- 原始文件路径（编辑草稿时可重读）
  file_size INTEGER,
  created_at INTEGER,
  FOREIGN KEY (draft_id) REFERENCES drafts(id) ON DELETE CASCADE
)
```

**原因**:
- 草稿可能多次编辑，需保留附件原始路径
- 发送草稿时从原始路径重新读取文件
- 删除草稿时自动清理附件记录

### 5. 拖拽区域设计

**选择**: 整个撰写区域作为拖拽目标，拖入时显示边框高亮

**原因**:
- 符合用户直觉（拖到哪里都行）
- 不需要额外预留拖拽区域
- 拖拽时视觉反馈清晰

**替代方案**:
- 专门的拖拽区域 → 占用空间，拖错位置用户困惑，放弃

## Risks / Trade-offs

### [风险] 大附件内存溢出
→ Mitigation: 附件 >10MB 时传递文件路径而非内容，后端流式读取

### [风险] 发送失败后附件丢失
→ Mitigation: 草稿保存时记录原始文件路径，用户可重新编辑发送

### [风险] 文件路径失效（用户移动/删除原文件）
→ Mitigation: 发送前检查文件是否存在，失败时提示用户重新添加

### [风险] 拖拽误操作
→ Mitigation: 拖拽时显示明确提示 "拖拽文件添加附件"，支持撤销删除