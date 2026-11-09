## Why

AetherMail 目前缺少通讯录功能，用户无法管理邮件联系人。每次发邮件都需要手动输入完整邮箱地址，没有自动补全，也没有联系人信息管理。这降低了邮件处理效率，特别是对于频繁沟通的用户。

实现通讯录基础功能是提升用户体验的关键一步，为后续高级功能（如联系人分组、VIP标记等）奠定基础。

## What Changes

- 收到邮件时自动识别发件人，upsert 到联系人表（存在则更新频率，不存在则创建）
- 新增联系人管理界面，支持查看、编辑联系人信息（显示名称、头像）
- 发件时提供联系人自动补全功能，基于历史沟通频率排序
- 支持手动搜索联系人

## Capabilities

### New Capabilities

- `contacts-management`: 联系人数据模型、数据库表、服务层 API，包括自动创建/更新联系人、联系人 CRUD 操作
- `contacts-autocomplete`: 发件时的地址自动补全功能，包括搜索、排序、UI 交互
- `contacts-editor`: 联系人编辑界面，支持修改显示名称、头像等基本信息

### Modified Capabilities

<!-- 无现有能力变更 -->

## Impact

### 数据库

- 新增 `contacts` SQLite 表

### 前端

- 新增联系人相关 Vue 组件（编辑器、自动补全输入框）
- 修改邮件列表和邮件详情组件，触发联系人 upsert
- 修改写邮件组件，集成自动补全

### 后端 (Rust/Tauri)

- 新增联系人相关的 Tauri 命令
- 邮件同步流程中集成联系人 upsert 逻辑

### 状态管理

- 新增 `contactsStore` (Pinia)