import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'mail',
      component: () => import('@/components/layout/MailLayout.vue')
    },
    {
      path: '/settings',
      name: 'settings',
      component: () => import('@/views/SettingsPage.vue'),
      redirect: '/settings/accounts',
      children: [
        {
          path: 'accounts',
          name: 'settings-accounts',
          component: () => import('@/components/settings/AccountManagement.vue')
        },
        {
          path: 'general',
          name: 'settings-general',
          component: () => import('@/components/settings/GeneralSettings.vue')
        },
        {
          path: 'ai',
          name: 'settings-ai',
          component: () => import('@/components/settings/AiSettings.vue')
        }
      ]
    }
  ]
})

export default router