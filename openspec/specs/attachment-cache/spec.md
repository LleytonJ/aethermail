---
name: attachment-cache
description: 附件缓存管理规格
---

# 规格：附件缓存管理

## 概述

定义附件缓存文件的存储结构和管理接口。

## 文件存储

### 目录结构

```
app_data_dir/
├── aethermail.db              # SQLite 数据库
└── attachments/               # 附件缓存目录
    ├── att-a1b2c3d4.dat       # 附件缓存文件
    ├── att-e5f6g7h8.dat
    ├── att-i9j0k1l2.dat
    └── ...
```

### 文件命名

- 格式: `att-<attachment_id>.dat`
- `<attachment_id>` 是数据库中的 UUID
- 示例: `att-550e8400-e29b-41d4-a716-446655440000.dat`

### 路径生成

```rust
fn get_attachment_cache_path(app_handle: &AppHandle, attachment_id: &str) -> Result<PathBuf, String> {
    let cache_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("获取应用目录失败: {}", e))?
        .join("attachments");
    
    // 确保目录存在
    std::fs::create_dir_all(&cache_dir)
        .map_err(|e| format!("创建缓存目录失败: {}", e))?;
    
    Ok(cache_dir.join(format!("{}.dat", attachment_id)))
}
```

## 缓存统计

### CacheStats 结构

```rust
#[derive(Serialize)]
pub struct CacheStats {
    pub file_count: u32,       // 缓存文件数量
    pub total_size: u64,       // 总大小（字节）
    pub cache_path: String,    // 缓存目录路径
}
```

### 获取统计

```rust
#[tauri::command]
pub async fn get_attachment_cache_stats(app_handle: AppHandle) -> Result<CacheStats, String> {
    let cache_dir = get_cache_directory(&app_handle)?;
    
    let mut file_count = 0u32;
    let mut total_size = 0u64;
    
    for entry in std::fs::read_dir(&cache_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().map(|e| e == "dat").unwrap_or(false) {
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
```

## 缓存清空

### 清空流程

```rust
#[tauri::command]
pub async fn clear_attachment_cache(app_handle: AppHandle) -> Result<(), String> {
    let db = get_db().await?;
    let cache_dir = get_cache_directory(&app_handle)?;
    
    // 1. 删除所有缓存文件
    for entry in std::fs::read_dir(&cache_dir)? {
        let path = entry?.path();
        if path.is_file() {
            std::fs::remove_file(&path)
                .map_err(|e| format!("删除文件失败: {}", e))?;
        }
    }
    
    // 2. 更新数据库状态
    db.mark_all_attachments_not_downloaded().await?;
    
    Ok(())
}
```

### 数据库更新

```sql
UPDATE attachments SET 
    is_downloaded = 0,
    local_path = NULL;
```

## 打开缓存目录

### 跨平台实现

```rust
#[tauri::command]
pub async fn open_attachment_cache_directory(app_handle: AppHandle) -> Result<(), String> {
    let cache_dir = get_cache_directory(&app_handle)?;
    
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(&cache_dir)
            .spawn()
            .map_err(|e| format!("打开目录失败: {}", e))?;
    }
    
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&cache_dir)
            .spawn()
            .map_err(|e| format!("打开目录失败: {}", e))?;
    }
    
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&cache_dir)
            .spawn()
            .map_err(|e| format!("打开目录失败: {}", e))?;
    }
    
    Ok(())
}
```

## UI 设计

### 缓存管理设置组件

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  附件缓存                                                                    │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  缓存位置                                                                    │
│  C:\Users\...\AppData\Roaming\AetherMail\attachments                        │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │ 打开目录                                                              │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│  缓存统计                                                                    │
│  已缓存附件: 23 个                                                           │
│  占用空间: 156.8 MB                                                          │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │ 清空缓存                                                              │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│  ⚠️ 清空缓存后，附件需要重新从服务器下载                                      │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 清空确认对话框

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  确认清空缓存                                                                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  将删除 23 个缓存文件，共 156.8 MB                                           │
│                                                                             │
│  清空后附件需要重新从服务器下载                                               │
│                                                                             │
│  ┌─────────────┐    ┌─────────────┐                                         │
│  │ 取消        │    │ 确认清空    │                                         │
│  └─────────────┘    └─────────────┘                                         │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## 边界情况

| 场景 | 处理 |
|------|------|
| 缓存目录不存在 | 自动创建 |
| 缓存文件被外部删除 | 重新下载 |
| 缓存文件损坏 | 删除后重新下载 |
| 权限不足无法创建目录 | 返回错误，前端提示 |
| 清空缓存失败 | 显示错误信息 |