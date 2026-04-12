<template>
  <button
    :class="[
      'w-full flex items-center gap-2 px-2 py-1.5 rounded-md text-sm transition-colors press-scale',
      collapsed ? 'justify-center' : '',
      active
        ? 'bg-accent/10 text-accent font-medium'
        : 'text-sidebar-text/80 hover:bg-sidebar-hover hover:text-sidebar-text'
    ]"
    :title="collapsed ? label : undefined"
    @click="$emit('click')"
  >
    <component :is="icon" :size="16" class="shrink-0" />
    <span v-if="!collapsed" class="flex-1 truncate text-left">{{ label }}</span>
    <span
      v-if="!collapsed && count && count > 0"
      class="text-[0.625rem] bg-accent/15 text-accent px-1.5 rounded-full leading-normal"
    >
      {{ count }}
    </span>
  </button>
</template>

<script setup lang="ts">
import type { Component } from 'vue'

interface Props {
  icon: Component
  label: string
  count?: number
  active?: boolean
  collapsed?: boolean
}

defineProps<Props>()
defineEmits<{
  click: []
}>()
</script>