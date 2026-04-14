import type { Account } from '@/stores/account'
import type { Pop3Preset, SecurityType } from '@/services/email/providerPresets'

/**
 * POP3 配置接口
 */
export interface Pop3Config {
  host: string
  port: number
  security: SecurityType
  username: string
  password: string
}

/**
 * 测试连接结果
 */
export interface Pop3TestResult {
  success: boolean
  message: string
  serverInfo?: string
}

/**
 * 模拟账户数据存储
 */
let mockAccounts: Account[] = []
let activeAccountId: string | null = null

/**
 * 初始化 - 从 localStorage 加载账户
 */
export function initMockAccounts(): void {
  try {
    const saved = localStorage.getItem('aethermail_accounts')
    if (saved) {
      mockAccounts = JSON.parse(saved)
    }
    const savedActiveId = localStorage.getItem('aethermail_active_account_id')
    if (savedActiveId) {
      activeAccountId = savedActiveId
    }
  } catch (e) {
    console.error('Failed to load accounts from localStorage:', e)
    mockAccounts = []
  }
}

/**
 * 保存账户到 localStorage
 */
function persistAccounts(): void {
  try {
    localStorage.setItem('aethermail_accounts', JSON.stringify(mockAccounts))
    if (activeAccountId) {
      localStorage.setItem('aethermail_active_account_id', activeAccountId)
    }
  } catch (e) {
    console.error('Failed to save accounts to localStorage:', e)
  }
}

/**
 * 获取所有账户
 */
export async function getMockAccounts(): Promise<Account[]> {
  await new Promise(resolve => setTimeout(resolve, 50))
  return [...mockAccounts]
}

/**
 * 获取当前激活账户
 */
export async function getActiveAccount(): Promise<Account | undefined> {
  await new Promise(resolve => setTimeout(resolve, 50))
  return mockAccounts.find(a => a.id === activeAccountId) || mockAccounts[0]
}

/**
 * 设置激活账户
 */
export async function setActiveAccount(id: string): Promise<void> {
  await new Promise(resolve => setTimeout(resolve, 50))
  activeAccountId = id
  mockAccounts.forEach(a => {
    a.isActive = a.id === id
  })
  persistAccounts()
}

/**
 * 添加新账户
 */
export async function addMockAccount(
  email: string,
  displayName: string | null,
  _password: string, // Would be encrypted in production
  pop3Config: Pop3Preset
): Promise<Account> {
  await new Promise(resolve => setTimeout(resolve, 100))

  const accountId = `account-${Date.now()}`

  const newAccount: Account = {
    id: accountId,
    name: displayName || email.split('@')[0] || 'User',
    email,
    isActive: true,
    protocol: 'pop3',
    provider: 'pop3',
    incomingServer: pop3Config.host,
    outgoingServer: '',
    incomingPort: pop3Config.port,
    outgoingPort: 0,
    useSsl: pop3Config.security === 'ssl',
    phishingDetectionEnabled: false
  }

  // 将其他账户设为非激活
  mockAccounts.forEach(a => {
    a.isActive = false
  })

  mockAccounts.push(newAccount)
  activeAccountId = accountId
  persistAccounts()

  return newAccount
}

/**
 * 更新账户
 */
export async function updateMockAccount(
  id: string,
  updates: Partial<Account>
): Promise<Account | undefined> {
  await new Promise(resolve => setTimeout(resolve, 50))

  const account = mockAccounts.find(a => a.id === id)
  if (!account) return undefined

  // Apply updates
  Object.assign(account, updates)

  persistAccounts()
  return account
}

/**
 * 删除账户
 */
export async function removeMockAccount(id: string): Promise<void> {
  await new Promise(resolve => setTimeout(resolve, 50))

  mockAccounts = mockAccounts.filter(a => a.id !== id)

  if (activeAccountId === id) {
    activeAccountId = mockAccounts[0]?.id || null
    if (mockAccounts[0]) {
      mockAccounts[0].isActive = true
    }
  }

  persistAccounts()
}

/**
 * Mock 测试 POP3 连接
 * 模拟 ~1.5s 延迟
 * 密码包含 "wrong" 时模拟失败
 */
export async function mockTestConnection(config: Pop3Config): Promise<Pop3TestResult> {
  // 模拟网络延迟
  await new Promise(resolve => setTimeout(resolve, 1500))

  // 模拟失败场景：密码包含 "wrong"
  if (config.password.toLowerCase().includes('wrong')) {
    return {
      success: false,
      message: '认证失败：用户名或密码错误'
    }
  }

  // 模拟失败场景：主机名无效
  if (!config.host || config.host.length < 3) {
    return {
      success: false,
      message: '连接失败：无效的服务器地址'
    }
  }

  // 默认成功
  return {
    success: true,
    message: '连接成功',
    serverInfo: `${config.host} 就绪 (模拟)`
  }
}

// 导出原有兼容接口
export { getMockAccounts as getAccounts }