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
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { X, Send, Loader2, Save, Paperclip, Upload } from 'lucide-vue-next'
import { open } from '@tauri-apps/plugin-dialog'
import { useAccountStore } from '@/stores/account'
import { useNotificationStore } from '@/stores/notification'
import { sendEmail, createDraft, updateDraft, sendDraft, readFileForAttachment, type AttachmentData, type DraftAttachmentData } from '@/services/email/emailService'
import AddressInput from '@/components/ui/AddressInput.vue'
import RichTextEditor from '@/components/ui/RichTextEditor.vue'
import ComposeAttachmentList from '@/components/email/ComposeAttachmentList.vue'
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

// 表单状态 - 收件人改为数组形式
const form = ref({
  to: [] as string[],
  cc: [] as string[],
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
      form.value.to = draftData.toAddresses || []
      form.value.cc = draftData.ccAddresses || []
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
    } else if (replyTo) {
      currentDraftId.value = null
      form.value.to = [replyTo.from]
      form.value.subject = replyTo.subject.startsWith('Re:')
        ? replyTo.subject
        : `Re: ${replyTo.subject}`
      // 回复内容 + 签名
      form.value.body = `<br><br>--- 原始邮件 ---<br>${replyTo.body}${signatureHtml ? `<br><br>${signatureHtml}` : ''}`
      attachments.value = []
    } else if (forward) {
      currentDraftId.value = null
      form.value.subject = forward.subject.startsWith('Fwd:')
        ? forward.subject
        : `Fwd: ${forward.subject}`
      // 转发内容 + 签名
      form.value.body = `<br><br>--- 转发邮件 ---<br>${forward.body}${signatureHtml ? `<br><br>${signatureHtml}` : ''}`
      attachments.value = []
    } else {
      // 新邮件模式：重置表单，附加签名（签名前留空行供用户撰写内容）
      currentDraftId.value = null
      form.value.to = []
      form.value.cc = []
      form.value.subject = ''
      // 新邮件签名前留3个空行，方便用户在签名上方撰写正文
      form.value.body = signatureHtml ? `<br><br><br>${signatureHtml}` : ''
      attachments.value = []
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
    if (currentDraftId.value) {
      // 发送草稿 - 传递当前附件数据
      result = await sendDraft(currentDraftId.value, signatureHtml, attachmentData)
    } else {
      // 发送新邮件
      result = await sendEmail(
        account.id,
        form.value.to,
        form.value.cc.length > 0 ? form.value.cc : null,
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

    const draftData = {
      accountId: account.id,
      toAddresses: form.value.to,
      ccAddresses: form.value.cc,
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
  const hasContent = form.value.to.length > 0 || form.value.subject || form.value.body || attachments.value.length > 0
  if (hasContent) {
    if (confirm('您已填写了一些内容，确定要关闭吗？')) {
      emit('close')
    }
  } else {
    emit('close')
  }
}
</script>

