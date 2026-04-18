# UseStakly — Stratégie de Déploiement Coolify

> Version : 1.0 — 2026-04-18
> Statut : stratégie d'hébergement cible MVP
> Référence : documentation Coolify consultée via Context7 le 2026-04-18

## Objectif

Héberger le MVP de `UseStakly` entièrement sur **Coolify** :

- frontend
- backend
- base PostgreSQL

L'objectif est d'éviter une architecture éclatée entre plusieurs providers alors que le MVP doit rester simple à déployer, simple à opérer et simple à comprendre.

---

## Décision d'hébergement

### Hébergement cible retenu

- **Frontend** : Coolify
- **Backend Rust** : Coolify
- **PostgreSQL** : base managée sur Coolify

### Conséquence directe

Le MVP n'est plus pensé comme :

- frontend sur Vercel
- backend sur Fly.io / Railway
- base sur Supabase / Neon

Le MVP est maintenant pensé comme :

- **un déploiement unifié sur Coolify**

---

## Architecture cible MVP sur Coolify

```text
Coolify Project: UseStakly
├── Application: usestakly-frontend
├── Application: usestakly-backend
└── Database: usestakly-postgres
```

### Recommandation

- frontend séparé
- backend séparé
- base séparée

Ce découpage permet des déploiements indépendants, des variables séparées et un rollback plus simple.

---

## Domaines recommandés

- frontend : `usestakly.com`
- backend : `api.usestakly.com`

---

## Variables d'environnement recommandées

### Frontend

```env
VITE_API_BASE_URL=https://api.usestakly.com
VITE_SUPABASE_URL=
VITE_SUPABASE_ANON_KEY=
```

### Backend

```env
DATABASE_URL=
APP_HOST=0.0.0.0
APP_PORT=4000
APP_BASE_URL=https://api.usestakly.com
FRONTEND_BASE_URL=https://usestakly.com
RUST_LOG=info
SUPABASE_URL=
SUPABASE_JWT_JWKS_URL=
SUPABASE_JWT_ISSUER=
```

### Note importante

L'hébergement sur Coolify et la stratégie d'auth sont **deux décisions séparées**.

Donc :

- hébergement cible = Coolify
- auth MVP documentée actuellement = GitHub + Supabase Auth

---

## Recommandations issues de la doc Coolify

La documentation consultée confirme des patterns utiles pour le MVP :

- les **bases managées** conviennent très bien pour PostgreSQL
- les **applications séparées** sont un pattern naturel
- les variables d'environnement doivent être injectées par la plateforme
- les variables critiques peuvent être rendues obligatoires

Cela confirme la stratégie `frontend + backend + postgres` séparés.

---

## Plan recommandé

1. créer le projet Coolify `UseStakly`
2. créer la base managée `usestakly-postgres`
3. déployer l'application backend depuis `backend/`
4. injecter `DATABASE_URL` et les variables backend
5. déployer l'application frontend depuis `frontend/`
6. configurer `usestakly.com` et `api.usestakly.com`
7. vérifier `frontend`, `/health` et la connexion DB

---

## Décision finale

> `UseStakly` sera déployé sur Coolify avec un frontend séparé, un backend séparé, et une base PostgreSQL managée sur la même plateforme.
