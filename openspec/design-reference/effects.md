# 视觉效果

## Glass 毛玻璃效果

项目使用 Glass 效果增加界面层次感。

### 变量定义

| 变量 | 值 | 描述 |
|------|-----|------|
| `--glass-blur` | `20px` | 模糊程度 |
| `--glass-shadow` | `0 8px 32px rgba(0, 0, 0, 0.2)` | 阴影 |
| `--glass-highlight` | `inset 0 1px 0 0 rgba(255, 255, 255, 0.06)` | 高光 |

### 使用方式

```html
<div class="glass-panel">毛玻璃面板</div>
```

```css
.glass-panel {
  backdrop-filter: blur(var(--glass-blur));
  -webkit-backdrop-filter: blur(var(--glass-blur));
  box-shadow: var(--glass-shadow), var(--glass-highlight);
}
```

### 应用场景

- 侧边栏 `.glass-panel`
- 邮件列表面板
- 阅读面板
- Modal 背景

---

## 动态背景 Blob

页面背景有四个动态模糊圆形，增加视觉活力。

### 结构

```html
<div class="animated-bg">
  <div class="blob" />  <!-- 蓝色 -->
  <div class="blob" />  <!-- 紫色 -->
  <div class="blob" />  <!-- 粉色 -->
  <div class="blob" />  <!-- 绿色 -->
</div>
```

### 特性

- 固定定位，z-index: -1
- 圆形，blur(80px)，opacity: 0.3
- 使用 `blobMove` 关键帧动画
- 不同 blob 有不同的颜色、大小、位置、动画时长

---

## 动画系统

### 时长变量

| 变量 | 值 | 用途 |
|------|-----|------|
| `--anim-fast` | `150ms` | 快速交互（按钮、hover） |
| `--anim-normal` | `200ms` | 常规过渡（Modal、下拉） |
| `--anim-slow` | `300ms` | 缓慢动画（页面切换） |

### 关键帧动画

| 名称 | 效果 | 用途 |
|------|------|------|
| `fadeIn` | 透明度 0 → 1 | 淡入 |
| `slideUp` | Y: 16px → 0 | 从下往上滑入 |
| `slideDown` | Y: -16px → 0 | 从上往下滑入 |
| `scaleIn` | scale(0.95) + opacity 0 → 1 | 缩放淡入 |
| `starPop` | scale 1 → 1.3 → 1 | 星标弹跳 |

### 使用示例

```css
.element {
  animation: fadeIn var(--anim-normal) ease;
}
```

---

## 交互效果类

### .hover-lift

悬停时轻微上浮 + 阴影。

```html
<div class="hover-lift">悬停上浮</div>
```

### .press-scale

点击时轻微缩小。

```html
<button class="press-scale">点击缩小</button>
```

### .interactive-btn

综合交互效果：悬停上浮 + 点击缩小。

```html
<button class="interactive-btn">交互按钮</button>
```

---

## Vue Transition 组件

### Modal 过渡

```css
.modal-enter-active,
.modal-leave-active {
  transition: all 0.2s ease;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}

.modal-enter-from .relative,
.modal-leave-to .relative {
  opacity: 0;
  transform: scale(0.95) translateY(-10px);
}
```

### 下拉菜单过渡

```css
.dropdown-enter-active,
.dropdown-leave-active {
  transition: all 0.15s ease;
}

.dropdown-enter-from,
.dropdown-leave-to {
  opacity: 0;
  transform: translateY(-4px);
}
```

---

## 滚动条样式

### 隐藏滚动条

```html
<div class="hide-scrollbar">内容</div>
```

### 自定义滚动条

```css
::-webkit-scrollbar {
  width: 6px;
  height: 6px;
}

::-webkit-scrollbar-track {
  background: transparent;
}

::-webkit-scrollbar-thumb {
  background-color: var(--color-text-tertiary);
  border-radius: 3px;
}

::-webkit-scrollbar-thumb:hover {
  background-color: var(--color-text-secondary);
}
```

---

## 无障碍

### prefers-reduced-motion

尊重用户的减少动画偏好：

```css
@media (prefers-reduced-motion: reduce) {
  *,
  *::before,
  *::after {
    animation: none !important;
    transition-duration: 0.01ms !important;
  }

  .animated-bg {
    display: none;
  }
}
```