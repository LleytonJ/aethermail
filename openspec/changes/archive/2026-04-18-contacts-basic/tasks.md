## 1. 后端数据层

- [x] 1.1 在 `storage/models.rs` 添加 `DbContact` 结构体
- [x] 1.2 在 `storage/database.rs` 添加 contacts 表迁移（init_tables 或 run_migrations）
- [x] 1.3 在 `storage/database.rs` 实现 `upsert_contact` 方法（邮件保存时调用）
- [x] 1.4 在 `storage/database.rs` 实现 `search_contacts` 方法（自动补全搜索）
- [x] 1.5 在 `storage/database.rs` 实现 `get_all_contacts` 方法
- [x] 1.6 在 `storage/database.rs` 实现 `get_contact_by_email` 方法
- [x] 1.7 在 `storage/database.rs` 实现 `update_contact` 方法
- [x] 1.8 在 `storage/database.rs` 实现 `delete_contact` 方法
- [x] 1.9 修改 `save_email` 方法，在保存邮件后调用 `upsert_contact`

## 2. 后端命令层

- [x] 2.1 创建 `commands/contact.rs` 模块文件
- [x] 2.2 实现 `search_contacts` Tauri 命令
- [x] 2.3 实现 `get_all_contacts` Tauri 命令
- [x] 2.4 实现 `get_contact_by_email` Tauri 命令
- [x] 2.5 实现 `update_contact` Tauri 命令
- [x] 2.6 实现 `delete_contact` Tauri 命令
- [x] 2.7 在 `commands/mod.rs` 导出 contact 模块
- [x] 2.8 在 `lib.rs` 注册 contact 命令到 Tauri

## 3. 前端状态管理

- [x] 3.1 创建 `stores/contacts.ts` Pinia store
- [x] 3.2 实现 `loadContacts` action（加载所有联系人）
- [x] 3.3 实现 `searchContacts` action（搜索联系人）
- [x] 3.4 实现 `updateContact` action（更新联系人）
- [x] 3.5 实现 `deleteContact` action（删除联系人）

## 4. 前端自动补全组件

- [x] 4.1 创建 `components/ui/AddressInput.vue` 组件
- [x] 4.2 实现输入框和多地址标签显示
- [x] 4.3 实现搜索触发逻辑（≥2字符，200ms debounce）
- [x] 4.4 实现补全下拉列表显示
- [x] 4.5 实现键盘导航（↑↓ Enter Tab Escape Backspace）
- [x] 4.6 实现选择补全建议添加地址
- [x] 4.7 实现删除已添加地址功能

## 5. 前端联系人编辑组件

- [x] 5.1 创建 `components/settings/ContactEditor.vue` 组件
- [x] 5.2 实现联系人列表显示（按频率排序）
- [x] 5.3 实现搜索筛选功能
- [x] 5.4 实现编辑模式切换（点击编辑按钮）
- [x] 5.5 实现显示名称编辑保存
- [x] 5.6 实现删除联系人功能（带确认）
- [x] 5.7 在 `SettingsPage.vue` 中集成 ContactEditor 组件

## 6. 集成与测试

- [x] 6.1 在 `ComposeEmail.vue` 中替换收件人输入为 AddressInput
- [x] 6.2 在 `ComposeEmail.vue` 中替换抄送/密送输入为 AddressInput
- [ ] 6.3 测试邮件同步时联系人自动创建
- [ ] 6.4 测试联系人自动补全功能
- [ ] 6.5 测试联系人编辑功能
- [ ] 6.6 测试联系人删除功能