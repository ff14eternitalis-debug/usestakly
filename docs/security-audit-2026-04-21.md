# Audit de sécurité — post-pivot GitHub observability

> Version : 2 — mise à jour le 2026-04-23  
> Portée : produit vivant **UseStakly** après pivot GitHub, incluant `repos/add`, watchlist, MCP read/write, signaux actifs, modération, dispute owner et audit trail.  
> Branche : `main`

## Résumé

Le socle actuel est **raisonnablement sûr pour un MVP fermé / early access**, avec des garde-fous désormais présents sur les surfaces les plus risquées :

- auth OAuth côté backend, session cookie HttpOnly
- MCP Bearer tokens hashés en base
- quotas write MCP par token
- réputation minimale avant signaux actifs
- consensus multi-users avant exposition publique des flags
- review admin pour `security_issue`
- dispute owner sans suppression silencieuse
- audit trail des transitions de signal

Je ne vois pas de vulnérabilité évidente de niveau critique dans l’état actuel du code. En revanche, il reste des **risques produit / trust** importants avant ouverture large, surtout autour de l’identité GitHub d’organisation et de la modération humaine.

## Changements de sécurité significatifs depuis l’audit initial

### 1. Surface snippets retirée du runtime

Les anciennes routes snippets/libraries/resolve publiques ne participent plus au produit vivant. Cela réduit la surface HTTP exposée et supprime une source de confusion entre ancien et nouveau modèle.

### 2. `/api/repos/add` et ingestion GitHub

Le flow public d’ingestion de repo existe maintenant. Les points positifs :

- parsing strict de `owner/repo` ou URL GitHub
- erreurs GitHub mieux typées
- dépendance explicite à `GITHUB_TOKEN`

Risques restants :

- pas encore de backoff/ETag/gestion de quota GitHub avancée
- un utilisateur peut proposer des repos arbitraires, ce qui est acceptable produit, mais peut créer du bruit si la gouvernance corpus reste floue

### 3. MCP write tools durcis

`log_usage` et `watch_repo` sont désormais protégés par :

- quota write par token
- cooldown anti-doublon
- fenêtre de refroidissement sur outcomes négatifs

Cela réduit le spam et le poisoning trivial. Le point faible restant est la **qualité de la réputation v1** : elle est correcte pour un MVP, mais encore trop simple pour une ouverture publique agressive.

### 4. Signaux actifs et flags toxiques

Le risque le plus important du produit était ici. L’état actuel est bien meilleur :

- evidence obligatoire pour les signaux actifs sensibles
- seuil réputation minimal avant soumission
- `security_issue` démarre en `pending`
- seuls les signaux `accepted` et portés par assez de users éligibles alimentent les flags publics
- une dispute owner ajoute une contestation mais ne retire pas magiquement un signal accepté

Cette partie est maintenant défendable pour un MVP fermé.

## Contrôles revérifiés

### Auth web — OK

- session cookie `usestakly_session`
- fallback dev user toujours présent mais connu et documenté
- pas de secret hardcodé ajouté

### Auth MCP — OK

- tokens `usk_<64 hex>`
- hash SHA-256 en base
- plaintext montré une seule fois
- révocation disponible

### Admin API — Correcte mais encore sensible

Le token admin reste simple et efficace. C’est acceptable tant que :

- le token est long et rotaté
- son usage reste limité
- les actions admin sont monitorées

Le panneau admin léger dans `/account` n’ajoute pas un nouveau modèle de sécurité : il ne fait qu’exposer la même API admin déjà existante. Le vrai risque reste donc la compromission du token admin lui-même.

### SQL injection — OK

Les requêtes restent paramétrées via SQLx. Aucun signe de concat SQL dangereuse dans les nouvelles briques de réputation, modération ou audit trail.

### Contrôle owner GitHub — Partiellement OK

Un repo peut maintenant être contesté par :

- son owner GitHub direct
- ou un membre **public** de l’organisation GitHub propriétaire

C’est une bonne amélioration MVP, mais cela ne couvre pas :

- membres privés d’org
- maintainers/collaborators sans membership public
- rôles fins côté org

Donc le contrôle owner n’est pas “faux”, mais il est **incomplet par design**.

## Risques restants

### 1. Membership GitHub d’organisation incomplète

C’est le principal point de fragilité trust actuel.

Conséquences :

- un vrai maintainer d’org avec membership privé peut être bloqué à tort
- inversement, la notion “owner” est encore plus proche de “owner ou membre public d’org” que de “maintainer métier”

Recommandation :

- documenter explicitement cette limite
- prévoir une v2 avec vérification GitHub plus riche si le produit s’ouvre

### 2. Modération humaine centralisée

Le process `pending -> accepted/rejected` repose encore sur un admin unique. Cela crée un risque opérationnel :

- erreur humaine
- décisions inconsistantes
- pression sociale si l’outil s’ouvre

Recommandation :

- journaliser systématiquement les reviews
- éventuellement double validation pour `security_issue`

### 3. Réputation v1 encore simple

La réputation actuelle suffit pour filtrer le bruit grossier, mais pas encore pour résister à une attaque sociale ou coordonnée à plus grande échelle.

Recommandation :

- enrichir avec plus de signaux GitHub
- pondérer davantage les historiques de contribution fiables
- différencier reporter / reviewer / owner

### 4. Dev user fallback

Toujours acceptable en local, toujours dangereux en prod si mal configuré. Rien de nouveau ici, mais le risque reste réel.

## État des garde-fous par surface

### `/api/repos/{id}/signals`

État : **beaucoup mieux maîtrisé**

- auth requise
- seuil réputation
- evidence
- `security_issue` en pending
- dispute owner
- audit trail

### `/mcp`

État : **acceptable pour early access**

- token dédié
- quota
- cooldown
- write guardrails

Reste à renforcer :

- réputation plus robuste côté write
- observabilité d’abus par token

### `/api/admin/*`

État : **fonctionnel mais sensible**

Le risque n’est pas dans le code métier, mais dans la gouvernance du token admin.

## Recommandations prioritaires

1. Documenter clairement la limite actuelle sur les owners d’organisation GitHub.
2. Ajouter des logs/metrics d’usage sur :
   - reviews admin
   - disputes owner
   - quotas MCP dépassés
3. Prévoir une v2 de réputation avant ouverture large.
4. Envisager une double review ou une checklist explicite pour `security_issue`.
5. Désactiver strictement le fallback dev user en prod.

## Checklist pré-ouverture

- [ ] `APP_SESSION_SECRET` défini en prod
- [ ] `ADMIN_API_TOKEN` rotaté et stocké côté secrets manager
- [ ] `GITHUB_TOKEN` avec droits minimaux nécessaires
- [ ] monitoring des reviews admin activé
- [ ] documentation claire sur la limite “owner org = membership public seulement”
- [ ] politique opérationnelle pour `security_issue` définie

## Conclusion

Le projet est passé d’un socle prometteur à un **MVP techniquement cohérent avec des garde-fous crédibles**.  
La sécurité applicative n’est plus le principal blocage immédiat. Le vrai enjeu restant est la **gouvernance trust** :

- qualité de la réputation
- qualité des reviews admin
- qualité de la preuve d’ownership GitHub pour les organisations

Pour un cercle restreint d’utilisateurs, c’est acceptable. Pour une ouverture large, il faudra une v2 de cette couche trust.
