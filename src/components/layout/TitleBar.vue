<template>
  <header
    data-tauri-drag-region
    class="flex items-center h-8 bg-sidebar-bg border-b border-border-primary select-none shrink-0"
  >
    <!-- 左侧：应用名称 -->
    <div
      data-tauri-drag-region
      class="flex items-center gap-2 px-4"
    >
      <Mail :size="16" class="text-accent" />
      <span
        data-tauri-drag-region
        class="text-xs font-semibold text-sidebar-text tracking-wide"
      >
        AetherMail
      </span>
    </div>
  </header>
</template>

<script setup lang="ts">
import { Mail } from 'lucide-vue-next'
</script>