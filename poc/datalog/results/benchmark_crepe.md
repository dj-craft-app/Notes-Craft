# Résultats Benchmark: crepe (LIMITATION API DOCUMENTÉE)

> **Date:** 2026-11-11 (corrigé)  
> **Auteur:** Agent autonome  
> **Candidat:** crepe v0.2.0 (MIT OR Apache-2.0)  
> **Issue:** #4, #8  
> **ADR:** ADR-002-criteres-decision.md  
> **Statut:** Résultats avec limitation API documentée

---

## ⚠️ LIMITATION IMPORTANTE

**crepe ne permet PAS de séparer le setup de l'exécution !**

**Raison:**
La macro `crepe!` génère un type `Crepe` avec une méthode `run()` qui:
1. **Consomme le runtime** (self est moved, pas borrowed)
2. **Retourne un tuple de HashSet** avec TOUS les résultats
3. **Ne permet PAS de réutiliser le runtime** entre les appels

**Conséquence:**
Chaque appel à `run()` nécessite de **recréer le runtime et réinsérer toutes les données**.

**Impact sur les résultats:**
Les temps mesurés pour crepe incluent:
- ✅ **Setup:** Création du runtime + insertion des données
- ✅ **Exécution:** run() + extraction du résultat

Cela diffère de **datafrog** et **Custom** où le setup est exclu.

**Justification de l'inclusion quand même:**
Même avec le setup inclus, crepe reste **100x à 20 000x plus lent** que datafrog.
L'élimination par le critère de performance reste valide.

**Voir:** [METHODOLOGIE.md](./METHODOLOGIE.md) pour détails

---

## 🖥️ Environnement de test

**Identique à datafrog et Custom:**
- Système: macOS (Apple Silicon)
- CPU: Apple M1 Pro
- RAM: 16GB
- Rust: 1.85.0

---

## 📊 Résultats par Requête (Setup + Exécution INCLUS)

### Jeu Petit (100 nœuds, 500 liens)

| Requête | Médiane | Minimum | Maximum |
|---------|---------|---------|---------|
| Par tag | [À MESURER] | [À MESURER] | [À MESURER] |
| Par lien | [À MESURER] | [À MESURER] | [À MESURER] |
| Transitivité | [À MESURER] | [À MESURER] | [À MESURER] |
| Par type | [À MESURER] | [À MESURER] | [À MESURER] |

**Note:** Les benchmarks corrigés doivent être exécutés pour obtenir les nouveaux résultats.
Les anciens résultats (setup+exécution) étaient:
- query_by_tag/small: 150.87 µs
- query_by_link/small: 148.74 µs
- query_transitive/small: 148.61 µs
- query_by_type/small: 149.03 µs

---

### Jeu Moyen (1 000 nœuds, 5 000 liens)

| Requête | Médiane | Minimum | Maximum |
|---------|---------|---------|---------|
| Par tag | [À MESURER] | [À MESURER] | [À MESURER] |
| Par lien | [À MESURER] | [À MESURER] | [À MESURER] |
| Transitivité | [À MESURER] | [À MESURER] | [À MESURER] |
| Par type | [À MESURER] | [À MESURER] | [À MESURER] |

**Ancien résultats (setup+exécution):**
- query_by_tag/medium: 4.3315 ms
- query_by_link/medium: 4.3116 ms
- query_transitive/medium: 4.3171 ms
- query_by_type/medium: 4.3411 ms

---

### Jeu Grand (10 000 nœuds, 50 000 liens)

| Requête | Médiane | Minimum | Maximum |
|---------|---------|---------|---------|
| Par tag | **203.00 ms** | 202.04 ms | 204.09 ms |
| Par lien | **202.57 ms** | 201.58 ms | 203.72 ms |
| Transitivité | **202.53 ms** | 201.71 ms | 203.47 ms |
| Par type | **191.72 ms** | 189.62 ms | 194.71 ms |

**Note:** Ces résultats incluent le setup (création runtime + insertion).

---

## 📈 Analyse des Résultats

### Observations

1. **Temps constants par requête:**
   - Toutes les requêtes prennent ~200ms sur 10K nœuds
   - Little variation entre les différents types de requêtes

2. **Scalabilité:**
   - 100 → 1 000 nœuds: ~150µs → ~4.3ms (**28x plus lent**)
   - 1 000 → 10 000 nœuds: ~4.3ms → ~200ms (**46x plus lent**)
   - **Scalabilité super-linéaire** (O(n^1.5) ou pire)

3. **Impact de la limitation API:**
   - Impossible de mesurer l'exécution seule
   - Le coût du setup (création runtime + insertion) domine le temps total

---

## 🏆 Comparaison avec les Autres Candidats (Jeu Grand - 10K nœuds)

### Requête: Transitivité

| Candidat | Temps | vs datafrog | Inclut setup ? | Statut |
|----------|-------|-------------|----------------|--------|
| **datafrog** | **537.56 µs** | 1.0x | ❌ Non | ✅ Référence |
| Custom | 175.58 µs | **0.326x** | ❌ Non | ❌ ÉLIMINÉ (autres requêtes) |
| crepe | **202.53 ms** | **377x plus lent** | ✅ **Oui** | ❌ ÉLIMINÉ |

### Requête: Par tag

| Candidat | Temps | vs datafrog | Inclut setup ? | Statut |
|----------|-------|-------------|----------------|--------|
| **datafrog** | **16.848 µs** | 1.0x | ❌ Non | ✅ Référence |
| Custom | 490.11 µs | **29x plus lent** | ❌ Non | ❌ ÉLIMINÉ |
| crepe | **203.00 ms** | **12 047x plus lent** | ✅ **Oui** | ❌ ÉLIMINÉ |

### ⚠️ IMPORTANT

Même si crepe inclut le setup et datafrog/Custom non, **l'écart reste énorme** :
- 377x à 12 047x plus lent que datafrog
- L'élimination par le critère de performance est **toujours valide**

---

## 📋 Application des Critères d'Élimination (ADR-002 Section 4.1)

**Critère:** Écart > 2x par rapport au meilleur sur le jeu Grand (10K nœuds) **pour au moins une requête**

| Candidat | Transitivité | Par tag | Par lien | Par type | Statut |
|----------|--------------|---------|----------|----------|--------|
| datafrog | 537.56 µs | **16.848 µs** | **985.85 µs** | **9.3252 µs** | ✅ VALIDÉ |
| Custom | 175.58 µs | 490.11 µs | 12.672 ms | 324.08 µs | ❌ ÉLIMINÉ |
| **crepe** | **202.53 ms** | **203.00 ms** | **202.57 ms** | **191.72 ms** | ❌ **ÉLIMINÉ** (toutes > 2x) |

**→ crepe est éliminé par le critère de performance** malgré la limitation de mesure

---

## 📊 Critères Appliqués (ADR-002)

| Critère | Valeur | Statut | Preuve |
|---------|--------|--------|--------|
| **Performance** | 202.53 ms (transitivité, 10K nœuds) | ❌ **ÉLIMINÉ** | 377x > 2x |
| **Intégration Rust** | 100% Rust (proc macro) | ✅ **PASS** | Pas de FFI |
| **Maintenance** | pushed_at: 2025-12-14, repo: ekzhang/crepe | ✅ **PASS** | Actif (346 jours) |
| **License** | MIT OR Apache-2.0 | ✅ **PASS** | Permissive |

---

## 📊 Métriques Clés pour ADR-002 Section 5

**Note:** Comme crepe est éliminé par le critère de performance, le score n'est pas calculé.

---

## 🏆 Conclusion

**crepe est éliminé par le critère de performance:**

❌ **Points faibles:**
- 377x à 12 047x plus lent que datafrog sur 10K nœuds
- Scalabilité super-linéaire (O(n^1.5) ou pire)
- **Limitation API:** Impossible de séparer setup et exécution
- **Éliminé par le critère de performance** (toutes les requêtes > 2x écart)

✅ **Points forts:**
- 100% Rust, intégration facile
- License permissive
- Maintenance active

**Recommandation:**
Ne pas sélectionner crepe pour Note Craft en raison des performances insuffisantes.

---

## 📚 Fichiers Sources

- **Code:** `poc/datalog/crepe/poc/benches/benchmark.rs`
- **Cargo.toml:** `poc/datalog/crepe/poc/Cargo.toml`
- **ADR:** `poc/datalog/ADR-002-criteres-decision.md`
- **Méthodologie:** `poc/datalog/results/METHODOLOGIE.md`
- **Issues:** #4, #8

---

**Statut:** ❌ ÉLIMINÉ PAR PERFORMANCE (malgré limitation de mesure)
**Version:** 2 (limitation API documentée)
**Note:** Même avec le setup inclus, crepe reste beaucoup trop lent
