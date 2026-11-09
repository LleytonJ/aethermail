## 1. 基础设置

- [x] 1.1 创建 `src/views/` 目录结构
- [x] 1.2 创建 `src/components/settings/` 目录结构

## 2. 路由配置

- [x] 2.1 在 `src/router/index.ts` 添加 `/settings` 嵌套路由
- [x] 2.2 配置 `/settings` 重定向到 `/settings/accounts`
- [x] 2.3 配置 `/settings/accounts` 和 `/settings/general` 子路由

## 3. 设置页面组件（参考 velo）

- [x] 3.1 创建 `SettingsPage.vue` 页面组件（包含 Header + 导航 + 内容区）
- [x] 3.2 实现 Header（返回按钮 + "设置" 标题）
- [x] 3.3 实现左侧导航菜单（w-48 固定宽度，账户管理、通用设置预留）
- [x] 3.4 实现右侧内容区（max-w-2xl 最大宽度限制）
- [x] 3.5 实现导航切换逻辑（根据路由参数渲染对应内容）

## 4. 确认对话框组件

- [x] 4.1 创建 `ConfirmDialog.vue` 通用确认对话框
- [x] 4.2 实现确认对话框的 props（title, message, confirmText, cancelText）
- [x] 4.3 实现确认/取消按钮事件处理

## 5. 账户管理组件

- [x] 5.1 创建 `AccountManagement.vue` 账户管理组件
- [x] 5.2 实现账户列表展示（卡片样式）
- [x] 5.3 每个账户卡片显示：头像、名称、邮箱、服务器信息
- [x] 5.4 为每个账户添加编辑和删除按钮
- [x] 5.5 实现单账户删除保护（最后账户禁用删除按钮）

## 6. 删除账户功能

- [x] 6.1 点击删除按钮显示确认对话框
- [x] 6.2 确认对话框显示删除账户邮箱和警告信息
- [x] 6.3 确认后调用 accountStore.removeAccount 删除账户
- [x] 6.4 删除成功显示通知提示
- [x] 6.5 处理删除活跃账户后的自动切换

## 7. 账户编辑功能

- [x] 7.1 创建 `EditAccount.vue` 编辑账户 Modal 组件
- [x] 7.2 修改 `AddPop3Account.vue` 支持 `mode` prop（add/edit）
- [x] 7.3 编辑模式下预填充现有账户数据
- [x] 7.4 编辑模式下密码字段显示为空并添加提示
- [x] 7.5 实现账户数据更新保存逻辑

## 8. Store 和 Service 扩展

- [x] 8.1 在 `accountStore.ts` 添加 `updateAccount` 方法
- [x] 8.2 在 `accountService.ts` 添加 `updateMockAccount` 方法
- [x] 8.3 实现 localStorage 更新持久化

## 9. 侧边栏集成

- [x] 9.1 在 `Sidebar.vue` 添加设置按钮
- [x] 9.2 设置按钮点击跳转到 `/settings/accounts`

## 10. UI 样式和本地化

- [x] 10.1 应用 Tailwind CSS 样式，保持与现有 UI 一致
- [x] 10.2 所有文案使用中文
- [x] 10.3 实现过渡动画效果（Modal、确认对话框）