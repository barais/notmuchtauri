<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { Message, FolderNode, AppConfig } from "./types";
import MailDetail from "./components/MailDetail.vue";
import FolderTree from "./components/FolderTree.vue";

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

async function initApp() {
  try {
    // 1. Load configuration
    const config = await invoke<AppConfig>('get_config');
    rootPath.value = config.root_mail_dir;

    // 2. Initial scan if root path exists
    if (rootPath.value) {
      await scanFolders(rootPath.value);
      currentPath.value = config.default_path ;
      await search();

    }
  } catch (e) {
    console.error("Init error:", e);
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

async function search(limit: number = 1000, sort: string = "newest-first", customQuery: string = "") {
  isLoading.value = true;
  error.value = null;

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
    isLoading.value = false;
  }
}

function selectMessage(id: string) {
  selectedMessageId.value = id;
}

function onFolderSelected(path: string) {
  currentPath.value = path;
  search();
}

onMounted(() => {
  initApp();
});
</script>

<template>
  <div class="flex h-screen w-screen overflow-hidden bg-gray-100 dark:bg-zinc-900 text-gray-900 dark:text-zinc-100">

    <!-- Volet 1: Navigation & Filtres -->
    <aside class="w-64 flex-shrink-0 border-r border-gray-200 dark:border-zinc-800 bg-white dark:bg-zinc-950 flex flex-col">
      <!-- Search Section (NOW AT THE TOP) -->
      <div class="p-4 border-b border-gray-200 dark:border-zinc-800 bg-white dark:bg-zinc-950">
        <div class="text-xs font-semibold text-gray-500 uppercase tracking-wider mb-2">Search</div>
        <div class="flex gap-2">
          <input
            v-model="searchQuery"
            @keyup.enter="search()"
            placeholder="Search messages..."
            class="w-full px-3 py-2 text-sm border rounded-md bg-gray-50 dark:bg-zinc-900 border-gray-300 dark:border-zinc-700 focus:ring-2 focus:ring-blue-500 outline-none"
          />
          <button @click="search()" class="px-3 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 text-sm transition-colors">
            Search
          </button>
        </div>
      </div>

      <!-- Folder Tree Section (NOW AT THE BOTTOM) -->
      <div class="flex-1 overflow-hidden flex flex-col">
        <div class="p-3 bg-gray-50 dark:bg-zinc-900 border-b border-gray-200 dark:border-zinc-800">
           <div class="text-xs font-semibold text-gray-500 uppercase mb-2">Mail Root</div>
           <div class="flex gap-2">
             <input
               v-model="rootPath"
               @keyup.enter="scanFolders(rootPath)"
               placeholder="/home/user/mail"
               class="w-full px-2 py-1 text-xs border rounded bg-white dark:bg-zinc-800 border-gray-300 dark:border-zinc-700 outline-none"
             />
             <button @click="scanFolders(rootPath)" class="px-2 py-1 bg-gray-200 dark:bg-zinc-700 text-xs rounded hover:bg-gray-300 dark:hover:bg-zinc-600">
               Scan
             </button>
           </div>
        </div>
        <div class="flex-1 overflow-y-auto">
          <FolderTree
            :nodes="folderTree"
            :selected-path="currentPath"
            @folder-selected="onFolderSelected"
          />
          <div v-if="isScanning" class="p-4 text-center text-xs text-gray-500">Scanning folders...</div>
          <div v-else-if="error" class="p-4 text-center text-xs text-red-500">{{ error }}</div>
        </div>
      </div>
    </aside>

    <!-- Volet 2: Liste des Threads -->
    <section class="flex-1 flex flex-col min-w-0 bg-white dark:bg-zinc-900">
      <div class="p-3 border-b border-gray-200 dark:border-zinc-800 flex justify-between items-center bg-gray-50 dark:bg-zinc-900/50">
        <span class="text-sm font-medium">{{ messages.length }} results found</span>
        <div class="flex gap-2">
          <button class="text-xs px-2 py-1 border rounded hover:bg-gray-100 dark:hover:bg-zinc-800 transition-colors">Select All</button>
          <button class="text-xs px-2 py-1 bg-red-100 text-red-600 rounded hover:bg-red-200 transition-colors">Delete</button>
        </div>
      </div>

      <div class="flex-1 overflow-y-auto">
        <div v-if="isLoading" class="p-8 text-center text-gray-500">Searching...</div>
        <div v-else-if="error" class="p-8 text-center text-red-500">{{ error }}</div>
        <ul v-else class="divide-y divide-gray-100 dark:divide-zinc-800">
          <li
            v-for="msg in messages"
            :key="msg.id"
            :class="['p-4 cursor-pointer transition-colors hover:bg-gray-50 dark:hover:bg-zinc-800',
                     selectedMessageId === msg.id ? 'bg-blue-50 dark:bg-blue-900/20 border-l-4 border-blue-500' : '']"
            @dblclick="selectMessage(msg.id)"
          >
            <div class="flex justify-between items-start mb-1">
              <div class="font-semibold truncate text-sm">{{ msg.subject }}</div>
              <div class="text-xs text-gray-400">{{ msg.date }}</div>
            </div>
            <div class="text-xs text-gray-500 truncate">{{ msg.from }}</div>
          </li>
        </ul>
        <div v-if="messages.length === 0 && !isLoading" class="p-8 text-center text-gray-400">
          No messages found in this folder.
        </div>
      </div>
    </section>

    <!-- Volet 3: Vue du Thread & Message -->
    <main class="flex-1 flex flex-col bg-white dark:bg-zinc-950 border-l border-gray-200 dark:border-zinc-800 overflow-hidden">
      <div v-if="selectedMessageId" class="h-full overflow-hidden">
        <MailDetail :message-id="selectedMessageId" />
      </div>
      <div v-else class="h-full flex items-center justify-center text-gray-400 italic p-8 text-center">
        Double-click a thread to read its messages
      </div>
    </main>
  </div>
</template>
