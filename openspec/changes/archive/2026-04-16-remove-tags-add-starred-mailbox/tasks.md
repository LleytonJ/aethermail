## 1. 移除标签模块

- [x] 1.1 删除 `aethermail/src/components/layout/TagList.vue` 文件
- [x] 1.2 从 `Sidebar.vue` 移除 `import TagList` 语句（第 99 行）
- [x] 1.3 从 `Sidebar.vue` 移除标签分组 UI（第 47-55 行）
- [x] 1.4 从 `Sidebar.vue` 移除 `tags` 数组定义（第 190-194 行）

## 2. 后端 - 数据库查询方法

- [x] 2.1 在 `database.rs` 添加 `get_starred_emails()` 方法
- [x] 2.2 在 `database.rs` 添加 `get_starred_count()` 方法

## 3. 后端 - Tauri 命令

- [x] 3.1 在 `email.rs` 添加 `get_starred_emails` 命令
- [x] 3.2 在 `email.rs` 添加 `get_starred_count` 命令
- [x] 3.3 在 `lib.rs` 注册新命令

## 4. 前端 - 服务层

- [x] 4.1 在 `emailService.ts` 添加 `getStarredEmails()` 函数
- [x] 4.2 在 `emailService.ts` 添加 `getStarredCount()` 函数

## 5. 前端 - Store

- [x] 5.1 在 `mailbox.ts` loadEmails 函数添加 `starred` 分支处理
- [x] 5.2 在 `mailbox.ts` 添加 `starredCount` ref 状态

## 6. 前端 - Sidebar UI

- [x] 6.1 在 `Sidebar.vue` 导入 Star 图标
- [x] 6.2 在 `folders` computed 添加星标文件夹项
- [x] 6.3 在 `Sidebar.vue` 添加 `starredCount` ref
- [x] 6.4 在 `Sidebar.vue` 添加 `loadStarredCount()` 函数
- [x] 6.5 在 `Sidebar.vue` 添加 `loadStarredEmails()` 函数
- [x] 6.6 在 `handleFolderClick` 添加 starred 分支处理
- [x] 6.7 在账户变化 watch 中添加星标数量加载

## 7. 验证与测试

- [x] 7.1 编译前后端代码，确认无错误
- [ ] 7.2 运行应用，确认标签模块已移除
- [ ] 7.3 测试星标邮件切换功能正常
- [ ] 7.4 测试星标邮件箱显示正确的星标邮件列表
- [ ] 7.5 测试取消星标后邮件从星标邮件箱消失
- [ ] 7.6 测试星标数量实时更新