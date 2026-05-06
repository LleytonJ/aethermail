/**
 * 邮件服务层 - 调用 Tauri 后端命令
 */

import { invoke } from '@tauri-apps/api/core'
import type { Email } from '@/stores/mailbox'

/**
 * 检查数据库是否就绪
 */
export async function isDatabaseReady(): Promise<boolean> {
  return invoke<boolean>('is_database_ready')
}

/**
 * 等待数据库就绪（带超时）
 */
export async function waitForDatabase(timeoutMs: number = 5000): Promise<boolean> {
  return invoke<boolean>('wait_for_database', { timeoutMs })
}

/**
 * 同步结果
 */
export interface SyncResult {
  total: number
  new: number
  success: boolean
  message: string
}

/**
 * 邮件详情（含完整正文和附件）
 */
export interface EmailDetail {
  id: string
  account_id: string
  from_address: string | null
  from_name: string | null
  to_addresses: string[]
  cc_addresses: string[]
  subject: string | null
  body_text: string | null
  body_html: string | null
  date: number
  is_read: boolean
  is_starred: boolean
  has_attachment: boolean
  attachments: AttachmentInfo[]
  is_phishing: boolean
  phishing_reasons: string[] | null
}

/**
 * 附件信息
 */
export interface AttachmentInfo {
  id: string
  filename: string
  mime_type: string
  size: number
}

/**
 * 附件信息（含下载状态）
 */
export interface AttachmentInfoWithStatus {
  id: string
  filename: string
  mime_type: string
  size: number
  is_downloaded: boolean
  local_path: string | null
}

/**
 * 缓存统计信息
 */
export interface CacheStats {
  file_count: number
  total_size: number
  cache_path: string
}

/**
 * 同步账户邮件
 */
export async function syncEmails(accountId: string): Promise<SyncResult> {
  console.log('[emailService] syncEmails - 开始同步, accountId:', accountId)

  try {
    const result = await invoke<SyncResult>('sync_emails', { accountId })

    console.log('[emailService] syncEmails - 同步完成:', result)
    return result
  } catch (error) {
    console.error('[emailService] syncEmails - 同步失败:', error)
    throw error
  }
}

/**
 * 获取账户邮件列表
 */
export async function getEmails(accountId: string): Promise<Email[]> {
  const result = await invoke<any[]>('get_emails', { accountId })

  // 转换字段名（后端使用 snake_case，前端使用 camelCase）
  return result.map(e => ({
    id: e.id,
    accountId: e.account_id,
    fromAddress: e.from_address,
    fromName: e.from_name,
    toAddresses: e.to_addresses,
    ccAddresses: e.cc_addresses,
    subject: e.subject,
    snippet: e.snippet,
    date: e.date,
    isRead: e.is_read,
    isStarred: e.is_starred,
    hasAttachment: e.has_attachment,
    isPhishing: e.is_phishing,
    phishingReasons: e.phishing_reasons
  }))
}

/**
 * 获取邮件详情
 */
export async function getEmailDetail(id: string): Promise<EmailDetail | null> {
  const result = await invoke<EmailDetail | null>('get_email_detail', { id })

  if (!result) return null

  // 转换字段名
  return {
    id: result.id,
    account_id: result.account_id,
    from_address: result.from_address,
    from_name: result.from_name,
    to_addresses: result.to_addresses,
    cc_addresses: result.cc_addresses,
    subject: result.subject,
    body_text: result.body_text,
    body_html: result.body_html,
    date: result.date,
    is_read: result.is_read,
    is_starred: result.is_starred,
    has_attachment: result.has_attachment,
    attachments: result.attachments.map(a => ({
      id: a.id,
      filename: a.filename,
      mime_type: a.mime_type,
      size: a.size
    })),
    is_phishing: result.is_phishing,
    phishing_reasons: result.phishing_reasons
  }
}

/**
 * 标记邮件已读
 */
export async function markAsRead(id: string, isRead: boolean = true): Promise<void> {
  return invoke('mark_email_read', { id, isRead })
}

/**
 * 切换邮件星标
 */
export async function toggleStar(id: string, isStarred: boolean): Promise<void> {
  return invoke('toggle_email_star', { id, isStarred })
}

/**
 * 获取未读邮件数量
 */
export async function getUnreadCount(accountId: string): Promise<number> {
  return invoke<number>('get_unread_count', { accountId })
}

/**
 * 删除账户的所有邮件（账户删除时调用）
 */
export async function deleteAccountEmails(accountId: string): Promise<void> {
  return invoke('delete_account_emails', { accountId })
}

/**
 * 删除单封邮件
 */
export async function deleteEmail(id: string): Promise<void> {
  return invoke('delete_email', { id })
}

// ==================== 回收站功能 ====================

/**
 * 移动邮件到回收站
 */
export async function moveToTrash(id: string): Promise<void> {
  return invoke('move_to_trash', { id })
}

/**
 * 获取回收站邮件列表
 */
export async function getTrashEmails(accountId: string): Promise<Email[]> {
  const result = await invoke<any[]>('get_trash_emails', { accountId })

  // 转换字段名
  return result.map(e => ({
    id: e.id,
    accountId: e.account_id,
    fromAddress: e.from_address,
    fromName: e.from_name,
    toAddresses: e.to_addresses,
    ccAddresses: e.cc_addresses,
    subject: e.subject,
    snippet: e.snippet,
    date: e.date,
    isRead: e.is_read,
    isStarred: e.is_starred,
    hasAttachment: e.has_attachment,
    isPhishing: e.is_phishing,
    phishingReasons: e.phishing_reasons
  }))
}

/**
 * 获取回收站邮件数量
 */
export async function getTrashCount(accountId: string): Promise<number> {
  return invoke<number>('get_trash_count', { accountId })
}

/**
 * 恢复邮件（到收件箱或钓鱼邮件箱）
 */
export async function restoreEmail(id: string, targetFolder: 'inbox' | 'phishing'): Promise<void> {
  return invoke('restore_email', { id, targetFolder })
}

/**
 * 永久删除邮件
 */
export async function permanentDelete(id: string): Promise<void> {
  return invoke('permanent_delete', { id })
}

/**
 * 更新邮件钓鱼状态
 */
export async function updatePhishingStatus(
  id: string,
  isPhishing: boolean,
  phishingReasons: string[] | null,
  userOverridden?: boolean
): Promise<void> {
  return invoke('update_phishing_status', {
    id,
    isPhishing,
    phishingReasons,
    userOverridden
  })
}

/**
 * 获取钓鱼邮件列表
 */
export async function getPhishingEmails(accountId: string): Promise<Email[]> {
  const result = await invoke<any[]>('get_phishing_emails', { accountId })

  // 转换字段名
  return result.map(e => ({
    id: e.id,
    accountId: e.account_id,
    fromAddress: e.from_address,
    fromName: e.from_name,
    toAddresses: e.to_addresses,
    ccAddresses: e.cc_addresses,
    subject: e.subject,
    snippet: e.snippet,
    date: e.date,
    isRead: e.is_read,
    isStarred: e.is_starred,
    hasAttachment: e.has_attachment,
    isPhishing: e.is_phishing,
    phishingReasons: e.phishing_reasons
  }))
}

/**
 * 获取钓鱼邮件数量
 */
export async function getPhishingCount(accountId: string): Promise<number> {
  return invoke<number>('get_phishing_count', { accountId })
}

/**
 * 获取钓鱼邮件未读数量
 */
export async function getPhishingUnreadCount(accountId: string): Promise<number> {
  return invoke<number>('get_phishing_unread_count', { accountId })
}

/**
 * 获取星标邮件列表
 */
export async function getStarredEmails(accountId: string): Promise<Email[]> {
  const result = await invoke<any[]>('get_starred_emails', { accountId })

  // 转换字段名
  return result.map(e => ({
    id: e.id,
    accountId: e.account_id,
    fromAddress: e.from_address,
    fromName: e.from_name,
    toAddresses: e.to_addresses,
    ccAddresses: e.cc_addresses,
    subject: e.subject,
    snippet: e.snippet,
    date: e.date,
    isRead: e.is_read,
    isStarred: e.is_starred,
    hasAttachment: e.has_attachment,
    isPhishing: e.is_phishing,
    phishingReasons: e.phishing_reasons
  }))
}

/**
 * 获取星标邮件数量
 */
export async function getStarredCount(accountId: string): Promise<number> {
  return invoke<number>('get_starred_count', { accountId })
}

/**
 * 发送邮件结果
 */
export interface SendEmailResult {
  success: boolean
  message: string
}

/**
 * 附件数据（用于发送）
 */
export interface AttachmentData {
  filename: string
  content_type: string
  content: string  // Base64 编码内容
}

/**
 * 草稿附件数据
 */
export interface DraftAttachmentData {
  id: string
  filename: string
  content_type: string | null
  file_path: string | null
  /** Base64 编码的附件内容 */
  content: string | null
  file_size: number
}

/**
 * 发送邮件
 */
export async function sendEmail(
  accountId: string,
  toAddresses: string[],
  ccAddresses: string[] | null,
  subject: string,
  body: string,
  bodyHtml?: string | null,
  signatureHtml?: string | null,
  attachments?: AttachmentData[] | null
): Promise<SendEmailResult> {
  console.log('[emailService] sendEmail - 开始发送', {
    accountId,
    toAddresses,
    ccAddresses,
    subject,
    bodyLength: body.length,
    hasHtml: !!bodyHtml,
    hasSignature: !!signatureHtml,
    attachmentCount: attachments?.length || 0
  })

  try {
    const result = await invoke<SendEmailResult>('send_email', {
      accountId,
      toAddresses,
      ccAddresses,
      subject,
      body,
      bodyHtml: bodyHtml || null,
      signatureHtml: signatureHtml || null,
      attachments: attachments || null
    })

    console.log('[emailService] sendEmail - 发送结果:', result)
    return result
  } catch (error) {
    console.error('[emailService] sendEmail - 发送失败:', error)
    throw error
  }
}

// ==================== 附件懒加载相关函数 ====================

/**
 * 下载附件
 * 返回本地缓存路径
 */
export async function downloadAttachment(attachmentId: string): Promise<string> {
  console.log('[emailService] downloadAttachment - 开始下载:', attachmentId)

  try {
    const result = await invoke<string>('download_attachment', { attachmentId })
    console.log('[emailService] downloadAttachment - 下载完成:', result)
    return result
  } catch (error) {
    console.error('[emailService] downloadAttachment - 下载失败:', error)
    throw error
  }
}

/**
 * 保存附件到用户选择的位置
 */
export async function saveAttachmentToPath(attachmentId: string, targetPath: string): Promise<void> {
  return invoke('save_attachment_to_path', { attachmentId, targetPath })
}

/**
 * 获取附件信息（含下载状态）
 */
export async function getAttachmentInfo(attachmentId: string): Promise<AttachmentInfoWithStatus> {
  const result = await invoke<AttachmentInfoWithStatus>('get_attachment_info', { attachmentId })
  return result
}

/**
 * 获取邮件附件列表（含下载状态）
 */
export async function getAttachmentsWithStatus(emailId: string): Promise<AttachmentInfoWithStatus[]> {
  const result = await invoke<AttachmentInfoWithStatus[]>('get_attachments_with_status', { emailId })
  return result
}

/**
 * 获取缓存统计信息
 */
export async function getCacheStats(): Promise<CacheStats> {
  return invoke<CacheStats>('get_attachment_cache_stats')
}

/**
 * 清空附件缓存
 */
export async function clearCache(): Promise<void> {
  return invoke('clear_attachment_cache')
}

/**
 * 打开缓存目录
 */
export async function openCacheDirectory(): Promise<void> {
  return invoke('open_attachment_cache_directory')
}

// ==================== 已发送/草稿功能 ====================

/**
 * 获取已发送邮件列表
 */
export async function getSentEmails(accountId: string): Promise<Email[]> {
  const result = await invoke<any[]>('get_sent_emails', { accountId })

  // 转换字段名
  return result.map(e => ({
    id: e.id,
    accountId: e.account_id,
    fromAddress: e.from_address,
    fromName: e.from_name,
    toAddresses: e.to_addresses,
    ccAddresses: e.cc_addresses,
    subject: e.subject,
    snippet: e.snippet,
    date: e.date,
    isRead: e.is_read,
    isStarred: e.is_starred,
    hasAttachment: e.has_attachment,
    isPhishing: e.is_phishing,
    phishingReasons: e.phishing_reasons
  }))
}

/**
 * 获取已发送邮件数量
 */
export async function getSentCount(accountId: string): Promise<number> {
  return invoke<number>('get_sent_count', { accountId })
}

/**
 * 获取草稿邮件列表
 */
export async function getDraftEmails(accountId: string): Promise<Email[]> {
  const result = await invoke<any[]>('get_draft_emails', { accountId })

  // 转换字段名
  return result.map(e => ({
    id: e.id,
    accountId: e.account_id,
    fromAddress: e.from_address,
    fromName: e.from_name,
    toAddresses: e.to_addresses,
    ccAddresses: e.cc_addresses,
    subject: e.subject,
    snippet: e.snippet,
    date: e.date,
    isRead: e.is_read,
    isStarred: e.is_starred,
    hasAttachment: e.has_attachment,
    isPhishing: false,
    phishingReasons: undefined
  }))
}

/**
 * 获取草稿邮件数量
 */
export async function getDraftCount(accountId: string): Promise<number> {
  return invoke<number>('get_draft_count', { accountId })
}

/**
 * 草稿数据
 */
export interface DraftData {
  accountId: string
  toAddresses: string[]
  ccAddresses: string[]
  subject: string | null
  bodyText: string | null
  bodyHtml: string | null
  attachments?: DraftAttachmentData[] | null
}

/**
 * 创建草稿
 * 返回草稿 ID
 */
export async function createDraft(draft: DraftData): Promise<string> {
  return invoke<string>('create_draft', {
    draft: {
      account_id: draft.accountId,
      to_addresses: draft.toAddresses,
      cc_addresses: draft.ccAddresses,
      subject: draft.subject,
      body_text: draft.bodyText,
      body_html: draft.bodyHtml,
      attachments: draft.attachments?.map(a => ({
        id: a.id,
        filename: a.filename,
        content_type: a.content_type,
        file_path: a.file_path,
        content: a.content,
        file_size: a.file_size
      })) || null
    }
  })
}

/**
 * 更新草稿
 */
export async function updateDraft(draftId: string, draft: DraftData): Promise<void> {
  return invoke('update_draft', {
    draftId,
    draft: {
      account_id: draft.accountId,
      to_addresses: draft.toAddresses,
      cc_addresses: draft.ccAddresses,
      subject: draft.subject,
      body_text: draft.bodyText,
      body_html: draft.bodyHtml,
      attachments: draft.attachments?.map(a => ({
        id: a.id,
        filename: a.filename,
        content_type: a.content_type,
        file_path: a.file_path,
        content: a.content,
        file_size: a.file_size
      })) || null
    }
  })
}

/**
 * 删除草稿
 */
export async function deleteDraft(draftId: string): Promise<void> {
  return invoke('delete_draft', { draftId })
}

/**
 * 发送草稿
 */
export async function sendDraft(draftId: string, signatureHtml?: string | null, attachments?: AttachmentData[] | null): Promise<SendEmailResult> {
  console.log('[emailService] sendDraft - 开始发送草稿', {
    draftId,
    hasSignature: !!signatureHtml,
    attachmentCount: attachments?.length || 0
  })

  return invoke<SendEmailResult>('send_draft', {
    draftId,
    signatureHtml: signatureHtml || null,
    attachments: attachments || null
  })
}

/**
 * 获取草稿详情
 */
export async function getDraftDetail(draftId: string): Promise<EmailDetail | null> {
  const result = await invoke<EmailDetail | null>('get_draft_detail', { draftId })

  if (!result) return null

  return {
    id: result.id,
    account_id: result.account_id,
    from_address: result.from_address,
    from_name: result.from_name,
    to_addresses: result.to_addresses,
    cc_addresses: result.cc_addresses,
    subject: result.subject,
    body_text: result.body_text,
    body_html: result.body_html,
    date: result.date,
    is_read: result.is_read,
    is_starred: result.is_starred,
    has_attachment: result.has_attachment,
    attachments: result.attachments.map(a => ({
      id: a.id,
      filename: a.filename,
      mime_type: a.mime_type,
      size: a.size
    })),
    is_phishing: false,
    phishing_reasons: null
  }
}

/**
 * 获取草稿附件列表
 */
export async function getDraftAttachments(draftId: string): Promise<DraftAttachmentData[]> {
  const result = await invoke<any[]>('get_draft_attachments', { draftId })
  console.log('[emailService] getDraftAttachments - result:', result)
  return result.map(a => ({
    id: a.id,
    filename: a.filename,
    content_type: a.content_type,
    file_path: a.file_path,
    content: a.content,
    file_size: a.file_size
  }))
}

/**
 * 读取文件内容用于附件（返回 Base64 编码内容）
 */
export async function readFileForAttachment(filePath: string): Promise<AttachmentData> {
  return invoke<AttachmentData>('read_file_for_attachment', { filePath })
}

/**
 * 验证附件文件路径是否存在
 */
export async function validateAttachmentPath(filePath: string): Promise<boolean> {
  return invoke<boolean>('validate_attachment_path', { filePath })
}

// ==================== 邮件会话功能 ====================

/**
 * 会话信息
 */
export interface Thread {
  id: string
  account_id: string
  subject_base: string | null
  last_message_at: number
  message_count: number
  has_unread: boolean
}

/**
 * 会话邮件（含 threading headers）
 */
export interface ThreadEmail {
  id: string
  account_id: string
  from_address: string | null
  from_name: string | null
  to_addresses: string[]
  cc_addresses: string[]
  subject: string | null
  snippet: string
  date: number
  is_read: boolean
  is_starred: boolean
  has_attachment: boolean
  is_phishing: boolean
  phishing_reasons: string[] | null
  message_id_header: string | null
  in_reply_to_header: string | null
  references_header: string | null
  thread_id: string | null
}

/**
 * 线程分配
 */
export interface ThreadAssignment {
  email_id: string
  thread_id: string
}

/**
 * 获取账户的所有会话
 */
export async function getThreads(accountId: string): Promise<Thread[]> {
  const result = await invoke<Thread[]>('get_threads', { accountId })
  return result.map(t => ({
    id: t.id,
    account_id: t.account_id,
    subject_base: t.subject_base,
    last_message_at: t.last_message_at,
    message_count: t.message_count,
    has_unread: t.has_unread
  }))
}

/**
 * 获取会话内的所有邮件
 */
export async function getThreadEmails(threadId: string): Promise<ThreadEmail[]> {
  const result = await invoke<any[]>('get_thread_emails', { threadId })
  return result.map(e => ({
    id: e.id,
    account_id: e.account_id,
    from_address: e.from_address,
    from_name: e.from_name,
    to_addresses: e.to_addresses,
    cc_addresses: e.cc_addresses,
    subject: e.subject,
    snippet: e.snippet,
    date: e.date,
    is_read: e.is_read,
    is_starred: e.is_starred,
    has_attachment: e.has_attachment,
    is_phishing: e.is_phishing,
    phishing_reasons: e.phishing_reasons,
    message_id_header: e.message_id_header,
    in_reply_to_header: e.in_reply_to_header,
    references_header: e.references_header,
    thread_id: e.thread_id
  }))
}

/**
 * 分配线程 ID 到邮件
 */
export async function assignThreads(assignments: ThreadAssignment[]): Promise<void> {
  return invoke('assign_threads', {
    assignments: assignments.map(a => ({
      email_id: a.email_id,
      thread_id: a.thread_id
    }))
  })
}

/**
 * 获取带 threading headers 的邮件列表（用于计算会话）
 */
export async function getEmailsForThreading(accountId: string): Promise<ThreadEmail[]> {
  const result = await invoke<any[]>('get_emails_for_threading', { accountId })
  return result.map(e => ({
    id: e.id,
    account_id: e.account_id,
    from_address: e.from_address,
    from_name: e.from_name,
    to_addresses: e.to_addresses,
    cc_addresses: e.cc_addresses,
    subject: e.subject,
    snippet: e.snippet,
    date: e.date,
    is_read: e.is_read,
    is_starred: e.is_starred,
    has_attachment: e.has_attachment,
    is_phishing: e.is_phishing,
    phishing_reasons: e.phishing_reasons,
    message_id_header: e.message_id_header,
    in_reply_to_header: e.in_reply_to_header,
    references_header: e.references_header,
    thread_id: e.thread_id
  }))
}