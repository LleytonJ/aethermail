## Context

当前 aethermail 项目使用 Vue 3 + Pinia + Tauri 技术栈。邮件撰写功能 (`ComposeEmail.vue`) 使用简单的 textarea 编辑器，仅支持纯文本。需要引入 TipTap 富文本编辑器实现格式化编辑和签名功能。

竞品 velo 项目已验证该方案可行性：
- React 版 TipTap (`@tiptap/react`)
- 签名存储于 SQLite 数据库

aethermail 需要采用 Vue 版 TipTap (`@tiptap/vue-3`) 实现相同功能。

**设计调整**：根据用户需求，签名功能简化为：
- 每个账户只有一个签名
- 签名设置放在账户编辑页面下
- 撰写邮件时通过开关控制是否包含签名

## Goals / Non-Goals

**Goals:**
- 替换 textarea 为 TipTap 富文本编辑器
- 支持基础格式化（加粗、斜体、列表、链接）
- 支持插入图片
- 每个账户可设置一个富文本签名（位于账户设置）
- 发送邮件时可开关控制是否附加签名

**Non-Goals:**
- 多签名管理（v1 简化为单签名）
- 签名选择下拉列表（改为开关控制）
- 实时协作编辑
- Markdown 模式
- HTML 源码编辑模式

## Decisions

### 1. 富文本编辑器选型：TipTap

**决定**: 使用 TipTap (`@tiptap/vue-3`)

**理由**:
- velo 项目已验证可行，降低技术风险
- Headless 设计，UI 可控性强
- 扩展生态丰富
- Vue 3 Composition API 原生支持

### 2. 签名存储方案：accounts 表新增字段

**决定**: 在 `accounts` 表新增 `signature_html` 字段

**理由**:
- 每账户只有一个签名，无需独立表
- 签名是账户的属性，放在账户表更合理
- 简化数据库设计，减少关联查询

**表结构变更**:
```sql
ALTER TABLE accounts ADD COLUMN signature_html TEXT DEFAULT '';
```

### 3. 签名 UI 位置：账户编辑页面

**决定**: 签名编辑放在 `EditAccountForm.vue` 中

**结构**:
```
账户编辑页面
├── 基本信息（邮箱、显示名称）
├── 服务器设置（IMAP/SMTP）
├── 签名设置 ← 新增区域
│   ├── 签名开关（是否启用签名）
│   └── RichTextEditor（签名内容编辑）
└── ...
```

**理由**:
- 签名属于账户，层级清晰
- 用户编辑账户时可一并设置签名
- 不需要额外的签名管理页面

### 4. 撰写时的签名控制：开关而非下拉选择

**决定**: 使用开关控制是否包含签名

**UI**:
```
ComposeEmail.vue 工具栏
├── [格式化工具]
├── [附件按钮]
├── [签名开关] ← 新增，默认开启
└── [发送按钮]
```

**理由**:
- 每账户只有一个签名，不需要"选择"
- 开关操作更简洁
- 用户可快速决定是否包含签名

### 5. 编辑器组件封装策略

**决定**: 创建可复用 `RichTextEditor.vue` 组件

**Props**:
- `content: string` - HTML 内容 (v-model)
- `placeholder: string` - 占位提示
- `minHeight: number` - 最小高度
- `editable: boolean` - 是否可编辑

**理由**:
- 签名编辑和邮件正文共用同一编辑器
- 降低组件复杂度，提升复用性

### 6. 签名附加策略

**决定**: 发送时拼接 HTML

**格式**:
```html
{email-body-html}
<div style="margin-top:16px;border-top:1px solid #e5e5e5;padding-top:12px">
  {signature-html}
</div>
```

**理由**:
- velo 项目采用相同方案
- 分隔线视觉清晰
- 用户可预览签名效果

## Risks / Trade-offs

### [TipTap 学习曲线]
- **风险**: 团队可能不熟悉 TipTap API
- **缓解**: 参考 velo 项目实现，有现成代码可借鉴

### [HTML 安全性]
- **风险**: 用户输入 HTML 可能包含恶意内容
- **缓解**: 使用 DOMPurify 或类似库进行 sanitize

### [邮件客户端兼容性]
- **风险**: 复杂 HTML 签名在某些邮件客户端显示异常
- **缓解**: 提供简洁签名建议，限制过于复杂的样式

### [单签名限制]
- **风险**: 用户可能需要多个签名（正式/非正式）
- **缓解**: v1 保持简单，后续版本可扩展为多签名

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         组件层级关系                                      │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│   SettingsPage.vue                                                      │
│   └── AccountManagement.vue                                             │
│       └── EditAccountForm.vue                                           │
│           ├── 基本信息                                                   │
│           ├── 服务器设置                                                 │
│           └── RichTextEditor.vue  ← 签名编辑区域                         │
│                                                                         │
│   ────────────────────────────────────────────────────────────────────  │
│                                                                         │
│   ComposeEmail.vue                                                      │
│   ├── 地址输入（收件人、抄送）                                            │
│   ├── 主题输入                                                          │
│   ├── RichTextEditor.vue  ← 邮件正文编辑                                 │
│   ├── [签名开关]  ← Toggle 控制                                          │
│   └── 发送按钮                                                          │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

## Open Questions

1. 签名图片是否支持上传到服务器？还是仅支持 base64 嵌入？（建议 v1 仅支持 base64）
2. 签名开关的默认状态是开启还是关闭？（建议默认开启）