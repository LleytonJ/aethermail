<template>
  <div class="p-4" @keydown="handleKeyDown">
    <!-- Step Indicator -->
    <div class="flex items-center justify-center gap-1 mb-6">
      <template v-for="(step, index) in steps" :key="step.id">
        <div
          v-if="index > 0"
          class="w-6 h-px transition-colors"
          :class="currentStepIndex > index - 1 ? 'bg-accent' : 'bg-border-primary'"
        />
        <div
          class="flex items-center gap-1.5 px-2 py-1 rounded-md text-xs font-medium transition-colors"
          :class="
            currentStepIndex === index
              ? 'bg-accent/10 text-accent'
              : currentStepIndex > index
                ? 'text-accent'
                : 'text-text-tertiary'
          "
        >
          <component :is="step.icon" class="w-4 h-4" />
          <span class="hidden sm:inline">{{ step.label }}</span>
        </div>
      </template>
    </div>

    <!-- Step Content -->
    <div class="min-h-[280px]">
      <!-- Step 1: Basic Info -->
      <div v-if="currentStep === 'basic'" class="space-y-4">
        <div>
          <label for="email" class="block text-xs font-medium text-text-secondary mb-1">
            邮箱地址
          </label>
          <input
            id="email"
            v-model="form.email"
            type="email"
            placeholder="you@example.com"
            class="w-full px-3 py-2 bg-bg-secondary border border-border-primary rounded-lg text-sm text-text-primary outline-none focus:border-accent transition-colors"
            :class="{ 'border-danger': errors.email }"
            @blur="validateEmail"
          />
          <p v-if="errors.email" class="text-xs text-danger mt-1">
            {{ errors.email }}
          </p>
        </div>

        <div>
          <label for="displayName" class="block text-xs font-medium text-text-secondary mb-1">
            显示名称（可选）
          </label>
          <input
            id="displayName"
            v-model="form.displayName"
            type="text"
            placeholder="您的名字"
            class="w-full px-3 py-2 bg-bg-secondary border border-border-primary rounded-lg text-sm text-text-primary outline-none focus:border-accent transition-colors"
          />
        </div>

        <div>
          <label for="password" class="block text-xs font-medium text-text-secondary mb-1">
            密码
          </label>
          <div class="relative">
            <input
              id="password"
              v-model="form.password"
              :type="showPassword ? 'text' : 'password'"
              placeholder="输入邮箱密码"
              class="w-full px-3 py-2 pr-10 bg-bg-secondary border border-border-primary rounded-lg text-sm text-text-primary outline-none focus:border-accent transition-colors"
              :class="{ 'border-danger': errors.password }"
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
          <p v-if="errors.password" class="text-xs text-danger mt-1">
            {{ errors.password }}
          </p>
          <p class="text-xs text-text-tertiary mt-1">
            提示：请确保已开启邮箱的 POP3 服务
          </p>
        </div>
      </div>

      <!-- Step 2: POP3 Server -->
      <div v-else-if="currentStep === 'server'" class="space-y-4">
        <div class="text-xs text-text-tertiary mb-3">
          POP3 用于接收邮件，邮件会被下载到本地。
        </div>
        <div v-if="presetMatched" class="text-xs text-text-tertiary">
          服务器设置已自动填充，您可以手动修改。
        </div>

        <div>
          <label for="host" class="block text-xs font-medium text-text-secondary mb-1">
            POP3 服务器
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
      </div>

      <!-- Step 3: SMTP Server -->
      <div v-else-if="currentStep === 'smtp'" class="space-y-4">
        <div class="text-xs text-text-tertiary mb-3">
          SMTP 用于发送邮件，无论 POP3 还是 IMAP 收件协议都使用 SMTP 发件。
        </div>
        <div v-if="presetMatched" class="text-xs text-text-tertiary">
          服务器设置已自动填充，您可以手动修改。
        </div>

        <div>
          <label for="smtpHost" class="block text-xs font-medium text-text-secondary mb-1">
            SMTP 服务器
          </label>
          <input
            id="smtpHost"
            v-model="form.smtpHost"
            type="text"
            placeholder="smtp.example.com"
            class="w-full px-3 py-2 bg-bg-secondary border border-border-primary rounded-lg text-sm text-text-primary outline-none focus:border-accent transition-colors"
          />
        </div>

        <div class="grid grid-cols-2 gap-3">
          <div>
            <label for="smtpPort" class="block text-xs font-medium text-text-secondary mb-1">
              端口
            </label>
            <input
              id="smtpPort"
              v-model.number="form.smtpPort"
              type="number"
              class="w-full px-3 py-2 bg-bg-secondary border border-border-primary rounded-lg text-sm text-text-primary outline-none focus:border-accent transition-colors"
            />
          </div>
          <div>
            <label for="smtpSecurity" class="block text-xs font-medium text-text-secondary mb-1">
              安全
            </label>
            <select
              id="smtpSecurity"
              v-model="form.smtpSecurity"
              class="w-full px-3 py-2 bg-bg-secondary border border-border-primary rounded-lg text-sm text-text-primary outline-none focus:border-accent transition-colors appearance-none"
              @change="handleSmtpSecurityChange"
            >
              <option value="ssl">SSL/TLS</option>
              <option value="none">无</option>
            </select>
          </div>
        </div>
      </div>

      <!-- Step 4: Test Connection -->
      <div v-else-if="currentStep === 'test'" class="space-y-4">
        <div class="text-sm text-text-secondary mb-2">
          添加账户前请先测试连接设置。
        </div>

        <div class="space-y-3">
          <!-- POP3 Test Result -->
          <div class="flex items-start gap-3 p-3 rounded-lg bg-bg-secondary border border-border-primary">
            <div class="mt-0.5">
              <Loader2 v-if="testStatus.state === 'testing'" class="w-4 h-4 animate-spin text-accent" />
              <CheckCircle2 v-else-if="testStatus.state === 'success'" class="w-4 h-4 text-success" />
              <XCircle v-else-if="testStatus.state === 'error'" class="w-4 h-4 text-danger" />
              <div v-else class="w-4 h-4 rounded-full border-2 border-border-primary" />
            </div>
            <div class="flex-1 min-w-0">
              <div class="text-sm font-medium text-text-primary">POP3 连接</div>
              <div
                v-if="testStatus.message"
                class="text-xs mt-0.5"
                :class="{
                  'text-danger': testStatus.state === 'error',
                  'text-success': testStatus.state === 'success',
                  'text-text-tertiary': testStatus.state === 'testing'
                }"
              >
                {{ testStatus.message }}
              </div>
              <div v-if="testStatus.serverInfo" class="text-xs text-text-tertiary mt-1">
                {{ testStatus.serverInfo }}
              </div>
            </div>
          </div>

          <!-- SMTP Test Result -->
          <div class="flex items-start gap-3 p-3 rounded-lg bg-bg-secondary border border-border-primary">
            <div class="mt-0.5">
              <Loader2 v-if="smtpTestStatus.state === 'testing'" class="w-4 h-4 animate-spin text-accent" />
              <CheckCircle2 v-else-if="smtpTestStatus.state === 'success'" class="w-4 h-4 text-success" />
              <XCircle v-else-if="smtpTestStatus.state === 'error'" class="w-4 h-4 text-danger" />
              <div v-else class="w-4 h-4 rounded-full border-2 border-border-primary" />
            </div>
            <div class="flex-1 min-w-0">
              <div class="text-sm font-medium text-text-primary">SMTP 连接</div>
              <div
                v-if="smtpTestStatus.message"
                class="text-xs mt-0.5"
                :class="{
                  'text-danger': smtpTestStatus.state === 'error',
                  'text-success': smtpTestStatus.state === 'success',
                  'text-text-tertiary': smtpTestStatus.state === 'testing'
                }"
              >
                {{ smtpTestStatus.message }}
              </div>
            </div>
          </div>
        </div>

        <button
          class="w-full px-4 py-2 text-sm bg-bg-secondary border border-border-primary rounded-lg text-text-primary hover:bg-bg-hover transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          :disabled="testStatus.state === 'testing' || smtpTestStatus.state === 'testing'"
          @click="runTest"
        >
          <span v-if="testStatus.state === 'testing' || smtpTestStatus.state === 'testing'">测试中...</span>
          <span v-else-if="testStatus.state === 'idle' && smtpTestStatus.state === 'idle'">测试连接</span>
          <span v-else>重新测试</span>
        </button>
      </div>

      <!-- Step 5: Success -->
      <div v-else-if="currentStep === 'done'" class="text-center py-6">
        <div class="w-16 h-16 mx-auto mb-4 rounded-full bg-success/10 flex items-center justify-center">
          <CheckCircle2 class="w-8 h-8 text-success" />
        </div>
        <h3 class="text-lg font-semibold text-text-primary mb-2">
          账户已添加！
        </h3>
        <p class="text-sm text-text-secondary mb-6">
          {{ form.email }} 已成功添加。
        </p>

        <div class="flex gap-3 justify-center">
          <button
            class="px-4 py-2 text-sm bg-bg-secondary border border-border-primary rounded-lg text-text-primary hover:bg-bg-hover transition-colors"
            @click="handleAddAnother"
          >
            继续添加
          </button>
          <button
            class="px-4 py-2 text-sm bg-accent text-white rounded-lg hover:bg-accent-hover transition-colors"
            @click="handleGoToInbox"
          >
            前往收件箱
          </button>
        </div>
      </div>
    </div>

    <!-- Navigation Buttons -->
    <div v-if="currentStep !== 'done'" class="flex items-center justify-between mt-6">
      <button
        class="flex items-center gap-1 px-3 py-2 text-sm text-text-secondary hover:text-text-primary transition-colors"
        @click="goBack"
      >
        <ArrowLeft class="w-3.5 h-3.5" />
        上一步
      </button>

      <div class="flex gap-2">
        <button
          class="px-4 py-2 text-sm text-text-secondary hover:text-text-primary transition-colors"
          @click="handleCancel"
        >
          取消
        </button>

        <button
          v-if="currentStep === 'test'"
          class="px-4 py-2 text-sm bg-accent text-white rounded-lg hover:bg-accent-hover transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          :disabled="testStatus.state !== 'success' || smtpTestStatus.state !== 'success'"
          @click="handleSave"
        >
          添加账户
        </button>
        <button
          v-else
          class="flex items-center gap-1 px-4 py-2 text-sm bg-accent text-white rounded-lg hover:bg-accent-hover transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          :disabled="!canGoNext"
          @click="goNext"
        >
          下一步
          <ArrowRight class="w-3.5 h-3.5" />
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import {
  Mail,
  Server,
  Send,
  ShieldCheck,
  CheckCircle2,
  ArrowLeft,
  ArrowRight,
  Loader2,
  XCircle,
  Eye,
  EyeOff
} from 'lucide-vue-next'
import { useAccountStore } from '@/stores/account'
import {
  getProviderPreset,
  getDefaultPop3Port,
  getDefaultSmtpPort,
  type SecurityType
} from '@/services/email/providerPresets'
import {
  testConnection,
  testSmtpConnection,
  type Pop3Config,
  type SmtpConfig
} from '@/services/email/accountService'
import {
  parseConnectionError,
  parseAuthError
} from '@/services/email/errorUtils'

const emit = defineEmits<{
  close: []
  success: []
  back: []
}>()

const accountStore = useAccountStore()

// Steps configuration
type StepId = 'basic' | 'server' | 'smtp' | 'test' | 'done'

interface Step {
  id: StepId
  label: string
  icon: any
}

const steps: Step[] = [
  { id: 'basic', label: '账户', icon: Mail },
  { id: 'server', label: '收件', icon: Server },
  { id: 'smtp', label: '发件', icon: Send },
  { id: 'test', label: '测试', icon: ShieldCheck },
  { id: 'done', label: '完成', icon: CheckCircle2 }
]

const currentStep = ref<StepId>('basic')
const showPassword = ref(false)

// Form state
const form = ref({
  email: '',
  displayName: '',
  password: '',
  host: '',
  port: 110,
  security: 'none' as SecurityType,
  smtpHost: '',
  smtpPort: 25,
  smtpSecurity: 'none' as SecurityType
})

// Validation errors
const errors = ref({
  email: '',
  password: ''
})

// Test status
const testStatus = ref<{
  state: 'idle' | 'testing' | 'success' | 'error'
  message?: string
  serverInfo?: string
}>({
  state: 'idle'
})

// SMTP test status
const smtpTestStatus = ref<{
  state: 'idle' | 'testing' | 'success' | 'error'
  message?: string
}>({
  state: 'idle'
})

// Preset matched flag
const presetMatched = ref(false)

// Computed
const currentStepIndex = computed(() => {
  return steps.findIndex(s => s.id === currentStep.value)
})

const canGoNext = computed(() => {
  switch (currentStep.value) {
    case 'basic':
      return form.value.email.includes('@') && form.value.password.length > 0
    case 'server':
      return form.value.host.length > 0 && form.value.port > 0
    case 'smtp':
      return form.value.smtpHost.length > 0 && form.value.smtpPort > 0
    default:
      return false
  }
})

// Methods
function validateEmail() {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/
  if (form.value.email && !emailRegex.test(form.value.email)) {
    errors.value.email = '请输入有效的邮箱地址'
  } else {
    errors.value.email = ''
  }
}

// Auto-fill from preset when email changes
watch(
  () => form.value.email,
  (email) => {
    if (email.includes('@')) {
      const preset = getProviderPreset(email)
      if (preset) {
        form.value.host = preset.pop3.host
        form.value.port = preset.pop3.port
        form.value.security = preset.pop3.security
        // 同时填充 SMTP 预设
        if (preset.smtp) {
          form.value.smtpHost = preset.smtp.host
          form.value.smtpPort = preset.smtp.port
          form.value.smtpSecurity = preset.smtp.security
        }
        presetMatched.value = true
      } else {
        presetMatched.value = false
      }
    }
  }
)

function handleSecurityChange() {
  form.value.port = getDefaultPop3Port(form.value.security)
}

function handleSmtpSecurityChange() {
  form.value.smtpPort = getDefaultSmtpPort(form.value.smtpSecurity)
}

async function runTest() {
  testStatus.value = { state: 'testing' }
  smtpTestStatus.value = { state: 'testing' }

  // 测试 POP3 连接
  const pop3Config: Pop3Config = {
    host: form.value.host,
    port: form.value.port,
    use_ssl: form.value.security === 'ssl',
    username: form.value.email,
    password: form.value.password
  }

  // 测试 SMTP 连接
  const smtpConfig: SmtpConfig = {
    host: form.value.smtpHost,
    port: form.value.smtpPort,
    use_ssl: form.value.smtpSecurity === 'ssl',
    username: form.value.email,
    password: form.value.password
  }

  // 并行测试两个连接
  const [pop3Result, smtpResult] = await Promise.allSettled([
    testConnection(pop3Config),
    testSmtpConnection(smtpConfig)
  ])

  // 处理 POP3 结果
  if (pop3Result.status === 'fulfilled') {
    const result = pop3Result.value
    if (result.success) {
      testStatus.value = {
        state: 'success',
        message: result.message,
        serverInfo: result.server_greeting
      }
    } else {
      const parsed = parseConnectionError(result.message)
      testStatus.value = {
        state: 'error',
        message: parsed.title + (parsed.message ? ': ' + parsed.message : '')
      }
    }
  } else {
    const errorMsg = pop3Result.reason instanceof Error ? pop3Result.reason.message : String(pop3Result.reason)
    let parsed: { title: string; message: string }
    if (errorMsg.includes('认证') || errorMsg.includes('USER') || errorMsg.includes('PASS')) {
      parsed = parseAuthError(pop3Result.reason)
    } else {
      parsed = parseConnectionError(pop3Result.reason)
    }
    testStatus.value = {
      state: 'error',
      message: parsed.title + (parsed.message ? ': ' + parsed.message : '')
    }
  }

  // 处理 SMTP 结果
  if (smtpResult.status === 'fulfilled') {
    const result = smtpResult.value
    if (result.success) {
      smtpTestStatus.value = {
        state: 'success',
        message: result.message
      }
    } else {
      smtpTestStatus.value = {
        state: 'error',
        message: result.message
      }
    }
  } else {
    const errorMsg = smtpResult.reason instanceof Error ? smtpResult.reason.message : String(smtpResult.reason)
    smtpTestStatus.value = {
      state: 'error',
      message: errorMsg
    }
  }
}

async function handleSave() {
  try {
    // Add account via backend
    await accountStore.addAccount(
      form.value.email,
      form.value.displayName || null,
      form.value.password,
      form.value.host,
      form.value.port,
      form.value.security === 'ssl',
      form.value.smtpHost,
      form.value.smtpPort,
      form.value.smtpSecurity === 'ssl'
    )

    // Move to done step
    currentStep.value = 'done'
  } catch (e) {
    const errorMsg = e instanceof Error ? e.message : String(e)
    let parsed: { title: string; message: string }

    if (errorMsg.includes('认证') || errorMsg.includes('USER') || errorMsg.includes('PASS')) {
      parsed = parseAuthError(e)
    } else {
      parsed = parseConnectionError(e)
    }

    testStatus.value = {
      state: 'error',
      message: parsed.title + (parsed.message ? ': ' + parsed.message : '')
    }
  }
}

function goNext() {
  const idx = currentStepIndex.value
  if (idx < steps.length - 1) {
    currentStep.value = steps[idx + 1].id
  }
}

function goBack() {
  const idx = currentStepIndex.value
  if (idx > 0) {
    currentStep.value = steps[idx - 1].id
    // Reset test status when going back to test step
    if (currentStep.value === 'test') {
      testStatus.value = { state: 'idle' }
      smtpTestStatus.value = { state: 'idle' }
    }
  } else {
    emit('back')
  }
}

function handleKeyDown(e: KeyboardEvent) {
  if (e.key === 'Enter' && currentStep.value !== 'test' && canGoNext.value) {
    e.preventDefault()
    goNext()
  }
  if (e.key === 'Escape') {
    handleCancel()
  }
}

// 检查是否有已填写的内容
const hasFormData = computed(() => {
  return form.value.email.length > 0 ||
         form.value.password.length > 0 ||
         form.value.displayName.length > 0 ||
         form.value.host.length > 0
})

// 处理取消操作
function handleCancel() {
  if (hasFormData.value && currentStep.value !== 'done') {
    // 用户已填写内容，弹出确认提示
    if (confirm('您已填写了一些内容，确定要取消吗？')) {
      emit('close')
    }
  } else {
    emit('close')
  }
}

function handleGoToInbox() {
  emit('success')
}

function handleAddAnother() {
  // Reset form
  form.value = {
    email: '',
    displayName: '',
    password: '',
    host: '',
    port: 110,
    security: 'none',
    smtpHost: '',
    smtpPort: 25,
    smtpSecurity: 'none'
  }
  testStatus.value = { state: 'idle' }
  smtpTestStatus.value = { state: 'idle' }
  presetMatched.value = false
  currentStep.value = 'basic'
}
</script>