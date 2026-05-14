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
- base et auth sur un SaaS externe (Supabase, Neon, etc.)

Le MVP est maintenant pensé comme :

- **un déploiement unifié sur Coolify (VPS auto-hébergé)**
- auth OAuth implémentée directement dans le backend Rust (GitHub + Discord), pas de service d'auth externe

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
```

### Backend

```env
DATABASE_URL=
APP_HOST=0.0.0.0
APP_PORT=4000
APP_BASE_URL=https://api.usestakly.com
FRONTEND_BASE_URL=https://usestakly.com
APP_SESSION_SECRET=
APP_NOTIFICATION_SECRET=
APP_EMAIL_SMTP_HOST=smtp-relay.brevo.com
APP_EMAIL_SMTP_PORT=587
APP_EMAIL_SMTP_USERNAME=
APP_EMAIL_SMTP_PASSWORD=
APP_EMAIL_FROM_ADDRESS=noreply@usestakly.com
APP_EMAIL_FROM_NAME=UseStakly
GITHUB_CLIENT_ID=
GITHUB_CLIENT_SECRET=
DISCORD_CLIENT_ID=
DISCORD_CLIENT_SECRET=
APP_SCHEDULER_ENABLED=true
APP_RECOMPUTE_INTERVAL_SECS=86400
APP_DIGEST_INTERVAL_SECS=1800
RUST_LOG=info
```

### Note importante

L'app est auto-hébergée sur VPS via Coolify. L'auth est implémentée directement dans le backend Rust :

- OAuth GitHub + Discord callbacks servis par le backend
- session persistée dans un cookie JWT signé avec `APP_SESSION_SECRET`
- destinations de notification sensibles (ex: Discord webhook) chiffrées avec `APP_NOTIFICATION_SECRET`, séparé du secret de session
- alertes email via Brevo SMTP si `APP_EMAIL_SMTP_USERNAME` et `APP_EMAIL_SMTP_PASSWORD` sont configurés ; l'expéditeur vérifié est `UseStakly <noreply@usestakly.com>`
- scheduler opt-in : refresh GitHub quotidien via `APP_RECOMPUTE_INTERVAL_SECS`, digests email/Discord vérifiés toutes les 30 min via `APP_DIGEST_INTERVAL_SECS`
- aucune dépendance à un SaaS d'auth (Supabase, Auth0, Clerk...) — pas de valeur ajoutée sur un VPS auto-hébergé

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
