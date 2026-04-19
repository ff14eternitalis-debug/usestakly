# UseStakly — Secrets Rotation Playbook

> Version: 1.0  
> Dernière mise à jour: 2026-04-18

## Objectif

Ce document décrit comment reprendre la main sur tous les secrets du projet **sans les montrer à l'assistant**.

Principe directeur:

- si une valeur sensible a été affichée pendant une session d'assistance, elle doit être considérée comme **potentiellement compromise**
- elle doit donc être **remplacée**
- les nouvelles valeurs doivent être **générées et saisies par le propriétaire du projet uniquement**

Ce playbook est volontairement rédigé sans aucune valeur réelle.

---

## Règle d'or

Pour la suite du projet:

- l'assistant peut aider sur les **noms** de variables
- l'assistant peut aider sur leur **format**
- l'assistant peut aider sur **où les saisir**
- l'assistant peut aider sur les **tests après rotation**
- l'assistant ne doit pas recevoir les **valeurs**

---

## Secrets à traiter en priorité

### 1. Base PostgreSQL

À considérer comme à faire tourner:

- mot de passe PostgreSQL
- `DATABASE_URL` du backend

Pourquoi:

- pendant la phase de déploiement et de debug, la base a été utilisée avec une URL connue pendant la session
- la base a aussi été rendue publique temporairement pour débloquer l'hébergement

### 2. Session backend

À créer ou remplacer:

- `APP_SESSION_SECRET`

Pourquoi:

- ce secret signe les sessions backend
- il doit être long, aléatoire, unique, et connu uniquement du propriétaire

### 3. GitHub OAuth

À créer au moment de brancher le vrai login:

- `GITHUB_CLIENT_ID`
- `GITHUB_CLIENT_SECRET`

Pourquoi:

- ce sont les credentials du futur login GitHub
- ils ne doivent jamais être saisis dans une conversation

### 4. Token d'administration Coolify

À faire tourner si tu veux repartir d'une base totalement saine:

- token API Coolify

Pourquoi:

- ce token donne accès à l'infra
- c'est un secret d'administration, pas un secret applicatif

---

## Variables d'environnement cibles

### Backend Coolify

Variables minimales à contrôler:

- `DATABASE_URL`
- `APP_SESSION_SECRET`
- `APP_BASE_URL`
- `FRONTEND_BASE_URL`
- `GITHUB_CLIENT_ID`
- `GITHUB_CLIENT_SECRET`
- `RUST_LOG`

### Frontend Coolify

Variables minimales à contrôler:

- `VITE_API_BASE_URL`

### Local / développement

Variables à garder uniquement en local:

- `DEV_USER_ID`
- `DEV_USER_EMAIL`
- `DEV_USER_USERNAME`
- `DEV_USER_DISPLAY_NAME`
- `DEV_USER_AVATAR_URL`

Important:

- les variables `DEV_USER_*` ne sont là que pour le fallback MVP tant que l'auth réelle n'est pas activée
- elles ne doivent pas être traitées comme des secrets critiques

---

## Ordre de rotation recommandé

Le bon ordre est le suivant:

1. Préparer les nouvelles valeurs hors assistant
2. Mettre à jour la base PostgreSQL
3. Mettre à jour `DATABASE_URL` du backend
4. Définir un nouveau `APP_SESSION_SECRET`
5. Redéployer le backend
6. Vérifier `GET /health`
7. Vérifier `GET /api/me`
8. Ensuite seulement créer les secrets GitHub OAuth
9. Enfin, si souhaité, régénérer le token API Coolify

---

## Procédure détaillée

### Étape 1 — Générer les nouvelles valeurs

À faire toi-même uniquement.

Tu dois préparer:

- un nouveau mot de passe PostgreSQL
- un nouveau `APP_SESSION_SECRET`
- plus tard, un `GITHUB_CLIENT_SECRET`

Recommandations:

- longueur élevée
- aléatoire
- jamais réutilisé
- stocké dans ton gestionnaire de mots de passe

Exemple PowerShell pour générer un secret long:

```powershell
[Convert]::ToBase64String((1..64 | ForEach-Object { Get-Random -Minimum 0 -Maximum 256 }))
```

Ne colle jamais la sortie dans le chat.

---

### Étape 2 — Faire tourner le mot de passe PostgreSQL

Dans Coolify:

1. Ouvre la ressource PostgreSQL `usestakly-postgres`
2. Change le mot de passe de l'utilisateur de base
3. Sauvegarde
4. Note la nouvelle URL de connexion, sans la montrer à l'assistant

Objectif:

- invalider totalement l'ancienne valeur

---

### Étape 3 — Mettre à jour `DATABASE_URL`

Dans Coolify:

1. Ouvre l'application `usestakly-backend`
2. Ouvre les variables d'environnement
3. Remplace `DATABASE_URL`
4. Sauvegarde

Recommandation importante:

- préfère une URL **interne** au réseau Coolify plutôt qu'une URL publique
- si la base et l'application tournent sur la même instance Coolify, l'accès interne est la cible correcte

La doc Coolify indique que les services peuvent communiquer sur le réseau interne et recommande l'usage de variables de base de données internes ou d'URLs internes quand les ressources sont sur le même réseau. Source utilisée: docs Coolify via Context7 sur la communication interne et les variables d'environnement.

---

### Étape 4 — Définir `APP_SESSION_SECRET`

Dans Coolify, application backend:

1. Ajoute ou remplace `APP_SESSION_SECRET`
2. Colle la nouvelle valeur
3. Sauvegarde

Règles:

- très longue valeur aléatoire
- une seule source de vérité
- ne jamais l'écrire dans le repo
- ne jamais la partager dans une conversation

Effet attendu:

- toutes les sessions signées avec l'ancienne clé deviennent invalides

---

### Étape 5 — Redéployer le backend

Une fois `DATABASE_URL` et `APP_SESSION_SECRET` mis à jour:

1. Redéploie `usestakly-backend`
2. Attends que l'application revienne

Vérifications attendues:

- `GET /health` doit répondre `200`
- `GET /api/me` doit répondre normalement

Tu peux donner à l'assistant seulement:

- le code HTTP
- le message d'erreur si ça casse

Tu n'as jamais besoin de montrer la valeur des variables.

---

### Étape 6 — Brancher GitHub OAuth proprement

Quand tu seras prêt:

1. Crée une OAuth App GitHub
2. Configure le callback backend
3. Renseigne dans Coolify:
   - `GITHUB_CLIENT_ID`
   - `GITHUB_CLIENT_SECRET`
4. Redéploie le backend

Ne montre jamais:

- le client secret GitHub

Le `client_id` est moins sensible, mais la meilleure discipline reste de ne montrer aucune valeur.

---

### Étape 7 — Tourner le token API Coolify

Option recommandée si tu veux une remise à zéro complète de la confiance:

1. Régénère le token API Coolify
2. Mets à jour ton CLI local
3. Supprime toute ancienne configuration inutile

Objectif:

- repartir avec un secret d'infra que l'assistant n'a jamais vu

---

## Vérifications après rotation

Après la rotation, vérifie:

- le frontend répond
- le backend répond sur `/health`
- `/api/me` répond
- le backend lit bien la base
- le login GitHub marche quand il sera activé

Si quelque chose casse, tu peux partager avec l'assistant:

- le code HTTP
- le message d'erreur
- le fichier concerné
- le log applicatif

Tu ne dois pas partager:

- les secrets
- les URLs complètes avec mot de passe
- les tokens d'admin

---

## Cible sécurité long terme

La cible saine pour UseStakly est:

- base PostgreSQL non publique, ou strictement limitée
- `DATABASE_URL` interne Coolify
- `APP_SESSION_SECRET` connu uniquement du propriétaire
- `GITHUB_CLIENT_SECRET` connu uniquement du propriétaire
- token API Coolify régénéré si nécessaire
- aucun secret réel dans le repo
- aucun secret réel dans `.env.example`

---

## Checklist courte

- [ ] Générer un nouveau mot de passe PostgreSQL
- [ ] Mettre à jour la base Coolify
- [ ] Remplacer `DATABASE_URL` du backend
- [ ] Générer un nouveau `APP_SESSION_SECRET`
- [ ] Le définir dans Coolify
- [ ] Redéployer le backend
- [ ] Vérifier `/health`
- [ ] Vérifier `/api/me`
- [ ] Créer l'OAuth App GitHub
- [ ] Définir `GITHUB_CLIENT_ID`
- [ ] Définir `GITHUB_CLIENT_SECRET`
- [ ] Redéployer le backend
- [ ] Régénérer le token API Coolify si souhaité

