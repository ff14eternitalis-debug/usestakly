# UseStakly — Documentation

> **Veille GitHub OSS avec scoring qualité**, consommable par des devs et des agents IA via MCP.
> Public beta exposable au 2026-04-26 (TODO v5.5). Ouverture publique large conditionnée à la finition ops MCP.

`UseStakly` est le nom produit retenu. `Project-K` reste le nom de travail historique encore présent dans certains chemins techniques (DB `project_k`, repo).

## Comment lire cette doc

| Tu es… | Lis dans cet ordre |
|---|---|
| Découverte du produit | `strategy-pivot-2026-04-21.md` → `strategy-quality-scored-registry.md` → `../README.md` |
| Tu codes | `../TODO.md` → `architecture-backend-current.md` → `mcp-protocol.md` |
| Tu opères / déploies | `deployment-coolify.md` → `ops-mcp-coolify-hardening.md` → `security-secrets-playbook.md` |
| Tu intègres un agent | `mcp-protocol.md` → `mcp-cli-release.md` → `mcp-examples.md` → `mcp-endpoint-security.md` |

## Index

### Source de vérité d'exécution
- [`../TODO.md`](../TODO.md) — checklist globale, version 5.5
- [`../CLAUDE.md`](../CLAUDE.md) — instructions agent Claude Code (synthèse projet)
- [`../AGENTS.md`](../AGENTS.md) — équivalent Codex
- [`../GEMINI.md`](../GEMINI.md) — équivalent Gemini

### Stratégie produit
- [`strategy-pivot-2026-04-21.md`](./strategy-pivot-2026-04-21.md) — scope produit actuel : repos GitHub publics OSS uniquement
- [`strategy-quality-scored-registry.md`](./strategy-quality-scored-registry.md) — moat et principes de scoring qualité (Team tier obsolète depuis le pivot)
- [`tech-stack.md`](./tech-stack.md) — choix techniques actuels
- [`user-journey.md`](./user-journey.md) — parcours user et agent vivants

### Architecture
- [`architecture-backend-current.md`](./architecture-backend-current.md) — découpage backend, sous-domaines
- [`trust-model-v1.md`](./trust-model-v1.md) — réputation, consensus, review, dispute
- [`coherence-audit-2026-04-23.md`](./coherence-audit-2026-04-23.md) — audit cohérence post-refacto sprints 1–4

### MCP
- [`mcp-protocol.md`](./mcp-protocol.md) — protocole MCP UseStakly v2 (post-pivot)
- [`mcp-endpoint-security.md`](./mcp-endpoint-security.md) — durcissement entrypoint HTTP `/mcp`
- [`mcp-cli-release.md`](./mcp-cli-release.md) — release CLI npm `usestakly-mcp`
- [`mcp-examples.md`](./mcp-examples.md) — exemples d'appels MCP

### Ops / déploiement
- [`deployment-coolify.md`](./deployment-coolify.md) — stratégie Coolify
- [`ops-mcp-coolify-hardening.md`](./ops-mcp-coolify-hardening.md) — durcissement ops avant ouverture large
- [`dev-workflow.md`](./dev-workflow.md) — démarrage local, commandes courantes
- [`security-secrets-playbook.md`](./security-secrets-playbook.md) — rotation secrets

### Audits et validations (snapshots datés)
- [`security-audit-2026-04-21.md`](./security-audit-2026-04-21.md) — audit sécu post-pivot
- [`audits/user-journey-audit-2026-04-23.md`](./audits/user-journey-audit-2026-04-23.md) — phase 1 (anonyme)
- [`audits/user-journey-audit-phase2-2026-04-24.md`](./audits/user-journey-audit-phase2-2026-04-24.md) — phase 2 (connecté), corrections livrées
- [`validation/formula-v1.1-smoke-test-2026-04-24.md`](./validation/formula-v1.1-smoke-test-2026-04-24.md) — smoke test scoring v1.1

### Plans
- [`plans/remaining-work-2026-05-03.md`](./plans/remaining-work-2026-05-03.md) — **vue priorisée du reste à terminer** (audit faux positifs + items vraiment ouverts)
- [`plans/refactor-plan-2026-04-23.md`](./plans/refactor-plan-2026-04-23.md) — refacto sprints 1 à 4 (terminés)
- [`plans/anti-slop-vitality-v2.md`](./plans/anti-slop-vitality-v2.md) — formula v2 vitality (livré, followup release_at)
- [`plans/source-of-truth-oss-radar-plan.md`](./plans/source-of-truth-oss-radar-plan.md) — radar maturity (phases 1/2/3/5 livrées, 4/6 partielles)
- [`plans/use-case-recommendation-watch-plan.md`](./plans/use-case-recommendation-watch-plan.md) — recherche par besoin (lots 1/2/3/5 livrés hors notifs, 4 MCP ouvert)
- [`plans/rename-to-usestakly.md`](./plans/rename-to-usestakly.md) — transition `Project-K` → `UseStakly`
- [`domain-proposals.md`](./domain-proposals.md) — propositions de noms de domaine

### Archives
- [`archive/snippets/`](./archive/snippets/) — docs de l'ancien produit snippets (abandonné au pivot)
- [`archive/business-prepivot/`](./archive/business-prepivot/) — analyses business pré-pivot (concurrence, marché, finance) — non valides depuis le 2026-04-21

> Les archives ne sont **plus une source de vérité**. Elles vivent pour la traçabilité de la réflexion produit, pas pour décrire l'état présent.

## Conventions

- Chaque doc active porte une **version** et une **date** dans son en-tête
- Les docs de stratégie ont une section "Décisions encore ouvertes" datée
- Les snapshots (audits, validations) sont **immuables** une fois écrits — créer un nouveau fichier daté plutôt que modifier l'ancien
- Les docs pré-pivot sont déplacées dans `archive/`, pas modifiées (préserve l'historique de réflexion)
