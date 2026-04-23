# Phase 7 — Monétisation (Free / Premium)

> Version : 1.0 — 2026-04-15 *(pré-pivot 2026-04-20 — à refondre)*
> Durée estimée : 1 semaine
> Dépendances : Phase 2, 3, 6

> ### ⚠ Bandeau de reconciliation — pivot 2026-04-20
>
> Le modèle Free/Premium à ~9 €/mois décrit ici est **remplacé** par le modèle 4 tiers défini dans [`../strategy-quality-scored-registry.md`](../strategy-quality-scored-registry.md) §💰 :
>
> | Tier | Prix | Valeur |
> |---|---|---|
> | Free | 0 € | Registry perso + lecture scoring public |
> | Pro solo | ~12 €/mo | Registry privé, filtres custom, mode strict |
> | Team | ~40 €/user/mo | Registry équipe, reputation partagée, collecte CI |
> | Enterprise | contact | On-prem, compliance signals, SLA, rules custom |
>
> Le **vrai cash est Team** : dans une équipe de 15 devs, le graphe d'usage interne est le signal parfait pour cette équipe. Valeur perçue très élevée.
>
> Le reste du document (limites techniques par tier, Stripe, webhooks) reste inspirant mais les limites exactes sont à recalibrer.

## 🎯 Objectif

Mettre en place un **modèle Freemium** durable : usage illimité côté création, limites sur les fonctionnalités avancées.

## 💰 Modèle tarifaire (proposition MVP)

| Fonctionnalité | Free | Premium (~9 €/mois) |
|---|---|---|
| Snippets privés | 50 | ∞ |
| Snippets publics | ∞ | ∞ |
| Générations IA / mois | 30 | 500 |
| Projets | 2 | ∞ |
| Rule sets personnalisés | 1 | ∞ |
| Packs privés | 1 | ∞ |
| Export des snippets | ✅ | ✅ |
| Accès API MCP | ✅ (rate-limited) | ✅ (plus généreux) |
| Support | Communauté | Prioritaire |

## 📋 Livrables

1. Intégration Stripe (Checkout + webhooks)
2. Compteurs d'usage et enforcement des limites
3. Page `/pricing` publique
4. Page `/settings/billing` privée
5. Middleware de quota côté backend
6. Modèle provider-agnostic (prêt pour PayPal, crypto plus tard)

## 🔨 Tâches détaillées

### 7.1 Stripe Checkout
- [ ] Compte Stripe (ou sandbox) + clés API en env
- [ ] Produit `premium_monthly` + `premium_yearly` (-20 %)
- [ ] Endpoint `POST /billing/checkout` → URL Stripe Checkout
- [ ] Webhook `POST /billing/webhook` qui met à jour `subscriptions`
- [ ] Signature vérifiée côté serveur

### 7.2 Enforcement des limites
- [ ] Middleware de quota (lecture depuis `subscriptions` + compteurs)
- [ ] Réponse 402 Payment Required si quota dépassé, avec message clair
- [ ] Compteurs matérialisés (table `usage_counters` ou requête agrégée avec index)
- [ ] Reset mensuel automatique (job cron ou calcul à la volée par période)

### 7.3 UI
- [ ] Page `/pricing` avec comparatif des plans
- [ ] Badges "Free" / "Premium" sur le profil
- [ ] Modale d'upsell quand un quota est atteint ("Tu as utilisé 30/30 générations ce mois, passe Premium")
- [ ] Page `/settings/billing` : plan actuel, date de renouvellement, bouton "Gérer" (portail Stripe)

### 7.4 Analytics business
- [ ] Événements trackés : `signup`, `first_snippet`, `first_generation`, `quota_hit`, `checkout_started`, `subscription_created`, `subscription_canceled`
- [ ] Outil gratuit recommandé : PostHog self-hosted ou Plausible

### 7.5 Provider-agnostic
- [ ] Table `subscriptions.provider` (`stripe` / futur `paypal` / futur `crypto`)
- [ ] Trait Rust `BillingProvider` avec impl `StripeProvider`
- [ ] Interface prête pour ajouter d'autres providers sans migration

## ✅ Definition of Done

- [ ] Un user Free atteint ses 30 générations/mois → modale d'upsell
- [ ] Un checkout Stripe fonctionne de bout en bout (sandbox)
- [ ] Le webhook bascule l'user en Premium instantanément
- [ ] Les limites sont respectées côté backend (pas juste côté UI)
- [ ] Un user qui annule garde Premium jusqu'à la fin de période
- [ ] Les métriques business sont collectées et visibles sur un dashboard interne

## ⚠️ Pièges à éviter

- ❌ Vérifier le quota uniquement côté frontend → contournable trivialement
- ❌ Oublier de vérifier la signature des webhooks Stripe → vulnérable au fake
- ❌ Limiter trop agressivement le free tier → tue l'adoption
- ❌ Coupler fortement à Stripe → impossible de changer de provider plus tard
- ❌ Ne pas tester le cas "paiement échoué" et "période d'essai expirée"

## 📚 Références
- [data-model.md](../data-model.md) (table `subscriptions`)
- [vision.md](../vision.md) (personas, proposition de valeur)
