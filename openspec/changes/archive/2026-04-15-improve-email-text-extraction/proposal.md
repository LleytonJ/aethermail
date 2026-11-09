## Why

当前邮件纯文本提取存在缺陷：当邮件只包含 text/html 部分（无 text/plain）时，mail-parser 库的 body_text() 方法会保留 `<style>` 和 `<script>` 标签内的 CSS/JS 内容。这导致钓鱼检测传递给 AI 的内容包含大量无用代码，浪费 token 并可能干扰判断准确性。

示例问题：课程通知邮件的 body_text 包含几千字符的 CSS 样式代码，而非真正的邮件内容。

## What Changes

- 添加 scraper 库作为 HTML 解析依赖
- 实现智能文本提取策略：
  - 优先使用 mail-parser 提供的 body_text（如果存在且非空）
  - 仅当 body_text 为空或明显包含 CSS/JS 代码时，使用 scraper 从 body_html 提取纯净文本
- scraper 提取时排除 `<style>`、`<script>`、`<head>` 等非内容标签

## Capabilities

### New Capabilities

- `html-text-extraction`: HTML 邮件内容提取能力，使用 DOM 解析器提取可见文本

### Modified Capabilities

- `phishing-detection`: 钓鱼检测的输入内容质量改进（检测逻辑不变，但输入数据更干净）

## Impact

- **代码**: `src-tauri/src/commands/email.rs` 的 parse_email 函数
- **依赖**: 新增 scraper crate
- **API**: 无变化
- **性能**: 仅影响只有 HTML 的邮件，提取过程增加少量开销但结果更准确