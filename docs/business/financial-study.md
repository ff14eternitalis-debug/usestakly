# Étude de Rentabilité Financière — UseStakly

> Analyse des flux de revenus, de la structure des coûts et des perspectives de croissance (Projection 2026-2027).

## 1. Modèle Économique (Freemium & SaaS)

*   **Plan Free :** Bibliothèque locale, recherche sémantique limitée, 0 €.
*   **Plan Pro (B2C) :** Synchronisation Cloud, recherche illimitée, accès aux Packs premium, **12 € / mois**.
*   **Plan Team (B2B) :** Bibliothèque partagée, Mode Architecte (Rules forcées pour l'équipe), **29 € / utilisateur / mois**.
*   **Marketplace :** Commission de 25 % sur la vente de packs de snippets tiers.

## 2. Structure des Coûts (Burn Rate)

Grâce à l'architecture Rust et au traitement local, les coûts d'infrastructure sont minimisés.

### Coûts Fixes (Infrastructure de base)
*   Hébergement Coolify (frontend + backend + PostgreSQL) : dépend surtout du serveur retenu.
*   Outillage (Monitoring, Sentry) : ~40 € / mois.
*   **Total Fixe : à recalculer selon le serveur Coolify cible.**

### Coûts Variables (Unit Economics)
*   Tokens LLM Fallback (Détection échouée) : ~0,05 € / user / mois.
*   Stockage Cloud (Synchronisation) : ~0,02 € / user / mois.
*   **Marge Brute sur abonnement Pro : > 99 %.**

---

## 3. Analyse du Point Mort (Break-Even)

Pour couvrir les coûts fixes (220 €) :
*   **Scénario Pro :** 19 abonnés.
*   **Scénario Team :** 8 utilisateurs.

## 4. Projections à 12 Mois

| Métrique | Valeur Cible |
| :--- | :--- |
| Utilisateurs Actifs | 5 000 |
| Taux de Conversion Pro | 3 % (150 users) |
| Revenu Mensuel Récurrent (MRR) | **~2 500 €** |
| Charges Mensuelles | ~570 € |
| **Bénéfice Net Mensuel** | **~1 930 €** |

---

## 5. Avantages Financiers du Modèle "Local-First"

1.  **Indépendance aux APIs LLM :** La recherche sémantique (vecteurs) est faite sur la machine de l'utilisateur. Pas de facture OpenAI/Anthropic à chaque recherche.
2.  **Efficacité Rust :** Faible consommation mémoire, permettant de supporter des milliers de connexions sur des instances serveurs minimales.
3.  **Scalabilité "Infrastucture-less" :** Le coût de calcul est porté par l'utilisateur final (CPU/GPU local), et non par l'entreprise.
