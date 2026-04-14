<template>
  <div class="flex flex-col h-full">
    <!-- 空状态 -->
    <div
      v-if="!store.selectedEmailId"
      class="flex-1 flex flex-col items-center justify-center text-text-muted"
    >
      <div class="w-20 h-20 rounded-full bg-bg-tertiary/50 flex items-center justify-center mb-4">
        <Mail :size="32" class="text-text-muted/50" />
      </div>
      <p class="text-sm">选择一封邮件进行阅读</p>
    </div>

    <!-- 邮件内容 -->
    <div v-else class="flex flex-col h-full animate-fade-in">
      <!-- 钓鱼邮件警告 Banner -->
      <div
        v-if="currentEmail?.isPhishing"
        class="px-4 py-2 bg-danger/10 border-b border-danger/20"
      >
        <div class="flex items-center gap-2 text-danger text-sm">
          <ShieldAlert :size="16" />
          <span class="font-medium">此邮件被判定为钓鱼邮件</span>
        </div>
        <div v-if="currentEmail?.phishingReasons?.length" class="mt-1 text-xs text-danger/80">
          <span class="font-medium">检测原因：</span>
          <span v-for="(reason, index) in currentEmail.phishingReasons" :key="index">
            {{ reason }}{{ index < currentEmail.phishingReasons.length - 1 ? '；' : '' }}
          </span>
        </div>
      </div>

      <!-- 工具栏 -->
      <div class="flex items-center justify-between px-4 py-2 border-b border-border-secondary">
        <!-- 钓鱼邮件：只显示"这不是钓鱼邮件"按钮 -->
        <div v-if="currentEmail?.isPhishing" class="flex items-center gap-1">
          <button
            class="flex items-center gap-1.5 px-2.5 py-1.5 text-xs text-success hover:bg-success/10 rounded-md transition-colors"
            title="移回收件箱"
            @click="handleMoveToInbox"
          >
            <Undo2 :size="14" />
            这不是钓鱼邮件
          </button>
        </div>
        <!-- 正常邮件：显示回复、转发按钮 -->
        <div v-else class="flex items-center gap-1">
          <button
            class="flex items-center gap-1.5 px-2.5 py-1.5 text-xs text-text-secondary hover:text-text-primary hover:bg-bg-hover rounded-md transition-colors"
          >
            <Reply :size="14" />
            回复
          </button>
          <button
            class="flex items-center gap-1.5 px-2.5 py-1.5 text-xs text-text-secondary hover:text-text-primary hover:bg-bg-hover rounded-md transition-colors"
          >
            <Forward :size="14" />
            转发
          </button>
        </div>

        <!-- 钓鱼邮件：只显示删除按钮 -->
        <div v-if="currentEmail?.isPhishing" class="flex items-center gap-0.5">
          <button
            class="p-1.5 rounded-md text-text-muted hover:text-danger hover:bg-bg-hover transition-colors"
            title="删除"
          >
            <Trash2 :size="14" />
          </button>
        </div>
        <!-- 正常邮件：显示星标、归档、删除按钮 -->
        <div v-else class="flex items-center gap-0.5">
          <button
            class="p-1.5 rounded-md text-text-muted hover:text-yellow-400 hover:bg-bg-hover transition-colors"
            title="星标"
            @click="currentEmail && store.toggleStar(currentEmail.id)"
          >
            <Star
              :size="14"
              :fill="currentEmail?.isStarred ? 'currentColor' : 'none'"
              :class="currentEmail?.isStarred ? 'text-yellow-400' : ''"
            />
          </button>
          <button
            class="p-1.5 rounded-md text-text-muted hover:text-text-secondary hover:bg-bg-hover transition-colors"
            title="归档"
          >
            <Archive :size="14" />
          </button>
          <button
            class="p-1.5 rounded-md text-text-muted hover:text-danger hover:bg-bg-hover transition-colors"
            title="删除"
          >
            <Trash2 :size="14" />
          </button>
        </div>
      </div>

      <!-- 邮件头部 -->
      <EmailHeader v-if="currentEmail" :email="currentEmail" />

      <!-- 邮件正文 -->
      <div class="flex-1 overflow-y-auto hide-scrollbar">
        <EmailBody v-if="currentEmail" :html="currentEmail.bodyHtml ?? undefined" :is-phishing="currentEmail.isPhishing" />
      </div>

      <!-- 附件列表 -->
      <AttachmentList
        v-if="currentEmail?.attachments?.length"
        :attachments="currentEmail.attachments"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useMailboxStore } from '@/stores/mailbox'
import { useNotificationStore } from '@/stores/notification'
import { Mail, Reply, Forward, Star, Archive, Trash2, ShieldAlert, Undo2 } from 'lucide-vue-next'
import EmailHeader from '../email/EmailHeader.vue'
import EmailBody from '../email/EmailBody.vue'
import AttachmentList from '../email/AttachmentList.vue'
import { updatePhishingStatus } from '@/services/email/emailService'

const store = useMailboxStore()
const notificationStore = useNotificationStore()

const currentEmail = computed(() => store.selectedEmail)

// 将邮件移回收件箱
async function handleMoveToInbox() {
  if (!currentEmail.value) return

  try {
    // 更新钓鱼状态为 false
    await updatePhishingStatus(currentEmail.value.id, false, null)

    // 更新本地邮件状态
    const email = store.emails.find(e => e.id === currentEmail.value?.id)
    if (email) {
      email.isPhishing = false
      email.phishingReasons = undefined
    }

    // 从当前列表移除该邮件
    store.emails = store.emails.filter(e => e.id !== currentEmail.value?.id)
    store.clearSelection()

    // 显示成功通知
    notificationStore.success('已移回收件箱', '邮件已标记为正常邮件')

    // 刷新钓鱼邮件未读数量（通过重新加载）
    // 这里需要触发 Sidebar 的刷新，可以通过 accountStore 的变化来触发
  } catch (e) {
    console.error('移回收件箱失败:', e)
    notificationStore.error('操作失败', '无法将邮件移回收件箱')
  }
}
</script>