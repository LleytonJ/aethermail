<template>
  <div class="space-y-4">
    <h3 class="text-lg font-medium text-text-primary">附件缓存</h3>

    <!-- 缓存位置 -->
    <div class="space-y-2">
      <label class="text-sm text-text-secondary">缓存位置</label>
      <div class="flex items-center gap-2">
        <div class="flex-1 text-sm text-text-muted truncate">
          {{ cacheStats?.cache_path || '加载中...' }}
        </div>
        <Button variant="secondary" size="sm" @click="openDirectory">
          打开目录
        </Button>
      </div>
    </div>

    <!-- 缓存统计 -->
    <div class="space-y-2">
      <label class="text-sm text-text-secondary">缓存统计</label>
      <div class="flex gap-4">
        <div class="text-sm text-text-muted">
          已缓存附件: <span class="text-text-primary">{{ cacheStats?.file_count || 0 }}</span> 个
        </div>
        <div class="text-sm text-text-muted">
          占用空间: <span class="text-text-primary">{{ formatSize(cacheStats?.total_size || 0) }}</span>
        </div>
      </div>
    </div>

    <!-- 清空缓存按钮 -->
    <div class="space-y-2">
      <Button
        variant="secondary"
        :disabled="cacheStats?.file_count === 0 || clearing"
        @click="showConfirmDialog = true"
      >
        <Loader2 v-if="clearing" :size="16" class="animate-spin mr-2" />
        清空缓存
      </Button>
      <p class="text-xs text-text-muted">
        清空缓存后，附件需要重新从服务器下载
      </p>
    </div>

    <!-- 确认对话框 -->
    <Modal
      :is-open="showConfirmDialog"
      title="确认清空缓存"
      @close="showConfirmDialog = false"
    >
      <div class="space-y-4">
        <p class="text-text-secondary">
          将删除 {{ cacheStats?.file_count || 0 }} 个缓存文件，共 {{ formatSize(cacheStats?.total_size || 0) }}
        </p>
        <p class="text-sm text-text-muted">
          清空后附件需要重新从服务器下载
        </p>
        <div class="flex gap-3 justify-end">
          <Button variant="secondary" @click="showConfirmDialog = false">
            取消
          </Button>
          <Button variant="primary" @click="clearCache">
            确认清空
          </Button>
        </div>
      </div>
    </Modal>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { getCacheStats, clearCache as clearCacheService, openCacheDirectory } from '@/services/email/emailService'
import { useNotificationStore } from '@/stores/notification'
import Button from '@/components/ui/Button.vue'
import Modal from '@/components/ui/Modal.vue'
import { Loader2 } from 'lucide-vue-next'

interface CacheStats {
  file_count: number
  total_size: number
  cache_path: string
}

const notificationStore = useNotificationStore()
const cacheStats = ref<CacheStats | null>(null)
const clearing = ref(false)
const showConfirmDialog = ref(false)

// 格式化大小
function formatSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`
  if (bytes < 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(1)} MB`
  return `${(bytes / (1024 * 1024 * 1024)).toFixed(1)} GB`
}

// 加载缓存统计
async function loadStats() {
  try {
    cacheStats.value = await getCacheStats()
  } catch (e) {
    console.error('加载缓存统计失败:', e)
  }
}

// 打开缓存目录
async function openDirectory() {
  try {
    await openCacheDirectory()
  } catch (e) {
    notificationStore.error('打开目录失败', String(e))
  }
}

// 清空缓存
async function clearCache() {
  clearing.value = true
  showConfirmDialog.value = false

  try {
    await clearCacheService()
    notificationStore.success('缓存已清空', '所有附件缓存已删除')
    await loadStats()
  } catch (e) {
    notificationStore.error('清空缓存失败', String(e))
  } finally {
    clearing.value = false
  }
}

onMounted(() => {
  loadStats()
})
</script>