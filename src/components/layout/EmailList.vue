<template>
  <div class="flex flex-col h-full">
    <!-- 搜索栏 -->
    <div class="p-2 border-b border-border-secondary">
      <div class="relative">
        <Search
          :size="14"
          class="absolute left-2.5 top-1/2 -translate-y-1/2 text-text-muted"
        />
        <input
          v-model="searchQuery"
          type="text"
          placeholder="搜索邮件..."
          class="w-full h-8 pl-8 pr-3 bg-bg-tertiary border border-border-secondary rounded-lg text-sm text-text-primary placeholder-text-muted focus:outline-none focus:border-accent transition-colors"
        />
      </div>
    </div>

    <!-- 工具栏 -->
    <div class="flex items-center justify-between px-2 py-1.5 border-b border-border-secondary">
      <div class="flex items-center gap-0.5">
        <button
          class="p-1.5 rounded-md text-text-muted hover:text-text-secondary hover:bg-bg-hover transition-colors"
          title="全选"
        >
          <CheckSquare :size="14" />
        </button>
        <button
          class="p-1.5 rounded-md text-text-muted hover:text-text-secondary hover:bg-bg-hover transition-colors"
          title="刷新"
        >
          <RefreshCw :size="14" />
        </button>
      </div>
      <div class="flex items-center gap-0.5">
        <button
          class="p-1.5 rounded-md text-text-muted hover:text-text-secondary hover:bg-bg-hover transition-colors"
          title="更多"
        >
          <MoreVertical :size="14" />
        </button>
      </div>
    </div>

    <!-- 邮件列表 -->
    <div class="flex-1 overflow-y-auto">
      <!-- 加载状态 -->
      <div v-if="store.loading" class="p-2 space-y-1">
        <Skeleton v-for="i in 5" :key="i" height="56px" />
      </div>

      <!-- 空状态 -->
      <div
        v-else-if="store.emails.length === 0"
        class="flex flex-col items-center justify-center h-full text-text-muted p-6"
      >
        <Inbox :size="40" class="mb-3 opacity-30" />
        <p class="text-sm">{{ emptyMessage }}</p>
      </div>

      <!-- 邮件列表 -->
      <div v-else>
        <EmailListItem
          v-for="email in filteredEmails"
          :key="email.id"
          :email="email"
          :selected="selectedEmailId === email.id"
          @click="handleSelect(email.id)"
          @toggleStar="store.toggleStar(email.id)"
        />
      </div>
    </div>

    <!-- 底部状态栏 -->
    <div class="px-3 py-1.5 border-t border-border-secondary text-xs text-text-muted">
      {{ store.emails.length }} 封邮件
      <span v-if="store.unreadCount > 0"> · {{ store.unreadCount }} 封未读</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useMailboxStore } from '@/stores/mailbox'
import { Search, Inbox, CheckSquare, RefreshCw, MoreVertical } from 'lucide-vue-next'
import Skeleton from '../ui/Skeleton.vue'
import EmailListItem from '../email/EmailListItem.vue'

const store = useMailboxStore()
const searchQuery = ref('')
const selectedEmailId = computed(() => store.selectedEmailId)

const filteredEmails = computed(() => {
  if (!searchQuery.value) return store.emails
  const query = searchQuery.value.toLowerCase()
  return store.emails.filter(
    (email) =>
      (email.subject || '').toLowerCase().includes(query) ||
      (email.fromName || '').toLowerCase().includes(query) ||
      email.snippet.toLowerCase().includes(query)
  )
})

const emptyMessage = computed(() => {
  const folderNames: Record<string, string> = {
    inbox: '收件箱为空',
    sent: '没有已发送的邮件',
    drafts: '没有草稿',
    archive: '归档为空',
    trash: '已删除为空'
  }
  return folderNames[store.currentFolder] || '暂无邮件'
})

function handleSelect(emailId: string) {
  store.selectEmail(emailId)
  store.markAsRead(emailId)
}
</script>