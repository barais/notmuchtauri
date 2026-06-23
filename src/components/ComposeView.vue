<template>
  <div class="flex flex-col h-full bg-white shadow-lg rounded-lg overflow-hidden border border-gray-200">
    <!-- Header/Topbar -->
    <div class="flex items-center justify-between px-4 py-3 bg-gray-50 border-b border-gray-200">
      <h2 class="text-lg font-semibold text-gray-700">
        {{ replyMode === 'forward' ? 'Transférer' : replyMode === 'reply' ? 'Répondre' : replyMode === 'new' ? 'Nouveau message' : 'Répondre à tous' }}
      </h2>
      <div class="space-x-2">
        <button @click="close"
          class="px-3 py-1 text-sm text-gray-600 bg-white border border-gray-300 rounded hover:bg-gray-100 transition-colors">
          Annuler
        </button>
        <button @click="sendEmail" :disabled="isSending || !form.to"
          class="px-4 py-1 text-sm text-white bg-blue-600 rounded hover:bg-blue-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed">
          {{ isSending ? 'Envoi...' : 'Envoyer' }}
        </button>
      </div>
    </div>

    <!-- Formulaire (Champs d'entête) -->
    <div class="flex flex-col px-4 py-2 space-y-2 border-b border-gray-200 bg-gray-50 text-sm">

      <!-- Compte expéditeur -->
      <div class="flex items-center">
        <label class="w-20 font-medium text-gray-500 text-right pr-4">De</label>
        <select v-model="form.account" class="flex-1 bg-transparent border-none focus:ring-0 text-gray-800 p-1">
          <option v-for="account in accounts" :key="account.id" :value="account.id">
            {{ account.label }} ({{ account.email }})
          </option>
        </select>
      </div>

      <!-- À (To) -->
      <div class="flex items-center">
        <label class="w-20 font-medium text-gray-500 text-right pr-4">À</label>
         <div
        :ref="(el) => setContainerRef(el, 'to')"
        class="relative flex flex-wrap items-center gap-1 p-1 rounded-md dark:bg-zinc-800 focus-within:ring-2 focus-within:ring-blue-500 transition-all  w-full"
      >
        <!-- Pills -->
        <div v-for="addr in recipients['to']" :key="addr.email"
             class="flex items-center gap-1 px-2 py-1 text-blue-700 dark:bg-blue-900 dark:text-blue-300 rounded-full text-xs">
          {{ addr.name }}
          <button @click="removeRecipient('to', addr.email)" class="hover:text-blue-900">×</button>
        </div>

        <input
          v-model="queries['to']"
          @input="onInputChanged('to')"
          @keydown.enter="completeAddress('to')"
          @keydown.down.prevent="highlightNext('to')"
          @keydown.up.prevent="highlightPrev('to')"
          class="flex-1 outline-none bg-transparent text-sm"
        />

        <!-- Dropdown -->
        <ul v-if="suggestions['to'].length"
            class="absolute z-50 w-64 bg-white dark:bg-zinc-800 border rounded-md shadow-lg max-h-60 overflow-y-auto"
            :style="getDropdownStyle()">
          <li
            v-for="(s, index) in suggestions['to']"
            :key="s.email"
            @click="selectAddress('to', s)"
            :class="['p-2 text-sm cursor-pointer hover:bg-blue-50 dark:hover:bg-zinc-700 transition-colors',
                     highlightedIndex['to'] === index ? 'bg-blue-100 dark:bg-zinc-700' : '']"
          >
            <div class="font-medium">{{ s.name }}</div>
            <div class="text-xs text-gray-500">{{ s.email }}</div>
          </li>
        </ul>
      </div>
    
      </div>

      <!-- Cc -->
      <div class="flex items-center">
        <label class="w-20 font-medium text-gray-500 text-right pr-4">Cc</label>
<!--        <input v-model="form.cc" type="text" class="flex-1 bg-transparent border-none focus:ring-0 text-gray-800 p-1" />-->
                 <div
        :ref="(el) => setContainerRef(el, 'cc')"
        class="relative flex flex-wrap items-center gap-1 p-1 rounded-md dark:bg-zinc-800 focus-within:ring-2 focus-within:ring-blue-500 transition-all  w-full"
      >
        <!-- Pills -->
        <div v-for="addr in recipients['cc']" :key="addr.email"
             class="flex items-center gap-1 px-2 py-1 text-blue-700 dark:bg-blue-900 dark:text-blue-300 rounded-full text-xs">
          {{ addr.name }}
          <button @click="removeRecipient('cc', addr.email)" class="hover:text-blue-900">×</button>
        </div>

        <input
          v-model="queries['cc']"
          @input="onInputChanged('cc')"
          @keydown.enter="completeAddress('cc')"
          @keydown.down.prevent="highlightNext('cc')"
          @keydown.up.prevent="highlightPrev('cc')"
          class="flex-1 outline-none bg-transparent text-sm"
        />

        <!-- Dropdown -->
        <ul v-if="suggestions['cc'].length"
            class="absolute z-50 w-64 bg-white dark:bg-zinc-800 border rounded-md shadow-lg max-h-60 overflow-y-auto"
            :style="getDropdownStyle()">
          <li
            v-for="(s, index) in suggestions['cc']"
            :key="s.email"
            @click="selectAddress('cc', s)"
            :class="['p-2 text-sm cursor-pointer hover:bg-blue-50 dark:hover:bg-zinc-700 transition-colors',
                     highlightedIndex['cc'] === index ? 'bg-blue-100 dark:bg-zinc-700' : '']"
          >
            <div class="font-medium">{{ s.name }}</div>
            <div class="text-xs text-gray-500">{{ s.email }}</div>
          </li>
        </ul>
      </div>

      </div>

      <!-- Bcc -->
      <div class="flex items-center" v-show="showBcc">
        <label class="w-20 font-medium text-gray-500 text-right pr-4">Cci</label>
         <div
        :ref="(el) => setContainerRef(el, 'bcc')"
        class="relative flex flex-wrap items-center gap-1 p-1 rounded-md dark:bg-zinc-800 focus-within:ring-2 focus-within:ring-blue-500 transition-all  w-full"
      >
        <!-- Pills -->
        <div v-for="addr in recipients['bcc']" :key="addr.email"
             class="flex items-center gap-1 px-2 py-1 text-blue-700 dark:bg-blue-900 dark:text-blue-300 rounded-full text-xs">
          {{ addr.name }}
          <button @click="removeRecipient('bcc', addr.email)" class="hover:text-blue-900">×</button>
        </div>

        <input
          v-model="queries['bcc']"
          @input="onInputChanged('bcc')"
          @keydown.enter="completeAddress('bcc')"
          @keydown.down.prevent="highlightNext('bcc')"
          @keydown.up.prevent="highlightPrev('bcc')"
          class="flex-1 outline-none bg-transparent text-sm"
        />

        <!-- Dropdown -->
        <ul v-if="suggestions['bcc'].length"
            class="absolute z-50 w-64 bg-white dark:bg-zinc-800 border rounded-md shadow-lg max-h-60 overflow-y-auto"
            :style="getDropdownStyle()">
          <li
            v-for="(s, index) in suggestions['bcc']"
            :key="s.email"
            @click="selectAddress('bcc', s)"
            :class="['p-2 text-sm cursor-pointer hover:bg-blue-50 dark:hover:bg-zinc-700 transition-colors',
                     highlightedIndex['bcc'] === index ? 'bg-blue-100 dark:bg-zinc-700' : '']"
          >
            <div class="font-medium">{{ s.name }}</div>
            <div class="text-xs text-gray-500">{{ s.email }}</div>
          </li>
        </ul>
      </div>
      </div>

      <!-- Sujet et actions -->
      <div class="flex items-center pt-2 border-t border-gray-200">
        <label class="w-20 font-medium text-gray-500 text-right pr-4">Sujet</label>
        <input v-model="form.subject" type="text"
          class="flex-1 font-semibold bg-transparent border-none focus:ring-0 text-gray-800 p-1" />

        <div class="flex items-center space-x-3 ml-2">
          <button @click="showBcc = !showBcc" class="text-xs text-gray-500 hover:text-blue-600 transition-colors">
            {{ showBcc ? 'Cacher Cci' : 'Ajouter Cci' }}
          </button>

          <!-- Bouton Ajouter une pièce jointe -->
          <button @click="addAttachments"
            class="flex items-center text-xs text-gray-500 hover:text-blue-600 transition-colors"
            title="Ajouter une pièce jointe">
            <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24"
              xmlns="http://www.w3.org/2000/svg">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                d="M15.172 7l-6.586 6.586a2 2 0 102.828 2.828l6.414-6.586a4 4 0 00-5.656-5.656l-6.415 6.585a6 6 0 108.486 8.486L20.5 13">
              </path>
            </svg>
            Joindre
          </button>
        </div>
      </div>
    </div>

    <!-- Zone des pièces jointes -->
    <div v-if="attachments.length > 0" class="flex flex-wrap gap-2 px-4 py-2 bg-gray-100 border-b border-gray-200">
      <div v-for="(file, index) in attachments" :key="file.path"
        class="flex items-center bg-white border border-gray-300 rounded px-2 py-1 text-xs text-gray-700 shadow-sm">
        <svg class="w-3 h-3 text-gray-400 mr-1.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"
          xmlns="http://www.w3.org/2000/svg">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
            d="M15.172 7l-6.586 6.586a2 2 0 102.828 2.828l6.414-6.586a4 4 0 00-5.656-5.656l-6.415 6.585a6 6 0 108.486 8.486L20.5 13">
          </path>
        </svg>
        <span class="truncate max-w-[150px]" :title="file.path">{{ file.name }}</span>
        <button @click="removeAttachment(index)"
          class="ml-2 text-red-400 hover:text-red-600 font-bold focus:outline-none">
          &times;
        </button>
      </div>
    </div>

    <!-- Zone de texte (Body) -->
<!-- Zone de texte (Body) -->
<div class="flex-1 relative bg-white flex flex-col min-h-0"> <!-- Ajout de min-h-0 utile pour les flex-1 imbriqués -->
  
  <div v-if="isLoading" class="absolute inset-0 flex items-center justify-center bg-white bg-opacity-75 z-10">
    <span class="text-gray-500">Chargement du message...</span>
  </div>

  <!-- NOUVEAU CONTENEUR ROW : Regroupe l'Éditeur + le Copilote IA -->
  <div class="flex-1 flex flex-row mt-4 overflow-hidden">
    
    <!-- Zone Éditeur (Prend l'espace restant) -->
    <!-- Le border est déplacé ici, et on retire le mt-4 qui est remonté sur le wrapper parent -->
    <div class="flex-1 flex flex-col border border-gray-300 rounded-md overflow-hidden">
      
      <div v-if="props.isHtml" class="flex-1 flex flex-col">
        <QuillEditor  
          ref="quillEditorRef" 
          v-model:content="form.body" 
          contentType="html" 
          theme="snow" 
          class="flex-1 "
        />        
      </div>
      
      <div v-else class="flex-1 flex flex-col h-full">
        <textarea 
          v-model="form.body" 
          class="flex-1 h-full w-full p-2 outline-none resize-none"
        ></textarea>
      </div>

    </div>

    <!-- Zone Assistant IA -->
    <!-- Intercepte l'événement 'insert-text' émis par le composant enfant -->
     <div>
    <AiCopilot @insert-text="handleAiInsertion"  />
</div>
  </div>

  <!-- Messages area: these will push the editor up -->
  <div class="mt-2 space-y-2">
    <div v-if="errorMsg" class="p-3 bg-red-100 text-red-700 rounded-md text-sm">{{ errorMsg }}</div>
    <div v-if="successMsg" class="p-3 bg-green-100 text-green-700 rounded-md text-sm">{{ successMsg }}</div>
  </div>

</div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core' // Tauri v2
import {  open } from '@tauri-apps/plugin-dialog' // API native pour ouvrir l'explorateur de fichiers
import { QuillEditor } from '@vueup/vue-quill';
import '@vueup/vue-quill/dist/vue-quill.snow.css';
import { AddressMatch, AppConfig, Message, MessageDto } from '../types';
import AiCopilot from './AiCopilot.vue'
// Types
interface AttachmentUI {
  path: string
  name: string
}

// Props
const props = defineProps<{
  messageId: string
  replyMode: 'reply' | 'reply-all' | 'forward' | 'new' | 'none'
  tabsId: string,
  isHtml:boolean,
  message: MessageDto|null
  config:AppConfig|undefined
}>()

// const emit = defineEmits(['close','sent']);

const emit = defineEmits<{
  (e: 'close', id: string): void
  (e: 'sent', id: string): void
}>()
const isSearching = ref<Record<string, boolean>>({ to: false, cc: false, bcc: false });
const recipients = ref<Record<string, AddressMatch[]>>({ to: [], cc: [], bcc: [] });
const queries = ref<Record<string, string>>({ to: '', cc: '', bcc: '' });
const suggestions = ref<Record<string, AddressMatch[]>>({ to: [], cc: [], bcc: [] });
const highlightedIndex = ref<Record<string, number>>({ to: -1, cc: -1, bcc: -1 });
const containerRefs = ref<Record<string, HTMLElement | null>>({});

function setContainerRef(el: any, field: string) {
  if (el) containerRefs.value[field] = el;
}

function getDropdownStyle() {
  return { top: '100%', left: '0px', zIndex: '1000' };
}

  let debounceTimer: ReturnType<typeof setTimeout> | null = null;
async function onInputChanged(field: 'to' | 'cc' | 'bcc') {
  const q = queries.value[field];
   if (debounceTimer) clearTimeout(debounceTimer);
  if (q.length < 5) {
    suggestions.value[field] = [];
    return;
  }
   isSearching.value[field] = true; // Show loading state immediately
    debounceTimer = setTimeout(async () => {
  const results = await invoke<AddressMatch[]>('lookup_address', { query: q, limit:20 });
  if (queries.value[field] === q) {
      suggestions.value[field] = results;
      highlightedIndex.value[field] = 0;
    }
    isSearching.value[field] = false; // Hide loading state
  }, 150);
}

function selectAddress(field: 'to' | 'cc' | 'bcc', addr: AddressMatch) {
  // Optional: Validate if the string looks like an email before adding
  if (!addr.email.includes('@')) {
    // You could show an error toast here: "Invalid email address"
    console.error("Invalid email format");
    return;
  }

  recipients.value[field].push(addr);
  queries.value[field] = ''; // clear input
  suggestions.value[field] = []; // hide dropdown
  const emails = []
  for (const s of recipients.value[field].values()){
    emails.push(s.email);
  }
  form[field] = emails.join(',')
}
function removeRecipient(field: 'to' | 'cc' | 'bcc', email: string) {
  recipients.value[field] = recipients.value[field].filter(a => a.email !== email);
}

function highlightNext(field: 'to' | 'cc' | 'bcc') {
  if (suggestions.value[field].length > 0) {
    highlightedIndex.value[field] = (highlightedIndex.value[field] + 1) % suggestions.value[field].length;
  }
}

function highlightPrev(field: 'to' | 'cc' | 'bcc') {
  if (suggestions.value[field].length > 0) {
    highlightedIndex.value[field] = (highlightedIndex.value[field] - 1 + suggestions.value[field].length) % suggestions.value[field].length;
  }
}

function completeAddress(field: 'to' | 'cc' | 'bcc') {
  const query = queries.value[field].trim();
  if (!query) return;

  // 1. If we have suggestions and one is highlighted, use it
  if (suggestions.value[field].length > 0) {
    const idx = highlightedIndex.value[field];
    const selected = suggestions.value[field][idx];
    selectAddress(field, selected);
    return;
  }

  // 2. Fallback: Treat the raw input as an address
  // We create a dummy AddressMatch object for the unknown email
  selectAddress(field, {
    name: query,
    email: query
  });
}

// État de l'interface
const isLoading = ref(true)
const isSending = ref(false)
const showBcc = ref(false)
const attachments = ref<AttachmentUI[]>([])
const errorMsg = ref('');
const successMsg = ref('');
const quillEditorRef = ref<any>(null);
// Gestion des comptes

const accounts = ref(props.config?.accounts)

const form = reactive({
  account: accounts.value? accounts.value[0]?.id || '':'',
  to: '',
  cc: '',
  bcc: '',
  subject: '',
  body: ''
})


const init = async  ()=> {

    console.error('onmointed')
  try {
    isLoading.value = true
    if (props.messageId !== '') {
      let mess :Message={
            id: "-1",
            subject: '',
            from: '',
            to: '',
            date: '',
            body: '',
            tags: [],
            is_read: false,
            has_attachments: false
        }
      if (props.message!==null && props.isHtml){
         mess  ={
            id: props.message.id,
            subject: props.message.subject,
            from: props.message.from,
            to: props.message.to,
            date: props.message.date,
            body: props.message.htmlBody!,
            tags: [],
            is_read: false,
            has_attachments: false
        }
      }
      const replyData: any = await invoke('get_reply_data', {
        messageId: props.messageId,
        replyMode: props.replyMode,
         message :mess
      })



      form.to = replyData.to || ''
      recipients.value['to'].push({
        name:replyData.to,
        email:replyData.to
      })
      form.cc = replyData.cc || ''
      form.subject = replyData.subject || ''
      if (!props.isHtml){
        form.body = `\n\n${replyData.body || ''}`
      }else {
        form.body = replyData.bodyhtml
        setEditorText(replyData.bodyhtml)
      }
      
    } else {
      form.to = ''
      form.cc = ''
      form.subject = ''
      form.body = `\n\n${''}`

    }

  } catch (error) {
    console.error("Erreur lors de la préparation de la réponse:", error)
  } finally {
    isLoading.value = false
  }

}

onMounted(async () => {
    await init();
})

// --- GESTION DES PIÈCES JOINTES ---

const addAttachments = async () => {
  try {
    // Ouvre la fenêtre système native de sélection de fichiers
    const selected = await open({
      multiple: true,
      title: 'Sélectionnez des pièces jointes',
    })

    if (!selected) return // L'utilisateur a annulé

    // Tauri retourne soit un tableau (multiple: true), soit un string
    const paths = Array.isArray(selected) ? selected : [selected]

    for (const filePath of paths) {
      // Éviter les doublons de fichiers
      if (!attachments.value.find(a => a.path === filePath)) {
        // Extraire le nom du fichier depuis le chemin absolu (compatible Windows \ et Unix /)
        const fileName = filePath.split(/[/\\]/).pop() || filePath
        attachments.value.push({
          path: filePath,
          name: fileName
        })
      }
    }
  } catch (error) {
    console.error("Erreur lors de la sélection des pièces jointes:", error)
  }
}

/**
 * Updates the editor content using the Quill API
 * @param text The text to set in the editor
 */
function setEditorText(text: string) {
  if (quillEditorRef.value) {
    try {
      // 3. Access the actual Quill instance via getQuill()
      const quill = quillEditorRef.value.getQuill();

      // Use setText to update the content
      // 'silent' is used to avoid triggering the 'text-change' event
      // which prevents infinite loops if you have a watcher on the content
      // quill.setText(text, 'silent');
     // const value = text
      const delta = quill.clipboard.convert("<BR> <BR>"+text + "</pre>")
      quill.setContents(delta, 'silent')

      console.log('Editor text updated successfully');
    } catch (e) {
      console.error('Failed to set Quill text:', e);
    }
  } else {
    console.error('Quill editor reference is not yet available');
  }
}

const removeAttachment = (index: number) => {
  attachments.value.splice(index, 1)
}

// --- ENVOI DU MAIL ---

const splitEmails = (emailsStr: string): string[] => {
  return emailsStr
    .split(',')
    .map(e => e.trim())
    .filter(e => e.length > 0)
}

const sendEmail = async () => {
  try {
    isSending.value = true
    errorMsg.value = '';
    successMsg.value = '';

    const selectedAccount = accounts.value?.find(a => a.id === form.account)
    const fromAddress = selectedAccount?.email || ''

    // Mappage de notre tableau d'interface vers l'objet Rust attendu
    const attachmentPayload = attachments.value.map(file => ({
      path: file.path,
      // On passe null (ou undefined) car le backend Rust a "Option<String>"
      // Optionnel: On pourrait utiliser un package NPM "mime-types" côté JS, 
      // ou laisser le Backend Rust deviner le mimetype (recommandé).
      mimeType: null
    }))

    const payload = {
      from: fromAddress,
      to: splitEmails(form.to),
      cc: splitEmails(form.cc),
      bcc: splitEmails(form.bcc),
      subject: form.subject,
      body: form.body,
      isHtml: props.isHtml,
      attachments: attachmentPayload,
      account: form.account,
      sentFolder: form.account? accounts.value?.find(a=> a.id = form.account)?.sent_folder || props.config?.default_sent_folder : props.config?.default_sent_folder
    }
    if (payload.to.length === 0) {
      errorMsg.value = "Veuillez spécifier au moins un destinataire.";
      isSending.value = false;
      return;
    }

    console.error('send_email',payload)
    await invoke('send_email', { payload })
    successMsg.value = "Message envoyé avec succès !";
    emit('sent', props.tabsId)
    setTimeout(() => {
      close();
      // Reset du formulaire pour le prochain message
      form.to = ''; form.cc = ''; form.bcc = ''; form.subject = ''; form.body = '';
    }, 1500);
  } catch (error) {
    console.error("Erreur lors de l'envoi de l'email:", error)
    errorMsg.value = `Erreur lors de l'envoi : ${error}`;

  } finally {
    isSending.value = false
  }
}

const close = () => {
  emit('close', props.tabsId)
}
const handleAiInsertion = (aiText: string) => {
  if (props.isHtml) {
    // Si vous utilisez Quill en mode HTML, on ajoute des sauts de ligne HTML (<br>)
    form.body += `${aiText}`;
    setEditorText(form.body)
    
    // Note: Si vous vouliez insérer exactement là où est le curseur, il faudrait
    // interagir avec l'API de Quill via quillEditorRef.value.getQuill().insertText(...)
  } else {
    // Si c'est du texte brut (textarea)
    form.body += `${aiText}`;
  }
}

</script>
<style>
/* Surcharge mineure pour que l'éditeur Quill ait une belle hauteur et se fonde dans Tailwind */
.ql-container {
  font-family: inherit !important;
  font-size: 0.875rem !important;
  /* text-sm */
  border-bottom-left-radius: 0.375rem;
  border-bottom-right-radius: 0.375rem;
  flex: 1;
  display: flex;
  flex-direction: column;
}

.ql-toolbar {
  border-top-left-radius: 0.375rem;
  border-top-right-radius: 0.375rem;
  background-color: #f9fafb;
  /* bg-gray-50 */
}

.ql-editor {
  flex: 1;
  overflow-y: auto;
}

/* Ensure the Quill container fills the height of the wrapper */
:deep(.ql-container) {
  flex: 1;
  display: flex;
  flex-direction: column;
}

/* Ensure the actual editing area expands to fill the container */
:deep(.ql-editor) {
  flex: 1;
  overflow-y: auto;
}
</style>