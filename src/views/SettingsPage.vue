<template>
  <div class="flex-1 flex flex-col min-w-0 overflow-hidden bg-bg-primary/50">
    <!-- Header -->
    <div class="flex items-center gap-3 px-5 py-3 border-b border-border-primary shrink-0 bg-bg-primary/60 backdrop-blur-sm">
      <button
        class="p-1.5 -ml-1 rounded-md text-text-secondary hover:text-text-primary hover:bg-bg-hover transition-colors"
        title="返回收件箱"
        @click="handleGoBack"
      >
        <ArrowLeft :size="18" />
      </button>
      <h1 class="text-base font-semibold text-text-primary">设置</h1>
    </div>

    <!-- Body: sidebar nav + content -->
    <div class="flex flex-1 min-h-0">
      <!-- Vertical tab sidebar (w-48) -->
      <nav class="w-48 border-r border-border-primary py-2 overflow-y-auto shrink-0 bg-bg-primary/30">
        <button
          v-for="tab in tabs"
          :key="tab.id"
          :class="[
            'flex items-center gap-2.5 w-full px-4 py-2 text-[0.8125rem] transition-colors',
            activeTab === tab.id
              ? 'bg-bg-selected text-accent font-medium'
              : 'text-text-secondary hover:bg-bg-hover hover:text-text-primary'
          ]"
          @click="handleTabClick(tab.id)"
        >
          <component :is="tab.icon" :size="15" class="shrink-0" />
          {{ tab.label }}
        </button>
      </nav>

      <!-- Scrollable content (max-w-2xl) -->
      <div class="flex-1 overflow-y-auto">
        <div class="max-w-2xl px-8 py-6">
          <!-- Tab title -->
          <div class="mb-6">
            <h2 class="text-lg font-semibold text-text-primary">
              {{ activeTabDef?.label }}
            </h2>
          </div>

          <!-- Content -->
          <router-view @add-account="showAddAccount = true" />
        </div>
      </div>
    </div>

    <!-- Add Account Modal -->
    <AddAccount
      :is-open="showAddAccount"
      @close="showAddAccount = false"
      @success="handleAccountAdded"
    />

    <!-- Notifications -->
    <Notifications />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { ArrowLeft, UserCircle, Settings, Brain } from 'lucide-vue-next'
import type { LucideIcon } from 'lucide-vue-next'
import { useAccountStore } from '@/stores/account'
import AddAccount from '@/components/accounts/AddAccount.vue'
import Notifications from '@/components/common/Notifications.vue'

type SettingsTab = 'accounts' | 'general' | 'ai'

interface Tab {
  id: SettingsTab
  label: string
  icon: LucideIcon
}

const tabs: Tab[] = [
  { id: 'accounts', label: '账户管理', icon: UserCircle },
  { id: 'general', label: '通用设置', icon: Settings },
  { id: 'ai', label: 'AI 设置', icon: Brain }
]

const router = useRouter()
const route = useRoute()
const accountStore = useAccountStore()

const showAddAccount = ref(false)
const activeTab = ref<SettingsTab>('accounts')

// 监听路由变化更新 activeTab
watch(
  () => route.path,
  (path) => {
    if (path.includes('/settings/general')) {
      activeTab.value = 'general'
    } else if (path.includes('/settings/ai')) {
      activeTab.value = 'ai'
    } else {
      activeTab.value = 'accounts'
    }
  },
  { immediate: true }
)

const activeTabDef = computed(() => {
  return tabs.find(t => t.id === activeTab.value)
})

function handleGoBack() {
  router.push('/')
}

function handleTabClick(tabId: SettingsTab) {
  router.push(`/settings/${tabId}`)
}

async function handleAccountAdded() {
  showAddAccount.value = false
  await accountStore.init()
}
</script>