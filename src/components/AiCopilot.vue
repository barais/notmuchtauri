<template>
  <div 
    class="transition-all duration-300 ease-in-out flex border border-gray-300 rounded-md bg-gray-50 flex-shrink-0 overflow-hidden relative"
    :class="rootClasses"
  >
    <!-- VUE COLLAPSÉE (Fermée) -->
    <div 
      v-if="!isExpanded" 
      class="flex-1 flex items-center justify-center w-full h-full" 
      @click="isExpanded = true"
      title="Ouvrir l'assistant IA"
    >
      <div v-if="layout === 'vertical'" class="flex flex-col items-center py-4">
        <svg class="w-5 h-5 text-purple-600 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 3v4M3 5h4M6 17v4m-2-2h4m5-16l2.286 6.857L21 12l-5.714 2.143L13 21l-2.286-6.857L5 12l5.714-2.143L13 3z" />
        </svg>
        <span style="writing-mode: vertical-rl" class="text-xs text-gray-500 font-bold tracking-widest uppercase rotate-180">
          Assistant IA
        </span>
      </div>

      <div v-else class="flex flex-row items-center px-4 space-x-2">
        <svg class="w-5 h-5 text-purple-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 3v4M3 5h4M6 17v4m-2-2h4m5-16l2.286 6.857L21 12l-5.714 2.143L13 21l-2.286-6.857L5 12l5.714-2.143L13 3z" />
        </svg>
        <span class="text-xs text-gray-500 font-bold tracking-widest uppercase">
          Assistant IA
        </span>
      </div>
    </div>

    <!-- VUE EXPANDÉE (Ouverte) -->
    <div v-else class="flex-1 flex flex-col h-full w-full overflow-hidden">
      <!-- Header de l'IA -->
      <div class="flex justify-between items-center p-3 border-b border-gray-200 bg-white flex-shrink-0">
        <div class="flex items-center space-x-2 text-purple-600 font-medium text-sm">
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
          </svg>
          <span>Copilote IA</span>
        </div>
        <button @click="isExpanded = false" class="text-gray-400 hover:text-gray-600 focus:outline-none">
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" /></svg>
        </button>
      </div>

      <!-- Zone de Chat -->
      <div class="flex-1 overflow-y-auto p-3 space-y-4 text-sm bg-gray-50 min-h-0" ref="chatContainer">
        <div v-if="chatHistory.length === 0" class="text-center text-gray-400 italic mt-4">
          {{props.message}}
        </div>

        <div v-for="(msg, index) in chatHistory" :key="index" :class="msg.role === 'user' ? 'text-right' : 'text-left'">
          <div 
            :class="[
              'inline-block p-3 rounded-lg max-w-[95%] text-left shadow-sm', 
              msg.role === 'user' ? 'bg-purple-600 text-white rounded-br-none' : 'bg-white border border-gray-200 text-gray-800 rounded-bl-none'
            ]"
          >
            <template v-if="msg.role === 'user'">
              {{ msg.content }}
            </template>
            <div 
              v-else 
              v-html="renderMarkdown(msg.content)" 
              class="prose prose-sm max-w-none prose-purple dark:prose-invert prose-p:my-1.5 prose-ul:my-1.5 prose-ol:my-1.5 prose-li:my-0.5 prose-pre:my-2 prose-pre:bg-gray-800 prose-pre:text-gray-100 prose-pre:p-2 [&>*:first-child]:mt-0 [&>*:last-child]:mb-0"
            ></div>
          </div>          
          <div v-if="props.layout==='vertical' && msg.role === 'assistant'" class="mt-1 flex justify-start">
            <button 
              @click="$emit('insert-text', renderMarkdown(msg.content))" 
              class="text-xs text-purple-600 hover:text-purple-800 hover:underline flex items-center"
            >
              <svg class="w-3 h-3 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 5H6a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2v-1M8 5a2 2 0 002 2h2a2 2 0 002-2M8 5a2 2 0 012-2h2a2 2 0 012 2m0 0h2a2 2 0 012 2v3m2 4H10m0 0l3-3m-3 3l3 3" /></svg>
              Insérer dans l'email
            </button>
          </div>
        </div>

        <div v-if="isTyping" class="text-left">
          <div class="inline-block p-2 bg-white border border-gray-200 rounded-lg rounded-bl-none shadow-sm text-gray-400 text-xs italic flex space-x-1">
            <span class="animate-bounce">.</span><span class="animate-bounce delay-100">.</span><span class="animate-bounce delay-200">.</span>
          </div>
        </div>
      </div>

      <!-- Input de saisie et Auto-complétion -->
      <div class="p-2 border-t border-gray-200 bg-white flex-shrink-0 relative">
        
        <!-- MENU AUTO-COMPLÉTION -->
        <div 
          v-if="showSuggestions" 
          class="absolute bottom-full left-0 w-full mb-1 bg-white border border-gray-200 rounded-md shadow-lg z-50 max-h-48 overflow-y-auto"
        >
          <ul class="py-1">
            <li 
              v-for="(shortcut, index) in filteredShortcuts" 
              :key="index"
              @click="applyShortcut(shortcut)"
              @mouseover="selectedShortcutIndex = index"
              :class="[
                'px-3 py-2 cursor-pointer text-sm flex flex-col transition-colors', 
                selectedShortcutIndex === index ? 'bg-purple-100 text-purple-900' : 'text-gray-700 hover:bg-gray-50'
              ]"
            >
              <span class="font-bold text-xs">{{ shortcut.shortcut }}</span>
              <span class="text-xs text-gray-500 truncate">{{ shortcut.text }}</span>
            </li>
            <li v-if="filteredShortcuts.length === 0" class="px-3 py-2 text-sm text-gray-500 italic text-center">
              Aucun raccourci trouvé
            </li>
          </ul>
        </div>

        <!-- FORMULAIRE -->
        <form @submit.prevent="sendMessage" class="flex gap-1 relative">
          <input 
            ref="inputRef"
            v-model="userInput" 
            @keydown="handleKeydown"
            @input="handleInput"
            type="text" 
            placeholder="Demandez à l'IA... (Ctrl+Espace pour raccourcis)" 
            class="flex-1 text-sm p-2 border border-gray-300 rounded-md outline-none focus:border-purple-500 focus:ring-1 focus:ring-purple-500 transition-shadow pr-8"
            :disabled="isTyping"
          />
          <button 
            type="submit" 
            :disabled="!userInput.trim() || isTyping"
            class="absolute right-1.5 top-1.5 p-1 text-purple-600 hover:bg-purple-50 rounded disabled:opacity-50"
          >
            <svg class="w-4 h-4 transform rotate-90" fill="currentColor" viewBox="0 0 20 20"><path d="M10.894 2.553a1 1 0 00-1.788 0l-7 14a1 1 0 001.169 1.409l5-1.429A1 1 0 009 15.571V11a1 1 0 112 0v4.571a1 1 0 00.725.962l5 1.428a1 1 0 001.17-1.408l-7-14z" /></svg>
          </button>
        </form>

      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, nextTick, inject, computed, PropType } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { AppConfig } from '../types'
import { marked } from 'marked'
import DOMPurify from 'dompurify'

const props = defineProps({
  emailContext: { type: String, default: '' },
  layout: { type: String as PropType<'vertical' | 'horizontal'>, default: 'vertical' },
  message: { type: String, default: 'Comment puis-je vous aider à rédiger ce mail ?' },
})

const emit = defineEmits(['insert-text'])

const config: () => AppConfig = inject('appConfig')!;

const isExpanded = ref(false)
const userInput = ref('')
const isTyping = ref(false)
const chatContainer = ref<HTMLElement | null>(null)
const inputRef = ref<HTMLInputElement | null>(null)

interface ChatMessage {
  role: 'user' | 'assistant'
  content: string
}
const chatHistory = ref<ChatMessage[]>([])

// --- LOGIQUE AUTO-COMPLÉTION (Ctrl+Espace) ---

const showSuggestions = ref(false)
const selectedShortcutIndex = ref(0)
const currentTriggerWord = ref('')
const triggerWordStart = ref(0)

// Filtre les raccourcis en fonction du mot actuellement tapé
const filteredShortcuts = computed(() => {
  const shortcuts = config()?.shortcuts || [];
  if (!currentTriggerWord.value) return shortcuts;
  return shortcuts.filter(s => 
    s.shortcut.toLowerCase().includes(currentTriggerWord.value.toLowerCase()) || 
    s.text.toLowerCase().includes(currentTriggerWord.value.toLowerCase())
  );
})

const handleKeydown = (e: KeyboardEvent) => {
  // Détecte Ctrl+Espace
  if (e.ctrlKey && e.code === 'Space') {
    e.preventDefault();
    openSuggestions();
    return;
  }

  // Navigation dans la liste si elle est ouverte
  if (showSuggestions.value) {
    if (e.key === 'ArrowDown') {
      e.preventDefault();
      selectedShortcutIndex.value = (selectedShortcutIndex.value + 1) % filteredShortcuts.value.length;
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      selectedShortcutIndex.value = (selectedShortcutIndex.value - 1 + filteredShortcuts.value.length) % filteredShortcuts.value.length;
    } else if (e.key === 'Enter') {
      e.preventDefault(); // Empêche l'envoi du formulaire
      if (filteredShortcuts.value.length > 0) {
        applyShortcut(filteredShortcuts.value[selectedShortcutIndex.value]);
      }
    } else if (e.key === 'Escape') {
      showSuggestions.value = false;
    }
  }
}

const handleInput = () => {
  // Si le menu est ouvert, on met à jour la recherche en temps réel
  if (showSuggestions.value) {
    updateTriggerWord();
  }
}

const openSuggestions = () => {
  updateTriggerWord();
  selectedShortcutIndex.value = 0;
  showSuggestions.value = true;
}

const updateTriggerWord = () => {
  if (!inputRef.value) return;
  const cursorPosition = inputRef.value.selectionStart || 0;
  const textBeforeCursor = userInput.value.substring(0, cursorPosition);
  
  // Isole le mot courant avant le curseur (séparé par un espace)
  const words = textBeforeCursor.split(/\s+/);
  currentTriggerWord.value = words[words.length - 1] || '';
  triggerWordStart.value = cursorPosition - currentTriggerWord.value.length;
}

const applyShortcut = (shortcut: { shortcut: string, text: string }) => {
  const before = userInput.value.substring(0, triggerWordStart.value);
  const cursorPosition = inputRef.value?.selectionStart || 0;
  const after = userInput.value.substring(cursorPosition);
  
  // On remplace le mot courant par le texte complet du raccourci
  userInput.value = before + shortcut.text + after;
  showSuggestions.value = false;
  
  // On redonne le focus à l'input et on place le curseur à la fin du texte inséré
  nextTick(() => {
    if (inputRef.value) {
      inputRef.value.focus();
      const newPos = before.length + shortcut.text.length;
      inputRef.value.setSelectionRange(newPos, newPos);
    }
  });
}

// --- LOGIQUE GÉNÉRALE DU CHAT ---

const rootClasses = computed(() => {
  if (props.layout === 'vertical') {
    return isExpanded.value 
      ? 'w-80 ml-2 flex-col h-full' 
      : 'w-10 ml-2 flex-col h-full cursor-pointer hover:bg-gray-100'
  } else {
    return isExpanded.value 
      ? 'h-80 mt-2 flex-col w-full' 
      : 'h-10 mt-2 flex-row items-center cursor-pointer hover:bg-gray-100 w-full'
  }
})

const scrollToBottom = async () => {
  await nextTick()
  if (chatContainer.value) {
    chatContainer.value.scrollTop = chatContainer.value.scrollHeight
  }
}

const sendMessage = async () => {
  let text = userInput.value.trim()
  if (!text) return

  // CORRECTION : Remplacement de tous les raccourcis existants dans le texte final (au cas où l'utilisateur ne l'a pas fait via Ctrl+Espace)
  if (config() && config().shortcuts && config().shortcuts.length > 0) {
    for (const shortcut of config().shortcuts) {
      // split.join permet de faire un ReplaceAll dynamique sans RegEx
      text = text.split(shortcut.shortcut).join(shortcut.text);
    }
  }

  const llmConf = config()?.llm
  if (!llmConf?.api_key || !llmConf?.api_url) {
    alert("Veuillez configurer l'API LLM (URL et Clé API) dans les paramètres de l'application.")
    return
  }

  chatHistory.value.push({ role: 'user', content: text })
  userInput.value = ''
  showSuggestions.value = false // Cacher au cas où
  scrollToBottom()
  isTyping.value = true
  
  try {
    const aiResponse: string = await invoke('ask_llm', {
      prompt: text,
      context: props.emailContext,
      apiUrl: llmConf.api_url,
      apiKey: llmConf.api_key,
      model: llmConf.model || "gpt-3.5-turbo"
    })

    chatHistory.value.push({ role: 'assistant', content: aiResponse })
  } catch (error) {
    chatHistory.value.push({ 
      role: 'assistant', 
      content: `❌ Erreur de communication avec l'IA : ${error}` 
    })
  } finally {
    isTyping.value = false
    scrollToBottom()
  }
}

const renderMarkdown = (text: string) => {
  if (!text) return '';
  const rawHtml = marked.parse(text) as string;
  return DOMPurify.sanitize(rawHtml);
}
</script>