# Soufflé - PoC Moteur Datalog

> **Statut:** ⚠️ NON TESTÉ - Requiert installation FFI
> **Issue:** #4 - PoC: Moteur Datalog (Phase 0.13)

---

## 📋 Description

**Soufflé** est un moteur Datalog **C++** avec une interface en ligne de commande et une API C++. Il est connu pour ses performances élevées et son support avancé des règles Datalog.

- **Site:** https://souffle-lang.github.io/
- **Dépôt:** https://github.com/souffle-lang/souffle
- **License:** [À VÉRIFIER]
- **Intégration:** FFI Rust ↔ C++ (via bindings)

---

## ⚠️ Statut

**Non testé** dans le cadre de ce PoC car :
1. Soufflé n'est pas installé sur le système de test
2. Nécessite une compilation C++ et des bindings FFI
3. Les critères d'élimination (ADR-002 Section 4.2) indiquent que **FFI est acceptée SEULEMENT si performance ≥ 2x ET sandboxing validé**

---

## 🎯 Pourquoi tester Soufflé ?

Malgré les contraintes FFI, Soufflé pourrait être intéressant car :
- **Performances:** Soufflé est optimisé pour les requêtes Datalog complexes
- **Maturity:** Projet actif avec une communauté solide
- **Features:** Support avancé de Datalog (négation, agrégation, etc.)

---

## 📊 Benchmark attendu

Si Soufflé était testé, on s'attendrait à :
- **Performances:** Potentiellement comparables ou meilleures que datafrog
- **Scalabilité:** Bonne sur les grands jeux de données
- **FFI overhead:** Coût de la communication Rust ↔ C++

---

## 🔧 Comment tester ?

Pour tester Soufflé, il faudrait :

```bash
# 1. Installer Soufflé
# Sur macOS:
$ brew install souffle

# Sur Linux:
$ sudo apt-get install souffle

# 2. Créer un fichier Datalog (.dl)
# Exemple: transitivite.dl

# 3. Compiler et exécuter
$ souffle transitivite.dl
$ ./transitivite

# 4. Créer des bindings Rust
# Utiliser bindgen ou cbindgen pour générer les bindings
```

---

## 📋 Critères d'élimination (ADR-002 Section 4)

| Critère | Évaluation | Statut |
|---------|-------------|--------|
| **Performance** | [À MESURER] | ⚠️ [À TESTER] |
| **Intégration Rust** | FFI Rust ↔ C++ | ⚠️ Acceptable SEULEMENT si performance ≥ 2x |
| **Maintenance** | [À VÉRIFIER] | ⚠️ [À VÉRIFIER] |
| **License** | [À VÉRIFIER] | ⚠️ [À VÉRIFIER] |

---

## 🎯 Décision préliminaire

**Étant donné que datafrog atteint déjà la cible roadmap (< 100ms sur 10K nœuds) sans FFI**, et que Soufflé nécessiterait une intégration FFI, **il est peu probable que Soufflé soit choisi** sauf s'il offre des performances **au moins 2x meilleures** que datafrog.

---

## 📚 Références

- [Soufflé Documentation](https://souffle-lang.github.io/docs.md)
- [Soufflé GitHub](https://github.com/souffle-lang/souffle)
- [FFI entre Rust et C++](https://doc.rust-lang.org/nomicon/ffi.html)

---

**Statut:** 🔴 NON TESTÉ - PAS PRIORITAIRE (datafrog déjà excellent)
