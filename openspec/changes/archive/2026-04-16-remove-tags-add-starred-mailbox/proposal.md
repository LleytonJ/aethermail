## Why

当前标签功能（TagList.vue）仅为前端 UI 占位符，无后端 API、数据库存储和 CRUD 功能，给用户造成功能误解。星标切换功能已完整实现，但缺少专门的星标邮件箱视图，用户无法快速查看所有星标邮件。

## What Changes

- **移除**: 删除 `TagList.vue` 组件及相关引用
- **移除**: 从 Sidebar.vue 移除标签分组 UI 和数据
- **新增**: 星标邮件箱文件夹（类似钓鱼邮件箱的实现模式）
- **新增**: 后端 API `get_starred_emails`、`get_starred_count`
- **新增**: 前端服务方法和 store 分支处理

## Capabilities

### New Capabilities

- `starred-folder`: 星标邮件箱文件夹功能，提供侧边栏入口和星标邮件列表视图

### Modified Capabilities

- `email-storage`: 扩展邮件存储查询，新增获取星标邮件列表和数量的方法

## Impact

**前端文件**:
- `aethermail/src/components/layout/TagList.vue` - 删除
- `aethermail/src/components/layout/Sidebar.vue` - 移除标签 UI，添加星标文件夹
- `aethermail/src/stores/mailbox.ts` - 添加 starred 分支处理
- `aethermail/src/services/email/emailService.ts` - 添加星标邮件获取方法

**后端文件**:
- `aethermail/src-tauri/src/commands/email.rs` - 添加 `get_starred_emails`、`get_starred_count` 命令
- `aethermail/src-tauri/src/storage/database.rs` - 添加星标邮件查询方法
- `aethermail/src-tauri/src/lib.rs` - 注册新命令