<template>
  <Teleport to="body">
    <Transition name="modal">
      <div
        v-if="isOpen"
        class="fixed inset-0 z-50 flex items-center justify-center p-4"
      >
        <!-- Backdrop -->
        <div
          class="absolute inset-0 bg-black/70"
          @click="handleBackdropClick"
        />

        <!-- Modal Content -->
        <div
          ref="modalRef"
          class="relative bg-bg-primary border border-border-primary rounded-xl shadow-2xl max-h-[90vh] overflow-hidden"
          :class="widthClass"
          @keydown.esc="handleEscape"
        >
          <!-- Header -->
          <div
            v-if="title"
            class="flex items-center justify-between px-4 py-3 border-b border-border-primary"
          >
            <h2 class="text-lg font-semibold text-text-primary">
              {{ title }}
            </h2>
            <button
              v-if="closable"
              class="p-1 rounded-md text-text-tertiary hover:text-text-primary hover:bg-bg-hover transition-colors"
              @click="close"
            >
              <X :size="18" />
            </button>
          </div>

          <!-- Body -->
          <div class="overflow-y-auto max-h-[calc(90vh-60px)]">
            <slot />
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, computed, watch, onUnmounted } from 'vue'
import { X } from 'lucide-vue-next'

interface Props {
  isOpen: boolean
  title?: string
  width?: string
  closable?: boolean
  closeOnBackdrop?: boolean
  closeOnEscape?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  title: '',
  width: 'md',
  closable: true,
  closeOnBackdrop: true,
  closeOnEscape: true
})

const emit = defineEmits<{
  close: []
}>()

const modalRef = ref<HTMLElement | null>(null)

const widthClass = computed(() => {
  const widths: Record<string, string> = {
    sm: 'w-full max-w-sm',
    md: 'w-full max-w-md',
    lg: 'w-full max-w-lg',
    xl: 'w-full max-w-xl',
    '2xl': 'w-full max-w-2xl',
    full: 'w-full max-w-4xl'
  }
  return widths[props.width] || widths.md
})

function close() {
  emit('close')
}

function handleBackdropClick() {
  if (props.closeOnBackdrop) {
    close()
  }
}

function handleEscape() {
  if (props.closeOnEscape) {
    close()
  }
}

// Lock body scroll when modal is open
watch(
  () => props.isOpen,
  (isOpen) => {
    if (isOpen) {
      document.body.style.overflow = 'hidden'
      // Focus the modal for keyboard events
      setTimeout(() => {
        modalRef.value?.focus()
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
.modal-enter-active,
.modal-leave-active {
  transition: all 0.2s ease;
}

.modal-enter-active .relative,
.modal-leave-active .relative {
  transition: all 0.2s ease;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}

.modal-enter-from .relative,
.modal-leave-to .relative {
  opacity: 0;
  transform: scale(0.95) translateY(-10px);
}
</style>