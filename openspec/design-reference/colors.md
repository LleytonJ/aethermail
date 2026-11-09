# 颜色系统

基于 Tailwind CSS v4 的 `@theme` 配置，定义在 `src/styles/globals.css` 中。

## 背景色

| 变量 | 值 | 用途 |
|------|-----|------|
| `--color-bg-primary` | `rgba(15, 23, 42, 0.92)` | 主背景、面板背景 |
| `--color-bg-secondary` | `rgba(30, 41, 59, 0.85)` | 次级背景、卡片背景 |
| `--color-bg-tertiary` | `rgba(51, 65, 85, 0.75)` | 输入框、下拉菜单背景 |
| `--color-bg-hover` | `rgba(51, 65, 85, 0.55)` | 悬停状态背景 |
| `--color-bg-selected` | `rgba(99, 102, 241, 0.15)` | 选中状态背景 |
| `--color-bg-card` | `rgba(30, 41, 59, 0.9)` | 卡片背景 |

### 使用方式

```html
<div class="bg-bg-primary">主背景</div>
<div class="bg-bg-secondary">次级背景</div>
<div class="hover:bg-bg-hover">悬停效果</div>
```

## 文字颜色

| 变量 | 值 | 用途 |
|------|-----|------|
| `--color-text-primary` | `#f8fafc` | 主要文字、标题 |
| `--color-text-secondary` | `#cbd5e1` | 次级文字、描述 |
| `--color-text-tertiary` | `#94a3b8` | 辅助文字、提示 |
| `--color-text-muted` | `#64748b` | 禁用状态、占位符 |

### 使用方式

```html
<span class="text-text-primary">主要文字</span>
<span class="text-text-secondary">描述文字</span>
<span class="text-text-tertiary">提示文字</span>
```

## 边框颜色

| 变量 | 值 | 用途 |
|------|-----|------|
| `--color-border-primary` | `rgba(255, 255, 255, 0.1)` | 主要边框、分隔线 |
| `--color-border-secondary` | `rgba(255, 255, 255, 0.06)` | 次级边框 |

### 使用方式

```html
<div class="border border-border-primary">带边框的容器</div>
<div class="border-b border-border-primary">底部边框</div>
```

## 强调色

| 变量 | 值 | 用途 |
|------|-----|------|
| `--color-accent` | `#818cf8` | 主强调色、按钮、链接 |
| `--color-accent-hover` | `#6366f1` | 强调色悬停状态 |
| `--color-accent-light` | `#312e81` | 强调色浅色背景 |

### 使用方式

```html
<button class="bg-accent hover:bg-accent-hover text-white">按钮</button>
<span class="text-accent">链接</span>
<div class="bg-accent/15">浅色背景</div>
```

## 功能色

| 变量 | 值 | 用途 |
|------|-----|------|
| `--color-danger` | `#ef4444` | 错误、删除 |
| `--color-warning` | `#f59e0b` | 警告 |
| `--color-success` | `#10b981` | 成功、完成 |
| `--color-info` | `#3b82f6` | 信息 |

### 使用方式

```html
<button class="text-danger hover:text-danger-hover">删除</button>
<span class="text-success">操作成功</span>
<div class="bg-warning/15 text-warning">警告提示</div>
```

## 侧边栏专用色

| 变量 | 值 | 用途 |
|------|-----|------|
| `--color-sidebar-bg` | `rgba(15, 23, 42, 0.92)` | 侧边栏背景 |
| `--color-sidebar-text` | `#e2e8f0` | 侧边栏文字 |
| `--color-sidebar-hover` | `rgba(30, 41, 59, 0.7)` | 侧边栏悬停 |

## 全局背景

页面背景使用渐变色，定义在 `body` 样式中：

```css
background: linear-gradient(
  135deg,
  #0c1222 0%,
  #151030 35%,
  #1a0a2e 65%,
  #0f172a 100%
);
background-attachment: fixed;
```