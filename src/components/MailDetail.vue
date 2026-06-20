<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { Message } from '../types'

const props = defineProps<{
  messageId: string
}>()

const message = ref<Message | null>(null)
const loading = ref(false)
const error = ref<string | null>(null)

async function fetchDetails() {
  loading.value = true
  error.value = null
  try {
    const details = await invoke<Message>('get_message_details', { id: props.messageId })
    message.value = details
  } catch (e) {
    error.value = e instanceof Error ? e.message : String(e)
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  fetchDetails()
})
</script>

<template>
  <div class="mail-detail">
    <div v-if="loading">Loading message...</div>
    <div v-else-if="error" class="error">{{ error }}</div>
    <div v-else-if="message" class="content">
      <div class="header">
        <h1>{{ message.subject }}</h1>
        <div class="meta">
          <strong>From:</strong> {{ message.from }} <br>
          <strong>Date:</strong> {{ message.date }}
        </div>
      </div>
      <hr />
      <div class="body">
        <pre>{{ message.body }}</pre>
      </div>
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
