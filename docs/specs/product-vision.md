# Product Vision - NotMuchTauri

## 1. Pitch
Un client mail moderne, performant et intelligent, servant d'interface graphique (GUI) aux outils CLI `notmuch` (lecture/indexation) et `msmtp` (envoi).

## 2. Le Problème
L'écosystème `notmuch` et `msmtp` offre une puissance et une fiabilité exceptionnelles mais impose une barrière à l'entrée élevée via le terminal. L'application vise à :
- Moderniser l'expérience utilisateur sans sacrifier la performance du backend CLI.
- Faciliter l'intégration d'assistants IA (Claude) pour l'aide à la rédaction, la correction et la suggestion de réponses, rendant la gestion des emails plus fluide et intelligente.

## 3. Public Cible
- Utilisateurs de Linux/Unix privilégiant la gestion locale des mails (Maildir).
- Utilisateurs avancés souhaitant la rapidité de `notmuch` mais le confort d'une interface moderne.
- Utilisateurs souhaitant augmenter leur productivité grâce à l'IA intégrée.

## 4. Objectifs du MVP
### Fonctionnalités Critiques
- **Lecture & Navigation** : Interface fluide pour parcourir les messages indexés par `notmuch`.
- **Gestion des Messages** : Lecture du corps du mail et gestion des pièces jointes.
- **Envoi** : Composition et expédition de mails via `msmtp`.
- **Intégration IA** : Interface permettant à un agent Claude d'analyser un mail et de proposer/corriger une réponse.

### Hors-Scope (MVP)
- Gestion directe des protocoles IMAP/POP3 (déléguée aux outils externes).
- Calendrier et gestion de tâches avancée.
- Configuration complexe des serveurs mail (doit être faite via `msmtp` et `notmuch` en amont).

## 5. Contraintes Majeures
- **Performance** : L'interface doit rester réactive, en phase avec la rapidité de `notmuch`.
- **Interopérabilité** : Respect strict des formats gérés par les outils CLI sous-jacents.
- **UX** : Interface moderne, intuitive, potentiellement orientée "clavier" pour les utilisateurs avancés.
