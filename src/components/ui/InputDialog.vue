<template>
  <Teleport to="body">
    <Transition name="dialog">
      <div
        v-if="isOpen"
        class="fixed inset-0 z-50 flex items-center justify-center p-4"
      >
        <!-- Backdrop -->
        <div
          class="absolute inset-0 bg-black/70"
          @click="handleBackdropClick"
        />

        <!-- Dialog Content -->
        <div
          ref="dialogRef"
          class="relative bg-bg-primary border border-border-primary rounded-xl shadow-2xl w-full max-w-sm overflow-hidden"
          @keydown.esc="handleEscape"
        >
          <!-- Header -->
          <div class="flex items-center justify-between px-4 py-3 border-b border-border-primary">
            <h2 class="text-base font-semibold text-text-primary">
              {{ title }}
            </h2>
            <button
              class="p-1 rounded-md text-text-tertiary hover:text-text-primary hover:bg-bg-hover transition-colors"
              @click="handleCancel"
            >
              <X :size="18" />
            </button>
          </div>

          <!-- Body -->
          <div class="px-4 py-4">
            <input
              ref="inputRef"
              v-model="inputValue"
              type="text"
              :placeholder="placeholder"
              class="w-full px-3 py-2 bg-bg-secondary border border-border-primary rounded-lg text-sm text-text-primary outline-none focus:border-accent transition-colors"
              @keydown.enter="handleConfirm"
            />
            <p v-if="error" class="text-xs text-danger mt-2">
              {{ error }}
            </p>
          </div>

          <!-- Footer -->
          <div class="flex items-center justify-end gap-2 px-4 py-3 border-t border-border-primary bg-bg-secondary/30">
            <button
              class="px-3 py-1.5 text-sm text-text-secondary hover:text-text-primary bg-bg-secondary border border-border-primary rounded-lg transition-colors"
              @click="handleCancel"
            >
              {{ cancelText }}
            </button>
            <button
              class="px-3 py-1.5 text-sm text-white bg-accent hover:bg-accent-hover rounded-lg transition-colors"
              @click="handleConfirm"
            >
              {{ confirmText }}
            </button>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, watch, onUnmounted, nextTick } from 'vue'
import { X } from 'lucide-vue-next'

interface Props {
  isOpen: boolean
  title?: string
  placeholder?: string
  confirmText?: string
  cancelText?: string
  initialValue?: string
  closeOnBackdrop?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  title: '输入',
  placeholder: '',
  confirmText: '确认',
  cancelText: '取消',
  initialValue: '',
  closeOnBackdrop: true
})

const emit = defineEmits<{
  confirm: [value: string]
  cancel: []
}>()

const dialogRef = ref<HTMLElement | null>(null)
const inputRef = ref<HTMLInputElement | null>(null)
const inputValue = ref(props.initialValue)
const error = ref('')

// Reset value when dialog opens
watch(
  () => props.isOpen,
  (isOpen) => {
    if (isOpen) {
      inputValue.value = props.initialValue
      error.value = ''
      document.body.style.overflow = 'hidden'
      // Focus input after animation
      nextTick(() => {
        inputRef.value?.focus()
      })
    } else {
      document.body.style.overflow = ''
    }
  }
)

function handleConfirm() {
  emit('confirm', inputValue.value)
}

function handleCancel() {
  emit('cancel')
}

function handleBackdropClick() {
  if (props.closeOnBackdrop) {
    handleCancel()
  }
}

function handleEscape() {
  handleCancel()
}

// Clean up on unmount
onUnmounted(() => {
  document.body.style.overflow = ''
})

// Expose for parent to set error
defineExpose({
  setError: (msg: string) => {
    error.value = msg
  },
  clearError: () => {
    error.value = ''
  }
})
</script>

<style scoped>
.dialog-enter-active,
.dialog-leave-active {
  transition: all 0.2s ease;
}

.dialog-enter-active .relative,
.dialog-leave-active .relative {
  transition: all 0.2s ease;
}

.dialog-enter-from,
.dialog-leave-to {
  opacity: 0;
}

.dialog-enter-from .relative,
.dialog-leave-to .relative {
  opacity: 0;
  transform: scale(0.95) translateY(-10px);
}
</style>