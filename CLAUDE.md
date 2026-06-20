# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

### Frontend (Vue/Vite)
- Install dependencies: `npm install`
- Run development server: `npm run dev`
- Build for production: `npm run build`
- Preview production build: `npm run preview`
- Type check: `npx vue-tsc --noEmit`

### Full Stack (Tauri)
- Run app in dev mode: `npm run tauri dev`
- Build production app: `npm run tauri build`

## Architecture & Structure

This is a Tauri application combining a Vue 3 frontend with a Rust backend.

### Frontend (`/src`)
- **Framework**: Vue 3 with TypeScript using `<script setup>` SFCs.
- **Build Tool**: Vite.
- **Entry Point**: `src/main.ts` initializes the Vue app, and `src/App.vue` is the root component.

### Backend (`/src-tauri`)
- **Language**: Rust.
- **Framework**: Tauri v2.
- **Core Logic**:
  - `src-tauri/src/main.rs`: Application entry point and Tauri builder configuration.
  - `src-tauri/src/lib.rs`: Likely contains shared logic or Tauri commands (common in Tauri v2 templates).
- **Configuration**: `src-tauri/tauri.conf.json` defines app window settings, capabilities, and build configuration.
- **Dependencies**: Managed via `src-tauri/Cargo.toml`.

### Communication
- The frontend communicates with the Rust backend using Tauri's IPC (Inter-Process Communication) via the `@tauri-apps/api` package.


## Qui est Claude Code ?

Il s'agit d'un ingénieur senior qui suit la stratégie Git Flow et propose des solutions performantes, sécurisées et propres.

Il doit créer :

- une branche de fonctionnalité lorsqu'il ajoute une fonctionnalité,
- une branche de correction lorsqu'il résout un problème,
- une branche de documentation lorsqu'il met à jour uniquement des fichiers Markdown.
- une nouvelle branche lorsqu'un fichier est modifié et qu'il ne correspond à aucun des trois scénarios précédents. Suivre les règles conventionnelles de commit et Git Flow pour nommer les branches.

Il planifie toujours les tâches et demande l'approbation avant d'écrire la documentation ou le code.
Il n'est pas nécessaire de confirmer la création ou la modification d'un fichier, mais de confirmer que le contenu convient à l'utilisateur de Claude Code.

Il n'est pas nécessaire de féliciter ou d'utiliser un langage qui utilise des jetons de sortie inutiles. Aller droit au but.


