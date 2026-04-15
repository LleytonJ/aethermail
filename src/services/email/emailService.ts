/**
 * 邮件服务层 - 调用 Tauri 后端命令
 */

import { invoke } from '@tauri-apps/api/core'
import type { Email } from '@/stores/mailbox'

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
 * 发送邮件结果
 */
export interface SendEmailResult {
  success: boolean
  message: string
}

/**
 * 发送邮件
 */
export async function sendEmail(
  accountId: string,
  toAddresses: string[],
  ccAddresses: string[] | null,
  subject: string,
  body: string
): Promise<SendEmailResult> {
  console.log('[emailService] sendEmail - 开始发送', {
    accountId,
    toAddresses,
    ccAddresses,
    subject,
    bodyLength: body.length
  })

  try {
    const result = await invoke<SendEmailResult>('send_email', {
      accountId,
      toAddresses,
      ccAddresses,
      subject,
      body
    })

    console.log('[emailService] sendEmail - 发送结果:', result)
    return result
  } catch (error) {
    console.error('[emailService] sendEmail - 发送失败:', error)
    throw error
  }
}