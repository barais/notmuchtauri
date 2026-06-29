<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { ThreadDto } from '../types'
import ThreadView from './ThreadView.vue';

const props = defineProps<{
  messageId: string,
  exclude:boolean

}>()

  const read = defineEmits(['mark-as-read']);

const message = ref<ThreadDto[] | null>(null)
const loading = ref(false)
const error = ref<string | null>(null)

  

async function fetchDetails() {
  loading.value = true
  error.value = null
  try {
    const details = await invoke<ThreadDto[]>('get_message_details', { id: props.messageId, exclude:props.exclude })
    message.value = details
  } catch (e) {
    error.value = e instanceof Error ? e.message : String(e)
  } finally {
    loading.value = false
  }
}



onMounted(async () => {
  await fetchDetails()

          try {
    await invoke('modify_message_tag', { 
      messageId: props.messageId, 
      tag: 'unread', 
      action: 'remove' 
    })
    read('mark-as-read', props.messageId) 
  } catch (error) {
    console.error(`Erreur lors du retrait du tag remove:`, error)
  }


})
</script>

<template>
  <div class="mail-detail">
    <div v-if="loading">Loading message...</div>
    <div v-else-if="error" class="error">{{ error }}</div>
    <div v-else-if="message" class="content">
      <ThreadView :threads="message" />
    </div>
    <div v-else>No message selected</div>
  </div>
</template>

<style scoped>
.mail-detail {
  padding: 20px;
  height: 100%;
  overflow-y: auto;
}
.header {
  margin-bottom: 20px;
}
.meta {
  color: #666;
  font-size: 0.9em;
  margin-bottom: 10px;
}
.body {
  white-space: pre-wrap;
  font-family: inherit;
  line-height: 1.5;
}
.error {
  color: red;
}
</style>
