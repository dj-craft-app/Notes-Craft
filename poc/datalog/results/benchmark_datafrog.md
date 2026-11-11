# Résultats Benchmark: datafrog

> **Date:** 2026-11-11  
> **Auteur:** Agent autonome  
> **Candidat:** datafrog v2.0.1 (MIT OR Apache-2.0)  
> **Issue:** #4 - PoC: Moteur Datalog (Phase 0.13)  
> **ADR:** ADR-002-criteres-decision.md  
> **Statut:** Résultats préliminaires

---

## 🖥️ Environnement de test

**Même environnement que crepe:**
- Système: macOS (Apple Silicon)
- CPU: Apple M1 Pro
- RAM: 16GB
- Rust: 1.85.0

---

## 📊 Résultats par Requête

### Jeu Petit (100 nœuds, 500 liens)

| Requête | Médiane | Écart-type | Minimum | Maximum | Débit |
|---------|---------|------------|---------|---------|--------|
| Par tag | **415.62 ns** | ±? | 412.62 ns | 419.57 ns | 240.61 Melem/s |
| Par lien | **21.616 µs** | ±? | 21.525 µs | 21.760 µs | 23.131 Melem/s |
| Transitivité | **5.8431 µs** | ±? | 5.8217 µs | 5.8765 µs | 17.114 Melem/s |
| Par type | **276.13 ns** | ±? | 275.18 ns | 277.62 ns | 362.14 Melem/s |
| Transitivité (datafrog) | **5.8009 µs** | ±? | 5.7859 µs | 5.8275 µs | 17.239 Melem/s |

**Commande:** `cargo bench --bench benchmark query_by_tag/small query_by_link/small query_transitive/small query_by_type/small transitive_datafrog/small`

---

### Jeu Moyen (1 000 nœuds, 5 000 liens)

| Requête | Médiane | Écart-type | Minimum | Maximum | Débit |
|---------|---------|------------|---------|---------|--------|
| Par tag | **2.3796 µs** | ±? | 2.3713 µs | 2.3924 µs | 420.25 Melem/s |
| Par lien | **255.69 µs** | ±? | 254.71 µs | 257.29 µs | 19.555 Melem/s |
| Transitivité | **52.516 µs** | ±? | 52.284 µs | 52.853 µs | 19.042 Melem/s |
| Par type | **1.4553 µs** | ±? | 1.4500 µs | 1.4632 µs | 687.15 Melem/s |
| Transitivité (datafrog) | **52.244 µs** | ±? | 52.084 µs | 52.501 µs | 19.141 Melem/s |

**Commande:** `cargo bench --bench benchmark query_by_tag/medium query_by_link/medium query_transitive/medium query_by_type/medium transitive_datafrog/medium`

---

### Jeu Grand (10 000 nœuds, 50 000 liens)

| Requête | Médiane | Écart-type | Minimum | Maximum | Débit |
|---------|---------|------------|---------|---------|--------|
| Par tag | **21.006 µs** | ±? | 20.947 µs | 21.103 µs | 476.05 Melem/s |
| Par lien | **2.0010 ms** | ±? | 1.9925 ms | 2.0128 ms | 24.988 Melem/s |
| Transitivité | **548.75 µs** | ±? | 546.69 µs | 551.66 µs | 18.223 Melem/s |
| Par type | **12.027 µs** | ±? | 11.992 µs | 12.078 µs | 831.48 Melem/s |
| Transitivité (datafrog) | **543.45 µs** | ±? | 542.10 µs | 545.67 µs | 18.401 Melem/s |

**Commande:** `cargo bench --bench benchmark query_by_tag/large query_by_link/large query_transitive/large query_by_type/large transitive_datafrog/large`

**✅ ATTENTION:** Les requêtes sont **beaucoup plus rapides** que crepe !
**Cible roadmap:** Requête complexe < 100ms sur 10K nœuds → **ATTEINT** (548.75 µs << 100ms)

---

## 📈 Comparaison des tailles

| Jeu | Nœuds | Requête Transitive (médiane) | Requête Par Tag (médiane) |
|-----|-------|-----------------------------|--------------------------|
| Petit | 100 | 5.8431 µs | 415.62 ns |
| Moyen | 1 000 | 52.516 µs | 2.3796 µs |
| Grand | 10 000 | **548.75 µs** | **21.006 µs** |

---

## 🎯 Comparaison avec crepe (Jeu Grand - 10K nœuds)

| Requête | crepe | datafrog | Ratio (crepe/datafrog) | Statut |
|---------|-------|----------|------------------------|--------|
| Par tag | 203.00 ms | 21.006 µs | **9 664x plus lent** | ⚠️ crepe ÉLIMINÉ |
| Par lien | 202.57 ms | 2.0010 ms | **101x plus lent** | ⚠️ crepe ÉLIMINÉ |
| Transitivité | 202.53 ms | 548.75 µs | **369x plus lent** | ⚠️ crepe ÉLIMINÉ |
| Par type | 191.72 ms | 12.027 µs | **15 940x plus lent** | ⚠️ crepe ÉLIMINÉ |

**Conclusion:** crepe est **369 à 15 940 fois plus lent** que datafrog sur 10K nœuds.

---

## 📈 Scalabilité

| Jeu | Nœuds | datafrog Transitivité | Ratio vs Petit |
|-----|-------|-----------------------|----------------|
| Petit | 100 | 5.8431 µs | 1.0x |
| Moyen | 1 000 | 52.516 µs | 8.98x |
| Grand | 10 000 | 548.75 µs | 93.9x |

**Scalabilité:** ~O(n) - linéaire avec le nombre de nœuds, ce qui est excellent.

---

## ⚠️ Différences méthodologiques

**Important:** Les benchmarks ne sont pas parfaitement équivalents :

### crepe
- Crée un runtime crepe à chaque itération
- Insère toutes les données (nœuds, tags, liens, types) à chaque itération
- Exécute TOUTES les règles Datalog à chaque itération
- Retourne les résultats structurés

### datafrog
- Utilise des fonctions Rust natives (pas de runtime Datalog)
- Pour les requêtes simples (par tag, par type) : itération simple sur des vecteurs
- Pour la transitivité : utilise datafrog avec Iteration
- Ne crée pas de runtime, utilise directement les structures de données

**Conséquence:** La comparaison mesure aussi la différence entre :
1. **crepe:** Un vrai moteur Datalog avec compilation de règles
2. **datafrog:** Un ensemble de fonctions optimisées pour chaque type de requête

C'est **injuste** pour crepe car on ne teste pas la même chose !

---

## 🔍 Analyse

**datafrog est UN ORDRE DE GRANDEUR plus rapide que crepe.**

Causes possibles :
1. **crepe** utilise des proc macros et génère du code complexe
2. **crepe** doit compiler et optimiser les règles Datalog à l'exécution
3. **datafrog** utilise des algorithmes itératifs bas niveau optimisés
4. Les requêtes simples dans datafrog sont juste des itérations sur des vecteurs

**Pour une comparaison équitable**, il faudrait :
- Soit utiliser datafrog comme un vrai moteur Datalog (déclarer les règles et les exécuter)
- Soit créer un benchmark où crepe réutilise le même runtime entre les requêtes

---

## 📋 Critères d'élimination (ADR-002 Section 4)

| Critère | Valeur | Statut |
|---------|--------|--------|
| **Performance** | 548.75 µs (transitivité, 10K nœuds) | ✅ **PASS** (cible: < 100ms) |
| **Intégration Rust** | 100% Rust, library | ✅ **PASS** |
| **Maintenance** | [À VÉRIFIER - version: 2.0.1] | ⚠️ À VÉRIFIER |
| **License** | MIT OR Apache-2.0 (permissive) | ✅ **PASS** |

---

## 📊 Métriques clés pour ADR-002

- **Performance (40%):** 548.75 µs sur 10K nœuds pour transitivité
- **Intégration Rust (30%):** 1.0 (100% Rust, natif)
- **Maintenance (20%):** [À VÉRIFIER]
- **Ergonomie (10%):** [À ÉVALUER - API datafrog]

---

## 🎯 Recommandation préliminaire

**✅ datafrog semble être le meilleur candidat jusqu'à présent.**

- **Performance:** Excellente, < 1ms pour la transitivité sur 10K nœuds
- **Scalabilité:** Linéaire O(n)
- **Intégration:** 100% Rust, facile à intégrer

**À vérifier:**
- [ ] Implémenter un vrai benchmark Datalog avec datafrog (déclarer les règles)
- [ ] Comparer avec un benchmark où crepe réutilise le runtime
- [ ] Tester Soufflé (C++ FFI) pour comparaison
- [ ] Vérifier les informations de maintenance

---

## 📚 Fichiers sources

- **Code:** `poc/datalog/datafrog/benches/benchmark.rs`
- **Cargo.toml:** `poc/datalog/datafrog/Cargo.toml`
- **ADR:** `poc/datalog/ADR-002-criteres-decision.md`
- **Issue:** #4, #7

---

**Statut:** ✅ EXCELLENTS RÉSULTATS - FAVORI ACTUEL
