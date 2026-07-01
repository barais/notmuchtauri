<template>
  <div>
    <!-- Bouton pour ouvrir la modale -->
    <button 
      @click="openModal" 
      class="flex items-center px-3 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500 transition-colors"
      title="Paramètres"
    >
      <svg class="w-4 h-4 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
      </svg>
    </button>

    <!-- Modale Téléportée -->
    <Teleport to="body">
      <div 
        v-if="isOpen" 
        class="fixed inset-0 z-[100] flex items-center justify-center p-4 bg-gray-900 bg-opacity-50 backdrop-blur-sm"
        @click="closeModal"
      >
        <!-- Conteneur de la modale -->
        <div 
          @click.stop 
          class="bg-white dark:bg-zinc-900 rounded-xl shadow-2xl w-full max-w-[90vw] flex flex-col max-h-[90vh] overflow-hidden"
        >
          <!-- Header -->
          <div class="px-6 py-4 border-b border-gray-200 dark:border-zinc-800 flex justify-between items-center bg-gray-50 dark:bg-zinc-950">
            <h2 class="text-lg font-bold text-gray-800 dark:text-zinc-100">Configuration de l'application</h2>
            <button @click="closeModal" class="text-gray-400 hover:text-gray-600 dark:hover:text-zinc-300">
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" /></svg>
            </button>
          </div>

          <!-- Body (Scrollable) -->
          <div class="p-6 overflow-y-auto flex-1 space-y-8">
            
            <!-- Section : Paramètres Généraux -->
            <section>
              <h3 class="text-sm font-semibold text-gray-500 uppercase tracking-wider mb-4 border-b pb-2 dark:border-zinc-800">Paramètres Généraux</h3>
              <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
               <div>
                  <label class="block text-sm font-medium text-gray-700 dark:text-zinc-300 mb-1">Fichier de configuration mbsync</label>
                  <input v-model="localConfig.mbsyncrc_path" type="text" placeholder="~/.mbsyncrc" class="form-input" />
                </div>
                <div>
                  <label class="block text-sm font-medium text-gray-700 dark:text-zinc-300 mb-1">Fichier de configuration msmtprc</label>
                  <input v-model="localConfig.msmtprc_path" type="text" placeholder="~/.msmtprc" class="form-input" />
                </div>
                <div>
                  <label class="block text-sm font-medium text-gray-700 dark:text-zinc-300 mb-1">Dossier d'envoi par défaut</label>
                  <input v-model="localConfig.default_sent_folder" type="text" placeholder="Sent" class="form-input" />
                </div>
                <div>
                  <label class="block text-sm font-medium text-gray-700 dark:text-zinc-300 mb-1">Limite de messages</label>
                  <input v-model.number="localConfig.limit" type="number" placeholder="1000" class="form-input" />
                </div>
                
                <!-- Nouveaux champs généraux -->
                <div>
                  <label class="block text-sm font-medium text-gray-700 dark:text-zinc-300 mb-1">Email RTM (Tâches)</label>
                  <input v-model="localConfig.rmtmmail" type="email" placeholder="username+abc@rmilk.com" class="form-input" />
                </div>
                <div>
                  <label class="block text-sm font-medium text-gray-700 dark:text-zinc-300 mb-1">Email Agenda (ICS)</label>
                  <input v-model="localConfig.calendaremail" type="email" placeholder="mon-agenda@domaine.com" class="form-input" />
                </div>
                <div class="md:col-span-2">
                  <label class="block text-sm font-medium text-gray-700 dark:text-zinc-300 mb-1">Hôte LanguageTool</label>
                  <input v-model="localConfig.lthostport" type="text" placeholder="http://localhost:8081" class="form-input" />
                </div>
              </div>
            </section>

            <!-- Section : Paramètres IA (LLM) -->
            <section>
              <h3 class="text-sm font-semibold text-gray-500 uppercase tracking-wider mb-4 border-b pb-2 dark:border-zinc-800">Intelligence Artificielle (LLM)</h3>
              <div class="grid grid-cols-1 md:grid-cols-2 gap-4" v-if="localConfig.llm">
                <div class="md:col-span-2">
                  <label class="block text-sm font-medium text-gray-700 dark:text-zinc-300 mb-1">URL de l'API</label>
                  <input v-model="localConfig.llm.api_url" type="text" placeholder="https://api.openai.com/v1/chat/completions" class="form-input" />
                </div>
                <div>
                  <label class="block text-sm font-medium text-gray-700 dark:text-zinc-300 mb-1">Modèle</label>
                  <input v-model="localConfig.llm.model" type="text" placeholder="gpt-4o / mistral-large" class="form-input" />
                </div>
                <div>
                  <label class="block text-sm font-medium text-gray-700 dark:text-zinc-300 mb-1">Clé API (Bearer)</label>
                  <input v-model="localConfig.llm.api_key" type="password" placeholder="sk-..." class="form-input" />
                </div>
              </div>
            </section>

            <!-- Section : Raccourcis Texte -->
            <section>
              <div class="flex justify-between items-center mb-4 border-b pb-2 dark:border-zinc-800">
                <h3 class="text-sm font-semibold text-gray-500 uppercase tracking-wider">Raccourcis Texte</h3>
                <button @click="addShortcut" class="text-xs flex items-center text-blue-600 hover:text-blue-700 bg-blue-50 px-2 py-1 rounded transition-colors">
                  <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" /></svg>
                  Ajouter un raccourci
                </button>
              </div>

              <div v-if="!localConfig.shortcuts || localConfig.shortcuts.length === 0" class="text-sm text-gray-500 italic">
                Aucun raccourci configuré.
              </div>

              <div class="space-y-3">
                <div 
                  v-for="(shortcut, index) in localConfig.shortcuts" 
                  :key="'shortcut-' + index" 
                  class="flex items-start gap-3 p-3 bg-gray-50 dark:bg-zinc-800/50 border border-gray-200 dark:border-zinc-700 rounded-lg relative group"
                >
                  <div class="w-1/3">
                    <label class="block text-xs text-gray-500 dark:text-zinc-400 mb-1">Raccourci</label>
                    <input v-model="shortcut.shortcut" type="text" placeholder="ex: !bonjour" class="form-input text-sm py-1.5" />
                  </div>
                  <div class="flex-1">
                    <label class="block text-xs text-gray-500 dark:text-zinc-400 mb-1">Texte de remplacement</label>
                    <textarea v-model="shortcut.text" rows="2" placeholder="Bonjour,\n\nSuite à notre échange..." class="form-input text-sm py-1.5 resize-none"></textarea>
                  </div>
                  <button 
                    @click="removeShortcut(index)"
                    class="mt-5 text-gray-400 hover:text-red-500 p-1 rounded transition-colors"
                    title="Supprimer ce raccourci"
                  >
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                    </svg>
                  </button>
                </div>
              </div>
            </section>

            <!-- Section : Comptes Email -->
            <section>
              <div class="flex justify-between items-center mb-4 border-b pb-2 dark:border-zinc-800">
                <h3 class="text-sm font-semibold text-gray-500 uppercase tracking-wider">Comptes configurés</h3>
                <button @click="addAccount" class="text-xs flex items-center text-blue-600 hover:text-blue-700 bg-blue-50 px-2 py-1 rounded transition-colors">
                  <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" /></svg>
                  Ajouter un compte
                </button>
              </div>

              <div v-if="!localConfig.accounts || localConfig.accounts.length === 0" class="text-sm text-gray-500 italic">
                Aucun compte configuré.
              </div>

              <div class="space-y-4">
                <div 
                  v-for="(acc, index) in localConfig.accounts" 
                  :key="'account-' + index" 
                  class="p-4 bg-gray-50 dark:bg-zinc-800/50 border border-gray-200 dark:border-zinc-700 rounded-lg relative group"
                >
                  <button 
                    @click="removeAccount(index)"
                    class="absolute top-2 right-2 text-gray-400 hover:text-red-500 p-1 rounded transition-colors"
                    title="Supprimer ce compte"
                  >
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                    </svg>
                  </button>

                  <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 pr-6">
                    <div>
                      <label class="block text-xs text-gray-500 dark:text-zinc-400 mb-1">Identifiant unique (ID)</label>
                      <input v-model="acc.id" type="text" placeholder="ex: perso, travail..." class="form-input text-sm py-1.5" />
                    </div>
                    <div>
                      <label class="block text-xs text-gray-500 dark:text-zinc-400 mb-1">Label d'affichage</label>
                      <input v-model="acc.label" type="text" placeholder="ex: Boîte Personnelle" class="form-input text-sm py-1.5" />
                    </div>
                    <div>
                      <label class="block text-xs text-gray-500 dark:text-zinc-400 mb-1">Adresse Email</label>
                      <input v-model="acc.email" type="email" placeholder="nom@domaine.com" class="form-input text-sm py-1.5" />
                    </div>
                    <div>
                      <label class="block text-xs text-gray-500 dark:text-zinc-400 mb-1">Dossier d'envoi (Optionnel)</label>
                      <input v-model="acc.sent_folder" type="text" placeholder="ex: Sent" class="form-input text-sm py-1.5" />
                    </div>
                  </div>
                </div>
              </div>
            </section>
          </div>

          <!-- Footer -->
          <div class="px-6 py-4 border-t border-gray-200 dark:border-zinc-800 bg-gray-50 dark:bg-zinc-950 flex justify-end space-x-3">
            <button 
              @click="closeModal"
              class="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 hover:bg-gray-50 rounded-lg transition-colors focus:outline-none"
              :disabled="isSaving"
            >
              Annuler
            </button>
            <button 
              @click="saveConfig"
              class="flex items-center px-4 py-2 text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 rounded-lg shadow-sm transition-colors focus:outline-none"
              :disabled="isSaving"
            >
              <svg v-if="isSaving" class="animate-spin -ml-1 mr-2 h-4 w-4 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
              </svg>
              {{ isSaving ? 'Sauvegarde...' : 'Enregistrer' }}
            </button>
          </div>

        </div>
      </div>
    </Teleport>
  </div>
</template> 

<script setup lang="ts">
import { ref, PropType } from 'vue';
import { invoke } from '@tauri-apps/api/core'; // Ou '@tauri-apps/api/tauri' selon la v1 ou v2
import { AppConfig } from '../types';


const props = defineProps({
  config: {
    type: Object as PropType<AppConfig>,
    required: true
  }
});

const emit = defineEmits<{
  (e: 'config-saved', newConfig: AppConfig): void
}>();

const isOpen = ref(false);
const isSaving = ref(false);

const localConfig = ref<AppConfig>({
   mbsyncrc_path: '',
  msmtprc_path: '',
  limit: 1000,
  accounts: [],
  default_sent_folder: '',
  calendaremail: "",
  rmtmmail: "",
  lthostport: "http://localhost:8081",
  llm: {
    api_url: "https://ragarenn.eskemm-numerique.fr/sso/ch@t/api/chat/completions",
    api_key: "",
    model: "ilaas/gemma-4-31b"
  },
  shortcuts: []
});

// -- Méthodes de la Modale --

const openModal = () => {
  // Copie profonde de l'objet pour annuler proprement
  localConfig.value = JSON.parse(JSON.stringify(props.config));
  
  // S'assurer que les tableaux et sous-objets existent pour éviter les crashs
  if (!localConfig.value.accounts) localConfig.value.accounts = [];
  if (!localConfig.value.shortcuts) localConfig.value.shortcuts = [];
  if (!localConfig.value.llm) {
    localConfig.value.llm = { api_url: "", api_key: "", model: "" };
  }

  isOpen.value = true;
};

const closeModal = () => {
  isOpen.value = false;
};

// -- Gestion des raccourcis dynamiques --
const addShortcut = () => {
  if (!localConfig.value.shortcuts) localConfig.value.shortcuts = [];
  localConfig.value.shortcuts.push({
    shortcut: '',
    text: ''
  });
};

const removeShortcut = (index: number) => {
  if (localConfig.value.shortcuts) {
    localConfig.value.shortcuts.splice(index, 1);
  }
};

// -- Gestion des comptes dynamiques --
const addAccount = () => {
  if (!localConfig.value.accounts) localConfig.value.accounts = [];
  localConfig.value.accounts.push({
    id: crypto.randomUUID(), 
    label: '',
    email: '',
    sent_folder: undefined,
    is_default:false
  });
};

const removeAccount = (index: number) => {
  if (localConfig.value.accounts) {
    localConfig.value.accounts.splice(index, 1);
  }
};

// -- Sauvegarde --
const saveConfig = async () => {
  try {
    isSaving.value = true;

    // Assainissement des données avant envoi à Rust
    const payloadToSave: AppConfig = {
      mbsyncrc_path: localConfig.value.mbsyncrc_path || '',
      msmtprc_path: localConfig.value.msmtprc_path || '',
      limit: localConfig.value.limit || 1000,
      default_sent_folder: localConfig.value.default_sent_folder || 'Sent',
      calendaremail: localConfig.value.calendaremail || '',
      
      // On convertit les chaînes vides "" en undefined pour respecter Option<String> en Rust
      rmtmmail: localConfig.value.rmtmmail || undefined,
      lthostport: localConfig.value.lthostport || undefined,

      llm: localConfig.value.llm,

      accounts: localConfig.value.accounts?.map(acc => ({
        ...acc,
        id: acc.id.trim(),
        sent_folder: acc.sent_folder || undefined
      })) || [],
      
      shortcuts: localConfig.value.shortcuts?.map(short => ({
        ...short,
        shortcut: short.shortcut.trim()
      })) || []
    };

    // Appel à la commande backend Rust
    await invoke('save_config', { config: payloadToSave });

    emit('config-saved', payloadToSave);
    closeModal();
  } catch (error) {
    console.error("Erreur lors de la sauvegarde de la configuration :", error);
    alert("Impossible d'enregistrer la configuration : " + error);
  } finally {
    isSaving.value = false;
  }
};
</script>

<style scoped>
.form-input {
}
</style>