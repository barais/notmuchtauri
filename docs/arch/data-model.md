# Data Model - NotMuchTauri

Ce document définit les structures de données échangées entre le backend Rust et le frontend Vue.

## 1. Raw Notmuch Model (Backend)
Le format JSON renvoyé par `notmuch search --format=json` suit la structure suivante :

### Messag (Root)
Type: `Vec<MessagElement>` (Un tableau simple d'éléments de recherche)

### MessagElement
Structure contenant les métadonnées de recherche :
- `thread`: `String` (ID du thread)
- `timestamp`: `i64`
- `date_relative`: `String`
- `matched`: `i64`
- `total`: `i64`
- `authors`: `String`
- `subject`: `String`
- `query`: `Vec<Option<String>>`
- `tags`: `Vec<String>`

*Note: Pour obtenir le contenu complet d'un message (corps, headers détaillés), on utilise `notmuch show` ou une recherche spécifique par ID.*

## 2. Clean Frontend Model (API Contract)
Pour optimiser les performances du pont Tauri, le backend transforme le modèle brut en un modèle "plat" pour le frontend.

### Message (Simplified)
- `id`: `String` (Utilise l'ID du thread ou du message selon le contexte)
- `subject`: `String`
- `from`: `String`
- `to`: `String`
- `date`: `String`
- `body`: `String`
- `tags`: `String[]`
- `is_read`: `boolean`
- `has_attachments`: `boolean`

### Thread
- `id`: `String`
- `messages`: `String[]`
- `subject`: `String`
- `last_message_date`: `String`
- `participant_count`: `number`

## 3. Transformation Logic
Le wrapper Rust doit :
1. Mapper `MessagElement` vers `Message`.
2. Utiliser `authors` de l'élément de recherche comme champ `from` par défaut.
3. Utiliser `date_relative` pour l'affichage rapide.
4. Récupérer le corps via `notmuch show` uniquement lors de la demande de détails.
