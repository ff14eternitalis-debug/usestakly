# Herald Analysis Report

**Repository:** `local/usestakly`  
**Branch:** `local`  
**Author:** REDIOUS  
**Date:** 2026-05-06 19:05:59 UTC  

---

## Score: 51 / 100 — Grade F

| Metric | Value |
|--------|-------|
| Files analyzed | 209 / 209 |
| Total lines | 35939 |
| Total issues | 794 |
| Critical | 22 |
| Warning | 322 |
| Info | 450 |

---

## Category Scores

| Category | Score | Family |
|----------|-------|--------|
| Security | 65 D | Uriel |
| Dependencies | 100 A | Uriel |
| Architecture | 0 F | Auriel |
| Quality | 0 F | Barachiel |
| Tests | 33 F | Raziel |
| Documentation | 50 F | Raziel |
| Toxic AI | 49 F | Cassiel |
| Naming | 55 F | Auriel |
| Style | 0 F | Auriel |
| Dead code | 57 F | Barachiel |
| Duplicates | 100 A | Cassiel |
| Commits | 100 A | Zadkiel |
| Placeholders | 96 A | Zadkiel |

---

## Analysis by Family

### Structure du code — 18 / 100
> Architecture, organisation des fichiers, respect des conventions et patterns de design

220 issue(s) found.

#### `long-functions` (23 occurrences)
_6 critical | 17 warning_

| Severity | Location | Detail |
|----------|----------|--------|
| **WARNING** | `mcp-guide.tsx:320` | Fonction trop longue — La fonction "InstallAssistant" contient 118 lignes |
| **WARNING** | `account.tsx:21` | Fonction trop longue — La fonction "AccountPage" contient 192 lignes |
| **WARNING** | `RepoCard.tsx:37` | Fonction trop longue — La fonction "RepoCard" contient 141 lignes |
| **WARNING** | `mod.rs:38` | Fonction trop longue — La fonction "from_env" contient 147 lignes |
| **WARNING** | `mod.rs:46` | Fonction trop longue — La fonction "build_app" contient 125 lignes |
| **WARNING** | `pipeline.rs:332` | Fonction trop longue — La fonction "explain_external_scoring" contient 126 lignes |
| **WARNING** | `server.rs:544` | Fonction trop longue — La fonction "log_usage" contient 102 lignes |
| **CRITICAL** | `repos.rs:100` | Fonction trop longue — La fonction "search_github_repos" contient 230 lignes |
| **CRITICAL** | `discover.tsx:57` | Fonction trop longue — La fonction "DiscoverPage" contient 676 lignes |
| **WARNING** | `RepoHeader.tsx:36` | Fonction trop longue — La fonction "RepoHeader" contient 107 lignes |
| **CRITICAL** | `watchlist.tsx:27` | Fonction trop longue — La fonction "WatchlistPage" contient 253 lignes |
| **WARNING** | `mod.rs:270` | Fonction trop longue — La fonction "finish_discord_oauth" contient 127 lignes |
| **CRITICAL** | `repo-detail.tsx:23` | Fonction trop longue — La fonction "RepoDetailPage" contient 267 lignes |
| **CRITICAL** | `UseCaseSearchPanel.tsx:12` | Fonction trop longue — La fonction "UseCaseSearchPanel" contient 220 lignes |
| **WARNING** | `AppHeader.tsx:13` | Fonction trop longue — La fonction "AppHeader" contient 119 lignes |
| **WARNING** | `mod.rs:129` | Fonction trop longue — La fonction "finish_github_oauth" contient 139 lignes |
| **WARNING** | `RepoMetricsPanel.tsx:126` | Fonction trop longue — La fonction "RepoMetricsPanel" contient 116 lignes |
| **WARNING** | `notifications.tsx:41` | Fonction trop longue — La fonction "NotificationsPage" contient 192 lignes |
| **CRITICAL** | `mcp-guide.tsx:44` | Fonction trop longue — La fonction "McpGuidePage" contient 274 lignes |
| **WARNING** | `recommendations.rs:92` | Fonction trop longue — La fonction "parse_intent" contient 144 lignes |
| **WARNING** | `AgentTokensPanel.tsx:45` | Fonction trop longue — La fonction "AgentTokensPanel" contient 137 lignes |
| **WARNING** | `pipeline.rs:171` | Fonction trop longue — La fonction "recompute_externals_with_config" contient 102 lignes |
| **WARNING** | `how-to-read.tsx:5` | Fonction trop longue — La fonction "HowToReadPage" contient 173 lignes |

**Suggestions:**
- Divisez cette fonction en fonctions plus petites avec des responsabilités uniques

#### `too-many-parameters` (8 occurrences)
_2 critical | 6 warning_

| Severity | Location | Detail |
|----------|----------|--------|
| **WARNING** | `compute.rs:35` | Trop de paramètres — La fonction "unweighted" a 6 paramètres |
| **WARNING** | `agent_token_events.rs:177` | Trop de paramètres — La fonction "record_log_usage" a 7 paramètres |
| **WARNING** | `agent_token_events.rs:23` | Trop de paramètres — La fonction "enforce_write_quota" a 7 paramètres |
| **WARNING** | `use_case_watches.rs:46` | Trop de paramètres — La fonction "create_watch" a 6 paramètres |
| **WARNING** | `reputation.rs:290` | Trop de paramètres — La fonction "sample_metrics" a 6 paramètres |
| **CRITICAL** | `agent_token_events.rs:67` | Trop de paramètres — La fonction "enforce_log_usage_guards" a 9 paramètres |
| **WARNING** | `agent_token_events.rs:242` | Trop de paramètres — La fonction "record_event" a 7 paramètres |
| **CRITICAL** | `agent_token_events.rs:269` | Trop de paramètres — La fonction "record_rejection" a 8 paramètres |

**Suggestions:**
- Regroupez les paramètres dans un objet de configuration

#### `file-too-long` (1 occurrences)
_1 critical_

| Severity | Location | Detail |
|----------|----------|--------|
| **CRITICAL** | `server.rs` | Fichier très long — Ce fichier contient 2044 lignes |

**Suggestions:**
- Découpez ce fichier en plusieurs modules plus petits et spécialisés

#### `orphan-modules` (44 occurrences)
_44 info_

| Severity | Location | Detail |
|----------|----------|--------|
| **INFO** | `repos_ingestion.rs` | Module orphelin — 'backend/src/handlers/repos_ingestion.rs' exporte des symboles mais n'est importe par aucun autre fichier |
| **INFO** | `notifications.rs` | Module orphelin — 'backend/src/handlers/notifications.rs' exporte des symboles mais n'est importe par aucun autre fichier |
| **INFO** | `signal_reviews.rs` | Module orphelin — 'backend/src/services/trust/signal_reviews.rs' exporte des symboles mais n'est importe par aucun autre fichier |
| **INFO** | `auth.rs` | Module orphelin — 'backend/src/handlers/auth.rs' exporte des symboles mais n'est importe par aucun autre fichier |
| **INFO** | `repo_signals.rs` | Module orphelin — 'backend/src/handlers/repo_signals.rs' exporte des symboles mais n'est importe par aucun autre fichier |
| **INFO** | `playwright.config.ts` | Module orphelin — 'frontend/playwright.config.ts' exporte des symboles mais n'est importe par aucun autre fichier |
| **INFO** | `semantic_search.rs` | Module orphelin — 'backend/src/services/semantic_search.rs' exporte des symboles mais n'est importe par aucun autre fichier |
| **INFO** | `compute.rs` | Module orphelin — 'backend/src/services/quality/compute.rs' exporte des symboles mais n'est importe par aucun autre fichier |
| **INFO** | `reputation.rs` | Module orphelin — 'backend/src/services/trust/reputation.rs' exporte des symboles mais n'est importe par aucun autre fichier |
| **INFO** | `me.rs` | Module orphelin — 'backend/src/handlers/me.rs' exporte des symboles mais n'est importe par aucun autre fichier |
| **INFO** | `mcp_rate_limit.rs` | Module orphelin — 'backend/src/app/mcp_rate_limit.rs' exporte des symboles mais n'est importe par aucun autre fichier |
| **INFO** | `health.rs` | Module orphelin — 'backend/src/handlers/health.rs' exporte des symboles mais n'est importe par aucun autre fichier |
| **INFO** | `admin.rs` | Module orphelin — 'backend/src/handlers/admin.rs' exporte des symboles mais n'est importe par aucun autre fichier |
| **INFO** | `scheduler.rs` | Module orphelin — 'backend/src/services/scheduler.rs' exporte des symboles mais n'est importe par aucun autre fichier |
| **INFO** | `repos.rs` | Module orphelin — 'backend/src/services/repos.rs' exporte des symboles mais n'est importe par aucun autre fichier |
| **INFO** | `use_case_watches.rs` | Module orphelin — 'backend/src/services/use_case_watches.rs' exporte des symboles mais n'est importe par aucun autre fichier |
| **INFO** | `auth.rs` | Module orphelin — 'backend/src/mcp/auth.rs' exporte des symboles mais n'est importe par aucun autre fichier |
| **INFO** | `github.rs` | Module orphelin — 'backend/src/services/ingestion/github.rs' exporte des symboles mais n'est importe par aucun autre fichier |
| **INFO** | `repo_viewer.rs` | Module orphelin — 'backend/src/handlers/repo_viewer.rs' exporte des symboles mais n'est importe par aucun autre fichier |
| **INFO** | `notifications.rs` | Module orphelin — 'backend/src/services/notifications.rs' exporte des symboles mais n'est importe par aucun autre fichier |
| **INFO** | `mcp_metrics.rs` | Module orphelin — 'backend/src/services/trust/mcp_metrics.rs' exporte des symboles mais n'est importe par aucun autre fichier |
| **INFO** | `repo_categories.rs` | Module orphelin — 'backend/src/services/repo_categories.rs' exporte des symboles mais n'est importe par aucun autre fichier |
| **INFO** | `agent_tokens.rs` | Module orphelin — 'backend/src/handlers/agent_tokens.rs' exporte des symboles mais n'est importe par aucun autre fichier |
| **INFO** | `reference.rs` | Module orphelin — 'backend/src/domain/reference.rs' exporte des symboles mais n'est importe par aucun autre fichier |
| **INFO** | `agent_tokens.rs` | Module orphelin — 'backend/src/services/agent_tokens.rs' exporte des symboles mais n'est importe par aucun autre fichier |
| **INFO** | `capture.rs` | Module orphelin — 'backend/src/services/quality/capture.rs' exporte des symboles mais n'est importe par aucun autre fichier |
| **INFO** | `weighting.rs` | Module orphelin — 'backend/src/services/quality/weighting.rs' exporte des symboles mais n'est importe par aucun autre fichier |
| **INFO** | `error.rs` | Module orphelin — 'backend/src/app/error.rs' exporte des symboles mais n'est importe par aucun autre fichier |
| **INFO** | `agent_token_events.rs` | Module orphelin — 'backend/src/services/trust/agent_token_events.rs' exporte des symboles mais n'est importe par aucun autre fichier |
| **INFO** | `radar.rs` | Module orphelin — 'backend/src/services/radar.rs' exporte des symboles mais n'est importe par aucun autre fichier |
| **INFO** | `flags.rs` | Module orphelin — 'backend/src/services/quality/flags.rs' exporte des symboles mais n'est importe par aucun autre fichier |
| **INFO** | `formula.rs` | Module orphelin — 'backend/src/services/quality/formula.rs' exporte des symboles mais n'est importe par aucun autre fichier |
| **INFO** | `recommendations.rs` | Module orphelin — 'backend/src/services/recommendations.rs' exporte des symboles mais n'est importe par aucun autre fichier |
| **INFO** | `watchlist.rs` | Module orphelin — 'backend/src/handlers/watchlist.rs' exporte des symboles mais n'est importe par aucun autre fichier |
| **INFO** | `account.rs` | Module orphelin — 'backend/src/handlers/account.rs' exporte des symboles mais n'est importe par aucun autre fichier |
| **INFO** | `watchlist.rs` | Module orphelin — 'backend/src/services/watchlist.rs' exporte des symboles mais n'est importe par aucun autre fichier |
| **INFO** | `quality.rs` | Module orphelin — 'backend/src/domain/quality.rs' exporte des symboles mais n'est importe par aucun autre fichier |
| **INFO** | `signal_events.rs` | Module orphelin — 'backend/src/services/trust/signal_events.rs' exporte des symboles mais n'est importe par aucun autre fichier |
| **INFO** | `repo_owners.rs` | Module orphelin — 'backend/src/services/trust/repo_owners.rs' exporte des symboles mais n'est importe par aucun autre fichier |
| **INFO** | `search.rs` | Module orphelin — 'backend/src/handlers/search.rs' exporte des symboles mais n'est importe par aucun autre fichier |
| **INFO** | `pipeline.rs` | Module orphelin — 'backend/src/services/quality/pipeline.rs' exporte des symboles mais n'est importe par aucun autre fichier |
| **INFO** | `use_cases.rs` | Module orphelin — 'backend/src/handlers/use_cases.rs' exporte des symboles mais n'est importe par aucun autre fichier |
| **INFO** | `repos_query.rs` | Module orphelin — 'backend/src/handlers/repos_query.rs' exporte des symboles mais n'est importe par aucun autre fichier |
| **INFO** | `vite.config.ts` | Module orphelin — 'frontend/vite.config.ts' exporte des symboles mais n'est importe par aucun autre fichier |

**Suggestions:**
- Ce module est peut-etre du code mort. Verifiez s'il est encore utilise

#### `no-nested-ternary` (40 occurrences)
_40 warning_

| Severity | Location | Detail |
|----------|----------|--------|
| **WARNING** | `watchlist.tsx:139` | Ternaire imbriquee — Les ternaires imbriquees sont difficiles a lire |
| **WARNING** | `notifications.tsx:69` | Ternaire imbriquee — Les ternaires imbriquees sont difficiles a lire |
| **WARNING** | `repo-detail.tsx:267` | Ternaire imbriquee — Les ternaires imbriquees sont difficiles a lire |
| **WARNING** | `RepoCard.tsx:88` | Ternaire imbriquee — Les ternaires imbriquees sont difficiles a lire |
| **WARNING** | `RepoHeader.tsx:83` | Ternaire imbriquee — Les ternaires imbriquees sont difficiles a lire |
| **WARNING** | `mcp-guide.tsx:55` | Ternaire imbriquee — Les ternaires imbriquees sont difficiles a lire |
| **WARNING** | `account.tsx:125` | Ternaire imbriquee — Les ternaires imbriquees sont difficiles a lire |
| **WARNING** | `discover.tsx:304` | Ternaire imbriquee — Les ternaires imbriquees sont difficiles a lire |
| **WARNING** | `discover.tsx:150` | Ternaire imbriquee — Les ternaires imbriquees sont difficiles a lire |
| **WARNING** | `status.tsx:38` | Ternaire imbriquee — Les ternaires imbriquees sont difficiles a lire |
| **WARNING** | `api-client.ts:33` | Ternaire imbriquee — Les ternaires imbriquees sont difficiles a lire |
| **WARNING** | `status.tsx:46` | Ternaire imbriquee — Les ternaires imbriquees sont difficiles a lire |
| **WARNING** | `index.tsx:453` | Ternaire imbriquee — Les ternaires imbriquees sont difficiles a lire |
| **WARNING** | `discover.tsx:161` | Ternaire imbriquee — Les ternaires imbriquees sont difficiles a lire |
| **WARNING** | `notifications.tsx:136` | Ternaire imbriquee — Les ternaires imbriquees sont difficiles a lire |
| **WARNING** | `AppHeader.tsx:74` | Ternaire imbriquee — Les ternaires imbriquees sont difficiles a lire |
| **WARNING** | `repo-detail.tsx:139` | Ternaire imbriquee — Les ternaires imbriquees sont difficiles a lire |
| **WARNING** | `RepoCard.tsx:136` | Ternaire imbriquee — Les ternaires imbriquees sont difficiles a lire |
| **WARNING** | `repo-detail.tsx:50` | Ternaire imbriquee — Les ternaires imbriquees sont difficiles a lire |
| **WARNING** | `RepoSignalsList.tsx:40` | Ternaire imbriquee — Les ternaires imbriquees sont difficiles a lire |
| **WARNING** | `mvp.spec.ts:5` | Ternaire imbriquee — Les ternaires imbriquees sont difficiles a lire |
| **WARNING** | `AdminModerationPanel.tsx:55` | Ternaire imbriquee — Les ternaires imbriquees sont difficiles a lire |
| **WARNING** | `playwright.config.ts:3` | Ternaire imbriquee — Les ternaires imbriquees sont difficiles a lire |
| **WARNING** | `AppHeader.tsx:26` | Ternaire imbriquee — Les ternaires imbriquees sont difficiles a lire |
| **WARNING** | `status.tsx:15` | Ternaire imbriquee — Les ternaires imbriquees sont difficiles a lire |
| **WARNING** | `ScoreBar.tsx:20` | Ternaire imbriquee — Les ternaires imbriquees sont difficiles a lire |
| **WARNING** | `api-client.ts:1` | Ternaire imbriquee — Les ternaires imbriquees sont difficiles a lire |
| **WARNING** | `AgentTokensPanel.tsx:106` | Ternaire imbriquee — Les ternaires imbriquees sont difficiles a lire |
| **WARNING** | `AgentTokensPanel.tsx:151` | Ternaire imbriquee — Les ternaires imbriquees sont difficiles a lire |
| **WARNING** | `mvp.spec.ts:125` | Ternaire imbriquee — Les ternaires imbriquees sont difficiles a lire |
| **WARNING** | `AppHeader.tsx:42` | Ternaire imbriquee — Les ternaires imbriquees sont difficiles a lire |
| **WARNING** | `index.tsx:468` | Ternaire imbriquee — Les ternaires imbriquees sont difficiles a lire |
| **WARNING** | `RepoCard.tsx:115` | Ternaire imbriquee — Les ternaires imbriquees sont difficiles a lire |
| **WARNING** | `format.ts:63` | Ternaire imbriquee — Les ternaires imbriquees sont difficiles a lire |
| **WARNING** | `UseCaseSearchPanel.tsx:131` | Ternaire imbriquee — Les ternaires imbriquees sont difficiles a lire |
| **WARNING** | `watchlist.tsx:55` | Ternaire imbriquee — Les ternaires imbriquees sont difficiles a lire |
| **WARNING** | `notifications.tsx:21` | Ternaire imbriquee — Les ternaires imbriquees sont difficiles a lire |
| **WARNING** | `return-to.ts:15` | Ternaire imbriquee — Les ternaires imbriquees sont difficiles a lire |
| **WARNING** | `UseCaseSearchPanel.tsx:188` | Ternaire imbriquee — Les ternaires imbriquees sont difficiles a lire |
| **WARNING** | `UseCaseSearchPanel.tsx:31` | Ternaire imbriquee — Les ternaires imbriquees sont difficiles a lire |

**Suggestions:**
- Utilisez des if/else ou extrayez la logique dans une fonction

#### `missing-semicolons` (27 occurrences)
_27 info_

| Severity | Location | Detail |
|----------|----------|--------|
| **INFO** | `login.tsx` | Point-virgules inconsistants — Melange de lignes avec (6) et sans (9) point-virgule |
| **INFO** | `Button.tsx` | Point-virgules inconsistants — Melange de lignes avec (14) et sans (8) point-virgule |
| **INFO** | `repo-detail.tsx` | Point-virgules inconsistants — Melange de lignes avec (53) et sans (34) point-virgule |
| **INFO** | `RepoMetricsPanel.tsx` | Point-virgules inconsistants — Melange de lignes avec (53) et sans (20) point-virgule |
| **INFO** | `UseCaseSearchPanel.tsx` | Point-virgules inconsistants — Melange de lignes avec (12) et sans (29) point-virgule |
| **INFO** | `AdminModerationPanel.tsx` | Point-virgules inconsistants — Melange de lignes avec (16) et sans (12) point-virgule |
| **INFO** | `format.ts` | Point-virgules inconsistants — Melange de lignes avec (33) et sans (7) point-virgule |
| **INFO** | `router.tsx` | Point-virgules inconsistants — Melange de lignes avec (18) et sans (15) point-virgule |
| **INFO** | `notifications.tsx` | Point-virgules inconsistants — Melange de lignes avec (32) et sans (32) point-virgule |
| **INFO** | `RepoCard.tsx` | Point-virgules inconsistants — Melange de lignes avec (19) et sans (19) point-virgule |
| **INFO** | `status.tsx` | Point-virgules inconsistants — Melange de lignes avec (15) et sans (23) point-virgule |
| **INFO** | `discover.tsx` | Point-virgules inconsistants — Melange de lignes avec (98) et sans (117) point-virgule |
| **INFO** | `mvp.spec.ts` | Point-virgules inconsistants — Melange de lignes avec (99) et sans (25) point-virgule |
| **INFO** | `account.tsx` | Point-virgules inconsistants — Melange de lignes avec (30) et sans (14) point-virgule |
| **INFO** | `AdminMcpObservabilityPanel.tsx` | Point-virgules inconsistants — Melange de lignes avec (50) et sans (26) point-virgule |
| **INFO** | `SiteFooter.tsx` | Point-virgules inconsistants — Melange de lignes avec (10) et sans (10) point-virgule |
| **INFO** | `LocaleSwitch.tsx` | Point-virgules inconsistants — Melange de lignes avec (6) et sans (11) point-virgule |
| **INFO** | `Wordmark.tsx` | Point-virgules inconsistants — Melange de lignes avec (6) et sans (5) point-virgule |
| **INFO** | `OwnerDisputePanel.tsx` | Point-virgules inconsistants — Melange de lignes avec (18) et sans (6) point-virgule |
| **INFO** | `AppHeader.tsx` | Point-virgules inconsistants — Melange de lignes avec (6) et sans (16) point-virgule |
| **INFO** | `index.tsx` | Point-virgules inconsistants — Melange de lignes avec (54) et sans (84) point-virgule |
| **INFO** | `api-client.ts` | Point-virgules inconsistants — Melange de lignes avec (22) et sans (10) point-virgule |
| **INFO** | `RepoHeader.tsx` | Point-virgules inconsistants — Melange de lignes avec (21) et sans (10) point-virgule |
| **INFO** | `ReportSignalForm.tsx` | Point-virgules inconsistants — Melange de lignes avec (20) et sans (7) point-virgule |
| **INFO** | `watchlist.tsx` | Point-virgules inconsistants — Melange de lignes avec (32) et sans (38) point-virgule |
| **INFO** | `mcp-guide.tsx` | Point-virgules inconsistants — Melange de lignes avec (50) et sans (49) point-virgule |
| **INFO** | `AgentTokensPanel.tsx` | Point-virgules inconsistants — Melange de lignes avec (33) et sans (16) point-virgule |

**Suggestions:**
- Adoptez un style coherent pour les point-virgules (avec ou sans)

#### `magic-numbers` (27 occurrences)
_27 info_

| Severity | Location | Detail |
|----------|----------|--------|
| **INFO** | `mvp.spec.ts:18` | Nombre magique — Le nombre 82 devrait etre une constante nommee |
| **INFO** | `account.tsx:93` | Nombre magique — Le nombre 1500 devrait etre une constante nommee |
| **INFO** | `radar.ts:31` | Nombre magique — Le nombre 85 devrait etre une constante nommee |
| **INFO** | `format.ts:12` | Nombre magique — Le nombre 45 devrait etre une constante nommee |
| **INFO** | `discover.tsx:35` | Nombre magique — Le nombre 35 devrait etre une constante nommee |
| **INFO** | `discover.tsx:29` | Nombre magique — Le nombre 45 devrait etre une constante nommee |
| **INFO** | `format.ts:27` | Nombre magique — Le nombre 10000 devrait etre une constante nommee |
| **INFO** | `OwnerDisputePanel.tsx:75` | Nombre magique — Le nombre 10 devrait etre une constante nommee |
| **INFO** | `format.ts:47` | Nombre magique — Le nombre 30 devrait etre une constante nommee |
| **INFO** | `fr.ts:590` | Nombre magique — Le nombre 90 devrait etre une constante nommee |
| **INFO** | `playwright.config.ts:26` | Nombre magique — Le nombre 127 devrait etre une constante nommee |
| **INFO** | `mvp.spec.ts:20` | Nombre magique — Le nombre 84 devrait etre une constante nommee |
| **INFO** | `discover.tsx:31` | Nombre magique — Le nombre 75 devrait etre une constante nommee |
| **INFO** | `mcp-guide.tsx:80` | Nombre magique — Le nombre 2025 devrait etre une constante nommee |
| **INFO** | `radar.ts:35` | Nombre magique — Le nombre 55 devrait etre une constante nommee |
| **INFO** | `AppHeader.tsx:119` | Nombre magique — Le nombre 99 devrait etre une constante nommee |
| **INFO** | `index.tsx:109` | Nombre magique — Le nombre 65 devrait etre une constante nommee |
| **INFO** | `mvp.spec.ts:16` | Nombre magique — Le nombre 91 devrait etre une constante nommee |
| **INFO** | `LocaleSwitch.tsx:31` | Nombre magique — Le nombre 10 devrait etre une constante nommee |
| **INFO** | `vite.config.ts:8` | Nombre magique — Le nombre 5173 devrait etre une constante nommee |
| **INFO** | `format.ts:45` | Nombre magique — Le nombre 48 devrait etre une constante nommee |
| **INFO** | `AdminMcpObservabilityPanel.tsx:251` | Nombre magique — Le nombre 10 devrait etre une constante nommee |
| **INFO** | `AppHeader.tsx:54` | Nombre magique — Le nombre 99 devrait etre une constante nommee |
| **INFO** | `mvp.spec.ts:19` | Nombre magique — Le nombre 11 devrait etre une constante nommee |
| **INFO** | `mcp-guide.tsx:129` | Nombre magique — Le nombre 1500 devrait etre une constante nommee |
| **INFO** | `mvp.spec.ts:17` | Nombre magique — Le nombre 74 devrait etre une constante nommee |
| **INFO** | `discover.tsx:270` | Nombre magique — Le nombre 44 devrait etre une constante nommee |

**Suggestions:**
- Extrayez 10 dans une constante avec un nom descriptif
- Extrayez 10000 dans une constante avec un nom descriptif
- Extrayez 11 dans une constante avec un nom descriptif

#### `long-lines` (25 occurrences)
_25 info_

| Severity | Location | Detail |
|----------|----------|--------|
| **INFO** | `strategy-pivot-2026-04-21.md:5` | Lignes trop longues — 23 lignes depassent 120 caracteres |
| **INFO** | `anti-slop-vitality-v2.md:3` | Lignes trop longues — 30 lignes depassent 120 caracteres |
| **INFO** | `source-of-truth-oss-radar-plan.md:3` | Lignes trop longues — 12 lignes depassent 120 caracteres |
| **INFO** | `user-journey-audit-2026-04-23.md:5` | Lignes trop longues — 11 lignes depassent 120 caracteres |
| **INFO** | `strategy-quality-scored-registry.md:5` | Lignes trop longues — 29 lignes depassent 120 caracteres |
| **INFO** | `remaining-work-2026-05-03.md:4` | Lignes trop longues — 28 lignes depassent 120 caracteres |
| **INFO** | `TODO.md:5` | Lignes trop longues — 59 lignes depassent 120 caracteres |
| **INFO** | `refactor-plan-2026-04-23.md:4` | Lignes trop longues — 12 lignes depassent 120 caracteres |
| **INFO** | `user-journey.md:4` | Lignes trop longues — 20 lignes depassent 120 caracteres |
| **INFO** | `README.md:6` | Lignes trop longues — 14 lignes depassent 120 caracteres |
| **INFO** | `user-journey-audit-phase2-2026-04-24.md:5` | Lignes trop longues — 62 lignes depassent 120 caracteres |
| **INFO** | `competitive-analysis.md:7` | Lignes trop longues — 12 lignes depassent 120 caracteres |
| **INFO** | `en.ts:70` | Lignes trop longues — 49 lignes depassent 120 caracteres |
| **INFO** | `GEMINI.md:9` | Lignes trop longues — 17 lignes depassent 120 caracteres |
| **INFO** | `use-case-recommendation-watch-plan.md:4` | Lignes trop longues — 11 lignes depassent 120 caracteres |
| **INFO** | `fr.ts:50` | Lignes trop longues — 67 lignes depassent 120 caracteres |
| **INFO** | `product-vision-and-safety.md:8` | Lignes trop longues — 13 lignes depassent 120 caracteres |
| **INFO** | `discover.tsx:363` | Lignes trop longues — 15 lignes depassent 120 caracteres |
| **INFO** | `CLAUDE.md:7` | Lignes trop longues — 39 lignes depassent 120 caracteres |
| **INFO** | `architecture-backend-current.md:19` | Lignes trop longues — 17 lignes depassent 120 caracteres |
| **INFO** | `mvp-action-plan.md:6` | Lignes trop longues — 15 lignes depassent 120 caracteres |
| **INFO** | `security-audit-2026-04-21.md:4` | Lignes trop longues — 17 lignes depassent 120 caracteres |
| **INFO** | `AGENTS.md:3` | Lignes trop longues — 29 lignes depassent 120 caracteres |
| **INFO** | `user-journey-prepivot.md:5` | Lignes trop longues — 30 lignes depassent 120 caracteres |
| **INFO** | `functional-checks.md:4` | Lignes trop longues — 42 lignes depassent 120 caracteres |

**Suggestions:**
- Limitez la longueur des lignes a 80-120 caracteres pour ameliorer la lisibilite

#### `component-naming` (11 occurrences)
_11 warning_

| Severity | Location | Detail |
|----------|----------|--------|
| **WARNING** | `login.tsx:23` | Composant React non en PascalCase — Le composant "status" devrait commencer par une majuscule |
| **WARNING** | `watchlist.tsx:32` | Composant React non en PascalCase — Le composant "query" devrait commencer par une majuscule |
| **WARNING** | `LocaleSwitch.tsx:10` | Composant React non en PascalCase — Le composant "activeIndex" devrait commencer par une majuscule |
| **WARNING** | `notifications.tsx:46` | Composant React non en PascalCase — Le composant "query" devrait commencer par une majuscule |
| **WARNING** | `discover.tsx:123` | Composant React non en PascalCase — Le composant "results" devrait commencer par une majuscule |
| **WARNING** | `index.tsx:19` | Composant React non en PascalCase — Le composant "isAuthed" devrait commencer par une majuscule |
| **WARNING** | `mcp-guide.tsx:47` | Composant React non en PascalCase — Le composant "isAuthed" devrait commencer par une majuscule |
| **WARNING** | `router.tsx:25` | Composant React non en PascalCase — Le composant "rootRoute" devrait commencer par une majuscule |
| **WARNING** | `account.tsx:24` | Composant React non en PascalCase — Le composant "user" devrait commencer par une majuscule |
| **WARNING** | `status.tsx:9` | Composant React non en PascalCase — Le composant "status" devrait commencer par une majuscule |
| **WARNING** | `AppHeader.tsx:18` | Composant React non en PascalCase — Le composant "unreadQuery" devrait commencer par une majuscule |

**Suggestions:**
- Renommez "activeIndex" en "ActiveIndex"
- Renommez "isAuthed" en "IsAuthed"
- Renommez "query" en "Query"

#### `config-in-code` (7 occurrences)
_7 info_

| Severity | Location | Detail |
|----------|----------|--------|
| **INFO** | `mod.rs:184` | Configuration en dur dans le code — URL localhost détectée : `http://127.0.0.1` |
| **INFO** | `mod.rs:179` | Configuration en dur dans le code — URL localhost détectée : `http://localhost` |
| **INFO** | `mod.rs:181` | Configuration en dur dans le code — URL localhost détectée : `http://127.0.0.1` |
| **INFO** | `api-client.ts:1` | Configuration en dur dans le code — URL localhost détectée : `http://localhost` |
| **INFO** | `server.rs:2016` | Configuration en dur dans le code — Chaîne de connexion détectée : `"postgres://example"` |
| **INFO** | `mod.rs:176` | Configuration en dur dans le code — URL localhost détectée : `http://localhost` |
| **INFO** | `mod.rs:188` | Configuration en dur dans le code — URL localhost détectée : `http://localhost` |

**Suggestions:**
- Déplacez cette URL dans une variable d'environnement ou un fichier de configuration
- Déplacez cette chaîne de connexion dans une variable d'environnement

#### `single-letter-vars` (2 occurrences)
_2 info_

| Severity | Location | Detail |
|----------|----------|--------|
| **INFO** | `index.tsx:18` | Variables à une lettre — 10 variables avec un nom d'une seule lettre détectées |
| **INFO** | `repo-detail.tsx:24` | Variables à une lettre — 4 variables avec un nom d'une seule lettre détectées |

**Suggestions:**
- Utilisez des noms descriptifs pour améliorer la lisibilité du code

#### `boolean-naming` (2 occurrences)
_2 info_

| Severity | Location | Detail |
|----------|----------|--------|
| **INFO** | `mvp.spec.ts:94` | Nommage de booléen améliorable — "watching" pourrait avoir un préfixe is/has/can pour clarifier que c'est un booléen |
| **INFO** | `mvp.spec.ts:95` | Nommage de booléen améliorable — "notificationRead" pourrait avoir un préfixe is/has/can pour clarifier que c'est un booléen |

**Suggestions:**
- Considérez renommer "notificationRead" en "isNotificationRead"
- Considérez renommer "watching" en "isWatching"

#### `trailing-whitespace` (1 occurrences)
_1 info_

| Severity | Location | Detail |
|----------|----------|--------|
| **INFO** | `user-journey-audit-phase2-2026-04-24.md:37` | Espaces en fin de ligne — 8 lignes ont des espaces en fin de ligne |

**Suggestions:**
- Configurez votre editeur pour supprimer automatiquement les espaces en fin de ligne

#### `excessive-dependencies` (1 occurrences)
_1 info_

| Severity | Location | Detail |
|----------|----------|--------|
| **INFO** | `router.tsx:1` | Too many imports — 16 imports — this file may have too many responsibilities |

**Suggestions:**
- Split this file into smaller modules with focused responsibilities

#### `verb-for-functions` (1 occurrences)
_1 info_

| Severity | Location | Detail |
|----------|----------|--------|
| **INFO** | `mvp.spec.ts:66` | Fonctions sans verbe — 5 fonctions ne commencent pas par un verbe (ex: "json") |

**Suggestions:**
- Les fonctions devraient commencer par un verbe (ex: getUserById, validateEmail, formatDate)


### Failles de sécurité — 82 / 100
> Injection SQL, XSS, secrets exposés, dépendances vulnérables et configurations à risque

7 issue(s) found.

#### `insecure-http` (3 occurrences)
_3 warning_

| Severity | Location | Detail |
|----------|----------|--------|
| **WARNING** | `mod.rs:63` | URL HTTP non securisee — URL en HTTP detectee: http://{ - les communications doivent utiliser HTTPS |
| **WARNING** | `github.rs:518` | URL HTTP non securisee — URL en HTTP detectee: http://github.com/ - les communications doivent utiliser HTTPS |
| **WARNING** | `main.rs:42` | URL HTTP non securisee — URL en HTTP detectee: http://{ - les communications doivent utiliser HTTPS |

**Suggestions:**
- Utilisez https:// au lieu de http:// pour securiser les communications

#### `hardcoded-ip` (2 occurrences)
_2 warning_

| Severity | Location | Detail |
|----------|----------|--------|
| **WARNING** | `mcp_rate_limit.rs:92` | Adresse IP hardcodee — IP 203.0.113.10 hardcodee - utilisez une variable d'environnement |
| **WARNING** | `mcp_rate_limit.rs:113` | Adresse IP hardcodee — IP 203.0.113.10 hardcodee - utilisez une variable d'environnement |

**Suggestions:**
- Utilisez une variable d'environnement: process.env.SERVER_IP ou similaire

#### `insecure-cookie` (1 occurrences)
_1 warning_

| Severity | Location | Detail |
|----------|----------|--------|
| **WARNING** | `mod.rs:638` | Cookie sans options de securite — Cookie defini sans httpOnly, secure, sameSite - risque de vol de session |

**Suggestions:**
- Ajoutez les options: { httpOnly: true, secure: true, sameSite: 'strict' }

#### `missing-gitignore` (1 occurrences)
_1 warning_

| Severity | Location | Detail |
|----------|----------|--------|
| **WARNING** | - | Fichier .gitignore manquant — Aucun fichier .gitignore n'a ete trouve dans le repository |

**Suggestions:**
- Creez un fichier .gitignore pour exclure les fichiers sensibles et les dependances


### Patterns IA toxiques — 74 / 100
> Code généré mais non optimisé, fonctions dupliquées, logique incohérente typique du vibecoding

15 issue(s) found.

#### `copypaste-code` (12 occurrences)
_12 warning_

| Severity | Location | Detail |
|----------|----------|--------|
| **WARNING** | `server.rs:143` | Copy-paste code detected — 8 duplicate code blocks found — likely copy-pasted [L79 ~ L143, L79 ~ L242, L80 ~ L243, L81 ~ L244, L82 ~ L245 (+3 more) |
| **WARNING** | `auth.rs:69` | Copy-paste code detected — 4 duplicate code blocks found — likely copy-pasted [L42 ~ L69, L43 ~ L70, L45 ~ L72, L46 ~ L73] |
| **WARNING** | `repos.rs:338` | Copy-paste code detected — 54 duplicate code blocks found — likely copy-pasted [L126 ~ L338, L127 ~ L339, L128 ~ L340, L129 ~ L341, L130 ~ L342 (+4 |
| **WARNING** | `reputation.rs:166` | Copy-paste code detected — 8 duplicate code blocks found — likely copy-pasted [L140 ~ L166, L141 ~ L167, L142 ~ L168, L143 ~ L169, L144 ~ L170 (+3  |
| **WARNING** | `agent_token_events.rs:178` | Copy-paste code detected — 6 duplicate code blocks found — likely copy-pasted [L68 ~ L178, L69 ~ L179, L24 ~ L270, L68 ~ L308, L69 ~ L309 (+1 more) |
| **WARNING** | `weighting.rs:158` | Copy-paste code detected — 3 duplicate code blocks found — likely copy-pasted [L108 ~ L158, L109 ~ L159, L110 ~ L160] |
| **WARNING** | `use_case_watches.rs:160` | Copy-paste code detected — 6 duplicate code blocks found — likely copy-pasted [L123 ~ L160, L125 ~ L162, L126 ~ L163, L128 ~ L165, L129 ~ L166 (+1  |
| **WARNING** | `signal_reviews.rs:77` | Copy-paste code detected — 12 duplicate code blocks found — likely copy-pasted [L28 ~ L77, L29 ~ L78, L30 ~ L79, L31 ~ L80, L32 ~ L81 (+7 more)] |
| **WARNING** | `discover.tsx:662` | Copy-paste code detected — 3 duplicate code blocks found — likely copy-pasted [L320 ~ L662, L321 ~ L663, L322 ~ L664] |
| **WARNING** | `mod.rs:271` | Copy-paste code detected — 7 duplicate code blocks found — likely copy-pasted [L130 ~ L271, L131 ~ L272, L219 ~ L348, L221 ~ L350, L222 ~ L351 (+2  |
| **WARNING** | `types.ts:171` | Copy-paste code detected — 6 duplicate code blocks found — likely copy-pasted [L53 ~ L171, L54 ~ L172, L55 ~ L173, L56 ~ L174, L57 ~ L175 (+1 more) |
| **WARNING** | `AppHeader.tsx:101` | Copy-paste code detected — 9 duplicate code blocks found — likely copy-pasted [L36 ~ L101, L37 ~ L102, L39 ~ L104, L40 ~ L105, L42 ~ L107 (+4 more) |

**Suggestions:**
- Extract duplicated logic into reusable functions

#### `unvalidated-ai-input` (2 occurrences)
_2 warning_

| Severity | Location | Detail |
|----------|----------|--------|
| **WARNING** | `return-to.ts:14` | Unvalidated user input in AI prompt — User input passed to AI without validation (prompt injection risk) |
| **WARNING** | `UseCaseSearchPanel.tsx:23` | Unvalidated user input in AI prompt — User input passed to AI without validation (prompt injection risk) |

**Suggestions:**
- Validate and sanitize user input before passing to AI

#### `unused-function-params` (1 occurrences)
_1 info_

| Severity | Location | Detail |
|----------|----------|--------|
| **INFO** | `RepoCard.tsx:37` | Unused function parameters — 2 unused parameters (likely AI-generated signature) |

**Suggestions:**
- Remove unused parameters or prefix with _ to indicate intentional


### Performance — 28 / 100
> Memory leaks, boucles infinies potentielles, requêtes N+1 et goulots d'étranglement

449 issue(s) found.

#### `function-complexity` (43 occurrences)
_10 critical | 33 warning_

| Severity | Location | Detail |
|----------|----------|--------|
| **WARNING** | `usestakly-mcp.mjs:98` | High cyclomatic complexity — Function 'doctor' has cyclomatic complexity of 19 (threshold: 10) |
| **WARNING** | `repo_signals.rs:98` | High cyclomatic complexity — Function 'dispute_repo_signal' has cyclomatic complexity of 11 (threshold: 10) |
| **WARNING** | `status.tsx:82` | High cyclomatic complexity — Function 'StatusCheck' has cyclomatic complexity of 16 (threshold: 10) |
| **CRITICAL** | `radar.rs:26` | High cyclomatic complexity — Function 'compute_radar_snapshot' has cyclomatic complexity of 25 (threshold: 10) |
| **CRITICAL** | `AdminModerationPanel.tsx:22` | High cyclomatic complexity — Function 'AdminModerationPanel' has cyclomatic complexity of 25 (threshold: 10) |
| **WARNING** | `repo_categories.rs:274` | High cyclomatic complexity — Function 'classify_with_rule' has cyclomatic complexity of 13 (threshold: 10) |
| **WARNING** | `mod.rs:270` | High cyclomatic complexity — Function 'finish_discord_oauth' has cyclomatic complexity of 13 (threshold: 10) |
| **WARNING** | `AgentTokensPanel.tsx:45` | High cyclomatic complexity — Function 'AgentTokensPanel' has cyclomatic complexity of 20 (threshold: 10) |
| **WARNING** | `admin.rs:191` | High cyclomatic complexity — Function 'review_repo_signal' has cyclomatic complexity of 11 (threshold: 10) |
| **WARNING** | `admin.rs:124` | High cyclomatic complexity — Function 'ingest_github_repo' has cyclomatic complexity of 11 (threshold: 10) |
| **WARNING** | `RepoCard.tsx:37` | High cyclomatic complexity — Function 'RepoCard' has cyclomatic complexity of 18 (threshold: 10) |
| **WARNING** | `repo_signals.rs:25` | High cyclomatic complexity — Function 'create_repo_signal' has cyclomatic complexity of 12 (threshold: 10) |
| **WARNING** | `compute.rs:167` | High cyclomatic complexity — Function 'vitality_score' has cyclomatic complexity of 11 (threshold: 10) |
| **WARNING** | `RepoSignalsList.tsx:38` | High cyclomatic complexity — Function 'anonymous' has cyclomatic complexity of 12 (threshold: 10) |
| **WARNING** | `recommendations.rs:249` | High cyclomatic complexity — Function 'build_recommendation' has cyclomatic complexity of 16 (threshold: 10) |
| **CRITICAL** | `discover.tsx:57` | High cyclomatic complexity — Function 'DiscoverPage' has cyclomatic complexity of 112 (threshold: 10) |
| **WARNING** | `AppHeader.tsx:13` | High cyclomatic complexity — Function 'AppHeader' has cyclomatic complexity of 15 (threshold: 10) |
| **WARNING** | `server.rs:1155` | High cyclomatic complexity — Function 'recommendation_reasons' has cyclomatic complexity of 13 (threshold: 10) |
| **WARNING** | `admin.rs:249` | High cyclomatic complexity — Function 'list_pending_repo_signals' has cyclomatic complexity of 16 (threshold: 10) |
| **WARNING** | `api-client.ts:12` | High cyclomatic complexity — Function 'request' has cyclomatic complexity of 13 (threshold: 10) |
| **WARNING** | `mcp-guide.tsx:44` | High cyclomatic complexity — Function 'McpGuidePage' has cyclomatic complexity of 15 (threshold: 10) |
| **WARNING** | `seed_github.rs:28` | High cyclomatic complexity — Function 'main' has cyclomatic complexity of 13 (threshold: 10) |
| **CRITICAL** | `watchlist.tsx:153` | High cyclomatic complexity — Function 'anonymous' has cyclomatic complexity of 24 (threshold: 10) |
| **WARNING** | `quality.rs:40` | High cyclomatic complexity — Function 'as_str' has cyclomatic complexity of 11 (threshold: 10) |
| **WARNING** | `notifications.rs:46` | High cyclomatic complexity — Function 'detect_and_emit' has cyclomatic complexity of 12 (threshold: 10) |
| **CRITICAL** | `mod.rs:38` | High cyclomatic complexity — Function 'from_env' has cyclomatic complexity of 24 (threshold: 10) |
| **WARNING** | `recommendations.rs:92` | High cyclomatic complexity — Function 'parse_intent' has cyclomatic complexity of 14 (threshold: 10) |
| **WARNING** | `main.rs:9` | High cyclomatic complexity — Function 'main' has cyclomatic complexity of 14 (threshold: 10) |
| **WARNING** | `pipeline.rs:171` | High cyclomatic complexity — Function 'recompute_externals_with_config' has cyclomatic complexity of 13 (threshold: 10) |
| **CRITICAL** | `RepoHeader.tsx:36` | High cyclomatic complexity — Function 'RepoHeader' has cyclomatic complexity of 21 (threshold: 10) |
| **CRITICAL** | `UseCaseSearchPanel.tsx:12` | High cyclomatic complexity — Function 'UseCaseSearchPanel' has cyclomatic complexity of 29 (threshold: 10) |
| **WARNING** | `server.rs:544` | High cyclomatic complexity — Function 'log_usage' has cyclomatic complexity of 13 (threshold: 10) |
| **CRITICAL** | `repo-detail.tsx:23` | High cyclomatic complexity — Function 'RepoDetailPage' has cyclomatic complexity of 23 (threshold: 10) |
| **WARNING** | `OwnerDisputePanel.tsx:22` | High cyclomatic complexity — Function 'OwnerDisputePanel' has cyclomatic complexity of 11 (threshold: 10) |
| **WARNING** | `mod.rs:129` | High cyclomatic complexity — Function 'finish_github_oauth' has cyclomatic complexity of 17 (threshold: 10) |
| **CRITICAL** | `notifications.tsx:41` | High cyclomatic complexity — Function 'NotificationsPage' has cyclomatic complexity of 31 (threshold: 10) |
| **WARNING** | `RepoSignalsList.tsx:16` | High cyclomatic complexity — Function 'RepoSignalsList' has cyclomatic complexity of 16 (threshold: 10) |
| **CRITICAL** | `watchlist.tsx:27` | High cyclomatic complexity — Function 'WatchlistPage' has cyclomatic complexity of 37 (threshold: 10) |
| **WARNING** | `scheduler.rs:46` | High cyclomatic complexity — Function 'refresh_github_repos' has cyclomatic complexity of 11 (threshold: 10) |
| **WARNING** | `notifications.tsx:165` | High cyclomatic complexity — Function 'anonymous' has cyclomatic complexity of 17 (threshold: 10) |
| **WARNING** | `AdminModerationPanel.tsx:57` | High cyclomatic complexity — Function 'anonymous' has cyclomatic complexity of 19 (threshold: 10) |
| **WARNING** | `status.tsx:7` | High cyclomatic complexity — Function 'StatusPage' has cyclomatic complexity of 15 (threshold: 10) |
| **WARNING** | `mcp-guide.tsx:320` | High cyclomatic complexity — Function 'InstallAssistant' has cyclomatic complexity of 19 (threshold: 10) |

**Suggestions:**
- Break this function into smaller functions to reduce complexity

#### `n-plus-one-query` (3 occurrences)
_3 critical_

| Severity | Location | Detail |
|----------|----------|--------|
| **CRITICAL** | `use_case_watches.rs:110` | Potential N+1 query — Database query inside a loop — consider batching or using a JOIN |
| **CRITICAL** | `notifications.rs:124` | Potential N+1 query — Database query inside a loop — consider batching or using a JOIN |
| **CRITICAL** | `repo_categories.rs:156` | Potential N+1 query — Database query inside a loop — consider batching or using a JOIN |

**Suggestions:**
- Use batch queries, eager loading, or JOINs instead of querying in a loop

#### `missing-error-context-rust` (142 occurrences)
_142 info_

| Severity | Location | Detail |
|----------|----------|--------|
| **INFO** | `capture.rs:27` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `mod.rs:13` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `seed_github.rs:35` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `signal_reviews.rs:54` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `watchlist.rs:30` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `pipeline.rs:337` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `admin.rs:34` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `radar.rs:178` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `mod.rs:14` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `use_case_watches.rs:78` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `auth.rs:42` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `github.rs:306` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `agent_tokens.rs:125` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `repos.rs:404` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `radar.rs:147` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `mcp_metrics.rs:117` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `agent_tokens.rs:127` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `watchlist.rs:107` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `signal_events.rs:27` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `watchlist.rs:13` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `repo_categories.rs:157` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `mcp_metrics.rs:114` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `repo_signals.rs:37` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `capture.rs:26` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `semantic_search.rs:139` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `repo_categories.rs:203` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `account.rs:14` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `capture.rs:87` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `use_cases.rs:87` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `pipeline.rs:256` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `server.rs:528` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `auth.rs:32` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `reference.rs:24` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `repos_query.rs:70` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `main.rs:39` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `use_cases.rs:63` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `repo_signals.rs:93` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `agent_tokens.rs:110` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `agent_token_events.rs:101` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `use_case_watches.rs:91` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `agent_tokens.rs:39` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `watchlist.rs:22` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `repo_categories.rs:160` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `pipeline.rs:127` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `repos.rs:78` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `repo_owners.rs:143` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `server.rs:513` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `agent_tokens.rs:40` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `repo_signals.rs:84` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `agent_tokens.rs:74` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `agent_token_events.rs:151` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `repo_viewer.rs:31` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `notifications.rs:37` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `mod.rs:30` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `repos.rs:406` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `auth.rs:18` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `use_cases.rs:88` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `github.rs:258` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `use_case_watches.rs:60` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `semantic_search.rs:123` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `notifications.rs:200` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `repos_query.rs:88` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `mcp_metrics.rs:115` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `mod.rs:104` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `notifications.rs:189` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `auth.rs:43` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `auth.rs:59` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `main.rs:45` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `radar.rs:141` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `repos_ingestion.rs:63` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `account.rs:15` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `search.rs:52` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `mod.rs:135` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `use_case_watches.rs:111` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `semantic_search.rs:81` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `signal_events.rs:49` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `repos_ingestion.rs:62` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `notifications.rs:65` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `repos_ingestion.rs:55` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `main.rs:11` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `notifications.rs:37` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `github.rs:79` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `signal_reviews.rs:125` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `reputation.rs:181` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `watchlist.rs:20` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `use_cases.rs:76` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `mcp_rate_limit.rs:64` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `mod.rs:118` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `repo_viewer.rs:27` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `agent_token_events.rs:127` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `watchlist.rs:32` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `watchlist.rs:31` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `repo_owners.rs:106` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `admin.rs:49` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `github.rs:466` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `signal_reviews.rs:102` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `repo_signals.rs:92` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `watchlist.rs:47` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `notifications.rs:48` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `reputation.rs:157` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `mcp_metrics.rs:113` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `auth.rs:25` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `agent_tokens.rs:49` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `notifications.rs:47` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `server.rs:361` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `use_cases.rs:38` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `repos_ingestion.rs:60` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `repo_owners.rs:77` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `mod.rs:103` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `server.rs:419` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `admin.rs:129` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `repo_categories.rs:140` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `pipeline.rs:215` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `mod.rs:11` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `admin.rs:33` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `admin.rs:43` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `recommendations.rs:70` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `use_case_watches.rs:54` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `repos.rs:327` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `agent_tokens.rs:53` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `repo_owners.rs:56` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `pipeline.rs:130` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `agent_tokens.rs:28` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `repos_ingestion.rs:50` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `agent_token_events.rs:264` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `notifications.rs:39` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `seed_github.rs:31` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `mod.rs:119` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `server.rs:549` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `repo_categories.rs:135` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `agent_tokens.rs:31` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `mcp_metrics.rs:116` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `agent_token_events.rs:43` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `repo_signals.rs:38` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `github.rs:485` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `notifications.rs:57` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `repo_viewer.rs:30` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `notifications.rs:125` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `auth.rs:41` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `repo_owners.rs:129` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `me.rs:12` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |
| **INFO** | `repos.rs:433` | Contexte d'erreur manquant — L'operateur ? propage l'erreur sans contexte additionnel |

**Suggestions:**
- Ajoutez du contexte: .context("Failed to do X")?

#### `unused-exports` (128 occurrences)
_128 info_

| Severity | Location | Detail |
|----------|----------|--------|
| **INFO** | `repo_categories.rs:90` | Export inutilise — 'classify_repo' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `auth.rs:14` | Export inutilise — 'verify_agent' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `quality.rs:66` | Export inutilise — 'requires_evidence' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `github.rs:385` | Export inutilise — 'upsert_github_artifact' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `admin.rs:29` | Export inutilise — 'recompute_scores' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `watchlist.rs:73` | Export inutilise — 'list_for_user' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `radar.rs:26` | Export inutilise — 'compute_radar_snapshot' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `recommendations.rs:50` | Export inutilise — 'recommend_for_use_case' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `repos.rs:36` | Export inutilise — 'parse' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `formula.rs:89` | Export inutilise — 'load_v2' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `watchlist.rs:26` | Export inutilise — 'add_to_watchlist' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `mcp_rate_limit.rs:29` | Export inutilise — 'per_minute' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `mcp_rate_limit.rs:21` | Export inutilise — 'new' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `health.rs:12` | Export inutilise — 'health' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `admin.rs:124` | Export inutilise — 'ingest_github_repo' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `signal_reviews.rs:107` | Export inutilise — 'signal_belongs_to_repo' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `github.rs:55` | Export inutilise — 'build_client' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `watchlist.rs:16` | Export inutilise — 'list_watchlist' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `signal_reviews.rs:59` | Export inutilise — 'dispute_signal' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `agent_token_events.rs:67` | Export inutilise — 'enforce_log_usage_guards' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `health.rs:66` | Export inutilise — 'public_status' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `agent_tokens.rs:44` | Export inutilise — 'revoke_agent_token' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `reference.rs:91` | Export inutilise — 'as_str' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `Button.tsx:65` | Export inutilise — 'LinkButton' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `mcp_metrics.rs:30` | Export inutilise — 'parse' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `reputation.rs:75` | Export inutilise — 'requires_strict_active_review' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `notifications.rs:46` | Export inutilise — 'detect_and_emit' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `semantic_search.rs:27` | Export inutilise — 'build_search_document' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `repo_categories.rs:120` | Export inutilise — 'upsert_repo_categories' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `reputation.rs:79` | Export inutilise — 'to_summary' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `repo_signals.rs:98` | Export inutilise — 'dispute_repo_signal' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `use_cases.rs:58` | Export inutilise — 'create_use_case_watch' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `agent_tokens.rs:107` | Export inutilise — 'verify' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `signal_events.rs:8` | Export inutilise — 'record_signal_event' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `recommendations.rs:92` | Export inutilise — 'parse_intent' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `semantic_search.rs:47` | Export inutilise — 'embed_passage' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `compute.rs:167` | Export inutilise — 'vitality_score' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `admin.rs:168` | Export inutilise — 'backfill_repo_embeddings' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `repo_viewer.rs:22` | Export inutilise — 'get_repo_viewer_state' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `repo_categories.rs:128` | Export inutilise — 'upsert_repo_categories_with_readme' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `watchlist.rs:7` | Export inutilise — 'add_watch' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `github.rs:471` | Export inutilise — 'ingest_repo' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `semantic_search.rs:64` | Export inutilise — 'update_repo_embedding' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `repos.rs:60` | Export inutilise — 'find_github_artifact_id' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `quality.rs:40` | Export inutilise — 'as_str' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `notifications.rs:32` | Export inutilise — 'list_notifications' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `weighting.rs:90` | Export inutilise — 'aggregate_weighted_counts' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `agent_token_events.rs:220` | Export inutilise — 'record_watch_use_case' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `repo_owners.rs:7` | Export inutilise — 'user_can_manage_repo_signal' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `repo_categories.rs:94` | Export inutilise — 'classify_repo_with_readme' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `repo_categories.rs:164` | Export inutilise — 'backfill_missing_repo_categories' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `agent_tokens.rs:26` | Export inutilise — 'create' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `auth.rs:63` | Export inutilise — 'discord_callback' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `compute.rs:73` | Export inutilise — 'compute_score' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `use_case_watches.rs:46` | Export inutilise — 'create_watch' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `error.rs:35` | Export inutilise — 'forbidden' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `notifications.rs:43` | Export inutilise — 'unread_count' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `reputation.rs:66` | Export inutilise — 'review_weight' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `auth.rs:55` | Export inutilise — 'discord_start' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `use_cases.rs:23` | Export inutilise — 'recommend_use_case' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `auth.rs:28` | Export inutilise — 'github_start' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `pipeline.rs:119` | Export inutilise — 'recompute_all_scores' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `watchlist.rs:35` | Export inutilise — 'remove_watch' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `repos.rs:100` | Export inutilise — 'search_github_repos' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `search.rs:33` | Export inutilise — 'search' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `repo_signals.rs:25` | Export inutilise — 'create_repo_signal' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `notifications.rs:22` | Export inutilise — 'fetch_prev_snapshot' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `format.ts:73` | Export inutilise — 'notificationLabel' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `use_case_watches.rs:39` | Export inutilise — 'default_watch_label' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `compute.rs:35` | Export inutilise — 'unweighted' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `repos_query.rs:49` | Export inutilise — 'search_repos' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `admin.rs:38` | Export inutilise — 'explain_scoring' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `watchlist.rs:52` | Export inutilise — 'update_watch' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `semantic_search.rs:54` | Export inutilise — 'embed_query' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `notifications.rs:160` | Export inutilise — 'list_for_user' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `repos_ingestion.rs:46` | Export inutilise — 'add_repo' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `reputation.rs:48` | Export inutilise — 'as_str' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `mcp_rate_limit.rs:33` | Export inutilise — 'check' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `reference.rs:16` | Export inutilise — 'parse_reference' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `radar.rs:153` | Export inutilise — 'upsert_repo_radar_snapshot' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `github.rs:508` | Export inutilise — 'parse_github_repo_input' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `agent_tokens.rs:63` | Export inutilise — 'list_for_user' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `use_case_watches.rs:118` | Export inutilise — 'list_watches' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `error.rs:21` | Export inutilise — 'bad_request' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `radar.rs:101` | Export inutilise — 'refresh_all_repo_radar_snapshots' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `signal_events.rs:31` | Export inutilise — 'list_events_for_signals' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `agent_token_events.rs:201` | Export inutilise — 'record_watch_repo' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `mcp_metrics.rs:22` | Export inutilise — 'as_str' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `recommendations.rs:238` | Export inutilise — 'score_candidate' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `notifications.rs:52` | Export inutilise — 'mark_notification_read' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `repos.rs:49` | Export inutilise — 'as_str' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `account.rs:10` | Export inutilise — 'account_summary' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `reputation.rs:59` | Export inutilise — 'active_signal_eligible' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `agent_token_events.rs:177` | Export inutilise — 'record_log_usage' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `auth.rs:10` | Export inutilise — 'verify_bearer' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `weighting.rs:145` | Export inutilise — 'explain_signals' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `mcp_metrics.rs:108` | Export inutilise — 'gather_metrics' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `repos_query.rs:84` | Export inutilise — 'get_repo' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `semantic_search.rs:23` | Export inutilise — 'enabled' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `me.rs:8` | Export inutilise — 'me' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `formula.rs:85` | Export inutilise — 'load_v1' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `pipeline.rs:123` | Export inutilise — 'recompute_all_scores_with_config' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `error.rs:42` | Export inutilise — 'too_many_requests' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `mcp_rate_limit.rs:59` | Export inutilise — 'is_limited' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `error.rs:28` | Export inutilise — 'not_found' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `notifications.rs:194` | Export inutilise — 'unread_count' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `error.rs:49` | Export inutilise — 'unauthorized' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `capture.rs:22` | Export inutilise — 'record_signal' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `scheduler.rs:16` | Export inutilise — 'spawn_recompute_loop' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `notifications.rs:204` | Export inutilise — 'mark_read' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `watchlist.rs:42` | Export inutilise — 'remove_from_watchlist' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `agent_tokens.rs:23` | Export inutilise — 'create_agent_token' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `use_cases.rs:83` | Export inutilise — 'list_use_case_watches' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `agent_tokens.rs:87` | Export inutilise — 'revoke' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `flags.rs:24` | Export inutilise — 'from_config' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `notifications.rs:68` | Export inutilise — 'mark_all_read' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `admin.rs:191` | Export inutilise — 'review_repo_signal' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `auth.rs:36` | Export inutilise — 'github_callback' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `github.rs:62` | Export inutilise — 'fetch_repo' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `quality.rs:55` | Export inutilise — 'is_passive' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `agent_token_events.rs:23` | Export inutilise — 'enforce_write_quota' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `pipeline.rs:332` | Export inutilise — 'explain_external_scoring' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `auth.rs:82` | Export inutilise — 'logout' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `agent_tokens.rs:35` | Export inutilise — 'list_agent_tokens' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `watchlist.rs:51` | Export inutilise — 'set_muted' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `mcp_metrics.rs:14` | Export inutilise — 'as_interval' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `signal_reviews.rs:7` | Export inutilise — 'review_signal' est exporte mais n'est importe par aucun fichier du projet |
| **INFO** | `repos.rs:332` | Export inutilise — 'get_repo_profile' est exporte mais n'est importe par aucun fichier du projet |

**Suggestions:**
- Verifiez si 'LinkButton' est encore necessaire, sinon retirez l'export
- Verifiez si 'account_summary' est encore necessaire, sinon retirez l'export
- Verifiez si 'active_signal_eligible' est encore necessaire, sinon retirez l'export

#### `react-missing-key` (47 occurrences)
_47 warning_

| Severity | Location | Detail |
|----------|----------|--------|
| **WARNING** | `privacy.tsx:17` | Missing key prop in React list — map() with JSX - can cause rendering bugs |
| **WARNING** | `RepoSignalsList.tsx:56` | Missing key prop in React list — map() with JSX - can cause rendering bugs |
| **WARNING** | `how-to-read.tsx:55` | Missing key prop in React list — map() with JSX - can cause rendering bugs |
| **WARNING** | `index.tsx:227` | Missing key prop in React list — map() with JSX - can cause rendering bugs |
| **WARNING** | `index.tsx:453` | Missing key prop in React list — map() with JSX - can cause rendering bugs |
| **WARNING** | `RepoHeader.tsx:88` | Missing key prop in React list — map() with JSX - can cause rendering bugs |
| **WARNING** | `how-to-read.tsx:161` | Missing key prop in React list — map() with JSX - can cause rendering bugs |
| **WARNING** | `discover.tsx:464` | Missing key prop in React list — map() with JSX - can cause rendering bugs |
| **WARNING** | `how-to-read.tsx:122` | Missing key prop in React list — map() with JSX - can cause rendering bugs |
| **WARNING** | `watchlist.tsx:132` | Missing key prop in React list — map() with JSX - can cause rendering bugs |
| **WARNING** | `mcp-guide.tsx:225` | Missing key prop in React list — map() with JSX - can cause rendering bugs |
| **WARNING** | `AgentTokensPanel.tsx:143` | Missing key prop in React list — map() with JSX - can cause rendering bugs |
| **WARNING** | `watchlist.tsx:296` | Missing key prop in React list — map() with JSX - can cause rendering bugs |
| **WARNING** | `AdminMcpObservabilityPanel.tsx:200` | Missing key prop in React list — map() with JSX - can cause rendering bugs |
| **WARNING** | `discover.tsx:482` | Missing key prop in React list — map() with JSX - can cause rendering bugs |
| **WARNING** | `RepoCard.tsx:93` | Missing key prop in React list — map() with JSX - can cause rendering bugs |
| **WARNING** | `UseCaseSearchPanel.tsx:195` | Missing key prop in React list — map() with JSX - can cause rendering bugs |
| **WARNING** | `discover.tsx:431` | Missing key prop in React list — map() with JSX - can cause rendering bugs |
| **WARNING** | `discover.tsx:339` | Missing key prop in React list — map() with JSX - can cause rendering bugs |
| **WARNING** | `mcp-guide.tsx:282` | Missing key prop in React list — map() with JSX - can cause rendering bugs |
| **WARNING** | `UseCaseSearchPanel.tsx:113` | Missing key prop in React list — map() with JSX - can cause rendering bugs |
| **WARNING** | `how-to-read.tsx:140` | Missing key prop in React list — map() with JSX - can cause rendering bugs |
| **WARNING** | `RepoCard.tsx:88` | Missing key prop in React list — map() with JSX - can cause rendering bugs |
| **WARNING** | `watchlist.tsx:188` | Missing key prop in React list — map() with JSX - can cause rendering bugs |
| **WARNING** | `mcp-guide.tsx:194` | Missing key prop in React list — map() with JSX - can cause rendering bugs |
| **WARNING** | `notifications.tsx:142` | Missing key prop in React list — map() with JSX - can cause rendering bugs |
| **WARNING** | `mcp-guide.tsx:306` | Missing key prop in React list — map() with JSX - can cause rendering bugs |
| **WARNING** | `how-to-read.tsx:107` | Missing key prop in React list — map() with JSX - can cause rendering bugs |
| **WARNING** | `AdminMcpObservabilityPanel.tsx:155` | Missing key prop in React list — map() with JSX - can cause rendering bugs |
| **WARNING** | `RepoHeader.tsx:83` | Missing key prop in React list — map() with JSX - can cause rendering bugs |
| **WARNING** | `AdminMcpObservabilityPanel.tsx:133` | Missing key prop in React list — map() with JSX - can cause rendering bugs |
| **WARNING** | `discover.tsx:554` | Missing key prop in React list — map() with JSX - can cause rendering bugs |
| **WARNING** | `RepoHeader.tsx:78` | Missing key prop in React list — map() with JSX - can cause rendering bugs |
| **WARNING** | `UseCaseSearchPanel.tsx:203` | Missing key prop in React list — map() with JSX - can cause rendering bugs |
| **WARNING** | `discover.tsx:694` | Missing key prop in React list — map() with JSX - can cause rendering bugs |
| **WARNING** | `AdminMcpObservabilityPanel.tsx:246` | Missing key prop in React list — map() with JSX - can cause rendering bugs |
| **WARNING** | `repo-detail.tsx:213` | Missing key prop in React list — map() with JSX - can cause rendering bugs |
| **WARNING** | `discover.tsx:495` | Missing key prop in React list — map() with JSX - can cause rendering bugs |
| **WARNING** | `UseCaseSearchPanel.tsx:118` | Missing key prop in React list — map() with JSX - can cause rendering bugs |
| **WARNING** | `UseCaseSearchPanel.tsx:166` | Missing key prop in React list — map() with JSX - can cause rendering bugs |
| **WARNING** | `discover.tsx:404` | Missing key prop in React list — map() with JSX - can cause rendering bugs |
| **WARNING** | `AdminMcpObservabilityPanel.tsx:178` | Missing key prop in React list — map() with JSX - can cause rendering bugs |
| **WARNING** | `watchlist.tsx:304` | Missing key prop in React list — map() with JSX - can cause rendering bugs |
| **WARNING** | `mcp-guide.tsx:247` | Missing key prop in React list — map() with JSX - can cause rendering bugs |
| **WARNING** | `AdminMcpObservabilityPanel.tsx:224` | Missing key prop in React list — map() with JSX - can cause rendering bugs |
| **WARNING** | `RepoCard.tsx:83` | Missing key prop in React list — map() with JSX - can cause rendering bugs |
| **WARNING** | `OwnerDisputePanel.tsx:49` | Missing key prop in React list — map() with JSX - can cause rendering bugs |

**Suggestions:**
- Add a unique key prop (e.g., key={item.id} or key={index})

#### `unwrap-panic-rust` (36 occurrences)
_36 warning_

| Severity | Location | Detail |
|----------|----------|--------|
| **WARNING** | `mcp_metrics.rs:343` | .unwrap() risque — .unwrap() causera un panic si la valeur est None/Err |
| **WARNING** | `recommendations.rs:742` | .unwrap() risque — .unwrap() causera un panic si la valeur est None/Err |
| **WARNING** | `reference.rs:128` | .unwrap() risque — .unwrap() causera un panic si la valeur est None/Err |
| **WARNING** | `weighting.rs:324` | .unwrap() risque — .unwrap() causera un panic si la valeur est None/Err |
| **WARNING** | `recommendations.rs:731` | .unwrap() risque — .unwrap() causera un panic si la valeur est None/Err |
| **WARNING** | `mcp_metrics.rs:351` | .unwrap() risque — .unwrap() causera un panic si la valeur est None/Err |
| **WARNING** | `server.rs:1506` | .unwrap() risque — .unwrap() causera un panic si la valeur est None/Err |
| **WARNING** | `github.rs:561` | .unwrap() risque — .unwrap() causera un panic si la valeur est None/Err |
| **WARNING** | `server.rs:1501` | .unwrap() risque — .unwrap() causera un panic si la valeur est None/Err |
| **WARNING** | `server.rs:1532` | .unwrap() risque — .unwrap() causera un panic si la valeur est None/Err |
| **WARNING** | `mcp_metrics.rs:335` | .unwrap() risque — .unwrap() causera un panic si la valeur est None/Err |
| **WARNING** | `weighting.rs:313` | .unwrap() risque — .unwrap() causera un panic si la valeur est None/Err |
| **WARNING** | `weighting.rs:249` | .unwrap() risque — .unwrap() causera un panic si la valeur est None/Err |
| **WARNING** | `server.rs:1620` | .unwrap() risque — .unwrap() causera un panic si la valeur est None/Err |
| **WARNING** | `github.rs:553` | .unwrap() risque — .unwrap() causera un panic si la valeur est None/Err |
| **WARNING** | `server.rs:1497` | .unwrap() risque — .unwrap() causera un panic si la valeur est None/Err |
| **WARNING** | `mcp_metrics.rs:320` | .unwrap() risque — .unwrap() causera un panic si la valeur est None/Err |
| **WARNING** | `radar.rs:279` | .unwrap() risque — .unwrap() causera un panic si la valeur est None/Err |
| **WARNING** | `recommendations.rs:749` | .unwrap() risque — .unwrap() causera un panic si la valeur est None/Err |
| **WARNING** | `repo_categories.rs:561` | .unwrap() risque — .unwrap() causera un panic si la valeur est None/Err |
| **WARNING** | `weighting.rs:294` | .unwrap() risque — .unwrap() causera un panic si la valeur est None/Err |
| **WARNING** | `server.rs:1531` | .unwrap() risque — .unwrap() causera un panic si la valeur est None/Err |
| **WARNING** | `mcp_metrics.rs:339` | .unwrap() risque — .unwrap() causera un panic si la valeur est None/Err |
| **WARNING** | `weighting.rs:277` | .unwrap() risque — .unwrap() causera un panic si la valeur est None/Err |
| **WARNING** | `server.rs:1529` | .unwrap() risque — .unwrap() causera un panic si la valeur est None/Err |
| **WARNING** | `server.rs:1504` | .unwrap() risque — .unwrap() causera un panic si la valeur est None/Err |
| **WARNING** | `server.rs:1493` | .unwrap() risque — .unwrap() causera un panic si la valeur est None/Err |
| **WARNING** | `mcp_metrics.rs:327` | .unwrap() risque — .unwrap() causera un panic si la valeur est None/Err |
| **WARNING** | `mcp_metrics.rs:347` | .unwrap() risque — .unwrap() causera un panic si la valeur est None/Err |
| **WARNING** | `server.rs:1530` | .unwrap() risque — .unwrap() causera un panic si la valeur est None/Err |
| **WARNING** | `mcp_metrics.rs:323` | .unwrap() risque — .unwrap() causera un panic si la valeur est None/Err |
| **WARNING** | `mcp_metrics.rs:321` | .unwrap() risque — .unwrap() causera un panic si la valeur est None/Err |
| **WARNING** | `reference.rs:120` | .unwrap() risque — .unwrap() causera un panic si la valeur est None/Err |
| **WARNING** | `github.rs:568` | .unwrap() risque — .unwrap() causera un panic si la valeur est None/Err |
| **WARNING** | `weighting.rs:228` | .unwrap() risque — .unwrap() causera un panic si la valeur est None/Err |
| **WARNING** | `github.rs:576` | .unwrap() risque — .unwrap() causera un panic si la valeur est None/Err |

**Suggestions:**
- Utilisez .unwrap_or(), .unwrap_or_else(), ou match/if let

#### `rs-unwrap-ast` (11 occurrences)
_11 warning_

| Severity | Location | Detail |
|----------|----------|--------|
| **WARNING** | `server.rs:1928` | Usage de .unwrap() detecte (AST) — .unwrap() peut causer un panic si la valeur est None/Err |
| **WARNING** | `server.rs:1818` | Usage de .unwrap() detecte (AST) — .unwrap() peut causer un panic si la valeur est None/Err |
| **WARNING** | `server.rs:1621` | Usage de .unwrap() detecte (AST) — .unwrap() peut causer un panic si la valeur est None/Err |
| **WARNING** | `server.rs:1685` | Usage de .unwrap() detecte (AST) — .unwrap() peut causer un panic si la valeur est None/Err |
| **WARNING** | `server.rs:1926` | Usage de .unwrap() detecte (AST) — .unwrap() peut causer un panic si la valeur est None/Err |
| **WARNING** | `server.rs:1853` | Usage de .unwrap() detecte (AST) — .unwrap() peut causer un panic si la valeur est None/Err |
| **WARNING** | `server.rs:1856` | Usage de .unwrap() detecte (AST) — .unwrap() peut causer un panic si la valeur est None/Err |
| **WARNING** | `server.rs:1816` | Usage de .unwrap() detecte (AST) — .unwrap() peut causer un panic si la valeur est None/Err |
| **WARNING** | `server.rs:1684` | Usage de .unwrap() detecte (AST) — .unwrap() peut causer un panic si la valeur est None/Err |
| **WARNING** | `server.rs:2017` | Usage de .unwrap() detecte (AST) — .unwrap() peut causer un panic si la valeur est None/Err |
| **WARNING** | `server.rs:1952` | Usage de .unwrap() detecte (AST) — .unwrap() peut causer un panic si la valeur est None/Err |

**Suggestions:**
- Utilisez match, if let, unwrap_or, ou ? operator

#### `magic-numbers` (10 occurrences)
_10 info_

| Severity | Location | Detail |
|----------|----------|--------|
| **INFO** | `mcp-guide.tsx:92` | Magic numbers detected — 8 magic numbers detected (e.g., 401) |
| **INFO** | `format.ts:27` | Magic numbers detected — 3 magic numbers detected (e.g., 10000) |
| **INFO** | `AppHeader.tsx:29` | Magic numbers detected — 5 magic numbers detected (e.g., 30) |
| **INFO** | `index.tsx:336` | Magic numbers detected — 5 magic numbers detected (e.g., 30) |
| **INFO** | `Button.tsx:18` | Magic numbers detected — 3 magic numbers detected (e.g., 150) |
| **INFO** | `Chip.tsx:15` | Magic numbers detected — 5 magic numbers detected (e.g., 30) |
| **INFO** | `discover.tsx:216` | Magic numbers detected — 8 magic numbers detected (e.g., 45) |
| **INFO** | `how-to-read.tsx:19` | Magic numbers detected — 4 magic numbers detected (e.g., 150) |
| **INFO** | `watchlist.tsx:163` | Magic numbers detected — 5 magic numbers detected (e.g., 40) |
| **INFO** | `notifications.tsx:108` | Magic numbers detected — 4 magic numbers detected (e.g., 30) |

**Suggestions:**
- Extract numbers into named constants (e.g., MAX_RETRIES = 5)

#### `sync-file-ops` (9 occurrences)
_9 warning_

| Severity | Location | Detail |
|----------|----------|--------|
| **WARNING** | `cli.test.mjs:54` | Synchronous operation — writeFileSync blocks the event loop |
| **WARNING** | `cli.test.mjs:91` | Synchronous operation — readdirSync blocks the event loop |
| **WARNING** | `cli.test.mjs:84` | Synchronous operation — readFileSync blocks the event loop |
| **WARNING** | `usestakly-mcp.mjs:104` | Synchronous operation — readFileSync blocks the event loop |
| **WARNING** | `usestakly-mcp.mjs:269` | Synchronous operation — writeFileSync blocks the event loop |
| **WARNING** | `usestakly-mcp.mjs:258` | Synchronous operation — readFileSync blocks the event loop |
| **WARNING** | `usestakly-mcp.mjs:247` | Synchronous operation — writeFileSync blocks the event loop |
| **WARNING** | `usestakly-mcp.mjs:252` | Synchronous operation — readFileSync blocks the event loop |
| **WARNING** | `cli.test.mjs:111` | Synchronous operation — existsSync blocks the event loop |

**Suggestions:**
- Use the async version: exists with await
- Use the async version: readFile with await
- Use the async version: readdir with await

#### `floating-promise` (6 occurrences)
_6 warning_

| Severity | Location | Detail |
|----------|----------|--------|
| **WARNING** | `usestakly-mcp.mjs:63` | Potentially unwaited promise — This async function is not awaited - errors will be ignored |
| **WARNING** | `usestakly-mcp.mjs:61` | Potentially unwaited promise — This async function is not awaited - errors will be ignored |
| **WARNING** | `usestakly-mcp.mjs:73` | Potentially unwaited promise — This async function is not awaited - errors will be ignored |
| **WARNING** | `usestakly-mcp.mjs:75` | Potentially unwaited promise — This async function is not awaited - errors will be ignored |
| **WARNING** | `usestakly-mcp.mjs:78` | Potentially unwaited promise — This async function is not awaited - errors will be ignored |
| **WARNING** | `hooks.ts:37` | Potentially unwaited promise — This async function is not awaited - errors will be ignored |

**Suggestions:**
- Add await or use .catch() to handle errors

#### `unused-variables` (3 occurrences)
_3 info_

| Severity | Location | Detail |
|----------|----------|--------|
| **INFO** | `auth-store.ts:16` | Variable non utilisee — La variable "useAuthStore" est declaree mais jamais utilisee |
| **INFO** | `fr.ts:3` | Variable non utilisee — La variable "fr" est declaree mais jamais utilisee |
| **INFO** | `locale-store.ts:12` | Variable non utilisee — La variable "useLocaleStore" est declaree mais jamais utilisee |

**Suggestions:**
- Supprimez la variable inutilisee ou prefixez-la avec _ si intentionnel

#### `long-ternary` (3 occurrences)
_3 warning_

| Severity | Location | Detail |
|----------|----------|--------|
| **WARNING** | `discover.tsx:287` | Nested ternaries — 6 nested ternaries detected |
| **WARNING** | `notifications.tsx:136` | Nested ternaries — 2 nested ternaries detected |
| **WARNING** | `AppHeader.tsx:52` | Nested ternaries — 2 nested ternaries detected |

**Suggestions:**
- Replace nested ternaries with if/else or a function

#### `shadow-variable` (2 occurrences)
_2 warning_

| Severity | Location | Detail |
|----------|----------|--------|
| **WARNING** | `semantic_search.rs:160` | Variable shadows outer scope — 'model' shadows a variable from an outer scope |
| **WARNING** | `usestakly-mcp.mjs:89` | Variable shadows outer scope — 'endpoint' shadows a variable from an outer scope |

**Suggestions:**
- Rename this variable to avoid confusion with the outer one

#### `inefficient-collection-ops` (2 occurrences)
_2 info_

| Severity | Location | Detail |
|----------|----------|--------|
| **INFO** | `usestakly-mcp.mjs:105` | Array lookup in loop — Using .includes()/.indexOf()/.find() in a loop is O(n²) |
| **INFO** | `usestakly-mcp.mjs:106` | Array lookup in loop — Using .includes()/.indexOf()/.find() in a loop is O(n²) |

**Suggestions:**
- Convert array to Set for O(1) lookups: new Set(array).has(item)

#### `empty-catch` (1 occurrences)
_1 warning_

| Severity | Location | Detail |
|----------|----------|--------|
| **WARNING** | `api-client.ts:40` | Empty catch block — An empty catch block silently ignores errors |

**Suggestions:**
- Log the error or rethrow it: catch (e) { console.error(e); throw e; }

#### `sql-select-star` (1 occurrences)
_1 info_

| Severity | Location | Detail |
|----------|----------|--------|
| **INFO** | `repos.rs:217` | SELECT * detecte — SELECT * retourne toutes les colonnes, preferez lister les colonnes explicitement |

**Suggestions:**
- Listez explicitement les colonnes necessaires au lieu de SELECT *

#### `file-too-long` (1 occurrences)
_1 warning_

| Severity | Location | Detail |
|----------|----------|--------|
| **WARNING** | `server.rs` | File too long — This file has 2044 lines (threshold: 1000 lines) |

**Suggestions:**
- Split this file into smaller modules for better readability

#### `excessive-clone-rust` (1 occurrences)
_1 info_

| Severity | Location | Detail |
|----------|----------|--------|
| **INFO** | `mod.rs` | Nombreux .clone() — 17 appels a .clone() dans ce fichier - considerez l'utilisation de references |

**Suggestions:**
- Utilisez des references (&T) ou Rc/Arc quand possible


### Qualité — 98 / 100
> Historique Git, branches orphelines, fichiers sensibles et qualité globale du code

4 issue(s) found.

#### `hardcoded-test-data` (4 occurrences)
_4 info_

| Severity | Location | Detail |
|----------|----------|--------|
| **INFO** | `server.rs:1432` | Données de test détectées — Donnée de test "localhost IP" trouvée dans du code de production |
| **INFO** | `server.rs:2014` | Données de test détectées — Donnée de test "localhost IP" trouvée dans du code de production |
| **INFO** | `server.rs:1522` | Données de test détectées — Donnée de test "localhost IP" trouvée dans du code de production |
| **INFO** | `mod.rs:45` | Données de test détectées — Donnée de test "localhost IP" trouvée dans du code de production |

**Suggestions:**
- Remplacez par des données dynamiques ou déplacez dans un fichier de fixtures


### Documentation & Tests — 41 / 100
> README incomplet, fonctions non documentées, couverture de tests insuffisante et assertions faibles

99 issue(s) found.

#### `test-source-coverage-gap` (80 occurrences)
_61 warning | 19 info_

| Severity | Location | Detail |
|----------|----------|--------|
| **WARNING** | `mod.rs` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 21 mais aucun fichier de test ne l'importe |
| **INFO** | `account.ts` | Fichier sans tests — Fichier avec complexite cyclomatique de 6 sans tests associes |
| **WARNING** | `recommendations.rs` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 90 mais aucun fichier de test ne l'importe |
| **WARNING** | `auth.rs` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 17 mais aucun fichier de test ne l'importe |
| **WARNING** | `account.tsx` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 26 mais aucun fichier de test ne l'importe |
| **INFO** | `signal_reviews.rs` | Fichier sans tests — Fichier avec complexite cyclomatique de 8 sans tests associes |
| **INFO** | `ReportSignalForm.tsx` | Fichier sans tests — Fichier avec complexite cyclomatique de 10 sans tests associes |
| **WARNING** | `mcp_rate_limit.rs` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 11 mais aucun fichier de test ne l'importe |
| **INFO** | `auth-store.ts` | Fichier sans tests — Fichier avec complexite cyclomatique de 8 sans tests associes |
| **WARNING** | `repo_signals.rs` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 23 mais aucun fichier de test ne l'importe |
| **WARNING** | `agent_tokens.rs` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 17 mais aucun fichier de test ne l'importe |
| **WARNING** | `watchlist.rs` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 13 mais aucun fichier de test ne l'importe |
| **WARNING** | `server.rs` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 167 mais aucun fichier de test ne l'importe |
| **INFO** | `repos_query.rs` | Fichier sans tests — Fichier avec complexite cyclomatique de 10 sans tests associes |
| **WARNING** | `status.tsx` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 37 mais aucun fichier de test ne l'importe |
| **INFO** | `agent_tokens.rs` | Fichier sans tests — Fichier avec complexite cyclomatique de 10 sans tests associes |
| **WARNING** | `repo-detail.tsx` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 58 mais aucun fichier de test ne l'importe |
| **INFO** | `Wordmark.tsx` | Fichier sans tests — Fichier avec complexite cyclomatique de 7 sans tests associes |
| **WARNING** | `hooks.ts` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 19 mais aucun fichier de test ne l'importe |
| **WARNING** | `quality.rs` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 16 mais aucun fichier de test ne l'importe |
| **WARNING** | `semantic_search.rs` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 32 mais aucun fichier de test ne l'importe |
| **WARNING** | `AdminModerationPanel.tsx` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 47 mais aucun fichier de test ne l'importe |
| **INFO** | `mod.rs` | Fichier sans tests — Fichier avec complexite cyclomatique de 6 sans tests associes |
| **WARNING** | `repo_owners.rs` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 39 mais aucun fichier de test ne l'importe |
| **WARNING** | `router.tsx` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 14 mais aucun fichier de test ne l'importe |
| **WARNING** | `AgentTokensPanel.tsx` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 30 mais aucun fichier de test ne l'importe |
| **WARNING** | `mod.rs` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 75 mais aucun fichier de test ne l'importe |
| **WARNING** | `repo_categories.rs` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 56 mais aucun fichier de test ne l'importe |
| **WARNING** | `RepoSignalsList.tsx` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 31 mais aucun fichier de test ne l'importe |
| **INFO** | `repos.ts` | Fichier sans tests — Fichier avec complexite cyclomatique de 8 sans tests associes |
| **WARNING** | `reference.rs` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 17 mais aucun fichier de test ne l'importe |
| **WARNING** | `scheduler.rs` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 25 mais aucun fichier de test ne l'importe |
| **WARNING** | `github.rs` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 73 mais aucun fichier de test ne l'importe |
| **WARNING** | `AppHeader.tsx` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 17 mais aucun fichier de test ne l'importe |
| **WARNING** | `discover.tsx` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 216 mais aucun fichier de test ne l'importe |
| **WARNING** | `capture.rs` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 16 mais aucun fichier de test ne l'importe |
| **INFO** | `locale-store.ts` | Fichier sans tests — Fichier avec complexite cyclomatique de 7 sans tests associes |
| **INFO** | `auth.rs` | Fichier sans tests — Fichier avec complexite cyclomatique de 6 sans tests associes |
| **WARNING** | `api-client.ts` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 33 mais aucun fichier de test ne l'importe |
| **WARNING** | `notifications.rs` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 24 mais aucun fichier de test ne l'importe |
| **INFO** | `use-cases.ts` | Fichier sans tests — Fichier avec complexite cyclomatique de 8 sans tests associes |
| **WARNING** | `RepoHeader.tsx` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 30 mais aucun fichier de test ne l'importe |
| **WARNING** | `admin.rs` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 61 mais aucun fichier de test ne l'importe |
| **WARNING** | `AdminMcpObservabilityPanel.tsx` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 33 mais aucun fichier de test ne l'importe |
| **WARNING** | `return-to.ts` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 13 mais aucun fichier de test ne l'importe |
| **WARNING** | `use_cases.rs` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 11 mais aucun fichier de test ne l'importe |
| **INFO** | `repo_viewer.rs` | Fichier sans tests — Fichier avec complexite cyclomatique de 6 sans tests associes |
| **WARNING** | `notifications.rs` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 12 mais aucun fichier de test ne l'importe |
| **WARNING** | `seed_github.rs` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 15 mais aucun fichier de test ne l'importe |
| **WARNING** | `pipeline.rs` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 27 mais aucun fichier de test ne l'importe |
| **WARNING** | `flags.rs` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 20 mais aucun fichier de test ne l'importe |
| **WARNING** | `notifications.tsx` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 77 mais aucun fichier de test ne l'importe |
| **WARNING** | `use_case_watches.rs` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 23 mais aucun fichier de test ne l'importe |
| **WARNING** | `OwnerDisputePanel.tsx` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 24 mais aucun fichier de test ne l'importe |
| **WARNING** | `compute.rs` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 36 mais aucun fichier de test ne l'importe |
| **WARNING** | `UseCaseSearchPanel.tsx` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 51 mais aucun fichier de test ne l'importe |
| **INFO** | `ScoreBar.tsx` | Fichier sans tests — Fichier avec complexite cyclomatique de 7 sans tests associes |
| **WARNING** | `RepoCard.tsx` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 32 mais aucun fichier de test ne l'importe |
| **WARNING** | `main.rs` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 14 mais aucun fichier de test ne l'importe |
| **WARNING** | `error.rs` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 12 mais aucun fichier de test ne l'importe |
| **WARNING** | `index.tsx` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 60 mais aucun fichier de test ne l'importe |
| **WARNING** | `RepoMetricsPanel.tsx` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 24 mais aucun fichier de test ne l'importe |
| **INFO** | `repos_ingestion.rs` | Fichier sans tests — Fichier avec complexite cyclomatique de 7 sans tests associes |
| **INFO** | `signal_events.rs` | Fichier sans tests — Fichier avec complexite cyclomatique de 6 sans tests associes |
| **INFO** | `LocaleSwitch.tsx` | Fichier sans tests — Fichier avec complexite cyclomatique de 8 sans tests associes |
| **WARNING** | `repos.rs` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 55 mais aucun fichier de test ne l'importe |
| **WARNING** | `weighting.rs` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 35 mais aucun fichier de test ne l'importe |
| **INFO** | `radar.ts` | Fichier sans tests — Fichier avec complexite cyclomatique de 9 sans tests associes |
| **WARNING** | `watchlist.rs` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 13 mais aucun fichier de test ne l'importe |
| **WARNING** | `radar.rs` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 40 mais aucun fichier de test ne l'importe |
| **WARNING** | `watchlist.tsx` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 86 mais aucun fichier de test ne l'importe |
| **WARNING** | `agent_token_events.rs` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 29 mais aucun fichier de test ne l'importe |
| **INFO** | `how-to-read.tsx` | Fichier sans tests — Fichier avec complexite cyclomatique de 7 sans tests associes |
| **WARNING** | `reputation.rs` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 45 mais aucun fichier de test ne l'importe |
| **WARNING** | `usestakly-mcp.mjs` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 110 mais aucun fichier de test ne l'importe |
| **WARNING** | `mcp_metrics.rs` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 36 mais aucun fichier de test ne l'importe |
| **WARNING** | `mcp-guide.tsx` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 63 mais aucun fichier de test ne l'importe |
| **WARNING** | `mod.rs` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 35 mais aucun fichier de test ne l'importe |
| **WARNING** | `health.rs` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 12 mais aucun fichier de test ne l'importe |
| **WARNING** | `format.ts` | Fichier complexe sans tests — Fichier avec complexite cyclomatique de 32 mais aucun fichier de test ne l'importe |

**Suggestions:**
- Considerez ajouter des tests pour ce module
- Creez un fichier de test pour couvrir ce module complexe

#### `broken-documentation-links` (10 occurrences)
_10 warning_

| Severity | Location | Detail |
|----------|----------|--------|
| **WARNING** | `README.md:185` | Lien de documentation probablement casse ou incomplet — Lien suspect: ./TODO.md |
| **WARNING** | `mvp-action-plan.md:11` | Lien de documentation probablement casse ou incomplet — Lien suspect: ../../TODO.md |
| **WARNING** | `README.md:16` | Lien de documentation probablement casse ou incomplet — Lien suspect: ../../../TODO.md |
| **WARNING** | `mvp-file-by-file-checklist.md:10` | Lien de documentation probablement casse ou incomplet — Lien suspect: ../../TODO.md |
| **WARNING** | `05-rules-engine.md:12` | Lien de documentation probablement casse ou incomplet — Lien suspect: ../../TODO.md |
| **WARNING** | `README.md:21` | Lien de documentation probablement casse ou incomplet — Lien suspect: ../TODO.md |
| **WARNING** | `README.md:17` | Lien de documentation probablement casse ou incomplet — Lien suspect: ../../../TODO.md |
| **WARNING** | `00-overview.md:10` | Lien de documentation probablement casse ou incomplet — Lien suspect: ../../TODO.md |
| **WARNING** | `04-detection-engine.md:10` | Lien de documentation probablement casse ou incomplet — Lien suspect: ../../TODO.md |
| **WARNING** | `mvp-one-shot-blueprint.md:10` | Lien de documentation probablement casse ou incomplet — Lien suspect: ../../TODO.md |

**Suggestions:**
- Verifiez et corrigez le lien

#### `assertion-density` (6 occurrences)
_5 warning | 1 info_

| Severity | Location | Detail |
|----------|----------|--------|
| **INFO** | `mvp.spec.ts:238` | Test avec trop d'assertions — Le test 'authenticated MVP flow covers discovery, repo profile, watchlist, and notifications' contient 26 assertions — d |
| **WARNING** | `cli.test.mjs:36` | Test sans assertion (AST) — Le test 'generic install accepts endpoint from environment' ne contient aucune assertion |
| **WARNING** | `cli.test.mjs:20` | Test sans assertion (AST) — Le test 'generic install prints MCP JSON' ne contient aucune assertion |
| **WARNING** | `cli.test.mjs:94` | Test sans assertion (AST) — Le test 'dry run does not write codex config' ne contient aucune assertion |
| **WARNING** | `cli.test.mjs:44` | Test sans assertion (AST) — Le test 'non-interactive install requires an endpoint' ne contient aucune assertion |
| **WARNING** | `cli.test.mjs:51` | Test sans assertion (AST) — Le test 'codex install writes config and removes previous usestakly section' ne contient aucune assertion |

**Suggestions:**
- Ajoutez des assertions pour verifier le comportement attendu
- Divisez ce test en plusieurs tests plus petits et cibles

#### `missing-license-file` (1 occurrences)
_1 warning_

| Severity | Location | Detail |
|----------|----------|--------|
| **WARNING** | - | Fichier LICENSE manquant — Le projet n'a pas de fichier LICENSE |

**Suggestions:**
- Ajoutez un fichier LICENSE (MIT, Apache 2.0, GPL, etc.)

#### `low-test-ratio` (1 occurrences)
_1 info_

| Severity | Location | Detail |
|----------|----------|--------|
| **INFO** | - | Peu de fichiers de test — Seulement 2 fichier(s) de test pour 113 fichiers source (ratio: 2%). |

**Suggestions:**
- Augmentez la couverture de tests en ajoutant des tests pour les fichiers critiques.

#### `missing-readme-sections` (1 occurrences)
_1 warning_

| Severity | Location | Detail |
|----------|----------|--------|
| **WARNING** | `README.md` | Sections essentielles manquantes dans README — Sections manquantes: Installation, Usage, API |

**Suggestions:**
- Ajoutez les sections: Installation, Usage, et API/Reference

---

## Top Affected Files

| File | Critical | Warning | Info | Total |
|------|----------|---------|------|-------|
| `server.rs` | 1 | 27 | 9 | 37 |
| `mod.rs` | 1 | 12 | 17 | 30 |
| `notifications.rs` | 1 | 3 | 21 | 25 |
| `discover.tsx` | 2 | 15 | 7 | 24 |
| `agent_tokens.rs` | 0 | 1 | 20 | 21 |
| `mcp_metrics.rs` | 0 | 10 | 10 | 20 |
| `watchlist.rs` | 0 | 2 | 18 | 20 |
| `auth.rs` | 0 | 2 | 17 | 19 |
| `agent_token_events.rs` | 2 | 5 | 11 | 18 |
| `github.rs` | 0 | 6 | 11 | 17 |

---

## Security References

**CWE:** CWE-200, CWE-319, CWE-614

**OWASP Top 10:**
- A05:2021 - Security Misconfiguration

---

*Generated by Herald — itsasync.fr*
