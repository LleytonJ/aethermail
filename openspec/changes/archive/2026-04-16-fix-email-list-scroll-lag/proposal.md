## Why

邮件列表滚动时存在明显的卡顿（鼠标滚动不跟手），影响用户体验。根本原因是 MailLayout 中三个面板同时使用了 `glass-panel` 的 `backdrop-filter: blur(10px)`，其中 EmailList 是滚动区域，每帧滚动时都需要重新计算背景模糊，GPU 开销巨大。

## What Changes

- 移除 EmailList 区域的 `glass-panel` blur 效果
- 保留 Sidebar 和 ReadingPane 的 blur 效果（它们不滚动，开销小）
- 新增替代样式确保 EmailList 区域视觉一致性

## Capabilities

### New Capabilities

无新增能力，这是纯性能优化。

### Modified Capabilities

无需求层面的变更，仅涉及 CSS 实现细节调整。

## Impact

- **代码影响**: `MailLayout.vue`（移除 EmailList 的 glass-panel 类），`globals.css`（新增替代样式）
- **视觉影响**: EmailList 区域将不再有背景模糊效果，但保持边框和阴影的一致性
- **性能影响**: 滚动流畅度将显著提升，GPU 合成层减少两个