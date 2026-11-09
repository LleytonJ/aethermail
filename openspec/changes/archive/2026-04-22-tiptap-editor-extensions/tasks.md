## 1. InputDialog 组件

- [x] 1.1 创建 `src/components/ui/InputDialog.vue` 组件，包含标题、输入框、确认/取消按钮
- [x] 1.2 实现 Escape 键关闭和 backdrop 点击关闭
- [x] 1.3 添加打开/关闭过渡动画（参考 ConfirmDialog）

## 2. TipTap 扩展配置

- [x] 2.1 修改 RichTextEditor.vue 的 StarterKit 配置，启用 `heading: { levels: [1, 2, 3] }`
- [x] 2.2 确保其他禁用项保持：`codeBlock: false`, `blockquote: false`, `horizontalRule: false`

## 3. 工具栏按钮扩展

- [x] 3.1 导入新图标：`Underline`, `Strikethrough` from `lucide-vue-next`
- [x] 3.2 添加下划线按钮（`toggleUnderline`），实现激活状态显示
- [x] 3.3 添加删除线按钮（`toggleStrike`），实现激活状态显示
- [x] 3.4 添加 H1、H2、H3 标题按钮（`toggleHeading({ level: 1/2/3 })`）
- [x] 3.5 重新组织工具栏布局：文字格式组 | 标题组 | 列表组 | 链接/图片组 | 颜色组

## 4. 链接插入改进

- [x] 4.1 在 RichTextEditor.vue 中添加 `showLinkDialog` 和 `linkUrl` 状态
- [x] 4.2 修改 `handleLink` 函数：已有链接时移除，无链接时打开 InputDialog
- [x] 4.3 实现 `confirmLink` 函数，设置链接并关闭对话框
- [x] 4.4 添加 URL 基本验证：检查是否为空，无协议时自动补全 `https://`

## 5. 图片插入改进

- [x] 5.1 在 RichTextEditor.vue 中添加隐藏的 `<input type="file" accept="image/*">` 元素
- [x] 5.2 修改图片按钮点击行为：触发文件选择而非 `window.prompt`
- [x] 5.3 实现 `handleImageSelect` 函数：FileReader 读取文件并转为 Base64
- [x] 5.4 调用 `editor.chain().focus().setImage({ src: dataUrl }).run()` 插入图片

## 6. CSS 样式

- [x] 6.1 添加标题样式：`:deep(.tiptap h1)`、`h2`、`h3` 的字号和间距
- [x] 6.2 确保下划线、删除线样式正确显示
- [x] 6.3 确保图片样式：`max-width: 100%`、`height: auto`

## 7. 测试验证

- [x] 7.1 手动测试所有格式化按钮的添加/移除功能
- [x] 7.2 测试链接对话框的打开、输入、关闭、URL 验证
- [x] 7.3 测试图片文件选择和 Base64 插入
- [x] 7.4 测试格式化内容的发送和接收兼容性