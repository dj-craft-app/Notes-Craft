# PLAN DE CORRECTION - ROADMAP NOTE CRAFT

> **Version:** 1.1  
> **Date:** 2026-11-09  
> **Objectif:** Corriger roadmap.md pour atteindre qualité TRÈS BIEN (100% sourcé, 4/4 critères)  
> **Cible:** 95%+ conformité sources, résolution des 4 problèmes techniques bloquants

---

## 1. PROBLÈMES DE SOURCES À CORRIGER (86 + 33 + 9 = 128 problèmes)

### 1.1 STATISTIQUES LOGSEQ (Section 1) - 15+ problèmes

| Lignes | Affirmation | Correction | Source exacte | Statut |
|--------|-------------|------------|----------------|--------|
| ~14-18 | 81 features totales | Conserver | `logseq_audit_results.json:metadata.total_features` (ligne 5) | ⏳ |
| ~14-18 | Core(18) | Conserver | `logseq_audit_results.json:statistics.by_category.Core` (ligne 9) | ⏳ |
| ~14-18 | UI(21) | Conserver | `logseq_audit_results.json:statistics.by_category.UI` (ligne 17) | ⏳ |
| ~14-18 | Editor(13) | Conserver | `logseq_audit_results.json:statistics.by_category.Editor` (ligne 10) | ⏳ |
| ~14-18 | Data(9) | Conserver | `logseq_audit_results.json:statistics.by_category.Data` (ligne 11) | ⏳ |
| ~14-18 | Plugins(3) | Conserver | `logseq_audit_results.json:statistics.by_category.Plugins` (ligne 15) | ⏳ |
| ~14-18 | Query(4) | Conserver | `logseq_audit_results.json:statistics.by_category.Query` (ligne 12) | ⏳ |
| ~14-18 | Sync(1) | Conserver | `logseq_audit_results.json:statistics.by_category.Sync` (ligne 14) | ⏳ |
| ~14-18 | Import-Export(3) | Conserver | `logseq_audit_results.json:statistics.by_category.Import-Export` (ligne 13) | ⏳ |
| ~14-18 | Collaboration(5) | Conserver | `logseq_audit_results.json:statistics.by_category.Collaboration` (ligne 8) | ⏳ |
| ~14-18 | Other(4) | Conserver | `logseq_audit_results.json:statistics.by_category.Other` (ligne 16) | ⏳ |
| ~14-18 | Par priorite: Haut(37), Moyen(23), Bas(21) | Conserver | `logseq_audit_results.json:statistics.by_priority` (lignes 20-22) | ⏳ |
| ~14-18 | Par complexite: 2(6), 3(24), 4(32), 5(19) | Conserver | `logseq_audit_results.json:statistics.by_complexity` (lignes 24-27) | ⏳ |
| ~14-18 | Features MVP: 30 identifiees | **CORRIGER** 35→30 | `logseq_audit_summary.md:49-150` (30 features listées) | ⏳ |
| ~14-18 | Dependances: 40+ analysees | **CORRIGER** | Compter dans logseq_audit_results.json | ⏳ |
| ~30-32 | Decisions: 7 domaines techniques | **CORRIGER** | Compter dans rust_architecture_choices.json | ⏳ |
| ~30-32 | Frameworks compares: 6 GUI, 4 stockage, 2 parseurs, 4 moteurs Datalog | **CORRIGER** | Compter dans rust_architecture_report.md | ⏳ |
| ~30-32 | Dependances: 26 requises, 8 optionnelles | **CORRIGER** | Compter dans rust_architecture_report.md | ⏳ |
| ~34 | Total: 6 fichiers, ~185KB | **CORRIGER** | Calculer: 48+28+16+25+30+40 = 187KB | ⏳ |

### 1.2 STATISTIQUES OBSIDIAN (Section 1) - 4 problèmes

| Lignes | Affirmation | Correction | Source | Statut |
|--------|-------------|------------|--------|--------|
| ~24-28 | Palette: 15+ categories, 50+ couleurs | **CORRIGER** | Compter dans obsidian_design_system.json | ⏳ |
| ~24-28 | Typographie: 9 poids, 15 tailles, 4 polices | **CORRIGER** | Compter dans obsidian_design_system.json | ⏳ |
| ~24-28 | Composants: 12+ types documentes | **CORRIGER** | Compter dans note_craft_design_system.md | ⏳ |
| ~24-28 | Layout: Systeme grille 2px/4px base | **CORRIGER** | Vérifier dans note_craft_design_system.md | ⏳ |

### 1.3 STATISTIQUES ARCHITECTURE RUST (Section 1) - 3 problèmes

| Lignes | Affirmation | Correction | Source | Statut |
|--------|-------------|------------|--------|--------|
| ~30-32 | Decisions: 7 domaines techniques | Compter sections dans rust_architecture_report.md | rust_architecture_report.md | ⏳ |
| ~30-32 | Frameworks compares: 6 GUI, 4 stockage, 2 parseurs, 4 moteurs Datalog | Compter dans les tableaux | rust_architecture_report.md | ⏳ |
| ~30-32 | Dependances: 26 requises, 8 optionnelles | Compter dans rust_architecture_choices.json | rust_architecture_choices.json | ⏳ |

### 1.4 CONTRAINTES ORGANISATIONNELLES (Section 2) - 3 problèmes

| Lignes | Affirmation | Correction | Source | Statut |
|--------|-------------|------------|--------|--------|
| ~56-60 | Equipe reduite (3-5 developpeurs) | **MARQUER** [À VÉRIFIER - Decision projet] | Aucun fichier source | ⏳ |
| ~56-60 | Budget limite (open-source community-driven) | **MARQUER** [À VÉRIFIER - Decision projet] | Aucun fichier source | ⏳ |
| ~56-60 | Delai cible: 12-13 mois pour version 1.0 | **CORRIGER** vers 10-15 mois | Voir Phase 7 planning révisé | ⏳ |

### 1.5 PERFORMANCES CIBLES (Section 4) - 8+ problèmes

| Lignes | Affirmation | Correction | Source | Statut |
|--------|-------------|------------|--------|--------|
| ~230 | Requete simple: < 10ms | **MARQUER** [À VÉRIFIER - Benchmark à réaliser] | - | ⏳ |
| ~230 | Requete complexe recursive: < 100ms pour 10K notes | **MARQUER** [À VÉRIFIER - Benchmark à réaliser] | - | ⏳ |
| ~260 | Extensions supportees: Tables (GFM), Wikilinks, Tags, etc. | **SOURCER** | pulldown-cmark docs + tests | ⏳ |
| ~290 | Requete simple: < 10ms (duplique) | **MARQUER** [À VÉRIFIER - Benchmark à réaliser] | - | ⏳ |
| ~290 | Requete complexe: < 100ms pour 10K notes (duplique) | **MARQUER** [À VÉRIFIER - Benchmark à réaliser] | - | ⏳ |
| ~340 | Rendu < 100ms pour 10K noeuds | **MARQUER** [À VÉRIFIER - Benchmark à réaliser] | - | ⏳ |
| ~340 | Requetes < 10ms (simple) | **MARQUER** [À VÉRIFIER - Benchmark à réaliser] | - | ⏳ |
| ~340 | Requetes < 100ms (complexe) | **MARQUER** [À VÉRIFIER - Benchmark à réaliser] | - | ⏳ |
| ~340 | Chargement initial < 2 secondes | **MARQUER** [À VÉRIFIER - Benchmark à réaliser] | - | ⏳ |

### 1.6 SPÉCIFICATIONS DESIGN (Section 5) - 20+ problèmes

Toutes les spécifications numériques (32px, 4px, 16px, etc.) doivent être sourcées à note_craft_design_system.md avec numéros de ligne exacts.

### 1.7 RISQUES ORGANISATIONNELS (Section 7) - 4 problèmes

| Lignes | Affirmation | Correction | Source | Statut |
|--------|-------------|------------|--------|--------|
| R7-R10 | Risques organisationnels | **MARQUER** [À VÉRIFIER - Analyse à réaliser] | - | ⏳ |

### 1.8 SOURCES INVALIDES (33 problèmes)

- **"Rust best practices"** → Remplacer par des sources concrètes (crates.io, docs.rs)
- **"-"** → Sourcer ou supprimer l'affirmation
- **Lignes inexistantes** → Vérifier et corriger les références

---

## 2. PROBLÈMES TECHNIQUES À RÉSOUDRE (4 CRITIQUES)

### 2.1 FAI-001: Moteur Datalog Custom

**Problème:** Complexité sous-estimée, ddlog non disponible sur crates.io

**Solution:**
- Ajouter une **étape pré-Phase 5**: "PoC Moteur Datalog"
- Durée: 2-4 semaines
- Objectif: Valider faisabilité avec 1K-10K nœuds
- Critère d'acceptation: Requêtes < 100ms sur 10K nœuds
- Justification: `verification_technique_rapport.md:FAI-001`

**Modifications dans roadmap_v2:**
- Ajouter "Phase 4.5: PoC Moteur Datalog" avant Phase 5
- Ou intégrer comme étape 0 de Phase 5

### 2.2 COH-001: Architecture UI floue (Slint vs wry)

**Problème:** Incohérence entre Slint (GUI natif) et wry (WebView)

**Solution:**
- **Trancher clairement:** Slint = GUI principal + wry = WebView embarquée
- **Architecture:** Slint comme container, wry comme widget custom pour le rendu Markdown
- **Limites connues:**
  - Communication Slint ↔ wry via IPC
  - Gestion des événements souris/clavier
  - Focus et z-ordering
- **Documenter dans:** Section 4 (Choix Techniques)
- **Justification:** `verification_technique_rapport.md:COH-001`

**Modifications dans roadmap_v2:**
- Clarifier dans la section "GUI Framework"
- Ajouter diagramme d'architecture mis à jour

### 2.3 COH-002: Dépendances circulaires Phases 4/5

**Problème:** Phase 5 dépend de Phase 4 ET de Phase 1

**Solution:**
- **Découper Phase 5 en 5a et 5b:**
  - Phase 5a: Moteur Datalog (dépend de Phase 1)
  - Phase 5b: Query System (dépend de Phase 4 et 5a)
- **Nouvel ordre:** Phase 1 → Phase 4 → Phase 5a → Phase 5b
- **Justification:** `verification_technique_rapport.md:COH-002`

**Modifications dans roadmap_v2:**
- Renuméroter les phases
- Découper Phase 5 en 5a et 5b
- Mettre à jour les dépendances

### 2.4 COM-001: Collaboration manquante

**Problème:** Pas de phase dédiée à la collaboration temps réel

**Solution:**
- **Ajouter Phase 9: Collaboration et Synchronisation Avancée**
- Durée: 4-6 semaines
- Contenu:
  - Intégration yrs-rs (CRDT)
  - Synchronisation temps réel
  - Gestion des conflits
  - Tests de charge (100+ utilisateurs)
- **Justification:** `verification_technique_rapport.md:COM-001`

**Modifications dans roadmap_v2:**
- Ajouter Phase 9 après Phase 8
- Mettre à jour le planning global

---

## 3. PLANNING RÉVISÉ (10-15 mois)

**Impact des corrections:**
- PoC Moteur Datalog: +2-4 semaines
- PoC Slint + wry: +2-4 semaines
- Réordonnancement: +2 semaines
- Phase 9 Collaboration: +4-6 semaines
- Buffer risques: +4 semaines
- **Total:** +2-4 mois (de 8-13 à 10-15 mois)

**Nouveau planning:**
```
Phase 0: Setup Projet (1-2 semaines)
Phase 1: Core System (2-4 semaines)
Phase 2: Editor (4-6 semaines)
Phase 3: UI/UX (6-8 semaines)
Phase 4: Data & Storage (4-6 semaines)
Phase 4.5: PoC Moteur Datalog (2-4 semaines) [NOUVELLE]
Phase 5a: Query Engine (4-6 semaines) [MODIFIÉE]
Phase 5b: Query System (4-6 semaines) [NOUVELLE]
Phase 6: Plugins (6-8 semaines)
Phase 7: Sync & Import/Export (4-6 semaines)
Phase 8: Polish (4-6 semaines)
Phase 9: Collaboration (4-6 semaines) [NOUVELLE]
Total: 10-15 mois
```

**Source:** `verification_technique_rapport.md` (impact sur planning)

---

## 4. LIVRABLES ATTENDUS

### 4.1 Fichiers à produire:
- [ ] `/tmp/roadmap_v2.md` - Roadmap corrigée
- [ ] `/tmp/correction_rapport.md` - Rapport de correction détaillé

### 4.2 Contenu du rapport de correction:
- [ ] Avant/Après pour chaque problème corrigé
- [ ] Couverture complète: 86 + 33 + 9 = 128 problèmes
- [ ] Preuves des modifications (extraits de code)
- [ ] Statistiques avant/après
- [ ] Temps estimé pour les [À VÉRIFIER] restants

---

## 5. CRITÈRES DE VALIDATION

### 5.1 Critère (a): 100% des affirmations sourcées OU [À VÉRIFIER]
- **Cible:** 95%+ conformité (vs 67,7% actuel)
- **Méthode:** Compter les affirmations dans roadmap_v2.md
- **Validation:** VERIFICATEUR_SOURCES sur roadmap_v2

### 5.2 Critère (b): Roadmap complète
- **Cible:** MVP → v1, design system, choix d'outils, étapes testables
- **Méthode:** Vérifier la présence de toutes les sections
- **Validation:** Vérification manuelle + VERIFICATEUR_TECHNIQUE

### 5.3 Critère (c): Double vérification sans divergence
- **Cible:** Aucun problème non résolu
- **Méthode:** Relancer les deux vérificateurs
- **Validation:** Boucle correction → vérification jusqu'à TRÈS BIEN

### 5.4 Critère (d): Questions Phase 0 répondues
- **Cible:** 100%
- **Statut:** ✅ Déjà validé
- **Validation:** Vérification manuelle

---

## 6. PROCHAINES ÉTAPES (PHASE 7)

1. **Lancer VERIFICATEUR_SOURCES sur roadmap_v2**
2. **Lancer VERIFICATEUR_TECHNIQUE sur roadmap_v2**
3. **Analyser les rapports de vérification**
4. **Si problèmes restants:**
   - Corriger les problèmes identifiés
   - Relancer la vérification
5. **Si PAS de problèmes:**
   - Évaluer sur le barème qualité
   - Livrer la roadmap finale

---

## 7. TEMPS ESTIMÉ

| Étape | Durée | Statut |
|-------|-------|--------|
| Correction Phase 6 | 2-4 heures | ⏳ En cours |
| Vérification Phase 7 | 2-4 heures | ⏳ En attente |
| Correction itérative | 1-2 heures | ⏳ Si nécessaire |
| **Total** | **5-10 heures** | ⏳ |

---

**Statut:** PLAN PRÊT - EN ATTENTE D'EXÉCUTION  
**Prochaine action:** Exécuter les corrections selon ce plan