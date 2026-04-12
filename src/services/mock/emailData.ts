import type { Email } from '@/stores/mailbox'

/**
 * 模拟邮件数据 - 用于前端原型开发
 */
export const MOCK_EMAILS: Email[] = [
  {
    id: '1',
    accountId: 'account-1',
    fromAddress: 'alice.chen@gmail.com',
    fromName: 'Alice Chen',
    toAddresses: ['user@example.com'],
    ccAddresses: [],
    subject: 'Meet intentionant message',
    snippet: 'Hello your ships news the soed mation it...',
    date: Date.now() - 1000 * 60 * 30, // 30 分钟前
    isRead: false,
    isStarred: true,
    hasAttachment: false,
    bodyHtml: `
      <div>
        <p>Hello,</p>
        <p>Hello, there!</p>
        <p>That is at the full of for atramct perfomanced...</p>
        <ul>
          <li>Boost fitting of hesums and corsient</li>
          <li>Bullet plants, and ios inferestcommunication</li>
          <li>Urgent mettord and rnat content</li>
        </ul>
        <p>Best regards,<br>Alice</p>
      </div>
    `,
    bodyText: 'Hello, Hello there! That is at the full of for atramct perfomanced...'
  },
  {
    id: '2',
    accountId: 'account-1',
    fromAddress: 'maya.soarn@example.com',
    fromName: 'Maya Soarn',
    toAddresses: ['user@example.com'],
    ccAddresses: [],
    subject: 'Collaboration remation approved',
    snippet: 'What are Alice Chen fit about your need??...',
    date: Date.now() - 1000 * 60 * 60, // 1 小时前
    isRead: false,
    isStarred: true,
    hasAttachment: true,
    bodyHtml: '<p>What are Alice Chen fit about your need??</p>',
    bodyText: 'What are Alice Chen fit about your need??'
  },
  {
    id: '3',
    accountId: 'account-1',
    fromAddress: 'alice.chen@gmail.com',
    fromName: 'Alice Chen',
    toAddresses: ['user@example.com'],
    ccAddresses: [],
    subject: 'Meet infintionkant message',
    snippet: 'Hello, your ship is the mody croerws ne...',
    date: Date.now() - 1000 * 60 * 60 * 3, // 3 小时前
    isRead: true,
    isStarred: false,
    hasAttachment: false,
    bodyHtml: '<p>Hello, your ship is the mody croerws ne...</p>',
    bodyText: 'Hello, your ship is the mody croerws ne...'
  },
  {
    id: '4',
    accountId: 'account-1',
    fromAddress: 'maya.soarn@example.com',
    fromName: 'Maya Soarn',
    toAddresses: ['user@example.com'],
    ccAddresses: [],
    subject: 'Meeting new print message',
    snippet: 'We hond a nicerty topiss your fintt contin...',
    date: Date.now() - 1000 * 60 * 60 * 5, // 5 小时前
    isRead: true,
    isStarred: false,
    hasAttachment: true,
    bodyHtml: '<p>We hond a nicerty topiss your fintt contin...</p>',
    bodyText: 'We hond a nicerty topiss your fintt contin...'
  },
  {
    id: '5',
    accountId: 'account-1',
    fromAddress: 'alice.chen@gmail.com',
    fromName: 'Alice Chen',
    toAddresses: ['user@example.com'],
    ccAddresses: [],
    subject: 'Project simetane sustiors',
    snippet: 'You unsew note for sntents information...',
    date: Date.now() - 1000 * 60 * 60 * 6, // 6 小时前
    isRead: false,
    isStarred: true,
    hasAttachment: false,
    bodyHtml: '<p>You unsew note for sntents information...</p>',
    bodyText: 'You unsew note for sntents information...'
  },
  {
    id: '6',
    accountId: 'account-1',
    fromAddress: 'alice.chen@gmail.com',
    fromName: 'Alice Chen',
    toAddresses: ['user@example.com'],
    ccAddresses: [],
    subject: 'Planning ntotew',
    snippet: 'The only are the first-most of hatting reori...',
    date: Date.now() - 1000 * 60 * 60 * 8, // 8 小时前
    isRead: true,
    isStarred: false,
    hasAttachment: true,
    bodyHtml: '<p>The only are the first-most of hatting reori...</p>',
    bodyText: 'The only are the first-most of hatting reori...'
  }
]

export async function getMockEmails(accountId: string): Promise<Email[]> {
  // 模拟网络延迟
  await new Promise(resolve => setTimeout(resolve, 300))
  return MOCK_EMAILS.filter(e => e.accountId === accountId)
}

export async function getEmailById(id: string): Promise<Email | undefined> {
  await new Promise(resolve => setTimeout(resolve, 100))
  return MOCK_EMAILS.find(e => e.id === id)
}