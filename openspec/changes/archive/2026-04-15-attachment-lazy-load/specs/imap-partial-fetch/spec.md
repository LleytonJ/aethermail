---
name: imap-partial-fetch
description: IMAP 部分获取规格
---

# 规格：IMAP 部分获取

## 概述

定义 IMAP 协议的部分获取机制，用于实现附件懒加载。

## IMAP 命令

### 获取邮件结构

**命令**: `UID FETCH <uid> BODYSTRUCTURE`

**返回**: MIME 消息结构树

**用途**: 解析各部分的 section 编号，识别附件

**示例响应**:
```
* 1 FETCH (UID 123 BODYSTRUCTURE (
  ("TEXT" "PLAIN" ("CHARSET" "UTF-8") NIL NIL "7BIT" 1234 45 NIL NIL NIL)
  ("APPLICATION" "PDF" ("NAME" "report.pdf") NIL NIL "BASE64" 250000 1 NIL ("attachment" NIL) NIL)
  "MIXED"
))
```

### 获取正文

**命令**: `UID FETCH <uid> BODY.PEEK[1]`

**说明**:
- `BODY.PEEK[1]` 获取第一个 MIME 部分（通常是正文）
- `PEEK` 标记表示不设置 `\Seen` 标志
- `[1]` 是 section 编号

**返回**: 正文内容（text/plain 或 text/html）

### 获取附件

**命令**: `UID FETCH <uid> BODY[<section>]`

**示例**:
```
UID FETCH 123 BODY[2]    # 获取第二个 MIME 部分（第一个附件）
UID FETCH 123 BODY[3]    # 获取第三个 MIME 部分
UID FETCH 123 BODY[2.1]  # 获取嵌套 MIME 结构中的部分
```

**返回**: 该部分的原始内容（通常需要 Base64 解码）

## MIME Section 编号规则

```
MIME 消息结构:
┌─────────────────────────────────────────┐
│ multipart/mixed (root)                  │
│                                         │
│   ├── [1] text/plain (正文)             │
│   │                                     │
│   ├── [2] application/pdf (附件1)       │
│   │     filename="report.pdf"           │
│   │                                     │
│   └── [3] multipart/related (嵌套)      │
│         │                               │
│         ├── [3.1] text/html (HTML正文)  │
│         │                               │
│         ├── [3.2] image/png (内嵌图片)  │
│         │     Content-ID: <img1>        │
│         │                               │
│         └── [3.3] image/jpeg (内嵌图片) │
│               Content-ID: <img2>        │
│                                         │
└─────────────────────────────────────────┘
```

**编号规则**:
- 根消息的子部分从 1 开始编号
- 嵌套 multipart 的子部分用 `parent.child` 编号
- 非 multipart 的 leaf 部分是实际内容

**识别附件**:
- `Content-Disposition: attachment` → 是附件
- `Content-Disposition: inline` + 有 `filename` → 可能是附件
- 内嵌图片（有 Content-ID）→ 通常不视为附件

## 实现设计

### EmailStructure 类型

```rust
/// MIME 消息结构
pub struct EmailStructure {
    pub parts: Vec<MimePart>,
}

/// MIME 部分
pub struct MimePart {
    pub section: String,           // 如 "1", "2", "3.1"
    pub mime_type: String,         // 如 "application/pdf"
    pub filename: Option<String>,  // 文件名
    pub size: u64,                 // 大小（字节）
    pub content_id: Option<String>,// Content-ID（内嵌图片）
    pub is_attachment: bool,       // 是否为附件
    pub children: Vec<MimePart>,   // 子部分（multipart 时）
}
```

### 解析 BODYSTRUCTURE

```rust
fn parse_bodystructure(bs: &BodyStructure) -> Result<EmailStructure, String> {
    let mut parts = Vec::new();
    
    // 递归解析 MIME 树
    parse_mime_parts(bs, "", &mut parts);
    
    Ok(EmailStructure { parts })
}

fn parse_mime_parts(bs: &BodyStructure, parent_section: &str, parts: &mut Vec<MimePart>) {
    match bs {
        BodyStructure::Multi(multi) => {
            // multipart: 递归处理子部分
            for (i, child) in multi.parts.iter().enumerate() {
                let section = format!("{}.{}", parent_section, i + 1);
                parse_mime_parts(child, &section, parts);
            }
        }
        BodyStructure::Single(single) => {
            // single part: 检查是否为附件
            let is_attachment = single.disposition
                .map(|d| d.disposition == "attachment")
                .unwrap_or(false);
            
            parts.push(MimePart {
                section: parent_section.trim_start_matches('.').to_string(),
                mime_type: format!("{}/{}", single.type, single.subtype),
                filename: single.name.clone(),
                size: single.size as u64,
                content_id: single.content_id.clone(),
                is_attachment,
                children: Vec::new(),
            });
        }
    }
}
```

## 错误处理

| 场景 | 处理 |
|------|------|
| IMAP 服务器不支持 BODYSTRUCTURE | 回退到全量下载 |
| section 编号无效 | 返回错误，前端显示 |
| 附件内容编码错误 | 自动解码（Base64、Quoted-Printable） |

## 兼容性

| IMAP 服务器 | BODYSTRUCTURE | 部分获取 |
|-------------|---------------|----------|
| Gmail | ✅ | ✅ |
| Outlook.com | ✅ | ✅ |
| Yahoo Mail | ✅ | ✅ |
| Exchange | ✅ | ✅ |
| 自建 Postfix/Dovecot | ✅ | ✅ |

主流 IMAP 服务器均支持 BODYSTRUCTURE 和部分获取。