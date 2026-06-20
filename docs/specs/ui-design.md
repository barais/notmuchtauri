# UI Design Specification - NotMuchTauri

## 1. Stack Visuelle
- **Framework**: Vue 3 + Tailwind CSS.
- **Style**: Moderne, épuré, orienté productivité (densité d'information moyenne, support clavier).

## 2. Architecture des Volets (Layout 3 colonnes)

### Volet 1 : Navigation & Filtres (Gauche - ~250px)
- **Barre de recherche** : Champ texte pour requêtes `notmuch`.
- **Filtres Rapides (Tags)** : Liste de boutons/cases à cocher pour les tags communs :
  - `unread`
  - `attachment`
  - `flagged`
  - Tags personnalisés (chargés dynamiquement).
- **Actions de navigation** : Boutons pour basculer entre dossiers ou vues prédéfinies.

### Volet 2 : Liste des Threads (Centre - Flexible)
- **Affichage** : Liste compacte des threads correspondant à la recherche/filtre.
- **Éléments d'un Thread** : Sujet, dernier auteur, date relative, nombre de messages.
- **Interactions** :
    - **Sélection** : Case à cocher pour chaque thread + "Tout sélectionner".
    - **Actions de groupe** : Barre d'outils contextuelle pour appliquer un tag ou supprimer les threads sélectionnés.
    - **Accès rapide** : **Double-clic** $\rightarrow$ Ouverture du fil de discussion dans le Volet 3.

### Volet 3 : Vue du Thread & Message (Droite - Flexible)
- **Mode Thread (Fil)** : 
    - Liste chronologique des messages du thread.
    - **Interactions** : **Double-clic** sur un message $\rightarrow$ Ouverture du message seul.
- **Mode Message Unique** :
    - Affichage complet du corps du mail et des headers.
    - **Actions** : Répondre, Supprimer, Gérer les tags.
    - **IA Integration** : Bouton "Suggérer une réponse" ou "Analyser le mail".

## 3. Flux d'Interaction (User Journey)

1. **Recherche** : Utilisateur tape "projet laura" $\rightarrow$ Le Volet 2 se met à jour avec la liste des threads.
2. **Exploration** : Double-clic sur un thread $\rightarrow$ Le Volet 3 affiche la conversation complète.
3. **Action** : Double-clic sur un message spécifique $\rightarrow$ Le Volet 3 affiche le message seul avec les options de réponse et de tagging.
4. **Traitement groupés** : Sélection de 5 threads $\rightarrow$ clic sur "Tagger comme 'Archives'".

## 4. État du système
- **Loading** : Squelettes (skeletons) lors du chargement des threads et des messages.
- **Empty States** : Messages explicites quand aucune recherche ne correspond.
