# ADR-001: Sélection des candidats pour le PoC Moteur Datalog

> **Date:** 2026-11-11  
> **Auteurs:** Agent autonome  
> **Statut:** [PROPOSÉE - en attente de revue]  
> **Décision:** Choisir les candidats à évaluer pour le PoC Moteur Datalog (Phase 0.13)

---

## 🚨 ERRATUM (2026-11-11 12:30 UTC) - PRÉMISSE INITIALE INVALIDÉE

**Erreur identifiée:** La prémisse initiale "**crepe n'existe pas**" était **FAUSSE**.

**Preuve de l'erreur:**
- ❌ **Méthode incorrecte utilisée:** URL devinée `https://github.com/crepe-rs/crepe` → HTTP 404
- ❌ **Règle violée:** Une URL devinée + 404 n'est PAS une preuve d'inexistence (protocole AGENTS.md)

**Méthode correcte appliquée (source: protocole):**
```bash
# Recherche systématique via cargo (méthode valide)
$ cargo search crepe
crepe = "0.2.0"  # Datalog in Rust as a procedural macro

# Vérification via API GitHub (méthode valide)
$ curl -s "https://api.github.com/repos/ekzhang/crepe"
# Réponse: HTTP 200 OK
# Répo: ekzhang/crepe, 537 étoiles, 22 forks, licence Apache-2.0
```

**Faits établis (sourcés):**
| Propriété | Valeur | Source |
|-----------|--------|--------|
| **Nom** | crepe | crates.io/crates/crepe |
| **Version** | 0.2.0 | crates.io/crates/crepe |
| **Description** | Datalog compiler embedded in Rust as a procedural macro | crates.io/crates/crepe |
| **Licence** | Apache-2.0 | GitHub API: ekzhang/crepe/license (2026-11-11) |
| **Auteur** | ekzhang | crates.io/crates/crepe |
| **Dépôt GitHub** | https://github.com/ekzhang/crepe | GitHub API |
| **Étoiles GitHub** | 537 | GitHub API: ekzhang/crepe (2026-11-11) |
| **Forks GitHub** | 22 | GitHub API: ekzhang/crepe (2026-11-11) |
| **Dernier commit** | 2025-12-14T00:12:35Z | GitHub API: ekzhang/crepe (2026-11-11) |
| **Dernière release** | v0.2.0 (2025-12-14T00:12:46Z) | GitHub API: ekzhang/crepe/releases/latest (2026-11-11) |

**Conclusion:** 
✅ **crepe EXISTE et est un candidat VALIDE** pour le PoC.  
✅ **crepe doit être réintégré comme candidat à part entière** dans l'évaluation.

---

## 📋 COMPARAISON CREPE VS DATAFROG (Candidats à part entière)

**Les deux sont maintenant des candidats valides et doivent être comparés.**

| Critère | crepe | datafrog | Source |
|---------|-------|----------|--------|
| **Type** | Procedural macro (compile-time) | Engine (runtime) | Documentation |
| **Version** | 0.2.0 | 2.0.1 | crates.io |
| **Licence** | Apache-2.0 | Apache-2.0 | GitHub API |
| **Étoiles GitHub** | 537 | 911 | GitHub API (2026-11-11) |
| **Forks GitHub** | 22 | 48 | GitHub API (2026-11-11) |
| **Dernier commit** | 2025-12-14 | 2026-10-28 | GitHub API (2026-11-11) |
| **Dernière release** | v0.2.0 (2025-12-14) | v2.0.1 (2019-01-02) | GitHub API (2026-11-11) |
| **Maintenance** | ~11 mois d'activité | 14 jours d'activité | GitHub API (2026-11-11) |

**Analyse:**
- **datafrog:** Dernier commit très récent (14 jours), mais dernière release ancienne (2019)
- **crepe:** Dernier commit moins récent (~11 mois), mais dernière release récente (2025-12-14)
- **Les deux sont maintenus activement** (critère ADR-002: > 24 mois sans activité = éliminé)

---

## 1. CONTEXTE

La roadmap_v3.md (ligne 670-676) propose d'évaluer 3 candidats pour le moteur Datalog :
1. **crepe** - Moteur Datalog Rust natif
2. **Soufflé bindings** - Bindings Rust pour Soufflé (C++)
3. **Custom sur SQLite** - Implementation sur mesure

L investigation initiale a révélé que le candidat **crepe** mentionné dans la roadmap existe bel et bien (voir ERRATUM). 

De plus, une recherche systématique a identifié **datafrog** comme candidat supplémentaire pertinent.

---

## 2. PROBLÈME (CORRIGÉ)

**Statut:** Nous avons **4 candidats valides** à évaluer pour le PoC Moteur Datalog.

**Candidats identifiés:**
1. **crepe** (v0.2.0) - Procedural macro Datalog
2. **datafrog** (v2.0.1) - Lightweight Datalog engine
3. **Soufflé** (C++) - Standard Datalog (via FFI)
4. **Custom** - Implementation sur mesure sur SQLite

**Question:** Combien de candidats évaluer pour le PoC Phase 0.13 ?

---

## 3. OPTIONS CONSIDÉRÉES

### Option A: Évaluer les 4 candidats [RECOMMANDÉE]

**Candidats:**
1. **crepe** (v0.2.0) - Procedural macro Datalog
2. **datafrog** (v2.0.1) - Lightweight Datalog engine
3. **Soufflé** (C++) - Standard Datalog
4. **Custom** - Implementation sur mesure

**Avantages:**
- Comparaison **exhaustive** de toutes les options valides
- Respecte la roadmap originale (crepe + Soufflé + Custom)
- Ajoute datafrog comme candidat supplémentaire identifié
- Robustesse maximale dans la décision finale
- Permet de mesurer les trade-offs: procedural macro vs engine vs FFI vs custom

**Inconvénients:**
- 4 candidats à évaluer (durée estimée: ~15-20 jours au lieu de 10-15)
- Soufflé nécessite FFI Rust/C++ (complexité supplémentaire)

### Option B: Évaluer 3 candidats (exclure Soufflé)

**Candidats:**
1. crepe (Rust natif)
2. datafrog (Rust natif)
3. Custom sur SQLite (Rust)

**Avantages:**
- 100% Rust pour tous les candidats
- Moins de travail (durée estimée: ~12-15 jours)
- Évite la complexité FFI

**Inconvénients:**
- Exclut Soufflé (référence dans le domaine Datalog)
- Moins de robustesse dans la comparaison
- Perte de la comparaison avec le standard de fait

### Option C: Évaluer 3 candidats (exclure Custom)

**Candidats:**
1. crepe (Rust natif)
2. datafrog (Rust natif)
3. Soufflé avec FFI (C++)

**Avantages:**
- Évalue 2 implémentations Rust + 1 référence
- Bon équilibre entre contrôle et maturité

**Inconvénients:**
- Exclut l'option Custom (contrôle total sur les fonctionnalités)
- Soufflé nécessite FFI

### Option D: Évaluer 2 candidats uniquement (crepe + datafrog)

**Candidats:**
1. crepe (Rust natif)
2. datafrog (Rust natif)

**Avantages:**
- 100% Rust
- Le moins de travail (durée estimée: ~8-10 jours)
- Focalisé sur les implémentations Rust natives

**Inconvénients:**
- Exclut Soufflé (référence)
- Exclut Custom (contrôle total)
- Comparaison très limitée

---

## 4. DÉCISION

**Décision:** [PROPOSÉE] **Option A** - Évaluer les **4 candidats** suivants :
1. **crepe** (v0.2.0)
2. **datafrog** (v2.0.1)
3. **Soufflé** avec FFI (C++)
4. **Custom** sur SQLite

### Justification

1. **Respect de la roadmap:** La roadmap mentionne crepe, Soufflé et Custom. Les évaluer tous les trois respecte le plan original.

2. **Découverte supplémentaire:** datafrog a été identifié comme candidat valide supplémentaire. L'évaluer permet une comparaison complète.

3. **Robustesse:** Évaluer 4 candidats permet de mesurer différents paradigmes:
   - **crepe:** Procedural macro (compilation-time Datalog)
   - **datafrog:** Engine léger (runtime Datalog)
   - **Soufflé:** Standard de fait (C++ avec FFI)
   - **Custom:** Contrôle total (intégration SQLite)

4. **Décision éclairée:** Plus de candidats = plus de données pour une décision robuste.

5. **Durée acceptable:** ~15-20 jours reste dans le cadre de la Phase 0.13 (estimée à 10 jours initialement, mais extensible).

---

## 5. CONSÉQUENCES

### Positives
- Nous avons une comparaison exhaustive de toutes les options
- La décision finale sera basée sur des données complètes
- Tous les candidats mentionnés dans la roadmap sont évalués

### Négatives
- Plus de travail que prévu initialement
- Soufflé nécessite du travail FFI
- La phase prendra ~15-20 jours au lieu de 10

### Neutrales
- Le PoC produira des données précieuses pour Note Craft
- Les benchmarks pourront être réutilisés pour d'autres décisions

---

## 6. IMPACT SUR LA ROADMAP

**Modifications nécessaires dans roadmap_v3.md:**

```diff
- | crepe | Moteur Datalog Rust natif | Rust | À évaluer | https://github.com/crepe-rs/crepe | roadmap_v3.md:670 |
+ | crepe | Moteur Datalog Rust natif (proc macro) | Rust | À évaluer | https://github.com/ekzhang/crepe | poc/datalog/ADR-001-candidats-datalog.md |

- | Candidat | Type | Langage | Statut | Lien | Source |
+ | crepe | Moteur Datalog Rust natif (proc macro) | Rust | À évaluer | https://github.com/ekzhang/crepe | poc/datalog/ADR-001-candidats-datalog.md |
+ | datafrog | Moteur Datalog Rust natif (lightweight) | Rust | À évaluer | https://github.com/rust-lang-nursery/datafrog | poc/datalog/ADR-001-candidats-datalog.md |
```

**Impact sur la Phase 0.13:**
- **Durée:** ~15-20 jours (au lieu de 10 jours initialement prévus)
- **Candidats:** 4 au lieu de 3
- **Critères d'acceptation:** Identiques (PoC fonctionnel + comparaison documentée)

---

## 7. INFORMATIONS SUR LES CANDIDATS

### 7.1 crepe (v0.2.0)

| Propriété | Valeur | Source |
|-----------|--------|--------|
| **crates.io** | https://crates.io/crates/crepe | crates.io |
| **GitHub** | https://github.com/ekzhang/crepe | GitHub |
| **Version** | 0.2.0 | crates.io/crates/crepe |
| **Licence** | Apache-2.0 | crates.io/crates/crepe |
| **Description** | Datalog compiler embedded in Rust as a procedural macro | crates.io/crates/crepe |
| **Auteur** | ekzhang | crates.io/crates/crepe |
| **Étoiles GitHub** | 537 | GitHub API: ekzhang/crepe (2026-11-11) |
| **Forks GitHub** | 22 | GitHub API: ekzhang/crepe (2026-11-11) |
| **Dernier commit** | 2025-12-14T00:12:35Z | GitHub API: ekzhang/crepe (2026-11-11) |
| **Dernière release** | v0.2.0 (2025-12-14T00:12:46Z) | GitHub API: ekzhang/crepe/releases/latest (2026-11-11) |
| **MSRV** | [À VÉRIFIER] | Cargo.toml: ekzhang/crepe |
| **Taille du crate** | [À VÉRIFIER] | crates.io/crates/crepe |
| **Téléchargements** | [À VÉRIFIER] | crates.io/crates/crepe |

### 7.2 datafrog (v2.0.1)

| Propriété | Valeur | Source |
|-----------|--------|--------|
| **crates.io** | https://crates.io/crates/datafrog | crates.io |
| **GitHub** | https://github.com/rust-lang/datafrog | GitHub |
| **Version** | 2.0.1 | Cargo.toml: rust-lang/datafrog |
| **Licence** | Apache-2.0 | Cargo.toml: rust-lang/datafrog |
| **Description** | A lightweight Datalog engine in Rust | GitHub: rust-lang/datafrog |
| **Auteurs** | Frank McSherry, The Rust Project Developers, Datafrog Developers | Cargo.toml: rust-lang/datafrog |
| **Étoiles GitHub** | 911 | GitHub API: rust-lang/datafrog (2026-11-11) |
| **Forks GitHub** | 48 | GitHub API: rust-lang/datafrog (2026-11-11) |
| **Dernier commit** | 2026-10-28T12:50:23Z | GitHub API: rust-lang/datafrog (2026-11-11) |
| **Dernière release** | v2.0.1 (2019-01-02T19:37:30Z) | GitHub API: rust-lang/datafrog/tags (2026-11-11) |
| **MSRV** | 2018 Edition | Cargo.toml: rust-lang/datafrog |
| **Taille du crate** | [À VÉRIFIER] | crates.io/crates/datafrog |
| **Téléchargements** | [À VÉRIFIER] | crates.io/crates/datafrog |

### 7.3 Soufflé

| Propriété | Valeur | Source |
|-----------|--------|--------|
| **Site web** | https://souffle-lang.github.io | souffle-lang.github.io |
| **GitHub** | https://github.com/souffle-lang/souffle | GitHub |
| **Langage** | C++ | GitHub: souffle-lang/souffle |
| **Licence** | [À VÉRIFIER] | GitHub: souffle-lang/souffle |
| **Description** | A declarative logic programming language | GitHub: souffle-lang/souffle |
| **Bindings Rust** | ❌ Pas de bindings officiels | Recherche crates.io: souffle |
| **Étoiles GitHub** | [À VÉRIFIER] | GitHub API: souffle-lang/souffle |
| **Forks GitHub** | [À VÉRIFIER] | GitHub API: souffle-lang/souffle |

### 7.4 Custom sur SQLite

| Propriété | Valeur | Source |
|-----------|--------|--------|
| **Type** | Implementation sur mesure | roadmap_v3.md:676 |
| **Langage** | Rust | roadmap_v3.md:676 |
| **Stockage** | SQLite (rusqlite) | rust_architecture_choices.json:180 |
| **Description** | Moteur Datalog custom intégré avec SQLite | roadmap_v3.md:676 |
| **Avantage** | Contrôle total, intégration parfaite | roadmap_v3.md:676 |
| **Inconvénient** | Développement long (4-6 semaines) | rust_architecture_choices.json:270 |

---

## 8. COMPARAISON PRÉLIMINAIRE

### 8.1 crepe vs datafrog

| Critère | crepe | datafrog | Note |
|---------|-------|----------|------|
| **Paradigme** | Procedural macro (compile-time) | Engine (runtime) | Différents |
| **Syntaxe** | [À VÉRIFIER] | API Rust (pas de syntaxe textuelle) | crepe peut supporter syntaxe Datalog |
| **100% Rust** | ✅ Oui | ✅ Oui | Les deux sont valides |
| **Maturité** | 537 étoiles | rust-lang-nursery | Les deux sont matures |
| **Licence** | MIT OR Apache-2.0 | Apache-2.0/MIT | Les deux sont permissives |

**Hypothèse:** crepe pourrait être **mieux adapté** pour Note Craft car il utilise probablement une syntaxe Datalog textuelle (procedural macro), ce qui faciliterait la compatibilité avec Logseq.

### 8.2 Soufflé vs Custom

| Critère | Soufflé | Custom | Note |
|---------|--------|--------|------|
| **Langage** | C++ | Rust | Soufflé nécessite FFI |
| **Maturité** | Très mature (standard) | À développer | Trade-off maturité vs contrôle |
| **Performance** | [À VÉRIFIER] | [À VÉRIFIER] | Benchmarks nécessaires |
| **Intégration** | FFI nécessaire | Native | Custom a l'avantage |
| **Flexibilité** | Standard | Adapté à Note Craft | Custom a l'avantage |

---

## 9. ALTERNATIVES REJETÉES

### Alternative: Utiliser uniquement datafrog et Custom
- **Rejeté car:** Exclut crepe (découvert comme valide) et Soufflé (référence)
- **Impact:** Décision moins robuste

### Alternative: Utiliser uniquement crepe et datafrog
- **Rejeté car:** Exclut Soufflé et Custom
- **Impact:** Comparaison trop limitée

---

## 10. RÉFÉRENCES

1. [roadmap_v3.md:663-690](../../docs/roadmap/roadmap_v3.md)
2. [Issue #4: PoC: Moteur Datalog (Phase 0.13)](https://github.com/dj-craft-app/Notes-Craft/issues/4)
3. [crepe crates.io](https://crates.io/crates/crepe)
4. [crepe GitHub](https://github.com/ekzhang/crepe)
5. [datafrog crates.io](https://crates.io/crates/datafrog)
6. [datafrog GitHub](https://github.com/rust-lang-nursery/datafrog)
7. [Soufflé GitHub](https://github.com/souffle-lang/souffle)
8. [Soufflé Documentation](https://souffle-lang.github.io)

---

## 11. PROCHAINES ÉTAPES

1. [ ] Vérifier les métriques manquantes pour crepe et datafrog (MSRV, téléchargements, etc.)
2. [ ] Créer issue GitHub "revue: ADR-001" (@unepacrette)
3. [ ] Commenter l'issue #4 avec progrès et candidats
4. [ ] Évaluer crepe (procedural macro Datalog)
5. [ ] Compléter l'évaluation de datafrog (benchmarks statistiques)
6. [ ] Évaluer Soufflé (FFI Rust/C++)
7. [ ] Prototyper Custom sur SQLite
8. [ ] Comparer les 4 candidats
9. [ ] Documenter la décision finale

---

**Statut:** EN ATTENTE DE REVUE + MISE À JOUR EN COURS  
**Revue par:** @unepacrette (propriétaire)  
**Décision finale:** [À DÉCIDER]  
**Issue associée:** [À CRÉER - revue: ADR-001]