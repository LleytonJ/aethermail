## 1. 后端数据库方法

- [x] 1.1 在 database.rs 中添加 delete_email 方法，删除单封邮件
- [x] 1.2 测试数据库删除方法，验证 CASCADE 删除附件

## 2. 后端 Tauri 命令

- [x] 2.1 在 commands/email.rs 中添加 delete_email 命令
- [x] 2.2 在 lib.rs 中注册 delete_email 命令

## 3. 前端服务层

- [x] 3.1 在 emailService.ts 中添加 deleteEmail 函数
- [ ] 3.2 测试前端服务调用后端命令

## 4. 前端状态管理

- [x] 4.1 在 mailbox.ts store 中添加 deleteEmail action
- [x] 4.2 deleteEmail action 调用服务、更新 emails 数组、清空选中状态
- [x] 4.3 deleteEmail action 更新未读/星标计数（如果删除的邮件是未读/星标）

## 5. UI 绑定

- [x] 5.1 在 ReadingPane.vue 中绑定正常邮件删除按钮点击事件
- [x] 5.2 在 ReadingPane.vue 中绑定钓鱼邮件删除按钮点击事件
- [x] 5.3 使用 ConfirmDialog 组件实现删除确认对话框
- [x] 5.4 删除成功后显示通知

## 6. 测试验证

- [ ] 6.1 测试删除正常邮件功能
- [ ] 6.2 测试删除钓鱼邮件功能
- [ ] 6.3 测试删除当前选中的邮件后 UI 状态正确
- [ ] 6.4 测试删除未读邮件后计数更新
- [ ] 6.5 测试删除星标邮件后计数更新