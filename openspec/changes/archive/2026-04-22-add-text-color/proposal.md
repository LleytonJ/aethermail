## Why

用户在撰写邮件时需要通过文字颜色来突出重点内容、区分不同类型的信息，提升邮件的可读性和表现力。当前 RichTextEditor 只支持基础的文本格式（加粗、斜体、列表），无法满足用户对文字样式的个性化需求。

## What Changes

- 为 RichTextEditor 组件添加文字颜色设置功能
- 工具栏新增颜色选择按钮，支持快速选择常用颜色
- 支持自定义颜色输入（颜色选择器）
- 支持清除已设置的颜色
- 颜色样式以内联 `style="color: #xxx"` 形式保存，确保邮件客户端兼容性

## Capabilities

### New Capabilities

- `text-color`: 文字颜色设置功能，包括颜色选择器 UI、TipTap 扩展集成、颜色命令调用

### Modified Capabilities

（无现有 capability 需要修改）

## Impact

- **代码影响**:
  - `RichTextEditor.vue`: 新增颜色按钮、颜色选择器 UI、扩展配置
  - `package.json`: 新增依赖 `@tiptap/extension-color`、`@tiptap/extension-text-style`

- **依赖影响**:
  - 新增 npm 包依赖（约 2 个）

- **API 影响**:
  - 编辑器新增命令: `setColor(color)`, `unsetColor()`