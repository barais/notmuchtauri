<script setup lang="ts">
//import { defineProps, defineEmits } from 'vue';
import type { FolderNode } from '../types';
import FolderItem from './FolderItem.vue';

const props = defineProps<{
  nodes: FolderNode[];
  selectedPath: string | null;
}>();

const emit = defineEmits(['folder-selected']);

function selectFolder(path: string) {
  emit('folder-selected', path);
}
</script>

<template>
  <div class="flex flex-col py-2">
    <div v-for="node in nodes" :key="node.path" class="pl-1">
      <FolderItem
        :node="node"
        :selected-path="selectedPath"
        @select="selectFolder"
      />
    </div>
  </div>
</template>
