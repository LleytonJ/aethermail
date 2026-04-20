<template>
  <div class="flex flex-col h-full bg-bg-primary">
    <!-- 头部 -->
    <div class="flex items-center justify-between px-4 py-3 border-b border-border-primary">
      <h2 class="text-lg font-semibold text-text-primary">撰写邮件</h2>
      <button
        class="p-1.5 rounded-md text-text-muted hover:text-text-secondary hover:bg-bg-hover transition-colors"
        title="关闭"
        @click="handleClose"
      >
        <X :size="18" />
      </button>
    </div>

    <!-- 表单内容 -->
    <div class="flex-1 overflow-y-auto p-4 space-y-4">
      <!-- 收件人 -->
      <div>
        <AddressInput
          label="收件人"
          :model-value="form.to"
          placeholder="输入邮箱地址..."
          @update:model-value="form.to = $event"
        />
        <p v-if="errors.to" class="text-xs text-danger mt-1">
          {{ errors.to }}
        </p>
      </div>

      <!-- 抄送（可选） -->
      <div>
        <AddressInput
          label="抄送"
          :model-value="form.cc"
          placeholder="输入邮箱地址（可选）..."
          @update:model-value="form.cc = $event"
        />
      </div>

      <!-- 主题 -->
      <div>
        <label class="block text-xs font-medium text-text-secondary mb-1">
          主题
        </label>
        <input
          v-model="form.subject"
          type="text"
          placeholder="邮件主题"
          class="w-full px-3 py-2 bg-bg-secondary border border-border-primary rounded-lg text-sm text-text-primary outline-none focus:border-accent transition-colors"
          :class="{ 'border-danger': errors.subject }"
        />
        <p v-if="errors.subject" class="text-xs text-danger mt-1">
          {{ errors.subject }}
        </p>
      </div>

      <!-- 正文 -->
      <div class="flex-1">
        <label class="block text-xs font-medium text-text-secondary mb-1">
          正文
        </label>
        <RichTextEditor
          v-model="form.body"
          placeholder="邮件内容..."
          :min-height="256"
        />
        <p v-if="errors.body" class="text-xs text-danger mt-1">
          {{ errors.body }}
        </p>
      </div>

      <!-- 签名开关 -->
      <div v-if="showSignatureToggle" class="flex items-center justify-between py-2 bg-bg-secondary rounded-lg px-3">
        <div class="text-sm text-text-secondary">附加签名</div>
        <button
          :class="[
            'relative inline-flex h-5 w-9 shrink-0 rounded-full transition-colors',
            form.includeSignature ? 'bg-accent' : 'bg-bg-tertiary'
          ]"
          @click="form.includeSignature = !form.includeSignature"
        >
          <span
            :class="[
              'inline-block h-4 w-4 rounded-full bg-white shadow transform transition-transform',
              form.includeSignature ? 'translate-x-4.5' : 'translate-x-0.5'
            ]"
          />
        </button>
      </div>
    </div>

    <!-- 底部操作栏 -->
    <div class="flex items-center justify-between px-4 py-3 border-t border-border-primary">
      <!-- 发送状态 -->
      <div class="text-xs text-text-muted">
        <span v-if="sendingState === 'idle'">准备发送</span>
        <span v-else-if="sendingState === 'sending'" class="flex items-center gap-1">
          <Loader2 :size="14" class="animate-spin" />
          正在发送...
        </span>
        <span v-else-if="sendingState === 'success'" class="text-success">发送成功</span>
        <span v-else-if="sendingState === 'error'" class="text-danger">{{ errorMessage }}</span>
      </div>

      <!-- 操作按钮 -->
      <div class="flex gap-2">
        <button
          class="px-4 py-2 text-sm text-text-secondary hover:text-text-primary transition-colors"
          @click="handleClose"
        >
          取消
        </button>
        <button
          class="flex items-center gap-1.5 px-4 py-2 text-sm bg-accent text-white rounded-lg hover:bg-accent-hover transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          :disabled="!canSend || sendingState === 'sending'"
          @click="handleSend"
        >
          <Send :size="14" />
          发送
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { X, Send, Loader2 } from 'lucide-vue-next'
import { useAccountStore } from '@/stores/account'
import { useNotificationStore } from '@/stores/notification'
import { sendEmail } from '@/services/email/emailService'
import AddressInput from '@/components/ui/AddressInput.vue'
import RichTextEditor from '@/components/ui/RichTextEditor.vue'

interface Props {
  // 回复模式：预填充收件人、主题、引用正文
  replyTo?: {
    from: string
    subject: string
    body: string
  }
  // 转发模式：预填充主题、转发正文
  forward?: {
    subject: string
    body: string
  }
}

const props = defineProps<Props>()

const emit = defineEmits<{
  close: []
  sent: []
}>()

const accountStore = useAccountStore()
const notificationStore = useNotificationStore()

// 表单状态 - 收件人改为数组形式
const form = ref({
  to: [] as string[],
  cc: [] as string[],
  subject: '',
  body: '',
  includeSignature: true
})

// 签名开关显示条件
const showSignatureToggle = computed(() => {
  const account = accountStore.activeAccount
  return account && account.signatureHtml && account.signatureHtml.length > 0
})

// 验证错误
const errors = ref({
  to: '',
  subject: '',
  body: ''
})

// 发送状态
const sendingState = ref<'idle' | 'sending' | 'success' | 'error'>('idle')
const errorMessage = ref('')

// 初始化表单（回复或转发）
watch(
  () => props,
  (p) => {
    if (p.replyTo) {
      form.value.to = [p.replyTo.from]
      form.value.subject = p.replyTo.subject.startsWith('Re:')
        ? p.replyTo.subject
        : `Re: ${p.replyTo.subject}`
      form.value.body = `\n\n--- 原始邮件 ---\n${p.replyTo.body}`
    } else if (p.forward) {
      form.value.subject = p.forward.subject.startsWith('Fwd:')
        ? p.forward.subject
        : `Fwd: ${p.forward.subject}`
      form.value.body = `\n\n--- 转发邮件 ---\n${p.forward.body}`
    }
  },
  { immediate: true }
)

// 验证邮箱格式
function validateEmail(email: string): boolean {
  const regex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/
  return regex.test(email)
}

// 验证表单
function validateForm(): boolean {
  errors.value = { to: '', subject: '', body: '' }

  // 验证收件人
  if (form.value.to.length === 0) {
    errors.value.to = '请输入收件人地址'
    return false
  }

  // 验证收件人格式
  for (const email of form.value.to) {
    if (!validateEmail(email)) {
      errors.value.to = `无效的邮箱地址: ${email}`
      return false
    }
  }

  // 验证抄送格式（如果有）
  for (const email of form.value.cc) {
    if (!validateEmail(email)) {
      errors.value.to = `无效的抄送地址: ${email}`
      return false
    }
  }

  // 验证主题
  if (!form.value.subject.trim()) {
    errors.value.subject = '请输入邮件主题'
    return false
  }

  // 验证正文（支持 HTML 内容）
  // Strip HTML tags to check if there's actual content
  const textContent = form.value.body.replace(/<[^>]*>/g, '').trim()
  if (!textContent && !form.value.body.trim()) {
    errors.value.body = '请输入邮件内容'
    return false
  }

  return true
}

// 是否可以发送
const canSend = computed(() => {
  return form.value.to.length > 0 &&
         form.value.subject.trim() &&
         form.value.body.trim()
})

// 发送邮件
async function handleSend() {
  if (!validateForm()) return

  const account = accountStore.activeAccount
  if (!account) {
    notificationStore.error('发送失败', '没有可用的账户')
    return
  }

  sendingState.value = 'sending'
  errorMessage.value = ''

  // 获取签名 HTML（如果启用）
  const signatureHtml = form.value.includeSignature && account.signatureHtml
    ? account.signatureHtml
    : null

  try {
    const result = await sendEmail(
      account.id,
      form.value.to,
      form.value.cc.length > 0 ? form.value.cc : null,
      form.value.subject,
      form.value.body, // plain text fallback
      form.value.body, // HTML body (from rich text editor)
      signatureHtml
    )

    if (result.success) {
      sendingState.value = 'success'
      notificationStore.success('发送成功', '邮件已发送')

      // 短暂延迟后关闭
      setTimeout(() => {
        emit('sent')
        emit('close')
      }, 500)
    } else {
      sendingState.value = 'error'
      errorMessage.value = result.message
      notificationStore.error('发送失败', result.message)
    }
  } catch (e) {
    sendingState.value = 'error'
    errorMessage.value = e instanceof Error ? e.message : '发送失败'
    notificationStore.error('发送失败', errorMessage.value)
  }
}

// 关闭
function handleClose() {
  if (form.value.to.length > 0 || form.value.subject || form.value.body) {
    if (confirm('您已填写了一些内容，确定要关闭吗？')) {
      emit('close')
    }
  } else {
    emit('close')
  }
}
</script>

<style scoped>
.translate-x-4.5 {
  transform: translateX(18px);
}
</style>