## Why

当前用户只能通过侧边栏添加新账户，缺少账户管理入口。用户无法：
- 查看已配置账户的详细信息
- 编辑账户配置（显示名称、密码、服务器设置）
- 删除不再使用的账户

创建独立的设置页面是桌面应用的标准做法，为用户提供集中管理账户的入口。

## What Changes

- **新增** `/settings` 路由和 `SettingsPage.vue` 页面组件（参考 velo 布局）
- **新增** `AccountManagement.vue` 账户管理组件
- **新增** `ConfirmDialog.vue` 确认对话框组件
- **新增** `EditAccount.vue` 账户编辑 Modal（复用 AddPop3Account 组件）
- **修改** `Sidebar.vue` 设置按钮点击跳转到设置页面
- **修改** `accountStore` 添加 `updateAccount` 方法
- **修改** `router/index.ts` 添加 settings 路由
- **扩展** `accountService.ts` 添加 `updateMockAccount` 方法

## Capabilities

### New Capabilities

- `settings-page`: 设置页面能力 - 提供应用设置的统一入口，支持路由导航
- `account-editing`: 账户编辑能力 - 修改账户显示名称、服务器配置、密码
- `account-deletion`: 账户删除能力 - 删除账户并清理相关数据

### Modified Capabilities

（无现有 capability 需要修改）

## Impact

### 前端影响

| 文件/目录 | 变更类型 |
|----------|---------|
| `src/views/SettingsPage.vue` | 新增 |
| `src/components/settings/` | 新增目录 |
| `src/components/settings/AccountManagement.vue` | 新增 |
| `src/components/settings/EditAccount.vue` | 新增 |
| `src/components/ui/ConfirmDialog.vue` | 新增 |
| `src/router/index.ts` | 修改 |
| `src/components/layout/Sidebar.vue` | 修改 |
| `src/stores/account.ts` | 修改 |
| `src/services/mock/accountService.ts` | 修改 |

### 路由影响

```
/settings           → SettingsPage (重定向到 /settings/accounts)
/settings/accounts  → 账户管理页
/settings/general   → 通用设置页 (预留)
```

### 数据影响

- 删除账户时清理 localStorage 中的账户数据
- 编辑账户时更新 localStorage