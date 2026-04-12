import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export const useUiStore = defineStore('ui', () => {
  // State
  const sidebarCollapsed = ref(false)
  const readingPanePosition = ref<'right' | 'bottom' | 'hidden'>('right')
  const theme = ref<'light' | 'dark' | 'system'>('dark')
  const density = ref<'comfortable' | 'compact'>('comfortable')
  const fontScale = ref(1)
  const isOnline = ref(true)

  // Getters
  const sidebarWidth = computed(() => sidebarCollapsed.value ? '64px' : '240px')
  const emailListWidth = computed(() => 350)

  // Actions
  function setSidebarCollapsed(value: boolean) {
    sidebarCollapsed.value = value
    localStorage.setItem('sidebarCollapsed', String(value))
  }

  function toggleSidebar() {
    setSidebarCollapsed(!sidebarCollapsed.value)
  }

  function setReadingPanePosition(value: 'right' | 'bottom' | 'hidden') {
    readingPanePosition.value = value
    localStorage.setItem('readingPanePosition', value)
  }

  function setTheme(value: 'light' | 'dark' | 'system') {
    theme.value = value
    localStorage.setItem('theme', value)
    applyTheme(value)
  }

  function setDensity(value: 'comfortable' | 'compact') {
    density.value = value
    localStorage.setItem('density', value)
  }

  function setOnline(value: boolean) {
    isOnline.value = value
  }

  // 应用主题
  function applyTheme(themeValue: 'light' | 'dark' | 'system') {
    const html = document.documentElement
    if (themeValue === 'system') {
      const systemDark = window.matchMedia('(prefers-color-scheme: dark)').matches
      html.classList.toggle('dark', systemDark)
    } else {
      html.classList.toggle('dark', themeValue === 'dark')
    }
  }

  // 初始化
  function init() {
    const savedSidebar = localStorage.getItem('sidebarCollapsed')
    if (savedSidebar) sidebarCollapsed.value = savedSidebar === 'true'

    const savedPane = localStorage.getItem('readingPanePosition') as typeof readingPanePosition.value
    if (savedPane) readingPanePosition.value = savedPane

    const savedTheme = localStorage.getItem('theme') as typeof theme.value
    if (savedTheme) setTheme(savedTheme)

    const savedDensity = localStorage.getItem('density')
    if (savedDensity) density.value = savedDensity as typeof density.value

    // 监听网络状态
    window.addEventListener('online', () => setOnline(true))
    window.addEventListener('offline', () => setOnline(false))
    setOnline(navigator.onLine)
  }

  return {
    sidebarCollapsed,
    readingPanePosition,
    theme,
    density,
    fontScale,
    isOnline,
    sidebarWidth,
    emailListWidth,
    setSidebarCollapsed,
    toggleSidebar,
    setReadingPanePosition,
    setTheme,
    setDensity,
    setOnline,
    init
  }
})
