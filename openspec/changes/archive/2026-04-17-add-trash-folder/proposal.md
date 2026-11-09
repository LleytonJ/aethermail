## Why

当前删除邮件直接从数据库移除（硬删除），用户无法恢复误删的邮件。需要实现回收站功能，让删除的邮件可以先进入回收站，30天后自动清理，用户也可以手动恢复到收件箱或钓鱼邮件箱。

## What Changes

- 数据库新增 `deleted_at` 字段，实现软删除机制
- 删除邮件改为设置 `deleted_at` 时间戳（移动到回收站）
- 新增回收站文件夹，显示已删除邮件
- 新增从回收站恢复邮件功能（恢复到收件箱或钓鱼邮件箱）
- 新增永久删除功能（从回收站彻底删除）
- 应用启动时自动清理 30 天前删除的邮件

## Capabilities

### New Capabilities
- `trash-folder`: 回收站文件夹功能，包括查询已删除邮件、恢复邮件、永久删除、自动清理

### Modified Capabilities
- `email-storage`: 新增 `deleted_at` 字段和相关查询方法
- `email-deletion`: 删除逻辑改为软删除（设置 `deleted_at`），新增永久删除方法
- `mailbox`: 新增回收站文件夹加载、恢复邮件、永久删除 action

## Impact

- 后端：`database.rs` - 新增 `deleted_at` 字段迁移、回收站查询/恢复/永久删除方法
- 后端：`commands/email.rs` - 新增 `move_to_trash`、`restore_email`、`permanent_delete`、`get_trash_emails`、`auto_cleanup_trash` 命令
- 后端：`lib.rs` - 注册新命令，启动时执行自动清理
- 前端：`Sidebar.vue` - 添加回收站文件夹项
- 前端：`mailbox.ts` - 新增回收站加载、恢复、永久删除 action
- 前端：`ReadingPane.vue` - 回收站邮件显示恢复/永久删除按钮
- 前端：`emailService.ts` - 新增回收站相关服务函数