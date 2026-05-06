<template>
  <div class="thread-view h-full flex flex-col">
    <!-- Header -->
    <div class="p-4 border-b border-gray-200 dark:border-gray-700">
      <h2 class="text-lg font-medium text-gray-900 dark:text-gray-100">
        {{ threadSubject }}
      </h2>
      <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">
        {{ emails.length }} 封邮件
      </p>
    </div>

    <!-- Messages list -->
    <div class="flex-1 overflow-y-auto p-4 space-y-4">
      <div
        v-for="email in emails"
        :key="email.id"
        class="email-item p-4 rounded-lg border transition-colors"
        :class="[
          selectedEmailId === email.id
            ? 'bg-blue-50 border-blue-200 dark:bg-blue-900/20 dark:border-blue-800'
            : 'bg-white border-gray-200 dark:bg-gray-800 dark:border-gray-700'
        ]"
        @click="$emit('select-email', email.id)"
      >
        <!-- 发件人和时间 -->
        <div class="flex items-start gap-3 mb-2">
          <!-- 未读指示器 -->
          <div
            v-if="!email.isRead"
            class="w-2 h-2 mt-1.5 rounded-full bg-blue-500"
          />

          <div class="flex-1">
            <!-- 发件人 -->
            <p class="text-sm font-medium text-gray-900 dark:text-gray-100">
              {{ email.fromName || email.fromAddress || '未知' }}
            </p>
            <!-- 收件人 -->
            <p class="text-xs text-gray-500 dark:text-gray-400">
              发送给: {{ email.toAddresses.join(', ') }}
            </p>
          </div>

          <!-- 时间 -->
          <p class="text-xs text-gray-500 dark:text-gray-400">
            {{ formatDate(email.date) }}
          </p>
        </div>

        <!-- 正文预览 -->
        <p class="text-sm text-gray-600 dark:text-gray-300 line-clamp-3">
          {{ email.snippet || '(无正文)' }}
        </p>

        <!-- 底部操作 -->
        <div class="flex items-center gap-2 mt-2 pt-2 border-t border-gray-100 dark:border-gray-700">
          <!-- 星标 -->
          <button
            class="p-1 rounded hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
            @click.stop="$emit('toggle-star', email.id)"
          >
            <svg
              class="w-4 h-4"
              :class="email.isStarred ? 'text-yellow-500' : 'text-gray-400'"
              fill="currentColor"
              viewBox="0 0 24 24"
            >
              <path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z"/>
            </svg>
          </button>

          <!-- 附件指示 -->
          <svg
            v-if="email.hasAttachment"
            class="w-4 h-4 text-gray-400"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.172 7l-6.586 6.586a2 2 0 102.828 2.828l6.414-6.586a4 4 0 00-5.656-5.656l-6.415 6.585a6 6 0 108.486 8.486L20.5 13"/>
          </svg>
        </div>
      </div>
    </div>

    <!-- Empty state -->
    <div
      v-if="emails.length === 0"
      class="flex-1 flex items-center justify-center text-gray-500 dark:text-gray-400"
    >
      <p>加载中...</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Email } from '@/stores/mailbox'

interface Props {
  emails: Email[]
  threadSubject: string | null
  selectedEmailId: string | null
}

withDefaults(defineProps<Props>(), {
  emails: () => [],
  threadSubject: null,
  selectedEmailId: null
})

defineEmits<{
  (e: 'select-email', emailId: string): void
  (e: 'toggle-star', emailId: string): void
}>()

// 格式化日期
function formatDate(timestamp: number): string {
  const date = new Date(timestamp * 1000)
  return date.toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit'
  })
}
</script>

<style scoped>
.line-clamp-3 {
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>