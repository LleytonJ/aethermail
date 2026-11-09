<template>
  <div
    class="flex flex-col h-full bg-bg-primary relative"
    :class="{ 'ring-2 ring-accent ring-offset-2': isDragOver }"
    @dragover.prevent="handleDragOver"
    @dragenter.prevent="handleDragEnter"
    @dragleave.prevent="handleDragLeave"
    @drop.prevent="handleDrop"
  >
    <!-- 拖拽提示遮罩 -->
    <div
      v-if="isDragOver"
      class="absolute inset-0 bg-accent/10 backdrop-blur-sm z-10 flex items-center justify-center"
    >
      <div class="flex flex-col items-center gap-2 text-accent">
        <Upload :size="48" />
        <div class="text-lg font-medium">拖拽文件添加附件</div>
      </div>
    </div>

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

    <!-- 表单头部（固定）：收件人、抄送、主题 -->
    <div class="p-4 space-y-4 border-b border-border-primary">
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
        <div class="flex items-start gap-2">
          <span class="text-xs text-text-tertiary pt-2.5 min-w-[3rem] shrink-0">主题</span>
          <input
            v-model="form.subject"
            type="text"
            placeholder="邮件主题"
            class="flex-1 px-3 py-2 bg-bg-secondary border border-border-primary rounded-lg text-sm text-text-primary outline-none focus:border-accent transition-colors"
            :class="{ 'border-danger': errors.subject }"
          />
        </div>
        <p v-if="errors.subject" class="text-xs text-danger mt-1 ml-[3.5rem]">
          {{ errors.subject }}
        </p>
      </div>
    </div>

    <!-- 正文（可滚动区域） -->
    <div class="flex-1 flex flex-col min-h-0 p-4">
      <RichTextEditor
        v-model="form.body"
        placeholder="邮件内容..."
        class="flex-1 min-h-0"
        :min-height="0"
      />
      <p v-if="errors.body" class="text-xs text-danger mt-1">
        {{ errors.body }}
      </p>
    </div>

    <!-- 附件列表 (固定在底部栏上方) -->
    <ComposeAttachmentList
      :attachments="attachments"
      @remove="handleRemoveAttachment"
      @clear-all="handleClearAttachments"
    />

    <!-- 底部操作栏 -->
    <div class="flex items-center justify-between px-4 py-3 border-t border-border-primary">
      <!-- 发送状态 -->
      <div class="text-xs text-text-muted">
        <span v-if="sendingState === 'idle' && !isSavingDraft && !isProcessingAttachment">
          {{ attachments.length > 0 ? `${attachments.length} 个附件` : '准备发送' }}
        </span>
        <span v-else-if="sendingState === 'sending'" class="flex items-center gap-1">
          <Loader2 :size="14" class="animate-spin" />
          正在发送...
        </span>
        <span v-else-if="isSavingDraft" class="flex items-center gap-1">
          <Loader2 :size="14" class="animate-spin" />
          保存中...
        </span>
        <span v-else-if="isProcessingAttachment" class="flex items-center gap-1">
          <Loader2 :size="14" class="animate-spin" />
          处理附件...
        </span>
        <span v-else-if="sendingState === 'success'" class="text-success">发送成功</span>
        <span v-else-if="sendingState === 'error'" class="text-danger">{{ errorMessage }}</span>
      </div>

      <!-- 操作按钮 -->
      <div class="flex gap-2">
        <!-- 添加附件按钮 -->
        <button
          class="flex items-center gap-1.5 px-3 py-2 text-sm text-text-secondary hover:text-text-primary border border-border-primary rounded-lg hover:bg-bg-hover transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          :disabled="isProcessingAttachment || sendingState === 'sending' || isSavingDraft"
          title="添加附件"
          @click="handleAddAttachment"
        >
          <Paperclip :size="14" />
          附件
        </button>

        <button
          class="px-4 py-2 text-sm text-text-secondary hover:text-text-primary transition-colors"
          @click="handleClose"
        >
          取消
        </button>
        <button
          class="flex items-center gap-1.5 px-3 py-2 text-sm text-text-secondary hover:text-text-primary border border-border-primary rounded-lg hover:bg-bg-hover transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          :disabled="isSavingDraft || sendingState === 'sending'"
          @click="handleSaveDraft"
        >
          <Save :size="14" />
          保存草稿
        </button>
        <button
          class="flex items-center gap-1.5 px-4 py-2 text-sm bg-accent text-white rounded-lg hover:bg-accent-hover transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          :disabled="!canSend || sendingState === 'sending' || isSavingDraft"
          @click="handleSend"
        >
          <Send :size="14" />
          发送
        </button>
      </div>
    </div>
  </div>

  <!-- 关闭确认对话框 -->
  <ConfirmDialog
    :is-open="showCloseConfirm"
    title="保存邮件"
    message="邮件内容已修改，是否需要保存？"
    confirm-text="保存"
    neutral-text="不保存"
    cancel-text="取消"
    @confirm="handleCloseSave"
    @neutral="handleCloseDiscard"
    @cancel="handleCloseCancel"
  />
</template>

<script setup lang="ts">
import { ref, computed, watch, nextTick } from 'vue'
import { X, Send, Loader2, Save, Paperclip, Upload } from 'lucide-vue-next'
import { open } from '@tauri-apps/plugin-dialog'
import { useAccountStore } from '@/stores/account'
import { useNotificationStore } from '@/stores/notification'
import { sendEmail, createDraft, updateDraft, sendDraft, readFileForAttachment, type AttachmentData, type DraftAttachmentData, type EmailAddress, formatAddressForSend } from '@/services/email/emailService'
import AddressInput from '@/components/ui/AddressInput.vue'
import RichTextEditor from '@/components/ui/RichTextEditor.vue'
import ComposeAttachmentList from '@/components/email/ComposeAttachmentList.vue'
import ConfirmDialog from '@/components/ui/ConfirmDialog.vue'
import type { ComposeAttachment } from '@/stores/compose'

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
  // 编辑草稿模式：加载草稿内容
  draftId?: string
  draftData?: {
    toAddresses: string[]
    ccAddresses: string[]
    subject: string | null
    bodyHtml: string | null
    bodyText: string | null
  }
  draftAttachments?: ComposeAttachment[]
}

const props = defineProps<Props>()

const emit = defineEmits<{
  close: []
  sent: []
  saved: []
}>()

const accountStore = useAccountStore()
const notificationStore = useNotificationStore()

// 表单状态 - 收件人改为 EmailAddress 数组
const form = ref({
  to: [] as EmailAddress[],
  cc: [] as EmailAddress[],
  subject: '',
  body: ''
})

// 附件列表
const attachments = ref<ComposeAttachment[]>([])
console.log('[ComposeEmail] 初始化 attachments ref:', attachments.value)

// 草稿 ID（编辑草稿时有值）
const currentDraftId = ref<string | null>(null)

// 拖拽状态
const isDragOver = ref(false)

// 处理附件状态
const isProcessingAttachment = ref(false)

// 验证错误
const errors = ref({
  to: '',
  subject: '',
  body: ''
})

// 发送状态
const sendingState = ref<'idle' | 'sending' | 'success' | 'error'>('idle')
const errorMessage = ref('')
const isSavingDraft = ref(false)

// 关闭确认对话框状态
const showCloseConfirm = ref(false)

// 初始内容（用于判断是否有用户修改）
const initialTo = ref<EmailAddress[]>([])
const initialCc = ref<EmailAddress[]>([])
const initialSubject = ref('')
const initialBody = ref('')
const initialAttachments = ref<ComposeAttachment[]>([])

// 初始化表单（回复、转发或编辑草稿）
watch(
  [() => props.draftId, () => props.draftData, () => props.replyTo, () => props.forward, () => props.draftAttachments],
  ([draftId, draftData, replyTo, forward, draftAttachments]) => {
    console.log('[ComposeEmail] watch 触发 - draftId:', draftId, 'draftAttachments:', draftAttachments)
    // 获取账户签名
    const account = accountStore.activeAccount
    const signatureHtml = account?.signatureHtml || ''

    // 编辑草稿模式
    if (draftId && draftData) {
      currentDraftId.value = draftId
      // 转换 string[] 为 EmailAddress[]
      form.value.to = (draftData.toAddresses || []).map(addr => ({ email: addr }))
      form.value.cc = (draftData.ccAddresses || []).map(addr => ({ email: addr }))
      form.value.subject = draftData.subject || ''
      form.value.body = draftData.bodyHtml || draftData.bodyText || ''
      // 草稿不附加新签名（保留原有内容）

      // 直接使用草稿附件（content 已从后端获取）
      if (draftAttachments && draftAttachments.length > 0) {
        console.log('[ComposeEmail] 从草稿恢复附件, 数量:', draftAttachments.length)
        console.log('[ComposeEmail] draftAttachments 详情:', JSON.stringify(draftAttachments, null, 2))
        attachments.value = draftAttachments.map(att => ({
          id: att.id,
          filename: att.filename,
          contentType: att.contentType,
          size: att.size,
          path: att.path || '',
          content: att.content
        }))
        console.log('[ComposeEmail] 附件恢复完成:', attachments.value)
        for (const att of attachments.value) {
          console.log('[ComposeEmail] 恢复的附件:', att.filename, 'content长度:', att.content?.length || 0)
        }
      } else {
        attachments.value = []
      }

      // 记录初始值
      initialTo.value = [...form.value.to]
      initialCc.value = [...form.value.cc]
      initialSubject.value = form.value.subject
      initialBody.value = form.value.body
      initialAttachments.value = attachments.value.map(a => ({ ...a }))
    } else if (replyTo) {
      currentDraftId.value = null
      form.value.to = [{ email: replyTo.from }]
      form.value.cc = []
      form.value.subject = replyTo.subject.startsWith('Re:')
        ? replyTo.subject
        : `Re: ${replyTo.subject}`
      // 回复内容 + 签名
      form.value.body = `<br><br>--- 原始邮件 ---<br>${replyTo.body}${signatureHtml ? `<br><br><div style="color: #1a1a1a">${signatureHtml}</div>` : ''}`
      attachments.value = []

      // 记录初始值
      initialTo.value = [...form.value.to]
      initialCc.value = []
      initialSubject.value = form.value.subject
      initialBody.value = form.value.body
      initialAttachments.value = []
    } else if (forward) {
      currentDraftId.value = null
      form.value.to = []
      form.value.cc = []
      form.value.subject = forward.subject.startsWith('Fwd:')
        ? forward.subject
        : `Fwd: ${forward.subject}`
      // 转发内容 + 签名
      form.value.body = `<br><br>--- 转发邮件 ---<br>${forward.body}${signatureHtml ? `<br><br><div style="color: #1a1a1a">${signatureHtml}</div>` : ''}`
      attachments.value = []

      // 记录初始值
      initialTo.value = []
      initialCc.value = []
      initialSubject.value = form.value.subject
      initialBody.value = form.value.body
      initialAttachments.value = []
    } else {
      // 新邮件模式：重置表单，附加签名（签名前留空行供用户撰写内容）
      currentDraftId.value = null
      form.value.to = []
      form.value.cc = []
      form.value.subject = ''
      // 新邮件签名前留3个空行，方便用户在签名上方撰写正文
      form.value.body = signatureHtml ? `<br><br><br><div style="color: #1a1a1a">${signatureHtml}</div>` : ''
      attachments.value = []

      // 记录初始值
      initialTo.value = []
      initialCc.value = []
      initialSubject.value = ''
      initialBody.value = form.value.body
      initialAttachments.value = []
    }

    // TipTap 编辑器会规范化 HTML，需要等待编辑器稳定后重新同步 initialBody
    nextTick(() => {
      setTimeout(() => {
        console.log('[ComposeEmail] 同步 initialBody - form.body:', form.value.body?.substring(0, 100))
        initialBody.value = form.value.body
        console.log('[ComposeEmail] 同步后 initialBody:', initialBody.value?.substring(0, 100))
      }, 100)
    })
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
  for (const addr of form.value.to) {
    if (!validateEmail(addr.email)) {
      errors.value.to = `无效的邮箱地址: ${addr.email}`
      return false
    }
  }

  // 验证抄送格式（如果有）
  for (const addr of form.value.cc) {
    if (!validateEmail(addr.email)) {
      errors.value.to = `无效的抄送地址: ${addr.email}`
      return false
    }
  }

  // 验证主题
  if (!form.value.subject.trim()) {
    errors.value.subject = '请输入邮件主题'
    return false
  }

  // 验证正文（支持 HTML 内容）
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

// 推断 MIME 类型
function inferMimeType(filename: string): string {
  const ext = filename.split('.').pop()?.toLowerCase() || ''
  const mimeTypes: Record<string, string> = {
    'pdf': 'application/pdf',
    'doc': 'application/msword',
    'docx': 'application/vnd.openxmlformats-officedocument.wordprocessingml.document',
    'xls': 'application/vnd.ms-excel',
    'xlsx': 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
    'ppt': 'application/vnd.ms-powerpoint',
    'pptx': 'application/vnd.openxmlformats-officedocument.presentationml.presentation',
    'zip': 'application/zip',
    'rar': 'application/vnd.rar',
    '7z': 'application/x-7z-compressed',
    'jpg': 'image/jpeg',
    'jpeg': 'image/jpeg',
    'png': 'image/png',
    'gif': 'image/gif',
    'bmp': 'image/bmp',
    'webp': 'image/webp',
    'svg': 'image/svg+xml',
    'txt': 'text/plain',
    'html': 'text/html',
    'csv': 'text/csv',
    'json': 'application/json',
    'xml': 'application/xml',
    'mp3': 'audio/mpeg',
    'mp4': 'video/mp4',
  }
  return mimeTypes[ext] || 'application/octet-stream'
}

// 添加附件（从文件路径）
async function addAttachmentFromPath(filePath: string) {
  isProcessingAttachment.value = true

  console.log('[ComposeEmail] addAttachmentFromPath - filePath:', filePath)

  try {
    // 获取文件名
    const filename = filePath.split(/[\\/]/).pop() || 'unknown'

    // 读取文件内容
    const attachmentData = await readFileForAttachment(filePath)

    // 创建附件对象
    const attachment: ComposeAttachment = {
      id: `att-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`,
      filename: attachmentData.filename,
      contentType: attachmentData.content_type,
      size: 0, // 需要从 Base64 计算实际大小
      path: filePath,
      content: attachmentData.content
    }

    console.log('[ComposeEmail] addAttachmentFromPath - 创建附件对象:', attachment)
    console.log('[ComposeEmail] addAttachmentFromPath - path:', attachment.path)

    // 计算文件大小（从 Base64 内容）
    const base64Length = attachmentData.content.length
    const padding = attachmentData.content.endsWith('==') ? 2 : attachmentData.content.endsWith('=') ? 1 : 0
    attachment.size = Math.floor((base64Length - padding) * 3 / 4)

    // 添加到列表
    attachments.value.push(attachment)
    console.log('[ComposeEmail] addAttachmentFromPath - attachments.value:', attachments.value)
    console.log('[ComposeEmail] addAttachmentFromPath - 添加后的附件 path:', attachments.value[attachments.value.length - 1].path)

    notificationStore.success('附件已添加', filename)
  } catch (e) {
    notificationStore.error('添加附件失败', e instanceof Error ? e.message : '无法读取文件')
  } finally {
    isProcessingAttachment.value = false
  }
}

// 拖拽事件处理
function handleDragOver(e: DragEvent) {
  e.preventDefault()
}

function handleDragEnter(e: DragEvent) {
  e.preventDefault()
  isDragOver.value = true
}

function handleDragLeave(e: DragEvent) {
  e.preventDefault()
  // 检查是否真的离开了元素（而不是进入子元素）
  const rect = e.currentTarget as HTMLElement
  const x = e.clientX
  const y = e.clientY
  if (x < rect.offsetLeft || x > rect.offsetLeft + rect.offsetWidth ||
      y < rect.offsetTop || y > rect.offsetTop + rect.offsetHeight) {
    isDragOver.value = false
  }
}

async function handleDrop(e: DragEvent) {
  e.preventDefault()
  isDragOver.value = false

  // 获取拖拽的文件
  const items = e.dataTransfer?.items
  if (!items) return

  for (const item of items) {
    if (item.kind === 'file') {
      const entry = item.webkitGetAsEntry?.()
      if (entry && entry.isFile) {
        // 在 Tauri 中，我们需要获取文件路径
        // 由于安全限制，直接获取文件对象可能无法读取路径
        // 使用 Tauri 的文件系统 API
        const file = item.getAsFile()
        if (file) {
          // 在 Tauri 桌面应用中，拖拽的文件可能有路径信息
          // 但标准 Web File API 不提供路径，需要特殊处理
          // 检查是否有 path 属性（Tauri 扩展）
          const filePath = (file as any).path as string | undefined
          if (filePath) {
            await addAttachmentFromPath(filePath)
          } else {
            // 没有路径信息，读取文件内容并转为 Base64
            const reader = new FileReader()
            reader.onload = async () => {
              const arrayBuffer = reader.result as ArrayBuffer
              const base64 = btoa(new Uint8Array(arrayBuffer).reduce((acc, byte) => acc + String.fromCharCode(byte), ''))

              const attachment: ComposeAttachment = {
                id: `att-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`,
                filename: file.name,
                contentType: file.type || inferMimeType(file.name),
                size: file.size,
                path: '', // 无路径
                content: base64
              }
              attachments.value.push(attachment)
              notificationStore.success('附件已添加', file.name)
            }
            reader.readAsArrayBuffer(file)
          }
        }
      }
    }
  }
}

// 点击添加附件按钮
async function handleAddAttachment() {
  try {
    const selected = await open({
      multiple: true,
      title: '选择附件'
    })

    if (selected) {
      const paths = Array.isArray(selected) ? selected : [selected]
      for (const path of paths) {
        await addAttachmentFromPath(path)
      }
    }
  } catch (e) {
    console.error('选择文件失败:', e)
  }
}

// 移除附件
function handleRemoveAttachment(id: string) {
  attachments.value = attachments.value.filter(a => a.id !== id)
}

// 清空所有附件
function handleClearAttachments() {
  attachments.value = []
}

// 转换附件为发送格式
function prepareAttachmentsForSend(): AttachmentData[] | null {
  if (attachments.value.length === 0) return null

  return attachments.value.map(a => ({
    filename: a.filename,
    content_type: a.contentType,
    content: a.content || '' // 需要 Base64 内容
  }))
}

// 转换附件为草稿格式
function prepareAttachmentsForDraft(): DraftAttachmentData[] | null {
  if (attachments.value.length === 0) return null

  console.log('[ComposeEmail] prepareAttachmentsForDraft - attachments.value:', attachments.value)
  const result = attachments.value.map(a => ({
    id: a.id,
    filename: a.filename,
    content_type: a.contentType,
    file_path: a.path || null,
    content: a.content || null,  // 直接传递 Base64 content
    file_size: a.size
  }))
  console.log('[ComposeEmail] prepareAttachmentsForDraft - 结果:', result)
  for (const att of result) {
    console.log('[ComposeEmail] prepareAttachmentsForDraft - 附件:', att.filename, 'content长度:', att.content?.length || 0)
  }
  return result
}

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

  // 签名已附加到 body 中，发送时不需要额外处理
  const signatureHtml = null

  try {
    // 准备附件数据
    const attachmentData = prepareAttachmentsForSend()
    console.log('[ComposeEmail] handleSend - attachmentData:', attachmentData)
    if (attachmentData) {
      for (const att of attachmentData) {
        console.log('[ComposeEmail] handleSend - 附件:', att.filename, 'content长度:', att.content?.length || 0)
      }
    }

    // 区分草稿发送和新邮件发送
    let result
    // 格式化地址为 SMTP 兼容字符串
    const toAddresses = form.value.to.map(formatAddressForSend)
    const ccAddresses = form.value.cc.length > 0 ? form.value.cc.map(formatAddressForSend) : null

    if (currentDraftId.value) {
      // 发送草稿 - 传递当前附件数据
      result = await sendDraft(currentDraftId.value, signatureHtml, attachmentData)
    } else {
      // 发送新邮件
      result = await sendEmail(
        account.id,
        toAddresses,
        ccAddresses,
        form.value.subject,
        form.value.body, // plain text fallback
        form.value.body, // HTML body (from rich text editor)
        signatureHtml,
        attachmentData
      )
    }

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

// 保存草稿
async function handleSaveDraft() {
  const account = accountStore.activeAccount
  if (!account) {
    notificationStore.error('保存失败', '没有可用的账户')
    return
  }

  isSavingDraft.value = true

  try {
    const draftAttachmentData = prepareAttachmentsForDraft()

    // 转换 EmailAddress[] 为 string[]（保留显示名称信息）
    const toAddresses = form.value.to.map(formatAddressForSend)
    const ccAddresses = form.value.cc.map(formatAddressForSend)

    const draftData = {
      accountId: account.id,
      toAddresses,
      ccAddresses,
      subject: form.value.subject || null,
      bodyText: form.value.body.replace(/<[^>]*>/g, ''), // 提取纯文本
      bodyHtml: form.value.body,
      attachments: draftAttachmentData
    }

    if (currentDraftId.value) {
      // 更新现有草稿
      await updateDraft(currentDraftId.value, draftData)
      notificationStore.success('草稿已保存', '草稿内容已更新')
    } else {
      // 创建新草稿
      const draftId = await createDraft(draftData)
      currentDraftId.value = draftId
      notificationStore.success('草稿已保存', '草稿已保存到草稿箱')
    }

    emit('saved')
  } catch (e) {
    notificationStore.error('保存失败', e instanceof Error ? e.message : '保存草稿失败')
  } finally {
    isSavingDraft.value = false
  }
}

// 关闭
function handleClose() {
  // 检查是否有用户修改（与初始值比较）
  const hasChanges = checkHasChanges()
  if (hasChanges) {
    showCloseConfirm.value = true
  } else {
    emit('close')
  }
}

// 检查是否有用户修改
function checkHasChanges(): boolean {
  console.log('[ComposeEmail] checkHasChanges - 开始检查')
  console.log('[ComposeEmail] checkHasChanges - form.to:', form.value.to, 'initialTo:', initialTo.value)
  console.log('[ComposeEmail] checkHasChanges - form.cc:', form.value.cc, 'initialCc:', initialCc.value)
  console.log('[ComposeEmail] checkHasChanges - form.subject:', form.value.subject, 'initialSubject:', initialSubject.value)
  console.log('[ComposeEmail] checkHasChanges - form.body length:', form.value.body?.length, 'initialBody length:', initialBody.value?.length)
  console.log('[ComposeEmail] checkHasChanges - form.body:', form.value.body?.substring(0, 100))
  console.log('[ComposeEmail] checkHasChanges - initialBody:', initialBody.value?.substring(0, 100))
  console.log('[ComposeEmail] checkHasChanges - attachments:', attachments.value.length, 'initialAttachments:', initialAttachments.value.length)

  // 比较收件人
  if (form.value.to.length !== initialTo.value.length ||
      !form.value.to.every((addr, i) => addr === initialTo.value[i])) {
    console.log('[ComposeEmail] checkHasChanges - 收件人有变化')
    return true
  }

  // 比较抄送
  if (form.value.cc.length !== initialCc.value.length ||
      !form.value.cc.every((addr, i) => addr === initialCc.value[i])) {
    console.log('[ComposeEmail] checkHasChanges - 抄送有变化')
    return true
  }

  // 比较主题
  if (form.value.subject !== initialSubject.value) {
    console.log('[ComposeEmail] checkHasChanges - 主题有变化')
    return true
  }

  // 比较正文
  if (form.value.body !== initialBody.value) {
    console.log('[ComposeEmail] checkHasChanges - 正文有变化')
    console.log('[ComposeEmail] checkHasChanges - body length:', form.value.body?.length, 'initialBody length:', initialBody.value?.length)
    console.log('[ComposeEmail] checkHasChanges - body === initialBody:', form.value.body === initialBody.value)
    // 找出不同的位置
    if (form.value.body && initialBody.value) {
      for (let i = 0; i < Math.max(form.value.body.length, initialBody.value.length); i++) {
        if (form.value.body[i] !== initialBody.value[i]) {
          console.log(`[ComposeEmail] checkHasChanges - 差异位置 ${i}: body="${form.value.body[i]}" initial="${initialBody.value[i]}"`)
          if (i > 10) break // 只显示前10个差异
        }
      }
    }
    return true
  }

  // 比较附件数量
  if (attachments.value.length !== initialAttachments.value.length) {
    console.log('[ComposeEmail] checkHasChanges - 附件有变化')
    return true
  }

  console.log('[ComposeEmail] checkHasChanges - 无变化')
  return false
}

// 关闭确认对话框 - 保存草稿
async function handleCloseSave() {
  showCloseConfirm.value = false
  await handleSaveDraft()
  emit('close')
}

// 关闭确认对话框 - 不保存
function handleCloseDiscard() {
  showCloseConfirm.value = false
  emit('close')
}

// 关闭确认对话框 - 取消
function handleCloseCancel() {
  showCloseConfirm.value = false
}

// 暴露保存草稿方法，供父组件调用
defineExpose({
  saveDraft: handleSaveDraft,
  hasChanges: checkHasChanges
})
</script>

