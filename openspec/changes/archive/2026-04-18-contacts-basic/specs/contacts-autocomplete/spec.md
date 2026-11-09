## ADDED Requirements

### Requirement: 发件人输入时提供联系人自动补全

系统 SHALL 在用户输入收件人地址时提供基于联系人历史的自动补全建议。

#### Scenario: 输入触发搜索

- **WHEN** 用户在收件人输入框中输入至少 2 个字符
- **THEN** 系统搜索联系人，返回匹配结果列表

#### Scenario: 搜索结果排序

- **WHEN** 系统返回自动补全结果
- **THEN** 结果按 frequency DESC, display_name ASC 排序

#### Scenario: 选择补全建议

- **WHEN** 用户点击或按 Enter 选择某个补全建议
- **THEN** 该联系人邮箱地址添加到收件人列表

#### Scenario: 键盘导航补全列表

- **WHEN** 补全列表显示时
- **AND** 用户按 ↓ 键
- **THEN** 选中下一个建议项

#### Scenario: 手动输入非联系人地址

- **WHEN** 用户输入 `new@unknown.com` 并按 Enter
- **AND** 联系人表中不存在该地址
- **THEN** 系统允许用户使用该地址发件（不强制创建联系人）

### Requirement: 支持多收件人输入

系统 SHALL 允许用户输入多个收件人地址。

#### Scenario: 多地址标签显示

- **WHEN** 用户已添加收件人 `alice@example.com` 和 `bob@example.com`
- **THEN** 系统以标签形式显示已添加的地址，每个标签可单独删除

#### Scenario: 删除已添加地址

- **WHEN** 用户点击某地址标签的删除按钮
- **THEN** 该地址从收件人列表中移除

#### Scenario: 退格删除最后地址

- **WHEN** 输入框为空
- **AND** 用户按 Backspace 键
- **THEN** 删除最后一个已添加的地址

### Requirement: 搜索防抖优化

系统 SHALL 对自动补全搜索进行防抖处理，避免频繁查询。

#### Scenario: 输入防抖

- **WHEN** 用户连续快速输入字符
- **THEN** 系统延迟 200ms 后才执行搜索查询