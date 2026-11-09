# 布局模式

## 三栏邮件布局 (MailLayout)

主界面的核心布局，用于邮件列表和阅读。

### 结构

```
┌────────────────────────────────────────────────────────────────────┐
│  TitleBar (原生标题栏，由 Tauri 提供)                                │
├──────────┬──────────────────┬───────────────────────────────────────┤
│ Sidebar  │    EmailList     │           ReadingPane                 │
│ (200px)  │     (320px)      │            (flex-1)                   │
│          │                  │                                        │
│ 可折叠   │   固定宽度        │     自适应剩余空间                      │
└──────────┴──────────────────┴───────────────────────────────────────┘
```

### 代码结构

```html
<div class="flex flex-col h-screen overflow-hidden">
  <!-- 标题栏 -->
  <TitleBar />

  <!-- 主内容区 -->
  <div class="flex flex-1 min-w-0 overflow-hidden">
    <!-- 侧边栏 -->
    <aside class="flex-shrink-0 border-r border-border-primary glass-panel"
           :style="{ width: collapsed ? '56px' : '200px' }">
      <Sidebar />
    </aside>

    <!-- 邮件列表 -->
    <div class="flex-shrink-0 border-r border-border-primary glass-panel"
         style="width: 320px">
      <EmailList />
    </div>

    <!-- 阅读面板 -->
    <main class="flex-1 overflow-hidden glass-panel">
      <ReadingPane />
    </main>
  </div>
</div>
```

### 关键样式

- 侧边栏：`flex-shrink-0` + 可变宽度（折叠时 56px，展开 200px）
- 邮件列表：`flex-shrink-0` + 固定 320px
- 阅读面板：`flex-1` 自适应
- 所有面板：`glass-panel` + `border-r border-border-primary`

---

## 设置页面布局 (SettingsPage)

设置页面采用左侧导航 + 右侧内容区布局（参考 velo 设计）。

### 结构

```
┌────────────────────────────────────────────────────────────────────┐
│  Header: [← 返回] 设置                                              │
├────────────┬───────────────────────────────────────────────────────┤
│  Nav       │  Content (max-w-2xl)                                   │
│  (w-48)    │  最大宽度限制，提升阅读体验                              │
│  192px     │                                                        │
│            │                                                        │
│  ○ 账户管理 │  [Tab Title]                                           │
│  ○ 通用设置 │  ─────────────────────────────                         │
│            │  内容区域                                               │
└────────────┴───────────────────────────────────────────────────────┘
```

### 代码结构

```html
<div class="flex-1 flex flex-col min-w-0 overflow-hidden bg-bg-primary/50">
  <!-- Header -->
  <div class="flex items-center gap-3 px-5 py-3 border-b border-border-primary
              shrink-0 bg-bg-primary/60 backdrop-blur-sm">
    <button @click="goBack">
      <ArrowLeft :size="18" />
    </button>
    <h1>设置</h1>
  </div>

  <!-- Body -->
  <div class="flex flex-1 min-h-0">
    <!-- Nav -->
    <nav class="w-48 border-r border-border-primary py-2 overflow-y-auto
                shrink-0 bg-bg-primary/30">
      <!-- 导航项 -->
    </nav>

    <!-- Content -->
    <div class="flex-1 overflow-y-auto">
      <div class="max-w-2xl px-8 py-6">
        <router-view />
      </div>
    </div>
  </div>
</div>
```

### 导航项样式

```html
<button :class="[
  'flex items-center gap-2.5 w-full px-4 py-2 text-[0.8125rem] transition-colors',
  isActive
    ? 'bg-bg-selected text-accent font-medium'
    : 'text-text-secondary hover:bg-bg-hover hover:text-text-primary'
]">
  <Icon :size="15" />
  标签
</button>
```

### 关键样式

- 导航宽度：`w-48` (192px)
- 内容最大宽度：`max-w-2xl` (672px)
- 内容区：`flex-1 overflow-y-auto`
- 导航项激活：`bg-bg-selected text-accent font-medium`

---

## 侧边栏内部布局

### 结构

```
┌──────────────┐
│ 账户切换      │  p-2
├──────────────┤
│ 撰写按钮      │  px-2 py-1.5
├──────────────┤
│ 文件夹列表    │  flex-1 overflow-y-auto
│              │
│ 标签分组      │  (可选，展开时显示)
│              │
├──────────────┤
│ 设置 + 折叠   │  p-2 border-t
└──────────────┘
```

### 代码结构

```html
<div class="flex flex-col h-full bg-sidebar-bg text-sidebar-text">
  <!-- 账户切换 -->
  <div class="p-2">
    <AccountSwitcher />
  </div>

  <!-- 撰写按钮 -->
  <div class="px-2 py-1.5">
    <button class="w-full h-9 bg-accent text-white rounded-lg">
      撰写
    </button>
  </div>

  <!-- 文件夹列表 -->
  <nav class="flex-1 overflow-y-auto py-1 hide-scrollbar">
    <FolderItem v-for="folder in folders" />
    <!-- 标签分组 -->
    <TagList v-if="!collapsed" />
  </nav>

  <!-- 底部 -->
  <div class="p-2 border-t border-border-primary flex items-center gap-0.5">
    <button>设置</button>
    <button @click="toggleSidebar">折叠</button>
  </div>
</div>
```

---

## 响应式考虑

当前项目主要针对桌面端设计，暂未实现移动端响应式。

### 当前断点

- 侧边栏折叠：56px
- 侧边栏展开：200px
- 邮件列表：320px
- 设置导航：192px (w-48)
- 设置内容：max-w-2xl (672px)