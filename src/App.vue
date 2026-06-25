<script setup lang="ts">
import { ref, onMounted, onUnmounted, provide } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { Message, FolderNode, AppConfig, MessageDto } from "./types";
import MailDetail from "./components/MailDetail.vue";
import FolderTree from "./components/FolderTree.vue";
import ComposeView from './components/ComposeView.vue';
import SettingsModal from './components/SettingsModal.vue'

const searchQuery = ref("");
const messages = ref<Message[]>([]);
const selectedMessageId = ref<string | null>(null);
const isLoading = ref(false);
const error = ref<string | null>(null);
const currentPath = ref<string | null>(null);

// Configuration & Folder state
const rootPath = ref("");
const folderTree = ref<FolderNode[]>([]);
const isScanning = ref(false);

const isMailRootExpanded = ref(false); // Collapsé par défaut


// --- Logique de Redimensionnement (Splitters) ---
const pane1Width = ref(256); // w-64 correspond à 256px
const pane2Width = ref(512); // Largeur initiale pour la liste des messages
const isDragging = ref<'pane1' | 'pane2' | null>(null);
const config = ref<AppConfig>();

function startDrag(pane: 'pane1' | 'pane2') {
  isDragging.value = pane;
  document.addEventListener("mousemove", onDrag);
  document.addEventListener("mouseup", stopDrag);

  // Amélioration UX : Change le curseur global et empêche la sélection de texte pendant le redimensionnement
  document.body.style.cursor = "col-resize";
  document.body.style.userSelect = "none";
}

function onDrag(e: MouseEvent) {
  if (isDragging.value === 'pane1') {
    // Largeur min 150px, max 500px pour le volet 1
    const newWidth = Math.max(150, Math.min(e.clientX, 500));
    pane1Width.value = newWidth;
  } else if (isDragging.value === 'pane2') {
    // La largeur du volet 2 dépend de la position de la souris moins la largeur du volet 1
    const newWidth = Math.max(200, Math.min(e.clientX - pane1Width.value, 800));
    pane2Width.value = newWidth;
  }
}

function stopDrag() {
  isDragging.value = null;
  document.removeEventListener("mousemove", onDrag);
  document.removeEventListener("mouseup", stopDrag);

  // Restaure le curseur normal
  document.body.style.cursor = "";
  document.body.style.userSelect = "";
}

onUnmounted(() => {
  stopDrag(); // Nettoyage au cas où
});


const getConfig:()=> AppConfig|undefined = ()=> {
  return config.value;
}

provide('appConfig',getConfig);


async function initApp() {
  try {
    // 1. Load configuration
    const config1 = await invoke<AppConfig>('get_config');
    config.value = config1;
    rootPath.value = config.value.root_mail_dir;

    // 2. Initial scan if root path exists
    if (rootPath.value) {
      await scanFolders(rootPath.value);
      currentPath.value = config.value.default_path;
      await search();

    }
  } catch (e) {
    error.value = "Failed to initialize app configuration";
  }
}

async function scanFolders(path: string) {
  isScanning.value = true;
  try {
    folderTree.value = await invoke<FolderNode[]>('scan_mail_folders', { rootPath: path });
  } catch (e) {
    error.value = "Failed to scan folders: " + e;
  } finally {
    isScanning.value = false;
  }
}

function getRelativePath(absolutePath: string) {
  if (!rootPath.value) return absolutePath;

  // Remove the root path from the absolute path
  let relative = absolutePath.replace(rootPath.value, "");

  // Remove leading slash if present
  if (relative.startsWith("/")) {
    relative = relative.substring(1);
  }

  return relative;
}

async function search(sort: string = "newest-first", customQuery: string = "") {
  isLoading.value = true;
  error.value = null;
  const limit = config.value?.limit;
  let finalQuery = customQuery || searchQuery.value || "";

  if (currentPath.value) {
    const relativePath = getRelativePath(currentPath.value);

    if (customQuery || searchQuery.value) {
      finalQuery = `path:"${relativePath}/**" ${finalQuery}`;
    } else {
      finalQuery = `path:"${relativePath}/**"`;
    }
  }

  try {
    messages.value = await invoke<Message[]>("search_messages", {
      query: finalQuery,
      limit,
      sort
    });
  } catch (e) {
    error.value = e instanceof Error ? e.message : String(e);
  } finally {
    resetAutoSearchTimer();
    isLoading.value = false;
  }
}

function selectMessage(id: string) {

  //  selectedMessageId.value = null; // Reset to trigger re-render

  selectedMessageId.value = id;
  openThread(id);
}


function onFolderSelected(path: string) {
  currentPath.value = path;
  search();
}

onMounted(() => {
  initApp();
});

/*function removetag(messageId: string) {
  messages.value.forEach((msg) => {
    if (msg.id === messageId) {
      msg.tags = msg.tags.filter(tag => tag.toLowerCase() !== 'unread');
    }
  });
}*/

interface Tab {
  id: string;
  type: 'VIEW' | 'COMPOSE';
  replyMode: 'reply' | 'reply-all' | 'forward' | 'new' | 'editasnew' |'none';
  title: string;
  threadId?: string; // For VIEW
  originalMessageId?: string; // For COMPOSE (to know who we are replying to)
  isHtml: boolean,
  message: MessageDto | null
}

const openTabs = ref<Tab[]>([]);
const activeTabId = ref<string | null>(null);

function openThread(id: string) {
  // Check if already open
  const existing = openTabs.value.find(t => t.type === 'VIEW' && t.threadId === id);
  if (existing) {
    activeTabId.value = existing.id;
    return;
  }

  const newId = crypto.randomUUID();
  openTabs.value.push({
    id: newId,
    type: 'VIEW',
    replyMode: 'none',
    title: `Thread ${id.substring(0, 8)}`,
    threadId: id,
    isHtml: true,
    message: null
  });
  activeTabId.value = newId;
}



const openReply = (message: MessageDto, subject: string, replyMode: 'reply' | 'reply-all' | 'forward' | 'editasnew'| 'new' | 'none'
) => {
  const newId = crypto.randomUUID();

  openTabs.value.push({
    id: newId,
    type: 'COMPOSE',
    replyMode,
    title: replyMode === 'reply' ? 'reply' : replyMode === 'reply-all' ? 'reply-all' : 'forward' + `: ${subject}`,
    originalMessageId: message.id,
    isHtml: message.htmlBody !== null && message.htmlBody !== '',
    message: message
  });

  activeTabId.value = newId;
}
provide(/* key */ 'replyFunction', /* value */ openReply)


function openTabsNew() {
  const newId = crypto.randomUUID();
  openTabs.value.push({
    id: newId,
    replyMode: 'new',
    type: 'COMPOSE',
    title: `Nouveau message`,
    originalMessageId: '',
    isHtml: true,
    message: null

  });
  activeTabId.value = newId;
}


const closeTab = (id: string): void => {
  console.error("Closing tab with id:", id);
  openTabs.value = openTabs.value.filter(t => t.id !== id);
  if (activeTabId.value === id) {
    activeTabId.value = openTabs.value.length ? openTabs.value[0].id : null;
  }
}

// Quand l'utilisateur a cliqué sur "Enregistrer" dans la modale
function onConfigUpdated(newConfig: AppConfig) {
  config.value = newConfig;
  // Optionnel : Relancer un scanFolders ou recharger la racine si root_mail_dir a changé
}

const selectedIds = ref<Set<string>>(new Set());
const lastSelectedIndex = ref<number | null>(null);

// Pour le bouton "Select All"
const selectAll = () => {
  selectedIds.value = new Set(messages.value.map(m => m.id));
};

// Fonction de clic gérant Shift et Ctrl/Cmd
const handleRowClick = (event: MouseEvent, msg: any, index: number) => {
  const newSelection = new Set(selectedIds.value);

  if (event.shiftKey && lastSelectedIndex.value !== null) {
    // SÉLECTION PAR PLAGE (Shift)
    newSelection.clear(); // Optionnel : on efface la sélection précédente pour ne garder que la plage

    const start = Math.min(index, lastSelectedIndex.value);
    const end = Math.max(index, lastSelectedIndex.value);

    for (let i = start; i <= end; i++) {
      newSelection.add(messages.value[i].id);
    }
  }
  else if (event.ctrlKey || event.metaKey) {
    // SÉLECTION UNITAIRE (Ctrl ou Cmd)
    if (newSelection.has(msg.id)) {
      newSelection.delete(msg.id);
    } else {
      newSelection.add(msg.id);
    }
    lastSelectedIndex.value = index;
  }
  else {
    // CLIC NORMAL (Sélection unique)
    newSelection.clear();
    newSelection.add(msg.id);
    lastSelectedIndex.value = index;

    // On met à jour l'ID du message à lire dans le volet 3
    selectedMessageId.value = msg.id;
  }

  // On remplace le Set pour déclencher la réactivité Vue
  selectedIds.value = newSelection;
};

// --- Logique du Menu Contextuel ---
const contextMenu = ref({
  show: false,
  x: 0,
  y: 0,
  targetTabId: null as string | null
})

// Ouvre le menu au clic droit
const openContextMenu = (event: MouseEvent, tabId: string) => {
  contextMenu.value = {
    show: true,
    x: event.clientX,
    y: event.clientY,
    targetTabId: tabId
  }
}

// Ferme le menu
const closeContextMenu = () => {
  contextMenu.value.show = false
}

// Écouteur global pour fermer le menu quand on clique ailleurs
onMounted(() => document.addEventListener('click', closeContextMenu))
onUnmounted(() => document.removeEventListener('click', closeContextMenu))

// --- Actions de fermeture par lot ---

const closeAllTabs = () => {
  openTabs.value = []
  activeTabId.value = null
}

const closeTabsToRight = (targetId: string) => {
  const index = openTabs.value.findIndex(t => t.id === targetId)
  if (index !== -1) {
    // On garde uniquement les éléments de 0 jusqu'à l'index inclus
    openTabs.value = openTabs.value.slice(0, index + 1)
    
    // Si l'onglet actif faisait partie de ceux supprimés, on rend le targetId actif
    if (!openTabs.value.find(t => t.id === activeTabId.value)) {
      activeTabId.value = targetId
    }
  }
}

const closeTabsToLeft = (targetId: string) => {
  const index = openTabs.value.findIndex(t => t.id === targetId)
  if (index > 0) {
    // On garde uniquement les éléments à partir de l'index
    openTabs.value = openTabs.value.slice(index)
    
    // Si l'onglet actif faisait partie de ceux supprimés, on rend le targetId actif
    if (!openTabs.value.find(t => t.id === activeTabId.value)) {
      activeTabId.value = targetId
    }
  }
}
// Variable pour stocker le timer
let autoSearchTimer: ReturnType<typeof setTimeout> | null = null;

// Fonction qui annule l'ancien timer et en crée un nouveau de 2 minutes (120 000 ms)
function resetAutoSearchTimer() {
  if (autoSearchTimer) {
    clearTimeout(autoSearchTimer);
  }
  
  autoSearchTimer = setTimeout(() => {
    // On relance la recherche sans bloquer l'UI
    search();
  }, 120_000); 
}

// Nettoyage à la fermeture
onUnmounted(() => {
  if (autoSearchTimer) clearTimeout(autoSearchTimer);
});

</script>

<template>
  <div class="flex h-screen w-screen overflow-hidden bg-gray-100 dark:bg-zinc-900 text-gray-900 dark:text-zinc-100">

    <!-- Volet 1: Navigation & Filtres -->
    <aside class="flex-shrink-0 border-r border-gray-200 dark:border-zinc-800 bg-white dark:bg-zinc-950 flex flex-col"
      :style="{ width: pane1Width + 'px' }">
      <!-- Search Section (NOW AT THE TOP) -->
      <div class="p-4 border-b border-gray-200 dark:border-zinc-800 bg-white dark:bg-zinc-950">
        <div class="text-xs font-semibold text-gray-500 uppercase tracking-wider mb-2">Search</div>
        <div class="flex gap-2">
          <input v-model="searchQuery" @keyup.enter="search()" placeholder="Search messages..."
            class="w-full px-3 py-2 text-sm border rounded-md bg-gray-50 dark:bg-zinc-900 border-gray-300 dark:border-zinc-700 focus:ring-2 focus:ring-blue-500 outline-none" />
          <button @click="search()"
            class="px-3 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 text-sm transition-colors">
            Search
          </button>
        </div>
      </div>

      <!-- Folder Tree Section (NOW AT THE BOTTOM) -->
      <div class="flex-1 overflow-hidden flex flex-col">
        <div class="bg-gray-50 dark:bg-zinc-900 border-b border-gray-200 dark:border-zinc-800">
          <!-- Header Cliquable -->
          <button @click="isMailRootExpanded = !isMailRootExpanded"
            class="w-full flex items-center justify-between p-3 focus:outline-none hover:bg-gray-100 dark:hover:bg-zinc-800 transition-colors">
            <span class="text-xs font-semibold text-gray-500 uppercase">Mail Root</span>
            <!-- Icône chevron animée -->
            <svg class="w-4 h-4 text-gray-500 transform transition-transform duration-200"
              :class="isMailRootExpanded ? 'rotate-180' : ''" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
            </svg>
          </button>

          <!-- Contenu collapsable -->
          <div v-if="isMailRootExpanded" class="px-3 pb-3">
            <div class="flex gap-2">
              <input v-model="rootPath" placeholder="/home/user/mail"
                class="w-full px-2 py-1 text-xs border rounded bg-white dark:bg-zinc-800 border-gray-300 dark:border-zinc-700 outline-none focus:ring-1 focus:ring-blue-500" />
              <button @click="scanFolders(rootPath)"
                class="px-2 py-1 bg-gray-200 dark:bg-zinc-700 text-xs rounded hover:bg-gray-300 dark:hover:bg-zinc-600 transition-colors">
                Scan
              </button>
            </div>
          </div>
        </div>
        <!-- <ComposeEmailModal/>-->

        <div class="flex gap-2">
          <button @click="openTabsNew"
            class="flex items-center gap-2 bg-blue-600 text-white px-4 py-2 rounded-lg hover:bg-blue-700 transition-colors shadow-sm font-medium">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
              <path d="M17.414 2.586a2 2 0 00-2.828 0L7 10.172V13h2.828l7.586-7.586a2 2 0 000-2.828z" />
              <path fill-rule="evenodd"
                d="M2 6a2 2 0 012-2h4a1 1 0 010 2H4v10h10v-4a1 1 0 112 0v4a2 2 0 01-2 2H4a2 2 0 01-2-2V6z"
                clip-rule="evenodd" />
            </svg>
            Nouveau message
          </button>
          <SettingsModal v-if="config" :config="config" @config-saved="onConfigUpdated" />
        </div>

        <div class="flex-1 overflow-y-auto">
          <FolderTree :nodes="folderTree" :selected-path="currentPath" @folder-selected="onFolderSelected" />
          <div v-if="isScanning" class="p-4 text-center text-xs text-gray-500">Scanning folders...</div>
          <div v-else-if="error" class="p-4 text-center text-xs text-red-500">{{ error }}</div>
        </div>
      </div>
    </aside>

    <div
      class="w-1 bg-gray-200 dark:bg-zinc-800 hover:bg-blue-500 hover:w-1 cursor-col-resize z-10 transition-colors flex-shrink-0"
      @mousedown="startDrag('pane1')"></div>

    <!-- Volet 2: Liste des Threads -->
    <section class="flex-shrink-0 flex flex-col bg-white dark:bg-zinc-900" :style="{ width: pane2Width + 'px' }">
      <!-- Header de la liste -->
      <div
        class="p-3 border-b border-gray-200 dark:border-zinc-800 flex justify-between items-center bg-gray-50 dark:bg-zinc-900/50">
        <span class="text-sm font-medium">{{ messages.length }} results</span>
 <div class="flex gap-2">
          <!-- NOUVEAU : Bouton Rafraîchir / Forcer la recherche -->
          <button 
            @click="search()"
            class="text-xs px-2 py-1 flex items-center gap-1 border rounded hover:bg-gray-100 dark:hover:bg-zinc-800 transition-colors text-gray-700 dark:text-zinc-300"
            title="Rafraîchir les messages"
            :disabled="isLoading"
            :class="{'opacity-50 cursor-not-allowed': isLoading}"
          >
            <!-- Icône de synchronisation/rafraîchissement qui tourne quand isLoading est true -->
            <svg 
              class="w-3.5 h-3.5" 
              :class="{'animate-spin': isLoading}"
              fill="none" stroke="currentColor" viewBox="0 0 24 24"
            >
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
            </svg>
            Refresh
          </button>

          <!-- Câblage du Select All -->
          <button @click="selectAll"
            class="text-xs px-2 py-1 border rounded hover:bg-gray-100 dark:hover:bg-zinc-800 transition-colors">
            Select All
          </button>

          <!-- Bouton Delete -->
          <button class="text-xs px-2 py-1 bg-red-100 text-red-600 rounded hover:bg-red-200 transition-colors">
            Delete
            <!-- Afficher le nombre d'éléments à supprimer -->
            <span v-if="selectedIds.size > 0">({{ selectedIds.size }})</span>
          </button>
        </div>
           </div>

      <div class="flex-1 overflow-y-auto relative">
        <div v-if="isLoading" class="p-8 text-center text-gray-500">Searching...</div>
        <div v-else-if="error" class="p-8 text-center text-red-500">{{ error }}</div>

        <ul v-else class="divide-y divide-gray-100 dark:divide-zinc-800">
          <li v-for="(msg, index) in messages" :key="msg.id" @click="handleRowClick($event, msg, index)"
            @dblclick="selectMessage(msg.id)"
            class="relative p-4 pl-6 cursor-pointer transition-colors border-l-4 group select-none" :class="[
              // 1. STYLE SÉLECTIONNÉ
              selectedIds.has(msg.id)
                ? 'bg-blue-100 dark:bg-blue-900/40 border-blue-500'

                // 2. STYLE NON LU
                : msg.tags.includes('unread')
                  ? 'bg-white dark:bg-zinc-800/80 border-transparent hover:bg-gray-50 dark:hover:bg-zinc-700'

                  // 3. STYLE LU
                  : 'bg-gray-50/50 dark:bg-zinc-900 border-transparent hover:bg-gray-100 dark:hover:bg-zinc-800 text-gray-500 dark:text-gray-400'
            ]">
            <!-- Indicateur Point Bleu (Unread) -->
            <div v-if="msg.tags.includes('unread') && !selectedIds.has(msg.id)"
              class="absolute left-1.5 top-1/2 -translate-y-1/2 w-2 h-2 bg-blue-500 rounded-full"></div>

            <div class="flex justify-between items-start mb-1 gap-2">
              <div class="truncate text-sm flex-1"
                :class="msg.tags.includes('unread') ? 'font-bold text-gray-900 dark:text-zinc-100' : 'font-medium'">
                {{ msg.subject }}
              </div>

              <div class="text-xs whitespace-nowrap"
                :class="msg.tags.includes('unread') ? 'text-blue-600 dark:text-blue-400 font-semibold' : 'text-gray-400 dark:text-zinc-500'">
                {{ msg.date }}
              </div>
            </div>

            <div class="text-xs truncate"
              :class="msg.tags.includes('unread') ? 'text-gray-700 dark:text-zinc-300' : 'text-gray-400 dark:text-zinc-500'">
              {{ msg.from }}
            </div>
          </li>
        </ul>

        <div v-if="messages.length === 0 && !isLoading" class="p-8 text-center text-gray-400">
          No messages found in this folder.
        </div>
      </div>
    </section>

    <!-- Splitter 2 -->
    <div
      class="w-1 bg-gray-200 dark:bg-zinc-800 hover:bg-blue-500 hover:w-1 cursor-col-resize z-10 transition-colors flex-shrink-0"
      @mousedown="startDrag('pane2')"></div>

    <!-- Volet 3: Vue du Thread & Message -->
     <main class="flex-1 min-w-[300px] flex flex-col bg-white dark:bg-zinc-950 border-l border-gray-200 dark:border-zinc-800 overflow-hidden relative">

      <!-- Tab Bar -->
      <div v-if="openTabs.length" class="flex overflow-x-auto bg-gray-100 dark:bg-zinc-900 border-b border-gray-200 dark:border-zinc-800">
        <!-- 
          AJOUT : @contextmenu.prevent pour intercepter le clic droit 
        -->
        <div 
          v-for="tab in openTabs" 
          :key="tab.id" 
          @click="activeTabId = tab.id"
          @contextmenu.prevent="openContextMenu($event, tab.id)"
          :class="['flex items-center px-3 py-2 text-xs cursor-pointer border-r border-gray-200 dark:border-zinc-800 transition-colors min-w-fit select-none',
            activeTabId === tab.id ? 'bg-white dark:bg-zinc-950 text-blue-600' : 'text-gray-500 hover:bg-gray-200 dark:hover:bg-zinc-800']"
        >
          <span class="truncate max-w-[150px]">{{ tab.title }}</span>
          <button @click.stop="closeTab(tab.id)" class="ml-2 hover:text-red-500">×</button>
        </div>
      </div>

      <!-- Content Area -->
      <div class="flex-1 overflow-hidden">
        <template v-if="openTabs.length > 0">
          <!-- 
            MODIFICATION CRUCIALE : on itère sur TOUS les onglets, 
            et on utilise v-show pour cacher ceux qui sont inactifs.
            Les composants ne sont plus détruits !
          -->
          <div 
            v-for="tab in openTabs" 
            :key="tab.id" 
            v-show="tab.id === activeTabId" 
            class="h-full"
          >
            <!-- Gardez vos props telles quelles -->
            <MailDetail 
              v-if="tab.type === 'VIEW'" 
              :message-id="tab.threadId!" 
            />
            <ComposeView 
              v-else 
              :config="config" 
              @close="closeTab" 
              :message="tab.message" 
              :reply-mode="tab.replyMode"
              :isHtml="tab.isHtml" 
              :message-id="tab.originalMessageId!" 
              :tabs-id="tab.id" 
            />
          </div>
        </template>
        <div v-else class="h-full flex items-center justify-center text-gray-400 italic p-8 text-center">
          Double-click a thread to read its messages
        </div>
      </div>

      <!-- Menu Contextuel (Téléporté au niveau du body pour éviter d'être coupé par overflow-hidden) -->
      <Teleport to="body">
        <div 
          v-if="contextMenu.show" 
          :style="{ top: contextMenu.y + 'px', left: contextMenu.x + 'px' }" 
          class="fixed z-[100] w-48 py-1 bg-white dark:bg-zinc-800 border border-gray-200 dark:border-zinc-700 rounded-md shadow-xl text-sm"
          @contextmenu.prevent
        >
          <button 
            @click="closeAllTabs" 
            class="w-full text-left px-4 py-2 text-gray-700 dark:text-zinc-200 hover:bg-gray-100 dark:hover:bg-zinc-700 transition-colors"
          >
            Fermer tout
          </button>
          
          <div class="h-px bg-gray-200 dark:bg-zinc-700 my-1"></div>
          
          <button 
            @click="closeTabsToRight(contextMenu.targetTabId!)" 
            class="w-full text-left px-4 py-2 text-gray-700 dark:text-zinc-200 hover:bg-gray-100 dark:hover:bg-zinc-700 transition-colors"
          >
            Fermer les onglets à droite
          </button>
          
          <button 
            @click="closeTabsToLeft(contextMenu.targetTabId!)" 
            class="w-full text-left px-4 py-2 text-gray-700 dark:text-zinc-200 hover:bg-gray-100 dark:hover:bg-zinc-700 transition-colors"
          >
            Fermer les onglets à gauche
          </button>
        </div>
      </Teleport>

    </main>
  </div>
</template>
