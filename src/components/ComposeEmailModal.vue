<template>
  <div>
    <!-- Bouton pour ouvrir la modale -->
    <button 
      @click="openModal" 
      class="flex items-center gap-2 bg-blue-600 text-white px-4 py-2 rounded-lg hover:bg-blue-700 transition-colors shadow-sm font-medium"
    >
      <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
        <path d="M17.414 2.586a2 2 0 00-2.828 0L7 10.172V13h2.828l7.586-7.586a2 2 0 000-2.828z" />
        <path fill-rule="evenodd" d="M2 6a2 2 0 012-2h4a1 1 0 010 2H4v10h10v-4a1 1 0 112 0v4a2 2 0 01-2 2H4a2 2 0 01-2-2V6z" clip-rule="evenodd" />
      </svg>
      Nouveau message
    </button>

    <!-- Modale (Teleportée à la racine pour éviter les soucis de z-index) -->
    <Teleport to="body">
      <div v-if="isOpen" class="fixed inset-0 z-50 flex items-center justify-center bg-gray-900/50 backdrop-blur-sm p-4">
        
        <!-- Fenêtre de la modale -->
        <div class="bg-white rounded-xl shadow-2xl w-full max-w-3xl flex flex-col max-h-[90vh] overflow-hidden">
          
          <!-- Header -->
          <div class="px-6 py-4 border-b border-gray-200 flex justify-between items-center bg-gray-50">
            <h2 class="text-lg font-semibold text-gray-800">Nouveau message</h2>
            <button @click="closeModal" class="text-gray-500 hover:text-gray-700 hover:bg-gray-200 p-1 rounded transition-colors">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </div>

          <!-- Formulaire scrollable -->
          <div class="flex-1 overflow-y-auto p-6 space-y-4">
            
            <!-- Compte d'expédition -->
            <div class="flex items-center gap-4">
              <label class="w-16 text-right text-sm font-medium text-gray-600">De :</label>
              <select v-model="form.account" class="flex-1 border border-gray-300 rounded-md px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent">
                <option v-for="acc in availableAccounts" :key="acc.id" :value="acc.id">
                  {{ acc.label }} ({{ acc.email }})
                </option>
              </select>
            </div>

            <!-- Destinataire (To) -->
            <div class="flex items-center gap-4">
              <label class="w-16 text-right text-sm font-medium text-gray-600">À :</label>
              <div class="flex-1 flex gap-2">
                <input 
                  v-model="form.to" 
                  type="text" 
                  placeholder="destinataire@example.com (séparé par des virgules)"
                  class="w-full border border-gray-300 rounded-md px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                />
                <button 
                  v-if="!showCcBcc" 
                  @click="showCcBcc = true"
                  class="text-sm text-blue-600 hover:text-blue-800 font-medium px-2"
                >
                  Cc/Bcc
                </button>
              </div>
            </div>

            <!-- CC / BCC (Conditionnel) -->
            <template v-if="showCcBcc">
              <div class="flex items-center gap-4">
                <label class="w-16 text-right text-sm font-medium text-gray-600">Cc :</label>
                <input v-model="form.cc" type="text" class="flex-1 border border-gray-300 rounded-md px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"/>
              </div>
              <div class="flex items-center gap-4">
                <label class="w-16 text-right text-sm font-medium text-gray-600">Bcc :</label>
                <input v-model="form.bcc" type="text" class="flex-1 border border-gray-300 rounded-md px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"/>
              </div>
            </template>

            <!-- Sujet -->
            <div class="flex items-center gap-4">
              <label class="w-16 text-right text-sm font-medium text-gray-600">Sujet :</label>
              <input 
                v-model="form.subject" 
                type="text" 
                class="flex-1 border border-gray-300 rounded-md px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent font-medium"
              />
            </div>

            <!-- Editeur HTML Quill -->
            <div class="mt-4 border border-gray-300 rounded-md overflow-hidden flex flex-col h-64">
              <!-- La classe h-full est requise pour que l'éditeur remplisse le conteneur -->
              <QuillEditor 
                v-model:content="form.body" 
                contentType="html" 
                theme="snow"
                class="flex-1 bg-white"
                toolbar="minimal"
              />
            </div>

            <!-- Pièces jointes -->
            <div class="mt-4 bg-gray-50 p-4 rounded-md border border-gray-200">
              <div class="flex justify-between items-center mb-2">
                <h3 class="text-sm font-medium text-gray-700 flex items-center gap-2">
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.172 7l-6.586 6.586a2 2 0 102.828 2.828l6.414-6.586a4 4 0 00-5.656-5.656l-6.415 6.585a6 6 0 108.486 8.486L20.5 13" />
                  </svg>
                  Pièces jointes ({{ form.attachments.length }})
                </h3>
                <button @click="handleAddAttachment" class="text-sm bg-white border border-gray-300 px-3 py-1 rounded hover:bg-gray-100 transition-colors">
                  Ajouter un fichier
                </button>
              </div>
              
              <!-- Liste des PJs -->
              <ul v-if="form.attachments.length > 0" class="space-y-2 mt-3">
                <li v-for="(file, index) in form.attachments" :key="index" class="flex justify-between items-center bg-white p-2 border border-gray-200 rounded text-sm">
                  <span class="truncate pr-4" :title="file.path">{{ file.name }}</span>
                  <button @click="removeAttachment(index)" class="text-red-500 hover:text-red-700 px-2">
                    &times;
                  </button>
                </li>
              </ul>
            </div>

            <!-- Messages d'erreur ou succès -->
            <div v-if="errorMsg" class="p-3 bg-red-100 text-red-700 rounded-md text-sm">{{ errorMsg }}</div>
            <div v-if="successMsg" class="p-3 bg-green-100 text-green-700 rounded-md text-sm">{{ successMsg }}</div>

          </div>

          <!-- Footer / Actions -->
          <div class="px-6 py-4 border-t border-gray-200 bg-gray-50 flex justify-end gap-3">
            <button 
              @click="closeModal" 
              class="px-4 py-2 border border-gray-300 rounded-lg text-gray-700 hover:bg-gray-100 transition-colors text-sm font-medium"
            >
              Annuler
            </button>
            <button 
              @click="sendEmail" 
              :disabled="isSending"
              class="px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors text-sm font-medium disabled:opacity-50 flex items-center gap-2"
            >
              <svg v-if="isSending" class="animate-spin h-4 w-4 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
              </svg>
              {{ isSending ? 'Envoi...' : 'Envoyer' }}
            </button>
          </div>

        </div>
      </div>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue';
import { invoke } from '@tauri-apps/api/core'; // Tauri v2
import { open } from '@tauri-apps/plugin-dialog'; // Tauri v2
import { QuillEditor } from '@vueup/vue-quill';
import '@vueup/vue-quill/dist/vue-quill.snow.css';

// --- Interfaces ---
interface Account {
  id: string;
  label: string;
  email: string;
}

interface AttachmentFile {
  name: string;
  path: string;
}

// --- Données en dur (à remplacer par ta logique métier) ---
const availableAccounts: Account[] = [
  { id: 'irisa', label: 'Compte IRISA', email: 'barais@irisa.fr' },
{ id: 'rennes1', label: 'Compte Univ Rennes ', email: 'olivier.barais@univ-rennes.fr' }

];

// --- État du composant ---
const isOpen = ref(false);
const isSending = ref(false);
const showCcBcc = ref(false);
const errorMsg = ref('');
const successMsg = ref('');

const form = reactive({
  account: availableAccounts[0].id,
  to: '',
  cc: '',
  bcc: '',
  subject: '',
  body: '',
  attachments: [] as AttachmentFile[]
});

// --- Fonctions UI ---
const openModal = () => {
  isOpen.value = true;
  successMsg.value = '';
  errorMsg.value = '';
};

const closeModal = () => {
  if (isSending.value) return;
  isOpen.value = false;
  // Optionnel: Réinitialiser le formulaire ici
};

// Utilitaire pour extraire le nom du fichier depuis un chemin absolu (compatible Windows/Unix)
const extractFilename = (path: string) => {
  return path.split(/[/\\]/).pop() || 'Fichier';
};

// --- Gestion des Pièces Jointes (Tauri Dialog) ---
const handleAddAttachment = async () => {
  try {
    const selected = await open({
      multiple: true,
      directory: false,
    });

    if (Array.isArray(selected)) {
      // Plusieurs fichiers
      selected.forEach(path => {
        form.attachments.push({ name: extractFilename(path), path });
      });
    } else if (selected) {
      // Un seul fichier
      form.attachments.push({ name: extractFilename(selected), path: selected });
    }
  } catch (err) {
    console.error("Erreur lors du choix du fichier:", err);
  }
};

const removeAttachment = (index: number) => {
  form.attachments.splice(index, 1);
};

// --- Envoi du Mail ---
const sendEmail = async () => {
  isSending.value = true;
  errorMsg.value = '';
  successMsg.value = '';

  // Fonction utilitaire pour transformer "a@a.com, b@b.com" en tableau propre
  const splitEmails = (str: string) => str.split(',').map(e => e.trim()).filter(e => e);

  // Trouver l'email de l'expéditeur sélectionné
  const fromEmail = availableAccounts.find(a => a.id === form.account)?.email || '';

  // Construction du payload attendu par le backend Rust
  const payload = {
    account: form.account,
    from: fromEmail,
    to: splitEmails(form.to),
    cc: splitEmails(form.cc),
    bcc: splitEmails(form.bcc),
    subject: form.subject,
    body: form.body || '<p></p>', // Fallback si éditeur vide
    isHtml: true,
    attachments: form.attachments.map(att => ({
      path: att.path,
      // On laisse le backend Rust déterminer le mime_type ou on peut utiliser un lib frontend
      mimeType: null 
    }))
  };

  if (payload.to.length === 0) {
    errorMsg.value = "Veuillez spécifier au moins un destinataire.";
    isSending.value = false;
    return;
  }

  try {
    // Appel de la commande Rust écrite précédemment
    await invoke('send_email', { payload });
    
    successMsg.value = "Message envoyé avec succès !";
    
    // Fermeture automatique après 1.5s
    setTimeout(() => {
      closeModal();
      // Reset du formulaire pour le prochain message
      form.to = ''; form.cc = ''; form.bcc = ''; form.subject = ''; form.body = ''; form.attachments = [];
    }, 1500);

  } catch (error) {
    errorMsg.value = `Erreur lors de l'envoi : ${error}`;
  } finally {
    isSending.value = false;
  }
};
</script>

<style>
/* Surcharge mineure pour que l'éditeur Quill ait une belle hauteur et se fonde dans Tailwind */
.ql-container {
  font-family: inherit !important;
  font-size: 0.875rem !important; /* text-sm */
  border-bottom-left-radius: 0.375rem;
  border-bottom-right-radius: 0.375rem;
}
.ql-toolbar {
  border-top-left-radius: 0.375rem;
  border-top-right-radius: 0.375rem;
  background-color: #f9fafb; /* bg-gray-50 */
}
.ql-editor {
  min-height: 100%;
}
</style>