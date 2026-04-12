import type { Email } from '@/stores/mailbox'
import { getMockEmails as getMock, getEmailById as getById } from './emailData'

/**
 * 邮件服务层 - 封装 Mock 数据访问
 * 后续可轻松替换为真实 API 调用
 */

export async function getEmails(accountId: string, _folder?: string): Promise<Email[]> {
  return getMock(accountId)
}

export async function getEmailById(id: string): Promise<Email | undefined> {
  return getById(id)
}

export async function sendEmail(data: Partial<Email>): Promise<void> {
  // TODO: 实现发送邮件
  console.log('Sending email:', data)
}

export async function markAsRead(id: string): Promise<void> {
  // TODO: 调用 API 标记已读
  console.log('Mark as read:', id)
}

export async function deleteEmail(id: string): Promise<void> {
  // TODO: 调用 API 删除邮件
  console.log('Delete email:', id)
}

// 导出 getMockEmails 以供直接使用
export { getMock as getMockEmails }
