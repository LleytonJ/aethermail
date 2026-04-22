<template>
  <div class="address-input-wrapper">
    <span class="label">{{ label }}</span>
    <div class="address-input-container" ref="containerRef">
      <!-- 已添加地址标签 -->
      <span
        v-for="(addr, index) in addresses"
        :key="addr"
        class="address-tag"
      >
        <span class="address-text">{{ addr }}</span>
        <button
          class="address-remove"
          @click="removeAddress(index)"
          title="移除"
        >×</button>
      </span>

      <!-- 输入框 -->
      <input
        ref="inputRef"
        type="text"
        :value="inputValue"
        :placeholder="addresses.length === 0 ? placeholder : ''"
        class="address-input-field"
        @input="handleInput"
        @keydown="handleKeyDown"
        @blur="handleBlur"
      />

      <!-- 自动补全下拉列表 -->
      <div
        v-if="showSuggestions && suggestions.length > 0"
        class="suggestions-dropdown"
      >
        <button
          v-for="(contact, index) in suggestions"
          :key="contact.id"
          class="suggestion-item"
          :class="{ selected: index === selectedIndex }"
          @mousedown.prevent
          @click="selectSuggestion(contact)"
        >
          <span class="suggestion-name">
            {{ contact.displayName || contact.email }}
          </span>
          <span v-if="contact.displayName" class="suggestion-email">
            {{ contact.email }}
          </span>
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onUnmounted } from 'vue'
import { useContactsStore } from '@/stores/contacts'
import type { Contact } from '@/services/contact/contactService'

interface Props {
  label: string
  modelValue: string[]
  placeholder?: string
}

const props = withDefaults(defineProps<Props>(), {
  placeholder: '输入邮箱地址...'
})

const emit = defineEmits<{
  'update:modelValue': [value: string[]]
}>()

const contactsStore = useContactsStore()

// 状态
const inputValue = ref('')
const addresses = ref<string[]>([...props.modelValue])
const suggestions = ref<Contact[]>([])
const showSuggestions = ref(false)
const selectedIndex = ref(-1)
const inputRef = ref<HTMLInputElement | null>(null)
const containerRef = ref<HTMLDivElement | null>(null)

// 定时器
let searchTimer: ReturnType<typeof setTimeout> | null = null
let blurTimer: ReturnType<typeof setTimeout> | null = null

// 同步外部值
watch(() => props.modelValue, (newVal) => {
  addresses.value = [...newVal]
})

// 输入处理
function handleInput(e: Event) {
  const value = (e.target as HTMLInputElement).value
  inputValue.value = value

  // 清除之前的搜索定时器
  if (searchTimer) clearTimeout(searchTimer)

  if (value.length >= 2) {
    // 200ms debounce
    searchTimer = setTimeout(async () => {
      await contactsStore.searchContacts(value, 5)
      suggestions.value = contactsStore.searchResults
      showSuggestions.value = suggestions.value.length > 0
      selectedIndex.value = -1
    }, 200)
  } else {
    suggestions.value = []
    showSuggestions.value = false
  }
}

// 添加地址
function addAddress(addr: string) {
  const trimmed = addr.trim().toLowerCase()
  if (trimmed && !addresses.value.includes(trimmed)) {
    addresses.value = [...addresses.value, trimmed]
    emit('update:modelValue', addresses.value)
  }
  inputValue.value = ''
  suggestions.value = []
  showSuggestions.value = false
  inputRef.value?.focus()
}

// 移除地址
function removeAddress(index: number) {
  addresses.value = addresses.value.filter((_, i) => i !== index)
  emit('update:modelValue', addresses.value)
}

// 选择补全建议
function selectSuggestion(contact: Contact) {
  addAddress(contact.email)
}

// 键盘导航
function handleKeyDown(e: KeyboardEvent) {
  if (e.key === 'Enter' || e.key === 'Tab' || e.key === ',') {
    e.preventDefault()
    if (showSuggestions.value && selectedIndex.value >= 0) {
      addAddress(suggestions.value[selectedIndex.value].email)
    } else if (inputValue.value.trim()) {
      addAddress(inputValue.value)
    }
  } else if (e.key === 'Backspace' && !inputValue.value && addresses.value.length > 0) {
    removeAddress(addresses.value.length - 1)
  } else if (e.key === 'ArrowDown' && showSuggestions.value) {
    e.preventDefault()
    selectedIndex.value = Math.min(selectedIndex.value + 1, suggestions.value.length - 1)
  } else if (e.key === 'ArrowUp' && showSuggestions.value) {
    e.preventDefault()
    selectedIndex.value = Math.max(selectedIndex.value - 1, 0)
  } else if (e.key === 'Escape') {
    showSuggestions.value = false
  }
}

// 失焦处理
function handleBlur() {
  // 延迟关闭下拉框，以便点击建议项
  blurTimer = setTimeout(() => {
    showSuggestions.value = false
    if (inputValue.value.trim()) {
      addAddress(inputValue.value)
    }
  }, 150)
}

// 清理定时器
onUnmounted(() => {
  if (searchTimer) clearTimeout(searchTimer)
  if (blurTimer) clearTimeout(blurTimer)
})
</script>

<style scoped>
.address-input-wrapper {
  display: flex;
  align-items: flex-start;
  gap: 0.5rem;
  width: 100%;
}

.label {
  font-size: 0.75rem;
  color: var(--text-tertiary);
  padding-top: 0.375rem;
  min-width: 3rem;
  flex-shrink: 0;
}

.address-input-container {
  flex: 1;
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 0.25rem;
  min-height: 2rem;
  position: relative;
  padding: 0.25rem;
  background-color: var(--bg-tertiary);
  border: 1px solid var(--border-secondary);
  border-radius: 0.375rem;
}

.address-tag {
  display: inline-flex;
  align-items: center;
  gap: 0.25rem;
  background-color: var(--accent-light);
  color: var(--accent);
  font-size: 0.75rem;
  padding: 0.125rem 0.5rem;
  border-radius: 1rem;
}

.address-text {
  max-width: 200px;
  overflow: hidden;
  text-overflow: ellipsis;
}

.address-remove {
  background: none;
  border: none;
  color: var(--accent);
  font-size: 0.625rem;
  cursor: pointer;
  padding: 0;
  line-height: 1;
}

.address-remove:hover {
  color: var(--danger);
}

.address-input-field {
  flex: 1;
  min-width: 120px;
  background: transparent;
  border: none;
  color: var(--text-primary);
  font-size: 0.875rem;
  outline: none;
}

.address-input-field::placeholder {
  color: var(--text-tertiary);
}

.suggestions-dropdown {
  position: absolute;
  top: 100%;
  left: 0;
  right: 0;
  margin-top: 0.25rem;
  background-color: var(--bg-primary);
  border: 1px solid var(--border-primary);
  border-radius: 0.375rem;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  z-index: 50;
  padding: 0.25rem 0;
  max-height: 200px;
  overflow-y: auto;
}

.suggestion-item {
  display: flex;
  flex-direction: column;
  width: 100%;
  padding: 0.5rem 0.75rem;
  background: none;
  border: none;
  cursor: pointer;
  text-align: left;
}

.suggestion-item:hover,
.suggestion-item.selected {
  background-color: var(--bg-hover);
}

.suggestion-name {
  font-size: 0.875rem;
  color: var(--text-primary);
}

.suggestion-email {
  font-size: 0.75rem;
  color: var(--text-tertiary);
}
</style>