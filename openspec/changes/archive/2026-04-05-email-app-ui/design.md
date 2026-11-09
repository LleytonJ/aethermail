## Context

本项目参考 Velo 邮箱项目（https://github.com/avihaymenahem/velo）的架构设计，采用 Tauri v2 + Vue 3 的技术组合。当前处于项目初始阶段，需要搭建基础框架和 UI 系统，为后续 POP3/SMTP 功能开发做准备。

**约束条件：**
- 前端优先：先完成纯前端 UI 原型，后端接口后续实现
- 深色主题：默认采用深色模式设计
- 现代风格：参考 AetherMail 设计风格，简洁现代
- 可交互：UI 原型需支持基本交互，用于需求确认

## Goals / Non-Goals

**Goals:**
- 创建可运行的 Tauri + Vue 3 项目框架
- 实现完整的三栏布局 UI 系统
- 创建深色主题设计系统和基础组件库
- 配置 Pinia 状态管理框架
- 实现可交互的原型（模拟数据）

**Non-Goals:**
- 不包含真实的 POP3/SMTP 功能（后续版本）
- 不包含后端数据库实现
- 不包含邮件发送功能
- 不包含多账户切换（数据结构支持，UI 暂不实现）

## Decisions

### 1. 项目结构组织

**决策：** 采用功能导向的目录结构

```
src/
├── components/
│   ├── layout/       # 布局组件（MailLayout, Sidebar, EmailList, ReadingPane）
│   ├── email/        # 邮件相关组件（MessageItem, EmailRenderer, AttachmentList）
│   ├── accounts/     # 账户组件（AddAccount, AccountList）
│   ├── ui/           # 基础 UI 组件（Button, Input, Modal, Skeleton）
│   └── settings/     # 设置组件
├── stores/           # Pinia Store（account, mailbox, ui）
├── services/         # 服务层（API 调用、工具函数）
├── styles/           # 全局样式
└── router/           # 路由配置
```

**理由：** 功能导向结构便于团队协作，与 Velo 项目结构保持一致，降低后续开发认知负担。

**替代方案：**
- 按文件类型组织（components/views/store）- 不适合大型项目
- 按领域组织（email/accounts/settings）- 与功能导向类似，但粒度更粗

### 2. 状态管理策略

**决策：** 使用 Pinia，Store 分为三层：
- UI 状态：uiStore（主题、布局、密度）
- 业务状态：accountStore, mailboxStore
- 数据服务：通过 Tauri IPC 调用后端

**理由：** Pinia 是 Vue 官方推荐，支持 TypeScript，API 简洁，无 middleware 复杂度。

**替代方案：**
- Vuex - 已被 Vue 官方弃用
- Signals/Reactivity API - 生态不够成熟

### 3. 样式方案

**决策：** Tailwind CSS v4 + CSS 变量令牌系统

```css
:root {
  --bg-primary: #1e1e2e;
  --bg-secondary: #252537;
  --text-primary: #ffffff;
  --accent: #7c3aed;
}
```

**理由：** Tailwind v4 简化配置，CSS 变量支持主题切换，原子化 CSS 提高开发效率。

**替代方案：**
- SCSS 变量 - 不支持运行时主题切换
- CSS Modules - 学习曲线陡峭
- Styled Components - 运行时开销大

### 4. 布局方案

**决策：** Flexbox 三栏布局，固定 + 弹性宽度

```
Sidebar (240px fixed) | EmailList (350px fixed) | ReadingPane (flex-1)
```

**理由：** 经典邮件客户端布局，用户认知成本低，实现简单，响应式友好。

**替代方案：**
- Grid 布局 - 过度复杂
- 响应式可调节面板 - 需要额外的拖拽库，增加复杂度

### 5. 原型数据策略

**决策：** 前端 Mock 数据，通过 service 层模拟后端 API

```typescript
// services/mock/emailService.ts
export async function getEmails(): Promise<Email[]> {
  return MOCK_EMAILS;
}
```

**理由：** 前后端并行开发，UI 开发不依赖后端进度，后续替换为真实 API 容易。

**替代方案：**
- 等待后端完成后开发 UI - 延长开发周期
- 使用 MSW 等 Mock 框架 - 过度设计

## Risks / Trade-offs

**风险 1：Mock 数据与真实 API 不一致**
→ 缓解：定义清晰的 TypeScript 接口，Mock 数据严格遵守接口

**风险 2：Tailwind CSS v4 生态不成熟**
→ 缓解：v4 向下兼容 v3，大部分插件可用，必要时降级到 v3

**风险 3：深色主题配色设计难度高**
→ 缓解：参考 Velo 项目的颜色方案，基于现有成功设计调整

**风险 4：过度设计 UI 组件库**
→ 缓解：聚焦 MVP 需求，仅实现必要组件（Button, Input, Skeleton）

**权衡：**
- 选择 Flexbox 而非 Grid：牺牲灵活性换取简单性
- 前端 Mock 而非真实 API：增加后期集成工作量，但加速 UI 开发
- 固定宽度布局：牺牲部分响应式能力，保持邮件客户端经典体验
