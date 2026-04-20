<template>
  <div
    class="flex items-center gap-3 p-2 bg-bg-secondary rounded-lg border border-border-primary hover:border-accent transition-all group"
  >
    <!-- 文件类型图标 -->
    <div class="flex-shrink-0 w-8 h-8 rounded bg-bg-tertiary flex items-center justify-center">
      <component :is="fileIcon" :size="16" class="text-accent" />
    </div>

    <!-- 文件信息 -->
    <div class="flex-1 min-w-0">
      <div class="text-sm text-text-primary truncate">
        {{ attachment.filename }}
      </div>
      <div class="text-xs text-text-muted">
        {{ formatSize(attachment.size) }}
      </div>
    </div>

    <!-- 删除按钮 -->
    <button
      class="flex-shrink-0 w-6 h-6 rounded flex items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity hover:bg-danger/20 text-text-muted hover:text-danger"
      title="移除附件"
      @click.stop="handleRemove"
    >
      <X :size="14" />
    </button>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { X, File, FileImage, FileText, FileArchive, FileCode, FileSpreadsheet, Presentation } from 'lucide-vue-next'
import type { ComposeAttachment } from '@/stores/compose'

interface Props {
  attachment: ComposeAttachment
}

const props = defineProps<Props>()
const emit = defineEmits<{
  remove: [id: string]
}>()

// 文件图标映射
const fileIcon = computed(() => {
  const ext = getExtension(props.attachment.filename).toLowerCase()

  // 图片
  if (['jpg', 'jpeg', 'png', 'gif', 'bmp', 'webp', 'svg'].includes(ext)) return FileImage

  // PDF
  if (ext === 'pdf') return FileText

  // 文档
  if (['doc', 'docx', 'txt', 'rtf', 'odt'].includes(ext)) return FileText

  // 电子表格
  if (['xls', 'xlsx', 'csv', 'ods'].includes(ext)) return FileSpreadsheet

  // 演示文稿
  if (['ppt', 'pptx', 'odp'].includes(ext)) return Presentation

  // 压缩文件
  if (['zip', 'rar', '7z', 'tar', 'gz'].includes(ext)) return FileArchive

  // 代码文件
  if (['json', 'js', 'ts', 'html', 'css', 'xml', 'py', 'java', 'go', 'rs'].includes(ext)) return FileCode

  return File
})

// 格式化文件大小
function formatSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`
  if (bytes < 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(1)} MB`
  return `${(bytes / (1024 * 1024 * 1024)).toFixed(1)} GB`
}

// 获取文件扩展名
function getExtension(filename: string): string {
  const parts = filename.split('.')
  if (parts.length > 1) {
    return parts.pop() || ''
  }
  return ''
}

// 移除附件
function handleRemove() {
  emit('remove', props.attachment.id)
}
</script>