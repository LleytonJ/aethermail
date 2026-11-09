## Why

用户需要一个现代化、轻量级的桌面邮箱客户端，现有方案要么过于臃肿（Outlook），要么界面老旧（Foxmail）。本变更旨在完成项目框架搭建和 UI 设计，为后续功能开发奠定基础。

## What Changes

- 创建 Tauri v2 + Vue 3 项目框架
- 实现三栏布局 UI（侧边栏、邮件列表、阅读面板）
- 创建基础组件库（按钮、输入框、模态框等）
- 配置 Pinia 状态管理
- 实现深色主题样式系统
- 创建可交互的原型用于需求确认

## Capabilities

### New Capabilities

- `project-framework`: Tauri v2 + Vue 3 项目脚手架，包含基础构建配置和开发环境
- `ui-layout`: 三栏邮件客户端布局系统，支持响应式和可调节面板
- `design-system`: 深色主题设计系统，包含颜色令牌、组件样式和图标系统
- `state-management`: Pinia 状态管理，支持账户、邮箱、UI 状态存储

### Modified Capabilities

<!-- 无现有能力需要修改 -->

## Impact

- **前端**: Vue 3 Composition API + TypeScript
- **构建工具**: Vite + Tauri CLI
- **样式**: Tailwind CSS v4
- **依赖**: Pinia, Lucide Vue, DOMPurify
- **目标平台**: Windows 10+, macOS 10.13+, Linux
