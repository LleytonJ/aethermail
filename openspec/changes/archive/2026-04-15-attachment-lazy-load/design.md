---
name: attachment-lazy-load
description: 附件懒加载技术设计文档
---

# 设计文档：附件懒加载

## 架构概览

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                          附件懒加载架构                                       │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│   ┌─────────────┐                                                           │
│   │  前端 (Vue) │                                                           │
│   └─────────────┘                                                           │
│         │                                                                   │
│         │ invoke('download_attachment')                                     │
│         │ listen('attachment-progress')                                     │
│         ▼                                                                   │
│   ┌─────────────────────────────────────────────────────────────────────┐  │
│   │                    Tauri Commands                                    │  │
│   │  ┌──────────────────┐  ┌──────────────────┐  ┌──────────────────┐  │  │
│   │  │ sync_emails      │  │ download_att     │  │ get_cache_stats  │  │  │
│   │  │ (改造)           │  │ (新增)           │  │ (新增)           │  │  │
│   │  └──────────────────┘  └──────────────────┘  └──────────────────┘  │  │
│   └─────────────────────────────────────────────────────────────────────┘  │
│         │                                                                   │
│         ▼                                                                   │
│   ┌─────────────────────────────────────────────────────────────────────┐  │
│   │                    Protocol Clients                                  │  │
│   │  ┌──────────────────┐  ┌──────────────────┐                         │  │
│   │  │ IMAP Client      │  │ POP3 Client      │                         │  │
│   │  │ (改造)           │  │ (保持不变)       │                         │  │
│   │  │ fetch_structure  │  │                  │                         │  │
│   │  │ fetch_body_only  │  │                  │                         │  │
│   │  │ fetch_attachment │  │                  │                         │  │
│   │  └──────────────────┘  └──────────────────┘                         │  │
│   └─────────────────────────────────────────────────────────────────────┘  │
│         │                                                                   │
│         ▼                                                                   │
│   ┌─────────────────────────────────────────────────────────────────────┐  │
│   │                    Storage Layer                                     │  │
│   │  ┌──────────────────┐  ┌──────────────────────────────────────────┐│  │
│   │  │ SQLite Database  │  │ File System Cache                       ││  │
│   │  │ (改造)           │  │ app_data_dir/attachments/               ││  │
│   │  │ attachments:     │  │   ├── att-uuid-1.dat                    ││  │
│   │  │   - imap_section │  │   ├── att-uuid-2.dat                    ││  │
│   │  │   - is_downloaded│  │   └── ...                               ││  │
│   │  │   - local_path   │  │                                         ││  │
│   │  │   ❌ 无 content   │  │                                         ││  │
│   │  └──────────────────┘  └──────────────────────────────────────────┐│  │
│   └─────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## 模块设计

### 1. 数据库模型改造

**文件**: `storage/models.rs`

```rust
pub struct DbAttachment {
    pub id: String,
    pub email_id: String,
    pub filename: String,
    pub mime_type: String,
    pub size: i64,
    pub content_id: Option<String>,
    
    // 新增字段
    pub imap_section: Option<String>,   // IMAP MIME section 编号 (如 "2", "3", "2.1")
    pub is_downloaded: bool,            // 是否已下载到本地缓存
    pub local_path: Option<String>,     // 本地缓存文件路径
    
    // 移除字段
    // pub content: Option<Vec<u8>>,     // 不再存储在数据库
}
```

**表结构**:

```sql
CREATE TABLE IF NOT EXISTS attachments (
    id TEXT PRIMARY KEY,
    email_id TEXT NOT NULL,
    filename TEXT NOT NULL,
    mime_type TEXT NOT NULL,
    size INTEGER NOT NULL,
    content_id TEXT,
    imap_section TEXT,                      -- 新增
    is_downloaded INTEGER NOT NULL DEFAULT 0, -- 新增
    local_path TEXT,                        -- 新增
    FOREIGN KEY (email_id) REFERENCES emails(id) ON DELETE CASCADE
);
```

### 2. IMAP 客户端改造

**文件**: `imap/client.rs`

新增方法:

```rust
/// 获取邮件 MIME 结构（不含内容）
/// 用于解析附件 section 编号
pub async fn fetch_structure(&mut self, uid: u32) -> Result<EmailStructure, String> {
    // UID FETCH <uid> BODYSTRUCTURE
    // 返回 MIME 树结构，解析各部分的 section 编号
}

/// 获取邮件正文（第一个 MIME 部分）
/// 避免下载附件
pub async fn fetch_body_only(&mut self, uid: u32) -> Result<(String, Option<String>), String> {
    // UID FETCH <uid> (BODY.PEEK[1] BODY.PEEK[1.MIME])
    // 返回 (text_body, html_body)
}

/// 获取单个附件内容
/// section 编号如 "2", "3", "2.1" 等
pub async fn fetch_attachment_section(
    &mut self, 
    uid: u32, 
    section: &str
) -> Result<Vec<u8>, String> {
    // UID FETCH <uid> BODY[<section>]
    // 返回原始二进制数据
}
```

### 3. 邮件同步逻辑改造

**文件**: `commands/email.rs`

#### IMAP 同步改造

```rust
async fn sync_via_imap(account: &AccountConfig) -> Result<SyncResult, String> {
    // 1. 连接 IMAP
    let mut session = client.connect().await?;
    
    // 2. 获取新邮件 UID 列表
    let new_uids = get_new_uids(&session, &db, &account_id).await?;
    
    // 3. 遍历新邮件
    for uid in new_uids {
        // 3.1 获取 MIME 结构
        let structure = session.fetch_structure(uid).await?;
        
        // 3.2 只获取正文 (section 通常是 "1")
        let (body_text, body_html) = session.fetch_body_only(uid).await?;
        
        // 3.3 解析附件信息 (不含内容)
        let attachments = parse_attachments_from_structure(&structure);
        
        // 3.4 存入数据库
        // email: 正文 + 元信息
        // attachments: 元信息 + is_downloaded=false
    }
    
    session.logout().await?;
    Ok(SyncResult { ... })
}

/// 从 BODYSTRUCTURE 解析附件信息
fn parse_attachments_from_structure(structure: &EmailStructure) -> Vec<AttachmentMeta> {
    // 遍历 MIME 树，识别附件部分 (Content-Disposition: attachment)
    // 记录每个附件的 section 编号
}
```

#### POP3 同步（保持不变，但存文件而非数据库）

```rust
async fn sync_via_pop3(account: &AccountConfig) -> Result<SyncResult, String> {
    // POP3 无法部分获取，必须全量下载
    // 但附件存文件而非数据库 BLOB
    
    for att in parsed.attachments {
        // 保存到缓存目录
        let local_path = save_attachment_to_cache(&app_handle, &att)?;
        
        let db_att = DbAttachment {
            imap_section: None,       // POP3 不需要
            is_downloaded: true,      // 已下载
            local_path: Some(local_path),
            // ... 其他元信息
        };
        db.save_attachment(&db_att).await?;
    }
}
```

### 4. 附件下载命令

**文件**: `commands/email.rs`

```rust
/// 下载进度事件 payload
#[derive(Clone, Serialize)]
pub struct AttachmentProgress {
    pub attachment_id: String,
    pub downloaded: u64,
    pub total: u64,
    pub percent: u8,
}

/// 下载完成事件 payload
#[derive(Clone, Serialize)]
pub struct AttachmentComplete {
    pub attachment_id: String,
    pub local_path: String,
}

/// 下载单个附件
#[tauri::command]
pub async fn download_attachment(
    attachment_id: String,
    app_handle: tauri::AppHandle,
) -> Result<String, String> {
    let db = get_db().await?;
    let att = db.get_attachment_by_id(&attachment_id).await?;
    
    // 已下载 → 直接返回路径
    if att.is_downloaded {
        return Ok(att.local_path.unwrap_or_default());
    }
    
    // 获取邮件和账户信息
    let email = db.get_email_by_id(&att.email_id).await?;
    let account = get_account_by_id(&email.account_id)?;
    
    // 连接 IMAP
    let mut session = connect_imap(&account).await?;
    
    // 创建缓存文件
    let cache_path = get_attachment_cache_path(&app_handle, &attachment_id)?;
    let mut file = File::create(&cache_path)?;
    
    // 分块下载 + 发送进度事件
    let total = att.size as u64;
    let chunk_size = 64 * 1024; // 64KB
    let mut downloaded = 0u64;
    
    // 获取附件内容
    let content = session
        .fetch_attachment_section(
            email.uidl.parse()?,
            att.imap_section.as_ref().unwrap()
        )
        .await?;
    
    // 分块写入 + 发送进度
    for chunk in content.chunks(chunk_size) {
        file.write_all(chunk)?;
        downloaded += chunk.len() as u64;
        
        let percent = (downloaded * 100 / total).min(100) as u8;
        app_handle.emit("attachment-progress", AttachmentProgress {
            attachment_id: attachment_id.clone(),
            downloaded,
            total,
            percent,
        }).ok();
    }
    
    // 更新数据库
    db.update_attachment_downloaded(&attachment_id, &cache_path.to_string_lossy()).await?;
    
    // 发送完成事件
    app_handle.emit("attachment-complete", AttachmentComplete {
        attachment_id,
        local_path: cache_path.to_string_lossy().to_string(),
    });
    
    Ok(cache_path.to_string_lossy().to_string())
}

/// 保存附件到用户选择的位置
#[tauri::command]
pub async fn save_attachment_to_path(
    attachment_id: String,
    target_path: String,
) -> Result<(), String> {
    let db = get_db().await?;
    let att = db.get_attachment_by_id(&attachment_id).await?;
    
    if !att.is_downloaded {
        return Err("附件未下载".to_string());
    }
    
    let source_path = att.local_path.unwrap();
    std::fs::copy(&source_path, &target_path)
        .map_err(|e| format!("复制文件失败: {}", e))?;
    
    Ok(())
}
```

### 5. 缓存管理命令

```rust
/// 获取缓存统计信息
#[tauri::command]
pub async fn get_attachment_cache_stats(
    app_handle: tauri::AppHandle,
) -> Result<CacheStats, String> {
    let cache_dir = get_cache_directory(&app_handle)?;
    
    let mut total_size = 0u64;
    let mut file_count = 0u32;
    
    for entry in std::fs::read_dir(&cache_dir)? {
        let entry = entry?;
        if entry.path().extension().map(|e| e == "dat").unwrap_or(false) {
            file_count += 1;
            total_size += entry.metadata()?.len();
        }
    }
    
    Ok(CacheStats {
        file_count,
        total_size,
        cache_path: cache_dir.to_string_lossy().to_string(),
    })
}

/// 清空附件缓存
#[tauri::command]
pub async fn clear_attachment_cache(
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    let db = get_db().await?;
    let cache_dir = get_cache_directory(&app_handle)?;
    
    // 删除缓存文件
    for entry in std::fs::read_dir(&cache_dir)? {
        std::fs::remove_file(entry?.path())?;
    }
    
    // 更新数据库状态
    db.mark_all_attachments_not_downloaded().await?;
    
    Ok(())
}

/// 打开缓存目录
#[tauri::command]
pub async fn open_attachment_cache_directory(
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    let cache_dir = get_cache_directory(&app_handle)?;
    
    // 使用系统默认程序打开目录
    #[cfg(target_os = "windows")]
    std::process::Command::new("explorer")
        .arg(&cache_dir)
        .spawn()?;
    
    #[cfg(target_os = "macos")]
    std::process::Command::new("open")
        .arg(&cache_dir)
        .spawn()?;
    
    #[cfg(target_os = "linux")]
    std::process::Command::new("xdg-open")
        .arg(&cache_dir)
        .spawn()?;
    
    Ok(())
}
```

### 6. 前端组件设计

#### AttachmentItem.vue 状态模型

```typescript
interface AttachmentState {
  status: 'not_downloaded' | 'downloading' | 'downloaded' | 'error'
  progress: number        // 0-100
  downloadedBytes: number
  errorMessage?: string
}
```

#### 核心逻辑

```typescript
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { save } from '@tauri-apps/plugin-dialog'

const state = reactive<AttachmentState>({
  status: props.attachment.isDownloaded ? 'downloaded' : 'not_downloaded',
  progress: 0,
  downloadedBytes: 0
})

// 离线检测
const isOffline = computed(() => !navigator.onLine)
const showOfflineHint = computed(() => state.status === 'not_downloaded' && isOffline)

async function handleDownload() {
  if (isOffline.value) {
    notificationStore.warning('需要网络连接', '请连接网络后下载附件')
    return
  }
  
  state.status = 'downloading'
  state.progress = 0
  
  // 监听进度事件
  const unlistenProgress = await listen<ProgressPayload>('attachment-progress', (e) => {
    if (e.payload.attachment_id === props.attachment.id) {
      state.progress = e.payload.percent
      state.downloadedBytes = e.payload.downloaded
    }
  })
  
  try {
    const localPath = await invoke<string>('download_attachment', {
      attachmentId: props.attachment.id
    })
    state.status = 'downloaded'
  } catch (error) {
    state.status = 'error'
    state.errorMessage = String(error)
  } finally {
    unlistenProgress()
  }
}

async function handleSaveTo() {
  const targetPath = await save({
    defaultPath: props.attachment.filename,
    filters: [{ name: props.attachment.filename }]
  })
  
  if (targetPath) {
    await invoke('save_attachment_to_path', {
      attachmentId: props.attachment.id,
      targetPath
    })
    notificationStore.success('保存成功', `附件已保存到 ${targetPath}`)
  }
}
```

### 7. 离线处理

```typescript
// services/email/offlineService.ts
export function checkOfflineStatus(): boolean {
  return !navigator.onLine
}

export function getOfflineAttachmentMessage(): string {
  return '需要网络连接才能下载此附件'
}

// 窗口全局监听网络状态
window.addEventListener('offline', () => {
  // 触发全局离线提示
})

window.addEventListener('online', () => {
  // 触发全局在线提示
})
```

## 文件存储结构

```
app_data_dir/
├── aethermail.db              # 数据库
└── attachments/               # 附件缓存目录
    ├── att-<uuid-1>.dat       # 已下载的附件
    ├── att-<uuid-2>.dat
    ├── att-<uuid-3>.dat
    └── ...
```

## 数据迁移策略

存量数据迁移（从旧版 BLOB 存储迁移到文件存储）：

```rust
pub async fn migrate_attachments_to_file_cache(
    app_handle: &tauri::AppHandle,
) -> Result<(), String> {
    let db = get_db().await?;
    let cache_dir = get_cache_directory(app_handle)?;
    
    // 查询所有有 content 的附件
    let attachments = db.get_attachments_with_content().await?;
    
    for att in attachments {
        if let Some(content) = att.content {
            // 写入文件
            let path = cache_dir.join(format!("{}.dat", att.id));
            std::fs::write(&path, &content)?;
            
            // 更新数据库
            db.update_attachment_downloaded(
                &att.id, 
                &path.to_string_lossy()
            ).await?;
        }
    }
    
    // 移除 content 列（通过重建表或 ALTER）
    // ...
    
    Ok(())
}
```

## 错误处理

| 场景 | 处理方式 |
|------|----------|
| IMAP 连接失败 | 返回错误，前端显示"下载失败，请检查网络" |
| 下载中断 | 删除部分文件，下次重新下载 |
| 缓存目录不存在 | 自动创建 |
| 写入文件失败 | 返回错误，前端显示具体错误信息 |
| 附件 section 无效 | 返回错误，前端显示"附件信息错误" |

## 性能考虑

| 优化点 | 方案 |
|--------|------|
| 大文件下载 | 64KB 分块，避免一次性加载到内存 |
| 进度事件频率 | 每 64KB 发送一次，避免过于频繁 |
| 并发下载 | 单附件串行下载，未来可扩展并发 |
| 缓存清理 | 用户手动触发，不自动清理 |