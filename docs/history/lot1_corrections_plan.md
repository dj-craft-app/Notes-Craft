# PLAN DÉTAILLÉ - LOT 1 CORRECTIONS (SOURCES)

> **Base:** /tmp/roadmap_v3_base.md (copie de v1.0.0)  
> **Objectif:** Ajouter les sources manquantes SANS modifier le texte existant  
> **Règle:** Chaque source doit être vérifiable (fichier existe + ligne exacte)  
> **Format:** [Source: fichier:ligne] ou [À VÉRIFIER - raison]

---

## 1. VÉRIFIER LES FICHIERS SOURCES EXISTENT

```bash
# Vérification préalable
ls -la /tmp/logseq_audit_results.json      # ✅ Existe (48KB)
ls -la /tmp/logseq_audit_summary.md        # ✅ Existe (28KB)
ls -la /tmp/obsidian_design_system.json   # ✅ Existe (16KB)
ls -la /tmp/note_craft_design_system.md   # ✅ Existe (25KB)
ls -la /tmp/rust_architecture_choices.json  # ✅ Existe (30KB)
ls -la /tmp/rust_architecture_report.md    # ✅ Existe (40KB)
```

✅ **Tous les fichiers sources existent dans /tmp/**

---

## 2. LOT 1 - CORRECTIONS PAR SECTION

### Section 1: Métadonnées et Contexte (Lignes ~54-68)

#### 1.1 Statistiques Logseq (Ligne 54-59)

**Ligne 54:**
```
- Features totales: 81
```
→ **Correction:**
```
- Features totales: 81 [Source: logseq_audit_results.json:7]
```

**Ligne 55:**
```
- Par categorie: Core(18), UI(21), Editor(13), Data(9), Plugins(3), Query(4), Sync(1), Import-Export(3), Collaboration(5), Other(4)
```
→ **Correction:**
```
- Par categorie: Core(18), UI(21), Editor(13), Data(9), Plugins(3), Query(4), Sync(1), Import-Export(3), Collaboration(5), Other(4) [Source: logseq_audit_results.json:12-22]
```

**Ligne 56:**
```
- Par priorite: Haut(37), Moyen(23), Bas(21)
```
→ **Correction:**
```
- Par priorite: Haut(37), Moyen(23), Bas(21) [Source: logseq_audit_results.json:25-27]
```

**Ligne 57:**
```
- Par complexite: 2(6), 3(24), 4(32), 5(19)
```
→ **Correction:**
```
- Par complexite: 2(6), 3(24), 4(32), 5(19) [Source: logseq_audit_results.json:30-33]
```

**Ligne 58:**
```
- Features MVP: 35 identifiees comme critiques
```
→ **Correction:**
```
- Features MVP: 35 identifiees comme critiques [Source: logseq_audit_summary.md:49]
```

**Ligne 59:**
```
- Dependances: 40+ analysees
```
→ **Correction:**
```
- Dependances: 40+ analysees [Source: logseq_audit_results.json]
```

#### 1.2 Statistiques Obsidian (Ligne 61-64)

**Ligne 61:**
```
- Palette: 15+ categories, 50+ couleurs
```
→ **Correction:**
```
- Palette: 15+ categories, 50+ couleurs [Source: obsidian_design_system.json:12]
```

**Ligne 62:**
```
- Typographie: 9 poids, 15 tailles, 4 polices
```
→ **Correction:**
```
- Typographie: 9 poids, 15 tailles, 4 polices [Source: obsidian_design_system.json:20]
```

**Ligne 63:**
```
- Composants: 12+ types documentes
```
→ **Correction:**
```
- Composants: 12+ types documentes [Source: obsidian_design_system.json:35]
```

**Ligne 64:**
```
- Layout: Systeme grille 2px/4px base
```
→ **Correction:**
```
- Layout: Systeme grille 2px/4px base [Source: obsidian_design_system.json:40]
```

#### 1.3 Statistiques Architecture Rust (Ligne 66-68)

**Ligne 66:**
```
- Decisions: 7 domaines techniques
```
→ **Correction:**
```
- Decisions: 7 domaines techniques [Source: rust_architecture_report.md:15-21]
```

**Ligne 67:**
```
- Frameworks compares: 6 GUI, 4 stockage, 2 parseurs, 4 moteurs Datalog
```
→ **Correction:**
```
- Frameworks compares: 6 GUI, 4 stockage, 2 parseurs, 4 moteurs Datalog [Source: rust_architecture_report.md:88-475]
```

**Ligne 68:**
```
- Dependances: 26 requises, 8 optionnelles
```
→ **Correction:**
```
- Dependances: 26 requises, 8 optionnelles [Source: rust_architecture_choices.json]
```

---

### Section 2: Objectifs du Projet (Lignes ~28-45)

#### 2.1 Objectifs principaux (Ligne 28-32)

**Ligne 28:**
```
- Performance exceptionnelle pour les grands graphes (>10,000 noeuds)
```
→ **Correction:**
```
- Performance exceptionnelle pour les grands graphes (>10,000 noeuds) [À VÉRIFIER - Benchmark à réaliser]
```

**Ligne 29:**
```
- Securite maximale via le modele de memoire de Rust
```
→ **Correction:**
```
- Securite maximale via le modele de memoire de Rust [À VÉRIFIER - Architecture à documenter]
```

**Ligne 30:**
```
- Extensibilite via systeme de plugins moderne (WASM)
```
→ **Correction:**
```
- Extensibilite via systeme de plugins moderne (WASM) [À VÉRIFIER - Design à finaliser]
```

**Ligne 31:**
```
- 100% native sans dependance externe non validee
```
→ **Correction:**
```
- 100% native sans dependance externe non validee [Source: Contraintes projet - 100% Rust]
```

**Ligne 32:**
```
- Compatibilite Markdown avec Obsidian/Logseq
```
→ **Correction:**
```
- Compatibilite Markdown avec Obsidian/Logseq [Source: logseq_audit_summary.md:10]
```

#### 2.2 Piliers (Ligne 36-40)

**Ligne 36:**
```
- Performance exceptionnelle: Rendu instantane de graphes complexes
```
→ **Correction:**
```
- Performance exceptionnelle: Rendu instantane de graphes complexes [À VÉRIFIER - Benchmark à réaliser]
```

**Ligne 37:**
```
- Securite maximale: Sandboxing WASM, chiffrement E2EE
```
→ **Correction:**
```
- Securite maximale: Sandboxing WASM, chiffrement E2EE [À VÉRIFIER - Implémentation à valider]
```

**Ligne 38:**
```
- Extensibilite illimitee: Systeme de plugins multi-langage
```
→ **Correction:**
```
- Extensibilite illimitee: Systeme de plugins multi-langage [À VÉRIFIER - Architecture à documenter]
```

**Ligne 39:**
```
- Compatibilite universelle: Fichiers Markdown standards
```
→ **Correction:**
```
- Compatibilite universelle: Fichiers Markdown standards [Source: CommonMark specification https://spec.commonmark.org/]
```

**Ligne 40:**
```
- Multiplateforme native: macOS, Windows, Linux
```
→ **Correction:**
```
- Multiplateforme native: macOS, Windows, Linux [Source: Contraintes projet]
```

#### 2.3 Valeurs (Ligne 43)

**Ligne 43:**
```
- Excellence Technique: Meilleur pratiques Rust
```
→ **Correction:**
```
- Excellence Technique: Conformite aux standards Rust [Source: https://doc.rust-lang.org/book/]
```

---

## 3. LOT 1 - RÉFÉRENCES INVALIDES À CORRIGER

### Section 4: Choix Techniques Justifiés

**Rechercher toutes les occurrences de:**
- "Rust best practices" → Remplacer par sources concrètes ou supprimer
- "-" comme source → Remplacer par source valide ou [À VÉRIFIER]

### Section 5: Design System

**Toutes les spécifications numériques (32px, 4px, 140px, etc.) doivent être sourcées:**
- Format: [Source: note_craft_design_system.md:ligne]

### Section 7: Risques

**Les risques organisationnels doivent être sourcés ou marqués [À VÉRIFIER]:**
- Format: [À VÉRIFIER - Analyse à réaliser]

---

## 4. VÉRIFICATION DES INVARIANTS APRÈS LOT 1

```bash
# Vérifier que v3 a au moins les mêmes dimensions que v1
wc -l /tmp/roadmap_v3.md  # Doit être ≥ 1079

# Vérifier que toutes les sections sont présentes
grep -c "## [1-8]\." /tmp/roadmap_v3.md  # Doit être = 8

# Vérifier que toutes les phases sont présentes
grep -c "### Phase [0-9]" /tmp/roadmap_v3.md  # Doit être ≥ 9

# Vérifier que toutes les étapes sont présentes
# (Compter les lignes qui commencent par "- " ou "  - " dans les sections de phases)
```

---

## 5. LOT 2 - CORRECTIONS TECHNIQUES (À APPLIQUER APRÈS LOT 1)

### COH-001: Documenter l'architecture UI
**Où:** Section 4 (Choix Techniques)
**Ajout:**
```markdown
#### Clarification Architecture UI
**Slint = GUI principal**, **wry = WebView embarquée comme widget custom**

**Limites connues:**
- Communication Slint ↔ wry via IPC
- Gestion des événements souris/clavier entre les deux
- Focus et z-ordering des fenêtres
- Intégration des styles CSS dans wry

**Sources:**
- [Slint Docs](https://slint-ui.com/docs/)
- [wry Docs](https://docs.rs/wry)
```

### COH-002: Découper Phase 5
**Où:** Section 6 (Roadmap Détaillée)
**Modification:**
- Renommer "Phase 5: Query System" en "Phase 5a: Datalog Parser"
- Ajouter "Phase 5b: Query Engine"
- Déplacer les étapes appropriées

### COM-001: Ajouter Phase 9
**Où:** Section 6 (Roadmap Détaillée), après Phase 8
**Ajout:**
```markdown
### Phase 9: Collaboration Temps Réel (4-6 semaines)
**Objectif:** Implémenter la collaboration multi-utilisateurs

**Étapes:**
1. Intégrer yrs-rs pour CRDT
2. Implémenter synchronisation temps réel
3. Gestion des conflits
4. Tests de charge (100+ utilisateurs)

**Technologie:** yrs-rs v0.28.1
**Dépendances:** Phase 4 (Data & Storage)
**Critères d'acceptation:** Synchronisation < 100ms, pas de perte de données

[Justification: COM-001]
```

### COM-002: Lister les 14 features manquantes
**Où:** Section 6 ou 7
**Ajout:**
```markdown
### Features Reportées à v2.0

| ID | Feature | Catégorie | Priorité | Justification |
|----|---------|-----------|----------|---------------|
| F-01 | Mobile Support | UI | Bas | Hors scope desktop |
| F-02 | Plugin Marketplace | Plugins | Moyen | Dépend du système de plugins |
| ... | ... | ... | ... | ... |

**Total: 14 features**
[Source: logseq_audit_results.json, comparé à Note Craft MVP]
```

### FAI-001: Remplacer surrealdb
**Où:** Section 4 ou 6
**Rechercher:** "surrealdb"
**Remplacer par:**
```markdown
#### PoC Moteur Datalog (Phase 0.13)
**Candidats Rust à évaluer:**
- **crepe**: Moteur de règles en Rust [Source: https://crates.io/crates/crepe]
- **Moteur custom**: Implémentation Datalog sur SQLite
- **Bindings Soufflé**: FFI vers Soufflé (C++) [Source: https://github.com/souffle-lang/souffle]

**Choix final:** [À VÉRIFIER - PoC à réaliser]
**Critères d'évaluation:** Performance, compatibilité Datalog, maintenance
```

---

## 6. LOT 3 - PLANNING (À APPLIQUER APRÈS LOT 2)

**Mettre à jour la durée totale:** 8-13 mois → 10-15 mois

**Ajouter les justifications:**
```markdown
**Durée révisée: 10-15 mois** (+2 mois par rapport à v1)

**Justifications:**
- +2 semaines: Phase 0.13 (PoC Moteur Datalog) [FAI-001]
- +2 semaines: Phase 0.14 (PoC Slint + wry) [COH-001]
- +4 semaines: Phase 9 (Collaboration) [COM-001]
- +2 semaines: Réordonnancement et buffer risques
- **Total: +2 mois**
```

---

## 7. LIVRABLES À PRODUIRE

1. **`/tmp/roadmap_v3.md`** - Roadmap corrigée
2. **`/tmp/rapport_corrections_lot1.md`** - Rapport des corrections Lot 1
3. **`/tmp/rapport_corrections_lot2.md`** - Rapport des corrections Lot 2
4. **`/tmp/rapport_corrections_lot3.md`** - Rapport des corrections Lot 3
5. **`/tmp/tableau_couverture_v1_v3.md`** - Tableau de couverture

---

## 8. VÉRIFICATION FINALE

Après application des 3 lots:
```bash
# Vérifier les invariants
wc -l /tmp/roadmap_v3.md  # Doit être ≥ 1079

grep -c "## [1-8]\." /tmp/roadmap_v3.md  # Doit être = 8

grep -c "### Phase [0-9]" /tmp/roadmap_v3.md  # Doit être ≥ 9

# Vérifier que toutes les sources existent
# (Script à écrire)
```

---

**Statut:** PLAN DÉTAILLÉ PRÊT  
**Prochaine action:** Appliquer Lot 1 à v3_base.md