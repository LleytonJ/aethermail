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
  getStarredEmails,
  getStarredCount,
  moveToTrash as moveToTrashService,
  getTrashEmails,
  restoreEmail as restoreEmailService,
  permanentDelete as permanentDeleteService,
  getSentEmails,
  getSentCount,
  getDraftEmails,
  getDraftCount,
  deleteDraft as deleteDraftService,
  getEmailsForThreading,
  assignThreads,
  type SyncResult
} from '@/services/email/emailService'
import { buildThreads, type ThreadGroup, type ThreadableMessage } from '@/services/threading/threadBuilder'
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
  const starredCount = ref(0)
  const sentCount = ref(0)
  const draftCount = ref(0)

  // Thread state
  const threadGroups = ref<ThreadGroup[]>([])
  const selectedThreadId = ref<string | null>(null)
  const threadEmails = ref<Email[]>([])
  const loadingThread = ref(false)

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

      // 计算邮件线程分组
      syncProgress.value = '正在计算邮件会话...'
      await computeThreads(accountId)

      // 如果同步前有选中的邮件，重新加载该邮件的正文
      // 因为 loadEmails 会替换邮件列表，新对象不包含正文
      const previousSelectedId = selectedEmailId.value
      if (previousSelectedId) {
        // 检查该邮件是否还在新列表中
        const stillExists = emails.value.some(e => e.id === previousSelectedId)
        if (stillExists) {
          // 重新调用 selectEmail 加载正文
          await selectEmail(previousSelectedId)
        }
      }

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
      } else if (folder === 'starred') {
        emails.value = await getStarredEmails(accountId)
        starredCount.value = await getStarredCount(accountId)
      } else if (folder === 'trash') {
        emails.value = await getTrashEmails(accountId)
      } else if (folder === 'sent') {
        emails.value = await getSentEmails(accountId)
        sentCount.value = await getSentCount(accountId)
      } else if (folder === 'drafts') {
        emails.value = await getDraftEmails(accountId)
        draftCount.value = await getDraftCount(accountId)
      } else if (folder === 'archive') {
        // 归档功能尚未实现，显示空列表
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

        // Update starred count
        if (email.accountId) {
          starredCount.value = await getStarredCount(email.accountId)
        }
      } catch (e) {
        console.error('Failed to toggle star:', e)
        throw e
      }
    }
  }

  // 移动邮件到回收站（软删除）
  async function moveToTrash(id: string) {
    const email = emails.value.find(e => e.id === id)
    if (!email) return

    try {
      await moveToTrashService(id)

      // 从列表中移除邮件
      emails.value = emails.value.filter(e => e.id !== id)

      // 如果删除的是当前选中的邮件，清空选中状态
      if (selectedEmailId.value === id) {
        selectedEmailId.value = null
      }

      // 更新未读计数（如果删除的是未读邮件）
      if (!email.isRead && email.accountId) {
        unreadCount.value = await getUnreadCount(email.accountId)
      }

      // 更新星标计数（如果删除的是星标邮件）
      if (email.isStarred && email.accountId) {
        starredCount.value = await getStarredCount(email.accountId)
      }
    } catch (e) {
      console.error('Failed to move to trash:', e)
      throw e
    }
  }

  // 从回收站恢复邮件
  async function restoreEmail(id: string, targetFolder: 'inbox' | 'phishing') {
    const email = emails.value.find(e => e.id === id)
    if (!email) return

    try {
      await restoreEmailService(id, targetFolder)

      // 从回收站列表中移除邮件
      emails.value = emails.value.filter(e => e.id !== id)

      // 清空选中状态
      if (selectedEmailId.value === id) {
        selectedEmailId.value = null
      }
    } catch (e) {
      console.error('Failed to restore email:', e)
      throw e
    }
  }

  // 永久删除邮件（从回收站）
  async function permanentDelete(id: string) {
    const email = emails.value.find(e => e.id === id)
    if (!email) return

    try {
      await permanentDeleteService(id)

      // 从列表中移除邮件
      emails.value = emails.value.filter(e => e.id !== id)

      // 清空选中状态
      if (selectedEmailId.value === id) {
        selectedEmailId.value = null
      }
    } catch (e) {
      console.error('Failed to permanently delete email:', e)
      throw e
    }
  }

  // 删除草稿
  async function deleteDraft(id: string) {
    const email = emails.value.find(e => e.id === id)
    if (!email) return

    try {
      // 草稿的 id 是 "draft-{draft_id}"，需要提取 draft_id
      const draftId = email.id.replace('draft-', '')
      await deleteDraftService(draftId)

      // 从列表中移除草稿
      emails.value = emails.value.filter(e => e.id !== id)

      // 清空选中状态
      if (selectedEmailId.value === id) {
        selectedEmailId.value = null
      }

      // 更新草稿计数
      if (email.accountId) {
        draftCount.value = await getDraftCount(email.accountId)
      }
    } catch (e) {
      console.error('Failed to delete draft:', e)
      throw e
    }
  }

  function clearSelection() {
    selectedEmailId.value = null
    selectedThreadId.value = null
    threadEmails.value = []
  }

  function setCurrentFolder(folder: string) {
    currentFolder.value = folder
  }

  function clearError() {
    error.value = null
  }

  // ==================== 邮件会话功能 ====================

  // Get selected thread
  const selectedThread = computed(() => {
    if (!selectedThreadId.value) return null
    return threadGroups.value.find(t => t.threadId === selectedThreadId.value) || null
  })

  // 计算线程分组（在同步后调用）
  async function computeThreads(accountId: string): Promise<void> {
    try {
      console.log('[MailboxStore] computeThreads 开始, accountId:', accountId)

      // 获取带 threading headers 的邮件
      const threadEmailsData = await getEmailsForThreading(accountId)
      console.log('[MailboxStore] 获取到邮件数量:', threadEmailsData.length)

      // 打印每封邮件的 threading headers
      threadEmailsData.forEach(e => {
        console.log('[MailboxStore] 邮件:', {
          id: e.id,
          subject: e.subject,
          message_id: e.message_id_header,
          in_reply_to: e.in_reply_to_header,
          references: e.references_header
        })
      })

      // 转换为 ThreadableMessage 格式
      // 如果 message_id_header 为空，生成一个 synthetic ID
      const messages: ThreadableMessage[] = threadEmailsData.map(e => {
        const messageId = e.message_id_header || `synthetic-${e.id}`;
        return {
          id: e.id,
          messageId,
          inReplyTo: e.in_reply_to_header,
          references: e.references_header,
          subject: e.subject,
          date: e.date
        };
      })

      // 计算线程分组
      threadGroups.value = buildThreads(messages)
      console.log('[MailboxStore] 计算出线程数量:', threadGroups.value.length)
      threadGroups.value.forEach(t => {
        console.log('[MailboxStore] 线程:', {
          threadId: t.threadId,
          messageIds: t.messageIds,
          count: t.messageIds.length
        })
      })

      // 将线程分配写入数据库
      if (threadGroups.value.length > 0) {
        const assignments = threadGroups.value.flatMap(thread =>
          thread.messageIds.map(msgId => ({
            email_id: msgId,
            thread_id: thread.threadId
          }))
        )
        await assignThreads(assignments)
      }

      console.log('[MailboxStore] 计算了 %d 个线程', threadGroups.value.length)
      threadGroups.value.forEach((t, i) => {
        console.log('[MailboxStore] 线程 %d:', i, {
          threadId: t.threadId,
          messageCount: t.messageIds.length,
          messageIds: t.messageIds.slice(0, 5) // 只显示前5个
        });
      })
    } catch (e) {
      console.error('[MailboxStore] 计算线程失败:', e)
    }
  }

  // 选择线程并加载邮件
  async function selectThread(threadId: string): Promise<void> {
    console.log('[MailboxStore] selectThread 开始:', threadId)
    selectedThreadId.value = threadId
    loadingThread.value = true

    try {
      const threadGroup = threadGroups.value.find(t => t.threadId === threadId)
      if (!threadGroup) {
        console.log('[MailboxStore] 未找到线程组:', threadId)
        threadEmails.value = []
        return
      }

      console.log('[MailboxStore] 线程组 messageIds:', threadGroup.messageIds)

      // 从 emails 中查找该线程的邮件
      threadEmails.value = threadGroup.messageIds
        .map(msgId => emails.value.find(e => e.id === msgId))
        .filter((e): e is Email => e !== undefined)
        .sort((a, b) => a.date - b.date)  // 按时间升序排列

      console.log('[MailboxStore] threadEmails 数量:', threadEmails.value.length)
      console.log('[MailboxStore] threadEmails:', threadEmails.value.map(e => ({
        id: e.id,
        subject: e.subject,
        hasBody: !!e.bodyHtml || !!e.bodyText
      })))

      // 加载所有邮件的详情
      for (const email of threadEmails.value) {
        if (!email.bodyHtml && !email.bodyText) {
          try {
            const detail = await getEmailDetail(email.id)
            if (detail) {
              email.bodyHtml = detail.body_html
              email.bodyText = detail.body_text
              email.attachments = detail.attachments.map(a => ({
                id: a.id,
                filename: a.filename,
                mimeType: a.mime_type,
                size: a.size
              }))
            }
          } catch (e) {
            console.error('[MailboxStore] 加载邮件详情失败:', email.id, e)
          }
        }
      }

      console.log('[MailboxStore] 加载详情后 threadEmails:', threadEmails.value.map(e => ({
        id: e.id,
        subject: e.subject,
        hasBody: !!e.bodyHtml || !!e.bodyText
      })))
    } catch (e) {
      console.error('[MailboxStore] 加载线程邮件失败:', e)
      threadEmails.value = []
    } finally {
      loadingThread.value = false
    }
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
    starredCount,
    sentCount,
    draftCount,
    localUnreadCount,
    // Thread state
    threadGroups,
    selectedThreadId,
    selectedThread,
    threadEmails,
    loadingThread,
    sync,
    loadEmails,
    selectEmail,
    markAsRead,
    toggleStar,
    moveToTrash,
    restoreEmail,
    permanentDelete,
    deleteDraft,
    clearSelection,
    setCurrentFolder,
    clearError,
    // Thread actions
    computeThreads,
    selectThread,
    init,
    saveState
  }
})