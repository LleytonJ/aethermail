<template>
  <div class="flex flex-col h-full bg-sidebar-bg text-sidebar-text no-select">
    <!-- 账户切换区域 -->
    <div class="p-2">
      <AccountSwitcher
        :collapsed="uiStore.sidebarCollapsed"
        @add-account="emit('addAccount')"
      />
    </div>

    <!-- 撰写按钮 -->
    <div class="px-2 py-1.5 flex gap-1">
      <button
        class="flex-1 flex items-center justify-center gap-2 h-9 bg-accent hover:bg-accent-hover text-white rounded-lg text-sm font-medium interactive-btn"
        @click="composeStore.openNewEmail()"
      >
        <Plus :size="16" />
        <span v-if="!uiStore.sidebarCollapsed">撰写</span>
      </button>
      <button
        class="flex items-center justify-center w-9 h-9 bg-bg-secondary hover:bg-bg-hover border border-border-primary rounded-lg text-text-secondary interactive-btn"
        :class="{ 'opacity-50 cursor-not-allowed': !uiStore.isOnline }"
        :disabled="mailboxStore.syncing || !uiStore.isOnline"
        :title="!uiStore.isOnline ? '离线状态' : mailboxStore.syncing ? '同步中...' : '同步邮件'"
        @click="handleSync"
      >
        <WifiOff v-if="!uiStore.isOnline" :size="16" class="text-warning" />
        <RefreshCw v-else :size="16" :class="{ 'animate-spin': mailboxStore.syncing }" />
      </button>
    </div>

    <!-- 文件夹列表 -->
    <nav class="flex-1 overflow-y-auto py-1 hide-scrollbar">
      <div class="px-2 space-y-0.5">
        <FolderItem
          v-for="folder in folders"
          :key="folder.id"
          :icon="folder.icon"
          :label="folder.label"
          :count="folder.count"
          :active="mailboxStore.currentFolder === folder.id"
          :collapsed="uiStore.sidebarCollapsed"
          @click="handleFolderClick(folder.id)"
        />
      </div>
    </nav>

    <!-- 底部：设置 + 折叠 -->
    <div class="p-2 border-t border-border-primary flex items-center gap-0.5">
      <button
        class="flex-1 flex items-center gap-2 p-2 rounded-md text-sidebar-text/60 hover:text-sidebar-text hover:bg-sidebar-hover transition-colors"
        title="设置"
        @click="handleSettingsClick"
      >
        <Settings :size="16" />
        <span v-if="!uiStore.sidebarCollapsed" class="text-sm">设置</span>
      </button>
      <button
        class="p-2 rounded-md text-sidebar-text/60 hover:text-sidebar-text hover:bg-sidebar-hover transition-colors"
        :title="uiStore.sidebarCollapsed ? '展开侧边栏' : '折叠侧边栏'"
        @click="uiStore.toggleSidebar"
      >
        <PanelLeftClose v-if="!uiStore.sidebarCollapsed" :size="16" />
        <PanelLeftOpen v-else :size="16" />
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { useRouter } from 'vue-router'
import { useUiStore } from '@/stores/ui'
import { useMailboxStore } from '@/stores/mailbox'
import { useAccountStore } from '@/stores/account'
import { useNotificationStore } from '@/stores/notification'
import { useComposeStore } from '@/stores/compose'
import {
  Inbox,
  Plus,
  Settings,
  PanelLeftClose,
  PanelLeftOpen,
  RefreshCw,
  WifiOff,
  ShieldAlert,
  Star,
  Trash2,
  Send,
  FileText
} from 'lucide-vue-next'
import FolderItem from './FolderItem.vue'
import AccountSwitcher from '@/components/accounts/AccountSwitcher.vue'
import { getPhishingUnreadCount, getPhishingEmails, getUnreadCount, getStarredEmails, getStarredCount, getTrashEmails, getSentEmails, getSentCount, getDraftEmails, getDraftCount } from '@/services/email/emailService'

const emit = defineEmits<{
  addAccount: []
}>()

const router = useRouter()
const uiStore = useUiStore()
const mailboxStore = useMailboxStore()
const accountStore = useAccountStore()
const notificationStore = useNotificationStore()
const composeStore = useComposeStore()

// 钓鱼邮件未读数量
const phishingUnreadCount = ref(0)
// 星标邮件数量
const starredCount = ref(0)

// 加载钓鱼邮件未读数量
async function loadPhishingUnreadCount() {
  const account = accountStore.activeAccount
  if (account) {
    try {
      phishingUnreadCount.value = await getPhishingUnreadCount(account.id)
    } catch {
      phishingUnreadCount.value = 0
    }
  }
}

// 加载星标邮件数量
async function loadStarredCount() {
  const account = accountStore.activeAccount
  if (account) {
    try {
      starredCount.value = await getStarredCount(account.id)
    } catch {
      starredCount.value = 0
    }
  }
}

// 加载已发送邮件数量
async function loadSentCount() {
  const account = accountStore.activeAccount
  if (account) {
    try {
      mailboxStore.sentCount = await getSentCount(account.id)
    } catch {
      mailboxStore.sentCount = 0
    }
  }
}

// 加载草稿邮件数量
async function loadDraftCount() {
  const account = accountStore.activeAccount
  if (account) {
    try {
      mailboxStore.draftCount = await getDraftCount(account.id)
    } catch {
      mailboxStore.draftCount = 0
    }
  }
}

// 加载收件箱未读数量
async function loadInboxUnreadCount() {
  const account = accountStore.activeAccount
  if (account) {
    try {
      mailboxStore.unreadCount = await getUnreadCount(account.id)
    } catch {
      mailboxStore.unreadCount = 0
    }
  }
}

// 监听账户变化，更新所有未读数量并自动加载收件箱
watch(
  () => accountStore.activeAccount,
  async (account) => {
    if (!account) return

    // 更新未读数量
    await loadInboxUnreadCount()
    await loadPhishingUnreadCount()
    await loadStarredCount()
    await loadSentCount()
    await loadDraftCount()

    // 自动切换到收件箱并加载邮件
    mailboxStore.setCurrentFolder('inbox')
    await mailboxStore.loadEmails(account.id, 'inbox')

    // 自动选择第一封邮件并显示详情
    if (mailboxStore.emails.length > 0) {
      const firstEmail = mailboxStore.emails[0]
      await mailboxStore.selectEmail(firstEmail.id)
      await mailboxStore.markAsRead(firstEmail.id)
    } else {
      mailboxStore.clearSelection()
    }
  },
  { immediate: true }
)

// 监听邮件列表变化，当钓鱼邮件被阅读后刷新未读数量
watch(
  () => mailboxStore.emails.map(e => e.isRead),
  () => {
    // 如果当前在钓鱼邮件文件夹，邮件阅读后需要刷新未读数量
    if (mailboxStore.currentFolder === 'phishing') {
      loadPhishingUnreadCount()
    }
  }
)

// 监听同步完成，更新钓鱼邮件未读数量
watch(
  () => mailboxStore.syncing,
  (isSyncing, wasSyncing) => {
    // 同步完成时更新钓鱼邮件数量
    if (wasSyncing && !isSyncing) {
      loadPhishingUnreadCount()
    }
  }
)

// 钓鱼检测现在在后端执行，邮件同步后会自动刷新列表
// 不再需要监听前端钓鱼检测事件

const folders = computed(() => [
  { id: 'inbox', icon: Inbox, label: '收件箱', count: mailboxStore.unreadCount },
  { id: 'starred', icon: Star, label: '星标邮件', count: starredCount.value },
  { id: 'sent', icon: Send, label: '已发送', count: 0 },  // 已发送不显示计数
  { id: 'drafts', icon: FileText, label: '草稿箱', count: mailboxStore.draftCount },  // 草稿显示计数作为提醒
  { id: 'phishing', icon: ShieldAlert, label: '钓鱼邮件', count: phishingUnreadCount.value },
  { id: 'trash', icon: Trash2, label: '回收站', count: 0 }  // 回收站不显示计数
])

async function handleFolderClick(folderId: string) {
  mailboxStore.setCurrentFolder(folderId)
  mailboxStore.clearSelection()
  const account = accountStore.activeAccount
  if (account) {
    if (folderId === 'phishing') {
      // 加载钓鱼邮件列表
      await loadPhishingEmails(account.id)
    } else if (folderId === 'starred') {
      // 加载星标邮件列表
      await loadStarredEmails(account.id)
    } else if (folderId === 'trash') {
      // 加载回收站邮件列表
      await loadTrashEmails(account.id)
    } else if (folderId === 'sent') {
      // 加载已发送邮件列表
      await loadSentEmails(account.id)
    } else if (folderId === 'drafts') {
      // 加载草稿邮件列表
      await loadDraftEmails(account.id)
    } else {
      await mailboxStore.loadEmails(account.id, folderId)
    }
  }
}

async function loadPhishingEmails(accountId: string) {
  mailboxStore.loading = true
  try {
    const emails = await getPhishingEmails(accountId)
    mailboxStore.emails = emails
  } catch (e) {
    console.error('加载钓鱼邮件失败:', e)
  } finally {
    mailboxStore.loading = false
  }
}

async function loadStarredEmails(accountId: string) {
  mailboxStore.loading = true
  try {
    const emails = await getStarredEmails(accountId)
    mailboxStore.emails = emails
    // 更新星标数量
    starredCount.value = await getStarredCount(accountId)
  } catch (e) {
    console.error('加载星标邮件失败:', e)
  } finally {
    mailboxStore.loading = false
  }
}

async function loadTrashEmails(accountId: string) {
  mailboxStore.loading = true
  try {
    const emails = await getTrashEmails(accountId)
    mailboxStore.emails = emails
  } catch (e) {
    console.error('加载回收站邮件失败:', e)
  } finally {
    mailboxStore.loading = false
  }
}

async function loadSentEmails(accountId: string) {
  mailboxStore.loading = true
  try {
    const emails = await getSentEmails(accountId)
    mailboxStore.emails = emails
    // 更新已发送数量
    mailboxStore.sentCount = await getSentCount(accountId)
  } catch (e) {
    console.error('加载已发送邮件失败:', e)
  } finally {
    mailboxStore.loading = false
  }
}

async function loadDraftEmails(accountId: string) {
  mailboxStore.loading = true
  try {
    const emails = await getDraftEmails(accountId)
    mailboxStore.emails = emails
    // 更新草稿数量
    mailboxStore.draftCount = await getDraftCount(accountId)
  } catch (e) {
    console.error('加载草稿邮件失败:', e)
  } finally {
    mailboxStore.loading = false
  }
}

async function handleSync() {
  if (!uiStore.isOnline) {
    notificationStore.warning('离线状态', '无法同步，请检查网络连接')
    return
  }

  const account = accountStore.activeAccount
  if (account && !mailboxStore.syncing) {
    try {
      await mailboxStore.sync(account.id)
    } catch (e) {
      // Error notification already shown by mailboxStore.sync()
      console.warn('Sync failed:', e)
    }
  }
}

function handleSettingsClick() {
  router.push('/settings/accounts')
}
</script>