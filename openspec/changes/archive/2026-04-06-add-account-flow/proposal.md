## Why

当前 AetherMail 侧边栏显示的账户信息是硬编码在组件中的，用户无法：
- 看到账户切换下拉菜单
- 触发"添加账户"流程
- 体验完整的账户配置表单交互

本次迭代专注于账户管理的前端 UI 和交互体验，使用 Mock 数据模拟后端响应，为后续接入真实后端奠定基础。

## What Changes

- **新增** `Modal.vue` - 基础弹窗组件
- **新增** `AccountSwitcher.vue` - 侧边栏账户切换器，支持下拉菜单和添加入口
- **新增** `AddAccount.vue` - 账号添加入口 Modal，选择邮箱服务商类型
- **新增** `AddPop3Account.vue` - POP3 账号配置多步骤表单（基本信息 → 服务器配置 → 测试连接 → 保存）
- **新增** `providerPresets.ts` - 常见邮箱服务商预设配置（Mock 数据）
- **修改** `Sidebar.vue` - 集成 AccountSwitcher 组件
- **修改** `accountStore` - 扩展支持多账户状态管理
- **修改** `accountService.ts` - 扩展 Mock 服务支持添加账户

## Capabilities

### New Capabilities

- `account-ui`: 账户管理 UI - 账户切换器、添加账户表单、多步骤向导交互

### Modified Capabilities

（无现有 capability 需要修改）

## Impact

### 前端影响

| 文件/目录 | 变更类型 |
|----------|---------|
| `src/components/ui/Modal.vue` | 新增 |
| `src/components/accounts/` | 新增目录 |
| `src/components/accounts/AccountSwitcher.vue` | 新增 |
| `src/components/accounts/AddAccount.vue` | 新增 |
| `src/components/accounts/AddPop3Account.vue` | 新增 |
| `src/components/layout/Sidebar.vue` | 修改 |
| `src/stores/account.ts` | 修改 |
| `src/services/mock/accountService.ts` | 修改 |
| `src/services/email/providerPresets.ts` | 新增 |

### 后端影响

**无后端变更** - 本次迭代仅前端 UI，使用 Mock 数据。

### 数据库影响

**无数据库变更** - 数据仅存储在前端状态和 localStorage。

### 依赖关系

- 纯前端实现，无新增外部依赖