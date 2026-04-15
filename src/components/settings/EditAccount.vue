<template>
  <Modal
    :is-open="isOpen"
    title="编辑账户"
    width="lg"
    :close-on-backdrop="false"
    @close="emit('close')"
  >
    <EditAccountForm
      :account="account"
      @close="emit('close')"
      @saved="emit('saved')"
    />
  </Modal>
</template>

<script setup lang="ts">
import Modal from '@/components/ui/Modal.vue'
import EditAccountForm from './EditAccountForm.vue'
import type { Account } from '@/stores/account'

interface Props {
  isOpen: boolean
  account: Account | null
}

defineProps<Props>()

const emit = defineEmits<{
  close: []
  saved: []
}>()
</script>