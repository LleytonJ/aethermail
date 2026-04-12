<template>
  <div class="space-y-0.5">
    <button
      v-for="tag in tags"
      :key="tag.id"
      class="w-full flex items-center gap-2 px-2 py-1.5 text-sm text-sidebar-text/70 hover:bg-sidebar-hover rounded-md transition-colors"
    >
      <div
        class="w-3 h-3 rounded-sm shrink-0"
        :style="{ backgroundColor: tag.color }"
      />
      <span class="flex-1 truncate text-left">{{ tag.name }}</span>
      <span v-if="tag.count" class="text-xs text-sidebar-text/50">
        {{ tag.count }}
      </span>
    </button>
  </div>
</template>

<script setup lang="ts">
const tags = [
  { id: '1', name: '工作', color: '#3b82f6', count: 5 },
  { id: '2', name: '个人', color: '#10b981', count: 2 },
  { id: '3', name: '重要', color: '#ef4444', count: 3 }
]
</script>