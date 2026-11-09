## ADDED Requirements

### Requirement: 深色主题能正确应用
系统 SHALL 实现深色主题配色方案。

#### Scenario: 默认深色主题
- **WHEN** 应用首次加载
- **THEN** 使用深色背景色（#1e1e2e）和浅色文字（#ffffff）

#### Scenario: 颜色令牌系统
- **WHEN** 组件使用 bg-bg-primary 类
- **THEN** 应用深色主题的主背景色

#### Scenario: 强调色配置
- **WHEN** 用户选择紫色强调色
- **THEN** 按钮、链接等使用紫色（#7c3aed）

### Requirement: 基础 UI 组件可用
系统 SHALL 提供可复用的基础组件。

#### Scenario: Button 组件
- **WHEN** 使用 `<Button variant="primary">`
- **THEN** 渲染带背景色的按钮，hover 有过渡效果

#### Scenario: Input 组件
- **WHEN** 使用 `<Input type="text" placeholder="输入...">`
- **THEN** 渲染带边框的输入框，focus 显示强调色边框

#### Scenario: Skeleton 组件
- **WHEN** 数据加载中
- **THEN** 显示带 shimmer 动画的骨架屏占位

### Requirement: 图标系统正常工作
系统 SHALL 使用 Lucide Vue 图标库。

#### Scenario: 图标渲染
- **WHEN** 使用 `<Inbox />` 组件
- **THEN** 正确渲染收件箱图标

#### Scenario: 图标尺寸
- **WHEN** 设置 `:size="20"`
- **THEN** 图标以 20x20 像素显示

### Requirement: Typography 样式统一
系统 SHALL 定义统一的字体和排版样式。

#### Scenario: 字体家族
- **WHEN** 文本显示
- **THEN** 使用系统字体（-apple-system, BlinkMacSystemFont, Segoe UI）

#### Scenario: 字体大小层级
- **WHEN** 使用 text-sm, text-base, text-lg
- **THEN** 应用对应的字体大小
