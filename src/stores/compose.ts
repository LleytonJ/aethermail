import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

/**
 * 撰写附件接口
 */
export interface ComposeAttachment {
  id: string
  filename: string
  contentType: string
  size: number
  path: string
  content?: string  // Base64 编码内容（可选）
}

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
    attachments?: ComposeAttachment[]
  }
}

export const useComposeStore = defineStore('compose', () => {
  // 状态
  const state = ref<ComposeState>({
    isOpen: false,
    mode: 'new'
  })

  // 附件列表
  const attachments = ref<ComposeAttachment[]>([])

  // 计算属性
  const isOpen = computed(() => state.value.isOpen)
  const mode = computed(() => state.value.mode)
  const hasAttachments = computed(() => attachments.value.length > 0)

  // 打开撰写新邮件
  function openNewEmail() {
    state.value = {
      isOpen: true,
      mode: 'new'
    }
    attachments.value = []
  }

  // 打开回复邮件
  function openReply(from: string, subject: string, body: string) {
    state.value = {
      isOpen: true,
      mode: 'reply',
      replyTo: { from, subject, body }
    }
    attachments.value = []
  }

  // 打开转发邮件
  function openForward(subject: string, body: string) {
    state.value = {
      isOpen: true,
      mode: 'forward',
      forward: { subject, body }
    }
    attachments.value = []
  }

  // 打开编辑草稿
  function openDraft(draftId: string, toAddresses: string[], ccAddresses: string[], subject: string | null, bodyHtml: string | null, bodyText: string | null, draftAttachments?: ComposeAttachment[]) {
    console.log('[compose store] openDraft - draftId:', draftId, 'attachments:', draftAttachments)
    state.value = {
      isOpen: true,
      mode: 'draft',
      draft: { draftId, toAddresses, ccAddresses, subject, bodyHtml, bodyText, attachments: draftAttachments }
    }
    attachments.value = draftAttachments || []
    console.log('[compose store] openDraft - attachments.value:', attachments.value)
  }

  // 关闭撰写
  function close() {
    state.value = {
      isOpen: false,
      mode: 'new'
    }
    attachments.value = []
  }

  // 添加附件
  function addAttachment(attachment: ComposeAttachment) {
    // 检查是否已存在相同 ID 的附件
    if (!attachments.value.find(a => a.id === attachment.id)) {
      attachments.value.push(attachment)
    }
  }

  // 移除附件
  function removeAttachment(attachmentId: string) {
    attachments.value = attachments.value.filter(a => a.id !== attachmentId)
  }

  // 清空所有附件
  function clearAttachments() {
    attachments.value = []
  }

  return {
    state,
    attachments,
    isOpen,
    mode,
    hasAttachments,
    openNewEmail,
    openReply,
    openForward,
    openDraft,
    close,
    addAttachment,
    removeAttachment,
    clearAttachments
  }
})