<template>
  <div class="contact-editor space-y-3">
    <!-- 搜索框 -->
    <div class="search-wrapper">
      <Search :size="14" class="search-icon" />
      <input
        type="text"
        :value="searchQuery"
        placeholder="搜索联系人..."
        class="search-input"
        @input="searchQuery = ($event.target as HTMLInputElement).value"
      />
    </div>

    <!-- 联系人列表 -->
    <div v-if="filteredContacts.length > 0" class="contact-list">
      <div
        v-for="contact in filteredContacts"
        :key="contact.id"
        class="contact-item"
      >
        <!-- 编辑模式 -->
        <div v-if="editingId === contact.id" class="edit-form">
          <input
            type="text"
            :value="editName"
            placeholder="显示名称"
            class="edit-input"
            @input="editName = ($event.target as HTMLInputElement).value"
            @keydown.enter="saveEdit(contact.id)"
            @keydown.escape="cancelEdit"
            autofocus
          />
          <button class="save-btn" @click="saveEdit(contact.id)">
            <Check :size="14" />
          </button>
          <button class="cancel-btn" @click="cancelEdit">
            <X :size="14" />
          </button>
        </div>

        <!-- 显示模式 -->
        <template v-else>
          <div class="contact-info">
            <div class="contact-name">
              {{ contact.displayName || contact.email }}
            </div>
            <div v-if="contact.displayName" class="contact-email">
              {{ contact.email }}
            </div>
          </div>
          <div class="contact-actions">
            <span class="frequency">{{ contact.frequency }}次</span>
            <button
              class="edit-btn"
              title="编辑名称"
              @click="startEdit(contact)"
            >
              <Pencil :size="13" />
            </button>
            <button
              class="delete-btn"
              title="删除联系人"
              @click="handleDelete(contact)"
            >
              <Trash2 :size="13" />
            </button>
          </div>
        </template>
      </div>
    </div>

    <!-- 空状态 -->
    <div v-else class="empty-state">
      <p class="empty-text">
        {{ searchQuery ? '未找到匹配联系人' : '暂无联系人' }}
      </p>
    </div>

    <!-- 统计 -->
    <p class="contact-count">
      共 {{ contacts.length }} 个联系人
    </p>

    <!-- 删除确认对话框 -->
    <ConfirmDialog
      :is-open="showDeleteConfirm"
      title="删除联系人"
      message="确定要删除该联系人吗？此操作不可撤销。"
      confirm-text="删除"
      cancel-text="取消"
      @confirm="confirmDelete"
      @cancel="showDeleteConfirm = false"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { Search, Pencil, Trash2, Check, X } from 'lucide-vue-next'
import { useContactsStore } from '@/stores/contacts'
import type { Contact } from '@/services/contact/contactService'
import ConfirmDialog from '@/components/ui/ConfirmDialog.vue'

const contactsStore = useContactsStore()

// 状态
const searchQuery = ref('')
const editingId = ref<string | null>(null)
const editName = ref('')
const showDeleteConfirm = ref(false)
const deleteTargetId = ref<string | null>(null)

// 联系人列表
const contacts = computed(() => contactsStore.contacts)

// 过滤后的联系人
const filteredContacts = computed(() => {
  if (!searchQuery.value) return contacts.value
  const q = searchQuery.value.toLowerCase()
  return contacts.value.filter(c =>
    c.email.toLowerCase().includes(q) ||
    (c.displayName && c.displayName.toLowerCase().includes(q))
  )
})

// 加载联系人
onMounted(async () => {
  await contactsStore.loadContacts()
})

// 开始编辑
function startEdit(contact: Contact) {
  editingId.value = contact.id
  editName.value = contact.displayName || ''
}

// 保存编辑
async function saveEdit(id: string) {
  if (!editingId.value) return
  try {
    await contactsStore.updateContact(id, editName.value || null, null)
    editingId.value = null
  } catch (error) {
    console.error('保存失败:', error)
  }
}

// 取消编辑
function cancelEdit() {
  editingId.value = null
}

// 处理删除
function handleDelete(contact: Contact) {
  deleteTargetId.value = contact.id
  showDeleteConfirm.value = true
}

// 确认删除
async function confirmDelete() {
  if (!deleteTargetId.value) return
  try {
    await contactsStore.deleteContact(deleteTargetId.value)
  } catch (error) {
    console.error('删除失败:', error)
  }
  showDeleteConfirm.value = false
  deleteTargetId.value = null
}
</script>

<style scoped>
.contact-editor {
  width: 100%;
}

.search-wrapper {
  position: relative;
  width: 100%;
}

.search-icon {
  position: absolute;
  left: 0.625rem;
  top: 50%;
  transform: translateY(-50%);
  color: var(--text-tertiary);
}

.search-input {
  width: 100%;
  padding: 0.5rem 0.75rem 0.5rem 2rem;
  background-color: var(--bg-tertiary);
  border: 1px solid var(--border-primary);
  border-radius: 0.375rem;
  color: var(--text-primary);
  font-size: 0.875rem;
  outline: none;
}

.search-input:focus {
  border-color: var(--accent);
}

.contact-list {
  max-height: 300px;
  overflow-y: auto;
}

.contact-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0.5rem 0.75rem;
  border-radius: 0.25rem;
}

.contact-item:hover {
  background-color: var(--bg-hover);
}

.contact-info {
  flex: 1;
  min-width: 0;
}

.contact-name {
  font-size: 0.875rem;
  color: var(--text-primary);
  overflow: hidden;
  text-overflow: ellipsis;
}

.contact-email {
  font-size: 0.75rem;
  color: var(--text-tertiary);
  overflow: hidden;
  text-overflow: ellipsis;
}

.contact-actions {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.frequency {
  font-size: 0.75rem;
  color: var(--text-tertiary);
  margin-right: 0.5rem;
}

.edit-btn,
.delete-btn {
  background: none;
  border: none;
  padding: 0.25rem;
  cursor: pointer;
  color: var(--text-tertiary);
  opacity: 0;
  transition: opacity 0.15s, color 0.15s;
}

.contact-item:hover .edit-btn,
.contact-item:hover .delete-btn {
  opacity: 1;
}

.edit-btn:hover {
  color: var(--text-primary);
}

.delete-btn:hover {
  color: var(--danger);
}

.edit-form {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  flex: 1;
}

.edit-input {
  flex: 1;
  min-width: 0;
  padding: 0.375rem 0.5rem;
  background-color: var(--bg-tertiary);
  border: 1px solid var(--border-primary);
  border-radius: 0.25rem;
  color: var(--text-primary);
  font-size: 0.875rem;
  outline: none;
}

.edit-input:focus {
  border-color: var(--accent);
}

.save-btn {
  background: none;
  border: none;
  padding: 0.25rem;
  cursor: pointer;
  color: var(--success);
}

.cancel-btn {
  background: none;
  border: none;
  padding: 0.25rem;
  cursor: pointer;
  color: var(--text-tertiary);
}

.cancel-btn:hover {
  color: var(--text-primary);
}

.empty-state {
  text-align: center;
  padding: 1rem 0;
}

.empty-text {
  font-size: 0.875rem;
  color: var(--text-tertiary);
}

.contact-count {
  font-size: 0.75rem;
  color: var(--text-tertiary);
  margin-top: 0.5rem;
}
</style>