<template>
  <div class="space-y-4">
    <!-- Account List -->
    <div v-if="accounts.length > 0" class="space-y-3">
      <div
        v-for="account in accounts"
        :key="account.id"
        class="flex items-center justify-between py-2.5 px-4 bg-bg-secondary rounded-lg border border-border-primary"
      >
        <!-- Account Info -->
        <div class="flex items-center gap-3 min-w-0">
          <!-- Avatar -->
          <div class="w-9 h-9 rounded-full bg-accent/15 text-accent flex items-center justify-center shrink-0 text-sm font-semibold">
            {{ getAccountInitial(account) }}
          </div>
          <!-- Details -->
          <div class="min-w-0">
            <div class="text-sm font-medium text-text-primary truncate flex items-center gap-2">
              {{ account.name || account.email.split('@')[0] }}
              <span
                v-if="account.id === activeAccountId"
                class="text-[0.625rem] font-medium px-1.5 py-0.5 rounded-full bg-accent/15 text-accent"
              >
                当前
              </span>
              <span class="text-[0.6rem] font-medium px-1.5 py-0.5 rounded-full bg-bg-tertiary text-text-tertiary">
                POP3
              </span>
            </div>
            <div class="text-xs text-text-tertiary truncate">
              {{ account.email }}
            </div>
            <div class="text-xs text-text-tertiary truncate mt-0.5">
              {{ account.incomingServer }}:{{ account.incomingPort }}
            </div>
          </div>
        </div>

        <!-- Actions -->
        <div class="flex items-center gap-2 shrink-0">
          <button
            class="px-2.5 py-1.5 text-xs text-accent hover:text-accent-hover transition-colors"
            @click="handleEdit(account)"
          >
            编辑
          </button>
          <button
            class="px-2.5 py-1.5 text-xs text-danger hover:text-danger-hover transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
            :disabled="accounts.length === 1"
            :title="accounts.length === 1 ? '无法删除唯一的账户' : ''"
            @click="handleDelete(account)"
          >
            删除
          </button>
        </div>
      </div>
    </div>

    <!-- No Accounts -->
    <div v-else class="text-center py-8">
      <div class="text-sm text-text-secondary mb-2">
        暂无账户
      </div>
      <button
        class="px-4 py-2 text-sm bg-accent text-white rounded-lg hover:bg-accent-hover transition-colors"
        @click="handleAddAccount"
      >
        添加账户
      </button>
    </div>

    <!-- Add Account Button -->
    <div v-if="accounts.length > 0" class="pt-4 border-t border-border-primary">
      <button
        class="flex items-center gap-2 px-4 py-2 text-sm text-accent hover:text-accent-hover transition-colors"
        @click="handleAddAccount"
      >
        <Plus :size="16" />
        添加新账户
      </button>
    </div>

    <!-- Confirm Dialog -->
    <ConfirmDialog
      :is-open="showDeleteConfirm"
      :title="deleteDialogTitle"
      :message="deleteDialogMessage"
      confirm-text="删除"
      cancel-text="取消"
      :danger="true"
      @confirm="confirmDelete"
      @cancel="cancelDelete"
    />

    <!-- Edit Account Modal -->
    <EditAccount
      :is-open="showEditModal"
      :account="editingAccount"
      @close="closeEditModal"
      @saved="handleAccountSaved"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { Plus } from 'lucide-vue-next'
import { useAccountStore, type Account } from '@/stores/account'
import { useNotificationStore } from '@/stores/notification'
import { removeMockAccount } from '@/services/mock/accountService'
import ConfirmDialog from '@/components/ui/ConfirmDialog.vue'
import EditAccount from './EditAccount.vue'

const emit = defineEmits<{
  addAccount: []
}>()

const accountStore = useAccountStore()
const notificationStore = useNotificationStore()

const accounts = computed(() => accountStore.accounts)
const activeAccountId = computed(() => accountStore.activeAccountId)

// Delete state
const showDeleteConfirm = ref(false)
const deletingAccount = ref<Account | null>(null)

const deleteDialogTitle = computed(() => {
  if (!deletingAccount.value) return '确认删除'
  return `删除账户 "${deletingAccount.value.email}"`
})

const deleteDialogMessage = computed(() => {
  if (!deletingAccount.value) return ''
  if (accounts.value.length === 1) {
    return '这是最后一个账户，删除后将无法使用本应用。'
  }
  if (deletingAccount.value.id === activeAccountId.value) {
    return '此账户当前正在使用，删除后将切换到其他账户。确定要删除吗？'
  }
  return '确定要删除此账户吗？此操作无法撤销。'
})

// Edit state
const showEditModal = ref(false)
const editingAccount = ref<Account | null>(null)

function getAccountInitial(account: Account): string {
  const name = account.name || account.email
  return (name.charAt(0) || '?').toUpperCase()
}

function handleAddAccount() {
  emit('addAccount')
}

function handleEdit(account: Account) {
  editingAccount.value = account
  showEditModal.value = true
}

function handleDelete(account: Account) {
  if (accounts.value.length === 1) return
  deletingAccount.value = account
  showDeleteConfirm.value = true
}

async function confirmDelete() {
  if (!deletingAccount.value) return

  try {
    // Remove from mock service
    await removeMockAccount(deletingAccount.value.id)
    // Remove from store
    accountStore.removeAccount(deletingAccount.value.id)

    notificationStore.success('账户已删除')
  } catch (e) {
    console.error('Failed to delete account:', e)
    notificationStore.error('删除失败')
  }

  showDeleteConfirm.value = false
  deletingAccount.value = null
}

function cancelDelete() {
  showDeleteConfirm.value = false
  deletingAccount.value = null
}

function closeEditModal() {
  showEditModal.value = false
  editingAccount.value = null
}

function handleAccountSaved() {
  notificationStore.success('账户已更新')
  closeEditModal()
}
</script>