<template>
  <div
    class="thread-item p-3 rounded-lg border cursor-pointer transition-colors"
    :class="[
      isSelected
        ? 'bg-blue-50 border-blue-200 dark:bg-blue-900/20 dark:border-blue-800'
        : 'bg-white border-gray-200 hover:bg-gray-50 dark:bg-gray-800 dark:border-gray-700 dark:hover:bg-gray-700/50'
    ]"
    @click="$emit('select', thread.threadId)"
  >
    <!-- 发件人信息 -->
    <div class="flex items-center gap-2 mb-1">
      <!-- 未读指示器 -->
      <div
        v-if="hasUnread"
        class="w-2 h-2 rounded-full bg-blue-500"
      />

      <!-- 发件人头像/名称 -->
      <div class="flex-1 min-w-0">
        <p
          class="text-sm font-medium truncate"
          :class="hasUnread ? 'text-gray-900 dark:text-gray-100' : 'text-gray-700 dark:text-gray-300'"
        >
          {{ displayFrom }}
        </p>
      </div>

      <!-- 时间 -->
      <p class="text-xs text-gray-500 dark:text-gray-400">
        {{ formatDate(lastMessageAt) }}
      </p>
    </div>

    <!-- 主题 -->
    <p
      class="text-sm mb-2 truncate"
      :class="hasUnread ? 'font-medium text-gray-900 dark:text-gray-100' : 'text-gray-600 dark:text-gray-400'"
    >
      {{ subjectBase || '(无主题)' }}
    </p>

    <!-- 底部信息 -->
    <div class="flex items-center gap-2">
      <!-- 邮件数量徽章 -->
      <div
        v-if="messageCount > 1"
        class="inline-flex items-center px-2 py-0.5 rounded-full text-xs font-medium bg-gray-100 text-gray-600 dark:bg-gray-700 dark:text-gray-400"
      >
        {{ messageCount }} 封邮件
      </div>

      <!-- 星标指示 -->
      <svg
        v-if="hasStarred"
        class="w-4 h-4 text-yellow-500"
        fill="currentColor"
        viewBox="0 0 24 24"
      >
        <path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z"/>
      </svg>

      <!-- 附件指示 -->
      <svg
        v-if="hasAttachment"
        class="w-4 h-4 text-gray-400"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
      >
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.172 7l-6.586 6.586a2 2 0 102.828 2.828l6.414-6.586a4 4 0 00-5.656-5.656l-6.415 6.585a6 6 0 108.486 8.486L20.5 13"/>
      </svg>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { ThreadGroup } from '@/services/threading/threadBuilder'

interface Props {
  thread: ThreadGroup
  emails: Array<{
    id: string
    fromName: string | null
    fromAddress: string | null
    date: number
    isRead: boolean
    isStarred: boolean
    hasAttachment: boolean
    subject: string | null
  }>
  isSelected?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  isSelected: false
})

defineEmits<{
  (e: 'select', threadId: string): void
}>()

// 计算会话中的邮件（按日期升序）
const threadEmails = computed(() =>
  props.emails.sort((a, b) => a.date - b.date)
)

// 计算显示的发件人（最新的邮件发件人）
const displayFrom = computed(() => {
  const latestEmail = threadEmails.value[threadEmails.value.length - 1]
  if (!latestEmail) return '未知'
  return latestEmail.fromName || latestEmail.fromAddress || '未知'
})

// 计算主题（使用第一封邮件的主题）
const subjectBase = computed(() => {
  const firstEmail = threadEmails.value[0]
  return firstEmail?.subject || null
})

// 计算最后消息时间
const lastMessageAt = computed(() => {
  const latestEmail = threadEmails.value[threadEmails.value.length - 1]
  return latestEmail?.date || 0
})

// 计算消息数量
const messageCount = computed(() => props.thread.messageIds.length)

// 是否有未读邮件
const hasUnread = computed(() =>
  props.emails.some(e => !e.isRead)
)

// 是否有星标邮件
const hasStarred = computed(() =>
  props.emails.some(e => e.isStarred)
)

// 是否有附件
const hasAttachment = computed(() =>
  props.emails.some(e => e.hasAttachment)
)

// 格式化日期
function formatDate(timestamp: number): string {
  const date = new Date(timestamp * 1000)
  const now = new Date()
  const diffDays = Math.floor((now.getTime() - date.getTime()) / (1000 * 60 * 60 * 24))

  if (diffDays === 0) {
    return date.toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit' })
  } else if (diffDays === 1) {
    return '昨天'
  } else if (diffDays < 7) {
    return `${diffDays}天前`
  } else {
    return date.toLocaleDateString('zh-CN', { month: '2-digit', day: '2-digit' })
  }
}
</script>