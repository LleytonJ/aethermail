<template>
  <div class="p-4 border-t border-border-secondary bg-bg-secondary">
    <h3 class="text-sm font-medium text-text-secondary mb-3 flex items-center gap-2">
      <Paperclip :size="16" />
      附件 ({{ attachments.length }})
    </h3>
    <div class="grid grid-cols-2 gap-3">
      <AttachmentItem
        v-for="attachment in attachments"
        :key="attachment.id"
        :attachment="attachment"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Attachment } from '@/stores/mailbox'
import { Paperclip } from 'lucide-vue-next'
import AttachmentItem from './AttachmentItem.vue'

interface Props {
  attachments: Attachment[]
}

defineProps<Props>()
</script>