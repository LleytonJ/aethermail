---
name: attachment-lazy-load
description: 附件懒加载实现任务列表
---

# 任务列表：附件懒加载

## 阶段 1：数据库层改造

### Task 1.1: 更新数据库模型
- **文件**: `src-tauri/src/storage/models.rs`
- **内容**:
  - 移除 `DbAttachment.content` 字段
  - 新增 `imap_section: Option<String>`
  - 新增 `is_downloaded: bool`
  - 新增 `local_path: Option<String>`
- **依赖**: 无

### Task 1.2: 更新数据库表结构
- **文件**: `src-tauri/src/storage/database.rs`
- **内容**:
  - 修改 `init_tables()` 中的 CREATE TABLE 语句
  - 添加数据库迁移逻辑（处理存量数据）
  - 新增 `get_attachment_by_id()` 方法
  - 新增 `update_attachment_downloaded()` 方法
  - 新增 `get_attachments_with_content()` 方法（用于迁移）
  - 新增 `mark_all_attachments_not_downloaded()` 方法
  - 移除 `save_attachment()` 中的 content 参数
- **依赖**: Task 1.1

### Task 1.3: 实现数据迁移
- **文件**: `src-tauri/src/storage/database.rs` 或新文件
- **内容**:
  - 检测旧表结构（content 列存在）
  - 将 BLOB 数据写入文件缓存
  - 更新 is_downloaded 和 local_path
  - 移除 content 列
- **依赖**: Task 1.2

---

## 阶段 2：IMAP 客户端改造

### Task 2.1: 新增 MIME 结构解析
- **文件**: `src-tauri/src/imap/client.rs`
- **内容**:
  - 新增 `EmailStructure` 类型定义
  - 实现 `fetch_structure(uid)` 方法
  - 解析 BODYSTRUCTURE 响应
  - 提取各部分的 section 编号
- **依赖**: 无

### Task 2.2: 新增正文单独获取
- **文件**: `src-tauri/src/imap/client.rs`
- **内容**:
  - 实现 `fetch_body_only(uid)` 方法
  - 使用 `BODY.PEEK[1]` 获取正文
  - 返回 (text_body, html_body)
- **依赖**: Task 2.1

### Task 2.3: 新增附件部分获取
- **文件**: `src-tauri/src/imap/client.rs`
- **内容**:
  - 实现 `fetch_attachment_section(uid, section)` 方法
  - 使用 `BODY[section]` 获取指定部分
  - 返回原始二进制数据
- **依赖**: Task 2.1

### Task 2.4: 新增 IMAP 类型定义
- **文件**: `src-tauri/src/imap/types.rs`
- **内容**:
  - 新增 `EmailStructure` 结构体
  - 新增 `MimePart` 结构体（含 section、filename、mime_type、size、is_attachment）
- **依赖**: 无

---

## 阶段 3：邮件同步逻辑改造

### Task 3.1: 改造 IMAP 同步流程
- **文件**: `src-tauri/src/commands/email.rs`
- **内容**:
  - 修改 `sync_via_imap()` 函数
  - 调用 `fetch_structure()` 获取 MIME 结构
  - 调用 `fetch_body_only()` 只获取正文
  - 解析附件信息（无内容）
  - 存入数据库时设置 `is_downloaded=false`
- **依赖**: Task 2.1, Task 2.2, Task 1.2

### Task 3.2: 改造 POP3 同步流程
- **文件**: `src-tauri/src/commands/email.rs`
- **内容**:
  - 修改 `sync_via_pop3()` 函数
  - POP3 全量下载后保存到文件缓存
  - 设置 `is_downloaded=true`、`local_path`
  - 不再存储到数据库 BLOB
- **依赖**: Task 1.2

### Task 3.3: 改造 parse_email 函数
- **文件**: `src-tauri/src/commands/email.rs`
- **内容**:
  - 修改 `parse_email()` 返回值
  - 附件信息包含 imap_section
  - POP3 分支保持全量解析
- **依赖**: Task 2.1

---

## 阶段 4：附件下载命令

### Task 4.1: 新增缓存路径管理
- **文件**: `src-tauri/src/commands/email.rs` 或新文件
- **内容**:
  - 实现 `get_cache_directory(app_handle)` 函数
  - 实现 `get_attachment_cache_path(app_handle, id)` 函数
  - 自动创建缓存目录
- **依赖**: 无

### Task 4.2: 实现下载命令
- **文件**: `src-tauri/src/commands/email.rs`
- **内容**:
  - 新增 `AttachmentProgress` 事件结构体
  - 新增 `AttachmentComplete` 事件结构体
  - 实现 `download_attachment(attachment_id, app_handle)` 命令
  - 分块下载 + 发送进度事件
  - 写入缓存文件
  - 更新数据库状态
- **依赖**: Task 2.3, Task 1.2, Task 4.1

### Task 4.3: 实现保存到指定路径命令
- **文件**: `src-tauri/src/commands/email.rs`
- **内容**:
  - 实现 `save_attachment_to_path(attachment_id, target_path)` 命令
  - 从缓存文件复制到用户指定位置
- **依赖**: Task 4.2

### Task 4.4: 注册新命令
- **文件**: `src-tauri/src/lib.rs`
- **内容**:
  - 在 `invoke_handler` 中注册新命令
- **依赖**: Task 4.2, Task 4.3

---

## 阶段 5：缓存管理命令

### Task 5.1: 实现缓存统计命令
- **文件**: `src-tauri/src/commands/email.rs`
- **内容**:
  - 新增 `CacheStats` 结构体
  - 实现 `get_attachment_cache_stats(app_handle)` 命令
  - 返回文件数量、总大小、缓存路径
- **依赖**: Task 4.1

### Task 5.2: 实现清空缓存命令
- **文件**: `src-tauri/src/commands/email.rs`
- **内容**:
  - 实现 `clear_attachment_cache(app_handle)` 命令
  - 删除所有缓存文件
  - 更新数据库 `is_downloaded=false`
- **依赖**: Task 4.1, Task 1.2

### Task 5.3: 实现打开缓存目录命令
- **文件**: `src-tauri/src/commands/email.rs`
- **内容**:
  - 实现 `open_attachment_cache_directory(app_handle)` 命令
  - 跨平台支持 (Windows/macOS/Linux)
- **依赖**: Task 4.1

---

## 阶段 6：前端服务层

### Task 6.1: 更新 emailService.ts
- **文件**: `src/services/email/emailService.ts`
- **内容**:
  - 更新 `AttachmentInfo` 类型定义
  - 新增 `downloadAttachment(id)` 函数
  - 新增 `saveAttachmentToPath(id, path)` 函数
- **依赖**: Task 4.2, Task 4.3

### Task 6.2: 新增缓存服务
- **文件**: `src/services/email/cacheService.ts` (新文件)
- **内容**:
  - 新增 `getCacheStats()` 函数
  - 新增 `clearCache()` 函数
  - 新增 `openCacheDirectory()` 函数
- **依赖**: Task 5.1, Task 5.2, Task 5.3

### Task 6.3: 新增离线检测服务
- **文件**: `src/services/email/offlineService.ts` (新文件)
- **内容**:
  - 导出 `isOffline()` 函数
  - 导出 `getOfflineMessage()` 函数
- **依赖**: 无

---

## 阶段 7：前端组件改造

### Task 7.1: 更新 mailbox store 类型
- **文件**: `src/stores/mailbox.ts`
- **内容**:
  - 更新 `Attachment` 类型定义
  - 新增 `isDownloaded`、`status` 字段
- **依赖**: Task 6.1

### Task 7.2: 改造 AttachmentItem 组件
- **文件**: `src/components/email/AttachmentItem.vue`
- **内容**:
  - 新增三种状态展示（未下载/下载中/已下载）
  - 实现进度条显示
  - 监听 `attachment-progress` 事件
  - 监听 `attachment-complete` 事件
  - 离线时显示提示
  - 点击触发下载/保存
- **依赖**: Task 6.1, Task 6.3

### Task 7.3: 改造 AttachmentList 组件
- **文件**: `src/components/email/AttachmentList.vue`
- **内容**:
  - 传递附件状态信息
- **依赖**: Task 7.2

---

## 阶段 8：缓存管理 UI

### Task 8.1: 新增缓存管理设置组件
- **文件**: `src/components/settings/CacheSettings.vue` (新文件)
- **内容**:
  - 显示缓存统计信息
  - "打开缓存目录" 按钮
  - "清空缓存" 按钮 + 确认对话框
  - 清空缓存警告提示
- **依赖**: Task 6.2, Task 7.2

### Task 8.2: 集成到设置页面
- **文件**: `src/views/SettingsPage.vue`
- **内容**:
  - 添加缓存管理组件
- **依赖**: Task 8.1

---

## 阶段 9：测试与验证

### Task 9.1: 数据库迁移测试
- **内容**:
  - 测试存量数据迁移
  - 验证旧附件数据完整性
- **依赖**: Task 1.3

### Task 9.2: IMAP 同步测试
- **内容**:
  - 测试新邮件同步（只获取正文）
  - 验证附件元信息正确
  - 验证 is_downloaded=false
- **依赖**: Task 3.1

### Task 9.3: 附件下载测试
- **内容**:
  - 测试点击下载流程
  - 验证进度事件
  - 验证文件完整性
  - 测试离线场景
- **依赖**: Task 7.2, Task 4.2

### Task 9.4: 缓存管理测试
- **内容**:
  - 测试清空缓存
  - 验证数据库状态更新
  - 测试重新下载
- **依赖**: Task 8.1

---

## 任务依赖图

```
阶段 1 (数据库)
  Task 1.1 ──▶ Task 1.2 ──▶ Task 1.3

阶段 2 (IMAP)
  Task 2.4 ──▶ Task 2.1 ──▶ Task 2.2
                     │
                     └──▶ Task 2.3

阶段 3 (同步)
  Task 2.1, 2.2, 1.2 ──▶ Task 3.1
  Task 1.2 ──▶ Task 3.2
  Task 2.1 ──▶ Task 3.3

阶段 4 (下载命令)
  Task 4.1
  Task 2.3, 1.2, 4.1 ──▶ Task 4.2 ──▶ Task 4.3 ──▶ Task 4.4

阶段 5 (缓存管理)
  Task 4.1 ──▶ Task 5.1
  Task 4.1, 1.2 ──▶ Task 5.2
  Task 4.1 ──▶ Task 5.3

阶段 6 (前端服务)
  Task 4.2, 4.3 ──▶ Task 6.1
  Task 5.1, 5.2, 5.3 ──▶ Task 6.2
  (无依赖) ──▶ Task 6.3

阶段 7 (前端组件)
  Task 6.1 ──▶ Task 7.1
  Task 6.1, 6.3 ──▶ Task 7.2 ──▶ Task 7.3

阶段 8 (缓存 UI)
  Task 6.2, 7.2 ──▶ Task 8.1 ──▶ Task 8.2

阶段 9 (测试)
  Task 1.3 ──▶ Task 9.1
  Task 3.1 ──▶ Task 9.2
  Task 7.2, 4.2 ──▶ Task 9.3
  Task 8.1 ──▶ Task 9.4
```

## 建议实施顺序

1. **阶段 1 + 阶段 2** - 后端基础改造
2. **阶段 3** - 同步逻辑改造
3. **阶段 4** - 下载命令实现
4. **阶段 6 + 阶段 7** - 前端改造
5. **阶段 5 + 阶段 8** - 缓存管理
6. **阶段 9** - 测试验证