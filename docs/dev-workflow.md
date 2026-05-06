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

## Refresh quotidien des données GitHub

Le scheduler est opt-in. Quand il est actif, il tourne toutes les 24 h par défaut et rafraîchit :

- les repos présents dans les watchlists ;
- les repos GitHub du corpus dont `priors_fetched_at` est absent ou vieux de plus de 24 h.

Dans `.env` :

```powershell
APP_SCHEDULER_ENABLED=true
APP_RECOMPUTE_INTERVAL_SECS=86400
```

Il faut aussi un `GITHUB_TOKEN`, sinon le scheduler saute le refresh GitHub et se limite au recompute.

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

## Résolution de problèmes fréquents

### `Accès refusé` sur `target/...exe` au build

Windows Defender ou un ancien process bloque le binaire.

```powershell
Get-Process usestakly-backend -ErrorAction SilentlyContinue | Stop-Process -Force
```

Fix durable (PowerShell admin, à faire une fois) :

```powershell
Add-MpPreference -ExclusionPath "C:\Users\forgo\Documents\Code\Project-DK\Project-K\backend\target"
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
