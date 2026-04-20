<template>
  <div class="rich-text-editor" :class="{ 'opacity-50 pointer-events-none': !editable }">
    <!-- Toolbar -->
    <div v-if="editable && showToolbar" class="toolbar flex items-center gap-1 p-2 border-b border-border-primary bg-bg-secondary">
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
        @click="handleImage"
      >
        <ImageIcon :size="16" />
      </button>
    </div>

    <!-- Editor Content -->
    <editor-content
      :editor="editor"
      class="editor-content"
      :style="{ minHeight: `${minHeight}px` }"
    />
  </div>
</template>

<script setup lang="ts">
import { watch, onBeforeUnmount } from 'vue'
import { useEditor, EditorContent } from '@tiptap/vue-3'
import StarterKit from '@tiptap/starter-kit'
import Image from '@tiptap/extension-image'
import Placeholder from '@tiptap/extension-placeholder'
import { Bold, Italic, List, ListOrdered, Link, Image as ImageIcon } from 'lucide-vue-next'

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

// 创建编辑器实例
const editor = useEditor({
  content: props.modelValue,
  editable: props.editable,
  extensions: [
    StarterKit.configure({
      heading: false,
      codeBlock: false,
      blockquote: false,
      horizontalRule: false
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

  const previousUrl = editor.value.getAttributes('link').href
  const url = window.prompt('链接地址', previousUrl)

  if (url === null) return

  if (url === '') {
    editor.value.chain().focus().extendMarkRange('link').unsetLink().run()
    return
  }

  editor.value.chain().focus().extendMarkRange('link').setLink({ href: url }).run()
}

// 处理图片插入
function handleImage() {
  if (!editor.value) return

  const url = window.prompt('图片地址或粘贴 base64 数据')

  if (url) {
    editor.value.chain().focus().setImage({ src: url }).run()
  }
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
}

.toolbar-btn {
  padding: 4px 8px;
  border-radius: 4px;
  color: var(--text-muted);
  transition: all 0.15s;
}

.toolbar-btn:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.toolbar-btn.active {
  background: var(--accent);
  color: white;
}

.editor-content {
  padding: 12px;
  overflow-y: auto;
}

.editor-content :deep(.tiptap) {
  outline: none;
  min-height: inherit;
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

.editor-content :deep(.tiptap img) {
  max-width: 100%;
  height: auto;
  border-radius: 4px;
}

.editor-content :deep(.tiptap a) {
  color: var(--accent);
  text-decoration: underline;
}
</style>