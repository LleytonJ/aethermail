<template>
  <div class="flex flex-col h-full">
    <!-- 空状态 -->
    <div
      v-if="!mailboxStore.selectedEmailId"
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
        <!-- 回收站邮件：显示恢复按钮 -->
        <div v-if="mailboxStore.currentFolder === 'trash'" class="flex items-center gap-1">
          <button
            class="flex items-center gap-1.5 px-2.5 py-1.5 text-xs text-success hover:bg-success/10 rounded-md transition-colors"
            title="恢复到收件箱"
            @click="showRestoreInboxConfirm"
          >
            <RotateCcw :size="14" />
            恢复到收件箱
          </button>
          <button
            v-if="currentEmail?.isPhishing"
            class="flex items-center gap-1.5 px-2.5 py-1.5 text-xs text-warning hover:bg-warning/10 rounded-md transition-colors"
            title="恢复到钓鱼邮件箱"
            @click="showRestorePhishingConfirm"
          >
            <ShieldAlert :size="14" />
            恢复到钓鱼邮件箱
          </button>
        </div>
        <!-- 钓鱼邮件：只显示"这不是钓鱼邮件"按钮 -->
        <div v-else-if="currentEmail?.isPhishing" class="flex items-center gap-1">
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
            @click="handleReply"
          >
            <Reply :size="14" />
            回复
          </button>
          <button
            class="flex items-center gap-1.5 px-2.5 py-1.5 text-xs text-text-secondary hover:text-text-primary hover:bg-bg-hover rounded-md transition-colors"
            @click="handleForward"
          >
            <Forward :size="14" />
            转发
          </button>
        </div>

        <!-- 回收站邮件：显示永久删除按钮 -->
        <div v-if="mailboxStore.currentFolder === 'trash'" class="flex items-center gap-0.5">
          <button
            class="p-1.5 rounded-md text-text-muted hover:text-danger hover:bg-bg-hover transition-colors"
            title="永久删除"
            @click="showPermanentDeleteConfirm"
          >
            <Trash2 :size="14" />
          </button>
        </div>
        <!-- 钓鱼邮件：只显示删除按钮 -->
        <div v-else-if="currentEmail?.isPhishing" class="flex items-center gap-0.5">
          <button
            class="p-1.5 rounded-md text-text-muted hover:text-danger hover:bg-bg-hover transition-colors"
            title="删除"
            @click="showDeleteConfirm"
          >
            <Trash2 :size="14" />
          </button>
        </div>
        <!-- 正常邮件：显示星标、归档、删除按钮 -->
        <div v-else class="flex items-center gap-0.5">
          <button
            class="p-1.5 rounded-md text-text-muted hover:text-yellow-400 hover:bg-bg-hover transition-colors"
            title="星标"
            @click="currentEmail && mailboxStore.toggleStar(currentEmail.id)"
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
            @click="showDeleteConfirm"
          >
            <Trash2 :size="14" />
          </button>
        </div>
      </div>

      <!-- 邮件头部 -->
      <EmailHeader v-if="currentEmail" :email="currentEmail" />

      <!-- 邮件正文 -->
      <div class="flex-1 overflow-y-auto">
        <EmailBody v-if="currentEmail" :html="currentEmail.bodyHtml ?? undefined" :is-phishing="currentEmail.isPhishing" />
      </div>

      <!-- 附件列表 -->
      <AttachmentList
        v-if="currentEmail?.attachments?.length"
        :attachments="currentEmail.attachments"
      />
    </div>

    <!-- 删除确认对话框 -->
    <ConfirmDialog
      :is-open="deleteConfirmOpen"
      title="移至回收站"
      :message="deleteConfirmMessage"
      confirm-text="确定"
      cancel-text="取消"
      @confirm="handleDelete"
      @cancel="deleteConfirmOpen = false"
    />

    <!-- 恢复到收件箱确认对话框 -->
    <ConfirmDialog
      :is-open="restoreInboxConfirmOpen"
      title="恢复邮件"
      :message="restoreConfirmMessage"
      confirm-text="恢复"
      cancel-text="取消"
      @confirm="handleRestoreToInbox"
      @cancel="restoreInboxConfirmOpen = false"
    />

    <!-- 恢复到钓鱼邮件箱确认对话框 -->
    <ConfirmDialog
      :is-open="restorePhishingConfirmOpen"
      title="恢复到钓鱼邮件箱"
      :message="restorePhishingConfirmMessage"
      confirm-text="恢复"
      cancel-text="取消"
      :danger="true"
      @confirm="handleRestoreToPhishing"
      @cancel="restorePhishingConfirmOpen = false"
    />

    <!-- 永久删除确认对话框 -->
    <ConfirmDialog
      :is-open="permanentDeleteConfirmOpen"
      title="永久删除"
      :message="permanentDeleteConfirmMessage"
      confirm-text="永久删除"
      cancel-text="取消"
      :danger="true"
      @confirm="handlePermanentDelete"
      @cancel="permanentDeleteConfirmOpen = false"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useMailboxStore } from '@/stores/mailbox'
import { useNotificationStore } from '@/stores/notification'
import { useComposeStore } from '@/stores/compose'
import { Mail, Reply, Forward, Star, Archive, Trash2, ShieldAlert, Undo2, RotateCcw } from 'lucide-vue-next'
import EmailHeader from '../email/EmailHeader.vue'
import EmailBody from '../email/EmailBody.vue'
import AttachmentList from '../email/AttachmentList.vue'
import ConfirmDialog from '../ui/ConfirmDialog.vue'
import { updatePhishingStatus } from '@/services/email/emailService'

const mailboxStore = useMailboxStore()
const notificationStore = useNotificationStore()
const composeStore = useComposeStore()

const currentEmail = computed(() => mailboxStore.selectedEmail)

// 删除确认对话框状态
const deleteConfirmOpen = ref(false)
const deleteConfirmMessage = computed(() => {
  const subject = currentEmail.value?.subject || '无主题'
  return `确定要将邮件「${subject}」移至回收站吗？`
})

// 恢复到收件箱确认对话框状态
const restoreInboxConfirmOpen = ref(false)
const restoreConfirmMessage = computed(() => {
  const subject = currentEmail.value?.subject || '无主题'
  return `确定要将邮件「${subject}」恢复到收件箱吗？`
})

// 恢复到钓鱼邮件箱确认对话框状态
const restorePhishingConfirmOpen = ref(false)
const restorePhishingConfirmMessage = computed(() => {
  const subject = currentEmail.value?.subject || '无主题'
  return `确定要将邮件「${subject}」恢复到钓鱼邮件箱吗？该邮件将被重新标记为钓鱼邮件。`
})

// 永久删除确认对话框状态
const permanentDeleteConfirmOpen = ref(false)
const permanentDeleteConfirmMessage = computed(() => {
  const subject = currentEmail.value?.subject || '无主题'
  return `确定要永久删除邮件「${subject}」吗？此操作无法撤销，邮件将彻底从系统中删除。`
})

// 显示删除确认对话框
function showDeleteConfirm() {
  deleteConfirmOpen.value = true
}

// 执行删除（移动到回收站）
async function handleDelete() {
  if (!currentEmail.value) return

  deleteConfirmOpen.value = false

  try {
    await mailboxStore.moveToTrash(currentEmail.value.id)
    notificationStore.success('已移至回收站', '邮件已移动到回收站')
  } catch (e) {
    console.error('移动邮件到回收站失败:', e)
    notificationStore.error('删除失败', '无法移动邮件到回收站')
  }
}

// 回复邮件
function handleReply() {
  if (!currentEmail.value) return

  composeStore.openReply(
    currentEmail.value.fromAddress || '',
    currentEmail.value.subject || '',
    currentEmail.value.bodyText || currentEmail.value.bodyHtml || ''
  )
}

// 转发邮件
function handleForward() {
  if (!currentEmail.value) return

  composeStore.openForward(
    currentEmail.value.subject || '',
    currentEmail.value.bodyText || currentEmail.value.bodyHtml || ''
  )
}

// 将邮件移回收件箱
async function handleMoveToInbox() {
  if (!currentEmail.value) return

  try {
    // 更新钓鱼状态为 false，并标记为用户覆盖（不再重复检测）
    await updatePhishingStatus(currentEmail.value.id, false, null, true)

    // 更新本地邮件状态
    const email = mailboxStore.emails.find(e => e.id === currentEmail.value?.id)
    if (email) {
      email.isPhishing = false
      email.phishingReasons = undefined
    }

    // 从当前列表移除该邮件
    mailboxStore.emails = mailboxStore.emails.filter(e => e.id !== currentEmail.value?.id)
    mailboxStore.clearSelection()

    // 显示成功通知
    notificationStore.success('已移回收件箱', '邮件已标记为正常邮件')

    // 刷新钓鱼邮件未读数量（通过重新加载）
    // 这里需要触发 Sidebar 的刷新，可以通过 accountStore 的变化来触发
  } catch (e) {
    console.error('移回收件箱失败:', e)
    notificationStore.error('操作失败', '无法将邮件移回收件箱')
  }
}

// 显示恢复到收件箱确认对话框
function showRestoreInboxConfirm() {
  restoreInboxConfirmOpen.value = true
}

// 显示恢复到钓鱼邮件箱确认对话框
function showRestorePhishingConfirm() {
  restorePhishingConfirmOpen.value = true
}

// 显示永久删除确认对话框
function showPermanentDeleteConfirm() {
  permanentDeleteConfirmOpen.value = true
}

// 恢复邮件到收件箱
async function handleRestoreToInbox() {
  if (!currentEmail.value) return

  restoreInboxConfirmOpen.value = false

  try {
    await mailboxStore.restoreEmail(currentEmail.value.id, 'inbox')
    notificationStore.success('已恢复', '邮件已恢复到收件箱')
  } catch (e) {
    console.error('恢复邮件到收件箱失败:', e)
    notificationStore.error('恢复失败', '无法恢复邮件到收件箱')
  }
}

// 恢复邮件到钓鱼邮件箱
async function handleRestoreToPhishing() {
  if (!currentEmail.value) return

  restorePhishingConfirmOpen.value = false

  try {
    await mailboxStore.restoreEmail(currentEmail.value.id, 'phishing')
    notificationStore.success('已恢复', '邮件已恢复到钓鱼邮件箱')
  } catch (e) {
    console.error('恢复邮件到钓鱼邮件箱失败:', e)
    notificationStore.error('恢复失败', '无法恢复邮件到钓鱼邮件箱')
  }
}

// 永久删除邮件
async function handlePermanentDelete() {
  if (!currentEmail.value) return

  permanentDeleteConfirmOpen.value = false

  try {
    await mailboxStore.permanentDelete(currentEmail.value.id)
    notificationStore.success('已永久删除', '邮件已彻底从系统中删除')
  } catch (e) {
    console.error('永久删除邮件失败:', e)
    notificationStore.error('删除失败', '无法永久删除邮件')
  }
}
</script>