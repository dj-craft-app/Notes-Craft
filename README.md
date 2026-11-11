# Note Craft

> **Alternative 100% Rust à Notion, Obsidian et Logseq**
> Direction artistique inspirée d'Obsidian, fonctionnalités enrichies de Logseq

## 📋 Statut du projet

- **Roadmap**: v3 ✅ **VALIDÉE TRÈS BIEN** (critères a/b/c/d satisfaits)
- **Date de validation**: 2026-11-11
- **Décision moteur Datalog**: ✅ **datafrog retenu** (Phase 0.13 COMPLÈTE)
- **Tag**: v0.2.0-datalog-decision
- **[À VÉRIFIER]**: Benchmarks et PoC à réaliser (voir issues GitHub)

## 🗂️ Structure du dépôt

```
Note-Craft/
├── README.md                    # Ce fichier
├── .gitignore                   # Exclusions Git
├── .github/                     # Workflows CI/CD (futur)
├── docs/
│   ├── roadmap/
│   │   └── roadmap_v3.md         # Roadmap finale validée (1226 lignes)
│   ├── verification/            # Rapports de vérification
│   │   ├── verification_sources_v3_rapport.md
│   │   ├── verification_technique_v3_rapport.md
│   │   └── contre_lecture_critere_a_final_v2.md
│   ├── audits/                   # Audits sources
│   │   ├── logseq_audit_results.json
│   │   ├── logseq_audit_summary.md
│   │   ├── obsidian_design_system.json
│   │   ├── note_craft_design_system.md
│   │   ├── rust_architecture_choices.json
│   │   └── rust_architecture_report.md
│   └── history/                  # Historique des versions
│       └── roadmap.md
└── poc/
    └── datalog/                  # PoC Moteur Datalog (NOUVEAU)
        ├── ADR-001-candidats-datalog.md    # Sélection des candidats
        ├── ADR-002-criteres-decision.md     # Critères figés
        ├── ADR-002-ERRATUM-001.md           # ERRATUM criterion
        ├── README.md                        # Documentation PoC
        ├── results/                        # Résultats benchmarks
        │   ├── METHODOLOGIE.md                # Méthodologie de mesure
        │   ├── benchmark_crepe.md            # Benchmarks crepe
        │   ├── benchmark_custom.md           # Benchmarks Custom
        │   ├── benchmark_datafrog.md         # Benchmarks datafrog
        │   ├── benchmark_souffle.md          # Élimination Soufflé
        │   ├── comparison.md                  # Comparaison complète
        │   └── decision.md                   # Décision finale
        ├── crepe/                          # PoC crepe
        │   └── poc/                         # Code benchmark
        ├── datafrog/                       # PoC datafrog
        │   └── benches/                     # Benchmarks
        ├── custom/                         # PoC Custom
        │   └── benches/                     # Benchmarks
        └── scripts/                         # Scripts de calcul
            ├── calcul_ratios.py
            ├── calcul_ratios_final.py
            └── calcul_ratios_crepe_vs_datafrog.sh
```

## 🎯 Décisions Architecturales

### Moteur Datalog (Phase 0.13) ✅ **TERMINÉE**

**Décision**: **datafrog** retenu avec score **96.00/100**

| Candidat | Score | Performance (10K nœuds) | Intégration | Maintenance | Ergonomie |
|----------|-------|-------------------------|-------------|-------------|-----------|
| **datafrog** | **96.00** | **537.56 µs - 10 724x** | 100% Rust | ✅ Actif | 6/10 |
| Custom | 63.15 | 175.58 µs - 570x | 100% Rust | ✅ Contrôlé | 8/10 |
| crepe | 60.00 | 189.73 ms - 206.31 ms | 100% Rust | ✅ Actif | **10/10** |
| Soufflé | ❌ **ÉLIMINÉ** | - | FFI C++ | - | - |

**Détails**:
- **datafrog**: Meilleur score global, dépasse la cible de 101x à 10 724x sur toutes les requêtes
- **crepe**: Meilleure ergonomie (proc macros, syntaxe Datalog) mais performances insuffisantes
- **Soufflé**: Éliminé par ADR-002 Section 4.2 (FFI non justifiée, datafrog dépasse déjà la cible)

**Fichiers**:
- Décision: `poc/datalog/results/decision.md`
- Comparaison: `poc/datalog/results/comparison.md`
- Benchmarks: `poc/datalog/results/benchmark_*.md`
- ADR: `poc/datalog/ADR-001-candidats-datalog.md`, `poc/datalog/ADR-002-criteres-decision.md`

**Tag**: `v0.2.0-datalog-decision`

---

## 📚 Documentation

### Roadmap
- **Version actuelle**: `docs/roadmap/roadmap_v3.md` (1226 lignes, 54KB)
- **Statut**: TRÈS BIEN (100% des affirmations sourcées ou marquées [À VÉRIFIER])
- **Critères**: (a) 100% sourçage, (b) complétude, (c) cohérence technique, (d) questions bloquantes résolues

### Vérifications
- **Sources**: `docs/verification/verification_sources_v3_rapport.md`
- **Technique**: `docs/verification/verification_technique_v3_rapport.md`
- **Contre-lecture**: `docs/verification/contre_lecture_critere_a_final_v2.md`

### Audits
- **Logseq**: Analyse complète du code et des binaires
- **Obsidian**: Extraction du design system (palette, typographie, composants)
- **Architecture Rust**: Choix techniques documentés et comparés

### PoC Datalog
- **ADR-001**: Sélection des candidats (crepe, datafrog, Soufflé, Custom)
- **ADR-002**: Critères de décision (figés avant les benchmarks)
- **ERRATUM-001**: Résolution de la contradiction criterion
- **Résultats**: Tous les benchmarks avec méthodologie documentée

## 🎯 Prochaines étapes

### Backlog (Issues GitHub)
1. **Benchmark performance**: Rendu instantané de graphes complexes (>10,000 noeuds)
2. **Architecture sécurité**: Sandboxing WASM, chiffrement E2EE
3. **Design plugins**: Système de plugins multi-langage (WASM)
4. **Benchmark graphe**: Performance exceptionnelle pour grands graphes
5. **Validation sécurité**: Implémentation à valider
6. **Documentation architecture**: Design à finaliser
7. **✅ PoC Datalog**: Phase 0.13 **TERMINÉE** (datafrog retenu)
8. **Chargement initial**: < 2 secondes (Benchmark à réaliser)

---

### Issues à fermer (Décision validée)
- **#4**: PoC: Moteur Datalog (Phase 0.13) → **FERMER** (décision prise)
- **#6**: revue: ADR-001-candidats-datalog → **FERMER** (ERRATUM appliqué)
- **#7**: revue: ADR-002-criteres-decision → **FERMER** (ADR figé)
- **#8**: revue: décision moteur Datalog → **FERMER** (toutes corrections appliquées)

---

## 🛠️ Contribution

### Convention de travail
1. **TOUT** le travail futur se fait dans ce dépôt
2. Chaque étape de la roadmap = une branche `phase-X-nom` ou `issue-N-titre`
3. Commits réguliers + merge dans main après vérification
4. Jamais de commit direct sur main
5. Chaque session se termine par: commit + push + `git status` clean

### Branches récentes
- `main`: Version stable (inclut PoC Datalog)
- `issue-4-datalog-poc`: Branche de travail **MERGÉE** dans main

## 📊 Métriques Clés

### PoC Datalog - Performances (10K nœuds)

| Requête | datafrog | Ratio vs Cible |
|---------|----------|----------------|
| Transitivité | **537.56 µs** | **186x meilleur** |
| Par tag | **16.848 µs** | **5 935x meilleur** |
| Par lien | **985.85 µs** | **101x meilleur** |
| Par type | **9.3252 µs** | **10 724x meilleur** |

**Cible roadmap**: < 100ms sur 10K nœuds → **DÉPASSÉE PAR datafrog**

### Ratios crepe vs datafrog

| Requête | Ratio |
|---------|-------|
| Par tag | **11 397.79x** |
| Par lien | **190.79x** |
| Transitivité | **352.95x** |
| Par type | **22 123.92x** |

---

## 📄 Licence
À déterminer (en attente de votre choix)

---

## 🔗 Liens utiles

- **Repository**: https://github.com/dj-craft-app/Notes-Craft
- **Roadmap**: `docs/roadmap/roadmap_v3.md`
- **PoC Datalog**: `poc/datalog/`
- **Tag**: `v0.2.0-datalog-decision`

---

*Projet Note Craft - Alternative Rust à Notion/Obsidian/Logseq*
*Dernière mise à jour: 2026-11-11*
