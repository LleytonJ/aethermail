## Phase 1: 数据模型层

### 1. 数据模型扩展
- [x] 1.1 修改 `storage/models.rs` DbEmail 结构
  - 新增 `folder: String` 字段（默认 'inbox'）
  - 新增 `draft_id: Option<String>` 字段
- [x] 1.2 新增 `EmailFolder` 枚举类型
  - Inbox, Sent, Drafts, Phishing

### 2. 数据库迁移
- [x] 2.1 修改 `storage/database.rs` init_tables
  - CREATE TABLE 添加 folder、draft_id 字段
- [x] 2.2 实现 run_migrations 迁移逻辑
  - 检测并添加 folder 字段
  - 存量钓鱼邮件迁移 folder='phishing'
  - 检测并添加 draft_id 字段
  - 创建 folder 索引

### 3. 数据库查询方法
- [x] 3.1 修改 `get_emails` 使用 folder='inbox' 过滤
- [x] 3.2 修改 `get_phishing_emails` 使用 folder='phishing' 过滤
- [x] 3.3 新增 `get_sent_emails(account_id)` 方法
- [x] 3.4 新增 `get_draft_emails(account_id)` 方法
- [x] 3.5 新增 `get_sent_count(account_id)` 方法
- [x] 3.6 新增 `get_draft_count(account_id)` 方法
- [x] 3.7 新增 `create_draft(draft)` 方法
- [x] 3.8 新增 `update_draft(draft_id, updates)` 方法
- [x] 3.9 新增 `delete_draft(draft_id)` 方法
- [x] 3.10 新增 `mark_draft_as_sent(draft_id, sent_uidl)` 方法
- [x] 3.11 修改 `save_email` 支持 folder 字段

---

## Phase 2: 后端命令层

### 4. Tauri 命令新增
- [x] 4.1 新增 `get_sent_emails(account_id)` 命令
- [x] 4.2 新增 `get_draft_emails(account_id)` 命令
- [x] 4.3 新增 `get_sent_count(account_id)` 命令
- [x] 4.4 新增 `get_draft_count(account_id)` 命令
- [x] 4.5 新增 `create_draft(...)` 命令
- [x] 4.6 新增 `update_draft(draft_id, ...)` 命令
- [x] 4.7 新增 `delete_draft(draft_id)` 命令
- [x] 4.8 新增 `send_draft(draft_id)` 命令

### 5. send_email 流程修改
- [x] 5.1 构建 RFC 2822 原始邮件（含 Message-ID）
- [x] 5.2 SMTP 发送邮件
- [x] 5.3 发送成功后保存到本地 DB（folder='sent'）

### 6. 命令注册
- [x] 6.1 修改 `commands/mod.rs` 导出新命令
- [x] 6.2 修改 `lib.rs` 注册新命令到 invoke_handler

---

## Phase 3: 前端层

### 7. Store 扩展
- [x] 7.1 修改 `stores/mailbox.ts`
  - 新增 sentEmails 状态
  - 新增 draftEmails 状态
  - 新增 sentCount 状态
  - 新增 draftCount 状态
  - 新增 loadSentEmails 方法
  - 新增 loadDraftEmails 方法
- [x] 7.2 修改 `stores/compose.ts`
  - 新增 draftId 状态
  - 新增 lastSavedAt 状态
  - 新增 isSaving 状态
  - 新增 saveDraft 方法
  - 新增 loadDraft 方法

### 8. 服务层
- [x] 8.1 修改 `services/email/emailService.ts`
  - 新增 getSentEmails 函数
  - 新增 getDraftEmails 函数
  - 新增 createDraft 函数
  - 新增 updateDraft 函数
  - 新增 deleteDraft 函数
  - 新增 sendDraft 函数

### 9. 路由扩展
- [x] 9.1 修改 `router/index.ts`
  - 路由不需要修改，通过 MailLayout 内的 currentFolder 状态切换

### 10. UI 组件
- [x] 10.1 修改 Sidebar 组件
  - 新增"已发送"入口（图标 + 计数）
  - 新增"草稿箱"入口（图标 + 计数）
- [x] 10.2 已发送列表组件
  - 使用现有 EmailList 组件，修改 EmailListItem 显示收件人
- [x] 10.3 草稿列表组件
  - 使用现有 EmailList 组件，点击时需要打开编辑模式

### 11. 撰写组件修改
- [x] 11.1 修改 ComposeEmail.vue
  - 新增"保存草稿"按钮
  - 编辑草稿时加载草稿内容
  - 发送草稿时调用 sendDraft

---

## Phase 4: 测试验证

### 12. 后端测试
- [ ] 12.1 测试数据库迁移
  - 存量数据 folder 正确
  - 钓鱼邮件迁移正确
- [ ] 12.2 测试已发送保存
  - SMTP 发送成功后本地保存
- [ ] 12.3 测试草稿功能
  - 创建草稿
  - 更新草稿
  - 删除草稿
  - 发送草稿

### 13. 前端测试
- [ ] 13.1 测试已发送列表
  - 列表显示正确
  - 详情查看正确
- [ ] 13.2 测试草稿列表
  - 列表显示正确
  - 继续编辑正确
  - 删除草稿正确

---

## 实现进度

| 阶段 | 状态 | 备注 |
|-----|------|------|
| Phase 1: 数据模型 | ✅ 完成 | 数据库迁移、查询方法已实现 |
| Phase 2: 后端命令 | ✅ 完成 | Tauri 命令已注册 |
| Phase 3: 前端层 | ✅ 完成 | Store、服务层、UI 组件已修改 |
| Phase 4: 测试验证 | ⏳ 待测试 | 需手动测试验证 |

**注意**: ContactEditor.vue 存在现有 TypeScript 错误，与本次改动无关。

---

## 任务依赖关系

```
Phase 1 (数据模型)
├── 1.x (模型扩展)
│   └── 2.x (数据库迁移)
│       └── 3.x (查询方法)
│
Phase 2 (后端命令)
├── 4.x (Tauri 命令) ──────┬── 5.x (send_email 修改)
│                         │
│                         └── 6.x (命令注册)
│
Phase 3 (前端)
├── 7.x (Store) ──────────┤
├── 8.x (服务层) ─────────┤
├── 9.x (路由) ───────────┤
├── 10.x (UI 组件) ───────┤
├── 11.x (撰写修改) ──────┘
│
Phase 4 (测试)
└── 12.x + 13.x (测试验证)
```

## 工时估算

| 阶段 | 预计工时 |
|-----|---------|
| Phase 1: 数据模型 | 2 小时 |
| Phase 2: 后端命令 | 2 小时 |
| Phase 3: 前端层 | 3 小时 |
| Phase 4: 测试验证 | 1 小时 |
| **总计** | **8 小时** |

考虑缓冲和调试时间，预计 **1-2 天** 完成全部任务。