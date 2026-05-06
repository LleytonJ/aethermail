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
        <!-- 会话/单邮件切换按钮（仅收件箱显示） -->
        <button
          v-if="store.currentFolder === 'inbox' && store.threadGroups.length > 0"
          class="p-1.5 rounded-md transition-colors"
          :class="viewMode === 'thread'
            ? 'text-accent bg-accent/10'
            : 'text-text-muted hover:text-text-secondary hover:bg-bg-hover'"
          :title="viewMode === 'thread' ? '切换到单邮件视图' : '切换到会话视图'"
          @click="toggleViewMode"
        >
          <MessagesSquare :size="14" />
        </button>
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
        v-else-if="(viewMode === 'thread' ? filteredThreads.length : filteredEmails.length) === 0"
        class="flex flex-col items-center justify-center h-full text-text-muted p-6"
      >
        <Inbox :size="40" class="mb-3 opacity-30" />
        <p class="text-sm">{{ emptyMessage }}</p>
      </div>

      <!-- 会话列表 -->
      <div v-else-if="viewMode === 'thread'" class="p-2 space-y-1">
        <ThreadListItem
          v-for="thread in filteredThreads"
          :key="thread.threadId"
          :thread="thread"
          :emails="getThreadEmails(thread.messageIds)"
          :selected="selectedThreadId === thread.threadId"
          @select="handleSelectThread"
        />
      </div>

      <!-- 单邮件列表 -->
      <div v-else>
        <EmailListItem
          v-for="email in filteredEmails"
          :key="email.id"
          :email="email"
          :selected="selectedEmailId === email.id"
          :show-delete="store.currentFolder === 'drafts'"
          @click="handleSelect(email.id)"
          @toggleStar="store.toggleStar(email.id)"
          @delete="handleDelete(email.id)"
        />
      </div>
    </div>

    <!-- 底部状态栏 -->
    <div class="px-3 py-1.5 border-t border-border-secondary text-xs text-text-muted">
      <template v-if="viewMode === 'thread'">
        {{ store.threadGroups.length }} 个会话
      </template>
      <template v-else>
        {{ store.emails.length }} 封邮件
        <span v-if="store.unreadCount > 0"> · {{ store.unreadCount }} 封未读</span>
      </template>
    </div>

    <!-- 删除确认对话框 -->
    <ConfirmDialog
      :is-open="showDeleteConfirm"
      title="删除草稿"
      message="确定要删除这个草稿吗？删除后将无法恢复。"
      confirm-text="删除"
      cancel-text="取消"
      danger
      @confirm="confirmDelete"
      @cancel="cancelDelete"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useMailboxStore } from '@/stores/mailbox'
import { useComposeStore } from '@/stores/compose'
import { useNotificationStore } from '@/stores/notification'
import { getDraftDetail, getDraftAttachments } from '@/services/email/emailService'
import { Search, Inbox, CheckSquare, RefreshCw, MoreVertical, MessagesSquare } from 'lucide-vue-next'
import Skeleton from '../ui/Skeleton.vue'
import EmailListItem from '../email/EmailListItem.vue'
import ThreadListItem from '../ThreadListItem.vue'
import ConfirmDialog from '../ui/ConfirmDialog.vue'

const store = useMailboxStore()
const composeStore = useComposeStore()
const notificationStore = useNotificationStore()
const searchQuery = ref('')
const selectedEmailId = computed(() => store.selectedEmailId)
const selectedThreadId = computed(() => store.selectedThreadId)

// 视图模式：单邮件或会话
const viewMode = ref<'single' | 'thread'>('single')

// 删除确认对话框状态
const showDeleteConfirm = ref(false)
const deletingDraftId = ref<string | null>(null)

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

// 过滤会话列表
const filteredThreads = computed(() => {
  if (!searchQuery.value) return store.threadGroups
  const query = searchQuery.value.toLowerCase()
  return store.threadGroups.filter((thread) => {
    // 搜索会话中的邮件主题或发件人
    const threadEmails = getThreadEmails(thread.messageIds)
    return threadEmails.some(
      (email) =>
        (email.subject || '').toLowerCase().includes(query) ||
        (email.fromName || '').toLowerCase().includes(query) ||
        email.snippet.toLowerCase().includes(query)
    )
  })
})

// 获取会话中的邮件列表
function getThreadEmails(messageIds: string[]) {
  return messageIds
    .map((msgId) => store.emails.find((e) => e.id === msgId))
    .filter((e): e is NonNullable<typeof e> => e !== undefined)
}

// 切换视图模式
function toggleViewMode() {
  viewMode.value = viewMode.value === 'single' ? 'thread' : 'single'
  // 清空选中状态
  store.clearSelection()
}

// 选择会话
function handleSelectThread(threadId: string) {
  // 如果正在撰写邮件，需要先确认
  if (composeStore.isOpen) {
    composeStore.setPendingAction({ type: 'selectThread', threadId })
    return
  }

  store.selectThread(threadId)
}

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
  const email = store.emails.find(e => e.id === emailId)

  // 如果正在撰写邮件，需要先确认（除非是选择当前编辑的草稿）
  if (composeStore.isOpen) {
    // 如果是草稿且是当前正在编辑的草稿，直接返回（不需要切换）
    if (store.currentFolder === 'drafts' && email) {
      const draftId = email.id.replace('draft-', '')
      if (composeStore.currentDraftId === draftId) {
        return // 已经在编辑这个草稿，不需要切换
      }
    }
    composeStore.setPendingAction({ type: 'selectEmail', emailId })
    return
  }

  // 草稿邮件：先获取详情再打开编辑模式
  if (store.currentFolder === 'drafts' && email) {
    // 草稿 id 格式是 "draft-{draft_id}"
    const draftId = email.id.replace('draft-', '')
    loadDraftAndOpenEditor(draftId, email)
    return
  }

  // 其他邮件：正常选择并标记已读
  store.selectEmail(emailId)
  store.markAsRead(emailId)
}

// 加载草稿详情并打开编辑器
async function loadDraftAndOpenEditor(draftId: string, email: any) {
  console.log('[loadDraftAndOpenEditor] email.id:', email.id, 'draftId:', draftId)
  try {
    const detail = await getDraftDetail(draftId)
    // 获取草稿附件
    const draftAttachments = await getDraftAttachments(draftId)
    console.log('[loadDraftAndOpenEditor] 获取到附件数量:', draftAttachments.length, draftAttachments)

    // 转换附件为 ComposeAttachment 格式 - 直接使用后端返回的 content
    const attachments = draftAttachments.map(a => ({
      id: a.id,
      filename: a.filename,
      contentType: a.content_type || inferMimeType(a.filename),
      size: a.file_size,
      path: a.file_path || '',  // 保留路径（可选）
      content: a.content || undefined  // 直接使用后端返回的 Base64 content
    }))
    console.log('[loadDraftAndOpenEditor] 转换后的 attachments:', attachments)

    if (detail) {
      composeStore.openDraft(
        draftId,
        detail.to_addresses,
        detail.cc_addresses,
        detail.subject,
        detail.body_html,
        detail.body_text,
        attachments
      )
    } else {
      // 如果获取详情失败，使用列表中的基本信息
      composeStore.openDraft(
        draftId,
        email.toAddresses,
        email.ccAddresses,
        email.subject || null,
        email.bodyHtml || null,
        email.bodyText || null,
        attachments
      )
    }
  } catch (e) {
    console.error('获取草稿详情失败:', e)
    // 使用列表中的基本信息（无附件）
    composeStore.openDraft(
      draftId,
      email.toAddresses,
      email.ccAddresses,
      email.subject || null,
      email.bodyHtml || null,
      email.bodyText || null,
      []
    )
  }
}

// 推断 MIME 类型
function inferMimeType(filename: string): string {
  const ext = filename.split('.').pop()?.toLowerCase() || ''
  const mimeTypes: Record<string, string> = {
    'pdf': 'application/pdf',
    'doc': 'application/msword',
    'docx': 'application/vnd.openxmlformats-officedocument.wordprocessingml.document',
    'xls': 'application/vnd.ms-excel',
    'xlsx': 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
    'zip': 'application/zip',
    'jpg': 'image/jpeg',
    'jpeg': 'image/jpeg',
    'png': 'image/png',
    'gif': 'image/gif',
    'txt': 'text/plain',
  }
  return mimeTypes[ext] || 'application/octet-stream'
}

// 删除草稿
function handleDelete(emailId: string) {
  // 检查是否正在编辑这个草稿
  const draftId = emailId.replace('draft-', '')
  if (composeStore.isOpen && composeStore.currentDraftId === draftId) {
    // 正在编辑这个草稿，需要先关闭撰写页面
    composeStore.close()
  }

  deletingDraftId.value = emailId
  showDeleteConfirm.value = true
}

// 确认删除草稿
async function confirmDelete() {
  if (!deletingDraftId.value) return

  showDeleteConfirm.value = false
  try {
    await store.deleteDraft(deletingDraftId.value)
    notificationStore.success('删除成功', '草稿已删除')
  } catch (e) {
    notificationStore.error('删除失败', e instanceof Error ? e.message : '删除草稿失败')
  }
  deletingDraftId.value = null
}

// 取消删除
function cancelDelete() {
  showDeleteConfirm.value = false
  deletingDraftId.value = null
}
</script>