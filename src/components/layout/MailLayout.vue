<template>
  <div class="flex flex-col h-screen overflow-hidden text-text-primary">
    <!-- 动态背景 -->
    <div class="animated-bg" aria-hidden="true">
      <div class="blob" />
      <div class="blob" />
      <div class="blob" />
      <div class="blob" />
    </div>

    <!-- 标题栏 -->
    <TitleBar />

    <!-- 主内容区 -->
    <div class="flex flex-1 min-w-0 overflow-hidden">
      <!-- 左侧边栏 -->
      <aside
        class="flex-shrink-0 border-r border-border-primary transition-all duration-200 overflow-hidden glass-panel"
        :style="{ width: uiStore.sidebarCollapsed ? '56px' : '200px' }"
      >
        <Sidebar @add-account="showAddAccount = true" />
      </aside>

      <!-- 中间邮件列表 -->
      <div
        class="flex-shrink-0 border-r border-border-primary overflow-hidden glass-panel"
        style="width: 320px"
      >
        <EmailList />
      </div>

      <!-- 右侧阅读面板或撰写界面 -->
      <main class="flex-1 overflow-hidden glass-panel">
        <ComposeEmail
          v-if="composeStore.isOpen"
          :reply-to="composeStore.state.replyTo"
          :forward="composeStore.state.forward"
          @close="composeStore.close()"
          @sent="handleEmailSent"
        />
        <ReadingPane v-else />
      </main>
    </div>

    <!-- Add Account Modal -->
    <AddAccount
      :is-open="showAddAccount"
      @close="showAddAccount = false"
      @success="handleAccountAdded"
    />

    <!-- Notifications -->
    <Notifications />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useUiStore } from '@/stores/ui'
import { useAccountStore } from '@/stores/account'
import { useMailboxStore } from '@/stores/mailbox'
import { useNotificationStore } from '@/stores/notification'
import { useComposeStore } from '@/stores/compose'
import TitleBar from './TitleBar.vue'
import Sidebar from './Sidebar.vue'
import EmailList from './EmailList.vue'
import ReadingPane from './ReadingPane.vue'
import ComposeEmail from '@/components/email/ComposeEmail.vue'
import AddAccount from '@/components/accounts/AddAccount.vue'
import Notifications from '@/components/common/Notifications.vue'

const uiStore = useUiStore()
const accountStore = useAccountStore()
const mailboxStore = useMailboxStore()
const notificationStore = useNotificationStore()
const composeStore = useComposeStore()

const showAddAccount = ref(false)

onMounted(async () => {
  uiStore.init()
  mailboxStore.init()
  await accountStore.init()

  const account = accountStore.activeAccount
  if (account) {
    // Always load emails from local database first (offline-capable)
    await mailboxStore.loadEmails(account.id)

    // If online, attempt to sync from server
    if (uiStore.isOnline) {
      trySync(account.id)
    } else {
      notificationStore.warning('离线模式', '当前处于离线状态，显示本地缓存邮件')
    }
  } else {
    // No accounts - show add account modal
    showAddAccount.value = true
  }
})

// Attempt sync with error handling
async function trySync(accountId: string) {
  try {
    await mailboxStore.sync(accountId)
  } catch (e) {
    // Sync failed - already have local emails loaded
    // Error notification is shown by mailboxStore.sync()
    // Just log for debugging
    console.warn('Sync failed, using local data:', e)
  }
}

async function handleAccountAdded() {
  showAddAccount.value = false

  // The account store already has the new account from addAccount()
  const account = accountStore.activeAccount
  if (account) {
    // Load from database first
    await mailboxStore.loadEmails(account.id)

    // Sync emails from server if online
    if (uiStore.isOnline) {
      trySync(account.id)
    }
  }
}

function handleEmailSent() {
  // 邮件发送成功后，可以选择刷新邮件列表或关闭撰写界面
  composeStore.close()
}
</script>