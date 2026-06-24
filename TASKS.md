# Project Task List - NotMuchTauri

Ce fichier suit l'avancement du développement. Chaque tâche doit être validée par rapport aux spécifications dans `/docs`.

## Phase 1 : Fondations et Connectivité (Le "Pont")
- [x] **Tâche 1 : Créer le wrapper Rust pour `notmuch`**
  - Implémenter l'exécution de `notmuch search --format=json`.
  - Parser le JSON vers les structures Rust.
  - Gérer les erreurs de binaire manquant.
- [x] **Tâche 2 : Implémenter la commande Tauri `search_messages`**
  - Exposer le wrapper via un `#[tauri::command]`.
  - Mapper les données brutes vers le type `Message` du frontend.
- [x] **Tâche 3 : Créer la liste de messages dans le frontend Vue**
  - Interface de recherche et affichage des résultats.
  - Gestion de la sélection d'un message.
- [x] **Tâche 4 : Implémenter la récupération des détails du message**
  - Commande `get_message_details` utilisant `notmuch show`.
  - Affichage du corps du mail dans `MailDetail.vue`.

## Phase 2 : Interaction et Gestion (Le "Client Mail")
- [x] **Tâche 5 : Implémenter la vue Conversation/Thread**
  - Créer la commande Rust `get_thread`.
  - Interface de rendu chronologique des messages.
- [x] **Tâche 6 : Implémenter la gestion des tags**
  - Commande `toggle_tag` (appel à `notmuch tag`).
  - UI pour ajouter/supprimer des tags.
- [x] **Tâche 7 : Implémenter l'envoi de mail via `msmtp`**
  - Interface de rédaction (Draft).c
  - Construction du message RFC822 et pipe vers `msmtp`.
  - [x] Supporter les pièce jointe
  - [X] Supporter la completion dans les adresse mails
  - [x] Supporter le reply/replyall/forward (Modifier le composant pour pouvoir injecter les paramètre sous forme de props)
  - [x] Agrandir la largeur de la modal

## Phase 3 : Intelligence Artificielle (L' "Assistant")
- [ ] **Tâche 8 : Configurer le client API Claude dans Rust**
  - Gestion sécurisée des clés API.
  - Client HTTP pour les requêtes vers Anthropic.
- [ ] **Tâche 9 : Implémenter l'analyse et la suggestion de réponse**
  - Analyse du contexte d'un mail.
  - Génération de brouillons de réponse intelligents.

## Phase 4 : Polissage et UX (La "Finition")
- [ ] **Tâche 10 : Gestion des pièces jointes**
  - Parsing des attachments.
  - Ouverture des fichiers via l'OS.
  - Inline pdf avec pdfjs dans une modal
- [ ] **Tâche 11 : Polissage UX et optimisations**
  - Raccourcis clavier.
  - Design final.
- [ ] **Tâche 12 : Transformer en ICS un message**
- [ ] **Tâche 13 : Transformer en tâche un message** https://www.rememberthemilk.com/services/email/
- [x] **Tâche 14 : une unique instance**
- [x] ** Refresh new mail every é minutes**
- [ ] To get push, notifications when new email arrive, I use a personal version of [node-imapnotify](https://github.com/barais/node-imapnotify) based on the initial version of [node-imapnotify](https://github.com/a-sk/node-imapnotify/).
