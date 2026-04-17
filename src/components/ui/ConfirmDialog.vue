<template>
  <Teleport to="body">
    <Transition name="confirm">
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
            <p class="text-sm text-text-secondary leading-relaxed">
              {{ message }}
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
              v-if="confirmText"
              class="px-3 py-1.5 text-sm text-white rounded-lg transition-colors"
              :class="danger ? 'bg-danger hover:bg-danger-hover' : 'bg-accent hover:bg-accent-hover'"
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
import { ref, watch, onUnmounted } from 'vue'
import { X } from 'lucide-vue-next'

interface Props {
  isOpen: boolean
  title?: string
  message?: string
  confirmText?: string
  cancelText?: string
  danger?: boolean
  closeOnBackdrop?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  title: '确认',
  message: '确定要执行此操作吗？',
  confirmText: '确认',
  cancelText: '取消',
  danger: false,
  closeOnBackdrop: true
})

const emit = defineEmits<{
  confirm: []
  cancel: []
}>()

const dialogRef = ref<HTMLElement | null>(null)

function handleConfirm() {
  emit('confirm')
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

// Lock body scroll when dialog is open
watch(
  () => props.isOpen,
  (isOpen) => {
    if (isOpen) {
      document.body.style.overflow = 'hidden'
      setTimeout(() => {
        dialogRef.value?.focus()
      }, 100)
    } else {
      document.body.style.overflow = ''
    }
  }
)

// Clean up on unmount
onUnmounted(() => {
  document.body.style.overflow = ''
})
</script>

<style scoped>
.confirm-enter-active,
.confirm-leave-active {
  transition: all 0.2s ease;
}

.confirm-enter-active .relative,
.confirm-leave-active .relative {
  transition: all 0.2s ease;
}

.confirm-enter-from,
.confirm-leave-to {
  opacity: 0;
}

.confirm-enter-from .relative,
.confirm-leave-to .relative {
  opacity: 0;
  transform: scale(0.95) translateY(-10px);
}
</style>