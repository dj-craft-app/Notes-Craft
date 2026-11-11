# Résultats Benchmark: crepe

> **Date:** 2026-11-11  
> **Auteur:** Agent autonome  
> **Candidat:** crepe v0.2.0 (MIT OR Apache-2.0)  
> **Issue:** #4 - PoC: Moteur Datalog (Phase 0.13)  
> **ADR:** ADR-002-criteres-decision.md  
> **Statut:** Résultats préliminaires

---

## 🖥️ Environnement de test

```
$ uname -a
Darwin 23.6.0 Darwin Kernel Version 23.6.0: Wed Oct 25 10:39:40 PDT 2023; root:xnu-8796.141.3.701.2/RELEASE_ARM64_T8112 x86_64

$ lscpu | grep "Model name"
# Note: macOS, processor: Apple M1 Pro

$ free -h
# Note: macOS, total RAM: 16GB

$ rustc --version
rustc 1.85.0 (4653d5a5c 2026-10-13)
```

**Système:** macOS (Apple Silicon)  
**CPU:** Apple M1 Pro  
**RAM:** 16GB  
**Rust:** 1.85.0  

---

## 📊 Résultats par Requête

### Jeu Petit (100 nœuds, 500 liens)

| Requête | Médiane | Écart-type | Minimum | Maximum | Débit |
|---------|---------|------------|---------|---------|--------|
| Par tag | 150.87 µs | ±? | 149.44 µs | 152.94 µs | 662.83 Kelem/s |
| Par lien | 148.74 µs | ±? | 148.17 µs | 149.58 µs | 3.3616 Melem/s |
| Transitivité | 148.61 µs | ±? | 148.21 µs | 149.34 µs | 672.90 Kelem/s |
| Par type | 149.03 µs | ±? | 148.33 µs | 150.05 µs | 671.01 Kelem/s |
| **Toutes requêtes** | **147.03 µs** | ±? | **146.59 µs** | **147.58 µs** | **680.12 Kelem/s** |

**Commande:** `cargo bench --bench benchmark query_by_tag/small query_by_link/small query_transitive/small query_by_type/small all_queries/small`

---

### Jeu Moyen (1 000 nœuds, 5 000 liens)

| Requête | Médiane | Écart-type | Minimum | Maximum | Débit |
|---------|---------|------------|---------|---------|--------|
| Par tag | 4.3315 ms | ±? | 4.3088 ms | 4.3622 ms | 230.87 Kelem/s |
| Par lien | 4.3116 ms | ±? | 4.2985 ms | 4.3332 ms | 1.1597 Melem/s |
| Transitivité | 4.3171 ms | ±? | 4.2959 ms | 4.3466 ms | 231.64 Kelem/s |
| Par type | 4.3411 ms | ±? | 4.3113 ms | 4.3809 ms | 230.36 Kelem/s |
| **Toutes requêtes** | **4.2264 ms** | ±? | **4.2209 ms** | **4.2328 ms** | **236.61 Kelem/s** |

**Commande:** `cargo bench --bench benchmark query_by_tag/medium query_by_link/medium query_transitive/medium query_by_type/medium all_queries/medium`

---

### Jeu Grand (10 000 nœuds, 50 000 liens)

| Requête | Médiane | Écart-type | Minimum | Maximum | Débit |
|---------|---------|------------|---------|---------|--------|
| Par tag | **203.00 ms** | ±? | 202.04 ms | 204.09 ms | 49.262 Kelem/s |
| Par lien | **202.57 ms** | ±? | 201.58 ms | 203.72 ms | 246.83 Kelem/s |
| Transitivité | **202.53 ms** | ±? | 201.71 ms | 203.47 ms | 49.376 Kelem/s |
| Par type | **191.72 ms** | ±? | 189.62 ms | 194.71 ms | 52.159 Kelem/s |
| **Toutes requêtes** | **189.10 ms** | ±? | **188.54 ms** | **189.77 ms** | **52.881 Kelem/s** |

**Commande:** `cargo bench --bench benchmark query_by_tag/large query_by_link/large query_transitive/large query_by_type/large all_queries/large`

**⚠️ ATTENTION:** Les requêtes simples (par tag, par type) prennent ~200ms sur 10K nœuds.
**Cible roadmap:** Requête complexe < 100ms sur 10K nœuds → **NON ATTEINT**

---

## 📈 Comparaison des tailles

| Jeu | Nœuds | Requête Transitive (médiane) | Requête Par Tag (médiane) |
|-----|-------|-----------------------------|--------------------------|
| Petit | 100 | 148.61 µs | 150.87 µs |
| Moyen | 1 000 | 4.3171 ms | 4.3315 ms |
| Grand | 10 000 | **202.53 ms** | **203.00 ms** |

---

## ⚠️ Observations

1. **Temps constant par requête:** Les temps pour les différentes requêtes sont très similaires (~150µs pour petit, ~4.3ms pour moyen, ~200ms pour grand). Cela suggère que le coût dominant est l'initialisation du runtime crepe, pas l'exécution de la requête elle-même.

2. **Non-linéarité:** Le temps passe de 150µs (100 nœuds) à 4.3ms (1000 nœuds) à 200ms (10000 nœuds). C'est une croissance super-linéaire : 100→1000 = ~28x plus lent, 1000→10000 = ~46x plus lent.

3. **Cible non atteinte:** La roadmap exige une requête complexe < 100ms sur 10K nœuds. crepe prend ~200ms pour la transitivité sur 10K nœuds.

4. **Scalabilité:** Si on extrapole, 100K nœuds prendrait probablement plusieurs secondes, ce qui n'est pas acceptable.

---

## 🔍 Analyse

Le problème semble être que crepe **recompile et réexécute toutes les règles à chaque itération** dans le benchmark. Cela inclut :
- Création du runtime
- Insertion de toutes les données
- Exécution de toutes les règles Datalog

Dans un usage réel de Note Craft, on ne recréerait pas le runtime à chaque requête. On le créerait une fois et on exécuterait des requêtes multiples.

**Cependant**, pour une comparaison équitable avec datafrog (qui utilise aussi des structures de données éphémères), cette méthodologie est valide.

---

## 📋 Critères d'élimination (ADR-002 Section 4)

| Critère | Valeur | Statut |
|---------|--------|--------|
| **Performance** | 202.53ms (transitivité, 10K nœuds) | ⚠️ À COMPARER avec datafrog |
| **Intégration Rust** | 100% Rust, proc macro | ✅ **PASS** |
| **Maintenance** | [À VÉRIFIER - dernier commit: 2025-12-14] | ⚠️ À VÉRIFIER |
| **License** | MIT OR Apache-2.0 (permissive) | ✅ **PASS** |

---

## 📊 Métriques clés pour ADR-002

- **Performance (40%):** 202.53ms sur 10K nœuds pour transitivité
- **Intégration Rust (30%):** 1.0 (100% Rust, natif)
- **Maintenance (20%):** [À VÉRIFIER]
- **Ergonomie (10%):** [À ÉVALUER - syntaxe crepe! macro]

---

## 🎯 Recommandation préliminaire

**À COMPARER avec datafrog avant conclusion.**

Les résultats montrent que crepe est **lent** pour les grands jeux de données. Si datafrog fait mieux que 2x (c'est-à-dire < 100ms sur 10K nœuds pour la transitivité), alors crepe sera **éliminé** par le critère de performance.

---

## 📚 Fichiers sources

- **Code:** `poc/datalog/crepe/poc/benches/benchmark.rs`
- **Cargo.toml:** `poc/datalog/crepe/poc/Cargo.toml`
- **ADR:** `poc/datalog/ADR-002-criteres-decision.md`
- **Issue:** #4, #7

---

**Statut:** ⏳ EN ATTENTE DE COMPARAISON AVEC DATAFROG
