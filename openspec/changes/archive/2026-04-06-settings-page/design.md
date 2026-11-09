## Context

AetherMail 是一个基于 Tauri v2 + Vue 3 的桌面邮件客户端，当前已实现账户添加功能，但缺少账户管理入口。用户通过侧边栏的 AccountSwitcher 组件可以切换账户和添加新账户，但无法查看账户详情、编辑配置或删除账户。

### 当前架构

```
src/
├── components/
│   ├── accounts/
│   │   ├── AccountSwitcher.vue    # 账户切换下拉菜单
│   │   ├── AddAccount.vue         # 添加账户 Modal 入口
│   │   └── AddPop3Account.vue     # POP3 配置表单（4步 wizard）
│   └── layout/
│       └── Sidebar.vue            # 侧边栏，包含设置按钮
├── stores/
│   └── account.ts                 # Pinia store，有 loadAccounts/setActiveAccount/removeAccount
├── services/mock/
│   └── accountService.ts          # Mock 服务，有 getMockAccounts/addMockAccount/removeMockAccount
└── router/
    └── index.ts                   # Vue Router 配置
```

### 约束

- 本次迭代仅实现前端 UI，使用 Mock 数据
- 复用现有组件（AddPop3Account），避免重复开发
- 保持与现有 UI 风格一致（Tailwind CSS v4 + Glass Panel 风格）
- 需要中文本地化

## Goals / Non-Goals

**Goals:**

- 创建独立的设置页面路由 `/settings`
- 实现账户列表展示（显示名称、邮箱、服务器信息）
- 实现账户删除功能（带确认对话框）
- 实现账户编辑功能（复用 AddPop3Account 作为编辑 Modal）
- 侧边栏设置按钮跳转到设置页面

**Non-Goals:**

- 不实现真实的 POP3 连接测试（使用 Mock）
- 不实现账户密码加密存储
- 不实现其他设置项（如通用设置、主题设置等）
- 不实现账户数据导出/导入

## Decisions

### 1. 设置页面布局方案

**Decision:** 采用左侧导航 + 右侧内容区布局（参考 velo 设计）

**Rationale:**
- 符合桌面应用设置页面的标准设计模式
- 与 velo 项目保持一致，便于参考实现
- 为未来扩展其他设置项预留空间（如通用设置、外观设置）
- 单一组件结构更简单，避免不必要的拆分

**Alternatives considered:**
- **单页全屏布局**：简单但不易扩展，不适合多设置项场景
- **Tab 切换布局**：适合少量设置项，但对于桌面应用不够专业
- **拆分 SettingsLayout.vue**：增加不必要的组件层级

**Structure (参考 velo):**
```
┌──────────────────────────────────────────────────────┐
│  Header: [← 返回] 设置                                │
├────────────┬─────────────────────────────────────────┤
│  Nav (w-48)│  Content (max-w-2xl)                    │
│  192px     │  最大宽度限制，提升阅读体验               │
│            │                                         │
│  ○ 账户管理 │  [Tab Title]                            │
│  ○ 通用设置 │  ─────────────────────────────          │
│  (预留)    │  Setting rows...                        │
│            │                                         │
└────────────┴─────────────────────────────────────────┘
```

**Layout 参数 (参考 velo SettingsPage.tsx):**
- 导航宽度: `w-48` (192px)
- 内容最大宽度: `max-w-2xl` (672px)
- Header: 包含返回按钮 + 标题
- 导航项样式: 激活时 `bg-bg-selected text-accent font-medium`

### 2. 账户编辑方案

**Decision:** 复用 AddPop3Account 组件作为编辑 Modal

**Rationale:**
- AddPop3Account 已实现完整的表单逻辑（验证、预设、测试）
- 编辑与新增的表单字段完全相同
- 通过传入 `mode: 'edit'` 和 `accountId` 参数控制行为差异
- 避免代码重复，减少维护成本

**Alternatives considered:**
- **创建独立的 EditAccount.vue**：会导致大量重复代码
- **在 AccountManagement 内嵌表单**：不够模块化，难以复用

**Implementation:**
- 修改 AddPop3Account 接受 `mode` 和 `accountId` props
- 编辑模式下，初始化表单时加载现有账户数据
- 编辑模式下，跳过"测试连接"步骤或改为可选

### 3. 删除确认方案

**Decision:** 创建通用 ConfirmDialog 组件

**Rationale:**
- 确认对话框是常见交互模式，可复用于其他场景
- 提供清晰的删除确认流程，防止误操作
- 显示删除后果提示（如"相关邮件数据将被清除"）

**Alternatives considered:**
- **浏览器原生 confirm()**：不够美观，无法自定义样式
- **Toast/Snackbar**：不适合需要用户确认的破坏性操作

### 4. 路由结构

**Decision:** 使用嵌套路由结构

```
/settings           → 重定向到 /settings/accounts
/settings/accounts  → AccountManagement 组件
/settings/general   → 预留（占位组件）
```

**Rationale:**
- 嵌套路由支持左侧导航切换
- 默认重定向确保用户访问 `/settings` 时有明确内容

## Risks / Trade-offs

### Risk 1: 编辑时密码显示

**Risk:** 编辑账户时无法显示已保存的密码（Mock 数据未加密存储）

**Mitigation:**
- 编辑表单密码字段显示为空，提示"不修改请留空"
- 如需修改密码，用户需要重新输入

### Risk 2: 删除账户时的数据清理

**Risk:** 删除账户后，相关邮件数据如何处理？

**Mitigation:**
- 确认对话框中提示"此操作将删除账户配置，邮件数据不会被清除"
- 当前 Mock 实现仅删除账户配置，不涉及邮件数据
- 未来真实实现时需设计邮件数据清理策略

### Risk 3: 单账户删除保护

**Risk:** 删除最后一个账户后，应用无账户可用

**Mitigation:**
- 当只有一个账户时，禁用删除按钮或显示警告
- 确认对话框中特殊提示"这是最后一个账户"

### Trade-off: 编辑流程简化

**Trade-off:** 编辑模式下简化测试流程（可选而非必须）

**Reason:**
- 用户可能只想修改显示名称，无需重新测试连接
- 强制测试会增加用户负担
- 但跳过测试可能导致配置错误

**Resolution:**
- 编辑模式下测试按钮改为可选
- 保存前提示"建议测试连接后再保存"