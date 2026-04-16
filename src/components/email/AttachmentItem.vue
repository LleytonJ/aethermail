<template>
  <div
    class="flex items-center gap-3 p-3 bg-bg-tertiary rounded-lg border border-border-secondary hover:border-border-light transition-all cursor-pointer group"
    :class="{ 'opacity-50': isOffline && !attachment.isDownloaded }"
    @click="handleClick"
  >
    <!-- 文件类型图标 -->
    <div class="flex-shrink-0 w-10 h-10 rounded-lg bg-bg-card flex items-center justify-center">
      <component :is="fileIcon" :size="20" class="text-accent" />
    </div>

    <!-- 文件信息 -->
    <div class="flex-1 min-w-0">
      <div class="text-sm text-text-primary truncate font-medium">
        {{ attachment.filename }}
      </div>
      <div class="text-xs text-text-muted">
        {{ formatSize(attachment.size) }}
        <span v-if="attachment.isDownloaded" class="text-green-500 ml-2">已下载</span>
        <span v-else-if="isOffline" class="text-yellow-500 ml-2">需要网络</span>
        <span v-else class="text-text-muted ml-2">点击下载</span>
      </div>

      <!-- 下载进度条 -->
      <div v-if="status === 'downloading'" class="mt-1 w-full">
        <div class="h-1 bg-bg-card rounded-full overflow-hidden">
          <div
            class="h-full bg-accent transition-all duration-300"
            :style="{ width: `${progress}%` }"
          />
        </div>
        <div class="text-xs text-text-muted mt-1">
          {{ progress }}% · {{ formatSize(downloadedBytes) }} / {{ formatSize(attachment.size) }}
        </div>
      </div>

      <!-- 错误提示 -->
      <div v-if="status === 'error'" class="text-xs text-red-500 mt-1">
        {{ errorMessage }}
      </div>
    </div>

    <!-- 操作按钮 -->
    <div
      class="flex-shrink-0 w-8 h-8 rounded-lg bg-bg-card flex items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity"
    >
      <Download v-if="status === 'not_downloaded'" :size="16" class="text-text-secondary" />
      <Loader2 v-if="status === 'downloading'" :size="16" class="text-accent animate-spin" />
      <Check v-if="status === 'downloaded'" :size="16" class="text-green-500" />
      <AlertCircle v-if="status === 'error'" :size="16" class="text-red-500" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { listen } from '@tauri-apps/api/event'
import type { Attachment } from '@/stores/mailbox'
import { downloadAttachment, saveAttachmentToPath } from '@/services/email/emailService'
import { useNotificationStore } from '@/stores/notification'
import { Download, File, FileImage, FileText, FileArchive, FileCode, Loader2, Check, AlertCircle } from 'lucide-vue-next'
import { save } from '@tauri-apps/plugin-dialog'

interface Props {
  attachment: Attachment
}

const props = defineProps<Props>()
const notificationStore = useNotificationStore()

// 状态
type AttachmentStatus = 'not_downloaded' | 'downloading' | 'downloaded' | 'error'
const status = ref<AttachmentStatus>(
  props.attachment.isDownloaded ? 'downloaded' : 'not_downloaded'
)
const progress = ref(0)
const downloadedBytes = ref(0)
const errorMessage = ref<string | null>(null)

// 离线检测
const isOffline = computed(() => !navigator.onLine)

// 文件图标
const fileIcon = computed(() => {
  const type = props.attachment.mimeType.toLowerCase()
  if (type.startsWith('image/')) return FileImage
  if (type.includes('pdf')) return FileText
  if (type.includes('zip') || type.includes('compressed') || type.includes('rar') || type.includes('7z')) return FileArchive
  if (type.includes('json') || type.includes('javascript') || type.includes('xml')) return FileCode
  return File
})

// 格式化大小
function formatSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`
  if (bytes < 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(1)} MB`
  return `${(bytes / (1024 * 1024 * 1024)).toFixed(1)} GB`
}

// 获取文件扩展名
function getExtension(filename: string): string {
  const parts = filename.split('.')
  if (parts.length > 1) {
    return parts.pop() || '*'
  }
  return '*'
}

// 进度事件监听
interface ProgressPayload {
  attachment_id: string
  downloaded: number
  total: number
  percent: number
}

interface CompletePayload {
  attachment_id: string
  local_path: string
}

let unlistenProgress: (() => void) | null = null
let unlistenComplete: (() => void) | null = null

onMounted(async () => {
  unlistenProgress = await listen<ProgressPayload>('attachment-progress', (event) => {
    if (event.payload.attachment_id === props.attachment.id) {
      progress.value = event.payload.percent
      downloadedBytes.value = event.payload.downloaded
    }
  })

  unlistenComplete = await listen<CompletePayload>('attachment-complete', (event) => {
    if (event.payload.attachment_id === props.attachment.id) {
      status.value = 'downloaded'
      progress.value = 100
      // 更新 attachment 状态
      props.attachment.isDownloaded = true
      props.attachment.localPath = event.payload.local_path
    }
  })
})

onUnmounted(() => {
  if (unlistenProgress) unlistenProgress()
  if (unlistenComplete) unlistenComplete()
})

// 点击处理
async function handleClick() {
  // 已下载 -> 保存到指定位置
  if (status.value === 'downloaded') {
    try {
      const targetPath = await save({
        defaultPath: props.attachment.filename,
        filters: [{ name: props.attachment.filename, extensions: [getExtension(props.attachment.filename)] }]
      })

      if (targetPath) {
        await saveAttachmentToPath(props.attachment.id, targetPath)
        notificationStore.success('保存成功', `附件已保存到 ${targetPath}`)
      }
    } catch (e) {
      notificationStore.error('保存失败', String(e))
    }
    return
  }

  // 离线 -> 提示
  if (isOffline.value) {
    notificationStore.warning('需要网络连接', '请连接网络后下载附件')
    return
  }

  // 正在下载 -> 不处理
  if (status.value === 'downloading') {
    return
  }

  // 未下载 -> 开始下载
  status.value = 'downloading'
  progress.value = 0
  downloadedBytes.value = 0
  errorMessage.value = null

  try {
    await downloadAttachment(props.attachment.id)
  } catch (e) {
    status.value = 'error'
    errorMessage.value = String(e)
    notificationStore.error('下载失败', String(e))
  }
}
</script>