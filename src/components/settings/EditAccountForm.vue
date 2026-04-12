<template>
  <div class="p-4" @keydown="handleKeyDown">
    <div class="space-y-4">
      <!-- Display Name -->
      <div>
        <label for="displayName" class="block text-xs font-medium text-text-secondary mb-1">
          显示名称
        </label>
        <input
          id="displayName"
          v-model="form.displayName"
          type="text"
          placeholder="您的名字"
          class="w-full px-3 py-2 bg-bg-secondary border border-border-primary rounded-lg text-sm text-text-primary outline-none focus:border-accent transition-colors"
        />
      </div>

      <!-- Email (read-only) -->
      <div>
        <label for="email" class="block text-xs font-medium text-text-secondary mb-1">
          邮箱地址
        </label>
        <input
          id="email"
          v-model="form.email"
          type="email"
          disabled
          class="w-full px-3 py-2 bg-bg-tertiary border border-border-primary rounded-lg text-sm text-text-tertiary cursor-not-allowed"
        />
      </div>

      <!-- Password -->
      <div>
        <label for="password" class="block text-xs font-medium text-text-secondary mb-1">
          密码
        </label>
        <div class="relative">
          <input
            id="password"
            v-model="form.password"
            :type="showPassword ? 'text' : 'password'"
            placeholder="不修改请留空"
            class="w-full px-3 py-2 pr-10 bg-bg-secondary border border-border-primary rounded-lg text-sm text-text-primary outline-none focus:border-accent transition-colors"
          />
          <button
            type="button"
            class="absolute right-2 top-1/2 -translate-y-1/2 p-1 text-text-tertiary hover:text-text-primary"
            @click="showPassword = !showPassword"
          >
            <Eye v-if="showPassword" :size="16" />
            <EyeOff v-else :size="16" />
          </button>
        </div>
        <p class="text-xs text-text-tertiary mt-1">
          如需修改密码请输入新密码，否则请留空
        </p>
      </div>

      <!-- Divider -->
      <div class="border-t border-border-primary my-4" />

      <!-- Server Settings -->
      <div class="text-xs font-medium text-text-secondary mb-3">
        POP3 服务器设置
      </div>

      <div>
        <label for="host" class="block text-xs font-medium text-text-secondary mb-1">
          服务器地址
        </label>
        <input
          id="host"
          v-model="form.host"
          type="text"
          placeholder="pop.example.com"
          class="w-full px-3 py-2 bg-bg-secondary border border-border-primary rounded-lg text-sm text-text-primary outline-none focus:border-accent transition-colors"
        />
      </div>

      <div class="grid grid-cols-2 gap-3">
        <div>
          <label for="port" class="block text-xs font-medium text-text-secondary mb-1">
            端口
          </label>
          <input
            id="port"
            v-model.number="form.port"
            type="number"
            class="w-full px-3 py-2 bg-bg-secondary border border-border-primary rounded-lg text-sm text-text-primary outline-none focus:border-accent transition-colors"
          />
        </div>
        <div>
          <label for="security" class="block text-xs font-medium text-text-secondary mb-1">
            安全
          </label>
          <select
            id="security"
            v-model="form.security"
            class="w-full px-3 py-2 bg-bg-secondary border border-border-primary rounded-lg text-sm text-text-primary outline-none focus:border-accent transition-colors appearance-none"
            @change="handleSecurityChange"
          >
            <option value="ssl">SSL/TLS</option>
            <option value="none">无</option>
          </select>
        </div>
      </div>

      <!-- Divider -->
      <div class="border-t border-border-primary my-4" />

      <!-- Phishing Detection Settings -->
      <div class="text-xs font-medium text-text-secondary mb-3">
        钓鱼邮件检测
      </div>

      <div class="flex items-center justify-between py-2">
        <div>
          <div class="text-sm text-text-primary">启用钓鱼邮件检测</div>
          <div class="text-xs text-text-tertiary mt-0.5">
            使用 AI 分析邮件内容，识别潜在钓鱼邮件
          </div>
        </div>
        <button
          :class="[
            'relative inline-flex h-5 w-9 shrink-0 rounded-full transition-colors',
            form.phishingDetectionEnabled ? 'bg-accent' : 'bg-bg-tertiary'
          ]"
          @click="form.phishingDetectionEnabled = !form.phishingDetectionEnabled"
        >
          <span
            :class="[
              'inline-block h-4 w-4 rounded-full bg-white shadow transform transition-transform',
              form.phishingDetectionEnabled ? 'translate-x-4.5' : 'translate-x-0.5'
            ]"
          />
        </button>
      </div>

      <!-- AI Provider Warning -->
      <div v-if="form.phishingDetectionEnabled && !aiAvailable" class="p-2 rounded-md text-xs bg-yellow-500/10 text-yellow-500">
        <div class="flex items-center gap-1.5">
          <AlertCircle :size="14" />
          <span>AI Provider 未配置，请先在 AI 设置中完成配置</span>
        </div>
      </div>
    </div>

    <!-- Actions -->
    <div class="flex items-center justify-between mt-6">
      <div class="text-xs text-text-tertiary">
        建议修改服务器设置后测试连接
      </div>
      <div class="flex gap-2">
        <button
          class="px-4 py-2 text-sm text-text-secondary hover:text-text-primary transition-colors"
          @click="emit('close')"
        >
          取消
        </button>
        <button
          class="px-4 py-2 text-sm bg-accent text-white rounded-lg hover:bg-accent-hover transition-colors"
          @click="handleSave"
        >
          保存
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { Eye, EyeOff, AlertCircle } from 'lucide-vue-next'
import { useAccountStore, type Account } from '@/stores/account'
import { getDefaultPop3Port, type SecurityType } from '@/services/email/providerPresets'
import { updatePhishingDetection } from '@/services/email/accountService'
import { isAiAvailable } from '@/services/ai/aiConfig'

interface Props {
  account: Account | null
}

const props = defineProps<Props>()

const emit = defineEmits<{
  close: []
  saved: []
}>()

const accountStore = useAccountStore()

const showPassword = ref(false)
const aiAvailable = ref(false)

const form = ref({
  displayName: '',
  email: '',
  password: '',
  host: '',
  port: 110,
  security: 'none' as SecurityType,
  phishingDetectionEnabled: false
})

onMounted(async () => {
  if (props.account) {
    form.value.displayName = props.account.name || ''
    form.value.email = props.account.email
    form.value.host = props.account.incomingServer
    form.value.port = props.account.incomingPort
    form.value.security = props.account.useSsl ? 'ssl' : 'none'
    form.value.phishingDetectionEnabled = props.account.phishingDetectionEnabled || false
  }

  // Check AI availability
  try {
    aiAvailable.value = await isAiAvailable()
  } catch {
    aiAvailable.value = false
  }
})

function handleSecurityChange() {
  form.value.port = getDefaultPop3Port(form.value.security)
}

async function handleSave() {
  if (!props.account) return

  // Get password - use existing if not changed
  const password = form.value.password || ''

  // Update account settings
  await accountStore.updateAccount(
    props.account.id,
    form.value.displayName,
    password,
    form.value.host,
    form.value.port,
    form.value.security === 'ssl'
  )

  // Update phishing detection setting
  if (form.value.phishingDetectionEnabled !== props.account.phishingDetectionEnabled) {
    const updated = await updatePhishingDetection(props.account.id, form.value.phishingDetectionEnabled)
    // 更新本地 store 中的钓鱼检测开关状态
    const account = accountStore.accounts.find(a => a.id === props.account!.id)
    if (account) {
      account.phishingDetectionEnabled = updated.phishingDetectionEnabled
    }
  }

  emit('saved')
}

function handleKeyDown(e: KeyboardEvent) {
  if (e.key === 'Enter') {
    e.preventDefault()
    handleSave()
  }
  if (e.key === 'Escape') {
    emit('close')
  }
}
</script>

<style scoped>
.translate-x-4.5 {
  transform: translateX(18px);
}
</style>