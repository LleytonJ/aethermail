<template>
  <Modal
    :is-open="isOpen"
    :title="view === 'select' ? '添加账户' : view === 'pop3' ? '添加 POP3 账户' : '添加 IMAP 账户'"
    :width="view === 'select' ? 'md' : 'lg'"
    :close-on-backdrop="false"
    @close="handleClose"
  >
    <!-- Provider Selection View -->
    <div v-if="view === 'select'" class="p-4">
      <p class="text-text-secondary text-sm mb-4">
        选择您要连接的邮箱账户类型。
      </p>

      <div class="space-y-3">
        <!-- IMAP Option -->
        <button
          class="w-full flex items-center gap-4 p-4 rounded-lg border border-border-primary bg-bg-secondary hover:bg-bg-hover transition-colors text-left group"
          @click="view = 'imap'"
        >
          <div class="flex-shrink-0 w-10 h-10 rounded-lg bg-accent/10 flex items-center justify-center">
            <Inbox class="w-5 h-5 text-accent" />
          </div>
          <div class="flex-1 min-w-0">
            <div class="text-sm font-medium text-text-primary group-hover:text-accent transition-colors">
              IMAP
            </div>
            <div class="text-xs text-text-tertiary mt-0.5">
              双向同步，保留服务器邮件，支持文件夹（推荐）
            </div>
          </div>
        </button>

        <!-- POP3/SMTP Option -->
        <button
          class="w-full flex items-center gap-4 p-4 rounded-lg border border-border-primary bg-bg-secondary hover:bg-bg-hover transition-colors text-left group"
          @click="view = 'pop3'"
        >
          <div class="flex-shrink-0 w-10 h-10 rounded-lg bg-bg-tertiary flex items-center justify-center">
            <Mail class="w-5 h-5 text-text-secondary" />
          </div>
          <div class="flex-1 min-w-0">
            <div class="text-sm font-medium text-text-primary group-hover:text-accent transition-colors">
              POP3 / SMTP
            </div>
            <div class="text-xs text-text-tertiary mt-0.5">
              手动配置服务器，支持任意邮箱服务商
            </div>
          </div>
        </button>

        <!-- Gmail Option (Coming Soon) -->
        <button
          disabled
          class="w-full flex items-center gap-4 p-4 rounded-lg border border-border-primary bg-bg-secondary/50 text-left cursor-not-allowed opacity-60"
        >
          <div class="flex-shrink-0 w-10 h-10 rounded-lg bg-bg-tertiary flex items-center justify-center">
            <svg class="w-5 h-5" viewBox="0 0 24 24">
              <path
                d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92a5.06 5.06 0 0 1-2.2 3.32v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.1z"
                fill="#4285F4"
              />
              <path
                d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"
                fill="#34A853"
              />
              <path
                d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"
                fill="#FBBC05"
              />
              <path
                d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"
                fill="#EA4335"
              />
            </svg>
          </div>
          <div class="flex-1 min-w-0">
            <div class="text-sm font-medium text-text-primary">
              Google (Gmail)
              <span class="ml-2 text-xs text-text-tertiary font-normal">即将推出</span>
            </div>
            <div class="text-xs text-text-tertiary mt-0.5">
              通过 OAuth 登录，支持完整 Gmail API
            </div>
          </div>
        </button>
      </div>

      <div class="flex justify-end mt-4">
        <button
          class="px-4 py-2 text-sm text-text-secondary hover:text-text-primary transition-colors"
          @click="handleClose"
        >
          取消
        </button>
      </div>
    </div>

    <!-- POP3 Configuration View -->
    <AddPop3Account
      v-else-if="view === 'pop3'"
      @close="handleClose"
      @success="handleSuccess"
      @back="view = 'select'"
    />

    <!-- IMAP Configuration View -->
    <AddImapAccount
      v-else-if="view === 'imap'"
      @close="handleClose"
      @success="handleSuccess"
      @back="view = 'select'"
    />
  </Modal>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { Mail, Inbox } from 'lucide-vue-next'
import Modal from '@/components/ui/Modal.vue'
import AddPop3Account from './AddPop3Account.vue'
import AddImapAccount from './AddImapAccount.vue'

interface Props {
  isOpen: boolean
}

defineProps<Props>()

const emit = defineEmits<{
  close: []
  success: []
}>()

type View = 'select' | 'pop3' | 'imap'
const view = ref<View>('select')

function handleClose() {
  view.value = 'select'
  emit('close')
}

function handleSuccess() {
  view.value = 'select'
  emit('success')
}
</script>