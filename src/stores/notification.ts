import { defineStore } from 'pinia'
import { ref } from 'vue'

export interface Notification {
  id: string
  type: 'success' | 'error' | 'warning' | 'info'
  title: string
  message?: string
  duration?: number // ms, 0 = persistent
  createdAt: number
}

export const useNotificationStore = defineStore('notification', () => {
  const notifications = ref<Notification[]>([])

  function add(notification: Omit<Notification, 'id' | 'createdAt'>) {
    const id = `notif-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`
    const newNotif: Notification = {
      ...notification,
      id,
      createdAt: Date.now(),
      duration: notification.duration ?? 5000 // Default 5 seconds
    }

    notifications.value.push(newNotif)

    // Auto-remove if duration > 0
    if (newNotif.duration && newNotif.duration > 0) {
      setTimeout(() => remove(id), newNotif.duration)
    }

    return id
  }

  function remove(id: string) {
    notifications.value = notifications.value.filter(n => n.id !== id)
  }

  function clear() {
    notifications.value = []
  }

  // Convenience methods
  function success(title: string, message?: string) {
    return add({ type: 'success', title, message })
  }

  function error(title: string, message?: string, duration?: number) {
    return add({ type: 'error', title, message, duration: duration ?? 0 }) // Errors persist by default
  }

  function warning(title: string, message?: string) {
    return add({ type: 'warning', title, message })
  }

  function info(title: string, message?: string) {
    return add({ type: 'info', title, message })
  }

  return {
    notifications,
    add,
    remove,
    clear,
    success,
    error,
    warning,
    info
  }
})