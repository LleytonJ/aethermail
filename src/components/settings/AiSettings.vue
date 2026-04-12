<template>
  <div class="space-y-6">
    <!-- AI 功能开关 -->
    <div class="space-y-4">
      <div class="flex items-center justify-between">
        <div>
          <h3 class="text-sm font-medium text-text-primary">启用 AI 功能</h3>
          <p class="text-xs text-text-secondary mt-0.5">
            开启后可使用钓鱼邮件检测等 AI 功能
          </p>
        </div>
        <button
          :class="[
            'relative inline-flex h-5 w-9 shrink-0 rounded-full transition-colors',
            config.enabled ? 'bg-accent' : 'bg-bg-tertiary'
          ]"
          @click="toggleEnabled"
        >
          <span
            :class="[
              'inline-block h-4 w-4 rounded-full bg-white shadow transform transition-transform',
              config.enabled ? 'translate-x-4.5' : 'translate-x-0.5'
            ]"
          />
        </button>
      </div>
    </div>

    <!-- Provider 配置 -->
    <div class="space-y-4">
      <h3 class="text-sm font-medium text-text-primary">AI Provider 配置</h3>

      <!-- 预设 Provider 选择 -->
      <div class="space-y-2">
        <label class="text-xs text-text-secondary">选择 Provider</label>
        <select
          v-model="selectedProvider"
          class="w-full px-3 py-2 text-sm rounded-md border border-border-primary bg-bg-primary text-text-primary focus:border-accent focus:outline-none"
          @change="handleProviderChange"
        >
          <option value="">自定义</option>
          <option v-for="preset in PRESET_PROVIDERS" :key="preset.provider" :value="preset.provider">
            {{ preset.name }}
          </option>
        </select>
      </div>

      <!-- Base URL -->
      <div class="space-y-2">
        <label class="text-xs text-text-secondary">API Base URL</label>
        <input
          v-model="config.base_url"
          type="text"
          class="w-full px-3 py-2 text-sm rounded-md border border-border-primary bg-bg-primary text-text-primary focus:border-accent focus:outline-none"
          placeholder="https://api.openai.com/v1"
        />
      </div>

      <!-- API Key -->
      <div class="space-y-2">
        <label class="text-xs text-text-secondary">API Key</label>
        <input
          v-model="config.api_key"
          type="password"
          class="w-full px-3 py-2 text-sm rounded-md border border-border-primary bg-bg-primary text-text-primary focus:border-accent focus:outline-none"
          placeholder="sk-xxx"
        />
      </div>

      <!-- Model -->
      <div class="space-y-2">
        <label class="text-xs text-text-secondary">模型名称</label>
        <input
          v-model="config.model"
          type="text"
          class="w-full px-3 py-2 text-sm rounded-md border border-border-primary bg-bg-primary text-text-primary focus:border-accent focus:outline-none"
          placeholder="gpt-4o-mini"
        />
      </div>
    </div>

    <!-- 操作按钮 -->
    <div class="flex items-center gap-3">
      <button
        :disabled="!canSave"
        :class="[
          'px-4 py-2 text-sm font-medium rounded-md transition-colors',
          canSave
            ? 'bg-accent text-white hover:bg-accent-hover'
            : 'bg-bg-tertiary text-text-tertiary cursor-not-allowed'
        ]"
        @click="handleSave"
      >
        保存配置
      </button>
      <button
        :disabled="!canTest"
        :class="[
          'px-4 py-2 text-sm font-medium rounded-md transition-colors',
          canTest
            ? 'bg-bg-secondary text-text-primary hover:bg-bg-hover border border-border-primary'
            : 'bg-bg-tertiary text-text-tertiary cursor-not-allowed'
        ]"
        @click="handleTestConnection"
      >
        {{ testing ? '测试中...' : '测试连接' }}
      </button>
    </div>

    <!-- 状态提示 -->
    <div v-if="testResult" class="p-3 rounded-md text-sm" :class="testResult.success ? 'bg-green-500/10 text-green-500' : 'bg-red-500/10 text-red-500'">
      {{ testResult.message }}
    </div>

    <!-- 未配置警告 -->
    <div v-if="!config.enabled || !config.api_key" class="p-3 rounded-md text-sm bg-yellow-500/10 text-yellow-500">
      <div class="flex items-center gap-2">
        <AlertCircle :size="16" />
        <span>AI 功能未配置，钓鱼邮件检测将无法使用</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { AlertCircle } from 'lucide-vue-next'
import { getAiConfig, saveAiConfig, PRESET_PROVIDERS, type AiConfig } from '@/services/ai/aiConfig'
import { aiService } from '@/services/ai/aiService'
import { useNotificationStore } from '@/stores/notification'

const notificationStore = useNotificationStore()

const config = ref<AiConfig>({
  provider: 'openai',
  base_url: 'https://api.openai.com/v1',
  api_key: '',
  model: 'gpt-4o-mini',
  enabled: false
})

const selectedProvider = ref('')
const testing = ref(false)
const testResult = ref<{ success: boolean; message: string } | null>(null)

// 是否可以保存
const canSave = computed(() => {
  return config.value.base_url && config.value.model
})

// 是否可以测试连接
const canTest = computed(() => {
  return config.value.base_url && config.value.api_key && config.value.model
})

// 加载配置
async function loadConfig() {
  try {
    const loaded = await getAiConfig()
    config.value = loaded

    // 设置预设 Provider
    const preset = PRESET_PROVIDERS.find(p => p.provider === loaded.provider)
    if (preset) {
      selectedProvider.value = preset.provider
    }
  } catch (error) {
    console.error('加载 AI 配置失败:', error)
  }
}

// 切换启用状态
async function toggleEnabled() {
  config.value.enabled = !config.value.enabled
  await handleSave()
}

// Provider 选择变化
function handleProviderChange() {
  const preset = PRESET_PROVIDERS.find(p => p.provider === selectedProvider.value)
  if (preset) {
    config.value.provider = preset.provider
    config.value.base_url = preset.baseUrl
    config.value.model = preset.defaultModel
  }
}

// 保存配置
async function handleSave() {
  try {
    await saveAiConfig(config.value)
    await aiService.init(config.value)
    notificationStore.success('AI 配置已保存')
  } catch (error) {
    const message = error instanceof Error ? error.message : '保存失败'
    notificationStore.error('保存失败', message)
  }
}

// 测试连接
async function handleTestConnection() {
  testing.value = true
  testResult.value = null

  try {
    const result = await aiService.testConnection(config.value, 10000)
    testResult.value = result
    if (result.success) {
      notificationStore.success('连接测试成功')
    } else {
      notificationStore.error('测试失败', result.message)
    }
  } catch (error) {
    const message = error instanceof Error ? error.message : '测试失败'
    testResult.value = { success: false, message }
    notificationStore.error('测试失败', message)
  } finally {
    testing.value = false
  }
}

onMounted(() => {
  loadConfig()
})
</script>

<style scoped>
.translate-x-4.5 {
  transform: translateX(18px);
}
</style>