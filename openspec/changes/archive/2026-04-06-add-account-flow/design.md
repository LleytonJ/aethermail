## Context

AetherMail 当前处于 MVP 开发阶段，侧边栏的账户显示是硬编码的，缺少账户切换和添加的交互流程。

本次迭代目标：**仅实现前端 UI 和交互**，使用 Mock 数据模拟后端响应，暂不涉及：
- Rust 后端 POP3 连接实现
- SQLite 数据库操作
- 密码加密存储

**技术约束：**
- 前端：Vue 3 + TypeScript + Tailwind CSS v4
- 数据：Pinia Store + localStorage 持久化
- Mock：模拟测试连接的延迟和结果

## Goals / Non-Goals

**Goals:**
- 实现账户切换器 UI（下拉菜单、账户列表、添加入口）
- 实现账号添加 Modal 入口（服务商选择）
- 实现 POP3 账号配置多步骤表单（4 步向导）
- 实现邮箱服务商预设配置（Gmail/163/QQ/Outlook）
- 实现 Mock 测试连接功能（模拟成功/失败场景）
- 实现 Mock 账户保存功能（存入 Store + localStorage）
- 实现账户切换功能

**Non-Goals:**
- 真实 POP3 连接测试（后端 Rust 实现）
- 数据库持久化（SQLite）
- 密码加密存储
- SMTP 配置（v0.2）
- 账户编辑功能（v0.2）
- 账户删除功能（v0.2）

## Decisions

### 1. 表单流程设计

**决定：采用 4 步骤向导式表单**

```
Step 1: 基本信息        Step 2: POP3 配置       Step 3: 测试连接        Step 4: 保存完成
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│ 邮箱地址        │    │ 服务器地址      │    │ [测试按钮]      │    │ ✓ 账号已添加   │
│ 显示名称        │    │ 端口            │    │ POP3: ✓ 成功    │    │ [前往收件箱]   │
│ 密码            │    │ SSL/TLS 开关    │    │ (Mock模拟)      │    │ [添加更多]     │
└─────────────────┘    └─────────────────┘    └─────────────────┘    └─────────────────┘
```

**选择理由：**
- 分步引导降低用户认知负担
- 测试步骤独立，用户确认成功后才保存
- 后续接入真实后端时仅需替换 Mock 服务

### 2. 邮箱服务商预设

**决定：基于邮箱域名自动填充服务器配置**

```typescript
const PROVIDER_PRESETS: Record<string, Pop3Preset> = {
  'gmail.com':   { host: 'pop.gmail.com',         port: 995, security: 'ssl' },
  '163.com':     { host: 'pop.163.com',           port: 110, security: 'none' },
  '126.com':     { host: 'pop.126.com',           port: 110, security: 'none' },
  'qq.com':      { host: 'pop.qq.com',            port: 110, security: 'none' },
  'outlook.com': { host: 'pop-mail.outlook.com',  port: 995, security: 'ssl' },
}
```

**选择理由：**
- 减少用户手动配置
- 支持自定义服务器（手动修改）

### 3. Mock 测试连接实现

**决定：使用 setTimeout 模拟异步测试**

```typescript
// Mock 测试连接服务
async function mockTestConnection(config: Pop3Config): Promise<Pop3TestResult> {
  await delay(1500) // 模拟网络延迟
  
  // 模拟失败场景：密码包含 "wrong"
  if (config.password.toLowerCase().includes('wrong')) {
    return { success: false, message: 'Authentication failed: Invalid credentials' }
  }
  
  // 默认成功
  return { 
    success: true, 
    message: 'Connection successful',
    serverInfo: `${config.host} ready`
  }
}
```

**选择理由：**
- 可演示完整的测试流程
- 支持成功/失败两种场景演示
- 后续替换为真实后端调用即可

### 4. 数据持久化方案

**决定：Pinia Store + localStorage**

```typescript
// accountStore 扩展
const useAccountStore = defineStore('account', () => {
  const accounts = ref<Account[]>([])
  
  // 初始化时从 localStorage 加载
  function init() {
    const saved = localStorage.getItem('aethermail_accounts')
    if (saved) accounts.value = JSON.parse(saved)
  }
  
  // 添加账户后保存
  function addAccount(account: Account) {
    accounts.value.push(account)
    localStorage.setItem('aethermail_accounts', JSON.stringify(accounts.value))
  }
})
```

**选择理由：**
- MVP 阶段简单可靠
- 后续迁移到 SQLite 时接口不变
- 无需后端支持

### 5. 组件架构

```
App.vue
  └── MailLayout.vue
        ├── Sidebar.vue
        │     └── AccountSwitcher.vue  ← 触发添加
        │
        └── Teleport to body
              └── AddAccount.vue        ← Modal 入口
                    └── AddPop3Account.vue  ← 多步骤表单
```

**状态管理：**
- `showAddAccount: boolean` - 由 MailLayout 管理
- `currentStep: number` - 由 AddPop3Account 内部管理
- 表单数据 - 局部状态，提交时写入 accountStore

## Risks / Trade-offs

### 风险 1: Mock 数据与真实数据结构不一致
**风险：** 后端实现时发现数据结构需要调整
**缓解：** 参考设计文档中的数据库表结构设计 TypeScript 接口

### 风险 2: 用户体验断层
**风险：** Mock 测试总是成功，用户可能配置错误服务器
**缓解：** 明确提示"演示模式"，v0.2 接入真实测试

### 风险 3: localStorage 容量限制
**风险：** 存储大量邮件数据时可能超出限制
**缓解：** 本次仅存储账户配置，邮件数据后续使用 SQLite

## Migration Plan

### 实现步骤
1. 创建 Modal 基础组件
2. 实现 AccountSwitcher 组件
3. 实现 AddAccount 入口 Modal
4. 实现 AddPop3Account 多步骤表单
5. 集成到 Sidebar
6. 扩展 accountStore 和 Mock 服务

### 后续迁移（接入真实后端）
- 替换 `mockTestConnection` 为 `invoke('pop3_test_connection')`
- 替换 localStorage 为 SQLite 操作
- 添加密码加密逻辑