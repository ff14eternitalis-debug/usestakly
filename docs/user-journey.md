# Projet K — Parcours Utilisateur

> Version : 1.0 — 2026-04-15
>
> Ce document décrit les **parcours utilisateur** (user journeys) principaux du produit. Il sert de référence pour le design UX, les choix d'API et les priorités de développement.

## 🎯 Principes UX directeurs

1. **Time-to-value < 5 minutes** : un nouveau dev doit voir de la valeur dès sa première session
2. **Zéro friction** : chaque étape enlève un obstacle, n'en ajoute pas
3. **Progressif** : découvrir les features avancées au bon moment (pas à l'onboarding)
4. **Honnête** : pas d'upsell agressif, pas de dark patterns
5. **Dogfooding visible** : l'interface de Komorebi est construite avec Komorebi → démonstration vivante

## 👥 Personas & points d'entrée

| Persona | Point d'entrée | Besoin immédiat |
|---|---|---|
| Dev solo curieux | Landing page / réseaux | Comprendre en 30 s ce que ça apporte |
| Tech lead | Démo ou article | Voir une génération réelle avec contraintes |
| Créateur de contenu | Profil public d'un autre | Publier ses propres packs |
| Dev venant d'un IDE | Extension VS Code (v1.1) | Ajouter un snippet sans quitter l'IDE |

---

## 🚪 Parcours 1 — Premier contact (Activation)

**Objectif** : convertir un visiteur en utilisateur qui a créé son premier snippet.

### Étapes

```
┌──────────────────────────────────────────────────────────┐
│ 1. Landing page                                          │
│    - Hero : "L'IA qui code avec TES briques, pas les     │
│      siennes"                                            │
│    - Démo live 30 s (GIF ou Loom)                        │
│    - CTA : "Essayer gratuitement"                        │
│                      │                                    │
│                      ▼                                    │
│ 2. Signup (email + username + password)                  │
│    - OAuth GitHub en option (plus tard)                  │
│    - Pas de carte bancaire demandée                      │
│                      │                                    │
│                      ▼                                    │
│ 3. Onboarding interactif (3 écrans max)                  │
│    a. "Colle ton premier snippet" (un exemple pré-rempli)│
│    b. "On détecte automatiquement…" (démonstration)      │
│    c. "Maintenant, génère un composant" (flow MCP)       │
│                      │                                    │
│                      ▼                                    │
│ 4. Dashboard (librairie vide + appels à l'action)        │
│    - "Importe tes Gists GitHub" (v1.1)                   │
│    - "Utilise un starter pack officiel"                  │
│    - "Crée un snippet manuellement"                      │
└──────────────────────────────────────────────────────────┘
```

### Métriques
- **Conversion signup → 1er snippet** : cible > 60 %
- **Temps médian landing → 1er snippet** : cible < 5 min
- **Taux d'abandon à l'onboarding** : cible < 30 %

### Pièges à éviter
- ❌ Demander trop d'infos au signup (juste email/username/password)
- ❌ Forcer à créer un projet avant un snippet
- ❌ Tutoriel vidéo de 10 min imposé

---

## ✍️ Parcours 2 — Création d'un snippet

**Objectif** : stocker un bout de code existant en < 90 secondes.

### Étapes

```
┌──────────────────────────────────────────────────────────┐
│ 1. Page "Nouveau snippet" (bouton + depuis sidebar)      │
│                      │                                    │
│                      ▼                                    │
│ 2. Coller le code dans Monaco                            │
│    - Syntax highlighting instantané                      │
│    - Détection automatique en arrière-plan (debounced)   │
│                      │                                    │
│                      ▼                                    │
│ 3. Formulaire pré-rempli (badges "suggéré")              │
│    ┌─────────────────────────────────────┐                │
│    │ domain:    [frontend ▼]    suggéré │                │
│    │ kind:      [atom ▼]        suggéré │                │
│    │ language:  [tsx ▼]         suggéré │                │
│    │ framework: [react ▼]       suggéré │                │
│    │ category:  [action ▼]      suggéré │                │
│    │ name:      [button-primary]        │                │
│    │ slug:      frontend-atom-action-… (calculé en live)│                │
│    │ tags:      [react] [ui] [+]        │                │
│    └─────────────────────────────────────┘                │
│                      │                                    │
│                      ▼                                    │
│ 4. Variables {{...}} détectées                           │
│    "2 variables détectées : label (string), onClick      │
│     (function). Confirmer ?"                             │
│                      │                                    │
│                      ▼                                    │
│ 5. Description (optionnelle mais encouragée)             │
│    - Markdown simple                                     │
│    - Exemple d'usage auto-généré (v1.1)                  │
│                      │                                    │
│                      ▼                                    │
│ 6. Colonne gauche s'adapte automatiquement :             │
│    - Rendu live (Sandpack) si langage visuel             │
│    - Logo du langage sinon (Rust, Python, SQL…)          │
│                      │                                    │
│                      ▼                                    │
│ 7. Sauvegarder (privé par défaut)                        │
│    → Succès : redirection vers la page du snippet        │
└──────────────────────────────────────────────────────────┘
```

### Métriques
- **Temps médian** : cible < 90 s
- **Taux d'acceptation des suggestions** : cible > 75 %
- **% snippets créés avec description** : cible > 50 %

### Cas limites
- Code très long (> 500 lignes) → suggérer de le découper en sous-snippets
- Langage non détecté → dropdown manuel complet
- Aucune variable détectée → champ variables masqué (pas de friction)

---

## 🔍 Parcours 3 — Retrouver & réutiliser un snippet

**Objectif** : retrouver un snippet précis en < 10 secondes.

### Étapes

```
┌──────────────────────────────────────────────────────────┐
│ 1. Depuis n'importe quelle page : touche `/` (raccourci)│
│                      │                                    │
│                      ▼                                    │
│ 2. Barre de recherche globale apparaît                   │
│    - Recherche hybride : texte + sémantique              │
│    - Filtres rapides chips : domain, language            │
│                      │                                    │
│                      ▼                                    │
│ 3. Résultats en temps réel (debounced 200 ms)            │
│    - Card avec : slug, description courte, language     │
│    - Preview inline au survol                            │
│                      │                                    │
│                      ▼                                    │
│ 4. Clic → page détail                                    │
│    - Code + preview + variables + dependencies           │
│    - Bouton "Copier" (code brut ou avec variables)       │
│    - Bouton "Utiliser dans un projet"                    │
└──────────────────────────────────────────────────────────┘
```

### Fonctionnalités clés
- Recherche sémantique : "bouton rouge avec icône" trouve `frontend-atom-action-button-primary` même sans mots-clés exacts
- Filtre combiné : `domain:frontend kind:atom react` en syntaxe texte
- Historique des 5 derniers snippets consultés (raccourci)

### Métriques
- **Temps médian recherche → clic** : cible < 10 s
- **Taux de recherches avec résultat cliqué** : cible > 70 %

---

## 🤖 Parcours 4 — Génération IA via MCP (cœur de la valeur)

**Objectif** : générer un composant/fonction complet en réutilisant ses snippets.

### Pré-requis
- L'utilisateur a au moins 5-10 snippets dans sa librairie
- Un projet créé avec stack + rule_set défini

### Étapes

```
┌──────────────────────────────────────────────────────────┐
│ 1. Depuis la page projet : bouton "Générer avec l'IA"   │
│                      │                                    │
│                      ▼                                    │
│ 2. Prompt utilisateur                                    │
│    - Champ texte libre : "Crée une page de login"       │
│    - Dropdown target_domain : frontend | backend | full  │
│    - Bouton "Générer"                                    │
│                      │                                    │
│                      ▼                                    │
│ 3. Phase d'analyse (visible pour l'utilisateur)          │
│    ┌─────────────────────────────────────────┐            │
│    │ ⏳ L'IA cherche dans ta librairie…      │            │
│    │ ✓ Trouvé : frontend-atom-input-text     │            │
│    │ ✓ Trouvé : frontend-atom-action-button  │            │
│    │ ⚠ Aucun organism-form-auth trouvé       │            │
│    │   → l'IA propose d'en créer un          │            │
│    └─────────────────────────────────────────┘            │
│                      │                                    │
│                      ▼                                    │
│ 4. Plan d'assemblage proposé (VALIDATION HUMAINE)        │
│    "Voici ce que je vais faire :                         │
│     1. Utiliser atom-input-text@1.2 pour email/password  │
│     2. Utiliser atom-button-primary@1.0 pour submit      │
│     3. Créer molecule-form-login-email (nouveau)         │
│     4. Appliquer rule-set default-react-tailwind         │
│                                                           │
│     [Valider] [Modifier] [Annuler]"                     │
│                      │                                    │
│                      ▼                                    │
│ 5. Génération du code                                    │
│    - Progress en temps réel                              │
│    - Stream des fichiers créés                           │
│                      │                                    │
│                      ▼                                    │
│ 6. Rapport final                                         │
│    ┌─────────────────────────────────────────┐            │
│    │ ✓ 3 fichiers générés                    │            │
│    │ ✓ 2 snippets réutilisés                 │            │
│    │ ⚠ 1 nouveau snippet créé (à valider)   │            │
│    │ ✓ 5 règles appliquées                   │            │
│    │                                          │            │
│    │ [Télécharger .zip]  [Pousser sur GitHub]│            │
│    └─────────────────────────────────────────┘            │
└──────────────────────────────────────────────────────────┘
```

### Points critiques UX
- **L'étape 4 (validation du plan) est obligatoire** — jamais de génération "boîte noire"
- L'utilisateur peut éditer le plan avant de lancer la génération
- Chaque snippet utilisé est **cliquable** (ouvre son détail)

### Métriques
- **Taux de validation du plan** : cible > 80 % (si < 50 %, le plan est mal fait)
- **Taux de réutilisation vs création** : cible > 70 % de réutilisation
- **Durée totale génération** : cible < 30 s pour un composant simple

---

## 📜 Parcours 5 — Définir / cloner un rule_set

**Objectif** : imposer ses conventions à l'IA en < 3 minutes.

### Étapes

```
┌──────────────────────────────────────────────────────────┐
│ 1. Page "Rule Sets" (menu principal)                    │
│    - Liste : rule_sets défaut + miens                    │
│                      │                                    │
│                      ▼                                    │
│ 2. "Cloner le défaut React+Tailwind"                    │
│    - Copie instantanée dans mon espace                   │
│                      │                                    │
│                      ▼                                    │
│ 3. Éditeur dual-view                                     │
│    ┌──────────────────┬──────────────────────┐            │
│    │ Vue structurée   │ Vue JSON brute       │            │
│    │ (formulaires)    │ (Monaco + validation)│            │
│    └──────────────────┴──────────────────────┘            │
│                      │                                    │
│                      ▼                                    │
│ 4. Modification (ex: passer max_file_lines de 150 à 200) │
│    - Validation en live (JSON Schema)                   │
│    - Aperçu de l'impact ("Affectera tes prochaines       │
│      générations")                                       │
│                      │                                    │
│                      ▼                                    │
│ 5. Sauvegarder                                           │
│    - Option : "Utiliser par défaut pour mes projets"    │
└──────────────────────────────────────────────────────────┘
```

### Métriques
- **% d'utilisateurs avec ≥ 1 rule_set custom** : indicateur de profondeur d'usage
- **Taux de clonage du défaut vs création from scratch** : cible > 80 % cloné (bonne UX)

---

## 🌐 Parcours 6 — Publier & partager avec la communauté

**Objectif** : rendre un snippet public en < 60 secondes.

### Étapes

```
┌──────────────────────────────────────────────────────────┐
│ 1. Page d'un snippet privé → bouton "Publier"            │
│                      │                                    │
│                      ▼                                    │
│ 2. Checklist de publication                              │
│    ✓ Description remplie                                 │
│    ✓ Au moins 1 tag                                      │
│    ✓ Aucun secret détecté (scan auto)                    │
│    ⚠ Attention : email détecté dans le code (faux       │
│       positif ? À revoir)                                │
│                      │                                    │
│                      ▼                                    │
│ 3. Licence (MIT par défaut, ou autre)                   │
│                      │                                    │
│                      ▼                                    │
│ 4. Preview de la card publique                           │
│    - Aperçu tel qu'il apparaîtra sur Explore             │
│                      │                                    │
│                      ▼                                    │
│ 5. Publier                                               │
│    - URL publique : komorebi.dev/@user/slug             │
│    - Bouton "Copier le lien"                             │
│    - Partage : Twitter, HackerNews, Reddit               │
└──────────────────────────────────────────────────────────┘
```

### Cas d'usage communauté
- **Forker** un snippet public → copie privée modifiable + lien de provenance
- **Étoiler** → bookmark + signal de popularité
- **Suivre un créateur** → feed des nouveaux snippets (v1.1)

---

## 💳 Parcours 7 — Upgrade vers Premium

**Objectif** : convertir un utilisateur actif en payant, **au bon moment**.

### Déclencheurs (pas avant !)
1. User atteint 40/50 snippets privés → bannière douce "Tu approches de la limite"
2. User atteint 30/30 générations du mois → modale **uniquement quand il essaie la 31e**
3. User crée son 2e projet → "Un 3e nécessitera Premium"

### Étapes (modale d'upsell)

```
┌──────────────────────────────────────────────────────────┐
│ 1. Modale contextualisée                                 │
│    "Tu as utilisé 30/30 générations ce mois."           │
│    "Passe Premium pour 500 générations/mois."           │
│                      │                                    │
│                      ▼                                    │
│ 2. Comparatif rapide Free vs Premium (3 lignes max)      │
│                      │                                    │
│                      ▼                                    │
│ 3. CTA : "Passer Premium — 9 €/mois"                    │
│    - Lien secondaire : "Plus de détails sur /pricing"   │
│    - Lien tertiaire : "Pas maintenant" (pas d'insisting)│
│                      │                                    │
│                      ▼                                    │
│ 4. Stripe Checkout (hors app)                            │
│                      │                                    │
│                      ▼                                    │
│ 5. Retour → confirmation + badge Premium activé          │
│    - Email de bienvenue                                  │
│    - Débloquage immédiat                                 │
└──────────────────────────────────────────────────────────┘
```

### Règles anti-friction
- Jamais de modale bloquante en pleine action
- Bouton "Fermer" toujours accessible
- Pas d'upsell pendant l'onboarding

---

## 🔄 Parcours 8 — Usage récurrent (fidélisation)

**Objectif** : que l'utilisateur revienne **chaque semaine**.

### Boucles d'engagement

```
Lundi matin : "Nouveau projet pour un client"
     │
     ▼
Ouvrir Komorebi → créer projet → appliquer rule_set
     │
     ▼
Génération IA : réutilise 80 % de la librairie existante
     │
     ▼
Pendant le développement : ajouter 2-3 nouveaux snippets découverts
     │
     ▼
Vendredi : publier un pack "Composants client X" (privé)
     │
     ▼
Semaine suivante : la librairie est plus riche → génération encore meilleure
```

### Leviers de rétention
- **Newsletter hebdo** (opt-in) : "Tes stats de la semaine" + snippet populaire de la communauté
- **Notifications** (discrètes) : quand un snippet forké est mis à jour par l'original
- **Gamification légère** (v1.1) : badges pour contributions communautaires
- **Rappel de maintenance** : "5 de tes snippets n'ont pas été mis à jour depuis 6 mois"

---

## 🗺️ Synthèse : parcours principal (happy path)

```
Landing → Signup → Onboarding → 1er snippet → 1ère recherche
    → 1ère génération IA → Création d'un rule_set → Publication
    → Usage récurrent → Quota atteint → Upgrade Premium
```

Chacune de ces étapes est un **goulot d'étranglement** à mesurer et optimiser.

## 📊 Dashboard de suivi (interne)

À construire dès la Phase 7, avec les taux de conversion entre chaque étape du parcours. Cet outil guide les priorités produit post-MVP.

## 🚫 Anti-parcours (à éviter absolument)

- ❌ Demander une carte bancaire au signup
- ❌ Modale d'upsell dès la 1re connexion
- ❌ Forcer un tutoriel vidéo
- ❌ Masquer des features payantes sans les annoncer
- ❌ Rendre l'export des snippets payant (propriété user = sacrée)
- ❌ Captcha pour une action simple (créer un snippet)

## 📚 Références croisées
- [vision.md](./vision.md) — personas complets
- [plans/03-frontend-studio.md](./plans/03-frontend-studio.md) — implémentation UI
- [plans/07-monetization.md](./plans/07-monetization.md) — limites Free/Premium
