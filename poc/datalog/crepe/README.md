# crepe - CANDIDAT INVALIDE

> **Statut:** ❌ **NON DISPONIBLE**  
> **Date de vérification:** 2026-11-11  
> **Vérifié par:** Agent autonome

---

## ⚠️ AVERTISSEMENT

**Le candidat "crepe" mentionné dans la roadmap_v3.md n'existe pas.**

### Preuves:

1. **GitHub:** https://github.com/crepe-rs/crepe
   - **Résultat:** 404 Not Found
   - **Commande:** `curl -s -I "https://github.com/crepe-rs/crepe"`
   - **Sortie:** `HTTP/2 404`

2. **crates.io:** https://crates.io/crates/crepe
   - **Résultat:** Non disponible
   - **Commande:** `curl -s -w "%{http_code}" "https://crates.io/crates/crepe"`
   - **Sortie:** `403` (ou vide)

3. **Recherche GitHub:**
   - **Commande:** `curl -s "https://api.github.com/search/repositories?q=crepe+datalog+language:rust"`
   - **Résultat:** Aucun dépôt correspondant

---

## Remplacement

**crepe a été remplacé par [datafrog](../datafrog/) dans le PoC.**

### Justification:
- datafrog est un moteur Datalog 100% Rust
- Maintenu par rust-lang-nursery
- Disponible sur crates.io (version 2.0.1)
- Conçu pour être embarqué dans d'autres programmes Rust

### Voir:
- [ADR-001: Sélection des candidats pour le PoC Moteur Datalog](../ADR-001-candidats-datalog.md)
- [datafrog GitHub](https://github.com/rust-lang-nursery/datafrog)
- [datafrog crates.io](https://crates.io/crates/datafrog)

---

**Action requise:** Ne pas évaluer crepe. Utiliser datafrog à la place.
