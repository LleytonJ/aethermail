## ADDED Requirements

### Requirement: Pinia Store 能正常配置
系统 SHALL 配置 Pinia 状态管理框架。

#### Scenario: Store 初始化
- **WHEN** 应用启动
- **THEN** Pinia store 正确注册并可访问

#### Scenario: 状态持久化
- **WHEN** 用户更改主题设置
- **THEN** 设置保存到 localStorage，刷新后保持

### Requirement: accountStore 能管理账户状态
系统 SHALL 实现账户状态存储。

#### Scenario: 设置账户列表
- **WHEN** 从后端加载账户数据
- **THEN** 调用 setAccounts 更新状态

#### Scenario: 切换活跃账户
- **WHEN** 用户点击账户切换
- **THEN** 调用 setActiveAccount，UI 更新显示新账户

### Requirement: mailboxStore 能管理邮箱状态
系统 SHALL 实现邮箱状态存储。

#### Scenario: 加载邮件列表
- **WHEN** 调用 loadEmails(accountId)
- **THEN** 状态更新，emails 数组包含邮件数据

#### Scenario: 选择邮件
- **WHEN** 用户点击邮件项
- **THEN** selectedEmailId 更新，触发阅读面板加载

### Requirement: uiStore 能管理 UI 设置
系统 SHALL 实现 UI 状态存储。

#### Scenario: 折叠侧边栏
- **WHEN** 调用 setSidebarCollapsed(true)
- **THEN** sidebarCollapsed 变为 true，侧边栏收起

#### Scenario: 切换主题
- **WHEN** 调用 setTheme('dark')
- **THEN** HTML class 更新为 dark，主题切换

### Requirement: Store 间接访问后端数据
系统 SHALL 通过 Tauri IPC 与后端通信。

#### Scenario: 调用后端命令
- **WHEN** Store action 调用 invoke()
- **THEN** 正确发送 IPC 请求到 Rust 后端

#### Scenario: 错误处理
- **WHEN** 后端返回错误
- **THEN** Store 捕获错误并更新 error 状态
