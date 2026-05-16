# Dev workflow

> Version : 1.0 — 2026-04-20

Guide pratique pour travailler sur UseStakly au quotidien : démarrage, commandes courantes, principes d'automatisation.

## Prérequis installés

| Outil | Version de référence | Rôle |
|---|---|---|
| Rust + Cargo | 1.94+ | Backend |
| Node + npm | 22+ | Frontend |
| Docker Desktop | 29+ | Postgres local via compose |
| `sqlx-cli` | 0.8+ | Gestion des migrations |
| `cargo-watch` | 8+ | Rebuild auto backend |
| `cargo-nextest` | 0.9+ | Runner de tests Rust |
| `psql` | 17 | Client Postgres |
| `jq` | 1.8+ | Parsing JSON en terminal |
| `gh` | 2.90+ | CLI GitHub |

## Démarrer le stack

### Voie rapide (script)

```powershell
.\dev.ps1
```

Ce script :

1. Lance Postgres via `docker compose up -d`.
2. Ouvre une fenêtre PowerShell **Backend** — `cargo watch -x run`.
3. Ouvre une fenêtre PowerShell **Frontend** — `npm run dev`.

Endpoints :

- Backend : <http://localhost:4000>
- Frontend : <http://localhost:5173>

### Voie manuelle (si besoin de contrôle fin)

```powershell
# Racine
docker compose up -d

# Terminal 1
cd backend
cargo watch -x run

# Terminal 2
cd frontend
npm run dev
```

## Vérifier que tout répond

```powershell
curl http://localhost:4000/health | jq
curl http://localhost:4000/api/me | jq
```

`/api/me` doit retourner le dev user (`usestakly-dev`) tant que `APP_SESSION_SECRET` n'est pas un vrai secret et que les credentials OAuth sont vides.

## Recherche sémantique locale (R2b)

La recherche sémantique est **opt-in**.

Dans `.env` :

```powershell
APP_SEMANTIC_SEARCH_ENABLED=true
```

Quand elle est activée :

- les nouveaux repos ingérés reçoivent un embedding local via `fastembed`
- le ranking search devient hybride lexical + sémantique + score qualité

Pour backfiller le corpus déjà présent :

```powershell
curl -X POST http://localhost:4000/api/admin/embeddings/backfill `
  -H "Content-Type: application/json" `
  -H "x-admin-token: <ADMIN_API_TOKEN>" `
  -d "{""limit"":100,""onlyMissing"":true}"
```

Notes :

- `onlyMissing=true` évite de retraiter les repos déjà embeddés
- lancer plusieurs batches si le corpus est large
- le premier lancement télécharge le modèle local `fastembed`, donc il peut être plus lent

## Refresh automatique des données GitHub

Le scheduler ingère et recalcule en boucle. En **production** (`APP_ENV=production`), il est **activé par défaut** avec un cycle toutes les **30 min** (surcharge via env).

À chaque cycle :

1. **Watchlist** — tous les repos suivis (non mutés), en priorité.
2. **Corpus** — repos dont `priors_fetched_at` est absent ou plus vieux que `APP_CORPUS_REFRESH_STALE_SECS`, jusqu'à remplir `APP_INGEST_MAX_REPOS_PER_CYCLE` (les plus anciens d'abord).
3. **Recompute** des scores et évaluation des veilles d'intention.

Dans `.env` (dev local) :

```powershell
APP_SCHEDULER_ENABLED=true
APP_RECOMPUTE_INTERVAL_SECS=3600
APP_CORPUS_REFRESH_STALE_SECS=3600
APP_INGEST_MAX_REPOS_PER_CYCLE=40
GITHUB_TOKEN=ghp_...
```

`GITHUB_TOKEN` est obligatoire pour l'ingestion ; sans lui, seul le recompute tourne.

## Commandes courantes (à taper à la main)

Ces commandes **n'ont pas de script dédié** par choix (voir section *Principes d'automatisation*).

### Arrêter le stack

```powershell
# Fermer les fenêtres PowerShell Backend et Frontend (Ctrl+C)
docker compose down
```

### Reset complet de la base (perte de données)

```powershell
docker compose down -v
docker compose up -d
```

`-v` supprime le volume `pg_data`. Le prochain `cargo run` rejoue les 9 migrations sur une base vide.

### Se connecter à la base en SQL

```powershell
psql -h localhost -U postgres -d project_k
# mot de passe : postgres
```

### Créer une nouvelle migration

```powershell
cd backend
sqlx migrate add <nom_migration>
```

Le fichier est créé dans `backend/migrations/`. Rappel : `sqlx::migrate!` est compile-time → toute nouvelle migration exige un rebuild du backend.

### Tests

```powershell
cd backend
cargo nextest run           # tous les tests
cargo nextest run <nom>     # un test précis
```

### Checks CI en local avant push

```powershell
cd backend
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test

cd ../frontend
npm run build
```

### E2E réel local sans mocks

Le filet Playwright par défaut (`npm run test:e2e`) garde des mocks API pour rester rapide et compatible CI.
Pour auditer le parcours réel avec Postgres local, backend lancé et API non mockée :

```powershell
cd frontend
npm run test:e2e:real
```

Ce script :

1. lance uniquement le Postgres du `docker-compose.yml` ;
2. recrée une base dédiée `project_k_e2e` pour éviter tout conflit avec la base dev ;
3. démarre le backend local sur `127.0.0.1:4100` ;
4. seed une base locale déterministe via `frontend/e2e/real-api-seed.sql` ;
5. lance `frontend/e2e/real-api.spec.ts`.

Parcours couvert : landing → discover → repo detail → watchlist → notifications → account token → MCP initialize/search.

Notes :

- Docker Desktop doit être démarré.
- Le script utilise le conteneur `usestakly-db` du projet, mais une DB dédiée `project_k_e2e` ; il ne touche pas à ta DB dev `project_k`.
- Il ne touche pas aux conteneurs d'autres projets, notamment `kois-story`.
- En fin de run, le script arrête le backend enfant et stoppe le Postgres compose.
- Ce test est un bon "release gate" local. Il n'est pas encore branché dans la CI principale.
- Implémentation : `frontend/scripts/run-real-e2e.mjs`. Checklist manuelle prod/staging : [`docs/validation/live-release-checklist.md`](validation/live-release-checklist.md).

### Smoke MCP (staging / prod)

Pour valider uniquement l'endpoint Streamable HTTP MCP sur une URL distante (sans Playwright) :

```powershell
# Depuis la racine du dépôt
$env:USESTAKLY_MCP_TOKEN = "usk_..."   # token monitoring depuis /account sur l'env cible
.\scripts\mcp-live-smoke.ps1 -Endpoint "https://api.usestakly.com/mcp" -Token $env:USESTAKLY_MCP_TOKEN
```

Local (backend sur `:4000`) :

```powershell
.\scripts\mcp-live-smoke.ps1 -Endpoint "http://127.0.0.1:4000/mcp" -Token "usk_..."
```

Le script enchaîne `initialize` → `search_github_repos` → `get_repo_quality_context` (équivalent checks **H2**, **H4**, **H5** dans [`docs/functional-checks.md`](functional-checks.md)). Option `-WriteSignal` appelle aussi `log_usage` (**H7**) et **persiste un signal** dans la base de l'environnement cible — ne pas l'utiliser en prod sans intention.

Alternative CLI : `npx usestakly-mcp test --endpoint … --token …` (voir section **I** de la checklist).

## Résolution de problèmes fréquents

### `Accès refusé` sur `target/...exe` au build

Windows Defender ou un ancien process bloque le binaire.

```powershell
Get-Process usestakly-backend -ErrorAction SilentlyContinue | Stop-Process -Force
```

Fix durable (PowerShell admin, à faire une fois) :

```powershell
Add-MpPreference -ExclusionPath "C:\Users\forgo\Documents\Code\usestakly\backend\target"
Add-MpPreference -ExclusionPath "C:\Users\forgo\.cargo"
Add-MpPreference -ExclusionPath "C:\Users\forgo\.rustup"
```

### `failed to connect to database`

Deux causes fréquentes :

1. **Conflit de port 5432** — un Postgres natif Windows occupe le port. Vérifier :
   ```powershell
   sc query postgresql-x64-17
   ```
   S'il est `RUNNING`, l'arrêter et le passer en manuel :
   ```powershell
   Stop-Service postgresql-x64-17
   Set-Service postgresql-x64-17 -StartupType Manual
   ```

2. **Docker pas démarré** — vérifier avec `docker ps`. Relancer Docker Desktop si nécessaire.

### Erreurs encodage non-UTF-8 sur erreurs sqlx

Symptôme du point précédent sur Windows avec locale fr-FR. Le vrai problème est toujours une erreur d'auth ou de connexion : résoudre celle-ci fait disparaître le message.

### Frontend : `cannot find module` après un pull

```powershell
cd frontend
npm install
```

## Principes d'automatisation

### Règle générale

> **Écris un script quand l'absence du script commence à te faire mal, pas avant.**

Chaque script est de la dette de maintenance. Une commande d'une ligne tapée à la main coûte 0 à maintenir. Un script qui fait la même chose doit rester synchro avec la réalité du projet à chaque changement de workflow.

### Critères pour justifier un script

Un script mérite d'exister **seulement** si au moins deux des conditions suivantes sont vraies :

1. La commande est tapée **5+ fois par semaine**.
2. La commande est **non-triviale** (plusieurs étapes, flags à mémoriser, env vars).
3. **Quelqu'un d'autre** doit pouvoir la lancer (onboarding, CI, collaborateur).

### Pourquoi `dev.ps1` existe

- Lancement quotidien (fréquence haute).
- 3 commandes dans 3 dossiers, 2 terminaux à ouvrir (complexité réelle).
- Économie de 30 secondes × 1 fois par jour × 365 = ~3 h / an. ROI positif.

### Pourquoi `stop.ps1`, `reset-db.ps1` n'existent pas

- Fréquence trop basse (fermer deux fenêtres + 1 commande, quelques fois par semaine au plus).
- Complexité quasi nulle.
- Aucun autre consommateur pour l'instant.

Les commandes brutes sont documentées dans la section *Commandes courantes* — si tu en écrits un script un jour, c'est que l'usage aura prouvé la valeur.

### Le piège à éviter

Un script créé « au cas où » dérive du réel dès la première évolution de workflow oubliée. Le jour où tu le relances après 3 mois, il exécute à moitié le bon truc. Tu perds plus de temps à comprendre qu'il ne t'en aurait fait gagner en 3 mois.

### Moment où recréer ces scripts sera justifié

- Onboarding d'un second dev sur le projet.
- Setup d'une CI qui a besoin d'une DB propre avant chaque run.
- Frustration répétée et mesurable : si tu tapes la même commande 10× dans une semaine, automatise.

## Documentation drift audit

Après toute modification de doc active (`AGENTS.md`, `CLAUDE.md`, `docs/*.md`, etc.) :

```powershell
.\scripts\audit-doc-source-truth.ps1
```

Le script échoue si des motifs obsolètes (ancienne version CLI npm, ancien décompte de migrations, ancien wording rate-limit MCP, chemins locaux historiques du monorepo, etc.) réapparaissent dans les fichiers actifs listés dans le script.

Voir aussi `docs/source-of-truth.md` pour le routage doc agent.
