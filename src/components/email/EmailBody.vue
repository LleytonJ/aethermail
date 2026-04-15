<template>
  <div class="email-body-container">
    <!-- HTML 邮件使用 iframe 完全隔离 -->
    <iframe
      v-if="html"
      ref="htmlFrame"
      class="email-iframe"
      sandbox="allow-same-origin"
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
import { ref, watch, onMounted, nextTick } from 'vue'
import DOMPurify from 'dompurify'
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

// 邮件内容的基础样式（不覆盖邮件自有样式）
const emailStyles = `
  body {
    font-family: "Inter", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
    font-size: 14px;
    line-height: 1.6;
    background: transparent;
    margin: 0;
    padding: 16px;
    word-wrap: break-word;
    overflow-wrap: break-word;
    color: #cbd5e1;
  }
  /* 只设置图片和表格的基础约束，避免溢出 */
  img { max-width: 100%; height: auto; }
  table { max-width: 100%; }
  pre { overflow-x: auto; }
`

// 清理 HTML 内容，移除危险元素但保留样式
function sanitizeEmailHtml(html: string): string {
  return DOMPurify.sanitize(html, {
    ALLOW_UNKNOWN_PROTOCOLS: false,
    // 只禁止危险标签，保留样式相关标签
    FORBID_TAGS: ['script', 'style', 'iframe', 'object', 'embed', 'form', 'input', 'button'],
    // 允许样式属性，让邮件正常显示
    ALLOWED_ATTR: [
      'href', 'src', 'alt', 'title', 'width', 'height', 'class', 'style',
      'target', 'rel', 'colspan', 'rowspan', 'cellpadding', 'cellspacing',
      'border', 'align', 'valign', 'bgcolor', 'color', 'dir', 'lang',
      'background', 'data-*'
    ],
    FORCE_BODY: true
  })
}

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
    const sanitizedHtml = sanitizeEmailHtml(props.html)

    doc.open()
    doc.write(`
      <!DOCTYPE html>
      <html>
        <head>
          <meta charset="UTF-8">
          <style>${emailStyles}</style>
        </head>
        <body>
          ${sanitizedHtml}
        </body>
      </html>
    `)
    doc.close()

    // 内容写入后设置链接点击监听
    setupLinkClickListener(doc)
  }
}

// iframe 加载完成后写入内容
function onFrameLoad() {
  writeFrameContent()
}

// 监听 html prop 变化
watch(() => props.html, async () => {
  await nextTick()
  if (htmlFrame.value && props.html) {
    writeFrameContent()
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
    }
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
  height: 100%;
  display: flex;
  flex-direction: column;
}

.email-iframe {
  width: 100%;
  flex: 1;
  min-height: 200px;
  border: none;
  background: transparent;
  display: block;
}

.email-text-content {
  color: var(--text-secondary);
  flex: 1;
  overflow-y: auto;
}
</style>