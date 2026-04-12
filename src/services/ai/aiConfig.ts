/**
 * AI 配置服务 - 调用 Tauri 后端命令管理 AI Provider 配置
 */

import { invoke } from '@tauri-apps/api/core'

/**
 * AI Provider 配置接口
 */
export interface AiConfig {
  provider: string       // Provider 名称 (openai, bailian, deepseek 等)
  base_url: string       // API Base URL
  api_key: string        // API Key
  model: string          // 模型名称
  enabled: boolean       // 是否启用 AI 功能
}

/**
 * 预设 Provider 配置
 */
export const PRESET_PROVIDERS = [
  {
    name: 'OpenAI',
    provider: 'openai',
    baseUrl: 'https://api.openai.com/v1',
    defaultModel: 'gpt-4o-mini'
  },
  {
    name: '百炼 (阿里云)',
    provider: 'bailian',
    baseUrl: 'https://dashscope.aliyuncs.com/compatible-mode/v1',
    defaultModel: 'qwen-turbo'
  },
  {
    name: 'DeepSeek',
    provider: 'deepseek',
    baseUrl: 'https://api.deepseek.com/v1',
    defaultModel: 'deepseek-chat'
  },
  {
    name: 'Moonshot (月之暗面)',
    provider: 'moonshot',
    baseUrl: 'https://api.moonshot.cn/v1',
    defaultModel: 'moonshot-v1-8k'
  },
  {
    name: '智谱 AI',
    provider: 'zhipu',
    baseUrl: 'https://open.bigmodel.cn/api/paas/v4',
    defaultModel: 'glm-4-flash'
  },
  {
    name: 'Ollama (本地)',
    provider: 'ollama',
    baseUrl: 'http://localhost:11434/v1',
    defaultModel: 'llama3'
  }
] as const

/**
 * 获取 AI 配置
 */
export async function getAiConfig(): Promise<AiConfig> {
  return invoke<AiConfig>('get_ai_config')
}

/**
 * 保存 AI 配置
 */
export async function saveAiConfig(config: AiConfig): Promise<AiConfig> {
  return invoke<AiConfig>('save_ai_config', {
    provider: config.provider,
    baseUrl: config.base_url,
    apiKey: config.api_key,
    model: config.model,
    enabled: config.enabled
  })
}

/**
 * 更新 AI 配置 (部分更新)
 */
export async function updateAiConfig(
  updates: Partial<AiConfig>
): Promise<AiConfig> {
  return invoke<AiConfig>('update_ai_config', {
    provider: updates.provider,
    baseUrl: updates.base_url,
    apiKey: updates.api_key,
    model: updates.model,
    enabled: updates.enabled
  })
}

/**
 * 获取配置目录路径
 */
export async function getConfigDir(): Promise<string> {
  return invoke<string>('get_config_dir')
}

/**
 * 检查 AI 是否可用 (配置完成且已启用)
 */
export async function isAiAvailable(): Promise<boolean> {
  return invoke<boolean>('is_ai_available')
}