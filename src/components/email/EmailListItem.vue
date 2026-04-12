<template>
  <button
    :class="[
      'w-full text-left border-b border-border-secondary group hover-lift press-scale',
      selected ? 'bg-bg-selected' : 'hover:bg-bg-hover',
      !email.isRead ? 'bg-bg-tertiary/50' : ''
    ]"
    @click="$emit('click')"
  >
    <div class="flex items-start gap-2.5 px-3 py-2.5">
      <!-- 头像 -->
      <div
        :class="[
          'avatar shrink-0',
          selected ? 'avatar-sm' : 'avatar-sm',
          email.isRead ? 'bg-text-tertiary' : 'bg-accent'
        ]"
      >
        {{ avatarLetter }}
      </div>

      <!-- 内容 -->
      <div class="flex-1 min-w-0">
        <!-- 第一行：发件人 + 时间 -->
        <div class="flex items-center justify-between gap-2">
          <span
            :class="[
              'text-sm truncate',
              email.isRead ? 'text-text-secondary' : 'font-semibold text-text-primary'
            ]"
          >
            {{ email.fromName }}
          </span>
          <span class="text-xs text-text-muted whitespace-nowrap shrink-0">
            {{ formatTime(email.date) }}
          </span>
        </div>

        <!-- 主题 -->
        <div
          :class="[
            'text-sm truncate mt-0.5',
            email.isRead ? 'text-text-secondary' : 'text-text-primary'
          ]"
        >
          {{ email.subject }}
        </div>

        <!-- 摘要 + 图标 -->
        <div class="flex items-center gap-1.5 mt-0.5">
          <span class="text-xs text-text-muted truncate flex-1">
            {{ email.snippet }}
          </span>
          <Paperclip
            v-if="email.hasAttachment"
            :size="12"
            class="text-text-muted shrink-0"
          />
          <button
            class="shrink-0 p-0.5 rounded transition-colors"
            :class="email.isStarred ? 'text-yellow-400' : 'text-text-muted opacity-0 group-hover:opacity-100'"
            title="星标"
            @click.stop="$emit('toggleStar')"
          >
            <Star :size="12" :fill="email.isStarred ? 'currentColor' : 'none'" />
          </button>
        </div>
      </div>
    </div>
  </button>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { Email } from '@/stores/mailbox'
import { Paperclip, Star } from 'lucide-vue-next'

interface Props {
  email: Email
  selected?: boolean
}

const props = defineProps<Props>()
defineEmits<{
  click: []
  toggleStar: []
}>()

const avatarLetter = computed(() => (props.email.fromName || props.email.fromAddress || '?').charAt(0).toUpperCase())

function formatTime(timestamp: number): string {
  // 时间戳是秒级的 Unix timestamp，需要乘以 1000 转为毫秒
  const date = new Date(timestamp * 1000)
  const now = new Date()
  const isToday = date.toDateString() === now.toDateString()
  const isYesterday =
    new Date(now.getTime() - 86400000).toDateString() === date.toDateString()

  if (isToday) {
    return date.toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit' })
  }
  if (isYesterday) {
    return '昨天'
  }
  if (date.getFullYear() === now.getFullYear()) {
    return date.toLocaleDateString('zh-CN', { month: 'short', day: 'numeric' })
  }
  return date.toLocaleDateString('zh-CN', { year: 'numeric', month: 'short', day: 'numeric' })
}
</script>