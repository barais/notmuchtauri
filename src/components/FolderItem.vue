<script setup lang="ts">
import { defineProps, defineEmits } from 'vue';
import type { FolderNode } from '../types';

const props = defineProps<{
  node: FolderNode;
  selectedPath: string | null;
}>();

const emit = defineEmits(['select']);

function handleClick(e: Event) {
  e.stopPropagation();
  emit('select', props.node.path);
}
</script>

<template>
  <div
    :class="['group cursor-pointer py-1 px-2 rounded text-sm transition-colors',
             selectedPath === node.path ? 'bg-blue-100 text-blue-700 dark:bg-blue-900/30 dark:text-blue-300' : 'hover:bg-gray-100 dark:hover:bg-zinc-800']"
    @click="handleClick"
  >
    <div class="flex items-center gap-2">
      <span class="text-gray-400">{{ node.is_maildir ? '📁' : '📂' }}</span>
      <span class="truncate">{{ node.name }}</span>
    </div>

    <!-- Recursively render children if it's not a maildir -->
    <div v-if="node.children && node.children.length > 0" class="pl-4 mt-1 border-l border-gray-200 dark:border-zinc-800">
      <FolderItem
        v-for="child in node.children"
        :key="child.path"
        :node="child"
        :selected-path="selectedPath"
        @select="emit('select', $event)"
      />
    </div>
  </div>
</template>
