<template>
  <div class="relative" ref="containerRef">
    <!-- Trigger Button -->
    <button
      class="w-full flex items-center gap-2 p-2 rounded-lg hover:bg-sidebar-hover transition-colors"
      @click="toggleDropdown"
    >
      <div class="avatar avatar-sm">
        {{ activeAccountInitial }}
      </div>
      <div v-if="!collapsed" class="flex-1 min-w-0 text-left">
        <div class="text-sm font-medium text-sidebar-text truncate">
          {{ activeAccountName }}
        </div>
        <div class="text-xs text-sidebar-text/60 truncate">
          {{ activeAccountEmail }}
        </div>
      </div>
      <ChevronDown
        v-if="!collapsed"
        :size="14"
        class="shrink-0 text-sidebar-text/40 transition-transform duration-200"
        :class="{ 'rotate-180': isOpen }"
      />
    </button>

    <!-- Dropdown Menu -->
    <Transition name="dropdown">
      <div
        v-if="isOpen"
        class="absolute z-50 mt-1 py-1 rounded-lg border border-border-primary bg-bg-primary shadow-lg glass-panel"
        :class="collapsed ? 'left-full ml-2 top-0 w-64' : 'left-0 right-0'"
      >
        <!-- Account List -->
        <template v-if="accounts.length > 0">
          <div
            v-if="accounts.length > 1"
            class="px-3 py-1.5 text-[0.625rem] font-medium text-text-tertiary uppercase tracking-wider"
          >
            账户
          </div>
          <button
            v-for="account in accounts"
            :key="account.id"
            class="flex items-center gap-2.5 w-full px-3 py-2 text-left transition-colors"
            :class="
              account.id === activeAccountId
                ? 'bg-accent/8 text-accent'
                : 'text-text-primary hover:bg-bg-hover'
            "
            @click="handleSelectAccount(account.id)"
          >
            <div
              class="w-7 h-7 rounded-full flex items-center justify-center shrink-0 text-xs font-semibold overflow-hidden"
              :class="
                account.id === activeAccountId
                  ? 'bg-accent text-white'
                  : 'bg-accent/12 text-accent'
              "
            >
              {{ getAccountInitial(account) }}
            </div>
            <div class="flex-1 min-w-0">
              <div class="text-sm font-medium truncate leading-tight">
                {{ account.name || account.email.split('@')[0] }}
              </div>
              <div class="text-xs text-text-secondary truncate leading-tight">
                {{ account.email }}
              </div>
            </div>
            <Check
              v-if="account.id === activeAccountId"
              :size="14"
              class="shrink-0 text-accent"
            />
          </button>
        </template>

        <!-- No Accounts -->
        <div
          v-else
          class="px-3 py-4 text-center"
        >
          <div class="text-sm text-text-secondary mb-2">
            暂无账户
          </div>
        </div>

        <!-- Divider -->
        <div class="border-t border-border-primary my-1" />

        <!-- Add Account Button -->
        <button
          class="flex items-center gap-2.5 w-full px-3 py-2 text-sm text-text-secondary hover:bg-bg-hover hover:text-text-primary transition-colors"
          @click="handleAddAccount"
        >
          <div class="w-7 h-7 rounded-full bg-bg-tertiary flex items-center justify-center shrink-0">
            <Plus :size="14" />
          </div>
          <span>添加账户</span>
        </button>
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { ChevronDown, Check, Plus } from 'lucide-vue-next'
import { useAccountStore, type Account } from '@/stores/account'

interface Props {
  collapsed?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  collapsed: false
})

const emit = defineEmits<{
  addAccount: []
}>()

// Use props in a computed to avoid TS6133
const collapsed = computed(() => props.collapsed)

const accountStore = useAccountStore()
const containerRef = ref<HTMLElement | null>(null)
const isOpen = ref(false)

const accounts = computed(() => accountStore.accounts)
const activeAccountId = computed(() => accountStore.activeAccountId)

const activeAccount = computed(() => {
  return accounts.value.find(a => a.id === activeAccountId.value)
})

const activeAccountName = computed(() => {
  return activeAccount.value?.name || '用户'
})

const activeAccountEmail = computed(() => {
  return activeAccount.value?.email || 'user@example.com'
})

const activeAccountInitial = computed(() => {
  return activeAccountName.value.charAt(0).toUpperCase()
})

function getAccountInitial(account: Account): string {
  const name = account.name || account.email
  return (name.charAt(0) || '?').toUpperCase()
}

function toggleDropdown() {
  isOpen.value = !isOpen.value
}

function closeDropdown() {
  isOpen.value = false
}

function handleSelectAccount(id: string) {
  accountStore.setActiveAccount(id)
  closeDropdown()
}

function handleAddAccount() {
  closeDropdown()
  emit('addAccount')
}

// Click outside handler
function handleClickOutside(event: MouseEvent) {
  if (containerRef.value && !containerRef.value.contains(event.target as Node)) {
    closeDropdown()
  }
}

onMounted(() => {
  document.addEventListener('click', handleClickOutside)
})

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
})
</script>

<style scoped>
.dropdown-enter-active,
.dropdown-leave-active {
  transition: all 0.15s ease;
}

.dropdown-enter-from,
.dropdown-leave-to {
  opacity: 0;
  transform: translateY(-4px);
}

.avatar {
  width: 2rem;
  height: 2rem;
  border-radius: 9999px;
  background-color: rgba(124, 58, 237, 0.15);
  color: #7c3aed;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  font-size: 0.875rem;
  font-weight: 600;
  overflow: hidden;
}
</style>