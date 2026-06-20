# System Design - NotMuchTauri

## 1. Vue d'Ensemble
L'application suit une architecture découplée où le frontend (Vue 3) gère l'interface utilisateur et le backend (Tauri/Rust) sert de pont sécurisé et performant vers les outils système.

### Flux de Données Global
`Vue Frontend` $\xrightarrow{Tauri Invoke}$ `Rust Backend` $\xrightarrow{Command}$ `notmuch/msmtp CLI` $\xrightarrow{JSON}$ `Rust Parser` $\xrightarrow{Tauri Event/Return}$ `Vue Frontend`

## 2. Interaction avec les Outils CLI

### Notmuch (Lecture & Indexation)
- **Méthode** : Appels directs via `std::process::Command`.
- **Format de données** : Utilisation du flag `--format=json` pour toutes les requêtes.
- **Wrapper Rust** : Création d'une couche de service dédiée au parsing. 
  - Le wrapper doit transformer le JSON "étrange" de `notmuch` en structures de données Rust typées (`serde`) plus ergonomiques pour le frontend.
  - Gestion du cache local pour éviter des appels redondants aux binaires sur les vues identiques.

### msmtp (Envoi)
- **Méthode** : Appels via `std::process::Command`.
- **Flux** : Le frontend envoie le contenu du mail $\rightarrow$ Rust construit le message au format RFC $\rightarrow$ Pipe vers `msmtp`.

## 3. Intégration de l'IA (Claude)

L'intelligence artificielle est intégrée comme un service d'assistance à la rédaction et à l'analyse.

- **Contexte** : Le backend Rust extrait le contenu du mail (via notmuch) et le transmet à l'API Claude avec un prompt système spécifique.
- **Fonctionnalités** :
  - Analyse du ton et du contenu d'un mail reçu.
  - Proposition de réponses basées sur le contexte.
  - Correction et amélioration des brouillons.
- **Sécurité** : Les clés d'API sont gérées côté backend pour ne jamais être exposées au frontend.

## 4. Stack Technique
- **Frontend** : Vue 3, TypeScript, Vite.
- **Backend** : Rust, Tauri v2.
- **Parsing JSON** : `serde` et `serde_json` pour la transformation des données `notmuch`.
- **CLI Dependences** : `notmuch` et `msmtp` doivent être installés sur le système hôte.
