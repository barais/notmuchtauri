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
        <div :ref="(el) => setContainerRef(el, 'to')"
          class="relative flex flex-wrap items-center gap-1 p-1 rounded-md dark:bg-zinc-800 focus-within:ring-2 focus-within:ring-blue-500 transition-all  w-full">
          <!-- Pills -->
          <div v-for="addr in recipients['to']" :key="addr.email"
            class="flex items-center gap-1 px-2 py-1 text-blue-700 dark:bg-blue-900 dark:text-blue-300 rounded-full text-xs">
            {{ addr.name }}
            <button @click="removeRecipient('to', addr.email)" class="hover:text-blue-900">×</button>
          </div>

          <input v-model="queries['to']" @input="onInputChanged('to')" @keydown.enter="completeAddress('to')"
            @keydown.down.prevent="highlightNext('to')" @keydown.up.prevent="highlightPrev('to')"
            class="flex-1 outline-none bg-transparent text-sm" />

          <!-- Dropdown -->
          <ul v-if="suggestions['to'].length"
            class="absolute z-50 w-64 bg-white dark:bg-zinc-800 border rounded-md shadow-lg max-h-60 overflow-y-auto"
            :style="getDropdownStyle()">
            <li v-for="(s, index) in suggestions['to']" :key="s.email" @click="selectAddress('to', s)" :class="['p-2 text-sm cursor-pointer hover:bg-blue-50 dark:hover:bg-zinc-700 transition-colors',
              highlightedIndex['to'] === index ? 'bg-blue-100 dark:bg-zinc-700' : '']">
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
        <div :ref="(el) => setContainerRef(el, 'cc')"
          class="relative flex flex-wrap items-center gap-1 p-1 rounded-md dark:bg-zinc-800 focus-within:ring-2 focus-within:ring-blue-500 transition-all  w-full">
          <!-- Pills -->
          <div v-for="addr in recipients['cc']" :key="addr.email"
            class="flex items-center gap-1 px-2 py-1 text-blue-700 dark:bg-blue-900 dark:text-blue-300 rounded-full text-xs">
            {{ addr.name }}
            <button @click="removeRecipient('cc', addr.email)" class="hover:text-blue-900">×</button>
          </div>

          <input v-model="queries['cc']" @input="onInputChanged('cc')" @keydown.enter="completeAddress('cc')"
            @keydown.down.prevent="highlightNext('cc')" @keydown.up.prevent="highlightPrev('cc')"
            class="flex-1 outline-none bg-transparent text-sm" />

          <!-- Dropdown -->
          <ul v-if="suggestions['cc'].length"
            class="absolute z-50 w-64 bg-white dark:bg-zinc-800 border rounded-md shadow-lg max-h-60 overflow-y-auto"
            :style="getDropdownStyle()">
            <li v-for="(s, index) in suggestions['cc']" :key="s.email" @click="selectAddress('cc', s)" :class="['p-2 text-sm cursor-pointer hover:bg-blue-50 dark:hover:bg-zinc-700 transition-colors',
              highlightedIndex['cc'] === index ? 'bg-blue-100 dark:bg-zinc-700' : '']">
              <div class="font-medium">{{ s.name }}</div>
              <div class="text-xs text-gray-500">{{ s.email }}</div>
            </li>
          </ul>
        </div>

      </div>

      <!-- Bcc -->
      <div class="flex items-center" v-show="showBcc">
        <label class="w-20 font-medium text-gray-500 text-right pr-4">Cci</label>
        <div :ref="(el) => setContainerRef(el, 'bcc')"
          class="relative flex flex-wrap items-center gap-1 p-1 rounded-md dark:bg-zinc-800 focus-within:ring-2 focus-within:ring-blue-500 transition-all  w-full">
          <!-- Pills -->
          <div v-for="addr in recipients['bcc']" :key="addr.email"
            class="flex items-center gap-1 px-2 py-1 text-blue-700 dark:bg-blue-900 dark:text-blue-300 rounded-full text-xs">
            {{ addr.name }}
            <button @click="removeRecipient('bcc', addr.email)" class="hover:text-blue-900">×</button>
          </div>

          <input v-model="queries['bcc']" @input="onInputChanged('bcc')" @keydown.enter="completeAddress('bcc')"
            @keydown.down.prevent="highlightNext('bcc')" @keydown.up.prevent="highlightPrev('bcc')"
            class="flex-1 outline-none bg-transparent text-sm" />

          <!-- Dropdown -->
          <ul v-if="suggestions['bcc'].length"
            class="absolute z-50 w-64 bg-white dark:bg-zinc-800 border rounded-md shadow-lg max-h-60 overflow-y-auto"
            :style="getDropdownStyle()">
            <li v-for="(s, index) in suggestions['bcc']" :key="s.email" @click="selectAddress('bcc', s)" :class="['p-2 text-sm cursor-pointer hover:bg-blue-50 dark:hover:bg-zinc-700 transition-colors',
              highlightedIndex['bcc'] === index ? 'bg-blue-100 dark:bg-zinc-700' : '']">
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
    <div class="flex-1 relative bg-white flex flex-col min-h-0">


      <!-- Conteneur principal -->
      <div class="flex-1 flex flex-row mt-2 overflow-hidden gap-4">

        <!-- Conteneur GLOBAL Éditeur -->
        <div class="flex-1 border border-gray-300 rounded-md overflow-hidden bg-white flex flex-col">

          <!-- Barre d'outils Tiptap (Masquée si isHtml est faux) -->
          <div v-if="props.isHtml && editor"
            class="flex flex-wrap items-center gap-1 p-2 border-b border-gray-200 bg-gray-50 flex-shrink-0 z-20">
            <button @click="editor.chain().focus().toggleHeading({ level: 1 }).run()"
              :class="{ 'bg-gray-200': editor.isActive('heading', { level: 1 }) }"
              class="p-1.5 rounded hover:bg-gray-200 text-gray-700 font-bold w-8 h-8">H1</button>
            <button @click="editor.chain().focus().toggleHeading({ level: 2 }).run()"
              :class="{ 'bg-gray-200': editor.isActive('heading', { level: 2 }) }"
              class="p-1.5 rounded hover:bg-gray-200 text-gray-700 font-bold w-8 h-8">H2</button>

            <button @click="editor.chain().focus().toggleBold().run()"
              :class="{ 'bg-gray-200': editor.isActive('bold') }"
              class="p-1.5 rounded hover:bg-gray-200 text-gray-700 font-bold w-8 h-8">B</button>
            <button @click="editor.chain().focus().toggleItalic().run()"
              :class="{ 'bg-gray-200': editor.isActive('italic') }"
              class="p-1.5 rounded hover:bg-gray-200 text-gray-700 italic w-8 h-8">I</button>
            <button @click="editor.chain().focus().toggleStrike().run()"
              :class="{ 'bg-gray-200': editor.isActive('strike') }"
              class="p-1.5 rounded hover:bg-gray-200 text-gray-700 line-through w-8 h-8">S</button>
            <div class="w-px h-5 bg-gray-300 mx-1"></div>
            <button @click="editor.chain().focus().toggleBulletList().run()"
              :class="{ 'bg-gray-200': editor.isActive('bulletList') }"
              class="p-1.5 rounded hover:bg-gray-200 text-gray-700 w-8 h-8">•</button>
            <button @click="editor.chain().focus().toggleOrderedList().run()"
              :class="{ 'bg-gray-200': editor.isActive('orderedList') }"
              class="p-1.5 rounded hover:bg-gray-200 text-gray-700 w-8 h-8">1.</button>
            <div class="w-px h-5 bg-gray-300 mx-1"></div>

            <select v-model="spellLanguage" class="text-xs border border-gray-300 rounded p-1">
              <option value="fr">Français</option>
              <option value="en-US">English</option>
            </select>

            <button @click="checkSpelling" :disabled="isCheckingSpelling || !props.config?.lthostport"
              class="px-3 py-1.5 text-sm bg-blue-50 text-blue-700 hover:bg-blue-100 rounded-md transition-colors">
              {{ isCheckingSpelling ? 'Vérification...' : "Vérifier l'orthographe" }}
            </button>

          </div>

          <!-- ZONE DE TEXTE ET CALQUES (RELATIVE) -->
          <!-- ref="editorAreaRef" est appliqué ici pour calculer les coordonnées justes sous la toolbar -->
          <div ref="editorAreaRef" class="relative flex-1 w-full min-h-0">

            <!-- Zone d'édition Tiptap (Étirée aux 4 coins via absolute inset-0) -->
            <editor-content :editor="editor" class="absolute inset-0 overflow-y-auto outline-none tiptap-container"
              :class="{ 'is-plain-text': !props.isHtml }" />

            <!-- Calques des erreurs LT (Étirés aussi) -->
            <div class="absolute inset-0 pointer-events-none overflow-hidden z-10">
              <div v-for="(overlay, i) in errorOverlays" :key="i"
                class="absolute pointer-events-auto cursor-context-menu hover:bg-red-50 hover:bg-opacity-30" :style="{
                  top: overlay.bounds.top + 'px',
                  left: overlay.bounds.left + 'px',
                  width: overlay.bounds.width + 'px',
                  height: overlay.bounds.height + 'px',
                  borderBottom: '2px dotted #ef4444'
                }" @contextmenu.prevent="openSpellMenu($event, overlay.match)"
                @click.stop="openSpellMenu($event, overlay.match)" title="Clic pour corriger"></div>
            </div>
          </div>
        </div>

        <!-- INSÉRER ICI VOTRE COMPOSANT AI Copilot -->
        <AiCopilot @insert-text="handleAiInsertion" />

      </div>

      <!-- MENU CONTEXTUEL DES SUGGESTIONS -->
      <Teleport to="body">
        <div v-if="spellMenu.show && spellMenu.match"
          class="fixed z-[100] w-64 bg-white border border-gray-200 rounded-md shadow-xl overflow-hidden" :style="{
            top: spellMenu.y + 'px', left: spellMenu.x + 'px',
            transform: `translate(${spellMenu.isLeftwards ? 'calc(-100% - 5px)' : '5px'}, ${spellMenu.isUpwards ? 'calc(-100% - 5px)' : '5px'})`
          }" @contextmenu.prevent @click.stop>
          <div class="px-3 py-2 bg-red-50 border-b border-red-100 text-xs font-semibold text-red-800">
            {{ spellMenu.match.message }}
          </div>

          <div class="max-h-48 overflow-y-auto">
            <button v-for="rep in spellMenu.match.replacements" :key="rep.value" @click="applyCorrection(rep.value)"
              class="w-full text-left px-4 py-2 text-sm text-gray-700 hover:bg-blue-50 hover:text-blue-700 transition-colors">
              {{ rep.value }}
            </button>

            <div v-if="spellMenu.match.replacements.length === 0" class="px-4 py-3 text-sm text-gray-500 italic">
              Aucune suggestion
            </div>
          </div>
        </div>
      </Teleport>

    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, onUnmounted, nextTick } from 'vue'
import { invoke } from '@tauri-apps/api/core' // Tauri v2
import { open } from '@tauri-apps/plugin-dialog' // API native pour ouvrir l'explorateur de fichiers

import { useEditor, EditorContent } from '@tiptap/vue-3'
import StarterKit from '@tiptap/starter-kit'

import { AddressMatch, AppConfig, ErrorOverlay, LTMatch, Message, MessageDto } from '../types';
import AiCopilot from './AiCopilot.vue'

// Types
interface AttachmentUI {
  path: string
  name: string
  isPart: boolean
  messageId?: string
  part?: number

}

// Props
const props = defineProps<{
  messageId: string
  replyMode: 'reply' | 'reply-all' | 'forward' | 'editasnew' | 'new' | 'none'
  tabsId: string,
  isHtml: boolean,
  message: MessageDto | null
  config: AppConfig | undefined
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
    const results = await invoke<AddressMatch[]>('lookup_address', { query: q, limit: 20 });
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
  for (const s of recipients.value[field].values()) {
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


// Gestion des comptes

const accounts = ref(props.config?.accounts)

const form = reactive({
  account: accounts.value ? accounts.value[0]?.id || '' : '',
  to: '',
  cc: '',
  bcc: '',
  subject: '',
  body: ''
})


// --- 1. INITIALISATION DE TIPTAP ---
const editor = useEditor({
  extensions: [StarterKit.configure({
    heading: {
      levels: [1, 2, 3],
    }
  })
  ],
  content: form.body,
  onUpdate: ({ editor }) => {
    // Synchronisation avec la variable form.body
    form.body = props.isHtml ? editor.getHTML() : editor.getText()

    // Si l'utilisateur tape du texte, on efface les erreurs pour éviter les décalages
    if (spellingMatches.value.length > 0) {
      spellingMatches.value = []
      errorOverlays.value = []
    }
  },
  // Recalcule les soulignements lors du défilement interne de l'éditeur
  editorProps: {
    attributes: {
      class: 'prose-sm sm:prose-base lg:prose-lg xl:prose-2xl w-full',
    },
    handleScrollToSelection: () => {
      updateOverlays()
      return false
    }
  }
})

const init = async () => {

  document.addEventListener('click', closeSpellMenu)
  // On écoute le scroll de l'éditeur Tiptap
  if (editor.value?.view.dom) {
    editor.value.view.dom.addEventListener('scroll', updateOverlays)
  }
  if (props.replyMode == 'editasnew') {
    isLoading.value = true


    form.to = props.message?.to || ''
    let mails = splitEmails(props.message!.to);
    mails.forEach(m => {
      recipients.value['to'].push({
        name: m,
        email: m
      })

    })
    form.cc = props.message?.cc || ''
    mails = splitEmails(props.message!.cc);
    mails.forEach(m => {
      recipients.value['cc'].push({
        name: m,
        email: m
      })
    })


    form.subject = props.message?.subject || ''
    if (!props.isHtml) {
      form.body = `${props.message?.textBody}`
      if (editor.value) {
        editor.value.commands.setContent(`${props.message?.textBody}`)
      }
    } else {
      form.body = props.message?.htmlBody || ''
      if (editor.value) {
        editor.value.commands.setContent(`${props.message?.htmlBody} `)
      }

    }
    if (props.message?.attachments) {
      for (const a of props.message?.attachments!) {
        attachments.value.push({
          path: a.filename,
          name: a.filename,
          part: a.partId,
          messageId: props.message.id,
          isPart: true
        })
      }
    }


    isLoading.value = false


  } else {
    try {
      isLoading.value = true
      if (props.messageId !== '') {
        let mess: Message = {
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
        if (props.message !== null && props.isHtml) {
          mess = {
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
          message: mess
        })



        form.to = replyData.to || ''
    let mails = splitEmails(replyData.to);
    mails.forEach(m => {
      recipients.value['to'].push({
        name: m,
        email: m
      })
    })
        form.cc = replyData.cc || ''
            mails = splitEmails(replyData.cc);
    mails.forEach(m => {
      recipients.value['cc'].push({
        name: m,
        email: m
      })
    })

        form.subject = replyData.subject || ''
        if (!props.isHtml) {
          form.body = `\n${replyData.body}`
          if (editor.value) {
            editor.value.commands.insertContent(`\n${replyData.body} `)
          }
        } else {
          form.body = replyData.bodyhtml
          if (editor.value) {
            editor.value.commands.insertContent(`${replyData.bodyhtml}`)
          }

        }

        if (props.replyMode==='forward'){
        if (props.message?.attachments) {
          for (const a of props.message?.attachments!) {
            attachments.value.push({
              path: a.filename,
              name: a.filename,
              part: a.partId,
              messageId: props.message.id,
              isPart: true
            })
          }
        }

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
          name: fileName,
          isPart: false
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
      isPart: file.isPart,
      partId: file.isPart ? file.part : -1,
      messageId: file.isPart ? file.messageId : '',
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
      sentFolder: form.account ? accounts.value?.find(a => a.id = form.account)?.sent_folder || props.config?.default_sent_folder : props.config?.default_sent_folder
    }
    if (payload.to.length === 0) {
      errorMsg.value = "Veuillez spécifier au moins un destinataire.";
      isSending.value = false;
      return;
    }

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



// États du correcteur
const isCheckingSpelling = ref(false)
const spellingMatches = ref<LTMatch[]>([])
const errorOverlays = ref<ErrorOverlay[]>([])
const spellLanguage = ref('fr')
const editorAreaRef = ref<HTMLElement | null>(null) // Référence sur la zone de texte uniquement

// État du menu contextuel
const spellMenu = ref({
  show: false,
  x: 0,
  y: 0,
  isUpwards: false,   // Vrai si on est trop près du bas
  isLeftwards: false, // Vrai si on est trop près de la droite
  match: null as LTMatch | null
})

// Conversion du texte brut en HTML pour l'initialisation de Tiptap
let initialContent = form.body
if (!props.isHtml && initialContent) {
  initialContent = initialContent.split('\n').map(line => `<p>${line}</p>`).join('')
}


// --- 2. GESTION DES OFFSETS PROSEMIRROR ---
// LanguageTool reçoit du texte avec des retours à la ligne (\n). 
// Il faut synchroniser ce comptage avec les blocs de ProseMirror.
function offsetToProseMirrorPos(targetOffset: number): number {
  if (!editor.value) return 0
  let currentOffset = 0
  let resolvedPos = 0

  editor.value.state.doc.descendants((node, pos) => {
    if (resolvedPos) return false

    if (node.isText) {
      const textLen = node.text?.length || 0
      if (currentOffset + textLen >= targetOffset) {
        resolvedPos = pos + Math.max(0, targetOffset - currentOffset)
      }
      currentOffset += textLen
    } else if (node.isBlock) {
      if (pos > 0) currentOffset += 1
    }
  })

  return resolvedPos || 1
}

// --- 3. CALCUL DES SOULIGNEMENTS (OVERLAYS) ---
const updateOverlays = () => {
  if (!editor.value || spellingMatches.value.length === 0 || !editorAreaRef.value) {
    errorOverlays.value = []
    return
  }

  // On prend les coordonnées de la zone de texte EXACTE (sous la toolbar)
  const areaRect = editorAreaRef.value.getBoundingClientRect()

  errorOverlays.value = spellingMatches.value.map(match => {
    const startPos = offsetToProseMirrorPos(match.offset)
    const endPos = offsetToProseMirrorPos(match.offset + match.length)

    // Tiptap donne la position à l'écran
    const startCoords = editor.value!.view.coordsAtPos(startPos)
    const endCoords = editor.value!.view.coordsAtPos(endPos)

    return {
      match,
      bounds: {
        // CORRECTION MAJEURE ICI : on base le Top sur le "bottom" du curseur
        // On soustrait areaRect.top pour avoir la coordonnée locale
        // -2px pour placer la ligne exactement au ras du texte
        top: startCoords.bottom - areaRect.top - 2,
        left: startCoords.left - areaRect.left,
        width: Math.max(endCoords.left - startCoords.left, 5),
        height: 2 // On crée une simple div de 2px de haut avec un border-bottom !
      }
    }
  })
}

// --- 4. APPEL A LANGUAGETOOL ---
const checkSpelling = async () => {
  if (!editor.value) return

  // Extraction garantie pour LT : chaque paragraphe est séparé par \n
  const textToCheck = editor.value.state.doc.textBetween(0, editor.value.state.doc.content.size, '\n', '\n')

  if (!textToCheck.trim()) return

  isCheckingSpelling.value = true

  try {
    const params = new URLSearchParams()
    params.append('text', textToCheck)
    params.append('language', spellLanguage.value)

    const response = await fetch(props.config?.lthostport+'/v2/check', {
      method: 'POST',
      headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
      body: params
    })

    if (!response.ok) throw new Error(`Erreur HTTP: ${response.status}`)

    const data = await response.json()
    spellingMatches.value = data.matches

    await nextTick()
    updateOverlays()

  } catch (error) {
    console.error("Erreur LT :", error)
    alert("Impossible de joindre le serveur LanguageTool.")
  } finally {
    isCheckingSpelling.value = false
  }
}

// --- 5. CORRECTION ET MENU ---
const openSpellMenu = (event: MouseEvent, match: LTMatch) => {
  const MENU_HEIGHT = 250;
  const MENU_WIDTH = 260;
  const spaceBelow = window.innerHeight - event.clientY;
  const spaceRight = window.innerWidth - event.clientX;

  spellMenu.value = {
    show: true,
    x: event.clientX,
    y: event.clientY,
    isUpwards: spaceBelow < MENU_HEIGHT,
    isLeftwards: spaceRight < MENU_WIDTH,
    match: match
  }
}

const closeSpellMenu = () => {
  spellMenu.value.show = false
}

const applyCorrection = (replacementValue: string) => {
  if (!spellMenu.value.match || !editor.value) return

  const match = spellMenu.value.match
  const startPos = offsetToProseMirrorPos(match.offset)
  const endPos = offsetToProseMirrorPos(match.offset + match.length)

  // Applique la correction à l'index exact sans casser le style HTML ni le texte
  editor.value.chain().focus()
    .deleteRange({ from: startPos, to: endPos })
    .insertContentAt(startPos, replacementValue)
    .run()

  closeSpellMenu()
  checkSpelling()
}

// Pour l'intégration AI Copilot (Remplacement de la méthode Quill)
const handleAiInsertion = (aiText: string) => {
  if (props.isHtml && editor.value) {
    // Insère le HTML ou le texte à la position du curseur
    editor.value.commands.insertContent(`<p>${aiText}</p>`)
  } else if (editor.value) {
    editor.value.commands.insertContent(`${aiText}`)
  }
}


// Nettoyage global
// onMounted(() => )
onUnmounted(() => {
  document.removeEventListener('click', closeSpellMenu)
  if (editor.value) editor.value.destroy()
}
)



</script>
<style scoped>
:deep(.tiptap-container) {
  width: 100%;
  height: 100%;
}

/* La zone de frappe réelle */
:deep(.ProseMirror) {
  width: 100%;
  min-height: 100%;
  /* Permet de cliquer n'importe où pour focus l'éditeur */
  padding: 1rem;
  outline: none;
  box-sizing: border-box;
  /* Empêche le padding de faire déborder la div */
  word-wrap: break-word;
  white-space: pre-wrap;
}

/* Base style HTML */
:deep(.ProseMirror p) {
  margin-bottom: 0.25em;
  font-size: medium;
}

:deep(.ProseMirror ul) {
  list-style-type: disc;
  padding-left: 1.5rem;
  margin-bottom: 0.75em;
}

:deep(.ProseMirror ol) {
  list-style-type: decimal;
  padding-left: 1.5rem;
  margin-bottom: 0.75em;
}

:deep(.ProseMirror h1) {
  font-size: xx-large;
}

:deep(.ProseMirror h2) {
  font-size: larger;
}

:deep(.ProseMirror h3) {
  font-size: medium;
}


/* Style TEXTE BRUT (si isHtml est faux) */
.is-plain-text :deep(.ProseMirror) {
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
  font-size: 10px;
  color: #374151;
  /* gray-700 */
}

/* Retire les marges de paragraphe en mode texte brut pour simuler un textarea */
.is-plain-text :deep(.ProseMirror p) {
  margin-bottom: 0;
}
</style>