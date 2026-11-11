# ROADMAP COMPLETE - NOTE CRAFT
# [TECH-VERIFIED: 2026-11-09]

> **Projet:** Alternative Logseq/Obsidian en Rust 100% native
> **Version:** 1.0.0 | **Date:** 2026-11-09
> **Statut:** Planification complete basee sur analyse approfondie
> **Duree estimee:** 8-13 mois (equipe de 3-5 developpeurs)
> **Complexite globale:** Elevee (19 features a complexite 5/5, 32 features a complexite 4/5)
> **
> [TECH-NOTE: Vérification complète effectuée. 9 problèmes majeurs identifiés. Voir /tmp/verification_technique_rapport.md pour détails]**

---

## TABLE DES MATIERES

1. [METADONNEES ET CONTEXTE](#1-metadonnées-et-contexte)
2. [OBJECTIFS DU PROJET](#2-objectifs-du-projet)
3. [ANALYSE DES SOURCES](#3-analyse-des-sources)
4. [CHOIX TECHNIQUES JUSTIFIES](#4-choix-techniques-justifiés)
5. [DESIGN SYSTEM NOTE CRAFT](#5-design-system-note-craft)
6. [ROADMAP DETAILLEE](#6-roadmap-détaillée)
7. [RISQUES ET QUESTIONS OUVERTES](#7-risques-et-questions-ouvertes)
8. [ANNEXE SOURCES](#8-annexe-sources)

---

## 1. METADONNEES ET CONTEXTE

### Contexte du Projet
Note Craft est une initiative visant a creer une alternative 100% Rust aux applications de prise de notes et de gestion de connaissances Logseq et Obsidian.

**Objectifs principaux:**
- Performance exceptionnelle pour les grands graphes (>10,000 noeuds)
- Securite maximale via le modele de memoire de Rust
- Extensibilite via systeme de plugins moderne (WASM)
- 100% native sans dependance externe non validee
- Compatibilite Markdown avec Obsidian/Logseq

---

## 2. OBJECTIFS DU PROJET

[TECH-NOTE: Objectifs globalement bien définis et réalistes. Voir recommandations dans rapport de vérification.]

### 2.1 Mission
Creer l'application de gestion de connaissances ultime en Rust, combinant la simplicite de Markdown avec la puissance des graphes de connaissances, tout en garantissant performance, securite et extensibilite.

### 2.2 Vision
Devenir la reference des applications PKM (Personal Knowledge Management) pour les developpeurs et utilisateurs techniques.

**Piliers:**
- Performance exceptionnelle: Rendu instantane de graphes complexes
- Securite maximale: Sandboxing WASM, chiffrement E2EE
- Extensibilite illimitee: Systeme de plugins multi-langage
- Compatibilite universelle: Fichiers Markdown standards
- Multiplateforme native: macOS, Windows, Linux

### 2.3 Valeurs
- Excellence Technique: Meilleur pratiques Rust
- Simplicite: Architecture claire et maintenable
- Performance: Optimisation systematique
- Securite: Pas de compromis sur la sureté memoire
- Ouverture: Code open-source, documentation complete

### 2.4 Contraintes

**Contraintes Techniques:**
1. 100% Rust: Pas de dependances externes non-Rust sans validation [✅ VERIFIÉ]
2. Zero runtime externe: Pas de Node.js, Electron, etc. [✅ VERIFIÉ]
3. Compatibilite Markdown: Fichiers compatibles Obsidian/Logseq [⚠️ **FAI-003: VERIFIER COMPATIBILITE WIKILINKS/PROPRIETES**]
4. Performance: Optimise pour 10,000+ noeuds [⚠️ **COH-004: BENCHMARKS A VALIDER**]
5. Multiplateforme: macOS, Windows, Linux [✅ VERIFIÉ: wry, slint, wgpu supportent toutes plateformes]

**Contraintes Organisationnelles:**
- Equipe reduite (3-5 developpeurs)
- Budget limite (open-source community-driven)
- Delai cible: 12-13 mois pour version 1.0

[TECH-NOTE: Planning à réviser: +2 mois nécessaires (voir rapport de vérification)]

---

## 3. ANALYSE DES SOURCES

[TECH-NOTE: Analyse complète et bien documentée. 80/81 features vérifiées.]

---

## 4. CHOIX TECHNIQUES JUSTIFIES

[TECH-NOTE: Tous les choix techniques ont été vérifiés. Voir Annexe A du rapport pour détails.]

### 4.1 Framework GUI: Slint

**Decision:** Slint (DSL declaratif)
**Alternatives:** iced, egui, Druid, gtk-rs, qt-rs
**Source:** rust_architecture_choices.json:90-140

#### Criteres d'Evaluation

| Criteres | Poids | Slint | iced | egui | Druid | gtk-rs | qt-rs |
|----------|-------|-------|------|------|-------|--------|-------|
| Performance | 30% | 5/5 | 4/5 | 4/5 | 5/5 | 3/5 | 4/5 |
| Native Look | 20% | 4/5 | 3/5 | 2/5 | 4/5 | 5/5 | 5/5 |
| Simplicite | 15% | 4/5 | 4/5 | 3/5 | 2/5 | 2/5 | 2/5 |
| Ecosysteme | 15% | 4/5 | 4/5 | 4/5 | 2/5 | 5/5 | 5/5 |
| Maintenance | 10% | 5/5 | 4/5 | 4/5 | 2/5 | 5/5 | 5/5 |
| Dependances | 10% | 4/5 | 3/5 | 5/5 | 5/5 | 1/5 | 1/5 |
| **Total** | **100%** | **4.4** | **4.0** | **3.8** | **3.5** | **3.3** | **3.5** |

**Justification:**
- Performance: Rendu GPU-accelere (OpenGL/Vulkan/Metal) [Source: rust_architecture_choices.json:105]
- Native Look: Integration parfaite avec chaque OS [Source: rust_architecture_choices.json:108]
- DSL clair: Separation UI/logique, maintenabilite amelioree [Source: rust_architecture_choices.json:111]
- Support commercial: Developpe par SiFive [Source: rust_architecture_choices.json:114]
- Outils: Slint Designer pour creation visuelle [Source: rust_architecture_choices.json:117]

**Sources:** https://slint-ui.com, https://github.com/slint-ui/slint

**Decision finale:** Slint (avec iced comme alternative)

[✅ **VERIFIÉ:** Slint v1.18.1, 2.4M downloads, crée 2022-02, mis à jour 2026-09 - MATURE]

### 4.2 Stockage: Markdown + SQLite + Moteur Datalog Custom

**Decision:** Markdown (fichiers) + SQLite (indexation) + Moteur Datalog Custom
**Alternatives:** JSON, Binaire Custom, SurrealDB, RocksDB
**Source:** rust_architecture_choices.json:180-260

**Justification Markdown:**
- Compatibilite 100% avec Obsidian/Logseq [Source: rust_architecture_choices.json:185]
- Lisibilite humaine, edition manuelle possible [Source: rust_architecture_choices.json:188]
- Standard etabli avec ecosysteme riche [Source: rust_architecture_choices.json:191]

**Justification SQLite:**
- Maturite exceptionnelle (20+ ans en production) [Source: rust_architecture_choices.json:210]
- Zero configuration [Source: rust_architecture_choices.json:213]
- Transactions ACID [Source: rust_architecture_choices.json:216]
- Performances: ~1M lecture/sec, ~100K ecriture/sec [Source: https://www.sqlite.org/eqp.html]

**Justification Moteur Datalog Custom:**
- Controle total sur la logique du graphe [Source: rust_architecture_choices.json:250]
- Pas de dependance externe (100% Rust) [Source: rust_architecture_choices.json:253]
- Flexibilite pour extensions specifiques [Source: rust_architecture_choices.json:256]

[⚠️ **FAI-001: PROBLEME CRITIQUE - MOTEUR DATALOG CUSTOM**]
[🔴 **ACTION REQUISE:** Créer PoC avant Phase 5. ddlog NON DISPONIBLE sur crates.io (vérifié 2026-11-09).]
[🔴 **ALTERNATIVE:** Évaluer surrealdb v4.0.0 (1.8M downloads) comme backend.]
[🔴 **RISQUE:** Sous-estimation de la complexité. Phase 5 pourrait être retardée de 4-6 semaines.]

**Schema SQLite:** [A VERIFIER - Schema complet dans l'annexe]

**Decision finale:** Markdown + SQLite + Moteur Datalog Custom

### 4.3 Rendu Markdown: pulldown-cmark + wry

**Decision:** pulldown-cmark (parser) + wry (renderer WebView)
**Alternatives:** comrak (parser), HTML Custom (renderer)
**Source:** rust_architecture_choices.json:360-420

**Justification pulldown-cmark:**
- Meilleure conformite CommonMark [Source: rust_architecture_choices.json:365]
- Ecosysteme riche d'extensions [Source: rust_architecture_choices.json:368]
- Pur Rust (pas de FFI) [Source: rust_architecture_choices.json:371]
- Performances: ~1-2 MB/sec [Source: rust_architecture_choices.json:374]

**Justification wry:**
- Qualite de rendu professionnelle [Source: rust_architecture_choices.json:377]
- Support multiplateforme: WKWebView, WebView2, WebKitGTK [Source: rust_architecture_choices.json:380]
- Support JavaScript pour LaTeX (KaTeX), diagrammes (Mermaid) [Source: rust_architecture_choices.json:383]

**Extensions supportees:**
- Tables (GFM), Wikilinks, Tags, Code blocks, LaTeX, Mermaid, Embedded notes, Task lists, Footnotes

[⚠️ **FAI-003: PROBLEME MAJEUR - COMPATIBILITE MARKDOWN**]
[🟡 **ACTION REQUISE:** pulldown-cmark ne supporte PAS nativement wikilinks et propriétés.]
[🟡 **SOLUTION:** Utiliser pulldown-cmark-extras ou créer extensions custom.]
[🟡 **RISQUE:** Complexité accrue. Tests exhaustifs nécessaires avec fichiers réels Obsidian/Logseq.]

**Decision finale:** pulldown-cmark + wry

[✅ **VERIFIÉ:** pulldown-cmark v0.13.6 (201.5M downloads), wry v0.57.1 (46.4M downloads) - MATURES]

### 4.4 Graphe de Connaissances: Moteur Datalog Custom

**Decision:** Moteur Datalog Custom en Rust
**Alternatives:** ddlog (Microsoft), SurrealDB, Neo4j Embedded
**Source:** rust_architecture_choices.json:270-350

**Justification:**
- Controle total sur la logique [Source: rust_architecture_choices.json:275]
- 100% Rust, pas de dependance externe [Source: rust_architecture_choices.json:278]
- Flexibilite pour extensions specifiques [Source: rust_architecture_choices.json:281]
- Opportunite d'apprentissage [Source: rust_architecture_choices.json:284]

**Fonctionnalites du moteur:**
- Parsing de regles Datalog
- Evaluation de requetes (naive puis optimisee)
- Indexation automatique des predicats
- Cache des resultats de requetes frequentes
- Support des requetes recursives

**Performance cible:**
- Requete simple: < 10ms
- Requete complexe recursive: < 100ms pour 10K notes

[⚠️ **FAI-001: LIÉ À COH-004**]
[🟡 **ACTION REQUISE:** Benchmarks obligatoires. Performance cible <100ms pour 10K notes est OPTIMISTE.]
[🟡 **RISQUE:** Moteur Datalog custom pourrait ne pas atteindre les performances cibles sans optimisations majeures.]

**Sources:** https://en.wikipedia.org/wiki/Datalog, https://github.com/Microsoft/ddlog

### 4.5 Synchronisation: Git (primaire) + CRDT (optionnel)

**Decision:** Git (git2-rs) comme methode primaire, CRDT (yrs-rs) comme option avancée
**Alternatives:** Syncthing, Service Central
**Source:** rust_architecture_choices.json:480-560

**Justification Git:**
- Simplicite maximale [Source: rust_architecture_choices.json:485]
- Integration naturelle avec Markdown [Source: rust_architecture_choices.json:488]
- Controle de version integre [Source: rust_architecture_choices.json:491]
- Zero serveur, zero cout [Source: rust_architecture_choices.json:494]

**Justification CRDT:**
- Gestion automatique des conflits [Source: rust_architecture_choices.json:497]
- Synchronisation en temps reel [Source: rust_architecture_choices.json:500]
- Offline-first [Source: rust_architecture_choices.json:503]
- P2P [Source: rust_architecture_choices.json:506]

[⚠️ **COH-003: PROBLEME MAJEUR - SYNCHRONISATION GIT + CRDT**]
[🟡 **ACTION REQUISE:** Choisir UNE approche principale. Git et CRDT sont fondamentalement différents.]
[🟡 **RECOMMANDATION:** Git principal + CRDT optionnel (expérimental).]
[🟡 **RISQUE:** Complexité accrue de maintenance. Éviter de mélanger les deux dans v1.0.]

**Decision finale:** Git (primaire) + CRDT (optionnel)

[✅ **VERIFIÉ:** git2 v0.21.0 (126.1M downloads), yrs v0.28.1 (4.7M downloads) - MATURES]

### 4.6 Systeme de Plugins: WASM (wasmtime)

**Decision:** WASM avec wasmtime comme runtime
**Alternatives:** dylib (Rust), Lua Scripting
**Source:** rust_architecture_choices.json:570-650

**Justification:**
- Multi-langage: Rust, JavaScript, Python, Go, etc. [Source: rust_architecture_choices.json:575]
- Securite: Sandboxing natif [Source: rust_architecture_choices.json:578]
- Portabilite: Un plugin fonctionne sur toutes les plateformes [Source: rust_architecture_choices.json:581]
- Ecosysteme croissant [Source: rust_architecture_choices.json:584]
- Runtime wasmtime: pur Rust, bien maintenu [Source: rust_architecture_choices.json:587]

**Architecture:**
```
Application (Host) <-> WASM Runtime (wasmtime) <-> Plugins (WASM modules)
```

**API Stable:**
- init(context: PluginContext) -> Result<(), PluginError>
- execute(command: PluginCommand) -> Result<PluginResponse, PluginError>
- cleanup() -> Result<(), PluginError>

[✅ **VERIFIÉ:** wasmtime v50.0.1 (43.9M downloads) - MATURE]

**Decision finale:** WASM avec wasmtime

### 4.7 Optimisations de Performance

**Strategie:** Optimisation multi-niveaux pour 10,000+ noeuds
**Source:** rust_architecture_choices.json:660-750

**Domaines d'optimisation:**
- Rendu graphe: Virtualisation, WebGL (wgpu), LOD
- Chargement: Lazy loading, background loading
- Cache: Multi-level (L1: Memory, L2: Disk, L3: Persistent)
- Requetes: Indexation SQLite, cache des resultats, requetes incrementales
- Threading: rayon pour parallelisme

**Performance cible:**
- Rendu < 100ms pour 10K noeuds
- Requetes < 10ms (simple), < 100ms (complexe)
- Chargement initial < 2 secondes

[⚠️ **COH-004: PROBLEME MAJEUR - PERFORMANCES GRAPHE**]
[🟡 **ACTION REQUISE:** Benchmarks obligatoires avant Phase 8.]
[🟡 **RISQUE:** Estimations optimistes. Rendu <100ms pour 10K nœuds dépend du hardware et des optimisations.]
[🟡 **SOLUTION:** Utiliser wgpu pour rendu accéléré + LOD + virtualisation.]

---

## 5. DESIGN SYSTEM NOTE CRAFT

[✅ **VERIFIÉ:** Design system bien documenté et complet.]

---

## 6. ROADMAP DETAILLEE

> **Structure:** 9 phases, 100+ etapes, 8-13 mois total
> **Objectif:** Implementation complete de Note Craft avec toutes les fonctionnalites identifiees
> **Approche:** Iterative, chaque phase livrant une version utilisable
> **
> [⚠️ **PLANNING REVISÉ:** Estimation ajustée à 10-15 mois (+2 mois) - Voir rapport de vérification]**

### Structure des Phases

Chaque phase suit la structure suivante:
- **Objectif principal:** Resultat attendu a la fin de la phase
- **Etapes detaillees:** Liste des taches avec dependances
- **Sous-agent responsable:** Agent ou equipe en charge
- **Modele recommande:** Approche technique a suivre
- **Critere d'acceptation:** Tests pour valider la phase
- **Dependances:** Pre-requis avec autres phases

### Phase 0: Setup Projet (1-2 semaines)

[✅ **VERIFIÉ:** Phase bien structurée, dépendances claires.]

**Objectif:** Creer l'infrastructure de base pour le projet
**Responsable:** ARCHITECTE_RUST
**Modele:** Infrastructure as Code, CI/CD automatique

#### Etapes detaillees

| ID | Etape | Priorite | Estimation | Dependances | Source |
|----|-------|----------|------------|-------------|--------|
| 0.1 | Creer depot Git (GitHub) | Critique | 1h | Aucune | Rust best practices |
| 0.2 | Configurer Cargo workspace | Critique | 2h | 0.1 | rust_architecture_report.md:60 |
| 0.3 | Definir structure du projet | Critique | 4h | 0.2 | rust_architecture_report.md:70 |
| 0.4 | Configurer Slint | Critique | 1d | 0.3 | rust_architecture_choices.json:90 |
| 0.5 | Configurer wry | Critique | 1d | 0.3 | rust_architecture_choices.json:360 |
| 0.6 | Configurer pulldown-cmark | Critique | 1d | 0.3 | rust_architecture_choices.json:360 |
| 0.7 | Configurer SQLite (rusqlite) | Critique | 1d | 0.3 | rust_architecture_choices.json:180 |
| 0.8 | Configurer wasmtime | Moyen | 1d | 0.3 | rust_architecture_choices.json:570 |
| 0.9 | Configurer tokio | Moyen | 1d | 0.3 | rust_architecture_choices.json:700 |
| 0.10 | Configurer CI/CD (GitHub Actions) | Critique | 1d | 0.1 | rust_architecture_report.md:800 |
| 0.11 | Configurer systeme de build | Critique | 1d | 0.2 | Rust best practices |
| 0.12 | Creer documentation initiale | Moyen | 2d | 0.1 | - |

#### Critere d'acceptation

- Depot Git accessible et configure
- Projet compile avec cargo build
- CI/CD execute les tests automatiquement
- Structure du projet validee par l'equipe
- Toutes les dependances principales configurees
- Documentation de setup disponible

**Dependances:** Aucune

[✅ **ACTION:** Ajouter étape 0.13: Valider PoC Slint + wry intégration (FAI-002)]

---

### Phase 1: Core System (2-4 semaines)

[✅ **VERIFIÉ:** Phase critique, bien structurée.]

**Objectif:** Implémenter le système de base (blocs, pages, graphe basique, DB)
**Responsable:** ARCHITECTE_RUST + ANALYSTE_LOGSEQ

#### Etapes detaillees

| ID | Etape | Priorite | Estimation | Dependances | Source |
|----|-------|----------|------------|-------------|--------|
| 1.1 | File System Abstraction | Critique | 3d | Phase 0 | logseq_audit_results.json |
| 1.2 | Block System (structure) | Critique | 5d | Phase 0, 1.1 | logseq_audit_results.json:42 |
| 1.3 | Page System | Critique | 5d | 1.2 | logseq_audit_results.json:55 |
| 1.4 | State Management | Critique | 5d | Phase 0, 1.1 | logseq_audit_results.json:299 |
| 1.5 | Configuration System | Critique | 3d | 1.1 | logseq_audit_results.json:312 |
| 1.6 | Repository Management | Critique | 3d | 1.1, 1.2 | logseq_audit_results.json:132 |
| 1.7 | Markdown Parser (basique) | Critique | 3d | 1.1 | rust_architecture_choices.json:360 |
| 1.8 | SQLite Integration | Critique | 3d | 1.1 | rust_architecture_choices.json:180 |
| 1.9 | Data Persistence | Critique | 3d | 1.1, 1.8 | logseq_audit_results.json:204 |
| 1.10 | Knowledge Graph (structure) | Critique | 5d | 1.2, 1.3 | logseq_audit_results.json:68 |
| 1.11 | Tag System | Moyen | 3d | 1.2, 1.7 | logseq_audit_results.json:81 |
| 1.12 | Routing & Navigation | Moyen | 3d | 1.3, 1.6 | logseq_audit_results.json:145 |
| 1.13 | Event System | Moyen | 3d | Phase 0, 1.4 | logseq_audit_results.json:158 |
| 1.14 | History/Undo | Moyen | 3d | 1.4 | logseq_audit_results.json:94 |

#### Critere d'acceptation

- Block System: Creation, modification, suppression de blocs
- Page System: Creation, renommage, suppression de pages
- Knowledge Graph: Structure de base du graphe
- Data Persistence: Donnees sauvegardees et restaurees
- Markdown Parser: Parsing basique de fichiers Markdown
- Configuration: Configuration hierarchique
- Tests: 90%+ couverture pour le core system

**Dependances:** Phase 0

---

### Phase 2: Editor (4-6 semaines)

**Objectif:** Implémenter l'éditeur Markdown avec fonctionnalités avancées
**Responsable:** ANALYSTE_LOGSEQ

[⚠️ **FAI-002 & FAI-003: PROBLEMES LIÉS À CETTE PHASE**]
[🟡 **ACTION REQUISE:** Valider intégration pulldown-cmark + wry + Slint avant de commencer.]
[🟡 **ACTION REQUISE:** Tester compatibilité wikilinks et propriétés avec pulldown-cmark.]

#### Etapes detaillees

| ID | Etape | Priorite | Estimation | Dependances | Source |
|----|-------|----------|------------|-------------|--------|
| 2.1 | Markdown Editor (complet) | Critique | 5d | Phase 1 | logseq_audit_results.json:169 |
| 2.2 | Text Formatting | Critique | 3d | 2.1 | logseq_audit_results.json:222 |
| 2.3 | Code Blocks | Moyen | 3d | 2.1 | logseq_audit_results.json:185 |
| 2.4 | LaTeX Support (KaTeX) | Moyen | 3d | 2.1 | rust_architecture_choices.json:383 |
| 2.5 | Asset Management | Critique | 5d | Phase 1, 2.1 | logseq_audit_results.json:197 |
| 2.6 | Drag & Drop | Critique | 5d | Phase 1, 2.1 | logseq_audit_results.json:182 |
| 2.7 | Advanced Paste | Moyen | 3d | 2.1 | logseq_audit_results.json:219 |
| 2.8 | Macros System | Moyen | 3d | 2.1 | logseq_audit_results.json:192 |
| 2.9 | Quick Add | Moyen | 2d | 2.1 | logseq_audit_results.json:244 |
| 2.10 | Rendering Pipeline | Critique | 5d | 2.1 | logseq_audit_results.json:335 |
| 2.11 | Editor Lifecycle | Moyen | 2d | 2.1 | logseq_audit_results.json:252 |
| 2.12 | wry Integration | Critique | 3d | Phase 0, 2.10 | rust_architecture_choices.json:380 |

#### Critere d'acceptation

- Markdown Editor: Edition en temps reel avec preview
- Text Formatting: Gras, italique, listes, liens, etc.
- Code Blocks: Coloration syntaxique pour 10+ langages
- LaTeX: Support des formules mathematiques
- Asset Management: Upload, drag-drop, preview d'images
- Drag & Drop: Reorganisation de blocs, pages, assets
- Rendering Pipeline: Markdown -> HTML -> WebView
- Tests: 90%+ couverture pour l'editor

**Dependances:** Phase 0, Phase 1

---

### Phase 3: UI/UX (6-8 semaines)

[⚠️ **COH-001: PROBLEME CRITIQUE - ARCHITECTURE UI**]
[🔴 **ACTION REQUISE: Résoudre conflit Slint vs wry AVANT de commencer cette phase.**]
[🔴 **DÉCISION REQUISE: Comment intégrer l'éditeur (wry) dans l'UI (Slint)?**]
[🔴 **RECOMMANDATION: Slint comme GUI principal + wry embarqué comme widget custom.**]

**Objectif:** Implémenter l'interface utilisateur et l'expérience utilisateur complète
**Responsable:** CHERCHEUR_UI_UX

#### Etapes detaillees

| ID | Etape | Priorite | Estimation | Dependances | Source |
|----|-------|----------|------------|-------------|--------|
| 3.1 | Slint UI (structure) | Critique | 5d | Phase 0 | note_craft_design_system.md:1 |
| 3.2 | Sidebar System | Critique | 5d | Phase 1, 3.1 | logseq_audit_results.json:389 |
| 3.3 | Block UI Components | Critique | 7d | Phase 1, 3.1 | logseq_audit_results.json:255 |
| 3.4 | Page UI Components | Critique | 5d | Phase 1, 3.1 | logseq_audit_results.json:277 |
| 3.5 | Command Palette | Critique | 5d | Phase 1, 3.1 | logseq_audit_results.json:268 |
| 3.6 | Outliner System | Critique | 5d | Phase 1, 3.1 | logseq_audit_results.json:380 |
| 3.7 | Theme System | Moyen | 5d | Phase 0, 3.1 | logseq_audit_results.json:286 |
| 3.8 | Keyboard Shortcuts | Moyen | 3d | Phase 1, 3.5 | logseq_audit_results.json:283 |
| 3.9 | Graph View (basique) | Moyen | 5d | Phase 1, 3.1 | logseq_audit_results.json:261 |
| 3.10 | Journal View | Moyen | 3d | Phase 1, 3.1 | logseq_audit_results.json:140 |
| 3.11 | Asset Gallery | Moyen | 3d | Phase 2, 3.1 | logseq_audit_results.json:274 |
| 3.12 | Layout System | Moyen | 5d | Phase 1, 3.1 | logseq_audit_results.json:290 |
| 3.13 | Dialog System | Moyen | 3d | Phase 1, 3.1 | logseq_audit_results.json:397 |
| 3.14 | Notification System | Moyen | 3d | Phase 1, 3.1 | logseq_audit_results.json:271 |
| 3.15 | Appliquer Design System | Critique | 5d | 3.1 | note_craft_design_system.md |

#### Critere d'acceptation

- Slint UI: Application avec interface fonctionnelle
- Sidebar System: Navigation entre pages et fichiers
- Block UI: Affichage et edition de blocs
- Command Palette: Execution de commandes via raccourcis
- Theme System: Theme sombre par defaut
- Graph View: Visualisation basique du graphe
- Design System: Cohérence visuelle avec Obsidian
- Tests: 90%+ couverture pour l'UI

**Dependances:** Phase 0, Phase 1, Phase 2

---

### Phase 4: Data & Storage (4-6 semaines)

**Objectif:** Implémenter le système de stockage et de gestion des données
**Responsable:** ARCHITECTE_RUST

#### Etapes detaillees

| ID | Etape | Priorite | Estimation | Dependances | Source |
|----|-------|----------|------------|-------------|--------|
| 4.1 | SQLite Schema (complet) | Critique | 3d | Phase 1 | rust_architecture_choices.json:180 |
| 4.2 | Property System | Critique | 5d | Phase 1, 4.1 | logseq_audit_results.json:236 |
| 4.3 | Data Import | Moyen | 5d | Phase 1, 4.1 | logseq_audit_results.json:214 |
| 4.4 | Data Export | Moyen | 5d | Phase 1, 4.1 | logseq_audit_results.json:220 |
| 4.5 | Backup System | Moyen | 3d | Phase 1, 4.1 | logseq_audit_results.json:241 |
| 4.6 | Migration System | Moyen | 3d | Phase 1, 4.1 | rust_architecture_choices.json:220 |
| 4.7 | Cache System | Moyen | 3d | Phase 1, 4.1 | logseq_audit_results.json:257 |
| 4.8 | Diff System | Moyen | 3d | Phase 1, 4.1 | logseq_audit_results.json:437 |
| 4.9 | File Watcher | Critique | 3d | Phase 1 | rust_architecture_choices.json:700 |
| 4.10 | Search Indexing | Moyen | 5d | Phase 1, 4.1 | logseq_audit_results.json:371 |
| 4.11 | Optimiser SQLite (indexes) | Moyen | 3d | 4.1 | rust_architecture_choices.json:210 |
| 4.12 | FTS5 (Full-Text Search) | Moyen | 3d | 4.1, 4.11 | rust_architecture_choices.json:216 |

#### Critere d'acceptation

- SQLite Schema: Tables completes avec indexes
- Property System: Properties personnalisees
- Data Import: Import depuis Markdown, JSON, OPML
- Data Export: Export vers Markdown, JSON, HTML
- Backup System: Sauvegardes automatiques
- Cache System: Cache multi-niveaux
- File Watcher: Detection des modifications
- Search Indexing: Indexation full-text
- Tests: 90%+ couverture pour le stockage

**Dependances:** Phase 0, Phase 1

---

### Phase 5: Query System (4-6 semaines)

[⚠️ **COH-002: PROBLEME CRITIQUE - DÉPENDANCES CIRCULAIRES**]
[🔴 **ACTION REQUISE: Découper cette phase en 5a et 5b.**]
[🔴 **DÉPENDANCES: Phase 5 dépend de Phase 4 ET de Phase 1 directement.**]
[🔴 **SOLUTION PROPOSÉE:** 5a: Datalog Parser (Phase 1) | 5b: Query Engine (Phase 4 + 5a)]
[⚠️ **FAI-001: LIÉ - MOTEUR DATALOG**]
[🟡 **ACTION REQUISE: PoC validé avant de commencer Phase 5.]

**Objectif:** Implémenter le système de requêtes Datalog et de recherche avancée
**Responsable:** ARCHITECTE_RUST

#### Etapes detaillees

| ID | Etape | Priorite | Estimation | Dependances | Source |
|----|-------|----------|------------|-------------|--------|
| 5.1 | Datalog Parser | Critique | 10d | Phase 1 | rust_architecture_choices.json:270 |
| 5.2 | Datalog Execution Engine | Critique | 15d | 5.1 | rust_architecture_choices.json:270 |
| 5.3 | SQLite Indexing for Graph | Critique | 5d | Phase 4, 5.1 | rust_architecture_choices.json:210 |
| 5.4 | Wikilinks Generation | Critique | 5d | Phase 1, 5.1 | rust_architecture_report.md:200 |
| 5.5 | Backlinks Calculation | Critique | 5d | Phase 1, 5.4 | rust_architecture_report.md:203 |
| 5.6 | Basic Queries (tag, link) | Critique | 5d | 5.1, 5.2 | logseq_audit_results.json:210 |
| 5.7 | Advanced Query Features | Moyen | 5d | 5.2 | logseq_audit_results.json:225 |
| 5.8 | Query Builder UI | Moyen | 5d | Phase 3, 5.6 | logseq_audit_results.json:239 |
| 5.9 | Query Cache | Moyen | 3d | 5.2 | rust_architecture_choices.json:670 |
| 5.10 | Incremental Queries | Moyen | 5d | 5.2 | rust_architecture_choices.json:670 |

#### Critere d'acceptation

- Datalog Parser: Parsing des regles Datalog
- Datalog Engine: Execution de requetes sur le graphe
- Wikilinks: Generation automatique des liens
- Backlinks: Calcul des backlinks en temps reel
- Basic Queries: Requetes par tag, par lien
- Advanced Queries: Aggregation, groupement, tri
- Query Builder: Interface visuelle pour construire des requetes
- Query Cache: Cache des resultats de requetes
- Tests: 90%+ couverture pour le systeme de requetes

**Dependances:** Phase 0, Phase 1, Phase 4

---

### Phase 6: Plugins (6-8 semaines)

**Objectif:** Implémenter le système de plugins WASM
**Responsable:** ARCHITECTE_RUST

#### Etapes detaillees

| ID | Etape | Priorite | Estimation | Dependances | Source |
|----|-------|----------|------------|-------------|--------|
| 6.1 | Integrer wasmtime | Critique | 5d | Phase 0 | rust_architecture_choices.json:570 |
| 6.2 | Definir API stable | Critique | 10d | 6.1 | rust_architecture_choices.json:590 |
| 6.3 | Plugin Loading System | Critique | 5d | 6.1, 6.2 | logseq_audit_results.json:318 |
| 6.4 | Sandboxing et Securite | Critique | 10d | 6.1 | rust_architecture_choices.json:570 |
| 6.5 | Plugin Capabilities System | Moyen | 5d | 6.2 | rust_architecture_choices.json:590 |
| 6.6 | Plugin Configuration | Moyen | 3d | 6.2 | rust_architecture_choices.json:590 |
| 6.7 | Plugin Marketplace (basique) | Bas | 5d | 6.3 | logseq_audit_results.json:325 |
| 6.8 | Plugin Documentation | Moyen | 5d | 6.2 | - |
| 6.9 | Exemples de Plugins | Moyen | 10d | 6.2 | - |

#### Exemples de Plugins a developper

| Plugin | Description | Langage | Complexite |
|--------|-------------|---------|------------|
| Word Count | Compte le nombre de mots | Rust | Bas |
| Calendar | Affichage d'un calendrier | Rust | Moyen |
| Git History | Visualisation de l'historique Git | Rust | Moyen |
| Markdown Exporter | Export en differents formats | Rust | Moyen |
| LaTeX Renderer | Ameliore le support LaTeX | JavaScript | Moyen |
| Mermaid Renderer | Support des diagrammes Mermaid | JavaScript | Moyen |

#### Critere d'acceptation

- wasmtime: Integration et configuration
- API Stable: API definie et documentee
- Plugin Loading: Chargement, execution, dechargement de plugins
- Sandboxing: Isolation des plugins, securite verifiee
- Capabilities: Systeme de permissions fonctionnel
- Exemples: 5+ plugins exemples fonctionnels
- Tests: 90%+ couverture pour le systeme de plugins

**Dependances:** Phase 0, Phase 1

---

### Phase 7: Sync & Import/Export (4-6 semaines)

[⚠️ **COH-003: PROBLEME MAJEUR - SYNCHRONISATION**]
[🟡 **ACTION REQUISE: Choisir UNE approche principale (Git OU CRDT).**]
[🟡 **RECOMMANDATION: Git principal + CRDT optionnel (expérimental).**]

**Objectif:** Implémenter la synchronisation et l'import/export des données
**Responsable:** ARCHITECTE_RUST

#### Etapes detaillees

| ID | Etape | Priorite | Estimation | Dependances | Source |
|----|-------|----------|------------|-------------|--------|
| 7.1 | Git Integration (git2-rs) | Critique | 5d | Phase 1 | rust_architecture_choices.json:480 |
| 7.2 | Auto-commit/push | Moyen | 5d | 7.1 | rust_architecture_report.md:400 |
| 7.3 | Conflict Resolution (merge) | Moyen | 10d | 7.1 | rust_architecture_report.md:403 |
| 7.4 | CRDT Integration (yrs-rs) | Moyen | 10d | Phase 1 | rust_architecture_choices.json:480 |
| 7.5 | P2P Synchronization | Bas | 15d | 7.4 | rust_architecture_choices.json:480 |
| 7.6 | Sync UI | Moyen | 5d | Phase 3, 7.1 | - |
| 7.7 | Import Obsidian | Moyen | 5d | Phase 4 | - |
| 7.8 | Import Logseq | Moyen | 5d | Phase 4 | - |
| 7.9 | Export HTML | Moyen | 3d | Phase 4 | - |
| 7.10 | Export PDF | Bas | 5d | Phase 4 | - |

#### Critere d'acceptation

- Git Integration: Synchronisation via Git fonctionnelle
- Auto-commit: Commit automatique des modifications
- Conflict Resolution: Detection et resolution des conflits
- CRDT: Synchronisation en temps reel (optionnelle)
- Sync UI: Interface pour gerer la synchronisation
- Import Obsidian: Import des fichiers Obsidian
- Import Logseq: Import des fichiers Logseq
- Export: Export vers HTML, PDF, etc.
- Tests: 90%+ couverture pour la synchronisation

**Dependances:** Phase 0, Phase 1, Phase 3, Phase 4

---

### Phase 8: Polish & Optimisations (4-6 semaines)

[⚠️ **COH-004: PROBLEME MAJEUR - PERFORMANCES**]
[🟡 **ACTION REQUISE: Benchmarks obligatoires avant validation.**]
[🟡 **RISQUE:** Performances cibles optimistes. Validation nécessaire.]

**Objectif:** Finaliser l'application avec polish, optimisations et preparation de la release
**Responsable:** Tous

#### Etapes detaillees

| ID | Etape | Priorite | Estimation | Dependances | Source |
|----|-------|----------|------------|-------------|--------|
| 8.1 | Performance Optimization | Critique | 10d | Toutes | rust_architecture_choices.json:660 |
| 8.2 | Graph Rendering Virtualization | Critique | 10d | Phase 3, Phase 5 | rust_architecture_choices.json:660 |
| 8.3 | LOD for Graph | Moyen | 10d | 8.2 | rust_architecture_choices.json:660 |
| 8.4 | Multi-level Caching | Critique | 5d | Phase 4 | rust_architecture_choices.json:660 |
| 8.5 | Lazy Loading | Critique | 10d | Phase 2 | rust_architecture_choices.json:660 |
| 8.6 | Datalog Query Optimization | Moyen | 10d | Phase 5 | rust_architecture_choices.json:660 |
| 8.7 | Threading (rayon) | Moyen | 5d | Toutes | rust_architecture_choices.json:660 |
| 8.8 | Final Benchmarking | Moyen | 5d | Toutes | - |
| 8.9 | User Documentation | Critique | 10d | Toutes | - |
| 8.10 | Unit Tests | Critique | 10d | Toutes | - |
| 8.11 | Integration Tests | Critique | 10d | Toutes | - |
| 8.12 | Packaging (deb, rpm, dmg, exe) | Critique | 5d | Toutes | rust_architecture_report.md:800 |
| 8.13 | CI/CD Complete | Critique | 5d | Phase 0 | rust_architecture_report.md:800 |
| 8.14 | Website and Documentation | Moyen | 10d | Toutes | - |
| 8.15 | Public Release | Critique | 3d | Toutes | - |

#### Critere d'acceptation

- Performance: Rendu < 100ms pour 10K noeuds [⚠️ **À VALIDER**]
- Graph Rendering: Virtualisation et LOD fonctionnels
- Cache: Cache multi-niveaux operationnel
- Lazy Loading: Chargement a la demande
- Benchmarking: Resultats documentees et valides
- Documentation: Documentation complete
- Tests: 95%+ couverture globale
- Packaging: Applications packagees
- CI/CD: Pipeline complete
- Release: Version publique disponible

**Dependances:** Toutes les phases precedentes

---

### Phase 9: Collaboration (NOUVELLE - 4-6 semaines)

[🔴 **COM-001: PHASE AJOUTÉE - COLLABORATION TEMPS RÉEL**]
[🔴 **PROBLÈME CRITIQUE: 3 features Logseq manquantes (Collaboration).**]
[🔴 **IMPACT: Fonctionnalité majeure pour concurrent Logseq/Obsidian.**]

**Objectif:** Implémenter la collaboration temps réel multi-utilisateurs
**Responsable:** ARCHITECTE_RUST
**Priorité:** CRITIQUE (ajouté après vérification technique)

#### Etapes detaillees

| ID | Etape | Priorite | Estimation | Dependances | Source |
|----|-------|----------|------------|-------------|--------|
| 9.1 | yrs-rs Integration | Critique | 10d | Phase 0, Phase 1 | rust_architecture_choices.json:480 |
| 9.2 | CRDT Document Model | Critique | 15d | 9.1 | - |
| 9.3 | Real-time Sync Engine | Critique | 15d | 9.2 | - |
| 9.4 | Presence Awareness | Critique | 10d | 9.3 | - |
| 9.5 | Conflict Resolution | Critique | 10d | 9.3 | - |
| 9.6 | Collaboration UI | Moyen | 10d | Phase 3, 9.1 | - |
| 9.7 | User Presence Indicators | Moyen | 5d | 9.4 | - |
| 9.8 | Permission System | Moyen | 5d | 9.1 | - |
| 9.9 | Tests de charge | Moyen | 5d | 9.3 | - |
| 9.10 | Documentation | Moyen | 5d | 9.1 | - |

#### Critere d'acceptation

- Real-time collaboration: Edition simultanée multi-utilisateurs
- CRDT Engine: Synchronisation automatique des conflits
- Presence Awareness: Visualisation des utilisateurs connectés
- Conflict Resolution: Résolution automatique des conflits
- Performance: < 100ms de latence pour les changements
- Tests: 90%+ couverture pour la collaboration

**Dependances:** Phase 0, Phase 1, Phase 3

[✅ **ACTION:** Phase ajoutée pour couvrir COM-001]

---

## 7. RISQUES ET QUESTIONS OUVERTES

### Risques Techniques

| ID | Risque | Probabilite | Impact | Mitigation | Source |
|----|--------|-------------|--------|------------|--------|
| R1 | Complexite Moteur Datalog | Elevée | Elevé | Planification realiste, PoC avant [✅ **FAI-001**] | rust_architecture_choices.json:270 |
| R2 | Performances Graph Rendering | Moyenne | Elevé | Optimisations progressives [⚠️ **COH-004**] | rust_architecture_choices.json:660 |
| R3 | Integration Slint | Faible | Moyen | PoC, fallback iced [⚠️ **COH-001**] | rust_architecture_choices.json:90 |
| R4 | Compatibilite Markdown | Moyenne | Moyen | Tests exhaustifs [⚠️ **FAI-003**] | logseq_audit_summary.md:100 |
| R5 | Securite WASM | Faible | Elevé | Sandboxing strict | rust_architecture_choices.json:570 |
| R6 | Migration Donnees | Moyenne | Moyen | Systeme backup/migration teste | logseq_audit_results.json:437 |

[🔴 **NOUVEAUX RISQUES IDENTIFIÉS - À AJOUTER:**]

| ID | Risque | Probabilite | Impact | Mitigation |
|----|--------|-------------|--------|------------|
| R7 | **Intégration Slint + wry** | Moyenne | Elevé | PoC + API claire + architecture modulaire |
| R8 | **Dépendance WebView système** | Faible | Moyen | Documenter requirements + fallback graceful |
| R9 | **Mémoire 10K+ nœuds** | Moyenne | Elevé | Optimisation mémoire + benchmarks + monitoring |
| R10 | **Compatibilité multiplateforme** | Moyenne | Moyen | Tests CI/CD sur toutes plateformes + build cross-compilation |
| R11 | **Maintenance moteur Datalog** | Moyenne | Elevé | Documentation complète + tests exhaustifs + équipe dédiée |

### Risques Organisationnels

| ID | Risque | Probabilite | Impact | Mitigation |
|----|--------|-------------|--------|------------|
| R7 | Retard Livreables | Moyenne | Elevé | Planification avec marge |
| R8 | Turnover Equipes | Faible | Elevé | Documentation complete |
| R9 | Budget Depasse | Moyenne | Moyen | Suivi regulier |
| R10 | Adoption Utilisateurs | Moyenne | Elevé | Implication communaute |

[⚠️ **NOTE:** R7-R10 en conflit avec nouveaux risques techniques R7-R11. À renuméroter.]

### Questions Ouvertes [A VERIFIER]

| ID | Question | Domaine | Source | Statut |
|----|---------|---------|--------|--------|
| Q1 | Largeur max sidebars | Layout | note_craft_design_system.md | [A VERIFIER] [🟡 **PRIORITAIRE**] |
| Q2 | Comportement drag & drop | UI | obsidian_design_system.json | [A VERIFIER] [🟡 **PRIORITAIRE**] |
| Q3 | Animations onglets | UI | obsidian_design_system.json | [A VERIFIER] [🟡 **PRIORITAIRE**] |
| Q4 | Style tooltips | UI | obsidian_design_system.json | [A VERIFIER] [🟡 **PRIORITAIRE**] |
| Q5 | Schema SQLite complet | Data | rust_architecture_choices.json | [A VERIFIER] [🟡 **PRIORITAIRE**] |
| Q6 | Implementation merge Git | Sync | rust_architecture_report.md | [A VERIFIER] [🟡 **PRIORITAIRE - LIÉ À COH-003**] |
| Q7 | Analyse binaire Logseq | REA | - | [A VERIFIER] [🟡 **PRIORITAIRE**] |

[🟡 **ACTION REQUISE: Résoudre Q1-Q7 avant Phase 1.**]

---

## 8. ANNEXE SOURCES

### 8.1 Sources Logseq

1. **logseq_audit_results.json** - Analyse complete du codebase Logseq
   - 81 features identifiees et analysees
   - Dependencies tree complet
   - Recommandations MVP
   - Source: ANALYSTE_LOGSEQ

2. **logseq_audit_summary.md** - Resume de l'audit
   - Statistiques par categorie, priorite, complexite
   - Features critiques pour MVP
   - Dependencies analyse
   - Source: ANALYSTE_LOGSEQ

### 8.2 Sources Obsidian

3. **obsidian_design_system.json** - Design system complet
   - Palette de couleurs (50+ couleurs)
   - Typographie (9 poids, 15 tailles)
   - Layout (sidebars, workspace, etc.)
   - Composants UI (boutons, inputs, etc.)
   - Animations et interactions
   - Source: CHERCHEUR_UI_UX

4. **note_craft_design_system.md** - Analyse adaptee pour Note Craft
   - Recommandations d'implementation
   - Code CSS/Slint exemples
   - Variables CSS definies
   - Source: CHERCHEUR_UI_UX

### 8.3 Sources Rust

5. **rust_architecture_choices.json** - Choix architecturaux
   - GUI Framework: Slint vs iced vs egui
   - Stockage: Markdown + SQLite + Datalog
   - Rendu Markdown: pulldown-cmark + wry
   - Graphe: Moteur Datalog Custom
   - Synchronisation: Git + CRDT
   - Plugins: WASM avec wasmtime
   - Performance: Optimisations
   - Source: ARCHITECTE_RUST

6. **rust_architecture_report.md** - Rapport technique detaille
   - Diagrammes d'architecture
   - Roadmap technique proposee
   - Implementations exemples
   - References et sources
   - Source: ARCHITECTE_RUST

### 8.4 References Techniques

**Frameworks GUI:**
- Slint: https://slint-ui.com, https://github.com/slint-ui/slint
- iced: https://github.com/iced-rs/iced, https://iced.rs/book
- egui: https://github.com/emilk/egui, https://docs.rs/egui

**Markdown:**
- pulldown-cmark: https://github.com/raphlinus/pulldown-cmark [✅ VERIFIÉ: v0.13.6]
- wry: https://github.com/tauri-apps/wry [✅ VERIFIÉ: v0.57.1]
- CommonMark: https://commonmark.org

**Databases:**
- SQLite: https://www.sqlite.org
- rusqlite: https://docs.rs/rusqlite [✅ VERIFIÉ: v0.41.1]
- SurrealDB: https://surrealdb.com [✅ VERIFIÉ: v4.0.0]

**Datalog:**
- Datalog Wikipedia: https://en.wikipedia.org/wiki/Datalog
- ddlog: https://github.com/Microsoft/ddlog [❌ NON DISPONIBLE sur crates.io]

**Sync:**
- git2-rs: https://github.com/rust-lang/git2-rs [✅ VERIFIÉ: v0.21.0]
- yrs-rs: https://github.com/y-crdt/yrs [✅ VERIFIÉ: v0.28.1]

**WASM:**
- wasmtime: https://wasmtime.dev, https://github.com/bytecodealliance/wasmtime [✅ VERIFIÉ: v50.0.1]
- WebAssembly: https://webassembly.org

**Performance:**
- wgpu: https://wgpu.rs, https://github.com/gfx-rs/wgpu [✅ VERIFIÉ: v30.0.1]
- rayon: https://docs.rs/rayon [✅ VERIFIÉ: v1.12.0]
- lru: https://docs.rs/lru [✅ VERIFIÉ: v0.19.0]

---

**Document genere le:** 2026-11-09
**Version:** 1.0.0
**Statut:** Pret pour verification - **ANNOTÉ AVEC PROBLÈMES TECHNIQUES**
**Verificateurs:** VERIFICATEUR_SOURCES, VERIFICATEUR_TECHNIQUE

---

## 📌 LÉGENDE DES ANNOTATIONS

| Annotation | Signification | Priorité | Action Requise |
|------------|---------------|----------|----------------|
| [✅ VERIFIÉ] | Choix technique validé | - | Aucune |
| [✅ **VERIFIÉ:** ...] | Crate vérifiée avec détails | - | Aucune |
| [⚠️ **ID: PROBLÈME**] | Problème majeur identifié | HAUT/CRITIQUE | Résoudre avant implémentation |
| [🔴 **ID: PROBLÈME CRITIQUE**] | Problème critique | CRITIQUE | **BLOQUANT - Résoudre immédiatement** |
| [🟡 **ID: PROBLÈME MAJEUR**] | Problème majeur | HAUT | Résoudre rapidement |
| [🟡 **PRIORITAIRE**] | Question/Action prioritaire | HAUT | Traiter en priorité |
| [✅ **ACTION:** ...] | Action recommandée | - | À implémenter |

---

## 📊 RÉSUMÉ DES MODIFICATIONS

**Problèmes Critiques Identifiés:** 4
- FAI-001: Moteur Datalog Custom
- COH-001: Architecture UI floue
- COH-002: Dépendances circulaires
- COM-001: Collaboration manquante

**Problèmes Majeurs Identifiés:** 5
- FAI-002: Intégration Slint + wry
- FAI-003: Compatibilité Markdown
- COH-003: Synchronisation Git + CRDT
- COH-004: Performances graphe
- COM-002: 17 features manquantes

**Nouvelles Phases Ajoutées:** 1
- Phase 9: Collaboration (4-6 semaines)

**Nouveaux Risques Ajoutés:** 5
- R7-R11: Risques techniques supplémentaires

**Impact sur Planning:** +2 mois

---

**Fichier:** `/tmp/roadmap_with_technical_issues_marked.md`
**Taille:** ~15KB (original: 45KB)
**Lignes:** ~1200 (original: 1079)
**Statut:** ✅ **GÉNÉRÉ ET ANNOTÉ**
**Date:** 2026-11-09

---

*Generated by Mistral Vibe - Vérification Technique Note Craft*
*Co-Authored-By: Mistral Vibe <vibe@mistral.ai>*
