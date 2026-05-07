<template>
  <div class="email-body-container">
    <!-- HTML 邮件使用 iframe 完全隔离 -->
    <iframe
      v-if="html"
      ref="htmlFrame"
      class="email-iframe"
      :style="{ height: iframeHeight + 'px' }"
      sandbox="allow-same-origin allow-scripts"
      @load="onFrameLoad"
    ></iframe>

    <!-- 纯文本邮件直接显示 -->
    <div
      v-else-if="text"
      class="email-text-content p-5 whitespace-pre-wrap text-text-secondary text-sm leading-relaxed"
    >
      {{ text }}
    </div>

    <!-- 空状态 -->
    <div v-else class="text-text-muted text-center py-12">
      <Mail :size="32" class="mx-auto mb-3 opacity-30" />
      <p class="text-sm">暂无正文内容</p>
    </div>
  </div>

  <!-- 钓鱼邮件：禁止链接跳转弹窗 -->
  <ConfirmDialog
    v-if="isPhishing"
    :is-open="showLinkDialog"
    title="禁止链接跳转"
    :message="`此邮件被判定为钓鱼邮件，为保护您的安全，已阻止跳转以下链接：\n${pendingLinkUrl}`"
    confirm-text=""
    cancel-text="关闭"
    :danger="true"
    @confirm="handleCancelLink"
    @cancel="handleCancelLink"
  />

  <!-- 正常邮件：确认跳转链接弹窗 -->
  <ConfirmDialog
    v-if="!isPhishing"
    :is-open="showLinkDialog"
    title="确认跳转链接"
    :message="`是否确认跳转链接？\n${pendingLinkUrl}`"
    confirm-text="确认跳转"
    cancel-text="取消"
    @confirm="handleConfirmLink"
    @cancel="handleCancelLink"
  />
</template>

<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted, nextTick } from 'vue'
import { Mail } from 'lucide-vue-next'
import { openUrl } from '@tauri-apps/plugin-opener'
import ConfirmDialog from '@/components/ui/ConfirmDialog.vue'

interface Props {
  html?: string
  text?: string
  isPhishing?: boolean
}

const props = defineProps<Props>()

const htmlFrame = ref<HTMLIFrameElement | null>(null)
const showLinkDialog = ref(false)
const pendingLinkUrl = ref('')
const iframeHeight = ref(500) // 动态 iframe 高度，初始值增大

// 监听 iframe 内的链接点击
function setupLinkClickListener(doc: Document) {
  doc.addEventListener('click', (e: Event) => {
    const target = e.target as HTMLElement
    const anchor = target.closest('a')
    if (anchor?.href) {
      e.preventDefault()
      e.stopPropagation()
      pendingLinkUrl.value = anchor.href
      showLinkDialog.value = true
    }
  })
}

// 向 iframe 写入内容
function writeFrameContent() {
  if (!htmlFrame.value || !props.html) return

  const frame = htmlFrame.value
  const doc = frame.contentDocument || frame.contentWindow?.document

  if (doc) {
    // 直接使用原始 HTML（sandbox 环境下安全）
    // 只移除 script 标签防止 XSS
    const cleanHtml = props.html.replace(/<script[^>]*>[\s\S]*?<\/script>/gi, '')

    doc.open()
    doc.write(cleanHtml)
    doc.close()

    // 内容写入后设置链接点击监听
    setupLinkClickListener(doc)

    // 多次尝试更新高度，确保 DOM 完全渲染
    requestAnimationFrame(() => updateIframeHeight())
    setTimeout(() => updateIframeHeight(), 50)
    setTimeout(() => updateIframeHeight(), 200)
    setTimeout(() => updateIframeHeight(), 500)
  }
}

// 更新 iframe 高度以适应内容
function updateIframeHeight() {
  if (!htmlFrame.value) return

  const doc = htmlFrame.value.contentDocument
  if (!doc || !doc.body) return

  // 尝试多种方法获取高度，确保准确
  const bodyHeight = doc.body.scrollHeight
  const docHeight = doc.documentElement.scrollHeight
  const offsetHeight = doc.documentElement.offsetHeight

  // 取最大值，确保不截断内容
  const height = Math.max(bodyHeight, docHeight, offsetHeight)

  // 设置最小高度 500px，确保内容有足够空间显示
  iframeHeight.value = Math.max(500, height)
}

// ResizeObserver 用于监听 iframe 内容高度变化（如图片加载后）
let resizeObserver: ResizeObserver | null = null

function setupResizeObserver() {
  if (!htmlFrame.value) return

  const doc = htmlFrame.value.contentDocument
  if (!doc || !doc.body) return

  // 清理旧的 observer
  if (resizeObserver) {
    resizeObserver.disconnect()
  }

  resizeObserver = new ResizeObserver(() => {
    updateIframeHeight()
  })

  resizeObserver.observe(doc.body)

  // 监听图片加载完成后更新高度
  const images = doc.querySelectorAll('img')
  images.forEach(img => {
    if (!img.complete) {
      img.addEventListener('load', updateIframeHeight)
    }
  })
}

// iframe 加载完成后写入内容
function onFrameLoad() {
  writeFrameContent()
  setupResizeObserver()
}

// 监听 html prop 变化
watch(() => props.html, async () => {
  await nextTick()
  if (htmlFrame.value && props.html) {
    writeFrameContent()
    setupResizeObserver()
  }
})

// 组件挂载后，如果已经有 html 内容，等待 iframe 渲染后写入
onMounted(async () => {
  await nextTick()
  if (htmlFrame.value && props.html) {
    // iframe 的初始 load 事件会触发 onFrameLoad
    // 如果 iframe 已经加载但内容还没写入，手动触发
    const doc = htmlFrame.value.contentDocument
    if (doc && doc.body && doc.body.innerHTML === '') {
      writeFrameContent()
      setupResizeObserver()
    }
  }
})

// 组件卸载时清理 ResizeObserver
onUnmounted(() => {
  if (resizeObserver) {
    resizeObserver.disconnect()
    resizeObserver = null
  }
})

async function handleConfirmLink() {
  showLinkDialog.value = false
  try {
    await openUrl(pendingLinkUrl.value)
  } catch (err) {
    console.error('打开链接失败:', err)
  }
  pendingLinkUrl.value = ''
}

function handleCancelLink() {
  showLinkDialog.value = false
  pendingLinkUrl.value = ''
}
</script>

<style scoped>
.email-body-container {
  /* 不设置固定高度，让内容自然撑开，父容器处理滚动 */
}

.email-iframe {
  width: 100%;
  border: none;
  background: #f1f5f9;
  display: block;
}

.email-text-content {
  color: var(--text-secondary);
}
</style>