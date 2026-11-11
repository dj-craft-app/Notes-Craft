# PoC Moteur Datalog - Phase 0.13

> **Issue:** #4  
> **Source:** docs/roadmap/roadmap_v3.md:663  
> **Voir aussi:** docs/history/fai_001_audit.md  
> **Responsable:** Agent autonome  
> **Date:** 2026-11-11

---

## 1. CONTEXTE

Ce PoC a pour objectif d'évaluer et de sélectionner un moteur Datalog pour Note Craft.

**Problème FAI-001:** La complexité d'un moteur Datalog Custom a été sous-estimée dans les versions précédentes de la roadmap. Une solution alternative (SurrealDB) a été proposée mais **invalidée** car SurrealDB n'est PAS un moteur Datalog (voir [fai_001_audit.md](../../docs/history/fai_001_audit.md)).

**Solution:** Évaluer 3 candidats valides pour implémenter le système de requêtes Datalog de Note Craft.

---

## 2. CANDIDATS À ÉVALUER

| Candidat | Type | Langage | Statut | Lien | Source |
|----------|------|---------|--------|------|--------|
| **crepe** | Moteur Datalog natif | Rust | À évaluer | https://github.com/crepe-rs/crepe | roadmap_v3.md:670 |
| **Soufflé** | Moteur Datalog standard | C++ | À évaluer | https://souffle-lang.github.io | roadmap_v3.md:673 |
| **Custom SQLite** | Implementation sur mesure | Rust | À évaluer | - | roadmap_v3.md:676 |

---

## 3. CRITÈRES D'ÉVALUATION

### 3.1 Critères Techniques

| Critère | Poids | Description | Méthode de mesure |
|---------|-------|-------------|-------------------|
| **Performance** | 30% | Vitesse d'exécution des requêtes | Requêtes/s, latence (ms) |
| **Mémoire** | 20% | Consommation mémoire | MB utilisée pour 10K nœuds |
| **Intégration Rust** | 25% | Facilité d'intégration dans Rust | Complexité FFI, nombre de dépendances |
| **Fonctionnalités** | 15% | Support des features Datalog nécessaires | % des features Logseq supportées |
| **Maturité** | 10% | Stabilité et maintenance | Version, communauté, documentation |

### 3.2 Critères d'Acceptation (Issue #4)

- [ ] **PoC fonctionnel** : Chaque candidat doit être testé avec des requêtes Datalog basiques
- [ ] **Comparaison documentée** : Rapport complet avec benchmarks et recommandation
- [ ] **Évaluation complète** : Les 3 candidats doivent être évalués
- [ ] **Benchmark de performance** : Mesures de requêtes/s, mémoire, latence
- [ ] **Benchmark d'intégration** : Évaluation de la facilité d'utilisation dans Rust
- [ ] **Décision documentée** : Justification du choix final

---

## 4. STRUCTURE DU POC

```
poc/datalog/
├── README.md                    # Ce fichier
├── crepe/
│   ├── evaluation.md            # Évaluation de crepe
│   ├── Cargo.toml               # Fichier Cargo si applicable
│   ├── src/                     # Code source
│   └── results/                 # Résultats des tests
├── soufflé/
│   ├── evaluation.md            # Évaluation de Soufflé
│   ├── bindings/                # Bindings Rust si créés
│   └── results/                 # Résultats des tests
├── custom/
│   ├── evaluation.md            # Évaluation de l'implémentation custom
│   ├── Cargo.toml
│   ├── src/
│   │   └── lib.rs               # Implémentation du parser/engine
│   └── results/
├── benchmark/
│   ├── common/                  # Code commun aux benchmarks
│   │   └── queries.datalog      # Requêtes de test
│   ├── crepe_bench.rs           # Benchmark crepe
│   ├── soufflé_bench.rs         # Benchmark Soufflé
│   └── custom_bench.rs          # Benchmark custom
└── results/
    ├── comparison.md            # Comparaison finale
    └── decision.md              # Décision et justification
```

---

## 5. MÉTHODOLOGIE

### 5.1 Préparation

1. **Créer des jeux de données de test**
   - Petit: 100 nœuds, 500 relations
   - Moyen: 1,000 nœuds, 5,000 relations
   - Grand: 10,000 nœuds, 50,000 relations
   - Format: Fichiers Datalog basés sur des extraits de graphe Note Craft

2. **Définir des requêtes de test**
   - Requêtes simples: `:find ?x :where [?x :tag ?t]`
   - Requêtes complexes: `:find ?x ?y :where [?x :link ?y] [?y :tag "important"]`
   - Requêtes récursives: `:find ?x :where (or [?x :link ?y] [?y :link ?x])`

### 5.2 Évaluation par Candidat

Pour chaque candidat, effectuer :

1. **Installation et configuration**
   - Documenter les étapes
   - Noter les dépendances
   - Évaluer la complexité

2. **Implémentation des requêtes de test**
   - Adapter les requêtes au format du candidat
   - Vérifier la compatibilité avec la syntaxe Logseq

3. **Exécution des benchmarks**
   - Mesurer le temps d'exécution
   - Mesurer la consommation mémoire
   - Noter les erreurs ou limitations

4. **Évaluation qualitative**
   - Facilité d'utilisation
   - Qualité de la documentation
   - Support communautaire

### 5.3 Comparaison et Décision

1. **Analyser les résultats**
   - Comparer les performances
   - Comparer la consommation mémoire
   - Comparer la facilité d'intégration

2. **Identifier les trade-offs**
   - Performance vs Complexité
   - Rust natif vs FFI
   - Maturité vs Flexibilité

3. **Recommander un candidat**
   - Justifier le choix
   - Documenter les alternatives
   - Proposer un plan d'implémentation

---

## 6. RÉSULTATS ATTENDUS

### 6.1 Livrables

- [x] `poc/datalog/ADR-001-candidats-datalog.md` - ADR pour le changement de candidat
- [x] `poc/datalog/README.md` - Ce document
- [x] `poc/datalog/benchmark/common/queries.datalog` - Requêtes de test
- [x] `poc/datalog/crepe/README.md` - Documentation de l'indisponibilité de crepe
- [x] `poc/datalog/crepe/evaluation.md` - Template d'évaluation
- [x] `poc/datalog/datafrog/` - PoC datafrog avec code fonctionnel
- [x] `poc/datalog/datafrog/evaluation.md` - Évaluation de datafrog (en cours)
- [ ] `poc/datalog/souffle/evaluation.md` - Rapport d'évaluation Soufflé
- [ ] `poc/datalog/custom/evaluation.md` - Rapport d'évaluation custom
- [ ] `poc/datalog/custom/src/lib.rs` - Prototype d'implémentation custom
- [ ] `poc/datalog/benchmark/*.rs` - Scripts de benchmark
- [ ] `poc/datalog/results/comparison.md` - Comparaison finale
- [ ] `poc/datalog/results/decision.md` - Décision et justification

### 6.2 Métriques

**Résultats préliminaires (datafrog - 100 nœuds):**

| Métrique | Valeur | Statut | Notes |
|----------|--------|--------|-------|
| Requêtes simples (par tag) | 6.541µs | ✅ Très rapide | |
| Jointure (liens + tags) | 400.833µs | ⚠️ Lent (algo naif) | À optimiser |
| Récursif (algo naif) | 25.5µs | ✅ Rapide | |
| Récursif (datafrog) | 99.292µs | ⚠️ Overhead visible | À tester sur jeux plus grands |

**Métriques cibles (à valider):**

| Métrique | Valeur cible | datafrog | Soufflé | Custom |
|----------|---------------|---------|---------|--------|
| Requêtes simples (1K nœuds) | < 10ms | [À VÉRIFIER] | [À VÉRIFIER] | [À VÉRIFIER] |
| Requêtes complexes (1K nœuds) | < 100ms | [À VÉRIFIER] | [À VÉRIFIER] | [À VÉRIFIER] |
| Requêtes récursives (1K nœuds) | < 200ms | [À VÉRIFIER] | [À VÉRIFIER] | [À VÉRIFIER] |
| Mémoire (10K nœuds) | < 100MB | [À VÉRIFIER] | [À VÉRIFIER] | [À VÉRIFIER] |
| Temps d'intégration | < 1 semaine | [À VÉRIFIER] | [À VÉRIFIER] | [À VÉRIFIER] |

---

## 7. PLANIFICATION

### 7.1 Phase 1: Recherche et Préparation (1-2 jours)

- [ ] Lire la documentation de chaque candidat
- [ ] Créer les jeux de données de test
- [ ] Définir les requêtes de benchmark
- [ ] Configurer l'environnement de test

### 7.2 Phase 2: Évaluation crepe (2-3 jours)

- [ ] Installer et configurer crepe
- [ ] Implémenter les requêtes de test
- [ ] Exécuter les benchmarks
- [ ] Documenter les résultats

### 7.3 Phase 3: Évaluation Soufflé (3-4 jours)

- [ ] Installer Soufflé et créer les bindings Rust
- [ ] Implémenter les requêtes de test
- [ ] Exécuter les benchmarks
- [ ] Documenter les résultats

### 7.4 Phase 4: Prototypage Custom (3-4 jours)

- [ ] Concevoir l'architecture du moteur custom
- [ ] Implémenter le parser Datalog
- [ ] Implémenter l'execution engine
- [ ] Exécuter les benchmarks
- [ ] Documenter les résultats

### 7.5 Phase 5: Comparaison et Décision (1-2 jours)

- [ ] Analyser tous les résultats
- [ ] Comparer les candidats
- [ ] Identifier les trade-offs
- [ ] Rédiger le rapport final
- [ ] Recommander un candidat

**Durée totale estimée:** 10-15 jours

---

## 8. DÉPENDANCES

### 8.1 Dépendances Externes

- **crepe:** https://github.com/crepe-rs/crepe
- **Soufflé:** https://github.com/souffle-lang/souffle
- **Rust:** 1.70+ (pour les features utilisées)

### 8.2 Dépendances Internes

- Aucune dépendance avec le code principal de Note Craft (ce PoC est indépendant)

---

## 9. RISQUES ET MITIGATIONS

| Risque | Probabilité | Impact | Mitigation |
|--------|-------------|--------|------------|
| crepe trop immature | Moyenne | Élevé | Vérifier le nombre d'étoiles, commits, issues sur GitHub |
| Soufflé FFI trop complexe | Moyenne | Élevé | Évaluer la complexité des bindings existants |
| Custom trop long à développer | Élevée | Élevé | Commencer par un prototype minimal |
| Incompatibilité syntaxe Logseq | Moyenne | Élevé | Adapter les requêtes ou le parser |
| Performances insuffisantes | Moyenne | Élevé | Optimiser ou choisir un autre candidat |

---

## 10. RÉFÉRENCES

- [Issue #4: PoC: Moteur Datalog (Phase 0.13)](https://github.com/owner/repo/issues/4)
- [docs/roadmap/roadmap_v3.md:663-690](docs/roadmap/roadmap_v3.md)
- [docs/history/fai_001_audit.md](docs/history/fai_001_audit.md)
- [crepe GitHub](https://github.com/crepe-rs/crepe)
- [Soufflé GitHub](https://github.com/souffle-lang/souffle)
- [Soufflé Documentation](https://souffle-lang.github.io)
- [Datalog Wikipedia](https://en.wikipedia.org/wiki/Datalog)
- [Logseq Datalog Query System](https://logseq.github.io/docs/datalog)

---

**Statut:** EN COURS (Phase 2 - Évaluation datafrog terminée à 30%)  
**Prochaine action:** Phase 3 - Évaluation Soufflé et prototypage Custom

### Progress Summary:
- ✅ Phase 0: Setup et recherche - TERMINÉ
- ✅ Phase 1: PoC datafrog de base - TERMINÉ (benchmarks préliminaires obtenus)
- ⏳ Phase 2: Évaluation complète datafrog - 30% (benchmarks sur petits jeux)
- ⏳ Phase 3: Évaluation Soufflé - 0%
- ⏳ Phase 4: Prototypage Custom - 0%
- ⏳ Phase 5: Comparaison et décision - 0%