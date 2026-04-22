<template>
  <div class="flex flex-col h-screen overflow-hidden text-text-primary">
    <!-- 静态背景 -->
    <div class="static-bg" aria-hidden="true" />

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
        class="flex-shrink-0 border-r border-border-primary overflow-hidden solid-panel"
        style="width: 320px"
      >
        <EmailList />
      </div>

      <!-- 右侧阅读面板或撰写界面 -->
      <main class="flex-1 overflow-hidden glass-panel">
        <ComposeEmail
          v-if="composeStore.isOpen"
          ref="composeEmailRef"
          :reply-to="composeStore.state.replyTo"
          :forward="composeStore.state.forward"
          :draft-id="composeStore.state.draft?.draftId"
          :draft-data="composeStore.state.draft"
          :draft-attachments="composeStore.state.draft?.attachments"
          @close="composeStore.close()"
          @sent="handleEmailSent"
          @saved="handleDraftSaved"
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

    <!-- 切换确认对话框 -->
    <ConfirmDialog
      :is-open="showSwitchConfirm"
      title="保存邮件"
      message="邮件内容已修改，是否需要保存？"
      confirm-text="保存"
      neutral-text="不保存"
      cancel-text="取消"
      @confirm="handleSwitchSave"
      @neutral="handleSwitchDiscard"
      @cancel="handleSwitchCancel"
    />

    <!-- Notifications -->
    <Notifications />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { useUiStore } from '@/stores/ui'
import { useAccountStore } from '@/stores/account'
import { useMailboxStore } from '@/stores/mailbox'
import { useNotificationStore } from '@/stores/notification'
import { useComposeStore } from '@/stores/compose'
import { waitForDatabase, getDraftCount, getDraftDetail, getDraftAttachments } from '@/services/email/emailService'
import TitleBar from './TitleBar.vue'
import Sidebar from './Sidebar.vue'
import EmailList from './EmailList.vue'
import ReadingPane from './ReadingPane.vue'
import ComposeEmail from '@/components/email/ComposeEmail.vue'
import AddAccount from '@/components/accounts/AddAccount.vue'
import Notifications from '@/components/common/Notifications.vue'
import ConfirmDialog from '@/components/ui/ConfirmDialog.vue'

const uiStore = useUiStore()
const accountStore = useAccountStore()
const mailboxStore = useMailboxStore()
const notificationStore = useNotificationStore()
const composeStore = useComposeStore()

const showAddAccount = ref(false)

// ComposeEmail 组件引用
const composeEmailRef = ref<{ saveDraft: () => Promise<void>, hasChanges: () => boolean } | null>(null)

// 切换确认对话框状态
const showSwitchConfirm = ref(false)

// 监听 pendingAction，检查是否有变化后决定是否弹出确认对话框
watch(
  () => composeStore.pendingAction,
  async (action) => {
    if (action) {
      // 检查是否有变化
      const hasChanges = composeEmailRef.value?.hasChanges() ?? false
      console.log('[MailLayout] pendingAction 触发, hasChanges:', hasChanges)

      if (hasChanges) {
        // 有变化，弹出确认对话框
        showSwitchConfirm.value = true
      } else {
        // 无变化，直接执行操作
        console.log('[MailLayout] 无变化，直接执行操作')
        composeStore.clearPendingAction()
        composeStore.close()
        await executePendingAction(action)
      }
    }
  }
)

onMounted(async () => {
  uiStore.init()
  mailboxStore.init()

  // 等待数据库初始化完成
  const dbReady = await waitForDatabase(5000)
  if (!dbReady) {
    notificationStore.error('初始化失败', '数据库初始化超时，请重启应用')
    return
  }

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

  // 等待数据库就绪
  const dbReady = await waitForDatabase(1000)
  if (!dbReady) {
    notificationStore.error('错误', '数据库未就绪')
    return
  }

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
  // 邮件发送成功后，关闭撰写界面并刷新相关列表
  composeStore.close()

  const account = accountStore.activeAccount
  if (account) {
    // 刷新当前文件夹的邮件列表
    if (mailboxStore.currentFolder === 'drafts') {
      // 从草稿箱发送，需要刷新草稿列表
      mailboxStore.loadEmails(account.id, 'drafts')
    } else if (mailboxStore.currentFolder === 'sent') {
      // 刷新已发送列表
      mailboxStore.loadEmails(account.id, 'sent')
    }
  }
}

function handleDraftSaved() {
  // 草稿保存成功后，刷新草稿列表和计数
  const account = accountStore.activeAccount
  if (account) {
    // 更新草稿计数
    getDraftCount(account.id).then(count => {
      mailboxStore.draftCount = count
    })
    // 如果当前在草稿箱，刷新列表
    if (mailboxStore.currentFolder === 'drafts') {
      mailboxStore.loadEmails(account.id, 'drafts')
    }
  }
}

// 切换确认 - 保存草稿后切换
async function handleSwitchSave() {
  showSwitchConfirm.value = false
  const action = composeStore.pendingAction
  composeStore.clearPendingAction()

  // 调用 ComposeEmail 的保存方法
  if (composeEmailRef.value) {
    try {
      await composeEmailRef.value.saveDraft()
      // 保存成功后更新草稿计数
      const account = accountStore.activeAccount
      if (account) {
        mailboxStore.draftCount = await getDraftCount(account.id)
      }
    } catch (e) {
      notificationStore.error('保存失败', e instanceof Error ? e.message : '保存草稿失败')
    }
  }

  composeStore.close()
  await executePendingAction(action)
}

// 切换确认 - 不保存直接切换
async function handleSwitchDiscard() {
  showSwitchConfirm.value = false
  const action = composeStore.pendingAction
  composeStore.clearPendingAction()
  composeStore.close()
  await executePendingAction(action)
}

// 切换确认 - 取消切换
function handleSwitchCancel() {
  showSwitchConfirm.value = false
  composeStore.clearPendingAction()
}

// 执行待处理的操作
async function executePendingAction(action: any) {
  if (!action) return

  const account = accountStore.activeAccount
  if (!account) return

  if (action.type === 'switchFolder' && action.folderId) {
    // 执行文件夹切换
    mailboxStore.setCurrentFolder(action.folderId)
    mailboxStore.clearSelection()
    await mailboxStore.loadEmails(account.id, action.folderId)
    // 更新侧边栏计数
    if (action.folderId === 'drafts') {
      mailboxStore.draftCount = await getDraftCount(account.id)
    }
  } else if (action.type === 'selectEmail' && action.emailId) {
    const email = mailboxStore.emails.find(e => e.id === action.emailId)
    if (!email) return

    // 草稿邮件：打开编辑器
    if (mailboxStore.currentFolder === 'drafts') {
      const draftId = action.emailId.replace('draft-', '')
      try {
        const detail = await getDraftDetail(draftId)
        const draftAttachments = await getDraftAttachments(draftId)
        const attachments = draftAttachments.map(a => ({
          id: a.id,
          filename: a.filename,
          contentType: a.content_type || 'application/octet-stream',
          size: a.file_size,
          path: a.file_path || '',
          content: a.content || undefined
        }))
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
        }
      } catch (e) {
        notificationStore.error('加载失败', '无法加载草稿详情')
      }
    } else {
      // 其他邮件：正常选择并标记已读
      await mailboxStore.selectEmail(action.emailId)
      await mailboxStore.markAsRead(action.emailId)
    }
  }
}
</script>