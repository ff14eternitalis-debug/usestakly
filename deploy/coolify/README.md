# UseStakly — Déploiement Coolify

Ce dossier prépare le déploiement MVP de `UseStakly` sur Coolify avec :

- une application `usestakly-backend`
- une application `usestakly-frontend`
- une base `usestakly-postgres`

## Structure recommandée dans Coolify

### Backend

- type : application depuis repo Git
- chemin racine : `backend`
- Dockerfile : `backend/Dockerfile`
- port exposé : `4000`
- domaine recommandé : `api.usestakly.com`
- healthcheck : `GET /health`

### Frontend

- type : application depuis repo Git
- chemin racine : `frontend`
- Dockerfile : `frontend/Dockerfile`
- port exposé : `8080`
- domaine recommandé : `usestakly.com`
- healthcheck : `GET /health`

### Database

- type : PostgreSQL managé Coolify
- nom recommandé : `usestakly-postgres`
- visibilité : privée

## Build args frontend

Le frontend doit recevoir au build :

- `VITE_API_BASE_URL`
- `VITE_SUPABASE_URL`
- `VITE_SUPABASE_ANON_KEY`

## Variables runtime backend

Le backend doit recevoir au runtime :

- `DATABASE_URL`
- `APP_HOST=0.0.0.0`
- `APP_PORT=4000`
- `APP_BASE_URL`
- `FRONTEND_BASE_URL`
- `RUST_LOG`
- `SUPABASE_URL`
- `SUPABASE_JWT_JWKS_URL`
- `SUPABASE_JWT_ISSUER`

## Ordre recommandé

1. créer la base PostgreSQL dans Coolify
2. déployer le backend
3. injecter `DATABASE_URL`
4. vérifier `GET /health`
5. déployer le frontend
6. injecter les build args `VITE_*`
7. configurer les domaines

## Fichiers utiles dans ce dossier

- `backend.env.example` : variables runtime backend
- `frontend.env.example` : variables build frontend
- `production-resources.md` : UUIDs déjà créés sur l'instance Coolify
- `create-apps.example.ps1` : commandes CLI prêtes à adapter pour créer les deux applications
