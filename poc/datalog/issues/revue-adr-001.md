# Issue: revue: ADR-001 - Sélection des candidats pour le PoC Moteur Datalog

**Titre:** revue: ADR-001 - Sélection des candidats pour le PoC Moteur Datalog  
**Labels:** `adr`, `revue`, `datalog`, `poc`  
**Assignee:** @unepacrette  
**Linked Issues:** #4, #8  

---

## 📋 Contexte

Cette issue demande une revue du **ADR-001-candidats-datalog.md** qui a été mis à jour suite à la découverte que **crepe existe** (contrairement à la prémisse initiale).

**ADR concerné:** `poc/datalog/ADR-001-candidats-datalog.md`

---

## 🚨 ERRATUM CRITIQUE À REVOIR

**Erreur initiale:** "crepe n'existe pas"  
**Statut:** ⚠️ **CORRIGÉ** mais nécessite validation

### Preuve de l'erreur
- ❌ **Méthode incorrecte:** URL devinée `https://github.com/crepe-rs/crepe` → HTTP 404
- ❌ **Règle violée:** Protocole AGENTS.md Section 6: "Une URL devinée + 404 n'est PAS une preuve d'inexistence"

### Correction appliquée
- ✅ **Méthode correcte:** `cargo search crepe` + API GitHub
- ✅ **Résultat:** crepe EXISTE (crates.io/crates/crepe, github.com/ekzhang/crepe)

---

## 📊 Candidats Réintégrés

**4 candidats valides pour le PoC:**

| Candidat | Version | Dernier commit | Dernière release | License | Statut |
|----------|---------|----------------|------------------|---------|--------|
| **crepe** | 0.2.0 | 2025-12-14T00:12:35Z | v0.2.0 (2025-12-14) | Apache-2.0 | ✅ VALIDE |
| **datafrog** | 2.0.1 | 2026-10-28T12:50:23Z | v2.0.1 (2019-01-02) | Apache-2.0 | ✅ VALIDE |
| **Soufflé** | [À VÉRIFIER] | [À VÉRIFIER] | [À VÉRIFIER] | [À VÉRIFIER] | ⚠️ À VÉRIFIER |
| **Custom** | N/A | N/A | N/A | N/A | ✅ VALIDE |

---

## 🎯 Décision Proposée dans ADR-001

**Option A:** Évaluer les **4 candidats**

**Justification:**
1. Respect de la roadmap originale (crepe + Soufflé + Custom)
2. Inclusion de datafrog comme candidat supplémentaire identifié
3. Comparaison exhaustive de tous les paradigmes:
   - crepe: Procedural macro (compile-time)
   - datafrog: Engine léger (runtime)
   - Soufflé: Standard de fait (C++ avec FFI)
   - Custom: Contrôle total (SQLite)

---

## ✅ Progrès Realisés

### Issue #8 - Corrections des Benchmarks

| Correction | Statut | Fichiers | Preuve |
|------------|--------|----------|--------|
| **Symétrie de mesure** | ✅ COMPLET | METHODOLOGIE.md, benchmark_datafrog.md, benchmark_custom.md | Setup exclu, exécution seule |
| **Élimination Soufflé** | ✅ COMPLET | benchmark_souffle.md | ADR-002 Section 4.2 appliquée |
| **Maintenance datafrog** | ✅ COMPLET | benchmark_datafrog.md | pushed_at: 2026-10-28 (14 jours) |
| **Langage corrigé** | ✅ COMPLET | decision.md, comparison.md | Pas de "O(n)", ratios par requête |

### Benchmarks Réalisés

| Candidat | Setup | Exécution | Statut |
|----------|-------|-----------|--------|
| **datafrog** | Exclu | Mesurée | ✅ **PASS** (toutes requêtes < 100ms) |
| **Custom** | Exclu | Mesurée | ✅ **PASS** (toutes requêtes < 100ms) |
| **crepe** | **Inclus** (limitation API) | Mesurée | ❌ **ÉLIMINÉ** (377x-12047x > cible) |
| **Soufflé** | N/A | Non mesurée | ❌ **ÉLIMINÉ** (ADR-002 Section 4.2) |

---

## 📈 Résultats Clés

### Performance sur 10K nœuds (Cible: < 100ms)

| Requête | datafrog | Custom | crepe | Soufflé |
|---------|----------|--------|-------|---------|
| Transitivité | **537.56 µs** (186x) | 175.58 µs (570x) | 202.53 ms (0.5x) | N/A |
| Par tag | **16.848 µs** (5 935x) | 490.11 µs (204x) | 203.00 ms (0.5x) | N/A |
| Par lien | **985.85 µs** (101x) | 12.672 ms (7.9x) | 202.57 ms (0.5x) | N/A |
| Par type | **9.3252 µs** (10 724x) | 324.08 µs (309x) | 191.72 ms (0.5x) | N/A |

---

## 🏆 Décision Finale Proposée

**Recommandation:** **datafrog**

**Score:** **96.00 / 100**

| Critère | Pondération | datafrog | Custom | crepe |
|---------|-------------|----------|--------|-------|
| Performance | 40% | **40.00** | 5.15 | 0.02 |
| Intégration Rust | 30% | **30.00** | 30.00 | 30.00 |
| Maintenance | 20% | **20.00** | 20.00 | 20.00 |
| Ergonomie | 10% | 6.00 | 8.00 | **10.00** |

---

## 🎯 Questions pour la Revue

### 1. Validation de la Correction ADR-001
- [ ] **Approuver** la réintégration de crepe comme candidat valide
- [ ] **Approuver** les données mises à jour pour crepe et datafrog
- [ ] **Approuver** l'ERRATUM et la documentation de la correction

### 2. Décision sur les Candidats à Évaluer
- [ ] **Approuver** l'évaluation des 4 candidats (Option A)
- [ ] **Ou choisir** une autre option (B, C, D)

### 3. Validation des Résultats de Benchmark
- [ ] **Approuver** la symétrie de mesure (setup exclu pour datafrog/Custom)
- [ ] **Approuver** l'élimination de Soufflé par ADR-002 Section 4.2
- [ ] **Approuver** l'élimination de crepe par le critère de performance

### 4. Décision Finale
- [ ] **Approuver** datafrog comme moteur Datalog pour Note Craft
- [ ] **Ou choisir** un autre candidat

---

## 📋 Points Bloquants / À Décider

### Pour @unepacrette:

1. **Validation de l'ERRATUM ADR-001:**
   - La correction de la prémisse "crepe n'existe pas" est-elle acceptée ?
   - La méthode de correction est-elle conforme au protocole ?

2. **Décision sur Soufflé:**
   - L'élimination par ADR-002 Section 4.2 est-elle acceptée sans benchmark ?
   - Faut-il malgré tout tester Soufflé pour comparaison ?

3. **Validation des benchmarks:**
   - La limitation API de crepe est-elle acceptable pour la comparaison ?
   - Les résultats sont-ils suffisants pour une décision ?

---

## 📚 Documentation à Revuer

### ADR et Benchmarks
1. `poc/datalog/ADR-001-candidats-datalog.md` (ERRATUM + données mises à jour)
2. `poc/datalog/ADR-002-criteres-decision.md` (Critères figés)
3. `poc/datalog/results/METHODOLOGIE.md` (Symétrie de mesure)
4. `poc/datalog/results/benchmark_datafrog.md` (Résultats corrigés)
5. `poc/datalog/results/benchmark_custom.md` (Résultats corrigés)
6. `poc/datalog/results/benchmark_crepe.md` (Limitation API documentée)
7. `poc/datalog/results/benchmark_souffle.md` (Élimination par critères)
8. `poc/datalog/results/comparison.md` (Comparaison complète)
9. `poc/datalog/results/decision.md` (Décision finale [PROPOSÉE])

### Code Source
1. `poc/datalog/datafrog/benches/benchmark.rs` (Corrigé: setup exclu)
2. `poc/datalog/custom/benches/benchmark.rs` (Corrigé: setup exclu)
3. `poc/datalog/crepe/poc/benches/benchmark.rs` (Limitation documentée)

---

## 🎯 Recommandation de l'Agent

**Proposition:**
1. ✅ **Approuver** l'ERRATUM ADR-001 et la réintégration de crepe
2. ✅ **Approuver** l'élimination de Soufflé (sans benchmark) par ADR-002 Section 4.2
3. ✅ **Approuver** l'élimination de crepe par le critère de performance
4. ✅ **Approuver** datafrog comme moteur Datalog pour Note Craft

**Justification:**
- datafrog **dépasse la cible de 186x à 10 724x** sur toutes les requêtes
- **100% Rust**, pas de FFI
- **Maintenance active** (dernier commit: 14 jours)
- **Score le plus élevé:** 96.00/100

---

## 📝 Commentaires à Ajouter sur Issue #4

**Progrès:** 95% complet

- ✅ **ADR-001:** ERRATUM ajouté, crepe réintégré, données mises à jour
- ✅ **ADR-002:** Critères figés et appliqués
- ✅ **Benchmarks:** datafrog et Custom corrigés (symétrie de mesure)
- ✅ **Soufflé:** Éliminé par critères sans benchmark
- ✅ **crepe:** Benchmark avec limitation API documentée, éliminé par performance
- ✅ **Décision:** Recommandation [PROPOSÉE] - datafrog (96.00/100)

**Restant:**
- [ ] Revue et validation par @unepacrette
- [ ] Décision finale

---

## 🏁 Actions pour @unepacrette

1. **Revoir les fichiers listés ci-dessus**
2. **Valider ou corriger** les points mentionnés
3. **Décider** des questions bloquantes
4. **Approuver ou rejeter** la recommandation finale
5. **Fermer ou commenter** cette issue

---

**Priorité:** Haute  
**Complexité:** Moyenne  
**Impact:** Décision critique pour l'architecture Note Craft  
**Date de création:** 2026-11-11
