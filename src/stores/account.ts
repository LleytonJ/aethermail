import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import {
  getAccounts,
  getActiveAccount,
  setActiveAccount as setServiceActiveAccount,
  addAccount as addServiceAccount,
  addImapAccount as addServiceImapAccount,
  updateAccount as updateServiceAccount,
  deleteAccount as deleteServiceAccount,
  testConnection,
  testImapConnection,
  testSmtpConnection,
  testConnectionByProtocol,
  type Pop3Config,
  type ImapConfig,
  type SmtpConfig,
  type ConnectionTestResult,
  type SmtpTestResult,
  type Account,
  type ProtocolType
} from '@/services/email/accountService'

export type { Account, ProtocolType, SmtpConfig, SmtpTestResult } from '@/services/email/accountService'

export const useAccountStore = defineStore('account', () => {
  // State
  const accounts = ref<Account[]>([])
  const activeAccountId = ref<string | null>(null)
  const initialized = ref(false)
  const loading = ref(false)
  const error = ref<string | null>(null)

  // Getters
  const activeAccount = computed(() => {
    return accounts.value.find(a => a.id === activeAccountId.value) || null
  })

  // Actions
  function setAccounts(newAccounts: Account[]) {
    accounts.value = newAccounts
    if (newAccounts.length > 0 && !activeAccountId.value) {
      activeAccountId.value = newAccounts.find(a => a.isActive)?.id || newAccounts[0].id
    }
  }

  async function setActiveAccount(id: string) {
    loading.value = true
    error.value = null
    try {
      await setServiceActiveAccount(id)
      activeAccountId.value = id
      accounts.value.forEach(a => {
        a.isActive = a.id === id
      })
    } catch (e) {
      error.value = e instanceof Error ? e.message : '设置激活账户失败'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function addAccount(
    email: string,
    displayName: string | null,
    password: string,
    pop3Host: string,
    pop3Port: number,
    useSsl: boolean,
    smtpHost: string,
    smtpPort: number,
    smtpUseSsl: boolean
  ) {
    console.log('[accountStore] addAccount - 开始')
    console.log('[accountStore] addAccount - 参数:', {
      email,
      displayName,
      password: '***',
      pop3Host,
      pop3Port,
      useSsl,
      smtpHost,
      smtpPort,
      smtpUseSsl
    })

    loading.value = true
    error.value = null
    try {
      console.log('[accountStore] addAccount - 调用 addServiceAccount...')
      const newAccount = await addServiceAccount(
        email,
        displayName,
        password,
        pop3Host,
        pop3Port,
        useSsl,
        smtpHost,
        smtpPort,
        smtpUseSsl
      )

      console.log('[accountStore] addAccount - 成功:', newAccount)

      // Deactivate other accounts
      accounts.value.forEach(a => {
        a.isActive = false
      })

      accounts.value.push({
        ...newAccount,
        isActive: true
      })
      activeAccountId.value = newAccount.id

      return newAccount
    } catch (e) {
      console.error('[accountStore] addAccount - 错误:', e)
      error.value = e instanceof Error ? e.message : '添加账户失败'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function addImapAccount(
    email: string,
    displayName: string | null,
    password: string,
    imapHost: string,
    imapPort: number,
    imapUseSsl: boolean,
    smtpHost: string,
    smtpPort: number,
    smtpUseSsl: boolean
  ) {
    console.log('[accountStore] addImapAccount - 开始')
    console.log('[accountStore] addImapAccount - 参数:', {
      email,
      displayName,
      password: '***',
      imapHost,
      imapPort,
      imapUseSsl,
      smtpHost,
      smtpPort,
      smtpUseSsl
    })

    loading.value = true
    error.value = null
    try {
      const newAccount = await addServiceImapAccount(
        email,
        displayName,
        password,
        imapHost,
        imapPort,
        imapUseSsl,
        smtpHost,
        smtpPort,
        smtpUseSsl
      )

      console.log('[accountStore] addImapAccount - 成功:', newAccount)

      // Deactivate other accounts
      accounts.value.forEach(a => {
        a.isActive = false
      })

      accounts.value.push({
        ...newAccount,
        isActive: true
      })
      activeAccountId.value = newAccount.id

      return newAccount
    } catch (e) {
      console.error('[accountStore] addImapAccount - 错误:', e)
      error.value = e instanceof Error ? e.message : '添加 IMAP 账户失败'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function removeAccount(id: string) {
    loading.value = true
    error.value = null
    try {
      await deleteServiceAccount(id)
      accounts.value = accounts.value.filter(a => a.id !== id)

      if (activeAccountId.value === id) {
        activeAccountId.value = accounts.value[0]?.id || null
        if (accounts.value[0]) {
          accounts.value[0].isActive = true
        }
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : '删除账户失败'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function updateAccount(
    id: string,
    displayName: string,
    password: string,
    incomingHost: string,
    incomingPort: number,
    useSsl: boolean,
    smtpHost: string,
    smtpPort: number,
    smtpUseSsl: boolean
  ) {
    loading.value = true
    error.value = null
    try {
      const updated = await updateServiceAccount(
        id,
        displayName,
        password,
        incomingHost,
        incomingPort,
        useSsl,
        smtpHost,
        smtpPort,
        smtpUseSsl
      )

      const account = accounts.value.find(a => a.id === id)
      if (account) {
        Object.assign(account, updated)
      }

      return updated
    } catch (e) {
      error.value = e instanceof Error ? e.message : '更新账户失败'
      throw e
    } finally {
      loading.value = false
    }
  }

  function getActiveAccountLocal() {
    return accounts.value.find(a => a.id === activeAccountId.value) || null
  }

  // 初始化 - 从后端加载账户
  async function init() {
    if (initialized.value) return

    loading.value = true
    error.value = null
    try {
      // Load accounts from backend
      const loadedAccounts = await getAccounts()
      accounts.value = loadedAccounts

      // Load active account
      const active = await getActiveAccount()
      if (active) {
        activeAccountId.value = active.id
      } else if (accounts.value.length > 0) {
        activeAccountId.value = accounts.value[0].id
      }

      initialized.value = true
    } catch (e) {
      error.value = e instanceof Error ? e.message : '加载账户失败'
      console.error('Failed to load accounts:', e)
    } finally {
      loading.value = false
    }
  }

  // 测试连接（代理到 service）
  async function testPop3Connection(config: Pop3Config): Promise<ConnectionTestResult> {
    return testConnection(config)
  }

  // 测试 IMAP 连接
  async function testImapConn(config: ImapConfig): Promise<ConnectionTestResult> {
    return testImapConnection(config)
  }

  // 测试 SMTP 连接
  async function testSmtpConn(config: SmtpConfig): Promise<SmtpTestResult> {
    return testSmtpConnection(config)
  }

  // 根据协议测试连接（通用接口）
  async function testConnByProtocol(
    protocol: ProtocolType,
    config: Pop3Config | ImapConfig
  ): Promise<ConnectionTestResult> {
    return testConnectionByProtocol(protocol, config)
  }

  return {
    accounts,
    activeAccountId,
    activeAccount,
    initialized,
    loading,
    error,
    setAccounts,
    setActiveAccount,
    addAccount,
    addImapAccount,
    removeAccount,
    updateAccount,
    getActiveAccountLocal,
    init,
    testPop3Connection,
    testImapConn,
    testSmtpConn,
    testConnByProtocol
  }
})