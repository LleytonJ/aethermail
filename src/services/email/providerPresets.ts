/**
 * 邮箱服务商 POP3 预设配置
 * 根据邮箱域名自动填充服务器配置
 */

export type SecurityType = 'ssl' | 'none'

export interface Pop3Preset {
  host: string
  port: number
  security: SecurityType
}

export interface SmtpPreset {
  host: string
  port: number
  security: SecurityType
}

export interface EmailProviderPreset {
  name: string
  pop3: Pop3Preset
  smtp?: SmtpPreset
}

/**
 * 常见邮箱服务商预设配置
 */
export const PROVIDER_PRESETS: Record<string, EmailProviderPreset> = {
  'gmail.com': {
    name: 'Gmail',
    pop3: { host: 'pop.gmail.com', port: 995, security: 'ssl' },
    smtp: { host: 'smtp.gmail.com', port: 465, security: 'ssl' }
  },
  '163.com': {
    name: '163 Mail',
    pop3: { host: 'pop.163.com', port: 110, security: 'none' },
    smtp: { host: 'smtp.163.com', port: 25, security: 'none' }
  },
  '126.com': {
    name: '126 Mail',
    pop3: { host: 'pop.126.com', port: 110, security: 'none' },
    smtp: { host: 'smtp.126.com', port: 25, security: 'none' }
  },
  'qq.com': {
    name: 'QQ Mail',
    pop3: { host: 'pop.qq.com', port: 110, security: 'none' },
    smtp: { host: 'smtp.qq.com', port: 465, security: 'ssl' }
  },
  'outlook.com': {
    name: 'Outlook',
    pop3: { host: 'pop-mail.outlook.com', port: 995, security: 'ssl' },
    smtp: { host: 'smtp-mail.outlook.com', port: 587, security: 'ssl' }
  },
  'hotmail.com': {
    name: 'Hotmail',
    pop3: { host: 'pop-mail.outlook.com', port: 995, security: 'ssl' },
    smtp: { host: 'smtp-mail.outlook.com', port: 587, security: 'ssl' }
  },
  'live.com': {
    name: 'Live',
    pop3: { host: 'pop-mail.outlook.com', port: 995, security: 'ssl' },
    smtp: { host: 'smtp-mail.outlook.com', port: 587, security: 'ssl' }
  }
}

/**
 * 从邮箱地址提取域名
 */
export function extractDomain(email: string): string | null {
  const parts = email.split('@')
  return parts.length === 2 ? parts[1].toLowerCase() : null
}

/**
 * 根据邮箱地址查找预设配置
 */
export function getProviderPreset(email: string): EmailProviderPreset | null {
  const domain = extractDomain(email)
  if (!domain) return null
  return PROVIDER_PRESETS[domain] || null
}

/**
 * 获取默认 POP3 端口
 */
export function getDefaultPop3Port(security: SecurityType): number {
  return security === 'ssl' ? 995 : 110
}

/**
 * 获取默认 SMTP 端口
 */
export function getDefaultSmtpPort(security: SecurityType): number {
  return security === 'ssl' ? 465 : 25
}