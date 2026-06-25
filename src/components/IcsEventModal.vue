<template>
  <Teleport to="body">
    <div 
      v-if="isOpen" 
      class="fixed inset-0 z-[150] flex items-center justify-center p-4 bg-gray-900 bg-opacity-50 backdrop-blur-sm"
      @click="close"
    >
      <div 
        @click.stop 
        class="bg-white dark:bg-zinc-900 rounded-xl shadow-2xl w-full max-w-2xl max-h-[90vh] flex flex-col"
      >
        <!-- Header -->
        <div class="px-6 py-4 border-b border-gray-200 dark:border-zinc-800 flex justify-between items-center bg-gray-50 dark:bg-zinc-950">
          <h2 class="text-lg font-bold text-gray-800 dark:text-zinc-100 flex items-center gap-2">
            <svg class="w-5 h-5 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" /></svg>
            Transformer en Rendez-vous (ICS)
          </h2>
          <button @click="close" class="text-gray-400 hover:text-gray-600 dark:hover:text-zinc-300">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" /></svg>
          </button>
        </div>

        <!-- Body (Scrollable) -->
        <div class="p-6 overflow-y-auto flex-1 space-y-4">
          
          <!-- Choix des destinataires -->
          <div class="bg-blue-50 dark:bg-blue-900/20 p-4 rounded-lg border border-blue-100 dark:border-blue-800">
            <label class="block text-sm font-semibold text-blue-800 dark:text-blue-300 mb-2">Destinataires de l'invitation</label>
            <div class="flex flex-col space-y-2">
              <label class="flex items-center space-x-2 text-sm text-gray-700 dark:text-zinc-300">
                <input type="radio" v-model="form.recipientMode" value="config" class="text-blue-600 focus:ring-blue-500" />
                <span>M'envoyer uniquement (Adresse calendrier configurée)</span>
              </label>
              <label class="flex items-center space-x-2 text-sm text-gray-700 dark:text-zinc-300">
                <input type="radio" v-model="form.recipientMode" value="all" class="text-blue-600 focus:ring-blue-500" />
                <span>Envoyer à tous les participants du mail ({{ participantsCount }} personnes)</span>
              </label>
            </div>
            
            <div v-if="form.recipientMode === 'config' && !config()?.calendaremail" class="mt-2 text-xs text-red-600 font-medium">
              ⚠️ L'adresse "calendar_email" n'est pas configurée dans les paramètres.
            </div>
          </div>

          <!-- Détails de l'événement -->
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-zinc-300 mb-1">Titre de l'événement</label>
            <input v-model="form.summary" type="text" class="form-input" />
          </div>

          <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
            <div>
              <label class="block text-xs text-gray-500 mb-1">Début</label>
              <input v-model="form.startDate" type="datetime-local" class="form-input text-sm py-1.5" />
            </div>
            <div>
              <label class="block text-xs text-gray-500 mb-1">Fin</label>
              <input v-model="form.endDate" type="datetime-local" class="form-input text-sm py-1.5" />
            </div>
            <div class="sm:col-span-2">
              <label class="block text-xs text-gray-500 mb-1">Lieu (Location)</label>
              <input v-model="form.location" type="text" class="form-input text-sm py-1.5" placeholder="Visio, Bureau..." />
            </div>
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-zinc-300 mb-1">Description</label>
            <textarea v-model="form.description" rows="5" class="form-input font-sans text-sm"></textarea>
          </div>
        </div>

        <!-- Footer -->
        <div class="px-6 py-4 border-t border-gray-200 dark:border-zinc-800 bg-gray-50 dark:bg-zinc-950 flex justify-end space-x-3">
          <button @click="close" class="px-4 py-2 text-sm text-gray-700 bg-white border border-gray-300 hover:bg-gray-50 rounded-lg">Annuler</button>
          <button 
            @click="submitIcs" 
            :disabled="isSending || !canSubmit"
            class="flex items-center px-4 py-2 text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 rounded-lg disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <svg v-if="isSending" class="animate-spin -ml-1 mr-2 h-4 w-4 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path></svg>
            Envoyer l'invitation
          </button>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, watch, computed, PropType, inject } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { MessageDto, AppConfig } from '../types'

const props = defineProps({
  isOpen: Boolean,
  message: Object as PropType<MessageDto | null>,
})


const emit = defineEmits(['close', 'event-created'])
const isSending = ref(false)

const config:  ()=>AppConfig = inject(/* key */ 'appConfig')!

const form = ref({
  recipientMode: 'config' as 'config' | 'all',
  summary: '',
  location: '',
  startDate: '',
  endDate: '',
  description: ''
})

// Prépare les dates par défaut (Ex: Heure courante arrondie à l'heure supérieure + 1h)
const initDates = () => {
  const now = new Date()
  now.setMinutes(0, 0, 0)
  now.setHours(now.getHours() + 1)
  
  // Format requis par datetime-local (YYYY-MM-DDTHH:mm)
  // On utilise un petit hack pour éviter les décalages de fuseau horaire
  const toLocalString = (d: Date) => {
    return new Date(d.getTime() - (d.getTimezoneOffset() * 60000)).toISOString().slice(0, 16)
  }

  form.value.startDate = toLocalString(now)
  now.setHours(now.getHours() + 1)
  form.value.endDate = toLocalString(now)
}

watch(() => props.isOpen, (newVal) => {
  if (newVal && props.message) {
    initDates()
    form.value.summary = props.message.subject || 'Nouveau Rendez-vous'
    form.value.location = ''
    form.value.description = `Suite au mail de : ${props.message.from}\n\n${props.message.textBody || '(Voir mail HTML)'}`
  }
})

const close = () => emit('close')

// Extraction des destinataires du mail original (To + Cc)
const getAllRecipients = (): string[] => {
  if (!props.message) return []
  const allStr = [props.message.from, props.message.to, props.message.cc].filter(Boolean).join(',')
  return allStr.split(',').map(s => s.trim()).filter(Boolean)
}

const participantsCount = computed(() => getAllRecipients().length)

const canSubmit = computed(() => {
  if (form.value.recipientMode === 'config' && !config()?.calendaremail) return false
  if (!form.value.summary || !form.value.startDate || !form.value.endDate) return false
  return true
})

// Génère la chaîne formatée au standard iCalendar (RFC 5545)
const generateIcsString = () => {
  // Convertir le format de date (YYYY-MM-DDTHH:mm -> YYYYMMDDTHHMMSSZ) en UTC
  const formatIcsDate = (dateString: string) => {
    const d = new Date(dateString)
    return d.toISOString().replace(/[-:]/g, '').split('.')[0] + 'Z'
  }

  // Échapper les sauts de ligne pour iCalendar
  const safeDesc = form.value.description.replace(/\n/g, '\\n')

  return `BEGIN:VCALENDAR
VERSION:2.0
PRODID:-//NotmuchTauri//MUA//FR
CALSCALE:GREGORIAN
METHOD:REQUEST
BEGIN:VEVENT
DTSTAMP:${formatIcsDate(new Date().toISOString())}
DTSTART:${formatIcsDate(form.value.startDate)}
DTEND:${formatIcsDate(form.value.endDate)}
SUMMARY:${form.value.summary}
LOCATION:${form.value.location}
DESCRIPTION:${safeDesc}
UID:${crypto.randomUUID()}@notmuchtauri
STATUS:CONFIRMED
END:VEVENT
END:VCALENDAR`
}

const submitIcs = async () => {
  isSending.value = true

  // Déterminer la liste finale des adresses
  let recipients: string[] = []
  if (form.value.recipientMode === 'config' && config()?.calendaremail) {
    recipients = [config().calendaremail]
  } else {
    // Si on répond à tous, on filtre notre propre adresse si nécessaire
    recipients = getAllRecipients()
  }

  const icsContent = generateIcsString()

  try {
    await invoke('send_ics_email', {
      toAddresses: recipients,
      subject: `Invitation : ${form.value.summary}`,
      body: "Veuillez trouver ci-joint l'invitation à l'événement.\n\n" + form.value.description,
      icsContent: icsContent, 
      sentfolder: config()?.default_sent_folder || "Sent"
    })

    emit('event-created')
    close()
  } catch (error) {
    console.error("Erreur lors de l'envoi de l'invitation :", error)
    alert("Impossible d'envoyer l'invitation : " + error)
  } finally {
    isSending.value = false
  }
}
</script>

<style scoped>
</style>