<template>
  <div ref="contentRef" class="p-5">
    <div
      v-if="html"
      class="email-content prose prose-invert max-w-none"
      v-html="sanitizedHtml"
      @click="handleClick"
    ></div>
    <div
      v-else-if="text"
      class="email-content prose prose-invert max-w-none whitespace-pre-wrap"
    >
      {{ text }}
    </div>
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
import { ref, computed } from 'vue'
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

const contentRef = ref<HTMLElement | null>(null)
const showLinkDialog = ref(false)
const pendingLinkUrl = ref('')

const sanitizedHtml = computed(() => {
  if (!props.html) return ''
  return DOMPurify.sanitize(props.html, {
    USE_PROFILES: { html: true },
    ALLOWED_TAGS: [
      'p',
      'br',
      'strong',
      'em',
      'u',
      's',
      'a',
      'img',
      'div',
      'span',
      'h1',
      'h2',
      'h3',
      'h4',
      'h5',
      'h6',
      'ul',
      'ol',
      'li',
      'table',
      'thead',
      'tbody',
      'tr',
      'th',
      'td',
      'blockquote',
      'pre',
      'code',
      'hr'
    ],
    ALLOWED_ATTR: ['href', 'src', 'alt', 'title', 'style', 'class', 'id', 'width', 'height']
  })
})

function handleClick(e: MouseEvent) {
  const target = e.target as HTMLElement
  const anchor = target.closest('a')
  if (anchor?.href) {
    e.preventDefault()
    e.stopPropagation()
    pendingLinkUrl.value = anchor.href
    showLinkDialog.value = true
  }
}

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
.email-content {
  color: var(--text-secondary);
  font-size: 14px;
  line-height: 1.7;
}

.email-content :deep(p) {
  margin: 0 0 1em 0;
}

.email-content :deep(p:last-child) {
  margin-bottom: 0;
}

.email-content :deep(a) {
  color: var(--accent);
  text-decoration: none;
}

.email-content :deep(a:hover) {
  text-decoration: underline;
}

.email-content :deep(img) {
  max-width: 100%;
  height: auto;
  border-radius: 8px;
  margin: 1em 0;
}

.email-content :deep(blockquote) {
  border-left: 3px solid var(--border-primary);
  padding-left: 1rem;
  margin: 1em 0;
  color: var(--text-muted);
  font-style: italic;
}

.email-content :deep(pre) {
  background: var(--bg-tertiary);
  padding: 1rem;
  border-radius: 8px;
  overflow-x: auto;
  margin: 1em 0;
}

.email-content :deep(code) {
  font-family: 'JetBrains Mono', 'Fira Code', 'Consolas', monospace;
  font-size: 0.9em;
  background: var(--bg-tertiary);
  padding: 0.2em 0.4em;
  border-radius: 4px;
}

.email-content :deep(pre code) {
  background: none;
  padding: 0;
}

.email-content :deep(ul),
.email-content :deep(ol) {
  padding-left: 1.5rem;
  margin: 1em 0;
}

.email-content :deep(li) {
  margin: 0.25em 0;
}

.email-content :deep(h1),
.email-content :deep(h2),
.email-content :deep(h3),
.email-content :deep(h4) {
  color: var(--text-primary);
  font-weight: 600;
  margin: 1.5em 0 0.5em 0;
}

.email-content :deep(table) {
  width: 100%;
  border-collapse: collapse;
  margin: 1em 0;
}

.email-content :deep(th),
.email-content :deep(td) {
  border: 1px solid var(--border-secondary);
  padding: 0.5rem 0.75rem;
  text-align: left;
}

.email-content :deep(th) {
  background: var(--bg-tertiary);
  font-weight: 600;
}

.email-content :deep(hr) {
  border: none;
  border-top: 1px solid var(--border-secondary);
  margin: 2em 0;
}
</style>