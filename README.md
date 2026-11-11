# Note Craft

> **Alternative 100% Rust à Notion, Obsidian et Logseq**
> Direction artistique inspirée d'Obsidian, fonctionnalités enrichies de Logseq

## 📋 Statut du projet

- **Roadmap**: v3 ✅ **VALIDÉE TRÈS BIEN** (critères a/b/c/d satisfaits)
- **Date de validation**: 2026-11-11
- **8 [À VÉRIFIER]**: Benchmarks et PoC à réaliser (voir issues GitHub)

## 🗂️ Structure du dépôt

```
Note-Craft/
├── README.md                    # Ce fichier
├── .gitignore                   # Exclusions Git
├── .github/                     # Workflows CI/CD (futur)
└── docs/
    ├── roadmap/
    │   └── roadmap_v3.md         # Roadmap finale validée
    ├── verification/
    │   ├── verification_sources_v3_rapport.md
    │   ├── verification_technique_v3_rapport.md
    │   └── contre_lecture_critere_a_final_v2.md
    ├── audits/                   # Audits sources
    │   ├── logseq_audit_results.json
    │   ├── logseq_audit_summary.md
    │   ├── obsidian_design_system.json
    │   ├── note_craft_design_system.md
    │   ├── rust_architecture_choices.json
    │   └── rust_architecture_report.md
    └── history/                  # Historique des versions et audits
        ├── roadmap.md           # v1 (identique à roadmap_v3_base.md, roadmap_v3_working.md)
        ├── roadmap_v2.md         # REJETÉE
        └── [autres fichiers historiques]
```

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

## 🎯 Prochaines étapes

### Backlog (8 [À VÉRIFIER] → Issues GitHub)
1. Benchmark performance: Rendu instantané de graphes complexes (>10,000 noeuds)
2. Architecture sécurité: Sandboxing WASM, chiffrement E2EE
3. Design plugins: Système de plugins multi-langage (WASM)
4. Benchmark graphe: Performance exceptionnelle pour grands graphes
5. Validation sécurité: Implémentation à valider
6. Documentation architecture: Design à finaliser
7. PoC Datalog: Phase 0.13 (moteur à comparer: crepe, Soufflé, custom SQLite)
8. Chargement initial: < 2 secondes (Benchmark à réaliser)

## 🛠️ Contribution

### Convention de travail
1. **TOUT** le travail futur se fait dans ce dépôt
2. Chaque étape de la roadmap = une branche `phase-X-nom`
3. Commits réguliers + merge dans main après vérification
4. Jamais de commit direct sur main
5. Chaque session se termine par: commit + push + `git status` clean

### Branches actives
- `main`: Version stable
- `phase-0.13-datalog-poc`: PoC moteur Datalog (à créer)

## 📄 Licence
À déterminer (en attente de votre choix)

---

*Projet Note Craft - Alternative Rust à Notion/Obsidian/Logseq*
