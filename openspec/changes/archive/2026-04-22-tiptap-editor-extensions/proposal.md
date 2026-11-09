## Why

当前 aethermail 的 TipTap 富文本编辑器功能不完整，缺少下划线、删除线、标题层级等常用格式化功能。链接和图片插入使用 `window.prompt`，用户体验简陋，不支持本地文件选择。对比 velo 项目的编辑器实现，aethermail 需要补齐这些基础编辑能力，提升邮件撰写体验。

## What Changes

- 启用并扩展 TipTap StarterKit 配置，支持 H1/H2/H3 标题
- 工具栏添加下划线、删除线、标题按钮
- 创建 InputDialog 组件，替换链接插入的 `window.prompt`
- 图片插入改为文件选择器方式，支持本地图片上传
- 添加相关 CSS 样式，确保格式化内容正确显示

## Capabilities

### New Capabilities

- `rich-text-formatting`: 文本格式化能力（加粗、斜体、下划线、删除线、标题H1/H2/H3、有序/无序列表），覆盖工具栏按钮和编辑器样式
- `link-insertion`: 链接插入能力，使用对话框替代 `window.prompt`，支持添加/移除链接
- `image-insertion`: 图片插入能力，支持本地文件选择和 Base64 内嵌

### Modified Capabilities

- `email-sending`: 更新 Enter email body scenario，扩展支持的富文本格式类型列表