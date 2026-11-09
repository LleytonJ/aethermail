## Context

AetherMail 是一个基于 Tauri + Vue 3 的桌面邮件客户端，目前支持 POP3/IMAP 收件和 SMTP 发件。后端使用 Rust + sqlx (SQLite)，前端使用 Vue 3 + Pinia + TypeScript。

当前邮件存储流程（POP3/IMAP 同步）在 `save_email` 时保存邮件元数据，但不记录联系人信息。用户发送邮件时需要手动输入完整邮箱地址，缺乏自动补全支持。

### 技术约束

- 数据库迁移采用渐进式 `ALTER TABLE` 方式（参考现有 `run_migrations`）
- Tauri 命令使用 `#[tauri::command]` 注解，返回 `Result<T, String>`
- 前端状态管理使用 Pinia store
- 需兼容现有邮件同步流程（POP3/IMAP）

## Goals / Non-Goals

**Goals:**

1. 自动收集联系人：邮件同步时自动创建/更新联系人记录
2. 联系人管理：支持查看、搜索、编辑联系人基本信息
3. 地址自动补全：发件时提供基于频率排序的联系人建议
4. 数据持久化：使用 SQLite 存储，支持数据库迁移

**Non-Goals:**

- 头像获取（Gravatar/Libravatar）暂不实现，仅支持本地存储 URL
- VIP 标记、联系人分组等高级功能
- 联系人导入/导出（CSV/vCard）
- 联系人详情侧边栏（统计、附件历史等）

## Decisions

### D1: 联系人数据模型

参考 velo 项目设计，采用以下表结构：

```sql
CREATE TABLE contacts (
  id TEXT PRIMARY KEY,
  email TEXT NOT NULL UNIQUE,
  display_name TEXT,
  avatar_url TEXT,
  frequency INTEGER DEFAULT 1,
  last_contacted_at INTEGER,
  created_at INTEGER DEFAULT (unixepoch()),
  updated_at INTEGER DEFAULT (unixepoch())
);
CREATE INDEX idx_contacts_email ON contacts(email);
CREATE INDEX idx_contacts_frequency ON contacts(frequency DESC);
```

**设计考量：**
- `email` 作为唯一标识，使用 `UNIQUE` 约束
- `frequency` 用于智能排序，每次邮件交互时 +1
- `last_contacted_at` 记录最后联系时间，可用于后续高级排序

### D2: Upsert 触发时机

选择在邮件保存时（`save_email`）触发联系人 upsert，而非在邮件列表渲染时。

**理由：**
- 数据库层操作更可靠，避免前端状态丢失
- 批量同步时性能更好
- 不依赖用户是否查看邮件

**实现方式：**
在 `Database::save_email` 方法末尾调用 `upsert_contact(from_address, from_name)`。

### D3: 自动补全排序策略

采用 **频率优先 + 名称次优** 的排序策略：

```sql
ORDER BY frequency DESC, display_name ASC
```

**理由：**
- 用户更常联系的人应该排在前面
- 相同频率时按名称排序，便于定位

### D4: 前端组件设计

| 组件 | 位置 | 功能 |
|------|------|------|
| `AddressInput.vue` | `components/ui/` | 自动补全输入框，用于收件人输入 |
| `ContactEditor.vue` | `components/settings/` | 联系人编辑界面，嵌入设置页 |
| `contactsStore` | `stores/` | Pinia store，管理联系人状态 |

**AddressInput 设计参考 velo 的实现：**
- 输入 2 字符后触发搜索
- 200ms debounce
- 支持键盘导航（↑↓ Enter Tab）
- 支持多地址输入（标签式）

### D5: 后端架构

新增模块结构：

```
src-tauri/src/
├── storage/
│   ├── models.rs      # 新增 DbContact 结构体
│   └── database.rs    # 新增 contacts 表迁移和 CRUD 方法
├── commands/
│   ├── mod.rs         # 导出 contact 模块
│   └── contact.rs     # 新增 Tauri 命令
```

**Tauri 命令列表：**

| 命令 | 功能 |
|------|------|
| `search_contacts` | 搜索联系人（自动补全） |
| `get_all_contacts` | 获取所有联系人 |
| `get_contact_by_email` | 根据邮箱获取联系人 |
| `update_contact` | 更新联系人信息 |
| `delete_contact` | 删除联系人 |

## Risks / Trade-offs

### R1: 邮箱地址规范化

**风险：** 用户可能输入 `John@example.com` 和 `john@example.com`，被视为不同联系人。

**缓解：** 在 upsert 和查询时统一使用小写规范化。

### R2: 邮件同步性能影响

**风险：** 每封邮件保存时额外执行联系人 upsert，可能影响批量同步性能。

**缓解：** 使用 `INSERT ... ON CONFLICT DO UPDATE` 单语句 upsert，避免先查询后插入。

### R3: 显示名称来源优先级

**风险：** 同一联系人不同邮件可能使用不同显示名称。

**缓解：** 采用 `COALESCE(new_name, existing_name)` 策略，优先保留用户手动设置的名称，自动获取的名称仅作补充。