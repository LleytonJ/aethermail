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

      <!-- Protocol Type (read-only) -->
      <div>
        <label class="block text-xs font-medium text-text-secondary mb-1">
          收件协议
        </label>
        <div class="px-3 py-2 bg-bg-tertiary border border-border-primary rounded-lg text-sm text-text-tertiary">
          {{ form.protocol === 'imap' ? 'IMAP' : 'POP3' }}
        </div>
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

      <!-- Incoming Server Settings -->
      <div class="text-xs font-medium text-text-secondary mb-3">
        {{ form.protocol === 'imap' ? 'IMAP' : 'POP3' }} 收件服务器
      </div>

      <div>
        <label for="incomingHost" class="block text-xs font-medium text-text-secondary mb-1">
          服务器地址
        </label>
        <input
          id="incomingHost"
          v-model="form.incomingHost"
          type="text"
          :placeholder="form.protocol === 'imap' ? 'imap.example.com' : 'pop.example.com'"
          class="w-full px-3 py-2 bg-bg-secondary border border-border-primary rounded-lg text-sm text-text-primary outline-none focus:border-accent transition-colors"
        />
      </div>

      <div class="grid grid-cols-2 gap-3">
        <div>
          <label for="incomingPort" class="block text-xs font-medium text-text-secondary mb-1">
            端口
          </label>
          <input
            id="incomingPort"
            v-model.number="form.incomingPort"
            type="number"
            class="w-full px-3 py-2 bg-bg-secondary border border-border-primary rounded-lg text-sm text-text-primary outline-none focus:border-accent transition-colors"
          />
        </div>
        <div>
          <label for="incomingSecurity" class="block text-xs font-medium text-text-secondary mb-1">
            安全
          </label>
          <select
            id="incomingSecurity"
            v-model="form.incomingSecurity"
            class="w-full px-3 py-2 bg-bg-secondary border border-border-primary rounded-lg text-sm text-text-primary outline-none focus:border-accent transition-colors appearance-none"
            @change="handleIncomingSecurityChange"
          >
            <option value="ssl">SSL/TLS</option>
            <option value="none">无</option>
          </select>
        </div>
      </div>

      <!-- Divider -->
      <div class="border-t border-border-primary my-4" />

      <!-- SMTP Server Settings -->
      <div class="text-xs font-medium text-text-secondary mb-3">
        SMTP 发件服务器
      </div>

      <div>
        <label for="smtpHost" class="block text-xs font-medium text-text-secondary mb-1">
          服务器地址
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

      <!-- Divider -->
      <div class="border-t border-border-primary my-4" />

      <!-- Signature Settings -->
      <div class="text-xs font-medium text-text-secondary mb-3">
        签名设置
      </div>

      <div class="flex items-center justify-between py-2">
        <div>
          <div class="text-sm text-text-primary">启用签名</div>
          <div class="text-xs text-text-tertiary mt-0.5">
            发送邮件时自动附加签名到正文底部
          </div>
        </div>
        <button
          :class="[
            'relative inline-flex h-5 w-9 shrink-0 rounded-full transition-colors',
            form.signatureEnabled ? 'bg-accent' : 'bg-bg-tertiary'
          ]"
          @click="form.signatureEnabled = !form.signatureEnabled"
        >
          <span
            :class="[
              'inline-block h-4 w-4 rounded-full bg-white shadow transform transition-transform',
              form.signatureEnabled ? 'translate-x-4.5' : 'translate-x-0.5'
            ]"
          />
        </button>
      </div>

      <!-- Signature Editor -->
      <div v-if="form.signatureEnabled" class="mt-3">
        <label class="block text-xs font-medium text-text-secondary mb-1">
          签名内容
        </label>
        <RichTextEditor
          v-model="form.signatureHtml"
          placeholder="输入签名内容..."
          :min-height="120"
          :editable="form.signatureEnabled"
          :show-toolbar="true"
        />
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
import { useAccountStore, type Account, type ProtocolType } from '@/stores/account'
import { getDefaultPop3Port, getDefaultImapPort, getDefaultSmtpPort, type SecurityType } from '@/services/email/providerPresets'
import { updatePhishingDetection, updateAccount as updateAccountService, updateSignature } from '@/services/email/accountService'
import { isAiAvailable } from '@/services/ai/aiConfig'
import RichTextEditor from '@/components/ui/RichTextEditor.vue'

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
  protocol: 'pop3' as ProtocolType,
  incomingHost: '',
  incomingPort: 110,
  incomingSecurity: 'none' as SecurityType,
  smtpHost: '',
  smtpPort: 25,
  smtpSecurity: 'none' as SecurityType,
  phishingDetectionEnabled: false,
  signatureEnabled: false,
  signatureHtml: ''
})

onMounted(async () => {
  if (props.account) {
    form.value.displayName = props.account.name || ''
    form.value.email = props.account.email
    form.value.protocol = props.account.protocol
    form.value.incomingHost = props.account.incomingServer
    form.value.incomingPort = props.account.incomingPort
    form.value.incomingSecurity = props.account.useSsl ? 'ssl' : 'none'
    form.value.smtpHost = props.account.smtpHost || ''
    form.value.smtpPort = props.account.smtpPort || (props.account.protocol === 'imap' ? 465 : 25)
    form.value.smtpSecurity = props.account.smtpUseSsl ? 'ssl' : 'none'
    form.value.phishingDetectionEnabled = props.account.phishingDetectionEnabled || false
    form.value.signatureHtml = props.account.signatureHtml || ''
    form.value.signatureEnabled = (props.account.signatureHtml && props.account.signatureHtml.length > 0) || false
  }

  // Check AI availability
  try {
    aiAvailable.value = await isAiAvailable()
  } catch {
    aiAvailable.value = false
  }
})

function handleIncomingSecurityChange() {
  if (form.value.protocol === 'imap') {
    form.value.incomingPort = getDefaultImapPort(form.value.incomingSecurity)
  } else {
    form.value.incomingPort = getDefaultPop3Port(form.value.incomingSecurity)
  }
}

function handleSmtpSecurityChange() {
  form.value.smtpPort = getDefaultSmtpPort(form.value.smtpSecurity)
}

async function handleSave() {
  if (!props.account) return

  // Get password - use existing if not changed
  const password = form.value.password || ''

  // Update account settings via backend
  try {
    await updateAccountService(
      props.account.id,
      form.value.displayName,
      password,
      form.value.incomingHost,
      form.value.incomingPort,
      form.value.incomingSecurity === 'ssl',
      form.value.smtpHost,
      form.value.smtpPort,
      form.value.smtpSecurity === 'ssl'
    )

    // Update local store
    const account = accountStore.accounts.find(a => a.id === props.account!.id)
    if (account) {
      account.name = form.value.displayName
      account.incomingServer = form.value.incomingHost
      account.incomingPort = form.value.incomingPort
      account.useSsl = form.value.incomingSecurity === 'ssl'
      account.smtpHost = form.value.smtpHost
      account.smtpPort = form.value.smtpPort
      account.smtpUseSsl = form.value.smtpSecurity === 'ssl'
    }

    // Update phishing detection setting
    if (form.value.phishingDetectionEnabled !== props.account.phishingDetectionEnabled) {
      const updated = await updatePhishingDetection(props.account.id, form.value.phishingDetectionEnabled)
      if (account) {
        account.phishingDetectionEnabled = updated.phishingDetectionEnabled
      }
    }

    // Update signature
    const signatureToSave = form.value.signatureEnabled ? form.value.signatureHtml : ''
    if (signatureToSave !== (props.account.signatureHtml || '')) {
      const updated = await updateSignature(props.account.id, signatureToSave)
      if (account) {
        account.signatureHtml = updated.signatureHtml
      }
    }

    emit('saved')
  } catch (e) {
    console.error('Failed to update account:', e)
  }
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