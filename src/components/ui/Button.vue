<template>
  <button :class="buttonClass" :disabled="disabled" v-bind="$attrs">
    <slot />
  </button>
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  variant?: 'primary' | 'secondary' | 'ghost' | 'danger'
  size?: 'sm' | 'md' | 'lg'
  disabled?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  variant: 'primary',
  size: 'md',
  disabled: false
})

const buttonClass = computed(() => [
  'inline-flex items-center justify-center rounded-lg font-medium transition-all duration-150',
  'focus:outline-none focus:ring-2 focus:ring-accent focus:ring-offset-2 focus:ring-offset-bg-primary',
  'disabled:opacity-50 disabled:cursor-not-allowed',
  {
    // Primary
    'bg-accent text-white hover:bg-accent-hover active:bg-accent-dark':
      props.variant === 'primary',
    // Secondary
    'bg-bg-tertiary text-text-primary hover:bg-bg-hover border border-border-primary':
      props.variant === 'secondary',
    // Ghost
    'text-text-secondary hover:bg-bg-hover hover:text-text-primary':
      props.variant === 'ghost',
    // Danger
    'bg-danger text-white hover:bg-red-600 active:bg-red-700':
      props.variant === 'danger',
    // Sizes
    'px-3 py-1.5 text-xs': props.size === 'sm',
    'px-4 py-2 text-sm': props.size === 'md',
    'px-6 py-2.5 text-sm': props.size === 'lg'
  }
])
</script>