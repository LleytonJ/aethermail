## Context

当前 MailLayout 采用三列布局，每个面板都使用 `glass-panel` 类：

```vue
<!-- MailLayout.vue -->
<aside class="glass-panel">          <!-- Sidebar -->
<div class="glass-panel">            <!-- EmailList 区域 -->
<main class="glass-panel">           <!-- ReadingPane -->
```

`glass-panel` 定义：
```css
.glass-panel {
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
}
```

问题：EmailList 是唯一滚动区域，每帧滚动时 GPU 需重新计算 blur，导致帧率下降、滚动不跟手。

## Goals / Non-Goals

**Goals:**
- 移除 EmailList 区域的 backdrop-filter，消除滚动时的 GPU 开销
- 保持视觉一致性（边框、阴影、背景色）
- 保留 Sidebar 和 ReadingPane 的 blur 效果

**Non-Goals:**
- 不引入虚拟滚动（当前邮件数量不多，不是主要瓶颈）
- 不修改其他 CSS 动画（hover-lift 等）
- 不重构整体布局结构

## Decisions

### 决策 1：只移除 EmailList 的 blur

**方案比较：**
| 方案 | 优点 | 缺点 |
|-----|-----|-----|
| 移除所有 blur | 性能最好 | 丧失 glass 视觉效果 |
| 只移除 EmailList blur | 性能显著提升 + 保持 glass 感 | EmailList 视觉略有差异 |
| 降低 blur 强度 | 保持所有 blur | 改善有限 |

**选择**：只移除 EmailList blur。原因：它是唯一滚动区域，是性能瓶颈所在。

### 决策 2：使用替代样式而非直接移除 glass-panel

**方案比较：**
| 方案 | 优点 | 缺点 |
|-----|-----|-----|
| 直接移除 glass-panel | 简单 | 失去 box-shadow |
| 新增 solid-panel 类 | 样式可控 | 需新增 CSS |

**选择**：新增 `solid-panel` 类替代 glass-panel。保持边框和阴影一致性。

## Risks / Trade-offs

- **视觉差异**: EmailList 无模糊效果，与 Sidebar/ReadingPane 有轻微视觉差异 → 可接受，用户滚动时注意力在内容而非背景
- **代码改动小**: 仅涉及一个 Vue 文件和 CSS 文件 → 低风险