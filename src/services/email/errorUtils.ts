/**
 * Error message utilities for user-friendly error display
 */

/**
 * Parse connection error into user-friendly message
 * @param error - The error object or message
 * @param protocol - The protocol type ('imap' or 'pop3')
 */
export function parseConnectionError(error: unknown, protocol: 'imap' | 'pop3' = 'pop3'): { title: string; message: string } {
  const errorMsg = error instanceof Error ? error.message : String(error)
  const protocolName = protocol === 'imap' ? 'IMAP' : 'POP3'

  // Connection timeout
  if (errorMsg.includes('连接超时') || errorMsg.includes('timeout')) {
    return {
      title: '连接超时',
      message: '服务器响应时间过长，请检查网络连接或服务器地址是否正确。'
    }
  }

  // TLS/SSL errors
  if (errorMsg.includes('TLS') || errorMsg.includes('SSL') || errorMsg.includes('握手')) {
    return {
      title: '安全连接失败',
      message: '无法建立安全连接，请确认服务器支持 SSL/TLS，或尝试切换安全设置。'
    }
  }

  // Connection refused
  if (errorMsg.includes('连接失败') || errorMsg.includes('refused') || errorMsg.includes('ECONNREFUSED')) {
    return {
      title: '无法连接服务器',
      message: `服务器地址或端口可能不正确，请检查 ${protocolName} 服务器设置。`
    }
  }

  // DNS/Host resolution
  if (errorMsg.includes('主机') || errorMsg.includes('DNS') || errorMsg.includes('resolve')) {
    return {
      title: '无法找到服务器',
      message: `服务器地址不存在，请确认 ${protocolName} 服务器地址是否正确。`
    }
  }

  // Server error response
  if (errorMsg.includes('服务器返回错误')) {
    return {
      title: '服务器拒绝连接',
      message: errorMsg.replace('服务器返回错误: ', '')
    }
  }

  // Generic fallback
  return {
    title: '连接失败',
    message: errorMsg || '未知错误'
  }
}

/**
 * Parse authentication error into user-friendly message
 * @param error - The error object or message
 * @param protocol - The protocol type ('imap' or 'pop3')
 */
export function parseAuthError(error: unknown, protocol: 'imap' | 'pop3' = 'pop3'): { title: string; message: string } {
  const errorMsg = error instanceof Error ? error.message : String(error)
  const protocolName = protocol === 'imap' ? 'IMAP' : 'POP3'

  // Authentication failed
  if (errorMsg.includes('认证失败') || errorMsg.includes('PASS') || errorMsg.includes('LOGIN')) {
    return {
      title: '认证失败',
      message: `用户名或密码错误，请确认已开启邮箱的 ${protocolName} 服务并使用正确的密码。`
    }
  }

  // USER command failed (POP3 specific)
  if (errorMsg.includes('USER')) {
    return {
      title: '用户名错误',
      message: '服务器不接受该用户名，请确认邮箱地址是否正确。'
    }
  }

  // Account locked/disabled
  if (errorMsg.includes('locked') || errorMsg.includes('disabled') || errorMsg.includes('禁用')) {
    return {
      title: '账户被锁定',
      message: `该邮箱账户可能被锁定或 ${protocolName} 服务未开启，请联系邮箱服务商。`
    }
  }

  // Generic fallback
  return {
    title: '登录失败',
    message: errorMsg || '请检查用户名和密码是否正确'
  }
}

/**
 * Parse sync error into user-friendly message
 * @param error - The error object or message
 * @param protocol - The protocol type ('imap' or 'pop3')
 */
export function parseSyncError(error: unknown, protocol: 'imap' | 'pop3' = 'pop3'): { title: string; message: string } {
  const errorMsg = error instanceof Error ? error.message : String(error)

  // Connection errors during sync
  if (errorMsg.includes('连接') || errorMsg.includes('timeout')) {
    return parseConnectionError(error, protocol)
  }

  // Authentication errors during sync
  if (errorMsg.includes('认证') || errorMsg.includes('USER') || errorMsg.includes('PASS') || errorMsg.includes('LOGIN')) {
    return parseAuthError(error, protocol)
  }

  // Database errors
  if (errorMsg.includes('数据库') || errorMsg.includes('保存')) {
    return {
      title: '保存失败',
      message: '邮件保存到本地失败，请稍后重试。'
    }
  }

  // Generic fallback
  return {
    title: '同步失败',
    message: errorMsg || '未知错误'
  }
}

/**
 * Check if error is a network/connection error
 */
export function isNetworkError(error: unknown): boolean {
  const errorMsg = error instanceof Error ? error.message : String(error)
  return (
    errorMsg.includes('连接') ||
    errorMsg.includes('timeout') ||
    errorMsg.includes('网络') ||
    errorMsg.includes('ECONNREFUSED') ||
    errorMsg.includes('ENOTFOUND')
  )
}