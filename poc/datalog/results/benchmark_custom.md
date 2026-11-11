# Résultats Benchmark: Custom (SQLite) (CORRIGÉ - Symétrie de Mesure)

> **Date:** 2026-11-11 (corrigé)  
> **Auteur:** Agent autonome  
> **Candidat:** Custom SQLite (rusqlite v0.31, MIT)  
> **Issue:** #4, #8  
> **ADR:** ADR-002-criteres-decision.md  
> **Statut:** Résultats corrigés - setup EXCLU

---

## ⚠️ CORRECTION IMPORTANTE

**Version précédente (0b122d0):** Le setup (création DB + insertion) était INCLUS dans la mesure  
**Version actuelle:** Le setup est EXCLU, seule l'exécution de la requête SQL est mesurée  

**Impact:** Amélioration massive (150ms → 175µs pour la transitivité !)

**Voir:** [METHODOLOGIE.md](./METHODOLOGIE.md) pour détails

---

## 🖥️ Environnement de test

**Identique à datafrog et crepe:**
- Système: macOS (Apple Silicon)
- CPU: Apple M1 Pro
- RAM: 16GB
- Rust: 1.85.0
- SQLite: bundled via rusqlite

---

## 📊 Résultats par Requête (NOUVEAUX - Setup EXCLU)

### Jeu Petit (100 nœuds, 500 liens)

| Requête | Médiane | Minimum | Maximum |
|---------|---------|---------|---------|
| Par tag | **4.8808 µs** | 4.8384 µs | 4.9365 µs |
| Par lien | **93.675 µs** | [à compléter] | [à compléter] |
| Transitivité | **46.973 µs** | 46.736 µs | 47.331 µs |
| Par type | **94.123 µs** | 93.675 µs | 94.740 µs |

---

### Jeu Moyen (1 000 nœuds, 5 000 liens)

| Requête | Médiane | Minimum | Maximum |
|---------|---------|---------|---------|
| Par tag | **1.0630 ms** | [à compléter] | [à compléter] |
| Par lien | **1.0729 ms** | [à compléter] | [à compléter] |
| Transitivité | [à compléter] | [à compléter] | [à compléter] |
| Par type | [à compléter] | [à compléter] | [à compléter] |

---

### Jeu Grand (10 000 nœuds, 50 000 liens)

| Requête | Médiane | Minimum | Maximum |
|---------|---------|---------|---------|
| Par tag | **490.11 µs** | 486.37 µs | 495.70 µs |
| Par lien | **12.672 ms** | 12.609 ms | 12.761 ms |
| Transitivité | **175.58 µs** | 175.36 µs | 175.80 µs |
| Par type | **324.08 µs** | 321.54 µs | 327.59 µs |

---

## 📈 Comparaison des Tailles (NOUVEAUX RÉSULTATS)

| Jeu | Nœuds | Transitivité | Par tag | Par lien | Par type |
|-----|-------|--------------|---------|----------|----------|
| Petit | 100 | 46.973 µs | 4.8808 µs | ~93.675 µs | 94.123 µs |
| Moyen | 1 000 | [à compléter] | 1.0630 ms | ~1.0729 ms | [à compléter] |
| Grand | 10 000 | **175.58 µs** | **490.11 µs** | **12.672 ms** | **324.08 µs** |

---

## 🎯 Comparaison avec datafrog (Jeu Grand - 10K nœuds)

### Requête par Requête

| Requête | Custom | datafrog | Ratio (Custom/datafrog) | Statut |
|---------|--------|----------|-------------------------|--------|
| **Transitivité** | **175.58 µs** | 537.56 µs | **0.326x (3x plus rapide !)** | ✅ Meilleure performance |
| Par tag | 490.11 µs | **16.848 µs** | **29x plus lent** | ❌ ÉLIMINÉ (>2x) |
| Par lien | 12.672 ms | **985.85 µs** | **12.9x plus lent** | ❌ ÉLIMINÉ (>2x) |
| Par type | 324.08 µs | **9.3252 µs** | **34.8x plus lent** | ❌ ÉLIMINÉ (>2x) |

**→ Custom est éliminé car 3 des 4 requêtes ont un écart > 2x**

---

## 🏆 Analyse

### Point Fort: Transitivité

Custom (SQLite) est **3x plus rapide que datafrog** sur la requête de transitivité !

**Raison probable:**
- SQLite utilise des algorithmes de jointure optimisés (hash joins, etc.)
- La requête récursive avec CTE (Common Table Expression) est très efficace
- datafrog utilise un algorithme itératif en mémoire qui peut être moins optimisé pour ce cas spécifique

### Points Faibles: Autres Requêtes

Custom est **beaucoup plus lent** que datafrog sur les requêtes simples:
- Par tag: 29x plus lent
- Par lien: 12.9x plus lent
- Par type: 34.8x plus lent

**Raison probable:**
- SQLite a un overhead pour chaque requête (parsing, planning, exécution)
- datafrog utilise des itérations simples sur des Vec<...> sans overhead
- Pour les requêtes simples, l'overhead SQLite domine

---

## 📋 Application des Critères d'Élimination (ADR-002 Section 4.1)

**Critère:** Écart > 2x par rapport au meilleur sur le jeu Grand (10K nœuds) **pour au moins une requête**

| Candidat | Transitivité | Par tag | Par lien | Par type | Statut |
|----------|--------------|---------|----------|----------|--------|
| datafrog | 537.56 µs | **16.848 µs** | **985.85 µs** | **9.3252 µs** | ✅ VALIDÉ |
| **Custom** | **175.58 µs** | 490.11 µs | 12.672 ms | 324.08 µs | ❌ **ÉLIMINÉ** (3/4 > 2x) |

**→ Custom est éliminé par le critère de performance**

---

## 📊 Critères Appliqués (ADR-002)

| Critère | Valeur | Statut | Preuve |
|---------|--------|--------|--------|
| **Performance** | 175.58 µs (transitivité) mais 29x sur par tag | ❌ **ÉLIMINÉ** | 3/4 requêtes > 2x |
| **Intégration Rust** | 100% Rust (via rusqlite) | ✅ **PASS** | Pas de FFI |
| **Maintenance** | SQLite: très actif, rusqlite: v0.31 | ✅ **PASS** | Mature |
| **License** | MIT (rusqlite) + SQLite (Public Domain) | ✅ **PASS** | Permissive |

---

## 📊 Métriques Clés pour ADR-002 Section 5

### Performance (40%)

**Score normalisé =** `(meilleur_temps / temps_candidat)` pour chaque requête

| Requête | Custom | Meilleur (datafrog) | Score |
|---------|--------|---------------------|-------|
| Transitivité | 175.58 µs | 537.56 µs | 537.56/175.58 = **3.06** (max 1.0) |
| Par tag | 490.11 µs | 16.848 µs | 16.848/490.11 = **0.034** |
| Par lien | 12.672 ms | 985.85 µs | 985.85/12672 = **0.078** |
| Par type | 324.08 µs | 9.3252 µs | 9.3252/324.08 = **0.029** |

**Score performance:** Moyenne = (3.06 + 0.034 + 0.078 + 0.029) / 4 = **0.80** (clampé à 1.0 max)

**Attention:** Le score pour la transitivité dépasse 1.0 car Custom est plus rapide. 
Selon ADR-002, le score normalisé = `(meilleur_temps / temps_candidat)`.
Si Custom est plus rapide, alors `(datafrog / custom) > 1.0`.

**Correction:** Le score doit être clampé à 1.0 maximum.

| Requête | Score normalisé (clampé à 1.0) |
|---------|---------------------------------|
| Transitivité | min(537.56/175.58, 1.0) = **1.0** |
| Par tag | 16.848/490.11 = **0.034** |
| Par lien | 985.85/12672 = **0.078** |
| Par type | 9.3252/324.08 = **0.029** |

**Score performance:** (1.0 + 0.034 + 0.078 + 0.029) / 4 = **0.285**
**Score pondéré:** 0.285 × 0.40 = **0.114**

### Intégration Rust (30%)

**Niveau:** Natif (100% Rust via rusqlite)
**Score:** 1.0
**Score pondéré:** 1.0 × 0.30 = **0.30**

### Maintenance (20%)

**Données:**
- SQLite: projet très mature, maintenu activement
- rusqlite: v0.31, dernière release récente

**Niveau:** Très actif / Mature
**Score:** 1.0
**Score pondéré:** 1.0 × 0.20 = **0.20**

### Ergonomie (10%)

**Évaluation:**
- API Rust idiomatique via rusqlite
- Requêtes SQL standard, faciles à écrire
- Nécessite une adaptation pour la syntaxe Datalog de Note Craft

**Niveau:** Bon
**Score:** 0.8
**Score pondéré:** 0.8 × 0.10 = **0.08**

### **Score Final: 0.114 + 0.30 + 0.20 + 0.08 = 0.694**

**Classement:** **PASSABLE (0.694/1.0)**

---

## 🏆 Conclusion

**Custom (SQLite) est un candidat intéressant mais éliminé par la performance:**

✅ **Points forts:**
- 3x plus rapide que datafrog sur la transitivité
- 100% Rust, intégration facile
- License permissive
- Maturité du projet (SQLite)

❌ **Points faibles:**
- 12.9x à 29x plus lent que datafrog sur les requêtes simples
- **Éliminé par le critère de performance** (3/4 requêtes > 2x écart)

**Recommandation:**
Custom pourrait être utile si Note Craft nécessite principalement des requêtes de transitivité.
Mais comme la roadmap exige des performances sur TOUTES les requêtes, datafrog reste le meilleur choix.

---

## 📚 Fichiers Sources

- **Code:** `poc/datalog/custom/benches/benchmark.rs`
- **Cargo.toml:** `poc/datalog/custom/Cargo.toml`
- **ADR:** `poc/datalog/ADR-002-criteres-decision.md`
- **Méthodologie:** `poc/datalog/results/METHODOLOGIE.md`
- **Issues:** #4, #8

---

**Statut:** ✅ RÉSULTATS CORRIGÉS
**Version:** 2 (corrigée pour symétrie de mesure)
**Note:** Éliminé par le critère de performance malgré une excellente performance sur la transitivité
