import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import {
  syncEmails,
  getEmails,
  getEmailDetail,
  markAsRead as markAsReadService,
  toggleStar as toggleStarService,
  getUnreadCount,
  getAttachmentsWithStatus,
  type SyncResult
} from '@/services/email/emailService'
import { useNotificationStore } from '@/stores/notification'
import { useAccountStore } from '@/stores/account'
import { parseSyncError } from '@/services/email/errorUtils'

export interface Email {
  id: string
  accountId: string
  fromAddress: string | null
  fromName: string | null
  toAddresses: string[]
  ccAddresses: string[]
  subject: string | null
  snippet: string
  date: number
  isRead: boolean
  isStarred: boolean
  hasAttachment: boolean
  bodyHtml?: string | null
  bodyText?: string | null
  attachments?: Attachment[]
  /// 是否为钓鱼邮件
  isPhishing?: boolean
  /// 钓鱼检测原因
  phishingReasons?: string[]
}

export interface Attachment {
  id: string
  filename: string
  mimeType: string
  size: number
  /// 是否已下载到本地缓存
  isDownloaded?: boolean
  /// 本地缓存路径
  localPath?: string | null
}

export const useMailboxStore = defineStore('mailbox', () => {
  // State
  const emails = ref<Email[]>([])
  const selectedEmailId = ref<string | null>(null)
  const loading = ref(false)
  const syncing = ref(false)
  const syncProgress = ref<string | null>(null)
  const error = ref<string | null>(null)
  const currentFolder = ref('inbox')
  const unreadCount = ref(0)

  // Getters
  const selectedEmail = computed(() => {
    if (!selectedEmailId.value) return null
    return emails.value.find(e => e.id === selectedEmailId.value) || null
  })

  const localUnreadCount = computed(() => emails.value.filter(e => !e.isRead).length)

  // Actions
  async function sync(accountId: string): Promise<SyncResult> {
    const notificationStore = useNotificationStore()
    const accountStore = useAccountStore()
    syncing.value = true
    syncProgress.value = '正在连接服务器...'
    error.value = null

    try {
      syncProgress.value = '正在获取邮件列表...'
      const result = await syncEmails(accountId)

      console.log('[MailboxStore] 同步结果:', result)

      // 钓鱼检测现在在后端执行，邮件保存时已自动完成检测
      // 钓鱼邮件不会出现在收件箱中（数据库查询过滤 is_phishing=0）

      syncProgress.value = '正在更新本地数据...'
      // Refresh email list after sync
      await loadEmails(accountId)

      // Update unread count
      unreadCount.value = await getUnreadCount(accountId)

      syncProgress.value = null

      // Show success notification if new emails were synced
      if (result.new > 0) {
        notificationStore.success('同步完成', `收到 ${result.new} 封新邮件`)
      }

      return result
    } catch (e) {
      // 获取账户协议类型用于正确的错误信息
      const account = accountStore.accounts.find(a => a.id === accountId)
      const protocol = account?.protocol === 'imap' ? 'imap' : 'pop3'
      const parsed = parseSyncError(e, protocol)
      error.value = parsed.message

      // Show error notification
      notificationStore.error(parsed.title, parsed.message)

      syncProgress.value = null
      throw e
    } finally {
      syncing.value = false
    }
  }

  async function loadEmails(accountId: string, folder: string = 'inbox') {
    loading.value = true
    error.value = null
    currentFolder.value = folder

    try {
      // 根据文件夹类型加载不同内容
      if (folder === 'inbox') {
        emails.value = await getEmails(accountId)
        unreadCount.value = await getUnreadCount(accountId)
      } else if (folder === 'sent' || folder === 'drafts' || folder === 'archive' || folder === 'trash') {
        // 这些文件夹功能尚未实现，显示空列表
        emails.value = []
      } else {
        // 其他文件夹默认显示收件箱
        emails.value = await getEmails(accountId)
        unreadCount.value = await getUnreadCount(accountId)
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : '加载失败'
    } finally {
      loading.value = false
    }
  }

  async function selectEmail(id: string) {
    selectedEmailId.value = id

    // Load full email detail if not already loaded
    const email = emails.value.find(e => e.id === id)
    if (email && !email.bodyHtml && !email.bodyText) {
      try {
        const detail = await getEmailDetail(id)
        if (detail && email) {
          email.bodyHtml = detail.body_html
          email.bodyText = detail.body_text

          // 获取附件下载状态
          const attachmentsWithStatus = await getAttachmentsWithStatus(id)
          email.attachments = attachmentsWithStatus.map(a => ({
            id: a.id,
            filename: a.filename,
            mimeType: a.mime_type,
            size: a.size,
            isDownloaded: a.is_downloaded,
            localPath: a.local_path
          }))
        }
      } catch (e) {
        console.error('Failed to load email detail:', e)
      }
    }
  }

  async function markAsRead(id: string, isRead: boolean = true) {
    const email = emails.value.find(e => e.id === id)
    if (email) {
      try {
        await markAsReadService(id, isRead)
        email.isRead = isRead

        // Update unread count
        if (email.accountId) {
          unreadCount.value = await getUnreadCount(email.accountId)
        }
      } catch (e) {
        console.error('Failed to mark as read:', e)
        throw e
      }
    }
  }

  async function toggleStar(id: string) {
    const email = emails.value.find(e => e.id === id)
    if (email) {
      try {
        const newStarred = !email.isStarred
        await toggleStarService(id, newStarred)
        email.isStarred = newStarred
      } catch (e) {
        console.error('Failed to toggle star:', e)
        throw e
      }
    }
  }

  function clearSelection() {
    selectedEmailId.value = null
  }

  function setCurrentFolder(folder: string) {
    currentFolder.value = folder
  }

  function clearError() {
    error.value = null
  }

  // 初始化 - 从 localStorage 恢复状态
  function init() {
    const savedFolder = localStorage.getItem('currentFolder')
    if (savedFolder) currentFolder.value = savedFolder
  }

  function saveState() {
    localStorage.setItem('currentFolder', currentFolder.value)
  }

  return {
    emails,
    selectedEmailId,
    selectedEmail,
    loading,
    syncing,
    syncProgress,
    error,
    currentFolder,
    unreadCount,
    localUnreadCount,
    sync,
    loadEmails,
    selectEmail,
    markAsRead,
    toggleStar,
    clearSelection,
    setCurrentFolder,
    clearError,
    init,
    saveState
  }
})