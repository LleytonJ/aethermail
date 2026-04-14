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
  is_active: boolean
  phishing_detection_enabled: boolean
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
  phishingDetectionEnabled: boolean
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
 * 测试 POP3 连接
 */
export async function testConnection(config: Pop3Config): Promise<ConnectionTestResult> {
  try {
    // Tauri 参数命名规则: 前端用 camelCase，后端用 snake_case
    // Tauri 会自动转换 useSsl -> use_ssl
    const result = await invoke<ConnectionTestResult>('test_pop3_connection', {
      host: config.host,
      port: config.port,
      useSsl: config.use_ssl,       // camelCase: useSsl -> use_ssl
      email: config.username,       // 后端参数名是 email
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
  useSsl: boolean
): Promise<Account> {
  try {
    // Tauri 参数命名规则: 前端用 camelCase，后端用 snake_case
    // 后端命令参数: email, password, display_name, host, port, use_ssl
    const result = await invoke<AccountConfig>('add_pop3_account', {
      email,
      password,
      displayName,           // displayName -> display_name
      host: pop3Host,        // 后端参数名是 host，不是 pop3_host
      port: pop3Port,        // 后端参数名是 port，不是 pop3_port
      useSsl                 // useSsl -> use_ssl
    })

    // 转换为前端 Account 格式
    return {
      id: result.id,
      name: result.display_name || '',
      email: result.email,
      isActive: result.is_active,
      protocol: result.protocol,
      provider: 'pop3',
      incomingServer: result.pop3_host,
      outgoingServer: '',
      incomingPort: result.pop3_port,
      outgoingPort: 0,
      useSsl: result.use_ssl,
      phishingDetectionEnabled: result.phishing_detection_enabled
    }
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
  imapUseSsl: boolean
): Promise<Account> {
  try {
    const result = await invoke<AccountConfig>('add_imap_account', {
      email,
      password,
      displayName,
      imapHost,
      imapPort,
      imapUseSsl
    })

    // 转换为前端 Account 格式
    return {
      id: result.id,
      name: result.display_name || '',
      email: result.email,
      isActive: result.is_active,
      protocol: result.protocol,
      provider: 'imap',
      incomingServer: result.imap_host,
      outgoingServer: '',
      incomingPort: result.imap_port,
      outgoingPort: 0,
      useSsl: result.imap_use_ssl,
      phishingDetectionEnabled: result.phishing_detection_enabled
    }
  } catch (error) {
    throw error
  }
}

/**
 * 根据协议类型添加账户（通用接口）
 */
export async function addAccountByProtocol(
  protocol: ProtocolType,
  email: string,
  displayName: string | null,
  password: string,
  host: string,
  port: number,
  useSsl: boolean
): Promise<Account> {
  if (protocol === 'imap') {
    return addImapAccount(email, displayName, password, host, port, useSsl)
  } else {
    return addAccount(email, displayName, password, host, port, useSsl)
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
    outgoingServer: '',
    incomingPort: isIncomingImap ? config.imap_port : config.pop3_port,
    outgoingPort: 0,
    useSsl: isIncomingImap ? config.imap_use_ssl : config.use_ssl,
    phishingDetectionEnabled: config.phishing_detection_enabled
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
 * 更新账户
 */
export async function updateAccount(
  id: string,
  displayName: string,
  password: string,
  pop3Host: string,
  pop3Port: number,
  useSsl: boolean
): Promise<Account> {
  const result = await invoke<AccountConfig>('update_account', {
    id,
    displayName,
    password,
    host: pop3Host,
    port: pop3Port,
    useSsl
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