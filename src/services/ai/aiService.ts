/**
 * AI 服务 - OpenAI SDK 初始化和调用接口
 */

import OpenAI from 'openai'
import type { AiConfig } from './aiConfig'

/**
 * Mock 模式开关 - 用于测试钓鱼邮件判定流程
 * 设置为 true 时，所有 AI 调用将返回 Mock 数据而不调用真实 API
 */
const MOCK_MODE = true

/**
 * Mock 钓鱼邮件响应 - 模拟 AI 判定邮件为钓鱼邮件的结果
 */
const MOCK_PHISHING_RESPONSE = JSON.stringify({
  is_phishing: true,
  reasons: [
    '发件人地址 security@fake-paypal.com 仿冒 PayPal 官方域名',
    '邮件内容要求点击链接进行"账户验证"，链接域名 paypa1-verify.com 与官方域名相似但有细微差异（数字 1 替换字母 l）',
    '使用紧迫性语言"24小时内未验证将冻结账户"，制造心理压力',
    '要求用户提供敏感个人信息（登录凭据）'
  ]
})

/**
 * AI 服务类
 */
export class AIService {
  private client: OpenAI | null = null
  private config: AiConfig | null = null

  /**
   * 初始化 AI 客户端
   */
  async init(config: AiConfig): Promise<void> {
    console.log('[AIService] 初始化 AI 客户端')
    console.log('[AIService] 配置信息:', {
      provider: config.provider,
      baseUrl: config.base_url,
      model: config.model,
      enabled: config.enabled,
      hasApiKey: !!config.api_key,
      apiKeyLength: config.api_key?.length || 0
    })

    if (!config.enabled || !config.api_key || !config.base_url) {
      console.warn('[AIService] 配置不完整，无法初始化:', {
        enabled: config.enabled,
        hasApiKey: !!config.api_key,
        hasBaseUrl: !!config.base_url
      })
      this.client = null
      this.config = null
      return
    }

    this.config = config
    this.client = new OpenAI({
      apiKey: config.api_key,
      baseURL: config.base_url,
      dangerouslyAllowBrowser: true  // 允许在浏览器环境使用 (Tauri WebView)
    })

    console.log('[AIService] AI 客户端初始化成功')
  }

  /**
   * 检查 AI 服务是否可用
   */
  isAvailable(): boolean {
    const available = this.client !== null && this.config?.enabled === true
    console.log('[AIService] 服务可用性检查:', available)
    return available
  }

  /**
   * 调用 AI 完成对话
   * @param systemPrompt System Prompt
   * @param userContent 用户消息内容
   * @param timeout 超时时间 (毫秒)
   * @returns AI 响应文本
   */
  async complete(
    systemPrompt: string,
    userContent: string,
    timeout: number = 10000
  ): Promise<string> {
    console.log('[AIService] ========== 开始 AI API 调用 ==========')

    // Mock 模式：返回模拟的钓鱼邮件检测结果
    if (MOCK_MODE) {
      console.log('[AIService] ⚠️ Mock 模式已启用，跳过真实 API 调用')
      console.log('[AIService] Mock 响应:', MOCK_PHISHING_RESPONSE)
      console.log('[AIService] ========== Mock 模式返回完成 ==========')
      return MOCK_PHISHING_RESPONSE
    }

    if (!this.client || !this.config) {
      console.error('[AIService] 服务未初始化')
      throw new AIServiceError('AI 服务未初始化或未启用', 'NOT_INITIALIZED')
    }

    console.log('[AIService] 调用参数:', {
      model: this.config.model,
      baseUrl: this.config.base_url,
      systemPromptLength: systemPrompt.length,
      userContentLength: userContent.length,
      timeout
    })

    try {
      // 创建超时 Promise
      console.log('[AIService] 创建超时控制器，超时时间:', timeout, 'ms')
      let timeoutId: ReturnType<typeof setTimeout> | null = null
      const timeoutPromise = new Promise<never>((_, reject) => {
        timeoutId = setTimeout(() => {
          console.error('[AIService] API 调用超时!')
          reject(new AIServiceError(`AI 调用超时 (${timeout}ms)`, 'TIMEOUT'))
        }, timeout)
      })

      // 创建 API 调用请求
      console.log('[AIService] 构建 API 请求...')
      console.log('[AIService] 请求参数:', JSON.stringify({
        model: this.config.model,
        messages: [
          { role: 'system', content_length: systemPrompt.length },
          { role: 'user', content_length: userContent.length }
        ],
        temperature: 0.1,
        max_tokens: 500
      }, null, 2))

      const apiPromise = this.client.chat.completions.create({
        model: this.config.model,
        messages: [
          { role: 'system', content: systemPrompt },
          { role: 'user', content: userContent }
        ],
        temperature: 0.1,  // 低温度，更确定性的输出
        max_tokens: 500     // 限制输出长度
      })

      // 等待结果或超时
      console.log('[AIService] 发送 API 请求，等待响应...')
      const startTime = Date.now()

      const response = await Promise.race([apiPromise, timeoutPromise])

      // 清除超时定时器（API 成功返回）
      if (timeoutId) {
        clearTimeout(timeoutId)
      }

      const elapsed = Date.now() - startTime
      console.log('[AIService] API 响应耗时:', elapsed, 'ms')

      console.log('[AIService] API 响应结构:', {
        id: response.id,
        model: response.model,
        choicesCount: response.choices?.length,
        usage: response.usage
      })

      const content = response.choices[0]?.message?.content
      console.log('[AIService] 响应内容长度:', content?.length || 0)
      console.log('[AIService] 响应内容:', content)

      if (!content) {
        console.error('[AIService] 响应内容为空')
        throw new AIServiceError('AI 响应内容为空', 'EMPTY_RESPONSE')
      }

      console.log('[AIService] ========== AI API 调用成功 ==========')
      return content
    } catch (error) {
      console.error('[AIService] ========== AI API 调用失败 ==========')

      if (error instanceof AIServiceError) {
        console.error('[AIService] 错误类型:', error.code)
        console.error('[AIService] 错误信息:', error.message)
        throw error
      }

      // 转换 OpenAI SDK 错误
      console.error('[AIService] 原始错误:', error)
      const message = error instanceof Error ? error.message : '未知错误'
      console.error('[AIService] 错误消息:', message)
      throw new AIServiceError(`AI 调用失败: ${message}`, 'API_ERROR')
    }
  }

  /**
   * 测试 AI 连接
   * @param config AI 配置
   * @param timeout 超时时间 (毫秒)
   * @returns 测试结果
   */
  async testConnection(config: AiConfig, timeout: number = 10000): Promise<TestResult> {
    try {
      // 临时创建客户端测试连接
      const testClient = new OpenAI({
        apiKey: config.api_key,
        baseURL: config.base_url,
        dangerouslyAllowBrowser: true
      })

      let timeoutId: ReturnType<typeof setTimeout> | null = null
      const timeoutPromise = new Promise<never>((_, reject) => {
        timeoutId = setTimeout(() => {
          reject(new AIServiceError(`连接测试超时 (${timeout}ms)`, 'TIMEOUT'))
        }, timeout)
      })

      const apiPromise = testClient.chat.completions.create({
        model: config.model,
        messages: [
          { role: 'user', content: '你好，这是一条测试消息，请回复"测试成功"' }
        ],
        max_tokens: 10
      })

      await Promise.race([apiPromise, timeoutPromise])

      // 清除超时定时器
      if (timeoutId) {
        clearTimeout(timeoutId)
      }

      return { success: true, message: '连接测试成功' }
    } catch (error) {
      const message = error instanceof Error ? error.message : '未知错误'
      return { success: false, message: `连接失败: ${message}` }
    }
  }

  /**
   * 获取当前配置
   */
  getConfig(): AiConfig | null {
    return this.config
  }
}

/**
 * AI 服务错误类型
 */
export class AIServiceError extends Error {
  constructor(message: string, public code: AIServiceErrorCode) {
    super(message)
    this.name = 'AIServiceError'
  }
}

export type AIServiceErrorCode =
  | 'NOT_INITIALIZED'   // 服务未初始化
  | 'TIMEOUT'           // 调用超时
  | 'EMPTY_RESPONSE'    // 响应为空
  | 'API_ERROR'         // API 调用错误

/**
 * 连接测试结果
 */
export interface TestResult {
  success: boolean
  message: string
}

// 导出单例实例
export const aiService = new AIService()