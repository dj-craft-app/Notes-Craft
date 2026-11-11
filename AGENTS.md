# AGENTS.md — Note Craft (à copier à la racine du repo)

À placer dans `/Users/amaury/Documents/REBUILD APPS/Note-Craft/AGENTS.md`, puis commit.

---

```markdown
# AGENTS.md — Note Craft

Ce fichier est le contrat de travail pour tout agent IA (ou humain) intervenant
sur ce dépôt. Il est lu avant toute action. En cas de conflit entre ce fichier et
une autre consigne, CE fichier prime ; toute exception exige l'accord explicite
du propriétaire (@unepacrette).

## 1. Identité du projet

**Note Craft** : alternative à Notion, Obsidian et Logseq, codée **exclusivement
en Rust**, avec la direction artistique d'Obsidian.

- Statut : phase de conception terminée, développement non commencé.
- Référence unique : `docs/roadmap/roadmap_v3.md` (validée, tag `v0.1.0-roadmap`).
  Toute tâche de code s'y rattache (phase + étape).
- Backlog : GitHub issues #1 à #5 (PoC et benchmarks issus des [À VÉRIFIER]).

## 2. Carte du dépôt

| Chemin | Rôle | Règle d'accès |
|---|---|---|
| `docs/roadmap/roadmap_v3.md` | Roadmap validée, source de vérité du plan | Lecture. Modification = PR dédiée + justification |
| `docs/verification/` | Dossier de preuve de la roadmap (rapports, contre-lecture) | IMMUTABLE. Aucune modification, jamais |
| `docs/audits/` | Analyses Logseq (code + binaire), design systems Obsidian/Note Craft, architecture Rust | Lecture. Corrections = PR + preuve |
| `docs/history/` | Historique d'audit (v1, v2 rejetée, rapports v1/v2, fichiers de travail) | IMMUTABLE. C'est l'archive, pas la référence |
| `.github/` | Workflows CI (à venir) | Selon phase |
| Racine | `README.md`, `.gitignore`, ce fichier | Mise à jour raisonnée |

## 3. Commandes (état actuel)

Le dépôt ne contient PAS encore de code Rust (aucun workspace Cargo). Les
commandes suivantes s'appliqueront dès la phase 1 et DEVONT être remplies ici
à ce moment-là :

```
# Build :      cargo build
# Tests :      cargo test
# Lint :       cargo clippy --all-targets -- -D warnings
# Format :     cargo fmt --check
# Benchmarks : (à définir, issues #1 et #5)
```

Ne simule JAMAIS le résultat d'une commande non exécutable : exécute-la ou
marque son résultat [À VÉRIFIER].

## 4. Règles absolles du projet

1. **Rust uniquement.** Aucune dépendance à un runtime non-Rust sans validation
   explicite du propriétaire. Pas d'exception « temporaire ».
2. **Ne suppose rien, n'invente rien.** Toute affirmation technique est sourcée
   (fichier:ligne, lien, issue), ou marquée `[À VÉRIFIER]`, ou transformée en
   question au propriétaire. Les erreurs sont attendues : signale-les, ne les
   cache pas.
3. **Les données de validation sont mécaniques.** Un compte, un taux, une
   statistique sortent d'une commande (`wc`, `grep`, `md5`, `cargo test`) dont
   la sortie est citée. JAMAIS d'un paragraphe rédigé. Un chiffre non issu
   d'une commande est une invention.
4. **Aucun secret** (token, clé, mot de passe) dans un fichier, un commit ou
   une issue. Tout hit suspect : STOP + signalement.
5. **Dépôt privé, licence non choisie.** Ne publie RIEN de ce dépôt en dehors
   de GitHub sans accord explicite (les audits portent sur des produits tiers).
6. **Aucun fichier projet hors du repo.** Pas de travail dans `/tmp/` ni
   ailleurs : tout vit dans ce dépôt.

## 5. Workflow de contribution

1. **Une tâche = une branche = une issue.** Nom : `phase-<X>-<sujet>` ou
   `issue-<N>-<sujet>` (ex. `issue-4-datalog-poc`).
2. **Jamais de commit direct sur `main`.** Tout passe par une PR (même solo),
   mergée après vérification.
3. **Messages de commit** : `type: sujet` (types : `feat`, `fix`, `docs`,
   `test`, `chore`, `refactor`). Un commit = un sujet, sans mélange.
4. **Fin de session obligatoire** : commits poussés + `git status` propre +
   résumé distinguant ce qui est PRUVÉ (sorties de commandes citées) de ce qui
   reste OUVERT.
5. **Questions bloquantes** : demander au propriétaire ET créer une issue
   GitHub. Une question sans traçabilité est une question perdue.

## 6. Contrôle qualité : architecture de vérification

Tout livrable majeur (module, PoC, benchmark, document) suit la boucle :

```
production → double vérification indépendante → évaluation → (boucle si < TRÈS BIEN)
```

- **Deux passes de vérification distinctes** : exactitude/sources ET
  faisabilité technique, chacune avec ses preuves par commandes.
- **Barème** : passable / assez bien / bien / très bien. Un livrable n'est
  accepté qu'à **TRÈS BIEN**. Les critères du « très bien » : (a) 100% des
  affirmations sourcées ou marquées [À VÉRIFIER], (b) complétude par rapport
  à la tâche assignée, (c) double vérification sans divergence, (d) questions
  bloquantes résolues.
- **Corrections additives** : corriger = éditer de façon localisée, jamais
  supprimer ou condenser du contenu existant sans validation (leçon de la v2
  de la roadmap : une correction qui rétrécit le document est une perte).
- **Auto-évaluation interdite comme preuve** : l'auteur d'un livrable ne
  valide pas son propre livrable. Le verdict vient de la vérification.

Historique de cette méthode (à lire en cas de doute sur son application) :
`docs/history/` documente comment elle a rejeté une conformité fictive « 95%+ »,
une roadmap amputée de 64% et un « moteur Datalog » qui n'en était pas un.

## 7. Définition de « terminé » pour une contribution

Une tâche est terminée quand TOUS ces points sont vrais :

- [ ] Tests verts : `cargo test` passé (sortie citée dans la PR).
- [ ] Lint propre : `cargo clippy` et `cargo fmt --check` passés.
- [ ] Toute affirmation de la PR est sourcée ou marquée [À VÉRIFIER].
- [ ] Les critères d'acceptation de l'étape de roadmap correspondante
      (voir `docs/roadmap/roadmap_v3.md`) sont cochés avec preuve.
- [ ] La double vérification est passée sans divergence.
- [ ] L'issue GitHub correspondante est fermée avec un lien vers la PR.
- [ ] Aucun fichier hors du périmètre de la tâche n'a été modifié.

## 7.5. Politique de Revue à Deux Niveaux

### REVUE AUTO (au sein d'une phase)
Si CI verte ET auto-note TRÈS BIEN ET l'issue ne touche aucune décision réservée
(AGENTS.md section « ce qui exige le propriétaire »), tu merge toi-même la PR et
tu continues. La trace = CI + auto-note dans la PR ; le propriétaire relit en flux,
et un revert est toujours possible (c'est le rôle des petits commits).

### REVUE HUMAINE OBLIGATOIRE
À CHAQUE FIN DE PHASE (jalon), sur toute ADR, toute décision réservée, tout ce
qui touche docs/verification ou docs/history. Forme : issue "jalon: phase X terminée"
avec le résumé prouvé de la phase, les critères d'acceptation cochés un par un,
et l'ordre de marche vers la phase suivante. Tu passes ensuite à la phase suivante
SANS attendre la réponse : le jalon se valide en async.

**RÈGLE EXPLICITE:** Toute modification de ce fichier (AGENTS.md) **exige une revue
humaine obligatoire** et **jamais d'auto-merge**, même si la CI est verte et la
note est TRÈS BIEN. Ce fichier est le contrat de travail du projet ; son intégrité
prime sur toute politique d'autonomie.

## 8. Étiquette de l'agent

- Cite tes preuves (commande + sortie, fichier:ligne). Un rapport sans preuve
  n'a pas de valeur ici.
- Si une instruction de ce fichier semble contredite par la réalité du code :
  STOP, crée une issue, demande. N'improvise pas.
- Ne réécris pas l'historique : `docs/history/` et `docs/verification/` sont
  l'archive prouvée du projet.
- Quand tu ouvres une PR, son titre suit le même barème : indique la note de
  vérification obtenue (elle doit être TRÈS BIEN).
```