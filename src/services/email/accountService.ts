/**
 * 账户服务层 - 调用 Tauri 后端命令
 */

import { invoke } from '@tauri-apps/api/core'

/**
 * POP3 配置接口
 */
export interface Pop3Config {
  host: string
  port: number
  use_ssl: boolean
  username: string
  password: string
}

/**
 * IMAP 配置接口
 */
export interface ImapConfig {
  host: string
  port: number
  use_ssl: boolean
  username: string
  password: string
}

/**
 * SMTP 配置接口
 */
export interface SmtpConfig {
  host: string
  port: number
  use_ssl: boolean
  username: string
  password: string
}

/**
 * 协议类型
 */
export type ProtocolType = 'pop3' | 'imap'

/**
 * 账户配置接口（后端返回结构）
 */
export interface AccountConfig {
  id: string
  email: string
  display_name: string | null
  password: string
  protocol: ProtocolType
  pop3_host: string
  pop3_port: number
  use_ssl: boolean
  imap_host: string
  imap_port: number
  imap_use_ssl: boolean
  smtp_host: string
  smtp_port: number
  smtp_use_ssl: boolean
  is_active: boolean
  phishing_detection_enabled: boolean
  signature_html: string
}

/**
 * 账户接口（前端使用）
 */
export interface Account {
  id: string
  name: string
  email: string
  isActive: boolean
  protocol: ProtocolType
  provider: string
  incomingServer: string
  outgoingServer: string
  incomingPort: number
  outgoingPort: number
  useSsl: boolean
  smtpHost: string
  smtpPort: number
  smtpUseSsl: boolean
  phishingDetectionEnabled: boolean
  signatureHtml: string
}

/**
 * 连接测试结果
 */
export interface ConnectionTestResult {
  success: boolean
  message: string
  server_greeting?: string
}

/**
 * SMTP 测试结果
 */
export interface SmtpTestResult {
  success: boolean
  message: string
}

/**
 * 测试 POP3 连接
 */
export async function testConnection(config: Pop3Config): Promise<ConnectionTestResult> {
  try {
    const result = await invoke<ConnectionTestResult>('test_pop3_connection', {
      host: config.host,
      port: config.port,
      useSsl: config.use_ssl,
      email: config.username,
      password: config.password
    })

    return result
  } catch (error) {
    throw error
  }
}

/**
 * 测试 IMAP 连接
 */
export async function testImapConnection(config: ImapConfig): Promise<ConnectionTestResult> {
  try {
    const result = await invoke<ConnectionTestResult>('test_imap_connection', {
      host: config.host,
      port: config.port,
      useSsl: config.use_ssl,
      email: config.username,
      password: config.password
    })

    return result
  } catch (error) {
    throw error
  }
}

/**
 * 测试 SMTP 连接
 */
export async function testSmtpConnection(config: SmtpConfig): Promise<SmtpTestResult> {
  try {
    const result = await invoke<SmtpTestResult>('test_smtp_connection', {
      host: config.host,
      port: config.port,
      useSsl: config.use_ssl,
      email: config.username,
      password: config.password
    })

    return result
  } catch (error) {
    throw error
  }
}

/**
 * 根据 protocol 测试连接（通用接口）
 */
export async function testConnectionByProtocol(
  protocol: ProtocolType,
  config: Pop3Config | ImapConfig
): Promise<ConnectionTestResult> {
  if (protocol === 'imap') {
    return testImapConnection(config as ImapConfig)
  } else {
    return testConnection(config as Pop3Config)
  }
}

/**
 * 添加 POP3 账户
 */
export async function addAccount(
  email: string,
  displayName: string | null,
  password: string,
  pop3Host: string,
  pop3Port: number,
  useSsl: boolean,
  smtpHost: string,
  smtpPort: number,
  smtpUseSsl: boolean
): Promise<Account> {
  try {
    const result = await invoke<AccountConfig>('add_pop3_account', {
      email,
      password,
      displayName,
      host: pop3Host,
      port: pop3Port,
      useSsl,
      smtpHost,
      smtpPort,
      smtpUseSsl
    })

    return convertToAccount(result)
  } catch (error) {
    throw error
  }
}

/**
 * 添加 IMAP 账户
 */
export async function addImapAccount(
  email: string,
  displayName: string | null,
  password: string,
  imapHost: string,
  imapPort: number,
  imapUseSsl: boolean,
  smtpHost: string,
  smtpPort: number,
  smtpUseSsl: boolean
): Promise<Account> {
  try {
    const result = await invoke<AccountConfig>('add_imap_account', {
      email,
      password,
      displayName,
      imapHost,
      imapPort,
      imapUseSsl,
      smtpHost,
      smtpPort,
      smtpUseSsl
    })

    return convertToAccount(result)
  } catch (error) {
    throw error
  }
}

/**
 * 转换 AccountConfig 为 Account
 */
function convertToAccount(config: AccountConfig): Account {
  const isIncomingImap = config.protocol === 'imap'
  return {
    id: config.id,
    name: config.display_name || '',
    email: config.email,
    isActive: config.is_active,
    protocol: config.protocol,
    provider: config.protocol,
    incomingServer: isIncomingImap ? config.imap_host : config.pop3_host,
    outgoingServer: config.smtp_host || '',
    incomingPort: isIncomingImap ? config.imap_port : config.pop3_port,
    outgoingPort: config.smtp_port || 465,
    useSsl: isIncomingImap ? config.imap_use_ssl : config.use_ssl,
    smtpHost: config.smtp_host || '',
    smtpPort: config.smtp_port || 465,
    smtpUseSsl: config.smtp_use_ssl ?? true,
    phishingDetectionEnabled: config.phishing_detection_enabled,
    signatureHtml: config.signature_html || ''
  }
}

/**
 * 获取所有账户
 */
export async function getAccounts(): Promise<Account[]> {
  const result = await invoke<AccountConfig[]>('get_accounts')
  return result.map(convertToAccount)
}

/**
 * 获取当前激活账户
 */
export async function getActiveAccount(): Promise<Account | null> {
  const result = await invoke<AccountConfig | null>('get_active_account')
  return result ? convertToAccount(result) : null
}

/**
 * 设置激活账户
 */
export async function setActiveAccount(id: string): Promise<void> {
  return invoke('set_active_account', { id })
}

/**
 * 更新账户（包含 SMTP 配置）
 */
export async function updateAccount(
  id: string,
  displayName: string,
  password: string,
  host: string,
  port: number,
  useSsl: boolean,
  smtpHost: string,
  smtpPort: number,
  smtpUseSsl: boolean
): Promise<Account> {
  const result = await invoke<AccountConfig>('update_account', {
    id,
    displayName,
    password,
    host,
    port,
    useSsl,
    smtpHost,
    smtpPort,
    smtpUseSsl
  })
  return convertToAccount(result)
}

/**
 * 删除账户
 */
export async function deleteAccount(id: string): Promise<void> {
  return invoke('delete_account', { id })
}

/**
 * 更新账户钓鱼检测开关
 */
export async function updatePhishingDetection(id: string, enabled: boolean): Promise<Account> {
  const result = await invoke<AccountConfig>('update_account_phishing_detection', {
    id,
    phishingDetectionEnabled: enabled
  })
  return convertToAccount(result)
}

/**
 * 更新账户签名 HTML
 */
export async function updateSignature(id: string, signatureHtml: string): Promise<Account> {
  const result = await invoke<AccountConfig>('update_account_signature', {
    id,
    signatureHtml
  })
  return convertToAccount(result)
}

/**
 * 获取账户签名
 */
export async function getSignature(id: string): Promise<string> {
  const accounts = await getAccounts()
  const account = accounts.find(a => a.id === id)
  return account?.signatureHtml || ''
}