<template>
  <div class="rich-text-editor" :class="{ 'opacity-50 pointer-events-none': !editable }">
    <!-- Toolbar -->
    <div v-if="editable && showToolbar" class="toolbar flex items-center gap-1 p-2 border-b border-border-primary bg-bg-secondary">
      <!-- 文字格式组 -->
      <button
        type="button"
        :class="['toolbar-btn', { active: editor?.isActive('bold') }]"
        title="加粗"
        @click="editor?.chain().focus().toggleBold().run()"
      >
        <Bold :size="16" />
      </button>
      <button
        type="button"
        :class="['toolbar-btn', { active: editor?.isActive('italic') }]"
        title="斜体"
        @click="editor?.chain().focus().toggleItalic().run()"
      >
        <Italic :size="16" />
      </button>
      <button
        type="button"
        :class="['toolbar-btn', { active: editor?.isActive('underline') }]"
        title="下划线"
        @click="editor?.chain().focus().toggleUnderline().run()"
      >
        <Underline :size="16" />
      </button>
      <button
        type="button"
        :class="['toolbar-btn', { active: editor?.isActive('strike') }]"
        title="删除线"
        @click="editor?.chain().focus().toggleStrike().run()"
      >
        <Strikethrough :size="16" />
      </button>

      <div class="w-px h-4 bg-border-primary mx-1" />

      <!-- 标题组 -->
      <button
        type="button"
        :class="['toolbar-btn text-xs font-semibold', { active: editor?.isActive('heading', { level: 1 }) }]"
        title="标题 1"
        @click="editor?.chain().focus().toggleHeading({ level: 1 }).run()"
      >
        H1
      </button>
      <button
        type="button"
        :class="['toolbar-btn text-xs font-semibold', { active: editor?.isActive('heading', { level: 2 }) }]"
        title="标题 2"
        @click="editor?.chain().focus().toggleHeading({ level: 2 }).run()"
      >
        H2
      </button>
      <button
        type="button"
        :class="['toolbar-btn text-xs font-semibold', { active: editor?.isActive('heading', { level: 3 }) }]"
        title="标题 3"
        @click="editor?.chain().focus().toggleHeading({ level: 3 }).run()"
      >
        H3
      </button>

      <div class="w-px h-4 bg-border-primary mx-1" />

      <!-- 列表组 -->
      <button
        type="button"
        :class="['toolbar-btn', { active: editor?.isActive('bulletList') }]"
        title="无序列表"
        @click="editor?.chain().focus().toggleBulletList().run()"
      >
        <List :size="16" />
      </button>
      <button
        type="button"
        :class="['toolbar-btn', { active: editor?.isActive('orderedList') }]"
        title="有序列表"
        @click="editor?.chain().focus().toggleOrderedList().run()"
      >
        <ListOrdered :size="16" />
      </button>

      <div class="w-px h-4 bg-border-primary mx-1" />

      <!-- 链接/图片组 -->
      <button
        type="button"
        :class="['toolbar-btn', { active: editor?.isActive('link') }]"
        title="链接"
        @click="handleLink"
      >
        <Link :size="16" />
      </button>
      <button
        type="button"
        class="toolbar-btn"
        title="插入图片"
        @click="imageInputRef?.click()"
      >
        <ImageIcon :size="16" />
      </button>
      <!-- 隐藏的图片文件输入 -->
      <input
        ref="imageInputRef"
        type="file"
        accept="image/*"
        class="hidden"
        @change="handleImageSelect"
      />

      <div class="w-px h-4 bg-border-primary mx-1" />

      <!-- 颜色组 -->
      <div class="color-picker-container" ref="colorPickerRef">
        <button
          type="button"
          :class="['toolbar-btn', { active: hasActiveColor }]"
          title="文字颜色"
          @click="toggleColorPanel"
        >
          <Palette :size="16" />
          <span v-if="currentColor" class="color-indicator" :style="{ backgroundColor: currentColor }"></span>
        </button>
        <!-- 颜色选择面板 -->
        <div v-if="showColorPanel" class="color-panel">
          <div class="preset-colors">
            <button
              v-for="color in presetColors"
              :key="color"
              type="button"
              :class="['color-btn', { active: currentColor === color }]"
              :style="{ backgroundColor: color }"
              :title="color"
              @click="applyColor(color)"
            />
          </div>
          <div class="divider" />
          <button
            type="button"
            class="clear-color-btn"
            @click="clearColor"
          >
            清除颜色
          </button>
          <div class="custom-color">
            <label>自定义:</label>
            <input
              type="color"
              :value="currentColor || '#000000'"
              @input="applyColor(($event.target as HTMLInputElement).value)"
            />
          </div>
        </div>
      </div>
    </div>

    <!-- Editor Content -->
    <editor-content
      :editor="editor"
      class="editor-content"
    />

    <!-- Link Dialog -->
    <InputDialog
      :is-open="showLinkDialog"
      title="插入链接"
      placeholder="输入链接地址..."
      confirm-text="插入"
      cancel-text="取消"
      :initial-value="linkUrl"
      close-on-backdrop
      @confirm="confirmLink"
      @cancel="cancelLink"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onBeforeUnmount, computed, onMounted, onUnmounted } from 'vue'
import { useEditor, EditorContent } from '@tiptap/vue-3'
import StarterKit from '@tiptap/starter-kit'
import Image from '@tiptap/extension-image'
import Placeholder from '@tiptap/extension-placeholder'
import { TextStyle } from '@tiptap/extension-text-style'
import { Color } from '@tiptap/extension-color'
import { Bold, Italic, Underline, Strikethrough, List, ListOrdered, Link, Image as ImageIcon, Palette } from 'lucide-vue-next'
import InputDialog from './InputDialog.vue'

interface Props {
  modelValue?: string
  placeholder?: string
  minHeight?: number
  editable?: boolean
  showToolbar?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: '',
  placeholder: '',
  minHeight: 150,
  editable: true,
  showToolbar: true
})

const emit = defineEmits<{
  'update:modelValue': [value: string]
}>()

// 预设颜色
const presetColors = [
  '#FF0000', // 红色
  '#0000FF', // 蓝色
  '#008000', // 绿色
  '#800080', // 紫色
  '#FFA500', // 橙色
  '#4B0082', // 黛色
]

// 颜色面板状态
const showColorPanel = ref(false)
const colorPickerRef = ref<HTMLElement | null>(null)

// 链接对话框状态
const showLinkDialog = ref(false)
const linkUrl = ref('')
const imageInputRef = ref<HTMLInputElement | null>(null)

// 创建编辑器实例
const editor = useEditor({
  content: props.modelValue,
  editable: props.editable,
  extensions: [
    StarterKit.configure({
      heading: { levels: [1, 2, 3] },  // 启用 H1-H3
      codeBlock: false,
      blockquote: false,
      horizontalRule: false
    }),
    TextStyle, // 必须在 Color 之前
    Color.configure({
      types: ['textStyle']
    }),
    Image.configure({
      inline: true,
      allowBase64: true
    }),
    Placeholder.configure({
      placeholder: props.placeholder
    })
  ],
  onUpdate: ({ editor }) => {
    emit('update:modelValue', editor.getHTML())
  }
})

// 当前颜色状态
const currentColor = computed(() => {
  if (!editor.value) return null
  return editor.value.getAttributes('textStyle').color || null
})

const hasActiveColor = computed(() => {
  return currentColor.value !== null
})

// 切换颜色面板
function toggleColorPanel() {
  showColorPanel.value = !showColorPanel.value
}

// 应用颜色
function applyColor(color: string) {
  if (!editor.value) return
  editor.value.chain().focus().setColor(color).run()
  showColorPanel.value = false
}

// 清除颜色
function clearColor() {
  if (!editor.value) return
  editor.value.chain().focus().unsetColor().run()
  showColorPanel.value = false
}

// 点击外部关闭面板
function handleClickOutside(event: MouseEvent) {
  if (colorPickerRef.value && !colorPickerRef.value.contains(event.target as Node)) {
    showColorPanel.value = false
  }
}

onMounted(() => {
  document.addEventListener('click', handleClickOutside)
})

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
})

// 监听外部值变化
watch(
  () => props.modelValue,
  (newValue) => {
    if (editor.value && newValue !== editor.value.getHTML()) {
      editor.value.chain().setContent(newValue).run()
    }
  }
)

// 监听 editable 属性
watch(
  () => props.editable,
  (newEditable) => {
    if (editor.value) {
      editor.value.setEditable(newEditable)
    }
  }
)

// 处理链接
function handleLink() {
  if (!editor.value) return

  if (editor.value.isActive('link')) {
    // 已有链接，移除
    editor.value.chain().focus().unsetLink().run()
  } else {
    // 打开链接对话框
    const previousUrl = editor.value.getAttributes('link').href
    linkUrl.value = previousUrl || ''
    showLinkDialog.value = true
  }
}

// 确认链接
function confirmLink(url: string) {
  if (!editor.value) return

  // 验证 URL
  if (!url.trim()) {
    return // 空 URL 不插入
  }

  // 自动补全协议
  let finalUrl = url.trim()
  if (!finalUrl.startsWith('http://') && !finalUrl.startsWith('https://')) {
    finalUrl = 'https://' + finalUrl
  }

  editor.value.chain().focus().extendMarkRange('link').setLink({ href: finalUrl }).run()
  showLinkDialog.value = false
  linkUrl.value = ''
}

// 取消链接
function cancelLink() {
  showLinkDialog.value = false
  linkUrl.value = ''
}

// 处理图片选择
function handleImageSelect(event: Event) {
  const input = event.target as HTMLInputElement
  const file = input.files?.[0]
  if (!file) return

  const reader = new FileReader()
  reader.onload = () => {
    const dataUrl = reader.result as string
    editor.value?.chain().focus().setImage({ src: dataUrl }).run()
  }
  reader.readAsDataURL(file)

  // 清空 input 以允许再次选择同一文件
  input.value = ''
}

// 清理
onBeforeUnmount(() => {
  editor.value?.destroy()
})
</script>

<style scoped>
.rich-text-editor {
  border: 1px solid var(--border-primary);
  border-radius: 8px;
  background: var(--bg-secondary);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.toolbar-btn {
  padding: 4px 8px;
  border-radius: 4px;
  color: var(--text-muted);
  transition: all 0.15s;
  display: flex;
  align-items: center;
  gap: 2px;
}

.toolbar-btn:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.toolbar-btn.active {
  background: var(--accent);
  color: white;
}

/* 颜色指示器 */
.color-indicator {
  width: 8px;
  height: 8px;
  border-radius: 2px;
  border: 1px solid rgba(255, 255, 255, 0.3);
}

/* 颜色选择器容器 */
.color-picker-container {
  position: relative;
}

/* 颜色面板 */
.color-panel {
  position: absolute;
  top: 100%;
  left: 0;
  margin-top: 4px;
  padding: 8px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-primary);
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  z-index: 100;
  min-width: 140px;
}

/* 预设颜色网格 */
.preset-colors {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 4px;
}

.color-btn {
  width: 28px;
  height: 28px;
  border-radius: 4px;
  border: 2px solid transparent;
  cursor: pointer;
  transition: transform 0.15s, border-color 0.15s;
}

.color-btn:hover {
  transform: scale(1.1);
}

.color-btn.active {
  border-color: var(--accent);
}

/* 分隔线 */
.divider {
  height: 1px;
  background: var(--border-primary);
  margin: 8px 0;
}

/* 清除颜色按钮 */
.clear-color-btn {
  width: 100%;
  padding: 6px 8px;
  border-radius: 4px;
  background: var(--bg-hover);
  color: var(--text-muted);
  font-size: 12px;
  cursor: pointer;
  transition: background 0.15s;
}

.clear-color-btn:hover {
  background: var(--bg-tertiary);
  color: var(--text-primary);
}

/* 自定义颜色 */
.custom-color {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-top: 8px;
}

.custom-color label {
  font-size: 12px;
  color: var(--text-muted);
}

.custom-color input[type="color"] {
  width: 28px;
  height: 28px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.editor-content {
  padding: 12px;
  overflow-y: auto;
  background: #cbd5e1;
  flex: 1;
  min-height: 0;
}

.editor-content :deep(.tiptap) {
  outline: none;
  height: 100%;
  color: #1a1a1a;
}

.editor-content :deep(.tiptap p.is-editor-empty:first-child::before) {
  color: var(--text-muted);
  content: attr(data-placeholder);
  float: left;
  height: 0;
  pointer-events: none;
}

.editor-content :deep(.tiptap p) {
  margin: 0 0 8px 0;
}

.editor-content :deep(.tiptap ul),
.editor-content :deep(.tiptap ol) {
  padding-left: 20px;
  margin: 0 0 8px 0;
}

.editor-content :deep(.tiptap li) {
  margin: 2px 0;
}

/* 标题样式 */
.editor-content :deep(.tiptap h1) {
  font-size: 1.5em;
  font-weight: bold;
  margin: 0.5em 0 0.3em 0;
  line-height: 1.2;
}

.editor-content :deep(.tiptap h2) {
  font-size: 1.3em;
  font-weight: bold;
  margin: 0.4em 0 0.25em 0;
  line-height: 1.3;
}

.editor-content :deep(.tiptap h3) {
  font-size: 1.15em;
  font-weight: bold;
  margin: 0.3em 0 0.2em 0;
  line-height: 1.4;
}

/* 下划线和删除线样式 */
.editor-content :deep(.tiptap u) {
  text-decoration: underline;
}

.editor-content :deep(.tiptap s) {
  text-decoration: line-through;
}

/* 图片样式 */
.editor-content :deep(.tiptap img) {
  max-width: 100%;
  height: auto;
  border-radius: 4px;
  margin: 8px 0;
}

.editor-content :deep(.tiptap a) {
  color: var(--accent);
  text-decoration: underline;
}
</style>