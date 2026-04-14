<template>
  <div class="px-5 py-4 border-b border-border-secondary">
    <!-- 主题 -->
    <h1 class="text-lg font-semibold text-text-primary mb-3">
      {{ email.subject || '无主题' }}
    </h1>

    <!-- 发件人信息 -->
    <div class="flex items-center gap-3">
      <!-- 头像 -->
      <div class="avatar avatar-md">
        {{ avatarLetter }}
      </div>

      <!-- 发件人详情 -->
      <div class="flex-1 min-w-0">
        <div class="flex items-center gap-2">
          <span class="text-sm font-medium text-text-primary">
            {{ email.fromName || '未知发件人' }}
          </span>
          <span v-if="email.fromAddress" class="text-xs text-text-muted">
            &lt;{{ email.fromAddress }}&gt;
          </span>
        </div>
        <div class="text-xs text-text-muted mt-0.5">
          收件人: <span class="text-text-secondary">{{ email.toAddresses.length > 0 ? email.toAddresses.join(', ') : '未知' }}</span>
        </div>
      </div>

      <!-- 时间 -->
      <div class="text-xs text-text-muted">
        {{ formatFullDate(email.date) }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { Email } from '@/stores/mailbox'

interface Props {
  email: Email
}

const props = defineProps<Props>()

const avatarLetter = computed(() => (props.email.fromName || props.email.fromAddress || '?').charAt(0).toUpperCase())

function formatFullDate(timestamp: number): string {
  // 处理无效的时间戳
  if (!timestamp || timestamp <= 0) {
    return '未知时间'
  }

  // 时间戳是秒级的 Unix timestamp，需要乘以 1000 转为毫秒
  const date = new Date(timestamp * 1000)
  return date.toLocaleString('zh-CN', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  })
}
</script>