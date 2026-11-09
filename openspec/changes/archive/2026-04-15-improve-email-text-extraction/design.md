## Context

当前邮件解析使用 `mail-parser` 库，其 `body_text()` 方法对于 multipart/alternative 邮件会正确提取 text/plain 部分，但对于只有 text/html 的邮件，它会从 HTML 中提取文本但保留 `<style>` 和 `<script>` 标签内的内容。

邮件 MIME 结构：
- multipart/alternative: 同时包含 text/plain 和 text/html → mail-parser 正常工作
- text/html only: 只有 HTML 内容 → mail-parser 的 body_text() 包含 CSS/JS 干扰

## Goals / Non-Goals

**Goals:**
- 提取邮件纯文本用于钓鱼检测时，排除 CSS 样式和 JS 代码
- 优先使用邮件自带的 text/plain 内容
- 仅对纯 HTML 邮件使用 scraper 进行 DOM 解析提取

**Non-Goals:**
- 不改变邮件存储格式（仍保留原始 body_html）
- 不修改钓鱼检测逻辑本身
- 不处理富文本渲染（仅提取纯文本）

## Decisions

### Decision 1: 使用 scraper 库进行 HTML 解析

**选择**: scraper crate
**理由**: Rust 生态中成熟的 HTML 解析库，支持 CSS 选择器，类似 Python BeautifulSoup

**备选方案**:
- html2text: 更专注于转换，但 scraper 提供更精细的控制
- regex: 简单但无法正确处理嵌套标签和复杂结构

### Decision 2: 文本提取策略

```
┌─────────────────────────────────────────────────────────────────────┐
│                      文本提取流程                                    │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  1. mail-parser 解析原始邮件                                        │
│     ↓                                                               │
│  2. body_text 存在且内容有效?                                       │
│     ├─ YES → 直接使用 body_text                                     │
│     │         (邮件发送方已准备纯文本版本)                          │
│     │                                                               │
│     └─ NO → 使用 scraper 从 body_html 提取                          │
│              selector: "body :not(style):not(script)"               │
│              只提取可见文本节点                                      │
│                                                                     │
│  3. 用于钓鱼检测 AI 调用                                            │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

### Decision 3: CSS 选择器策略

**选择**: `body :not(style):not(script):not(head)`
**理由**: 排除所有非内容区域，保留可见文本

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| scraper 解析增加少量性能开销 | 仅在需要时调用，不影响已有 text/plain 的邮件 |
| 某些邮件 body 可能无内容（纯图片） | 返回空字符串，钓鱼检测正常处理 |
| scraper 依赖增加编译时间 | 库轻量，影响较小 |