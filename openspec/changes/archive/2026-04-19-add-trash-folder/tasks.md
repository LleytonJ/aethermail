## 1. 数据库迁移

- [ ] 1.1 在 database.rs 中添加 deleted_at 字段迁移
- [ ] 1.2 在 database.rs 中添加 deleted_at 索引

## 2. 数据库方法

- [ ] 2.1 在 database.rs 中添加 move_to_trash 方法（软删除）
- [ ] 2.2 在 database.rs 中添加 get_trash_emails 方法
- [ ] 2.3 在 database.rs 中添加 get_trash_count 方法
- [ ] 2.4 在 database.rs 中添加 restore_email 方法（恢复邮件）
- [ ] 2.5 在 database.rs 中添加 permanent_delete 方法（永久删除）
- [ ] 2.6 在 database.rs 中添加 cleanup_expired_trash 方法（自动清理）

## 3. 后端 Tauri 命令

- [ ] 3.1 在 commands/email.rs 中添加 move_to_trash 命令
- [ ] 3.2 在 commands/email.rs 中添加 get_trash_emails 命令
- [ ] 3.3 在 commands/email.rs 中添加 get_trash_count 命令
- [ ] 3.4 在 commands/email.rs 中添加 restore_email 命令
- [ ] 3.5 在 commands/email.rs 中添加 permanent_delete 命令
- [ ] 3.6 在 lib.rs 中注册新命令
- [ ] 3.7 在 lib.rs setup 中添加自动清理调用

## 4. 前端服务层

- [ ] 4.1 在 emailService.ts 中添加 moveToTrash 函数
- [ ] 4.2 在 emailService.ts 中添加 getTrashEmails 函数
- [ ] 4.3 在 emailService.ts 中添加 getTrashCount 函数
- [ ] 4.4 在 emailService.ts 中添加 restoreEmail 函数
- [ ] 4.5 在 emailService.ts 中添加 permanentDelete 函数

## 5. 前端状态管理

- [ ] 5.1 在 mailbox.ts 中添加 moveToTrash action
- [ ] 5.2 在 mailbox.ts 中添加 restoreEmail action
- [ ] 5.3 在 mailbox.ts 中添加 permanentDelete action

## 6. Sidebar UI

- [ ] 6.1 在 Sidebar.vue 中取消回收站文件夹注释
- [ ] 6.2 在 Sidebar.vue 中添加回收站点击处理逻辑
- [ ] 6.3 在 Sidebar.vue 中添加 loadTrashEmails 函数

## 7. ReadingPane UI

- [ ] 7.1 在 ReadingPane.vue 中添加回收站邮件判断逻辑
- [ ] 7.2 在 ReadingPane.vue 中添加恢复按钮（恢复到收件箱/钓鱼邮件箱）
- [ ] 7.3 在 ReadingPane.vue 中添加永久删除按钮
- [ ] 7.4 实现恢复邮件的确认对话框
- [ ] 7.5 实现永久删除的确认对话框

## 8. 测试验证

- [ ] 8.1 测试删除邮件移动到回收站
- [ ] 8.2 测试回收站邮件列表显示
- [ ] 8.3 测试恢复邮件到收件箱
- [ ] 8.4 测试恢复邮件到钓鱼邮件箱
- [ ] 8.5 测试永久删除邮件
- [ ] 8.6 测试应用启动时自动清理过期邮件