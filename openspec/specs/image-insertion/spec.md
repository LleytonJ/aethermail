## ADDED Requirements

### Requirement: 用户可以通过文件选择插入图片

用户 SHALL 能够通过文件选择对话框插入本地图片，替代 `window.prompt` 的 URL 输入方式。

#### Scenario: 打开文件选择
- **WHEN** 用户点击工具栏的图片按钮
- **THEN** 系统打开文件选择对话框
- **AND** 对话框限制文件类型为图片格式（jpg、png、gif、webp 等）

#### Scenario: 选择并插入图片
- **WHEN** 用户选择一个图片文件并确认
- **THEN** 图片被转换为 Base64 格式
- **AND** 图片插入到编辑器光标位置
- **AND** 图片以 `<img src="data:image/...;base64,...">` 形式保存

#### Scenario: 取消文件选择
- **WHEN** 用户取消文件选择对话框
- **THEN** 不插入任何内容

---

### Requirement: 图片正确显示在编辑器中

插入的图片 SHALL 在编辑器中正确显示，并保持合理的尺寸限制。

#### Scenario: 图片显示
- **WHEN** 图片插入到编辑器
- **THEN** 图片在编辑器中可见
- **AND** 图片宽度不超过编辑器宽度（自适应）

#### Scenario: 加载带图片的内容
- **WHEN** 编辑器加载包含 Base64 图片的 HTML 内容
- **THEN** 图片正确显示
- **AND** 用户可以继续编辑

---

### Requirement: 图片以 Base64 内嵌形式保存

图片 SHALL 以 Base64 DataURL 形式内嵌到 HTML 内容中，确保邮件发送时无需外部图片引用。

#### Scenario: 保存带图片的邮件
- **WHEN** 用户发送包含图片的邮件
- **THEN** 图片内容已内嵌到 HTML 中
- **AND** 邮件不依赖外部图片服务器

#### Scenario: 图片格式支持
- **WHEN** 用户插入图片
- **THEN** 系统支持常见图片格式：JPEG、PNG、GIF、WebP、BMP