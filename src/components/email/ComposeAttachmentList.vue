<template>
  <div v-if="attachments.length > 0" class="mt-4">
    <!-- 标题 -->
    <div class="flex items-center justify-between mb-2">
      <div class="text-xs font-medium text-text-secondary">
        附件 ({{ attachments.length }})
      </div>
      <button
        class="text-xs text-text-muted hover:text-danger transition-colors"
        @click="handleClearAll"
      >
        清空全部
      </button>
    </div>

    <!-- 附件列表 -->
    <div class="space-y-2">
      <ComposeAttachmentItem
        v-for="attachment in attachments"
        :key="attachment.id"
        :attachment="attachment"
        @remove="handleRemove"
      />
    </div>

    <!-- 总大小 -->
    <div class="mt-2 text-xs text-text-muted">
      总大小: {{ formatTotalSize() }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import ComposeAttachmentItem from './ComposeAttachmentItem.vue'
import type { ComposeAttachment } from '@/stores/compose'

interface Props {
  attachments: ComposeAttachment[]
}

const props = defineProps<Props>()
const emit = defineEmits<{
  remove: [id: string]
  clearAll: []
}>()

// 计算总大小
const totalSize = computed(() => {
  return props.attachments.reduce((sum, a) => sum + a.size, 0)
})

// 格式化总大小
function formatTotalSize(): string {
  const bytes = totalSize.value
  if (bytes < 1024) return `${bytes} B`
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`
  if (bytes < 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(1)} MB`
  return `${(bytes / (1024 * 1024 * 1024)).toFixed(1)} GB`
}

// 移除单个附件
function handleRemove(id: string) {
  emit('remove', id)
}

// 清空所有附件
function handleClearAll() {
  emit('clearAll')
}
</script>