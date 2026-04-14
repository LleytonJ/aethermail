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

// 邮件内容的内联样式（不影响外部）
const emailStyles = `
  body {
    font-family: "Inter", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
    color: #cbd5e1;
    font-size: 14px;
    line-height: 1.7;
    background: transparent;
    margin: 0;
    padding: 20px;
  }
  p { margin: 0 0 1em 0; }
  p:last-child { margin-bottom: 0; }
  a { color: #818cf8; text-decoration: none; }
  a:hover { text-decoration: underline; }
  img { max-width: 100%; height: auto; border-radius: 8px; margin: 1em 0; }
  blockquote {
    border-left: 3px solid rgba(255, 255, 255, 0.1);
    padding-left: 1rem;
    margin: 1em 0;
    color: #64748b;
    font-style: italic;
  }
  pre {
    background: rgba(51, 65, 85, 0.75);
    padding: 1rem;
    border-radius: 8px;
    overflow-x: auto;
    margin: 1em 0;
  }
  code {
    font-family: 'JetBrains Mono', 'Fira Code', 'Consolas', monospace;
    font-size: 0.9em;
    background: rgba(51, 65, 85, 0.75);
    padding: 0.2em 0.4em;
    border-radius: 4px;
  }
  pre code { background: none; padding: 0; }
  ul, ol { padding-left: 1.5rem; margin: 1em 0; }
  li { margin: 0.25em 0; }
  h1, h2, h3, h4 { color: #f8fafc; font-weight: 600; margin: 1.5em 0 0.5em 0; }
  table { width: 100%; border-collapse: collapse; margin: 1em 0; }
  th, td { border: 1px solid rgba(255, 255, 255, 0.06); padding: 0.5rem 0.75rem; text-align: left; }
  th { background: rgba(51, 65, 85, 0.75); font-weight: 600; }
  hr { border: none; border-top: 1px solid rgba(255, 255, 255, 0.06); margin: 2em 0; }
`

// 清理 HTML 内容，移除可能导致问题的元素
function sanitizeEmailHtml(html: string): string {
  return DOMPurify.sanitize(html, {
    USE_PROFILES: { html: true },
    ALLOWED_TAGS: [
      'p', 'br', 'strong', 'em', 'u', 's', 'a', 'img',
      'div', 'span',
      'h1', 'h2', 'h3', 'h4', 'h5', 'h6',
      'ul', 'ol', 'li',
      'table', 'thead', 'tbody', 'tr', 'th', 'td',
      'blockquote', 'pre', 'code', 'hr'
    ],
    // 禁止所有可能导致样式冲突的属性
    FORBID_ATTR: ['style', 'class', 'id'],
    ALLOWED_ATTR: ['href', 'src', 'alt', 'title', 'width', 'height', 'target'],
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