# Phase 6 — Communauté & Publication

> Version : 1.0 — 2026-04-15 *(pré-pivot 2026-04-20 — à refondre)*
> Durée estimée : 1-2 semaines
> Dépendances : Phase 2, 3

> ### ⚠ Bandeau de reconciliation — pivot 2026-04-20
>
> Ce plan est **à refondre** post-pivot.
>
> - **Étoiles / forks sont anti-pattern** : ce sont exactement les signaux politiques (stars de hype, cargo cult) que le pivot rejette. Ils ne doivent pas être le signal de qualité.
> - La « publication publique » devient **annotation scoring** : les snippets publics sont scorés par usage réel (`resolve_count`, `build_success_rate`), pas par popularité sociale.
> - La page « Explore » reste valide mais doit être **triée par scoring qualité**, pas par étoiles/récence.
> - Le modèle peut s'étendre à l'**annotation de code public externe** (npm / GitHub / shadcn) — cf. strategy §💡. Décision de scope ouverte (MVP ou V2).
> - Le flow de publication doit capturer dès le jour 1 les signaux d'usage sur les snippets publiés.
>
> Voir [`../strategy-quality-scored-registry.md`](../strategy-quality-scored-registry.md) §⚠ Problèmes durs (anti-gaming, transparence du scoring, flags toxiques) pour le cadre de refonte.

## 🎯 Objectif

Permettre aux utilisateurs de **publier** des snippets, les **forker**, les **étoiler**, et découvrir ceux des autres. Poser les bases de l'effet réseau.

## 📋 Livrables

1. Flow de publication (privé → public) avec review manuelle optionnelle
2. Page "Explore" publique avec tri et recherche
3. Système d'étoiles et de forks
4. Profils publics d'utilisateurs
5. Packs de snippets (collections thématiques)

## 🔨 Tâches détaillées

### 6.1 Publication
- [ ] Endpoint `POST /snippets/:id/publish` : passe `visibility` de `private` à `public`
- [ ] Vérification qualité auto : au moins un titre, une description, pas de secret dans le code
- [ ] Prévisualisation de la carte publique avant de publier
- [ ] Badge "published on {date}"

### 6.2 Modération (légère)
- [ ] Scan automatique à la publication : détection de secrets (`regex` tokens, clés API)
- [ ] File d'attente manuelle si le scan détecte qqch de suspect
- [ ] Signalement par les utilisateurs (bouton "Report")

### 6.3 Page Explore
- [ ] Liste paginée des snippets publics récents
- [ ] Tris : récents / populaires (stars) / tendance (stars récents)
- [ ] Filtres : domain, kind, language, framework, tags
- [ ] Recherche sémantique globale (partagée avec la librairie perso)

### 6.4 Stars & forks
- [ ] `POST /snippets/:id/star` / `DELETE`
- [ ] `POST /snippets/:id/fork` → crée une copie privée chez le user avec lien de provenance
- [ ] Afficher "Forked from {user/slug}@{version}" sur les forks
- [ ] Compteur de stars et forks visible sur la card

### 6.5 Profils publics
- [ ] Route `/@:username` — avatar, bio, liste de snippets publics, stats
- [ ] Suivi (follow) en v1.1

### 6.6 Packs de snippets
- [ ] Nouvelle table `packs(id, owner_id, name, description, visibility)`
- [ ] Table de liaison `pack_snippets(pack_id, snippet_id)`
- [ ] UI : créer un pack, y ajouter des snippets
- [ ] Page publique d'un pack : "Design System Minimaliste par @zenmaster"

### 6.7 Dogfooding
- [ ] Komorebi publie ses propres packs (le design system de l'app, par exemple)
- [ ] Starter kits : "React+Tailwind minimal", "Rust API standard"

## ✅ Definition of Done

- [ ] Un user peut rendre public un snippet en 2 clics
- [ ] Les secrets dans le code sont détectés avant publication
- [ ] La page Explore affiche les snippets publics avec tri et filtres
- [ ] Un user peut forker un snippet et le modifier chez lui
- [ ] Les étoiles et forks sont comptabilisés correctement
- [ ] Les packs permettent de regrouper plusieurs snippets sous un nom

## ⚠️ Pièges à éviter

- ❌ Permettre la publication sans scan de secrets → fuite de clés API garantie
- ❌ Oublier la clause "par défaut privé" à la création
- ❌ Stocker une copie complète au fork → garder juste la référence à la version forkée + diff
- ❌ Ignorer le SEO des profils publics → meta tags dès le départ

## 📚 Références
- [data-model.md](../data-model.md) (tables `snippet_stars`, `snippet_forks`)
- [vision.md](../vision.md) (persona "créateur de contenu")
