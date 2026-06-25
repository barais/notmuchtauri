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
            <svg class="w-5 h-5 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-6 9l2 2 4-4" /></svg>
            Convertir en tâche (Remember The Milk)
          </h2>
          <button @click="close" class="text-gray-400 hover:text-gray-600 dark:hover:text-zinc-300">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" /></svg>
          </button>
        </div>

        <!-- Avertissement si l'adresse RTM n'est pas configurée -->
        <div v-if="!config()?.rmtmmail" class="p-4 bg-orange-50 border-b border-orange-200 text-orange-800 text-sm">
          <strong>Attention :</strong> Votre adresse email Remember The Milk n'est pas configurée. Veuillez l'ajouter dans les paramètres de l'application (champ <code>rmtmmail</code>).
        </div>

        <!-- Body (Scrollable) -->
        <div class="p-6 overflow-y-auto flex-1 space-y-4">
          <!-- Nom de la tâche (Sujet du mail) -->
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-zinc-300 mb-1">Nom de la tâche (Sujet)</label>
            <input v-model="form.taskName" type="text" class="form-input" placeholder="Acheter du lait..." />
          </div>

          <!-- Grille des propriétés -->
          <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
            <div>
              <label class="block text-xs text-gray-500 mb-1">Priorité</label>
              <select v-model="form.priority" class="form-input text-sm py-1.5">
                <option value="">Aucune</option>
                <option value="1">1 (Haute)</option>
                <option value="2">2 (Moyenne)</option>
                <option value="3">3 (Basse)</option>
              </select>
            </div>
            
            <div>
              <label class="block text-xs text-gray-500 mb-1">Échéance (Due) <span class="text-[10px] italic">ex: Monday at 8pm</span></label>
              <input v-model="form.due" type="text" class="form-input text-sm py-1.5" placeholder="Demain, 15:00..." />
            </div>

            <div>
              <label class="block text-xs text-gray-500 mb-1">Début (Start)</label>
              <input v-model="form.start" type="text" class="form-input text-sm py-1.5" placeholder="Vendredi..." />
            </div>

            <div>
              <label class="block text-xs text-gray-500 mb-1">Répétition (Repeat)</label>
              <input v-model="form.repeat" type="text" class="form-input text-sm py-1.5" placeholder="Every Week..." />
            </div>

            <div>
              <label class="block text-xs text-gray-500 mb-1">Estimation (Estimate)</label>
              <input v-model="form.estimate" type="text" class="form-input text-sm py-1.5" placeholder="15 min, 2 hours..." />
            </div>

            <div>
              <label class="block text-xs text-gray-500 mb-1">Liste RTM (List)</label>
              <input v-model="form.list" type="text" class="form-input text-sm py-1.5" placeholder="Work, Inbox..." />
            </div>

            <div>
              <label class="block text-xs text-gray-500 mb-1">Tags (Séparez par des espaces)</label>
              <input v-model="form.tags" type="text" class="form-input text-sm py-1.5" placeholder="errand urgent..." />
            </div>

            <div>
              <label class="block text-xs text-gray-500 mb-1">URL / Lien</label>
              <input v-model="form.url" type="text" class="form-input text-sm py-1.5" placeholder="https://..." />
            </div>
          </div>

          <!-- Notes (Préremplies avec l'email) -->
          <div class="mt-4">
            <label class="block text-sm font-medium text-gray-700 dark:text-zinc-300 mb-1">Note de la tâche</label>
            <p class="text-xs text-gray-400 mb-2">Le texte ci-dessous sera ajouté comme note via le séparateur <code>---</code>.</p>
            <textarea v-model="form.notes" rows="6" class="form-input font-sans text-sm"></textarea>
          </div>
        </div>

        <!-- Footer -->
        <div class="px-6 py-4 border-t border-gray-200 dark:border-zinc-800 bg-gray-50 dark:bg-zinc-950 flex justify-end space-x-3">
          <button @click="close" class="px-4 py-2 text-sm text-gray-700 bg-white border border-gray-300 hover:bg-gray-50 rounded-lg">Annuler</button>
          <button 
            @click="submitTask" 
            :disabled="isSending || !config()?.rmtmmail || !form.taskName"
            class="flex items-center px-4 py-2 text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 rounded-lg disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            <svg v-if="isSending" class="animate-spin -ml-1 mr-2 h-4 w-4 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path></svg>
            Créer la tâche
          </button>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, watch, PropType, inject } from 'vue'
import { invoke } from '@tauri-apps/api/core' // Adapter selon Tauri v1/v2
import type { MessageDto, AppConfig } from '../types'

const props = defineProps({
  isOpen: Boolean,
  message: Object as PropType<MessageDto | null>,
//   config: Object as PropType<AppConfig | null>
})

const emit = defineEmits(['close', 'task-created'])
const config:  ()=>AppConfig = inject(/* key */ 'appConfig')!

const isSending = ref(false)

// Formulaire
const form = ref({
  taskName: '',
  priority: '',
  due: '',
  start: '',
  repeat: '',
  estimate: '',
  list: '',
  tags: '',
  url: '',
  notes: ''
})

// A chaque fois que la modale s'ouvre avec un nouveau message, on pré-remplit
watch(() => props.isOpen, (newVal) => {
  if (newVal && props.message) {
    // Nettoie l'objet form
    form.value = {
      taskName: props.message.subject || 'Nouvelle tâche issue d\'un mail',
      priority: '', due: '', start: '', repeat: '', estimate: '', list: '', tags: '', url: '',
      
      // On pré-remplit les notes avec un contexte clair
      notes: `Email original de : ${props.message.from}\nDate : ${props.message.date}\n\n${props.message.textBody || '(Message HTML uniquement)'}`
    }
  }
})

const close = () => {
  emit('close')
}

const submitTask = async () => {
  if (!config()?.rmtmmail) return

  isSending.value = true

  // 1. Construction du corps du mail avec la syntaxe exacte de RTM (Method 2)
  let emailBody = ''

  if (form.value.priority) emailBody += `Priority: ${form.value.priority}\n`
  if (form.value.due) emailBody += `Due: ${form.value.due}\n`
  if (form.value.start) emailBody += `Start: ${form.value.start}\n`
  if (form.value.repeat) emailBody += `Repeat: ${form.value.repeat}\n`
  if (form.value.estimate) emailBody += `Estimate: ${form.value.estimate}\n`
  if (form.value.list) emailBody += `List: ${form.value.list}\n`
  if (form.value.tags) emailBody += `Tags: ${form.value.tags}\n`
  if (form.value.url) emailBody += `URL: ${form.value.url}\n`

  // Ajout des notes avec le délimiteur '---' (Titre de la note puis contenu)
  if (form.value.notes.trim()) {
    emailBody += `\n---\nContexte de l'email\n\n${form.value.notes.trim()}\n`
  }

  try {
    // 2. Envoi du mail via Tauri (à créer côté Rust si pas déjà fait)

        const payload = {
      from:'barais@irisa.fr',
      to:  [config().rmtmmail],
      cc:[],
      bcc:[],
      attachments:[],
      subject: form.value.taskName,
      body: emailBody,
      isHtml: false,
      sentFolder: config()?.default_sent_folder
    }

    await invoke('send_email', { payload })

    // On peut aussi rajouter le tag 'todo' dans notmuch pour garder une trace locale !
    if (props.message?.id) {
      await invoke('modify_message_tag', { messageId: props.message.id, tag: 'todo', action: 'add' })
    }

    emit('task-created')
    close()
  } catch (error) {
    console.error("Erreur lors de la création de la tâche:", error)
    alert("Impossible d'envoyer la tâche : " + error)
  } finally {
    isSending.value = false
  }
}
</script>

<style scoped>
</style>