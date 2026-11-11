# ERRATUM-001 pour ADR-002: Contradiction sur l'utilisation de criterion

> **Date:** 2026-11-11  
> **Auteur:** Agent autonome  
> **ADR Parent:** ADR-002-criteres-decision.md  
> **Statut:** **CORRECTION DOCUMENTÉE**  
> **Issue:** #8

---

## 🚨 CONTRADICTION IDENTIFIÉE

**Problème:** Incohérence entre les affirmations sur l'utilisation de criterion.rs

### Affirmation 1: "criterion déjà en place"
**Source:** Implicite dans ADR-002 Section 3 (Méthodologie)

ADR-002 **spécifie** (Section 3.1) :
```
**Outil:** [criterion.rs](https://docs.rs/criterion/latest/criterion/)
**Version:** 0.5+ (ou dernière version stable)

**Configuration obligatoire:**
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
```
```

**Interprétation:** ADR-002 **exige** l'utilisation de criterion **AVANT** les benchmarks.

---

### Affirmation 2: "benchmarks statistiques à venir"
**Source:** Fichiers de résultats (après les benchmarks)

Dans `poc/datalog/results/comparison.md` et `poc/datalog/results/decision.md` :
```
1. **Exécuter benchmarks statistiques (criterion):**
   - Médiane + écart-type sur ≥ 20 exécutions
   - Jeux 100 / 1K / 10K nœuds
   - [À VÉRIFIER - benchmark statistique à produire]
```

**Interprétation:** Les benchmarks statistiques n'étaient **PAS encore exécutés**. 

---

## 🔍 INVESTIGATION

### Vérité établies (sourcées)

1. **ADR-002 est FIGÉ** (2026-11-11):
   - Documenté comme "À ne plus modifier après publication"
   - Spécifie criterion comme outil **obligatoire**

2. **Code benchmark.rs utilise criterion**:
   - `poc/datalog/crepe/poc/benches/benchmark.rs`: `use criterion::{...}`
   - `poc/datalog/datafrog/benches/benchmark.rs`: `use criterion::{...}`
   - `poc/datalog/custom/benches/benchmark.rs`: `use criterion::{...}`

3. **Sortie criterion obtenue** (2026-11-11):
   ```
   Benchmarking query_by_tag/crepe/large: Analyzing
   query_by_tag/crepe/large
                           time:   [190.49 ms 192.03 ms 193.96 ms]
                           thrpt:  [51.558 Kelem/s 52.075 Kelem/s 52.497 Kelem/s]
   Found 12 outliers among 100 measurements (12.00%)
   ```
   **→ Les benchmarks utiliser criterion DEPUIS LE DÉBUT**

4. **Affirmation fausse:**
   - Dans `comparison.md` et `decision.md`: "Exécuter benchmarks statistiques" comme action à venir
   - **C'est FAUX**: Les benchmarks utilisent déjà criterion

---

## ✅ RÉSOLUTION

**L'affirmation "benchmarks statistiques à venir" était FAUSSE.**

### Preuves que criterion était déjà utilisé:

1. **ADR-002 Section 3.1** (figée avant les benchmarks):
   ```
   **Outil:** [criterion.rs](https://docs.rs/criterion/latest/criterion/)
   **Version:** 0.5+ (ou dernière version stable)
   ```

2. **Code source des benchmarks**:
   - Tous les fichiers `benches/benchmark.rs` importent et utilisent criterion
   - Configuration: `criterion_group!`, `criterion_main!`
   - Exécution: `cargo bench` (qui utilise criterion)

3. **Sortie réelle**:
   - Médiane + intervalle de confiance
   - 100 échantillons
   - Warmup: 3.0s
   - **→ C'est exactement ce que fait criterion**

---

## 📝 CORRECTIONS APPLIQUÉES

### 1. Suppression des affirmations fausses

**Fichiers modifiés:**
- ✅ `poc/datalog/results/comparison.md`: Supprimé "Exécuter benchmarks statistiques (criterion)" des actions à faire
- ✅ `poc/datalog/results/decision.md`: Supprimé "Exécuter benchmarks avec criterion" des actions à venir
- ✅ `poc/datalog/issues/commentaire-issue-4.md`: Mis à jour pour refléter la réalité

### 2. Documentation de la vérité

**Faits établis:**
- ✅ **criterion était utilisé DEPUIS LE DÉBUT** (comme exigé par ADR-002)
- ✅ **Les benchmarks fournissent déjà médiane + écart-type + ≥100 échantillons**
- ✅ **ADR-002 a correctement spécifié l'outil avant les mesures**

---

## 🎯 ERREUR À L'ORIGINE

**Erreur:** Dans les fichiers de résultats, j'ai marqué "benchmarks statistiques à venir" 
**alors que** les benchmarks étaient déjà exécutés avec criterion.

**Cause:**
- Mauvaise interprétation de la session précédente
- Copier-coller d'une todo list sans vérifier l'état réel
- Pas de vérification que les benchmarks utilisaient déjà criterion

**Correction:**
- Supprimer les mentions "à venir" / "À FAIRE" / "À VÉRIFIER"
- Documenter que criterion est déjà utilisé
- Ajouter cet ERRATUM pour traçabilité

---

## ✅ ÉTAT ACTUEL (VÉRIFIÉ)

| Élément | Statut | Preuve |
|---------|--------|--------|
| **Outil criterion** | ✅ **UTILISÉ** | Code source `benches/benchmark.rs` |
| **Médiane + écart-type** | ✅ **FOURNI** | Sortie criterion: `[min median max]` |
| **≥20 échantillons** | ✅ **100 ÉCHANTILLONS** | Sortie criterion: `Collecting 100 samples` |
| **Warmup** | ✅ **INCLUS** | Sortie criterion: `Warming up for 3.0s` |
| **Jeux 100/1K/10K** | ✅ **EXÉCUTÉS** | Sortie criterion pour small/medium/large |

---

## 📚 RÉFÉRENCES

- **ADR-002:** `poc/datalog/ADR-002-criteres-decision.md` (Section 3.1)
- **Code crepe:** `poc/datalog/crepe/poc/benches/benchmark.rs`
- **Code datafrog:** `poc/datalog/datafrog/benches/benchmark.rs`
- **Code Custom:** `poc/datalog/custom/benches/benchmark.rs`
- **Sortie criterion:** Voir `benchmark_crepe.md` (sortie brute incluse)
- **Issue:** #8

---

## 🏁 CONCLUSION

**Vérité:** **criterion était déjà utilisé** depuis le début. 

**Affirmation fausse:** "benchmarks statistiques à venir" était incorrecte.

**Action:**
- ✅ Affirmation fausse **supprimée** des fichiers de résultats
- ✅ **Cet ERRATUM** documenté pour traçabilité
- ✅ **ADR-002 reste inchangé** (correctement figé)

**Impact:** Aucun sur la décision (datafrog reste recommandé). 
C'est une correction **pour l'exactitude de l'archive**. 

---

**Statut:** ✅ **CORRIGÉ**  
**Date:** 2026-11-11  
**Type:** ERRATUM de documentation (pas de changement technique)
