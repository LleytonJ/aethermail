<template>
  <div
    class="fixed top-4 right-4 z-50 flex flex-col gap-2 max-w-sm"
  >
    <TransitionGroup name="notification">
      <div
        v-for="notif in notifications"
        :key="notif.id"
        class="flex items-start gap-3 p-3 rounded-lg shadow-lg border"
        :class="{
          'bg-danger/10 border-danger/30 text-danger': notif.type === 'error',
          'bg-success/10 border-success/30 text-success': notif.type === 'success',
          'bg-warning/10 border-warning/30 text-warning': notif.type === 'warning',
          'bg-accent/10 border-accent/30 text-accent': notif.type === 'info'
        }"
      >
        <div class="mt-0.5">
          <CheckCircle2 v-if="notif.type === 'success'" class="w-5 h-5" />
          <XCircle v-else-if="notif.type === 'error'" class="w-5 h-5" />
          <AlertTriangle v-else-if="notif.type === 'warning'" class="w-5 h-5" />
          <Info v-else class="w-5 h-5" />
        </div>
        <div class="flex-1 min-w-0">
          <div class="font-medium text-sm">{{ notif.title }}</div>
          <div v-if="notif.message" class="text-xs mt-0.5 opacity-80">{{ notif.message }}</div>
        </div>
        <button
          class="p-1 hover:opacity-70 transition-opacity"
          @click="remove(notif.id)"
        >
          <X class="w-4 h-4" />
        </button>
      </div>
    </TransitionGroup>
  </div>
</template>

<script setup lang="ts">
import { XCircle, CheckCircle2, AlertTriangle, Info, X } from 'lucide-vue-next'
import { storeToRefs } from 'pinia'
import { useNotificationStore } from '@/stores/notification'

const notificationStore = useNotificationStore()
const { notifications } = storeToRefs(notificationStore)
const { remove } = notificationStore
</script>

<style scoped>
.notification-enter-active {
  transition: all 0.3s ease-out;
}

.notification-leave-active {
  transition: all 0.2s ease-in;
}

.notification-enter-from {
  opacity: 0;
  transform: translateX(30px);
}

.notification-leave-to {
  opacity: 0;
  transform: translateX(30px);
}
</style>