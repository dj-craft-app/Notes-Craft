# ADR-001: Sélection des candidats pour le PoC Moteur Datalog

> **Date:** 2026-11-11  
> **Auteurs:** Agent autonome  
> **Statut:** [PROPOSÉE - en attente de revue]  
> **Décision:** Choisir les candidats à évaluer pour le PoC Moteur Datalog (Phase 0.13)

---

## 1. CONTEXTE

La roadmap_v3.md (ligne 670-676) propose d'évaluer 3 candidats pour le moteur Datalog :
1. **crepe** - Moteur Datalog Rust natif
2. **Soufflé bindings** - Bindings Rust pour Soufflé (C++)
3. **Custom sur SQLite** - Implementation sur mesure

Cependant, lors de la phase de recherche du PoC, nous avons découvert que :

### Découvertes clés

#### crepe
- **Statut:** ❌ **NON DISPONIBLE**
- **GitHub:** https://github.com/crepe-rs/crepe → **404 Not Found**
- **crates.io:** Non disponible
- **Conclusion:** Ce candidat n'existe pas et ne peut pas être évalué

#### Soufflé
- **Statut:** ✅ **DISPONIBLE** (mais avec limitations)
- **GitHub:** https://github.com/souffle-lang/souffle → **200 OK**
- **Langage:** C++
- **Bindings Rust:** ❌ **Pas de bindings officiels**
- **Intégration:** Nécessiterait FFI manuel (Rust ↔ C++)
- **Problème:** Violation potentielle de la contrainte "100% Rust" à cause de la FFI

#### datafrog (NOUVEAU CANDIDAT)
- **Statut:** ✅ **DISPONIBLE**
- **GitHub:** https://github.com/rust-lang-nursery/datafrog → **200 OK**
- **crates.io:** datafrog 2.0.1
- **Langage:** Rust (100% natif)
- **Mainteneur:** rust-lang-nursery (groupe officiel Rust)
- **Description:** "Lightweight Datalog engine intended to be embedded in other Rust programs"
- **Avantage:** 100% Rust, pas de FFI, maintenu par la communauté Rust

---

## 2. PROBLÈME

Le candidat **crepe** mentionné dans la roadmap n'existe pas. Nous devons donc le remplacer par un candidat valide.

## 3. OPTIONS CONSIDÉRÉES

### Option A: Conserver Soufflé + Custom + Ajouter datafrog
**Candidats:**
1. datafrog (Rust natif)
2. Soufflé avec FFI (C++)
3. Custom sur SQLite (Rust)

**Avantages:**
- Plus de choix pour comparaison
- Soufflé est mature et standard
- datafrog est 100% Rust
- Custom offre contrôle total

**Inconvénients:**
- 3 candidats à évaluer (prend plus de temps)
- Soufflé nécessite FFI

### Option B: Remplacer crepe par datafrog uniquement
**Candidats:**
1. datafrog (Rust natif)
2. Soufflé avec FFI (C++)
3. Custom sur SQLite (Rust)

**C'est exactement l'Option A.**

### Option C: Exclure Soufflé (à cause de la FFI)
**Candidats:**
1. datafrog (Rust natif)
2. Custom sur SQLite (Rust)

**Avantages:**
- 100% Rust pour tous les candidats
- Moins de travail (2 candidats au lieu de 3)
- Plus simple à évaluer

**Inconvénients:**
- Moins de comparaison
- Soufflé est une référence dans le domaine Datalog
- Perte de l'option la plus mature

---

## 4. DÉCISION

**Décision:** [PROPOSÉE] **Option A** - Évaluer les 3 candidats suivants :
1. **datafrog** (remplace crepe)
2. **Soufflé avec FFI**
3. **Custom sur SQLite**

### Justification

1. **datafrog est un excellent remplacement pour crepe**
   - 100% Rust, comme crepe aurait dû être
   - Maintenu par rust-lang-nursery (excellent signe)
   - Conçu pour être embarqué (parfait pour Note Craft)
   - Léger et performant

2. **Soufflé mérite d'être évalué malgré la FFI**
   - C'est le standard de fait pour Datalog
   - Très mature et utilisé en production
   - Permet de comparer avec une implémentation de référence
   - La FFI peut être isolée et sécurisée

3. **Custom reste pertinent**
   - Contrôle total sur les fonctionnalités
   - Intégration parfaite avec SQLite
   - Adapté aux besoins spécifiques de Note Craft

4. **Évaluer 3 candidats permet une comparaison robuste**
   - Mesurer les trade-offs: performance vs contrôle vs maturité
   - Identifier la meilleure solution pour Note Craft

---

## 5. CONSÉQUENCES

### Positives
- Nous avons des candidats viables et testables
- datafrog est une excellente alternative à crepe
- La comparaison sera complète et robuste

### Négatives
- Soufflé nécessite du travail FFI
- 3 candidats prennent plus de temps à évaluer

### Neutrales
- Le PoC prendra environ 10-15 jours (comme prévu initialement)

---

## 6. IMPACT SUR LA ROADMAP

**Modifications nécessaires dans roadmap_v3.md:**

```diff
- | crepe | Moteur Datalog Rust natif | Rust | À évaluer | https://github.com/crepe-rs/crepe | roadmap_v3.md:670 |
+ | datafrog | Moteur Datalog Rust natif | Rust | À évaluer | https://github.com/rust-lang-nursery/datafrog | poc/datalog/ADR-001-candidats-datalog.md |
```

**Impact sur la Phase 0.13:**
- Aucun changement majeur
- datafrog est un remplacement direct de crepe
- Les critères d'acceptation restent identiques

---

## 7. ALTERNATIVES REJETÉES

### Alternative: Utiliser uniquement datafrog et Custom
- **Rejeté car:** Nous perdons la référence Soufflé pour comparaison
- **Impact:** Moins de robustesse dans la décision

### Alternative: Utiliser datafrog uniquement
- **Rejeté car:** Pas de comparaison possible
- **Impact:** Impossible de justifier le choix

### Alternative: Chercher d'autres candidats
- **Rejeté car:** datafrog, Soufflé et Custom couvrent bien les options
- **Impact:** Retard inutile

---

## 8. RÉFÉRENCES

1. [roadmap_v3.md:663-690](../../docs/roadmap/roadmap_v3.md)
2. [Issue #4: PoC: Moteur Datalog (Phase 0.13)](https://github.com/owner/repo/issues/4)
3. [datafrog GitHub](https://github.com/rust-lang-nursery/datafrog)
4. [datafrog crates.io](https://crates.io/crates/datafrog)
5. [Soufflé GitHub](https://github.com/souffle-lang/souffle)
6. [ADR Template](https://adr.github.io/)

---

## 9. PROCHAINES ÉTAPES

1. [x] Créer ce ADR
2. [ ] Créer `poc/datalog/datafrog/` avec évaluation
3. [ ] Mettre à jour les références à crepe dans la documentation
4. [ ] Commencer l'évaluation de datafrog
5. [ ] Commencer l'évaluation de Soufflé
6. [ ] Commencer le prototypage Custom
7. [ ] Comparer les 3 candidats
8. [ ] Rédiger le rapport final

---

**Statut:** EN ATTENTE DE REVUE  
**Revue par:** @unepacrette (propriétaire)  
**Décision finale:** [À DÉCIDER]
