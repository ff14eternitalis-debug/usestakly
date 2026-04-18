# Phase 3 — Studio Frontend

> Version : 1.0 — 2026-04-15
> Durée estimée : 2-3 semaines
> Dépendances : Phase 1, 2

## 🎯 Objectif

Construire le **Studio** : interface React + Tailwind pour explorer, créer, éditer et tester les snippets.

## 🖼️ Vue d'ensemble UI

Disposition 3 colonnes + sidebar + header :

```
┌──────────────────────────────────────────────────────────────┐
│ [Logo]   [Search]                              (User profile)│
├──────────┬─────────────────┬─────────────────┬───────────────┤
│ Sidebar  │   COL GAUCHE    │   COL CENTRE    │  COL DROITE   │
│          │                 │                 │               │
│ Frontend │  RENDU LIVE     │  DESCRIPTION    │  CODE (Monaco)│
│  Atoms   │  (si possible)  │  + metadata     │               │
│  Molec.  │                 │                 │               │
│  Organ.  │  ou             │  - titre        │               │
│          │                 │  - slug         │               │
│ Backend  │  LOGO DU        │  - tags         │               │
│  Handler │  LANGAGE        │  - variables    │               │
│  Service │  (fallback)     │  - dependencies │               │
│          │                 │  - version      │               │
│ DevOps   │                 │  - description  │               │
│ Data     │                 │    (Markdown)   │               │
│ Shared   │                 │                 │               │
└──────────┴─────────────────┴─────────────────┴───────────────┘
```

### Règle de bascule automatique de la colonne gauche

La **colonne gauche** affiche un contenu **dépendant du langage détecté du code dans la colonne droite** :

| Cas | Colonne gauche |
|---|---|
| Langage à rendu visuel possible (`tsx`, `jsx`, `vue`, `svelte`, `html`, `css`) | **Rendu live** via Sandpack, avec device frame (desktop/mobile) |
| Tout autre langage (`rust`, `python`, `go`, `sql`, `bash`, `yaml`, `dockerfile`…) | **Logo du langage** centré en grand + nom du langage en sous-texte |

La bascule est **automatique** : dès que le code est collé/édité dans la colonne droite, la détection se déclenche, et la colonne gauche s'adapte sans action utilisateur.

### Bibliothèque de logos
- Sources libres : `devicon`, `simple-icons`
- Pack local SVG (pas de CDN externe) pour performance et offline
- Fallback générique (icône `</>`) si le langage n'est pas dans le pack

## 📋 Livrables

1. Layout 3 colonnes responsive
2. Sidebar avec arbre de navigation adaptatif (Atomic Design pour frontend, par couches pour backend)
3. Page de création de snippet avec détection auto (stub Phase 4)
4. Éditeur Monaco avec syntax highlighting multi-langage
5. Preview live (Sandpack pour frontend, preview textuel pour backend)
6. Recherche globale (sémantique via backend)
7. Auth UI (signup, login, profile)

## 🔨 Tâches détaillées

### 3.1 Layout & routing
- [ ] Router `@tanstack/react-router` avec routes : `/`, `/login`, `/signup`, `/library`, `/snippets/:id`, `/new`, `/projects`, `/settings`
- [ ] Layout principal avec Header + Sidebar + Content
- [ ] Mode sombre par défaut (thème Komorebi = zen)

### 3.2 Auth UI
- [ ] Page signup (email, username, password)
- [ ] Page login
- [ ] Store JWT dans `localStorage` + refresh transparent
- [ ] Hook `useAuth()` + guards sur routes privées
- [ ] Page profil (avatar, bio, export de mes snippets)

### 3.3 Bibliothèque (liste de snippets)
- [ ] Grille de cards snippets avec filtres : domain, kind, language, framework
- [ ] Recherche textuelle + sémantique (debounced)
- [ ] Pagination infinie ou classique
- [ ] Tags cliquables pour filtrer
- [ ] Badge `private` / `public` visible

### 3.4 Sidebar adaptive
- [ ] Arbre frontend : Atoms / Molecules / Organisms / Templates / Utils
- [ ] Arbre backend : Handlers / Services / Models / Queries / Middlewares
- [ ] Arbre devops, data, shared
- [ ] Compteur de snippets par catégorie
- [ ] Drag'n'drop pour réorganiser (v1.1)

### 3.5 Création d'un snippet
- [ ] Formulaire en 3 étapes : Coller le code → Valider la détection → Décrire & tager
- [ ] Détection auto qui pré-remplit (appel backend)
- [ ] Badge "suggéré" sur chaque champ auto-rempli
- [ ] Prévisualisation du slug généré en live
- [ ] Validation côté client (regex nomenclature)

### 3.6 Éditeur Monaco
- [ ] Intégration `@monaco-editor/react`
- [ ] Syntax highlighting par `language` (rust, tsx, python, sql, yaml...)
- [ ] Thème sombre, font Fira Code
- [ ] Detection des `{{variables}}` → surlignage spécial
- [ ] Auto-save toutes les 5s (draft local)

### 3.7 Colonne gauche — Rendu ou Logo
- [ ] Détection automatique : si langage visuel (tsx/jsx/vue/svelte/html/css) → Sandpack
- [ ] Sinon : afficher le logo du langage (pack SVG local basé sur devicon/simple-icons) + nom
- [ ] Bascule transparente en direct quand l'utilisateur change le code ou le langage
- [ ] Device frame (desktop / mobile) uniquement quand un rendu est affiché
- [ ] Bouton "tester avec variables" → formulaire pour remplir les slots (uniquement si rendu)
- [ ] Fallback `</>` générique si le langage n'est pas dans le pack de logos

### 3.8 Documentation du snippet
- [ ] Titre, description (Markdown rendu)
- [ ] Tags éditables inline
- [ ] Variables listées avec types
- [ ] Dependencies (autres snippets requis)
- [ ] Historique des versions

### 3.9 Génération via MCP (UI)
- [ ] Page "Nouveau projet" : choix stack → création projet
- [ ] Bouton "Générer via IA" → prompt utilisateur → affichage du plan → validation → code généré
- [ ] Visualisation des snippets utilisés dans la génération

### 3.10 Qualité UI
- [ ] Design system interne : tokens Tailwind (couleurs, spacings, radii) définis
- [ ] Composants Komorebi eux-mêmes créés en `frontend-atom-*` → dogfooding !
- [ ] Responsive : tablette au minimum, mobile en v1.1
- [ ] Accessibilité : labels aria, navigation clavier, contraste AA

## ✅ Definition of Done

- [ ] Un utilisateur peut s'inscrire, se connecter, créer son premier snippet en < 3 min
- [ ] La bibliothèque charge en < 500 ms pour 100 snippets
- [ ] La preview Sandpack fonctionne pour un snippet React simple
- [ ] La recherche sémantique retourne des résultats pertinents (test : "bouton rouge" trouve `button-primary`)
- [ ] Le design reste cohérent et agréable en thème sombre
- [ ] Lighthouse > 90 sur Performance, Accessibility, Best Practices

## ⚠️ Pièges à éviter

- ❌ Partir sur Redux ou une lib state complexe — Zustand suffit
- ❌ Negliger le dark mode (c'est l'ADN Komorebi)
- ❌ Monaco + Sandpack chargés sur la home → lazy-load obligatoire
- ❌ Oublier les états de chargement / erreur / vide sur chaque vue
- ❌ Faire du CSS custom au lieu de Tailwind utility-first

## 📚 Références
- [vision.md](../vision.md)
- [tech-stack.md](../tech-stack.md)
- Schéma Excalidraw initial (disposition 3 colonnes)
