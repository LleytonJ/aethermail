---
name: attachment-state
description: 附件状态模型规格
---

# 规格：附件状态模型

## 概述

定义附件的下载状态模型，用于前端展示和后端存储。

## 状态定义

### 后端状态（数据库）

存储在 SQLite `attachments` 表中：

| 字段 | 类型 | 说明 |
|------|------|------|
| `is_downloaded` | INTEGER (bool) | 是否已下载到本地缓存 |
| `local_path` | TEXT | 本地缓存文件路径（已下载时有效） |
| `imap_section` | TEXT | IMAP MIME section 编号（IMAP 附件） |

### 前端状态（UI 展示）

```typescript
interface AttachmentStatus {
  // 三种状态
  status: 'not_downloaded' | 'downloading' | 'downloaded' | 'error'
  
  // 下载进度（downloading 状态时有效）
  progress: number       // 0-100 百分比
  downloadedBytes: number // 已下载字节数
  
  // 错误信息（error 状态时有效）
  errorMessage?: string
}
```

## 状态流转

```
┌──────────────────┐
│  not_downloaded  │  初始状态（IMAP 同步后）
│                  │
│  POP3 同步后直接 │
│  进入 downloaded │
└──────────────────┘
        │
        │ 用户点击下载
        ▼
┌──────────────────┐
│   downloading    │
│                  │
│  progress: 0-100 │
│  每更新发送事件   │
└──────────────────┘
        │
        ├─────────────── 下载成功
        │
        ▼
┌──────────────────┐
│    downloaded    │
│                  │
│  local_path 已设置│
│  可保存到用户位置 │
└──────────────────┘

        │
        ├─────────────── 下载失败
        │
        ▼
┌──────────────────┐
│      error       │
│                  │
│  errorMessage    │
│  可重新点击下载   │
└──────────────────┘
```

## API 接口

### 后端命令

#### `download_attachment`

**输入**:
```typescript
{
  attachment_id: string
}
```

**输出**:
```typescript
string  // 本地缓存路径
```

**副作用**:
- 发送 `attachment-progress` 事件
- 发送 `attachment-complete` 事件
- 更新数据库 `is_downloaded=true`

#### `save_attachment_to_path`

**输入**:
```typescript
{
  attachment_id: string,
  target_path: string
}
```

**输出**:
```typescript
void
```

**前提条件**:
- `is_downloaded=true`

### Tauri 事件

#### `attachment-progress`

**Payload**:
```typescript
interface AttachmentProgress {
  attachment_id: string
  downloaded: number    // 已下载字节
  total: number         // 总字节
  percent: number       // 百分比 (0-100)
}
```

**频率**: 每 64KB 发送一次

#### `attachment-complete`

**Payload**:
```typescript
interface AttachmentComplete {
  attachment_id: string
  local_path: string
}
```

## 边界情况

| 场景 | 处理 |
|------|------|
| 用户重复点击下载 | 第二次点击直接返回已缓存路径 |
| 下载中途取消 | 当前不支持取消，后续可扩展 |
| 网络中断 | 删除部分文件，下次重新下载 |
| 缓存文件损坏 | 删除缓存，重新下载 |
| 离线状态点击下载 | 前端阻止，显示提示 |

## POP3 特殊处理

POP3 协议不支持部分获取，同步时必须全量下载邮件（含附件）。

**处理方式**:
- 同步时下载附件 → 存入缓存目录
- 数据库记录 `is_downloaded=true`、`local_path`
- 用户点击时直接从缓存读取

**UI 差异**: 不显示，与 IMAP 统一体验