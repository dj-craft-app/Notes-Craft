#!/bin/bash

# Script pour créer les issues de la Phase 0
# Conformément à AGENTS.md: UNE ISSUE PAR ÉTAPE TESTABLE

REPO_DIR="/Users/amaury/Documents/REBUILD APPS/Note-Craft"
cd "$REPO_DIR" || exit 1

# Fonction pour créer une issue
h create_issue() {
    local title="$1"
    local body="$2"
    local labels="$3"
    
    echo "Création de l'issue: $title"
    echo "Labels: $labels"
    echo "---"
    
    # Utiliser gh CLI pour créer l'issue
    gh issue create \
        --title "$title" \
        --body "$body" \
        --label "$labels" \
        --no-editor
    
    echo "---"
    echo ""
}

# Phase 0: Setup Projet - Objective
PHASE_0_OBJECTIVE="Créer l'infrastructure de base pour le projet"
PHASE_0_DEPENDENCIES="Aucune"
PHASE_0_CRITERIA="- Dépôt Git accessible et configuré
- Projet compile avec cargo build
- CI/CD exécute les tests automatiquement
- Structure du projet validée par l'équipe
- Toutes les dépendances principales configurées
- Documentation de setup disponible"

# Étape 0.1
h create_issue \
    "0.1: Créer dépôt Git (GitHub)" \
    "# 0.1: Créer dépôt Git (GitHub)

## Phase
**Phase 0: Setup Projet (1-2 semaines)**

## Objectif de la Phase
$PHASE_0_OBJECTIVE

## Étape
**0.1** - Créer dépôt Git (GitHub)

### Priorité
Critique

### Estimation
1h

### Dépendances
Aucune

### Source
Conformité aux standards Rust

## Critères d'Acceptation
$PHASE_0_CRITERIA

---
### Dépendances de la Phase
$PHASE_0_DEPENDENCIES" \
    "phase-0,priority:critical"

# Étape 0.2
h create_issue \
    "0.2: Configurer Cargo workspace" \
    "# 0.2: Configurer Cargo workspace

## Phase
**Phase 0: Setup Projet (1-2 semaines)**

## Objectif de la Phase
$PHASE_0_OBJECTIVE

## Étape
**0.2** - Configurer Cargo workspace

### Priorité
Critique

### Estimation
2h

### Dépendances
0.1

### Source
rust_architecture_report.md:60

## Critères d'Acceptation
$PHASE_0_CRITERIA" \
    "phase-0,priority:critical"

# Étape 0.3
h create_issue \
    "0.3: Définir structure du projet" \
    "# 0.3: Définir structure du projet

## Phase
**Phase 0: Setup Projet (1-2 semaines)**

## Objectif de la Phase
$PHASE_0_OBJECTIVE

## Étape
**0.3** - Définir structure du projet

### Priorité
Critique

### Estimation
4h

### Dépendances
0.2

### Source
rust_architecture_report.md:70

## Critères d'Acceptation
$PHASE_0_CRITERIA" \
    "phase-0,priority:critical"

# Étape 0.4
h create_issue \
    "0.4: Configurer Slint" \
    "# 0.4: Configurer Slint

## Phase
**Phase 0: Setup Projet (1-2 semaines)**

## Objectif de la Phase
$PHASE_0_OBJECTIVE

## Étape
**0.4** - Configurer Slint

### Priorité
Critique

### Estimation
1d

### Dépendances
0.3

### Source
rust_architecture_choices.json:90

## Critères d'Acceptation
$PHASE_0_CRITERIA" \
    "phase-0,priority:critical"

# Étape 0.5
h create_issue \
    "0.5: Configurer wry" \
    "# 0.5: Configurer wry

## Phase
**Phase 0: Setup Projet (1-2 semaines)**

## Objectif de la Phase
$PHASE_0_OBJECTIVE

## Étape
**0.5** - Configurer wry

### Priorité
Critique

### Estimation
1d

### Dépendances
0.3

### Source
rust_architecture_choices.json:360

## Critères d'Acceptation
$PHASE_0_CRITERIA" \
    "phase-0,priority:critical"

# Étape 0.6
h create_issue \
    "0.6: Configurer pulldown-cmark" \
    "# 0.6: Configurer pulldown-cmark

## Phase
**Phase 0: Setup Projet (1-2 semaines)**

## Objectif de la Phase
$PHASE_0_OBJECTIVE

## Étape
**0.6** - Configurer pulldown-cmark

### Priorité
Critique

### Estimation
1d

### Dépendances
0.3

### Source
rust_architecture_choices.json:360

## Critères d'Acceptation
$PHASE_0_CRITERIA" \
    "phase-0,priority:critical"

# Étape 0.7
h create_issue \
    "0.7: Configurer SQLite (rusqlite)" \
    "# 0.7: Configurer SQLite (rusqlite)

## Phase
**Phase 0: Setup Projet (1-2 semaines)**

## Objectif de la Phase
$PHASE_0_OBJECTIVE

## Étape
**0.7** - Configurer SQLite (rusqlite)

### Priorité
Critique

### Estimation
1d

### Dépendances
0.3

### Source
rust_architecture_choices.json:180

## Critères d'Acceptation
$PHASE_0_CRITERIA" \
    "phase-0,priority:critical"

# Étape 0.8
h create_issue \
    "0.8: Configurer wasmtime" \
    "# 0.8: Configurer wasmtime

## Phase
**Phase 0: Setup Projet (1-2 semaines)**

## Objectif de la Phase
$PHASE_0_OBJECTIVE

## Étape
**0.8** - Configurer wasmtime

### Priorité
Moyen

### Estimation
1d

### Dépendances
0.3

### Source
rust_architecture_choices.json:570

## Critères d'Acceptation
$PHASE_0_CRITERIA" \
    "phase-0,priority:medium"

# Étape 0.9
h create_issue \
    "0.9: Configurer tokio" \
    "# 0.9: Configurer tokio

## Phase
**Phase 0: Setup Projet (1-2 semaines)**

## Objectif de la Phase
$PHASE_0_OBJECTIVE

## Étape
**0.9** - Configurer tokio

### Priorité
Moyen

### Estimation
1d

### Dépendances
0.3

### Source
rust_architecture_choices.json:700

## Critères d'Acceptation
$PHASE_0_CRITERIA" \
    "phase-0,priority:medium"

# Étape 0.10
h create_issue \
    "0.10: Configurer CI/CD (GitHub Actions)" \
    "# 0.10: Configurer CI/CD (GitHub Actions)

## Phase
**Phase 0: Setup Projet (1-2 semaines)**

## Objectif de la Phase
$PHASE_0_OBJECTIVE

## Étape
**0.10** - Configurer CI/CD (GitHub Actions)

### Priorité
Critique

### Estimation
1d

### Dépendances
0.1

### Source
rust_architecture_report.md:800

## Critères d'Acceptation
$PHASE_0_CRITERIA" \
    "phase-0,priority:critical"

# Étape 0.11
h create_issue \
    "0.11: Configurer système de build" \
    "# 0.11: Configurer système de build

## Phase
**Phase 0: Setup Projet (1-2 semaines)**

## Objectif de la Phase
$PHASE_0_OBJECTIVE

## Étape
**0.11** - Configurer système de build

### Priorité
Critique

### Estimation
1d

### Dépendances
0.2

### Source
Conformité aux standards Rust

## Critères d'Acceptation
$PHASE_0_CRITERIA" \
    "phase-0,priority:critical"

# Étape 0.12
h create_issue \
    "0.12: Créer documentation initiale" \
    "# 0.12: Créer documentation initiale

## Phase
**Phase 0: Setup Projet (1-2 semaines)**

## Objectif de la Phase
$PHASE_0_OBJECTIVE

## Étape
**0.12** - Créer documentation initiale

### Priorité
Moyen

### Estimation
2d

### Dépendances
0.1

### Source
Aucune

## Critères d'Acceptation
$PHASE_0_CRITERIA" \
    "phase-0,priority:medium"

echo "Toutes les issues de la Phase 0 ont été créées !"
echo "Vérifiez avec: gh issue list --state open --label phase-0"
