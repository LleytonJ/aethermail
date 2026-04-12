import type { Account } from '@/stores/account'

/**
 * 模拟账户数据 - 用于前端原型开发
 */
export const MOCK_ACCOUNTS: Account[] = [
  {
    id: 'account-1',
    name: 'Primary Account',
    email: 'user@example.com',
    isActive: true,
    provider: 'pop3',
    incomingServer: 'pop.example.com',
    outgoingServer: 'smtp.example.com',
    incomingPort: 995,
    outgoingPort: 587,
    useSsl: true,
    phishingDetectionEnabled: false
  },
  {
    id: 'account-2',
    name: 'Gmail Account',
    email: 'user@gmail.com',
    isActive: false,
    provider: 'pop3',
    incomingServer: 'pop.gmail.com',
    outgoingServer: 'smtp.gmail.com',
    incomingPort: 995,
    outgoingPort: 465,
    useSsl: true,
    phishingDetectionEnabled: false
  }
]

export async function getMockAccounts(): Promise<Account[]> {
  await new Promise(resolve => setTimeout(resolve, 100))
  return MOCK_ACCOUNTS
}

export async function getActiveAccount(): Promise<Account | undefined> {
  await new Promise(resolve => setTimeout(resolve, 50))
  return MOCK_ACCOUNTS.find(a => a.isActive)
}

export async function setActiveAccount(id: string): Promise<void> {
  await new Promise(resolve => setTimeout(resolve, 50))
  MOCK_ACCOUNTS.forEach(a => {
    a.isActive = a.id === id
  })
}
