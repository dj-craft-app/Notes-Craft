# ADR-002: Critères de Décision pour le PoC Moteur Datalog

> **Date:** 2026-11-11  
> **Auteurs:** Agent autonome  
> **Statut:** **FIGÉ** - À ne plus modifier après publication  
> **Décision:** Définir les critères de décision **AVANT** les benchmarks finaux
> **Lien:** Issue #4 - PoC: Moteur Datalog (Phase 0.13)

---

## ⚠️ AVERTISSEMENT - DOCUMENT FIGÉ

**Ce document est FIGÉ et ne doit PLUS être modifié après sa publication.**

Toute modification après les résultats de benchmark devra faire l'objet d'un **ERRATUM** 
daté et justifié, créé dans un fichier séparé.

**Raison:** Éviter le biais de confirmation (changer les critères pour faire correspondre les résultats).

---

## 1. CONTEXTE

Ce document définit les **critères de décision objectifs** pour sélectionner un moteur Datalog 
parmi les candidats identifiés (crepe, datafrog, Soufflé, Custom) dans le cadre de la 
**Phase 0.13: PoC Moteur Datalog** (Issue #4, roadmap_v3.md:663-690).

Ces critères **doivent être définis AVANT** la production des benchmarks finaux pour 
garantir l'objectivité de la décision.

---

## 2. CHARGE IDENTIQUE

**Exigence absolue:** Tous les candidats doivent être testés avec **exactement les mêmes** :

### 2.1 Requêtes de test

Les **4 requêtes suivantes** doivent être exécutées par chaque candidat :

1. **Requête par tag** (simple)
   ```datalog
   [:find ?node :where [?node :tag "important"]]
   ```

2. **Requête par lien** (jointure simple)
   ```datalog
   [:find ?node ?linked :where [?node :link ?linked]]
   ```

3. **Requête de transitivité** (récursive)
   ```datalog
   [:find ?from ?to :where [?from :link* ?to]]
   ```

4. **Requête par type** (filtre simple)
   ```datalog
   [:find ?node :where [?node :type :page]]
   ```

**Note:** Les requêtes doivent être adaptées à la syntaxe de chaque candidat (crepe, datafrog, etc.) 
tout en maintenant la **sémantique équivalente**.

### 2.2 Jeux de données

Les **3 jeux de données suivants** doivent être utilisés :

| Jeu | Nœuds | Liens | Tags/Type | Cible |
|-----|-------|-------|-----------|-------|
| **Petit** | 100 | 500 | ~200 | Vérification rapide |
| **Moyen** | 1,000 | 5,000 | ~2,000 | Validation intermédiaire |
| **Grand** | 10,000 | 50,000 | ~20,000 | **Cible roadmap** |

**Sources:**
- Génération déterministe via code Rust (fichiers dans `poc/datalog/<candidat>/src/`)  
- Mêmes algorithmes de génération pour tous les candidats

---

## 3. MESURES

### 3.1 Méthodologie

**Outil:** [criterion.rs](https://docs.rs/criterion/latest/criterion/)  
**Version:** 0.5+ (ou dernière version stable)

**Configuration obligatoire:**
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_group(c: &mut Criterion) {
    // Warmup inclus automatiquement par criterion
    c.bench_function("requete_par_tag", |b| {
        b.iter(|| {
            // Exécuter la requête
            black_box(execute_query());
        });
    });
    
    // ... autres benchmarks
}

criterion_group!(benches, benchmark_group);
criterion_main!(benches);
```

### 3.2 Métriques à mesurer

| Métrique | Outil | Commande | Format |
|----------|-------|----------|--------|
| **Temps CPU** | criterion | Médiane + écart-type sur ≥20 itérations | µs/ms |
| **Mémoire** | `/usr/bin/time -l` | `time -l cargo bench` | KB/MB |
| **Temps réel** | criterion | Inclus dans les benchmarks | µs/ms |

**Commandes à citer dans les résultats:**
```bash
# Linux/macOS
$ /usr/bin/time -l cargo bench --bench <nom> 2>&1 | grep maximum
# Sortie: maximum resident set size (KB)

# Ou avec time (GNU)
$ /usr/bin/time -v cargo bench --bench <nom> 2>&1 | grep Maximum
```

### 3.3 Résultats à produire

Pour chaque candidat et chaque jeu de données, produire :

**Fichier:** `poc/datalog/results/benchmark_<candidat>.md`

**Format:**
```markdown
### crepe - Benchmark Results

**Environnement:**
- Système: [Sortie de `uname -a`]
- CPU: [Sortie de `lscpu | grep "Model name"`]
- RAM: [Sortie de `free -h`]
- Rust: [Sortie de `rustc --version`]
- Date: [Date ISO 8601]

#### Jeu Petit (100 nœuds, 500 liens)

| Requête | Médiane | Écart-type | Minimum | Maximum | Mémoire |
|---------|---------|------------|---------|---------|---------|
| Par tag | X.XXXµs | ±Y.YYYµs | A.AAAµs | B.BBBµs | C.CCCMB |
| Par lien | X.XXXµs | ±Y.YYYµs | A.AAAµs | B.BBBµs | C.CCCMB |
| Transitivité | X.XXXµs | ±Y.YYYµs | A.AAAµs | B.BBBµs | C.CCCMB |
| Par type | X.XXXµs | ±Y.YYYµs | A.AAAµs | B.BBBµs | C.CCCMB |

#### Jeu Moyen (1,000 nœuds, 5,000 liens)

[Même format que ci-dessus]

#### Jeu Grand (10,000 nœuds, 50,000 liens)

[Même format que ci-dessus]
```

---

## 4. CRITÈRES D'ÉLIMINATION

**Règle:** Un candidat est **éliminé** s'il échoue **au moins un** des critères suivants.

### 4.1 Performance

**Critère:** Écart de performance > 2x par rapport au meilleur sur le **jeu Grand (10K nœuds)**

**Seuil:** Pour chaque requête sur le jeu Grand :
```
si (temps_candidat / temps_meilleur) > 2.0 → ÉLIMINÉ
```

**Cible roadmap:** Requête complexe < 100ms sur 10K nœuds  
**→ Un candidat avec requête complexe > 200ms est éliminé** (si un autre fait < 100ms)

**Preuve:** Sortie de benchmark citée dans `results/benchmark_<candidat>.md`

### 4.2 Intégration Rust

**Critère:** FFI non-Rust accepté **SEULEMENT SI** :
1. L'écart de performance le justifie à ≥ 2x par rapport à une solution 100% Rust
2. **ET** le sandboxing est jugé acceptable en revue par @unepacrette

**Application:**
- **Soufflé:** Nécessite FFI Rust ↔ C++ → À évaluer selon ce critère
- **crepe, datafrog, Custom:** 100% Rust → Automatiquement acceptés

**Preuve:** Documentation de l'intégration dans `poc/datalog/<candidat>/evaluation.md`

### 4.3 Maintenance

**Critère:** Dernière release > 24 mois **ET** aucune activité (commits) dans les 12 derniers mois

**Vérification:**
```bash
# Pour chaque candidat
$ curl -s "https://api.github.com/repos/<owner>/<repo>" | jq '.pushed_at'
# Comparer avec la date actuelle
```

**Élimination automatique si:**
- Pas de release depuis > 24 mois
- Pas de commit depuis > 12 mois

**Sources:** GitHub API, crates.io metadata

### 4.4 License

**Critère:** License **non permissive** pour une application propriétaire

**Licences permissives acceptées:**
- MIT
- Apache-2.0
- BSD-2-Clause
- BSD-3-Clause
- ISC
- CC0

**Licences non permissives (éliminatoires):**
- GPL (toutes versions)
- AGPL
- LGPL (sauf si FFI isolée)
- Propriétaire

**Preuve:** Fichier LICENSE ou metadata crates.io

---

## 5. GRILLE DE SCORE

**À appliquer UNIQUEMENT aux candidats non éliminés par les critères ci-dessus.**

### 5.1 Poids

| Critère | Poids | Description |
|---------|-------|-------------|
| **Performance** | 40% | Temps d'exécution des requêtes |
| **Intégration Rust** | 30% | Facilité d'intégration dans Note Craft |
| **Maintenance** | 20% | Maturité et support du projet |
| **Ergonomie** | 10% | Adaptation aux features roadmap |
| **Total** | 100% | |

### 5.2 Normalisation

Pour chaque critère, normaliser sur une échelle de **0 à 1** :

#### 5.2.1 Performance (40%)

**Score normalisé =** `(meilleur_temps / temps_candidat)` 
- Si temps_candidat = meilleur_temps → score = 1.0
- Si temps_candidat = 2× meilleur_temps → score = 0.5
- Si temps_candidat > 2× meilleur_temps → score = 0.0 (éliminé)

**Score final:** Moyenne des scores normalisés pour les 4 requêtes sur le jeu Grand

#### 5.2.2 Intégration Rust (30%)

| Niveau | Score | Description |
|--------|-------|-------------|
| **Natif** | 1.0 | 100% Rust, pas de FFI |
| **FFI acceptable** | 0.8 | FFI mais justifié par performance ≥ 2x |
| **FFI non justifié** | 0.3 | FFI sans avantage performance significatif |
| **Non Rust** | 0.0 | Non applicable (éliminé par critère 4.2) |

#### 5.2.3 Maintenance (20%)

| Niveau | Score | Description |
|--------|-------|-------------|
| **Très actif** | 1.0 | Release dans les 3 derniers mois |
| **Actif** | 0.8 | Release dans les 6 derniers mois |
| **Modéré** | 0.6 | Release dans les 12 derniers mois |
| **Peu actif** | 0.3 | Release dans les 24 derniers mois |
| **Inactif** | 0.0 | > 24 mois (éliminé par critère 4.3) |

**Source:** Date de la dernière release (crates.io ou GitHub tags)

#### 5.2.4 Ergonomie (10%)

| Niveau | Score | Description |
|--------|-------|-------------|
| **Parfait** | 1.0 | Syntaxe Datalog compatible Logseq sans adaptation |
| **Bon** | 0.8 | Adaptation mineure nécessaire |
| **Acceptable** | 0.6 | Adaptation significative nécessaire |
| **Médiocre** | 0.3 | API uniquement, pas de syntaxe textuelle |
| **Pauvre** | 0.0 | Impossible à utiliser pour les features roadmap |

### 5.3 Calcul du score final

```
score_final = (
    (performance_normalisée * 0.40) +
    (intégration_rust * 0.30) +
    (maintenance * 0.20) +
    (ergonomie * 0.10)
)
```

**Score final:** Valeur entre 0.0 et 1.0

---

## 6. RECOMMANDATION

**Règle de décision:**

1. **Élimination:** Appliquer les critères d'élimination (Section 4) → Supprimer les candidats non conformes
2. **Score:** Calculer le score final pour chaque candidat restant (Section 5)
3. **Recommandation:**
   - **Score ≥ 0.8:** EXCELLENT → Sélectionner
   - **0.6 ≤ Score < 0.8:** BON → À considérer
   - **0.4 ≤ Score < 0.6:** MOYEN → Réservé
   - **Score < 0.4:** FAIBLE → Rejeter

**Décision finale:**
- Sélectionner le candidat avec le **score final le plus élevé**
- Si égalité: privilégier le candidat 100% Rust
- Si toujours égalité: privilégier le candidat avec la meilleure maintenance

**⚠️ La décision finale appartient à la revue par @unepacrette, pas au benchmark seul.**

---

## 7. LIVRABLE FINAL

### 7.1 Fichiers à produire

1. **`poc/datalog/results/benchmark_crepe.md`** - Résultats crepe
2. **`poc/datalog/results/benchmark_datafrog.md`** - Résultats datafrog
3. **`poc/datalog/results/benchmark_souffle.md`** - Résultats Soufflé
4. **`poc/datalog/results/benchmark_custom.md`** - Résultats Custom
5. **`poc/datalog/results/comparison.md`** - Comparaison des 4 candidats
6. **`poc/datalog/results/decision.md`** - Décision finale avec grille remplie

### 7.2 Contenu de `decision.md`

```markdown
# Décision Finale: Moteur Datalog pour Note Craft

**Date:** [AAAA-MM-JJ]  
**Issue:** #4  
**ADR:** ADR-002-criteres-decision.md  
**Statut:** [PROPOSÉE - en attente de revue]

## 1. Résultats des Benchmarks

[Tableau comparatif des 4 candidats]

## 2. Application des Critères d'Élimination

| Candidat | Performance | Intégration | Maintenance | License | Statut |
|----------|-------------|-------------|-------------|---------|--------|
| crepe | [✅/❌] | [✅/❌] | [✅/❌] | [✅/❌] | [ÉLIMINÉ/VALIDÉ] |
| datafrog | [✅/❌] | [✅/❌] | [✅/❌] | [✅/❌] | [ÉLIMINÉ/VALIDÉ] |
| Soufflé | [✅/❌] | [✅/❌] | [✅/❌] | [✅/❌] | [ÉLIMINÉ/VALIDÉ] |
| Custom | [✅/❌] | [✅/❌] | [✅/❌] | [✅/❌] | [ÉLIMINÉ/VALIDÉ] |

## 3. Grille de Score

| Candidat | Performance (40%) | Intégration (30%) | Maintenance (20%) | Ergonomie (10%) | **Total** |
|----------|-------------------|--------------------|-------------------|------------------|-----------|
| crepe | X.XX | X.XX | X.XX | X.XX | **X.XX** |
| datafrog | X.XX | X.XX | X.XX | X.XX | **X.XX** |
| Soufflé | X.XX | X.XX | X.XX | X.XX | **X.XX** |
| Custom | X.XX | X.XX | X.XX | X.XX | **X.XX** |

## 4. Recommandation

**Candidat recommandé:** [nom]  
**Score:** [X.XX]  
**Justification:** [explication détaillée]

## 5. Issue de Revue

**Issue:** [revue: decision-moteur-datalog]  
**@Revue par:** @unepacrette  
**Statut:** [EN ATTENTE DE REVUE]
```

### 7.3 Issue GitHub

Créer une issue dédiée :
```
Titre: "revue: décision moteur Datalog (Phase 0.13)"
Body: Lien vers poc/datalog/results/decision.md
```

---

## 8. TRAÇABILITÉ

**Fichiers sources:**
- `poc/datalog/ADR-002-criteres-decision.md` - Ce document (CRITÈRES FIGÉS)
- `poc/datalog/ADR-001-candidats-datalog.md` - Sélection des candidats
- `poc/datalog/results/` - Résultats des benchmarks

**Issues GitHub:**
- #4 - PoC: Moteur Datalog (Phase 0.13)
- #6 - revue: ADR-001-candidats-datalog
- [À CRÉER] - revue: decision-moteur-datalog

---

## 9. HISTORIQUE

| Date | Version | Modification | Auteur |
|------|---------|--------------|--------|
| 2026-11-11 | 1.0 | Version initiale (FIGÉE) | Agent autonome |

**⚠️ Toute modification après cette date doit faire l'objet d'un ERRATUM.**

---

**Statut:** **FIGÉ - PRÊT POUR BENCHMARKS**  
**Prochaine étape:** Exécuter les benchmarks selon cette méthodologie  
**Issue associée:** [À CRÉER après benchmarks]
