# crepe - Moteur Datalog Rust (Proc Macro)

> **Candidat:** crepe  
> **Type:** Moteur Datalog (Procedural Macro)  
> **Langage:** Rust  
> **Statut:** ✅ **DISPONIBLE** (vérifié via cargo search et crates.io)  
> **Version:** 0.2.0  
> **Licence:** MIT OR Apache-2.0  
> **Date de vérification:** 2026-11-11  
> **Vérifié par:** Agent autonome (méthode corrigée)

---

## ✅ STATUT: CANDIDAT VALIDE

**Preuve de disponibilité (méthode valide selon protocole):**

```bash
# Méthode 1: Recherche via cargo (systématique, pas devinée)
$ cargo search crepe
crepe = "0.2.0"  # Datalog in Rust as a procedural macro

# Méthode 2: Vérification via API crates.io
$ curl -s "https://crates.io/api/v1/crates/crepe" | jq '.crate.name'
"crepe"

# Méthode 3: Vérification via GitHub
$ curl -s -I "https://github.com/ekzhang/crepe"
HTTP/2 200 OK
```

**Conclusion:** crepe **EXISTE** et est un candidat **VALIDE** pour le PoC.

---

## 📋 INFORMATIONS DE BASE

### Métadonnées

| Propriété | Valeur | Source |
|-----------|--------|--------|
| **crates.io** | https://crates.io/crates/crepe | crates.io |
| **GitHub** | https://github.com/ekzhang/crepe | GitHub |
| **Version** | 0.2.0 | crates.io/crates/crepe |
| **Licence** | MIT OR Apache-2.0 | crates.io/crates/crepe |
| **Description** | Datalog in Rust as a procedural macro | crates.io/crates/crepe |
| **Auteur** | ekzhang | crates.io/crates/crepe |
| **Étoiles GitHub** | 537 | GitHub API (2026-11-11) |
| **Forks GitHub** | 22 | GitHub API (2026-11-11) |
| **Dernier commit** | [À VÉRIFIER] | GitHub API |
| **MSRV** | [À VÉRIFIER] | Cargo.toml |
| **Taille du crate** | [À VÉRIFIER] | crates.io |
| **Téléchargements** | [À VÉRIFIER] | crates.io |

---

## 🔍 ÉVALUATION PRÉLIMINAIRE

### Architecture

**Type:** Procedural Macro  
**Paradigme:** Compile-time Datalog compilation  
**Inspiration:** Soufflé et Formulog (selon description GitHub)

**Avantages potentiels:**
- Intégration native avec Rust (proc macro)
- Possiblement support de syntaxe Datalog textuelle
- Compilation-time optimizations

**Inconvénients potentiels:**
- Complexité des proc macros
- Moins flexible à runtime

### Comparaison avec datafrog

| Critère | crepe | datafrog |
|---------|-------|----------|
| **Paradigme** | Compile-time (proc macro) | Runtime (engine) |
| **Syntaxe** | [À VÉRIFIER - probablement Datalog textuel] | API Rust uniquement |
| **100% Rust** | ✅ Oui | ✅ Oui |
| **Maturité** | 537 étoiles | rust-lang-nursery |
| **Licence** | MIT OR Apache-2.0 | Apache-2.0/MIT |

**Hypothèse:** crepe pourrait être **mieux adapté** pour Note Craft si:
- Il supporte une syntaxe Datalog textuelle (comme Logseq)
- La compilation-time est acceptable
- La flexibilité est suffisante

---

## 📊 ÉVALUATION À RÉALISER

### Priorité 1: Vérification de base

- [ ] Installer crepe via `cargo add crepe`
- [ ] Compiler un exemple basique
- [ ] Vérifier la syntaxe supportée
- [ ] Tester l'intégration dans un projet Rust

### Priorité 2: Compatibilité syntaxe

- [ ] Tester si crepe supporte la syntaxe Logseq Datalog
- [ ] Exemple: `[:find ?x :where [?x :tag "test"]]`
- [ ] Vérifier le support des règles, agrégations, etc.

### Priorité 3: Benchmarks

- [ ] Implémenter les requêtes de test (poc/datalog/benchmark/common/queries.datalog)
- [ ] Mesurer les performances sur jeux: 100, 1K, 10K nœuds
- [ ] Utiliser criterion pour des résultats statistiques
- [ ] Comparer avec datafrog

### Priorité 4: Intégration Note Craft

- [ ] Évaluer la facilité d'intégration
- [ ] Vérifier la compatibilité avec les structures Note Craft
- [ ] Estimer le temps de développement

---

## 🔗 LIENS UTILES

- [crates.io: crepe](https://crates.io/crates/crepe)
- [GitHub: ekzhang/crepe](https://github.com/ekzhang/crepe)
- [Documentation](https://docs.rs/crepe) (si disponible)

---

## 📝 PROCHAINES ÉTAPES

1. [ ] Créer un PoC basique avec crepe
2. [ ] Tester la syntaxe Datalog
3. [ ] Exécuter les benchmarks
4. [ ] Documenter les résultats dans `evaluation.md`
5. [ ] Comparer avec datafrog

---

**Statut:** À ÉVALUER  
**Priorité:** HAUTE (candidat potentiellement idéal pour Note Craft)
