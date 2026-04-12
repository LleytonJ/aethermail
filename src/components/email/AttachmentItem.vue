<template>
  <div
    class="flex items-center gap-3 p-3 bg-bg-tertiary rounded-lg border border-border-secondary hover:border-border-light hover:bg-bg-hover transition-all cursor-pointer group"
    @click="handleClick"
  >
    <!-- 文件类型图标 -->
    <div class="flex-shrink-0 w-10 h-10 rounded-lg bg-bg-card flex items-center justify-center">
      <component :is="fileIcon" :size="20" class="text-accent" />
    </div>

    <!-- 文件信息 -->
    <div class="flex-1 min-w-0">
      <div class="text-sm text-text-primary truncate font-medium">
        {{ attachment.filename }}
      </div>
      <div class="text-xs text-text-muted">
        {{ formatSize(attachment.size) }}
      </div>
    </div>

    <!-- 下载按钮 -->
    <div
      class="flex-shrink-0 w-8 h-8 rounded-lg bg-bg-card flex items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity"
    >
      <Download :size="16" class="text-text-secondary" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { Attachment } from '@/stores/mailbox'
import { Download, File, FileImage, FileText, FileArchive, FileCode } from 'lucide-vue-next'

interface Props {
  attachment: Attachment
}

const props = defineProps<Props>()

const fileIcon = computed(() => {
  const type = props.attachment.mimeType.toLowerCase()
  if (type.startsWith('image/')) return FileImage
  if (type.includes('pdf')) return FileText
  if (type.includes('zip') || type.includes('compressed') || type.includes('rar') || type.includes('7z')) return FileArchive
  if (type.includes('json') || type.includes('javascript') || type.includes('xml')) return FileCode
  return File
})

function formatSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`
  if (bytes < 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(1)} MB`
  return `${(bytes / (1024 * 1024 * 1024)).toFixed(1)} GB`
}

function handleClick() {
  console.log('Download attachment:', props.attachment.filename)
}
</script>