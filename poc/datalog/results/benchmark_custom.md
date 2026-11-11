# Résultats Benchmark: Custom (SQLite)

> **Date:** 2026-11-11  
> **Auteur:** Agent autonome  
> **Candidat:** Custom SQLite (rusqlite v0.31, MIT)  
> **Issue:** #4 - PoC: Moteur Datalog (Phase 0.13)  
> **ADR:** ADR-002-criteres-decision.md  
> **Statut:** Résultats préliminaires

---

## 🖥️ Environnement de test

**Même environnement que crepe et datafrog:**
- Système: macOS (Apple Silicon)
- CPU: Apple M1 Pro
- RAM: 16GB
- Rust: 1.85.0

---

## 📊 Résultats par Requête

### Jeu Petit (100 nœuds, 500 liens)

| Requête | Médiane | Écart-type | Minimum | Maximum | Débit |
|---------|---------|------------|---------|---------|--------|
| Par tag | 1.2916 ms | ±? | 1.2902 ms | 1.2931 ms | 77.420 Kelem/s |
| Par lien | 1.3869 ms | ±? | 1.3851 ms | 1.3890 ms | 360.51 Kelem/s |
| Transitivité | [À COMPLÉTER] | ±? | [À COMPLÉTER] | [À COMPLÉTER] | [À COMPLÉTER] |
| Par type | [À COMPLÉTER] | ±? | [À COMPLÉTER] | [À COMPLÉTER] | [À COMPLÉTER] |

---

### Jeu Moyen (1 000 nœuds, 5 000 liens)

| Requête | Médiane | Écart-type | Minimum | Maximum | Débit |
|---------|---------|------------|---------|---------|--------|
| Par tag | 13.672 ms | ±? | 13.645 ms | 13.705 ms | 73.141 Kelem/s |
| Par lien | 14.862 ms | ±? | 14.782 ms | 14.959 ms | 336.43 Kelem/s |
| Transitivité | [À COMPLÉTER] | ±? | [À COMPLÉTER] | [À COMPLÉTER] | [À COMPLÉTER] |
| Par type | [À COMPLÉTER] | ±? | [À COMPLÉTER] | [À COMPLÉTER] | [À COMPLÉTER] |

---

### Jeu Grand (10 000 nœuds, 50 000 liens)

| Requête | Médiane | Écart-type | Minimum | Maximum | Débit |
|---------|---------|------------|---------|---------|--------|
| Par tag | **139.97 ms** | ±? | 139.85 ms | 140.12 ms | 71.441 Kelem/s |
| Par lien | **152.82 ms** | ±? | 152.61 ms | 153.05 ms | 327.18 Kelem/s |
| Transitivité | **150.48 ms** | ±? | 148.03 ms | 153.29 ms | 66.454 Kelem/s |
| Par type | **146.08 ms** | ±? | 145.46 ms | 146.83 ms | 68.455 Kelem/s |

**Commande:** `cargo bench --bench benchmark`

**⚠️ ATTENTION:** Les requêtes sont plus rapides que crepe mais **beaucoup plus lentes que datafrog**.
**Cible roadmap:** Requête complexe < 100ms sur 10K nœuds → **NON ATTEINT** (150.48 ms)

---

## 📈 Comparaison des tailles

| Jeu | Nœuds | Requête Transitive (médiane) | Requête Par Tag (médiane) |
|-----|-------|-----------------------------|--------------------------|
| Petit | 100 | [À COMPLÉTER] | 1.2916 ms |
| Moyen | 1 000 | [À COMPLÉTER] | 13.672 ms |
| Grand | 10 000 | **150.48 ms** | **139.97 ms** |

---

## 🔍 Analyse

**Custom (SQLite) est un compromis intéressant :**

### Points forts ✅
- **Plus rapide que crepe:** ~25% plus rapide sur la transitivité (150ms vs 202ms)
- **100% Rust:** Utilise rusqlite (wrapper Rust autour de SQLite)
- **License permissive:** MIT
- **Maturity:** SQLite est un projet mature et stable

### Points faibles ❌
- **Plus lent que datafrog:** 274x plus lent sur la transitivité (150ms vs 548µs)
- **Ne rencontre pas la cible roadmap:** 150ms > 100ms pour requête complexe sur 10K nœuds
- **Overhead SQLite:** Initialisation de la base de données et exécution SQL

### Scalabilité
La scalabilité semble raisonnable (linéaire ou légèrement super-linéaire) mais pas aussi bonne que datafrog.

---

## 📊 Comparaison avec les autres candidats (Jeu Grand - 10K nœuds)

| Requête | crepe | Custom | datafrog | Ratio (Custom/datafrog) |
|---------|-------|--------|----------|-------------------------|
| Par tag | 203.00 ms | **139.97 ms** | **21.006 µs** | **6 663x plus lent** |
| Par lien | 202.57 ms | **152.82 ms** | **2.0010 ms** | **76x plus lent** |
| Transitivité | 202.53 ms | **150.48 ms** | **548.75 µs** | **274x plus lent** |
| Par type | 191.72 ms | **146.08 ms** | **12.027 µs** | **12 144x plus lent** |

**Conclusion:** Custom est **274 à 12 000x plus lent que datafrog** sur 10K nœuds.

---

## 📋 Application des Critères d'Élimination (ADR-002 Section 4)

| Critère | Valeur | Statut |
|---------|--------|--------|
| **Performance** | 150.48 ms (transitivité, 10K nœuds) | ❌ **ÉLIMINÉ** (>2x écart vs datafrog) |
| **Intégration Rust** | 100% Rust (via rusqlite) | ✅ **PASS** |
| **Maintenance** | [À VÉRIFIER - SQLite: très actif] | ⚠️ À VÉRIFIER |
| **License** | MIT (permissive) | ✅ **PASS** |

**→ Custom est ÉLIMINÉ par le critère de performance** (150ms > 100ms cible, et >2x écart vs datafrog)

---

## 📚 Implémentation

- **Approche:** Utilisation de SQLite avec CTE (Common Table Expression) pour les requêtes récursives
- **Avantages:** Simple à implémenter, SQL standard, persistance possible
- **Inconvénients:** Overhead SQL, moins optimisé que les solutions natives

---

## 🏆 Positionnement

**Classement des 3 candidats testés:**

1. **🥇 datafrog** - 548.75 µs (transitivité 10K) ✅ **VALIDÉ**
2. **🥈 Custom (SQLite)** - 150.48 ms (transitivité 10K) ❌ **ÉLIMINÉ**
3. **🥉 crepe** - 202.53 ms (transitivité 10K) ❌ **ÉLIMINÉ**

---

## 📚 Fichiers sources

- **Code:** `poc/datalog/custom/src/lib.rs`
- **Benchmark:** `poc/datalog/custom/benches/benchmark.rs`
- **Cargo.toml:** `poc/datalog/custom/Cargo.toml`
- **ADR:** `poc/datalog/ADR-002-criteres-decision.md`
- **Issue:** #4, #7

---

**Statut:** ❌ ÉLIMINÉ PAR PERFORMANCE
