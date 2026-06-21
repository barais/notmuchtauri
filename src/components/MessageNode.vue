<template>
  <div class="relative w-full">
    
    <!-- Ligne verticale pour montrer la hiérarchie visuelle (visible à partir du niveau 1) -->
    <div 
      v-if="depth > 0" 
      class="absolute left-[-16px] top-6 bottom-0 w-0.5 bg-gray-200"
    ></div>

    <!-- Carte du message -->
    <div 
      class="border rounded-lg mb-3 shadow-sm transition-all duration-200 bg-white overflow-hidden"
      :class="{ 'ml-8': depth > 0 }"
    >
      <!-- Header du message (Cliquable pour Collapse) -->
<!-- Header du message -->
<div 
  @click="toggle" 
  class="cursor-pointer p-3 flex justify-between items-start sm:items-center hover:bg-gray-50 transition-colors"
  :class="{ 'bg-gray-100 border-b': isExpanded }"
>
  <div class="flex items-start sm:items-center space-x-3 overflow-hidden">
    <!-- Avatar minimaliste -->
    <div class="w-8 h-8 mt-1 sm:mt-0 rounded-full bg-blue-100 text-blue-600 flex items-center justify-center font-bold flex-shrink-0">
      {{ getInitials(message.from) }}
    </div>
    
    <div class="flex flex-col truncate">
      <!-- Ligne 1 : Nom + Badges -->
      <div class="flex items-center gap-2 flex-wrap">
        <span class="font-semibold text-gray-800 text-sm truncate">
          {{ extractName(message.from) }}
        </span>

        <!-- Badges dynamiques des tags Notmuch -->
<!-- Ligne 1 : Nom + Badges -->
<div class="flex items-center gap-2 flex-wrap">
  <span class="font-semibold text-gray-800 text-sm truncate">
    {{ extractName(message.from) }}
  </span>

  <!-- Badges dynamiques des tags -->
  <div class="flex flex-wrap items-center gap-1.5">
    <span 
      v-for="tag in visibleTags" 
      :key="tag"
      :class="[getTagStyle(tag).bg, getTagStyle(tag).text]"
      class="group inline-flex items-center px-1.5 py-0.5 rounded text-[10px] font-semibold uppercase tracking-wider transition-colors border border-transparent hover:border-gray-300"
    >
      <!-- Icônes des tags (Gardez vos SVG définis dans la réponse précédente ici) -->
            <svg v-if="getTagStyle(tag).icon === 'todo'" class="w-3 h-3 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
            <!-- Icône Important/Flagged -->
            <svg v-else-if="getTagStyle(tag).icon === 'important'" class="w-3 h-3 mr-1" fill="currentColor" viewBox="0 0 20 20"><path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z" /></svg>
            <!-- Icône Brouillon -->
            <svg v-else-if="getTagStyle(tag).icon === 'draft'" class="w-3 h-3 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" /></svg>
            <!-- Icône Tag par défaut -->
            <svg v-else class="w-3 h-3 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 7h.01M7 3h5c.512 0 1.024.195 1.414.586l7 7a2 2 0 010 2.828l-7 7a2 2 0 01-2.828 0l-7-7A1.994 1.994 0 013 12V7a4 4 0 014-4z" /></svg>
            

      
      <span>{{ tag }}</span>

      <!-- Croix de suppression (Visible au survol grâce à group-hover, ou légèrement grisée) -->
      <!-- Le modificateur .stop empêche l'ouverture/fermeture de l'accordéon -->
      <button 
        @click.stop="handleRemoveTag(tag)" 
        class="ml-1 opacity-50 hover:opacity-100 hover:text-red-600 focus:outline-none transition-opacity rounded-full"
        title="Supprimer ce tag"
      >
        <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>
    </span>

    <!-- Bouton Todo Rapide -->
    <button 
      @click.stop="toggleTodo"
      class="ml-1 p-0.5 rounded text-gray-400 hover:bg-gray-200 transition-colors"
      :class="{ 'text-orange-500 bg-orange-100 hover:bg-orange-200': localTags.includes('todo') }"
      title="Marquer comme Todo"
    >
      <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <!-- Icône d'une liste de tâche (Check-square) -->
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
      </svg>
    </button>

    <!-- Bouton d'ajout de tag personnalisé -->
<!-- Bouton d'ajout de tag personnalisé -->
<button 
  @click.stop="openTagModal"
  class="p-0.5 rounded text-gray-400 hover:bg-gray-200 hover:text-blue-500 transition-colors"
  title="Ajouter un tag"
>
  <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
    <!-- Icône Plus -->
    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
  </svg>
</button>

  </div>
</div>

      </div>
      
      <!-- Ligne 2 : Aperçu du texte -->
      <span v-if="!isExpanded" class="text-xs text-gray-500 truncate mt-0.5">
         {{ snippet }}
      </span>
    </div>
  </div>

  <div class="flex items-center space-x-4 flex-shrink-0 text-sm text-gray-500 ml-2">
    <span>{{ formattedDate }}</span>
    <svg class="w-5 h-5 transform transition-transform" :class="isExpanded ? 'rotate-180' : ''" fill="none" stroke="currentColor" viewBox="0 0 24 24">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
    </svg>
  </div>
</div>
      <!-- Corps du message (Affiché si isExpanded est true) -->
      <div v-show="isExpanded" class="p-4 bg-white">
        <!-- En-têtes secondaires -->
        <div class="mb-4 text-sm text-gray-600 space-y-1 bg-gray-50 p-3 rounded">
          <div><span class="font-semibold text-gray-700">De :</span> {{ message.from }}</div>
          <div><span class="font-semibold text-gray-700">À :</span> {{ message.to }}</div>
          <div v-if="message.cc"><span class="font-semibold text-gray-700">Cc :</span> {{ message.cc }}</div>
        </div>

        <!-- Contenu (HTML prioritaire, sinon Texte) -->
        <!--<div v-if="message.htmlBody" class="email-body prose max-w-none text-sm" v-html="message.htmlBody"></div>-->
        <div v-if="processedHtml" class="email-body prose max-w-none text-sm" v-html="processedHtml"></div>

        <pre v-else-if="message.textBody" class="whitespace-pre-wrap font-sans text-sm text-gray-800">{{ message.textBody }}</pre>
        <div v-else class="italic text-gray-400 text-sm">Ce message est vide.</div>

        <!-- Pièces jointes (si présentes) -->
<!-- Pièces jointes (si présentes) -->
<div v-if="message.attachments.length > 0" class="mt-6 pt-4 border-t border-gray-100">
  <p class="text-xs font-semibold text-gray-500 mb-2 uppercase tracking-wider">Pièces jointes</p>
  
  <div class="flex flex-wrap gap-2">
    <div 
      v-for="att in message.attachments" 
      :key="att.partId" 
      @click="downloadAttachment(att)"
      class="group flex items-center space-x-2 px-3 py-2 rounded text-sm cursor-pointer transition-colors"
      :class="downloadingParts.has(att.partId) ? 'bg-blue-50 text-blue-600' : 'bg-gray-100 hover:bg-gray-200 text-gray-700'"
      title="Cliquer pour télécharger"
    >
      <!-- Icône de chargement (Spinner) -->
      <svg 
        v-if="downloadingParts.has(att.partId)" 
        class="animate-spin w-4 h-4 text-blue-600" 
        xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"
      >
        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
      </svg>
      
      <!-- Icône Trombone ou Téléchargement (Par défaut) -->
      <svg 
        v-else 
        class="w-4 h-4 text-gray-500 group-hover:text-blue-500 transition-colors" 
        fill="none" stroke="currentColor" viewBox="0 0 24 24"
      >
        <!-- Forme de téléchargement -->
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
      </svg>

      <span class="truncate max-w-xs font-medium">{{ att.filename }}</span>
    </div>
  </div>
</div>
      </div>
    </div>

    <!-- Récursion : Affichage des réponses en dessous du message parent -->
    <div v-if="message.replies && message.replies.length > 0" class="w-full">
      <MessageNode 
        v-for="reply in message.replies" 
        :key="reply.id" 
        :message="reply"
        :depth="depth + 1"
        :initially-expanded="false" 
      />
    </div>
  </div>
<!-- Modale de création de Tag (Téléportée à la racine de l'application) -->
  <Teleport to="body">
    <!-- Uniquement visible si isTagModalOpen est true -->
    <div 
      v-if="isTagModalOpen" 
      class="fixed inset-0 z-[100] flex items-center justify-center p-4 bg-gray-900 bg-opacity-40 backdrop-blur-sm transition-opacity"
      @click="closeTagModal" 
    >
      <!-- @click.stop empêche le clic sur la modale de fermer la modale (qui est géré par l'overlay bg-gray-900 au-dessus) -->
      <div 
        @click.stop 
        class="bg-white rounded-xl shadow-2xl w-full max-w-sm overflow-hidden transform transition-all"
      >
        <div class="px-6 py-4">
          <h3 class="text-lg font-semibold text-gray-900 mb-2">Ajouter un tag</h3>
          <p class="text-sm text-gray-500 mb-4">Saisissez le nom du nouveau tag pour ce message.</p>
          
          <!-- L'événement @keyup.enter valide, @keyup.esc ferme la modale -->
          <input 
            ref="tagInputRef"
            v-model="newTagValue"
            @keyup.enter="confirmCustomTag"
            @keyup.escape="closeTagModal"
            type="text" 
            placeholder="Ex: urgent, projet-x, a-lire..."
            class="w-full px-3 py-2 border border-gray-300 rounded-lg shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 text-sm"
          />
        </div>
        
        <div class="px-6 py-3 bg-gray-50 flex flex-row-reverse space-x-2 space-x-reverse rounded-b-xl border-t border-gray-100">
          <button 
            @click="confirmCustomTag"
            class="px-4 py-2 text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 rounded-lg shadow-sm focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 transition-colors"
          >
            Ajouter
          </button>
          <button 
            @click="closeTagModal"
            class="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 hover:bg-gray-50 rounded-lg shadow-sm focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 transition-colors"
          >
            Annuler
          </button>
        </div>
      </div>
    </div>
  </Teleport>

</template>

<script setup lang="ts">
import { ref, computed, PropType, onMounted, watch, nextTick } from 'vue'
import type { AttachmentDto, MessageDto } from '../types' // Ajustez le chemin
import { invoke } from '@tauri-apps/api/core'
import { save } from '@tauri-apps/plugin-dialog' // ou '@tauri-apps/api/dialog' (v1)


const props = defineProps({
  message: {
    type: Object as PropType<MessageDto>,
    required: true
  },
  depth: {
    type: Number,
    default: 0
  },
  initiallyExpanded: {
    type: Boolean,
    default: false
  }
})

// État local pour le toggle
const isExpanded = ref(props.initiallyExpanded)

const toggle = () => {
  isExpanded.value = !isExpanded.value
}

// --- NOUVEAU : HTML asynchrone ---
// On crée une ref qui contient le HTML de base, et qu'on va modifier
const processedHtml = ref(props.message.htmlBody || '')

const downloadingParts = ref<Set<number>>(new Set())


    
const downloadAttachment = async (att: AttachmentDto) => {
  if (downloadingParts.value.has(att.partId)) return // Évite les doubles clics

  try {
    // 1. Ouvrir la boîte de dialogue de sauvegarde native
    const filePath = await save({
      defaultPath: att.filename,
      title: 'Enregistrer la pièce jointe'
    })

    // Si l'utilisateur annule la boîte de dialogue, filePath est null
    if (!filePath) return

    // 2. Début du téléchargement (UI)
    downloadingParts.value.add(att.partId)

    // 3. Appel au backend Rust pour sauvegarder le fichier
    await invoke('save_message_part', {
      messageId: props.message.id,
      partId: att.partId,
      outputPath: filePath
    })

    // Optionnel: Ajouter une notification de succès ici
    console.log('Fichier enregistré avec succès sous:', filePath)

  } catch (error) {
    console.error('Erreur lors du téléchargement :', error)
    // Optionnel: Afficher une notification d'erreur à l'utilisateur
  } finally {
    // 4. Fin du téléchargement (UI)
    downloadingParts.value.delete(att.partId)
  }
}

// --- Gestion des tags ---
interface TagStyle {
  bg: string;
  text: string;
  icon: 'todo' | 'attachment' | 'important' | 'tag' | 'draft';
}

const getTagStyle = (tag: string): TagStyle => {
  const t = tag.toLowerCase()
  if (t === 'todo') {
    return { bg: 'bg-orange-100', text: 'text-orange-700', icon: 'todo' }
  }
  if (t === 'attachment') {
    return { bg: 'bg-gray-100', text: 'text-gray-700', icon: 'attachment' }
  }
  if (t === 'flagged' || t === 'important') {
    return { bg: 'bg-yellow-100', text: 'text-yellow-700', icon: 'important' }
  }
  if (t === 'draft') {
    return { bg: 'bg-red-100', text: 'text-red-700', icon: 'draft' }
  }
  // Style par défaut pour les autres tags (ex: lists, irisa-dir...)
  return { bg: 'bg-blue-50', text: 'text-blue-600', icon: 'tag' }
}



// 1. État local pour les tags (pour ne pas muter props.message.tags)
const localTags = ref<string[]>([...props.message.tags])

// Synchronisation si le parent force une mise à jour des données
watch(() => props.message.tags, (newTags) => {
  localTags.value = [...newTags]
})

// On remplace props.message.tags par localTags.value dans visibleTags
const visibleTags = computed(() => {
  const hiddenTags = ['inbox', 'unread', 'replied']
  return localTags.value.filter(tag => !hiddenTags.includes(tag.toLowerCase()))
})

// 2. Fonction d'ajout (Optimiste)
const handleAddTag = async (tag: string) => {
  const normalizedTag = tag.trim().toLowerCase()
  if (!normalizedTag || localTags.value.includes(normalizedTag)) return

  // Sauvegarde avant modification
  const prevTags = [...localTags.value]
  // Mise à jour immédiate de l'UI
  localTags.value.push(normalizedTag)

  try {
    await invoke('modify_message_tag', { 
      messageId: props.message.id, 
      tag: normalizedTag, 
      action: 'add' 
    })
  } catch (error) {
    console.error(`Erreur lors de l'ajout du tag ${normalizedTag}:`, error)
    localTags.value = prevTags // Rollback en cas d'erreur
  }
}

// 3. Fonction de suppression (Optimiste)
const handleRemoveTag = async (tag: string) => {
  // Sauvegarde avant modification
  const prevTags = [...localTags.value]
  // Mise à jour immédiate de l'UI
  localTags.value = localTags.value.filter(t => t !== tag)

  try {
    await invoke('modify_message_tag', { 
      messageId: props.message.id, 
      tag: tag, 
      action: 'remove' 
    })
  } catch (error) {
    console.error(`Erreur lors de la suppression du tag ${tag}:`, error)
    localTags.value = prevTags // Rollback en cas d'erreur
  }
}

// 4. Action rapides pour les boutons
const toggleTodo = () => {
  if (localTags.value.includes('todo')) {
    handleRemoveTag('todo')
  } else {
    handleAddTag('todo')
  }
}

const promptCustomTag = () => {
  // Utilisation de prompt() natif pour la simplicité. 
  // Vous pourrez le remplacer par un modal personnalisé plus tard.
  const newTag = prompt('Entrez le nom du tag à ajouter :')
  if (newTag) {
    handleAddTag(newTag)
  }
}
// --- État de la modale d'ajout de tag ---
const isTagModalOpen = ref(false)
const newTagValue = ref('')
const tagInputRef = ref<HTMLInputElement | null>(null)

// Ouvre la modale
const openTagModal = async () => {
  newTagValue.value = '' // Réinitialise le champ
  isTagModalOpen.value = true
  
  // Attend que la modale soit affichée dans le DOM puis donne le focus au champ texte
  await nextTick()
  if (tagInputRef.value) {
    tagInputRef.value.focus()
  }
}

// Confirme l'ajout
const confirmCustomTag = () => {
  const tag = newTagValue.value.trim()
  if (tag) {
    handleAddTag(tag)
  }
  isTagModalOpen.value = false
}

// Annule
const closeTagModal = () => {
  isTagModalOpen.value = false
}

// --- Fonctions utilitaires d'affichage ---
onMounted(async () => {

        try {
            props.message.tags = props.message.tags.filter(tag => tag.toLowerCase() !== 'unread')
    await invoke('modify_message_tag', { 
      messageId: props.message.id, 
      tag: 'unread', 
      action: 'remove' 
    })
  } catch (error) {
    console.error(`Erreur lors du retrait du tag remove:`, error)
  }


  // Si le message n'a pas d'images inline ou pas de HTML, on s'arrête là
  if (!props.message.htmlBody || props.message.inlineImages.length === 0) return

  let tempHtml = props.message.htmlBody

  for (const img of props.message.inlineImages) {
    if (!img.contentId) continue

    // Notmuch retourne parfois le content-id avec des chevrons, ex: <mon-image@domaine.com>
    // Dans le HTML, l'attribut src est souvent src="cid:mon-image@domaine.com"
    const rawCid = img.contentId.replace(/^<|>$/g, '')

    try {
      // 1. Demander le binaire encodé en base64 au backend Rust
      const base64Data: string = await invoke('get_message_part', { 
        messageId: props.message.id, 
        partId: img.partId 
      })

      // 2. Construire l'URL Base64
      const mimeType = img.contentType || 'image/png'
      const dataUrl = `data:${mimeType};base64,${base64Data}`

      // 3. Échapper le CID au cas où il contient des caractères spéciaux (points, +...)
      const safeCid = rawCid.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')
      
      // 4. Remplacer 'cid:VOTRE_ID' partout dans le HTML (src="", background-image, etc.)
      const regex = new RegExp(`cid:${safeCid}`, 'gi')
      tempHtml = tempHtml.replace(regex, dataUrl)
      

    } catch (error) {
      console.error(`Impossible de charger l'image inline CID: ${rawCid}`, error)
    }
  }

  // 5. Mettre à jour la vue avec le HTML modifié
  processedHtml.value = tempHtml;
})


// Formater la date proprement
const formattedDate = computed(() => {
  if (!props.message.date) return ''
  const d = new Date(props.message.date)
  if (isNaN(d.getTime())) return props.message.date
  return new Intl.DateTimeFormat('fr-FR', {
    day: '2-digit', month: 'short', year: 'numeric',
    hour: '2-digit', minute: '2-digit'
  }).format(d)
})

// Extraire "Prénom Nom" depuis "Prénom Nom <email@domaine.com>"
const extractName = (fromRaw: string) => {
  const match = fromRaw.match(/^([^<]+)/)
  return match ? match[1].replace(/"/g, '').trim() : fromRaw
}

// Générer les initiales (ex: "Olivier Barais" -> "OB")
const getInitials = (fromRaw: string) => {
  const name = extractName(fromRaw)
  const parts = name.split(' ').filter(p => p.length > 0)
  if (parts.length >= 2) return (parts[0][0] + parts[1][0]).toUpperCase()
  if (parts.length === 1) return parts[0].substring(0, 2).toUpperCase()
  return '?'
}

// Petit aperçu du corps du message quand il est fermé
const snippet = computed(() => {
  const text = props.message.textBody || ''
  const clean = text.replace(/\s+/g, ' ').trim()
  return clean.length > 80 ? clean.substring(0, 80) + '...' : clean || '(Contenu HTML / Sans texte)'
})
</script>

<style scoped>
/* 
  Si vous n'utilisez pas @tailwindcss/typography (plugin "prose"), 
  ajoutez des styles basiques ici pour l'affichage brut de l'email HTML.
*/
:deep(.email-body a) {
  color: #2563eb;
  text-decoration: underline;
}
:deep(.email-body blockquote) {
  border-left: 4px solid #e5e7eb;
  padding-left: 1rem;
  color: #4b5563;
  margin-left: 0;
  margin-right: 0;
}
:deep(.email-body img) {
  max-width: 100%;
  height: auto;
}
</style>