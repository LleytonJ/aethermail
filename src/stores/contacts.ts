import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import {
  searchContacts as searchContactsService,
  getAllContacts as getAllContactsService,
  getContactByEmail as getContactByEmailService,
  updateContact as updateContactService,
  deleteContact as deleteContactService,
  type Contact
} from '@/services/contact/contactService'

export const useContactsStore = defineStore('contacts', () => {
  // State
  const contacts = ref<Contact[]>([])
  const loading = ref(false)
  const searchResults = ref<Contact[]>([])

  // Actions

  // 加载所有联系人
  async function loadContacts() {
    loading.value = true
    try {
      contacts.value = await getAllContactsService()
    } catch (error) {
      console.error('加载联系人失败:', error)
      contacts.value = []
    } finally {
      loading.value = false
    }
  }

  // 搜索联系人（用于自动补全）
  async function searchContacts(query: string, limit = 10) {
    if (query.length < 2) {
      searchResults.value = []
      return
    }
    try {
      searchResults.value = await searchContactsService(query, limit)
    } catch (error) {
      console.error('搜索联系人失败:', error)
      searchResults.value = []
    }
  }

  // 清空搜索结果
  function clearSearchResults() {
    searchResults.value = []
  }

  // 根据邮箱获取联系人
  async function getContactByEmail(email: string): Promise<Contact | null> {
    try {
      return await getContactByEmailService(email)
    } catch (error) {
      console.error('获取联系人失败:', error)
      return null
    }
  }

  // 更新联系人
  async function updateContact(id: string, displayName: string | null, avatarUrl: string | null) {
    try {
      await updateContactService(id, displayName, avatarUrl)
      // 更新本地状态
      const contact = contacts.value.find(c => c.id === id)
      if (contact) {
        contact.displayName = displayName
        contact.avatarUrl = avatarUrl
      }
    } catch (error) {
      console.error('更新联系人失败:', error)
      throw error
    }
  }

  // 删除联系人
  async function deleteContact(id: string) {
    try {
      await deleteContactService(id)
      // 从本地状态移除
      contacts.value = contacts.value.filter(c => c.id !== id)
    } catch (error) {
      console.error('删除联系人失败:', error)
      throw error
    }
  }

  // Computed
  const contactCount = computed(() => contacts.value.length)

  return {
    // State
    contacts,
    loading,
    searchResults,
    // Computed
    contactCount,
    // Actions
    loadContacts,
    searchContacts,
    clearSearchResults,
    getContactByEmail,
    updateContact,
    deleteContact
  }
})