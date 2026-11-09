## ADDED Requirements

### Requirement: 用户可以通过对话框插入链接

用户 SHALL 能够通过对话框方式插入或编辑超链接，替代 `window.prompt` 的简陋体验。

#### Scenario: 打开链接对话框
- **WHEN** 用户点击工具栏的链接按钮
- **AND** 当前选中的文字没有链接
- **THEN** 系统打开链接输入对话框
- **AND** 对话框包含 URL 输入框和确认/取消按钮

#### Scenario: 插入链接
- **WHEN** 用户在对话框中输入 URL 并点击确认
- **THEN** 选中的文字变为超链接
- **AND** 链接以 `<a href="URL">文字</a>` 形式保存

#### Scenario: 取消插入
- **WHEN** 用户点击对话框的取消按钮或按 Escape 键
- **THEN** 对话框关闭，不插入链接

---

### Requirement: 用户可以移除已有链接

用户 SHALL 能够移除文字上已有的超链接，恢复为普通文字。

#### Scenario: 移除链接
- **WHEN** 用户选中带有超链接的文字
- **AND** 点击工具栏的链接按钮
- **THEN** 超链接被移除
- **AND** 文字恢复为普通文本

#### Scenario: 链接按钮状态指示
- **WHEN** 用户选中带有超链接的文字
- **THEN** 链接按钮显示激活状态
- **WHEN** 用户选中不带超链接的文字
- **THEN** 链接按钮显示默认状态

---

### Requirement: 链接对话框支持 URL 验证

链接对话框 SHALL 对输入的 URL 进行基本格式验证，提示无效输入。

#### Scenario: 输入有效 URL
- **WHEN** 用户输入以 `http://` 或 `https://` 开头的 URL
- **THEN** 系统接受该 URL 并插入链接

#### Scenario: 输入空 URL
- **WHEN** 用户未输入任何内容并点击确认
- **THEN** 对话框不关闭，提示需要输入 URL

#### Scenario: 输入无协议 URL
- **WHEN** 用户输入不含协议的 URL（如 `example.com`）
- **THEN** 系统自动补全为 `https://example.com`