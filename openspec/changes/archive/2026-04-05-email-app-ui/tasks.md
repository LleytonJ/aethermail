# AetherMail UI 开发任务列表

> 变更：email-app-ui | 目标：项目框架搭建 + UI 设计实现

---

## 1. 项目初始化

- [x] 1.1 运行 `npm create tauri-app@latest` 创建项目，选择 Vue + TypeScript 模板
- [x] 1.2 配置前端依赖（Vue 3, Pinia, Vue Router, Tailwind CSS v4, Lucide Vue）
- [x] 1.3 配置 Rust 依赖（Tauri v2, SQL, Dialog, Notification 插件）
- [x] 1.4 创建目录结构（components/layout, components/ui, stores, services）
- [x] 1.5 验证 `npm run tauri dev` 正常运行

## 2. 设计系统实现

- [x] 2.1 创建 `src/styles/globals.css`，定义深色主题颜色令牌
- [x] 2.2 配置 Tailwind CSS v4，启用自定义主题
- [x] 2.3 创建基础组件 Button.vue（primary, secondary, ghost 变体）
- [x] 2.4 创建基础组件 Input.vue（text, password 类型）
- [x] 2.5 创建基础组件 Skeleton.vue（带动画占位）
- [x] 2.6 配置 Lucide Vue 图标库，创建图标导出模块

## 3. 布局组件实现

- [x] 3.1 实现 MailLayout.vue（三栏 Flexbox 布局）
- [x] 3.2 实现 TitleBar.vue（自定义标题栏，窗口控制按钮）
- [x] 3.3 实现 Sidebar.vue（侧边导航容器）
- [x] 3.4 实现 EmailList.vue（邮件列表容器）
- [x] 3.5 实现 ReadingPane.vue（阅读面板容器）
- [x] 3.6 验证布局响应式（最小窗口 800x600）

## 4. 侧边栏组件实现

- [x] 4.1 实现 AccountSwitcher.vue（账户切换，头像显示）
- [x] 4.2 实现 ComposeButton.vue（撰写邮件按钮）
- [x] 4.3 实现 FolderList.vue（文件夹列表：收件箱/已发送/草稿箱/已删除）
- [x] 4.4 实现 FolderItem.vue（文件夹项，支持未读数角标）
- [x] 4.5 实现 TagList.vue（标签列表：工作/个人/紧急）
- [x] 4.6 实现 Sidebar 折叠功能

## 5. 邮件列表组件实现

- [x] 5.1 实现 EmailListItem.vue（邮件项组件）
- [x] 5.2 实现邮件项头部（发件人头像、姓名、时间）
- [x] 5.3 实现邮件项内容（主题、摘要、附件图标）
- [x] 5.4 实现已读/未读样式区分
- [x] 5.5 实现选中高亮效果
- [x] 5.6 实现星标功能按钮

## 6. 阅读面板组件实现

- [x] 6.1 实现 EmailHeader.vue（邮件头部信息）
- [x] 6.2 实现 EmailBody.vue（邮件正文区域）
- [x] 6.3 实现 EmailRenderer.vue（HTML 渲染，iframe 沙盒）
- [x] 6.4 实现 AttachmentList.vue（附件列表）
- [x] 6.5 实现 AttachmentItem.vue（附件项，图标 + 文件名 + 大小）
- [x] 6.6 实现邮件操作栏（回复/转发/删除按钮）

## 7. Pinia Store 实现

- [x] 7.1 创建 uiStore（主题、侧边栏状态、密度配置）
- [x] 7.2 创建 accountStore（账户列表、活跃账户）
- [x] 7.3 创建 mailboxStore（邮件列表、选中邮件、加载状态）
- [x] 7.4 实现 Store 状态持久化（localStorage）
- [x] 7.5 实现 Tauri IPC 调用封装

## 8. Mock 数据服务

- [x] 8.1 创建 mock/emailData.ts（模拟邮件数据）
- [x] 8.2 创建 mock/accountData.ts（模拟账户数据）
- [x] 8.3 实现 emailService.ts（ getEmails, getEmailById）
- [x] 8.4 实现 accountService.ts（getAccounts, setActiveAccount）
- [x] 8.5 在 Store 中集成 Mock 服务

## 9. 交互功能实现

- [x] 9.1 实现邮件列表点击选中
- [x] 9.2 实现文件夹切换（UI 状态切换）
- [x] 9.3 实现侧边栏折叠/展开
- [x] 9.4 实现加载状态（Skeleton 显示）
- [x] 9.5 实现空状态（EmptyState 组件）

## 10. 测试与优化

- [x] 10.1 验证所有组件单独渲染正常
- [x] 10.2 验证端到端交互流程（点击邮件→显示详情）
- [x] 10.3 验证窗口缩放布局正常
- [x] 10.4 验证深色主题一致性
- [x] 10.5 性能优化（组件懒加载、虚拟滚动预留）
- [x] 10.6 清理代码，添加必要注释

---

## 任务统计

| 阶段 | 任务数 |
|-----|--------|
| 1. 项目初始化 | 5 |
| 2. 设计系统实现 | 6 |
| 3. 布局组件实现 | 6 |
| 4. 侧边栏组件实现 | 6 |
| 5. 邮件列表组件实现 | 6 |
| 6. 阅读面板组件实现 | 6 |
| 7. Pinia Store 实现 | 5 |
| 8. Mock 数据服务 | 5 |
| 9. 交互功能实现 | 5 |
| 10. 测试与优化 | 6 |
| **总计** | **50** |

---

**预计工期：** 5-7 天
**开发人员：** 1 人
**开发模式：** 全职（8 小时/天）
