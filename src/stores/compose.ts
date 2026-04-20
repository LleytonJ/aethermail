import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

/**
 * 撰写邮件状态
 */
export interface ComposeState {
  isOpen: boolean
  mode: 'new' | 'reply' | 'forward' | 'draft'
  replyTo?: {
    from: string
    subject: string
    body: string
  }
  forward?: {
    subject: string
    body: string
  }
  draft?: {
    draftId: string
    toAddresses: string[]
    ccAddresses: string[]
    subject: string | null
    bodyHtml: string | null
    bodyText: string | null
  }
}

export const useComposeStore = defineStore('compose', () => {
  // 状态
  const state = ref<ComposeState>({
    isOpen: false,
    mode: 'new'
  })

  // 计算属性
  const isOpen = computed(() => state.value.isOpen)
  const mode = computed(() => state.value.mode)

  // 打开撰写新邮件
  function openNewEmail() {
    state.value = {
      isOpen: true,
      mode: 'new'
    }
  }

  // 打开回复邮件
  function openReply(from: string, subject: string, body: string) {
    state.value = {
      isOpen: true,
      mode: 'reply',
      replyTo: { from, subject, body }
    }
  }

  // 打开转发邮件
  function openForward(subject: string, body: string) {
    state.value = {
      isOpen: true,
      mode: 'forward',
      forward: { subject, body }
    }
  }

  // 打开编辑草稿
  function openDraft(draftId: string, toAddresses: string[], ccAddresses: string[], subject: string | null, bodyHtml: string | null, bodyText: string | null) {
    state.value = {
      isOpen: true,
      mode: 'draft',
      draft: { draftId, toAddresses, ccAddresses, subject, bodyHtml, bodyText }
    }
  }

  // 关闭撰写
  function close() {
    state.value = {
      isOpen: false,
      mode: 'new'
    }
  }

  return {
    state,
    isOpen,
    mode,
    openNewEmail,
    openReply,
    openForward,
    openDraft,
    close
  }
})