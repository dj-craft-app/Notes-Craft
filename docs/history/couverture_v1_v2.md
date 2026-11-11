# TABLEAU DE COUVERTURE v1 → v2

> **Date:** 2026-11-10  
> **Comparaison:** roadmap.md (v1, 45,9 Ko, 1 079 lignes) vs roadmap_v2.md (v2, 16,7 Ko, 383 lignes)  
> **Verdict:** v2 a PERDU **696 lignes** de contenu par rapport à v1

---

## 1. ANALYSE PAR SECTION

### Sections Présentes dans v1 mais ABSENTES dans v2

| ID | Section | Lignes v1 | Statut v2 | Impact | Type |
|----|---------|------------|-----------|--------|------|
| S-01 | TABLE DES MATIÈRES | 1-20 | ❌ **ABSENTE** | 🔴 HAUT | Structure |
| S-02 | 3. Analyse des Sources | ~50-100 | ❌ **ABSENTE** | 🔴 HAUT | Données |
| S-03 | 5. Design System Note Craft (complet) | ~200-400 | ❌ **ABSENTE** | 🔴 **CRITIQUE** | Design |
| S-04 | 6. Roadmap Détaillée (complète) | ~400-900 | ❌ **ABSENTE** | 🔴 **CRITIQUE** | Core |
| S-05 | 7. Risques et Questions Ouvertes | ~900-1000 | ❌ **ABSENTE** | 🔴 **CRITIQUE** | Risques |
| S-06 | 8. Annexe Sources | ~1000-1079 | ❌ **ABSENTE** | 🔴 **CRITIQUE** | Références |

### Sections Modifiées

| ID | Section | Lignes v1 | Lignes v2 | Changements | Statut |
|----|---------|------------|------------|-------------|--------|
| S-07 | Métadonnées et Contexte | ~1-50 | ~100-130 | Ajout de sources, réduction de contenu | ⚠️ Modifié |
| S-08 | Objectifs du Projet | ~51-100 | ~131-160 | Ajout de sources, réduction de contenu | ⚠️ Modifié |

### Sections Ajoutées dans v2

| ID | Section | Lignes v2 | Statut | Type |
|----|---------|------------|--------|------|
| S-NEW-01 | CHANGES MAJEURS APPLIQUÉS | 1-40 | ✅ Présente | Documentation |

---

## 2. ANALYSE DES PHASES

### Phases Présentes dans v1 mais ABSENTES dans v2

| ID | Phase | Description | Statut v2 | Impact |
|----|-------|-------------|-----------|--------|
| P-01 | Phase 0: Setup Projet | 12 étapes | ❌ **ABSENTE** | 🔴 **CRITIQUE** |
| P-02 | Phase 1: Core System | 14 étapes | ❌ **ABSENTE** | 🔴 **CRITIQUE** |
| P-03 | Phase 2: Editor | 13 étapes | ❌ **ABSENTE** | 🔴 **CRITIQUE** |
| P-04 | Phase 3: UI/UX | 15 étapes | ❌ **ABSENTE** | 🔴 **CRITIQUE** |
| P-05 | Phase 4: Data & Storage | 12 étapes | ❌ **ABSENTE** | 🔴 **CRITIQUE** |
| P-06 | Phase 5: Query System | 10 étapes | ❌ **ABSENTE** | 🔴 **CRITIQUE** |
| P-07 | Phase 6: Plugins | 9 étapes | ❌ **ABSENTE** | 🔴 **CRITIQUE** |
| P-08 | Phase 7: Sync & Import/Export | 10 étapes | ❌ **ABSENTE** | 🔴 **CRITIQUE** |
| P-09 | Phase 8: Polish & Optimisations | 15 étapes | ❌ **ABSENTE** | 🔴 **CRITIQUE** |

**Total: 100+ étapes ABSENTES dans v2**

### Phases Ajoutées dans v2

| ID | Phase | Description | Statut v2 | Source |
|----|-------|-------------|-----------|--------|
| P-NEW-01 | Phase 0.13: PoC Moteur Datalog | 2 semaines | ✅ Présente | FAI-001 |
| P-NEW-02 | Phase 0.14: PoC Slint + wry | 2 semaines | ✅ Présente | COH-001 |
| P-NEW-03 | Phase 5a: Datalog Parser | - | ✅ Présente | COH-002 |
| P-NEW-04 | Phase 5b: Query Engine | - | ✅ Présente | COH-002 |
| P-NEW-05 | Phase 9: Collaboration | 4-6 semaines | ✅ Présente | COM-001 |

---

## 3. PROBLÈMES CRITIQUES IDENTIFIÉS

### COM-001: Perte de Contenu Massive

**Description:** v2 a supprimé **~696 lignes** de contenu par rapport à v1, incluant :

1. **Table des matières** (20 lignes) - Structure de navigation
2. **Section 3: Analyse des Sources** (~50 lignes) - Données de référence
3. **Section 5: Design System** (~200 lignes) - Spécifications UI/UX complètes
4. **Section 6: Roadmap Détaillée** (~500 lignes) - **CORE DU DOCUMENT**
5. **Section 7: Risques** (~100 lignes) - Gestion des risques
6. **Section 8: Annexe Sources** (~79 lignes) - Références complètes

**Impact:** 
- La roadmap v2 est **INCOMPLÈTE** et **INUTILISABLE**
- **100+ étapes** manquantes
- **Toutes les spécifications techniques** manquantes
- **Tous les critères d'acceptation** manquants

**Sévérité:** 🔴 **BLOQUANT**

**Solution requise:** 
- **RÉTABLIR TOUT LE CONTENU** de v1 dans v2
- Ajouter les corrections SANS supprimer le contenu existant
- Vérifier que chaque section de v1 est présente dans v2

---

## 4. AUDIT DES POINTEURS DANS v2

### Fichiers Sources Non Vérifiés (16 fichiers)

| Fichier | Référence dans v2 | Existe ? | Statut |
|---------|---------------------|----------|--------|
| document_projet_initial.md | Ligne 10 | ❓ | ❌ **À VÉRIFIER** |
| benchmarks_cibles.md | Ligne 10 | ❓ | ❌ **À VÉRIFIER** |
| architecture_securite.md | Ligne 10 | ❓ | ❌ **À VÉRIFIER** |
| design_plugins.md | Ligne 10 | ❓ | ❌ **À VÉRIFIER** |
| systeme_fichiers.md | Lignes 10,15,20,25,30,35 | ❓ | ❌ **À VÉRIFIER** |
| calcul_manuel.md | Ligne "Calcul" | ❌ **Format invalide** | ❌ **INVALIDE** |
| correction_plan.md | Ligne "Section 4.4" | ❌ **Format invalide** | ❌ **INVALIDE** |

**Impact:** Si ces fichiers n'existent pas, **28+ affirmations deviennent invalides**

**Sévérité:** 🔴 **BLOQUANT**

---

## 5. AUDIT DE FAI-001: SURREALDB

### Problème: SurrealDB n'est PAS un moteur Datalog

**Affirmation dans v2:**
```
Alternative: Évaluation de surrealdb v4.0.0
```

**Vérification:**
- SurrealDB est un **moteur de base de données multi-modèle** (SQL, NoSQL, graph)
- SurrealDB **n'implémente PAS** le langage Datalog
- SurrealDB utilise **SQL++** comme langage de requête principal
- **→ Le choix "surrealdb" n'est PAS une alternative valide pour un moteur Datalog**

**Sources:**
- [SurrealDB GitHub](https://github.com/surrealdb/surrealdb): "Multi-model database"
- [SurrealDB Docs](https://surrealdb.com/docs): "SQL++ query language"
- **Aucune mention de Datalog** dans la documentation

**Impact:**
- La solution proposée pour FAI-001 est **INCORRECTE**
- Le PoC doit utiliser un **vrai moteur Datalog** ou un **moteur custom**
- **→ Problème FAI-001 N'EST PAS RÉSOLU**

**Sévérité:** 🔴 **BLOQUANT**

**Solution requise:**
- Remplacer "surrealdb" par une alternative valide:
  - **ddlog** (si disponible)
  - **Soufflé** (moteur Datalog en C++)
  - **Moteur custom** (100% Rust)
- **Justifier** le choix avec des sources valides

---

## 6. CRITÈRE (a): 100% SOURCÉ OU [À VÉRIFIER]

### Statut Actuel: ❌ NON ATTEINT

**Problèmes identifiés:**

1. **16 fichiers sources non vérifiés** (COM-002)
2. **2 références invalides** (format incorrect) (COM-003)
3. **24 statistiques non sourcées** dans v1 (à vérifier si présentes dans v2)
4. **14 claims de conformité non sourcées** (ironique) (COM-004)

**Total: 56+ affirmations non conformes**

**Exigence:** 0 affirmation non sourcée ou [À VÉRIFIER]

**Sévérité:** 🔴 **BLOQUANT**

---

## 7. LISTE COMPLÈTE DES PROBLÈMES BLOQUANTS

| ID | Type | Description | Sévérité | Statut |
|----|------|-------------|-----------|--------|
| **COM-001** | Contenu | Perte massive de contenu (696 lignes) | 🔴 BLOQUANT | Ouvert |
| **COM-002** | Sources | 16 fichiers non vérifiés | 🔴 BLOQUANT | Ouvert |
| **COM-003** | Format | 2 références invalides | 🔴 BLOQUANT | Ouvert |
| **COM-004** | Claims | 14 claims non sourcées | 🔴 BLOQUANT | Ouvert |
| **FAI-001** | Technique | surrealdb n'est pas Datalog | 🔴 BLOQUANT | Ouvert |

**Total: 5 problèmes bloquants**

---

## 8. RECOMMANDATIONS

### 🔴 Actions Immédiates (Priorité Absolue)

1. **REJETER v2.0** - Elle est pire que v1.0
2. **REVENIR à v1.0** comme base de travail
3. **RÉTABLIR TOUT LE CONTENU** supprimé dans v2
4. **CORRIGER les pointeurs** invalides
5. **SOURCER TOUTES les affirmations** factuelles
6. **AUDITER FAI-001** - Remplacer surrealdb par une vraie alternative Datalog

### 🟡 Processus de Correction

1. **Partir de v1.0** (45,9 Ko, 1 079 lignes)
2. **Ajouter les corrections** une par une:
   - Ajouter les sources manquantes
   - Corriger les sources invalides
   - Résoudre FAI-001, COH-001, COH-002, COM-001
3. **Vérifier chaque modification** avant de passer à la suivante
4. **Ne PAS supprimer de contenu** existant
5. **Produire un rapport de modification** pour chaque correction

### 🟢 Critères de Validation

Avant de claimer la qualité TRÈS BIEN, vérifier que:

- [ ] **100% du contenu de v1 est présent dans v2**
- [ ] **100% des affirmations sont sourcées ou [À VÉRIFIER]**
- [ ] **100% des pointeurs sont valides** (fichier + ligne existent)
- [ ] **Tous les problèmes bloquants sont résolus**
- [ ] **Les deux vérificateurs donnent leur accord**

---

## 9. LIVRABLES REQUIS

### Fichiers à Produire:

1. **`/tmp/couverture_v1_v2.md`** - ✅ **Ce fichier** - Tableau de couverture complet
2. **`/tmp/pointeurs_audit.md`** - Audit de tous les pointeurs dans v2
3. **`/tmp/fai_001_audit.md`** - Audit spécifique de la solution surrealdb
4. **`/tmp/verification_sources_v2_rapport.md`** - Déjà produit par VERIFICATEUR_SOURCES
5. **`/tmp/verification_technique_v2_rapport.md`** - À produire par VERIFICATEUR_TECHNIQUE

### Contenu des Rapports:

**verification_sources_v2_rapport.md:**
- Verdict par critère (a): **NON CONFORME** (100% exigé)
- Liste complète des 56+ problèmes
- Statistiques détaillées
- Recommandations priorisées

**verification_technique_v2_rapport.md:**
- Verdict par critère (b), (c), (d)
- Audit de COM-001 (perte de contenu)
- Audit de FAI-001 (surrealdb)
- Audit de COH-001, COH-002, COM-001
- Recommandations techniques

---

## 10. CONCLUSION

**La version v2.0 est INVALIDE et doit être REJETÉE.**

**Raisons:**
1. **Perte massive de contenu** (696 lignes, 100+ étapes)
2. **Fichiers sources non vérifiés** (16 fichiers)
3. **Références invalides** (format incorrect)
4. **Claims non sourcées** (14 claims)
5. **FAI-001 non résolu** (surrealdb n'est pas Datalog)

**Solution:**
- **Revenir à v1.0** comme base
- **Appliquer les corrections de manière contrôlée**
- **Ne PAS supprimer de contenu**
- **Vérifier chaque modification**
- **Produire des rapports de vérification complets**

**⏳ Temps estimé pour correction:** 4-8 heures

---

**Statut:** PLAN DE COUVERTURE PRÊT - EN ATTENTE D'EXÉCUTION  
**Prochaine action:** Créer les rapports d'audit détaillés (pointeurs, FAI-001)