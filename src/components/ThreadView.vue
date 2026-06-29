<template>
  <div class="max-w-8xl mx-auto p-2 space-y-8">
    <div v-for="(thread, threadIndex) in threads" :key="threadIndex" class="space-y-4">
      <h2 class="text-xl font-bold text-gray-800 border-b pb-2 mb-4">
        Sujet : {{ thread.roots[0]?.subject || 'Sans objet' }}
      </h2>
                          <!-- INSÉRER ICI VOTRE COMPOSANT AI Copilot -->
        <AiCopilot v-if="config()?.llm && config().llm?.api_key && 
        config().llm?.api_url && config().llm?.model"  
        layout="horizontal"
        message="Comment puis-je vous aider à comprendre ce thread ?"
        :email-context="formatThreadsForLlm(props.threads)"  />

      
      <!-- Affichage de la racine du thread -->
      <MessageNode 
        v-for="(rootMsg, index) in thread.roots" 
        :key="rootMsg.id" 
        :message="rootMsg"
        :depth="0"
        :initially-expanded="threadIndex === 0 && index === 0" 
      />
    </div>


  </div>
</template>

<script setup lang="ts">
import { inject, PropType } from 'vue'
import MessageNode from './MessageNode.vue'
// Assurez-vous d'importer vos interfaces ici
import type { AppConfig, MessageDto, ThreadDto } from '../types' 
import AiCopilot from './AiCopilot.vue'

const props  =defineProps({
  threads: {
    type: Array as PropType<ThreadDto[]>,
    required: true
  }
})
const config:  ()=>AppConfig = inject(/* key */ 'appConfig')!



const formatThreadsForLlm: (threads: ThreadDto[])=> string =  (threads: ThreadDto[]) =>{
  const output: string[] = [];

  // Fonction utilitaire pour nettoyer le HTML
  const stripHtml = (html: string): string => {
    if (!html) return '';
    const doc = new DOMParser().parseFromString(html, 'text/html');
    // textContent extrait le texte pur sans les balises
    return doc.body.textContent || doc.body.innerText || '';
  };

  // Fonction récursive pour traiter un message et ses réponses
  const processMessage = (message: MessageDto) => {
    // 1. Extraction du corps du texte (Priorité au textBody, fallback sur htmlBody converti)
    let bodyText = '';
    if (message.textBody && message.textBody.trim() !== '') {
      bodyText = message.textBody.trim();
    } else if (message.htmlBody) {
      bodyText = stripHtml(message.htmlBody).trim();
    }

    // 2. Formatage du message avec des séparateurs clairs pour le LLM
    const formattedMessage = `
--- DÉBUT DU MESSAGE ---
Sujet : ${message.subject || 'Sans objet'}
De : ${message.from}
À : ${message.to}
Date : ${message.date}

${bodyText}
--- FIN DU MESSAGE ---
`.trim();

    output.push(formattedMessage);

    // 3. Appel récursif pour traiter toutes les réponses de ce message
    if (message.replies && message.replies.length > 0) {
      for (const reply of message.replies) {
        processMessage(reply);
      }
    }
  };

  // Boucle principale sur tous les threads et leurs racines
  for (const thread of threads) {
    for (const rootMessage of thread.roots) {
      processMessage(rootMessage);
    }
  }


  // Concatène tous les messages séparés par 2 sauts de ligne
  return output.join('\n\n');
}

</script>