## Context

当前钓鱼检测在前端使用 OpenAI JavaScript SDK 执行，存在两个核心问题：

1. **时序问题**：邮件先保存到数据库（is_phishing=false），前端检测后才更新状态，导致钓鱼邮件短暂出现在收件箱
2. **重复检测问题**：用户手动移回收件箱的邮件没有标记，下次同步又被检测为钓鱼

项目技术栈：
- Rust 后端（Tauri）：tokio 异步运行时，serde 序列化，sqlx 数据库
- 前端：Vue 3 + Pinia，OpenAI JavaScript SDK

约束：
- 需兼容所有 OpenAI 协议的 API（OpenAI、DeepSeek、阿里百炼等）
- 检测超时不能阻塞邮件同步主流程
- 用户决策（移回收件箱）需持久化

## Goals / Non-Goals

**Goals:**
- 钓鱼邮件从不在收件箱显示（检测和保存原子操作）
- 用户手动处理的邮件不再重复检测
- API Key 在后端处理，安全性提升
- 支持自定义 base_url 兼容国产模型

**Non-Goals:**
- 不改变钓鱼检测的 AI 模型或提示词逻辑
- 不实现流式响应（钓鱼检测用单次请求足够）
- 不实现批量检测优化（当前每次同步只获取少量邮件）

## Decisions

### 1. 使用 async-openai crate

**选择**：使用 `async-openai` crate（而非手动 HTTP 请求）

**理由**：
- 类型安全，无需手写 JSON 序列化
- 已有 tokio/serde 依赖，无缝集成
- 支持自定义 base_url
- 社区活跃，维护良好

**替代方案**：
- reqwest 手写 HTTP → 需要手动处理 JSON，易出错
- openai crate（非 async）→ 与 tokio 不兼容

### 2. 检测时机：保存前而非保存后

**选择**：在 `sync_emails` 函数中，下载邮件后立即检测，检测完成后再保存

**理由**：
- 原子操作：数据库记录直接保存为正确状态
- 用户体验：钓鱼邮件从未出现

**流程**：
```
1. 从服务器下载新邮件
2. 解析邮件内容
3. 调用 AI 检测（带超时）
4. 根据检测结果设置 is_phishing
5. 保存到数据库
6. 返回结果给前端
```

### 3. user_overridden 字段

**选择**：新增 `user_overridden: boolean` 字段

**用途**：
- 用户手动将钓鱼邮件移回收件箱时设为 true
- 同步时跳过 user_overridden=true 的邮件检测

**存储**：emails 表新增字段，与 is_phishing 一起管理

### 4. 超时处理

**选择**：5 秒超时，检测失败时保存为非钓鱼邮件

**理由**：
- 不阻塞同步流程
- 安全是非钓鱼（用户可手动标记）

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| AI API 调用增加同步时间 | 设置合理超时（5s），用户可看到进度提示 |
| async-openai 版本兼容性 | 锁定版本，测试多家 API |
| 检测失败导致钓鱼邮件漏检 | 前端保留手动标记功能，用户可补救 |
| 并发检测导致 API rate limit | 当前每次同步只获取少量邮件，暂不需要批量优化 |

## Migration Plan

1. **添加依赖**：Cargo.toml 添加 async-openai
2. **新增模块**：创建 src-tauri/src/ai/client.rs
3. **数据库迁移**：新增 user_overridden 字段
4. **重构 sync_emails**：插入钓鱼检测逻辑
5. **前端简化**：移除 aiService.ts 的 API 调用
6. **测试验证**：多 API provider 兼容性测试

**Rollback**：如出现问题，可快速回退到前端检测（保留现有代码）

## Open Questions

- 是否需要支持 Mock 模式（当前前端有 MOCK_MODE=true）？
  → 建议：保留，用于开发测试