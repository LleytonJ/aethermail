# 组件模式

## Modal 模态框

### 基础结构

```html
<Teleport to="body">
  <Transition name="modal">
    <div v-if="isOpen" class="fixed inset-0 z-50 flex items-center justify-center p-4">
      <!-- Backdrop -->
      <div class="absolute inset-0 bg-black/60 backdrop-blur-sm"
           @click="handleBackdropClick" />

      <!-- Modal Content -->
      <div class="relative bg-bg-primary border border-border-primary
                  rounded-xl shadow-2xl max-h-[90vh] overflow-hidden">
        <!-- Header -->
        <div class="flex items-center justify-between px-4 py-3
                    border-b border-border-primary">
          <h2 class="text-lg font-semibold text-text-primary">{{ title }}</h2>
          <button @click="close">
            <X :size="18" />
          </button>
        </div>

        <!-- Body -->
        <div class="overflow-y-auto max-h-[calc(90vh-60px)]">
          <slot />
        </div>
      </div>
    </div>
  </Transition>
</Teleport>
```

### 宽度选项

| width prop | class | 实际宽度 |
|------------|-------|----------|
| `sm` | `max-w-sm` | 384px |
| `md` | `max-w-md` | 448px |
| `lg` | `max-w-lg` | 512px |
| `xl` | `max-w-xl` | 576px |
| `2xl` | `max-w-2xl` | 672px |
| `full` | `max-w-4xl` | 896px |

### 过渡动画

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

### 使用示例

```html
<Modal :is-open="showModal" title="标题" width="lg" @close="showModal = false">
  <div>Modal 内容</div>
</Modal>
```

---

## 确认对话框 (ConfirmDialog)

### 基础结构

```html
<Teleport to="body">
  <Transition name="confirm">
    <div v-if="isOpen" class="fixed inset-0 z-50 flex items-center justify-center p-4">
      <div class="absolute inset-0 bg-black/60 backdrop-blur-sm" @click="cancel" />

      <div class="relative bg-bg-primary border border-border-primary
                  rounded-xl shadow-2xl w-full max-w-sm">
        <!-- Header -->
        <div class="px-4 py-3 border-b border-border-primary">
          <h2 class="text-base font-semibold text-text-primary">{{ title }}</h2>
        </div>

        <!-- Body -->
        <div class="px-4 py-4">
          <p class="text-sm text-text-secondary">{{ message }}</p>
        </div>

        <!-- Footer -->
        <div class="flex justify-end gap-2 px-4 py-3 border-t border-border-primary
                    bg-bg-secondary/30">
          <button class="text-text-secondary" @click="cancel">{{ cancelText }}</button>
          <button :class="danger ? 'bg-danger' : 'bg-accent'" @click="confirm">
            {{ confirmText }}
          </button>
        </div>
      </div>
    </div>
  </Transition>
</Teleport>
```

### Props

| prop | 类型 | 默认值 | 描述 |
|------|------|--------|------|
| `isOpen` | boolean | - | 是否显示 |
| `title` | string | '确认' | 标题 |
| `message` | string | '确定要执行此操作吗？' | 提示内容 |
| `confirmText` | string | '确认' | 确认按钮文字 |
| `cancelText` | string | '取消' | 取消按钮文字 |
| `danger` | boolean | false | 是否为危险操作（红色确认按钮） |

---

## 卡片列表

用于账户管理等场景。

### 基础结构

```html
<div class="space-y-3">
  <div v-for="item in items" :key="item.id"
       class="flex items-center justify-between py-2.5 px-4
              bg-bg-secondary rounded-lg border border-border-primary">
    <!-- 左侧内容 -->
    <div class="flex items-center gap-3 min-w-0">
      <!-- 头像 -->
      <div class="w-9 h-9 rounded-full bg-accent/15 text-accent
                  flex items-center justify-center shrink-0 text-sm font-semibold">
        {{ initial }}
      </div>
      <!-- 详情 -->
      <div class="min-w-0">
        <div class="text-sm font-medium text-text-primary truncate">
          {{ name }}
        </div>
        <div class="text-xs text-text-tertiary truncate">
          {{ description }}
        </div>
      </div>
    </div>

    <!-- 右侧操作 -->
    <div class="flex items-center gap-2 shrink-0">
      <button class="text-xs text-accent hover:text-accent-hover">编辑</button>
      <button class="text-xs text-danger hover:text-danger-hover">删除</button>
    </div>
  </div>
</div>
```

### 单项样式要点

- 容器：`flex items-center justify-between py-2.5 px-4 bg-bg-secondary rounded-lg border border-border-primary`
- 左侧：`flex items-center gap-3 min-w-0`（min-w-0 确保文字截断生效）
- 头像：`w-9 h-9 rounded-full bg-accent/15 text-accent`
- 操作按钮：`text-xs text-accent` 或 `text-xs text-danger`

---

## 表单输入

### 文本输入框

```html
<div>
  <label for="field" class="block text-xs font-medium text-text-secondary mb-1">
    标签
  </label>
  <input
    id="field"
    v-model="value"
    type="text"
    placeholder="占位符"
    class="w-full px-3 py-2 bg-bg-secondary border border-border-primary
           rounded-lg text-sm text-text-primary outline-none
           focus:border-accent transition-colors"
  />
</div>
```

### 下拉选择

```html
<select class="w-full px-3 py-2 bg-bg-secondary border border-border-primary
               rounded-lg text-sm text-text-primary outline-none
               focus:border-accent transition-colors appearance-none">
  <option value="a">选项 A</option>
  <option value="b">选项 B</option>
</select>
```

### 错误状态

```html
<input class="... border-danger" />
<p class="text-xs text-danger mt-1">{{ errorMessage }}</p>
```

---

## Toast 通知

### 结构

```html
<Transition name="toast">
  <div v-if="show" class="fixed bottom-4 left-1/2 -translate-x-1/2
                        px-4 py-2 rounded-lg shadow-lg z-50"
       :class="type === 'success' ? 'bg-success text-white' : 'bg-danger text-white'">
    {{ message }}
  </div>
</Transition>
```

### 过渡

```css
.toast-enter-active,
.toast-leave-active {
  transition: all 0.3s ease;
}

.toast-enter-from,
.toast-leave-to {
  opacity: 0;
  transform: translate(-50%, 20px);
}
```

---

## 头像组件

### 尺寸

| class | 尺寸 | 字号 |
|-------|------|------|
| `avatar-sm` | 28px | 11px |
| `avatar-md` | 36px | 14px |
| `avatar-lg` | 42px | 16px |

### 样式

```css
.avatar {
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  background-color: var(--color-accent);
  color: white;
  font-weight: 600;
}
```

---

## 徽章 (Badge)

```html
<span class="badge">3</span>
```

```css
.badge {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 18px;
  height: 18px;
  padding: 0 5px;
  border-radius: 9px;
  background-color: var(--color-accent);
  color: white;
  font-size: 10px;
  font-weight: 600;
}
```

---

## 标签 (Tag)

```html
<span class="tag">标签名</span>
```

```css
.tag {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  padding: 2px 8px;
  border-radius: 9999px;
  font-size: 11px;
  font-weight: 500;
}
```