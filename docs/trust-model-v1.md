# Trust model actuel

> Version : 1.2
> Dernière mise à jour : 2026-05-16
> Portée : règles de confiance actives dans UseStakly post-pivot

## Objectif

UseStakly ne publie pas un flag toxique juste parce qu'un utilisateur l'a signalé.
Le trust model v1 sert à réduire :

- le bruit
- le spam
- le poisoning par agent ou faux compte
- les suppressions silencieuses par owner

## Briques actives

### 1. Réputation utilisateur

Chaque utilisateur possède une réputation calculée à partir de signaux d'usage et d'ancienneté.

Le score actuel prend en compte notamment :

- ancienneté du compte
- volume de signaux passifs
- `resolve`
- `re_resolve`
- `build_success`
- `build_failure`
- `regret`

La version runtime actuelle est une **réputation v2 légère** :

- poids positif pour le vrai usage observé
- bonus pour les outcomes positifs
- bonus pour la fiabilité build
- pénalité explicite sur le regret

L'UI compte expose aussi des métriques explicatives dérivées :

- `usage_signal_count`
- `successful_outcome_ratio`
- `build_reliability_ratio`
- `regret_ratio`

Cette réputation est exposée dans l'API compte et dans l'UI `/account`.

### 2. Éligibilité aux signaux actifs

Un utilisateur ne compte pour les signaux actifs publics que s'il est éligible.

Conditions v1 :

- score de réputation au-dessus du seuil configuré
- compte suffisamment ancien
- volume minimal de signaux passifs
- minimum de vrai usage (`resolve`, `re_resolve`, `build_success`, `build_failure`, `regret`)

Un compte neuf avec un bon score brut ne doit pas compter trop vite dans le consensus public.

Depuis le 2026-05-16, `scoring/formula_v2.toml` expose aussi une section `[trust]` :

- `new_account_active_signal_weight = 0.0`
- `min_real_usage_for_active_weight = 2`
- `owner_dispute_min_reputation = 0.35`
- `severe_signal_low_trust_review = true`

Conséquence : un compte qui a peu de vrai usage peut passer certains garde-fous d'API, mais son poids de review actif reste nul pour les signaux sévères jusqu'à ce qu'il ait au moins deux signaux d'usage réels. Ces signaux partent donc en review stricte au lieu de pouvoir nourrir trop vite le flux public.

### 3. Consensus avant exposition publique

Les flags actifs publics de `artifact_scores.flags` ne sont exposés que si plusieurs utilisateurs distincts et éligibles convergent.

Règles v1 :

- flags standards : consensus par défaut
- flags sévères (`security_issue`, `broken`) : consensus plus élevé
- déduplication par utilisateur
- normalisation de sortie (`security_issue` → `security-issue`)

### 4. Review admin

Les signaux sensibles ne deviennent pas automatiquement publics.

Cas actif v1 :

- `security_issue` entre en `pending`
- review admin nécessaire avant exposition publique

Le runtime actuel ajoute une couche v2 légère :

- un reporter à faible poids trust (`unproven` / score faible) déclenche aussi une **review stricte** pour certains signaux actifs sévères
- cela concerne aujourd'hui `security_issue`, `broken` et `doesnt_match_claim`
- l'objectif est d'éviter qu'un compte juste au-dessus du seuil d'éligibilité fasse entrer trop facilement un signal sévère dans le flux public
- la note de soumission journalise `active-weight`, ce qui permet de distinguer un reporter expérimenté d'un compte encore trop neuf

La review admin passe par les endpoints d'administration et par le panneau de modération du compte.

La file de modération expose maintenant aussi un contexte reporter :

- `reporter_tier`
- `reporter_score`
- volume de signaux d'usage
- indicateur `needs_strict_review`

### 5. Dispute owner

Un owner GitHub peut disputer un signal concernant son repo.

Important :

- la dispute n'efface pas silencieusement un signal accepté
- elle ouvre une nouvelle phase de review
- les transitions restent visibles dans l'audit trail

Support owner v1 :

- owner user GitHub direct
- membre public d'une organisation GitHub propriétaire

Le runtime actuel ajoute aussi un contexte trust owner à la modération :

- la dispute journalise désormais le niveau trust de l'owner qui conteste
- la file admin expose le score / tier / usage de cet owner
- un signal accepté puis disputé revient dans la boucle de review admin au lieu de disparaître du radar opérationnel
- la note de dispute inclut maintenant `owner-confidence=normal|low-trust-review` selon le seuil `owner_dispute_min_reputation`

Le support owner a aussi été étendu en **best effort** :

- owner direct GitHub
- membre public d'organisation GitHub
- membre privé d'organisation si le `GITHUB_TOKEN` serveur a les droits suffisants pour confirmer la membership
- collaborateur / maintainer repo si l'API GitHub peut confirmer un niveau de permission compatible

Important :

- cette vérification dépend encore des droits réels du `GITHUB_TOKEN` serveur
- si GitHub ne laisse pas confirmer une membership ou une permission, UseStakly reste volontairement conservateur et n'accorde pas le droit owner

Limite connue :

- memberships privés et rôles fins d'organisation non supportés

### 6. Audit trail

Les événements de signal sont journalisés.

Transitions typiques :

- `submitted`
- `review_accepted`
- `review_rejected`
- `disputed`

Ce journal est visible sur le profil repo et sert de garde-fou contre les dérives de modération.

### 7. Garde-fous MCP write

Les tools MCP write (`log_usage`, `watch_repo`) sont protégés par :

- tokens hashés
- quota write par token
- cooldown anti-spam
- fenêtre de refroidissement sur outcomes négatifs répétés
- réputation trust minimale pour certains outcomes négatifs
- historique d'usage sain exigé avant certains `build_failure` / `regret` / `re_resolve`
- notes minimales pour les outcomes négatifs les plus sensibles

## Ce que le modèle actuel garantit

- un flag toxique public n'est pas basé sur un seul compte
- un owner ne peut pas faire disparaître un signal seul
- un agent token ne peut pas flooder indéfiniment
- les transitions importantes sont auditables

## Ce que le modèle actuel ne garantit pas encore

- résistance forte aux Sybil attacks
- support complet des organisations GitHub privées
- double validation admin pour tous les cas sensibles

## Suite logique

La suite du trust model devrait prioritairement ajouter :

- graphe Sybil-resistant via OAuth GitHub (followers, contributions, âge compte, historique repo)
- ownership org privé / rôles fins
- anti-poisoning avancé sur `log_usage`
- règles de review plus fortes pour les signaux sévères
