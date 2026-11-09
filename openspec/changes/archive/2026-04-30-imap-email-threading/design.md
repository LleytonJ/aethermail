## Context

aethermail 当前邮件展示为扁平列表，用户无法看到同一对话的完整脉络。

邮件的 RFC 2822 标准定义了三个关键头用于建立邮件间关系：
- `Message-ID`: 邮件的唯一标识符
- `In-Reply-To`: 回复邮件指向的原始邮件 Message-ID
- `References`: 完整的引用链（所有祖先邮件的 Message-ID）

这些头信息可以通过 JWZ threading 算法组装成会话树。velo 项目已有成熟的 TypeScript 实现（`threadBuilder.ts`），可直接复用。

## Goals / Non-Goals

**Goals:**
- 仅针对 IMAP 协议邮件实现会话合并
- 提取并存储 Message-ID、In-Reply-To、References 头
- 前端使用 JWZ 算法计算会话分组
- 邮件列表以"会话卡片"形式展示
- 点击会话可展开查看所有关联邮件

**Non-Goals:**
- 不实现 POP3 邮件的会话合并（POP3 通常不提供完整邮件头）
- 不实现高级功能：Pin、Mute、Snooze、AI Summary
- 不处理存量邮件的迁移（仅对新同步邮件生效）
- 不同步会话状态到服务器

## Decisions

### D1: Threading 计算位置 — 前端 TypeScript

**选择**: 在前端 TypeScript 端使用 JWZ 算法计算会话

**理由**:
- velo 的 `threadBuilder.ts` 是纯 TypeScript 实现，零外部依赖
- 可直接复制复用，无需移植到 Rust
- 前端计算后通过 Tauri command 回写 thread_id 到数据库

**替代方案**:
- Rust 端计算：需要移植 JWZ 算法，工作量大，且增加 IPC 复杂度

### D2: 数据库结构 — threads 表 + thread_id 字段

**选择**: 新增独立的 `threads` 表，`emails` 表新增 `thread_id` 外键

**理由**:
- 符合关系数据库设计原则
- threads 表存储会话摘要（subject、last_message_at、message_count）
- 避免每次查询都重新计算会话

**替代方案**:
- 仅在 emails 表加 thread_id，不建 threads 表：查询会话摘要时需要聚合计算，性能差

### D3: Threading 头解析 — 使用 mail-parser

**选择**: 使用已有的 `mail-parser` 库解析 Message-ID 等头

**理由**:
- `commands/email.rs` 已使用 mail-parser 解析邮件
- mail-parser 提供 `message.message_id()`、`message.in_reply_to()`、`message.references()` API
- 无需引入新依赖

## Risks / Trade-offs

### R1: 存量邮件无 threading 头
- **风险**: 用户已有的本地邮件没有存储 Message-ID 等头，无法参与会话计算
- **缓解**: MVP 阶段仅对新同步邮件生效，存量邮件保持扁平展示；后续版本可提供"重新同步"功能

### R2: IMAP 连接断开风险
- **风险**: 当前 IMAP 同步流程已简化（避免多次 fetch 导致连接断开），新增头解析不应增加额外 fetch
- **缓解**: 头解析在现有 `fetch_email(BODY[])` 返回的完整邮件内容中提取，无需额外 IMAP 命令

### R3: Phantom Container 处理
- **风险**: JWZ 算法中的 Phantom Container（引用了不存在邮件）可能导致会话 ID 不稳定
- **缓解**: velo 的实现已处理此问题，使用 root Message-ID 的 djb2 hash 作为 thread ID，保证增量同步的一致性