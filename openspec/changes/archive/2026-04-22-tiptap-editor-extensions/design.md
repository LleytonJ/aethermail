## Context

当前 aethermail 使用 TipTap Vue-3 作为富文本编辑器，依赖 `@tiptap/starter-kit` 已安装，但配置时禁用了 heading、codeBlock、blockquote 等扩展。工具栏仅提供加粗、斜体、列表和颜色按钮，链接/图片插入使用 `window.prompt`。

对比 velo 项目（React + TipTap），其编辑器提供完整格式化支持和 AI 辅助功能。本次改造聚焦于补齐基础功能，不涉及 AI 特性。

**技术约束：**
- TipTap StarterKit 已内置 underline、strike、heading，只需启用配置
- 需新增 InputDialog 组件替代 `window.prompt`
- 图片需支持本地文件选择，使用 Base64 内嵌

## Goals / Non-Goals

**Goals:**
- 启用 TipTap StarterKit 的 heading 扩展（H1/H2/H3）
- 工具栏添加下划线、删除线、标题按钮
- 创建通用 InputDialog 组件用于链接插入
- 图片插入改为文件选择方式
- 所有格式化内容以内联样式保存，确保邮件客户端兼容

**Non-Goals:**
- 不引入 AI 辅助功能（如 velo 的 AiAssistPanel）
- 不添加代码块、引用块功能（邮件场景需求低）
- 不支持图片拖拽上传（本次仅支持按钮触发文件选择）
- 不添加 Undo/Redo 按钮（后续迭代）

## Decisions

### D1: 使用 StarterKit 内置扩展而非独立安装

**决定：** 启用 StarterKit 已内置的 `heading`、`underline`、`strike`，不单独安装 `@tiptap/extension-underline` 等。

**理由：** StarterKit 已包含这些扩展，只需修改 configure 配置即可。减少依赖数量，降低包体积。

**替代方案：** 单独安装各扩展 → 不必要，增加复杂度。

### D2: InputDialog 作为通用 UI 组件

**决定：** 创建 `src/components/ui/InputDialog.vue` 作为通用输入对话框组件，可用于链接插入及其他场景。

**理由：** 复用性好，符合现有组件结构（如 ConfirmDialog），后续可扩展支持多字段输入。

**替代方案：** 在 RichTextEditor 内部实现链接弹窗 → 组件耦合，不利于复用。

### D3: 图片使用 Base64 内嵌

**决定：** 图片通过 FileReader 转为 Base64 DataURL，直接内嵌到 HTML。

**理由：** TipTap Image extension 已配置 `allowBase64: true`，邮件发送时无需处理外部图片引用，简化实现。

**替代方案：** 图片上传到服务器获取 URL → 需要 API 支持，超出当前范围。

### D4: 标题样式使用 CSS :deep() 选择器

**决定：** 在 RichTextEditor.vue 的 scoped CSS 中使用 `:deep(.tiptap h1)` 等选择器定义标题样式。

**理由：** TipTap 生成的 HTML 位于编辑器内部，需要穿透 scoped 样式。内联样式不利于后续样式调整。

**替代方案：** 使用 TipTap 的 `editorProps.attributes.class` 添加全局类 → 当前方案更简洁。

## Risks / Trade-offs

### R1: Base64 图片增加邮件体积

**风险：** 大图片 Base64 编码后体积膨胀约 33%，可能影响邮件发送效率。

**缓解：** 后续可添加图片压缩或限制图片大小提示。当前阶段保持简单实现。

### R2: InputDialog 与 ConfirmDialog 代码重复

**风险：** InputDialog 与现有 ConfirmDialog 有相似结构，可能导致维护负担。

**缓解：** 两个组件用途不同（确认 vs 输入），暂保持独立。后续可考虑抽象基础 Dialog 组件。