#!/usr/bin/env python3
"""
verify_roadmap_citations.py — Vérificateur read-only des citations Source de la roadmap.

Usage :
    python3 scripts/verify_roadmap_citations.py [chemin_roadmap]
    (défaut : docs/roadmap/roadmap_v3.md, relatif à la racine du dépôt)

Pour CHAQUE citation `fichier.json:NNN` ou `fichier.json:NNN-MMM` de la roadmap :

  (1) le fichier cité existe dans docs/audits/ ;
  (2) la (ou les, pour une plage) ligne(s) citée(s) existe(nt) ;
  (3) la ligne citée n'est pas purement structurelle ({ } ] , [ vide) ;
  (4) vérification sémantique fuzzy entre le sujet de la ligne roadmap citante
      et le contenu ligne / bloc cité :
        - logseq_audit_results.json : dans la zone "features", la ligne citée
          doit appartenir au bloc feature d'un "id" dont le "name" matche le
          sujet (fuzzy : mots-clés, préfixes, insensible casse/accents).
          Hors zone features (metadata, statistics, recommendations,
          dependencies_tree), la ligne citée ou sa clé englobante doit matcher.
        - obsidian_design_system.json / rust_architecture_choices.json :
          logique adaptée — la ligne citée + son bloc englobant (clé ou entrée
          "name") doivent matcher le sujet, soit par mot-clé distinctif
          (token non générique), soit par valeur mesurable (identifiants comme
          'fts5', '590', 'tokio', ...). Tout token hexadécimal (#dadada) du
          sujet DOIT être présent dans le contexte (valeur exacte).
        - plages : au moins une ligne de la plage doit matcher le sujet.
        - MANUAL_KEYWORDS : mappings manuels documentés (issue #112) pour les
          sujets sans entrée directe dans le JSON ; chaque usage est signalé
          dans la sortie.

Règles du sujet analysé :
    - Par défaut le sujet = la ligne citante (pointeurs retirés).
    - Si la ligne citante porte moins de 3 tokens exploitables (ex : lignes
      '*Source: fichier:N*'), le sujet s'étend aux 6 lignes précédentes.
    - Un sujet dont TOUS les tokens sont génériques (system, features, ...)
      est validé par un seul match générique (couvre les collections
      citées en plage, ex « Core System (11 features) »).

Sortie : récap « OK: N/N » + détails des échecs. Exit 0 si N/N OK, exit 1 sinon.

Ce script est STRICTEMENT read-only : il n'écrit jamais aucun fichier.
"""

import os
import re
import sys
import unicodedata

# ---------------------------------------------------------------------------
# Configuration
# ---------------------------------------------------------------------------

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
AUDITS_DIR = os.path.join(REPO_ROOT, "docs", "audits")
DEFAULT_ROADMAP = os.path.join(REPO_ROOT, "docs", "roadmap", "roadmap_v3.md")

CITATION_RE = re.compile(r"([A-Za-z0-9_]+\.json):(\d+)(?:-(\d+))?")

# Fenêtre de contexte pour les lignes citantes trop pauvres (*Source: ...*).
PREV_CONTEXT_LINES = 6
MIN_SELF_TOKENS = 3

# Tokens trop génériques : ne suffisent pas seuls à valider un bloc feature.
GENERIC_TOKENS = {
    "system", "systeme", "core", "ui", "editor", "data",
    "plugins", "plugin", "query", "features", "feature", "interface",
    "source", "sources", "management",
}

HEX_RE = re.compile(r"^[0-9a-f]{6}$")

# Mappings manuels documentés (sujet -> mots-clés acceptés dans le contexte).
# Justifications détaillées : rapport issue #112.
MANUAL_KEYWORDS = {
    ("rust_architecture_choices.json", "migration"):
        # Aucune entrée "migration" dans le JSON : la migration concerne la
        # base de données retenue (SQLite). Cible : bloc database_options.
        ["sqlite", "database", "storage"],
    ("rust_architecture_choices.json", "standard"):
        # "Standard etabli" = la spécification CommonMark du format Markdown.
        ["commonmark", "specification"],
    ("rust_architecture_choices.json", "transactions"):
        # ACID : documenté par la source officielle SQLite citée dans le bloc.
        ["sqlite"],
    ("rust_architecture_choices.json", "peer"):
        # Peer discovery : assuré par libp2p (transport réseau P2P retenu).
        ["p2p", "networking", "libp2p"],
    ("rust_architecture_choices.json", "multiplateforme"):
        # Support multiplateforme de wry (WKWebView/WebView2/WebKitGTK) : non
        # détaillé dans le JSON, référent = l'option WebView (wry) retenue.
        ["webview", "wry", "multiplatform"],
    ("rust_architecture_choices.json", "capabilities"):
        # Plugin Capabilities System : non documenté dans le JSON, référent =
        # la décision plugin_system (système de plugins WASM retenu).
        ["plugin", "wasm", "wasmtime"],
    ("rust_architecture_choices.json", "configuration"):
        # Plugin Configuration : non documenté dans le JSON, référent =
        # la décision plugin_system (système de plugins WASM retenu).
        ["plugin", "wasm", "wasmtime"],
}

# ---------------------------------------------------------------------------
# Utilitaires
# ---------------------------------------------------------------------------


def strip_accents(text):
    nfkd = unicodedata.normalize("NFKD", text)
    return "".join(c for c in nfkd if not unicodedata.combining(c))


def normalize(text):
    return strip_accents(text).lower()


def extract_tokens(text):
    """Tokens du sujet : fragments alphanumériques (accents ôtés, casse basse).

    - tokens alphabétiques : >= 3 lettres ;
    - tokens courts (ui, p2p, crdt, git, api, ... uniquement connus) ;
    - tokens numériques/valeurs (hex, px, px, %, identifiants) : le '#' est
      retiré ; ils matchent par simple inclusion.
    """
    short_whitelist = {"ui", "p2p", "crdt", "git", "api", "lod", "was", "db"}
    raw = normalize(text).replace("#", " ")
    tokens = []
    for part in re.split(r"[^a-z0-9]+", raw):
        if not part:
            continue
        if any(c.isdigit() for c in part):
            if len(part) >= 2:  # évite les faux positifs "3" în "3d"
                tokens.append(part)
        elif len(part) >= 3:
            tokens.append(part)
        elif part in short_whitelist:
            tokens.append(part)
    return tokens


def word_matches(token, context_norm):
    """Matche un token contre un contexte normalisé.

    - tokens numériques (avec chiffre) : inclusion directe ;
    - tokens alphabétiques : inclusion, ou préfixe commun >= 3 avec un mot du
      contexte (tolère déclinaisons : sync/synchronization,
      stockage/storage, typographie/typography, ...).
    """
    if not token or not context_norm:
        return False
    if any(c.isdigit() for c in token):
        return token in context_norm
    if token in context_norm:
        return True
    for word in re.split(r"[^a-z0-9]+", context_norm):
        if not word:
            continue
        common = 0
        for a, b in zip(word, token):
            if a != b:
                break
            common += 1
        if common >= 4:
            return True
    return False


def match_tokens(tokens, context):
    """Renvoie (strong_matches, weak_matches) pour un contexte donné.

    strong = token distinctif (hors GENERIC_TOKENS) matché.
    weak = token générique matché.
    """
    ctx = normalize(context)
    strong = []
    weak = []
    for tok in tokens:
        if word_matches(tok, ctx):
            if tok in GENERIC_TOKENS:
                weak.append(tok)
            else:
                strong.append(tok)
    return strong, weak


def is_structural(line):
    """Ligne purement structurelle : accolades, crochets, virgules, vide."""
    return re.fullmatch(r"[\s{}\[\],]*", line) is not None


def load_lines(path):
    with open(path, "r", encoding="utf-8") as fh:
        return fh.read().split("\n")


def subject_for(rm_lines, rm_no):
    """Sujet = ligne citante seule si assez riche, sinon fenêtre précédente."""
    self_line = CITATION_RE.sub(" ", rm_lines[rm_no - 1])
    if len(extract_tokens(self_line)) >= MIN_SELF_TOKENS:
        return self_line
    start = max(0, rm_no - 1 - PREV_CONTEXT_LINES)
    return "\n".join(
        CITATION_RE.sub(" ", line) for line in rm_lines[start:rm_no])


# ---------------------------------------------------------------------------
# Logique par fichier source
# ---------------------------------------------------------------------------


class LogseqContext:
    """Cartographie de logseq_audit_results.json (id/name des features,
    zones metadata/statistics/recommendations/dependencies_tree)."""

    def __init__(self, path):
        self.lines = load_lines(path)
        self.features_ids = []  # (no_ligne, id, name)
        for no, line in enumerate(self.lines, 1):
            m = re.search(r'"id": "([^"]+)"', line)
            if m and 48 < no < 1491:
                name = ""
                if no + 1 <= len(self.lines):
                    nm = re.match(r'\s*"name": "([^"]+)"', self.lines[no])
                    if nm:
                        name = nm.group(1)
                self.features_ids.append((no, m.group(1), name))

    def context_for_line(self, no):
        """Contexte sémantique d'une ligne citée de logseq_audit_results.json."""
        if no <= 0 or no > len(self.lines):
            return ""
        line = self.lines[no - 1]
        if 48 < no < 1491:  # zone features : id + name du bloc englobant
            block = None
            for id_no, fid, fname in self.features_ids:
                if id_no <= no:
                    block = (fid, fname)
                else:
                    break
            if block:
                return "%s %s" % block
            return line
        if no <= 47:  # metadata + statistics
            subkey = ""
            for prev in range(no, max(0, no - 40), -1):
                m = re.match(r'\s{4}"([a-z_0-9]+)": ', self.lines[prev - 1])
                if m:
                    subkey = m.group(1)
                    break
            return ("metadata statistics %s %s" % (subkey, line)).strip()
        if no >= 1491:  # recommendations
            if 1638 <= no <= 1904 or '"dependencies_tree"' in line:
                key = ""
                for prev in range(no, 1637, -1):
                    m = re.match(r'\s{6}"([^"]+)": \[$', self.lines[prev - 1])
                    if m:
                        key = m.group(1)
                        break
                return ("recommendations dependencies_tree %s %s"
                        % (key, line)).strip()
            key = ""
            for prev in range(no, 1490, -1):
                m = re.match(r'\s{4}"([a-z_0-9]+)": ', self.lines[prev - 1])
                if m:
                    key = m.group(1)
                    break
            return ("recommendations %s %s" % (key, line)).strip()
        return line

    def feature_names_in_range(self, start, end):
        return [fname for id_no, _fid, fname in self.features_ids
                if start <= id_no <= end and fname]


class GenericJsonContext:
    """Contexte par indentation pour obsidian_design_system.json et
    rust_architecture_choices.json (clés + entrées "name")."""

    def __init__(self, path):
        self.lines = load_lines(path)

    @staticmethod
    def _indent(line):
        return len(line) - len(line.lstrip())

    def context_for_line(self, no):
        line = self.lines[no - 1]
        ctx = line
        if no > 1:
            base = self._indent(line)
            if base == 0 or not line.lstrip().startswith('"'):
                base += 1
            for prev in range(no - 1, max(0, no - 60), -1):
                prev_line = self.lines[prev - 1]
                m = re.match(r'\s*"([^"]+)": ', prev_line)
                if not m:
                    m = re.match(r'\s*\{?"name": "([^"]+)"', prev_line)
                if m and self._indent(prev_line) < base:
                    ctx = "%s %s" % (m.group(1), line)
                    break
        return ctx

    def context_for_range(self, start, end):
        chunks = []
        for no in range(start, min(end, start + 80) + 1):
            chunks.append(self.context_for_line(no))
        if end > start + 80:  # échantillonne aussi la fin de la plage
            for no in range(max(start, end - 40), end + 1):
                chunks.append(self.context_for_line(no))
        return "\n".join(chunks)


# ---------------------------------------------------------------------------
# Vérification d'une citation
# ---------------------------------------------------------------------------


def eval_subject(tokens, contexts):
    """Applique la règle de validation pour un sujet et des contextes.

    Renvoie passed :
      - tout token hex du sujet (ex : dadada issu de #dadada) DOIT
        apparaître dans un des contextes (valeur exacte) ;
      - sinon : >= 1 token distinctif (strong) matché suffit ;
      - sujet réduit à des tokens génériques (« Core System (11 features) ») :
        >= 1 match générique suffit (collections citées en plage) ;
      - sujet purement numérique : toutes les valeurs doivent figurer.
    """
    hex_tokens = [t for t in tokens if HEX_RE.match(t)]
    if hex_tokens:
        for ctx in contexts:
            ctx_norm = normalize(ctx)
            if all(h in ctx_norm for h in hex_tokens):
                return True
        return False
    alpha_strong = [t for t in tokens
                    if t not in GENERIC_TOKENS
                    and not any(c.isdigit() for c in t)]
    numeric = [t for t in tokens if any(c.isdigit() for c in t)]
    for ctx in contexts:
        if not ctx:
            continue
        strong, weak = match_tokens(tokens, ctx)
        if strong:
            return True
        if not alpha_strong and weak:
            return True
    if not alpha_strong and numeric:
        for ctx in contexts:
            if all(word_matches(t, ctx) for t in numeric):
                return True
    return False


def try_manual(fname, tokens, contexts, manual_hits, rm_no, label):
    """Tente les mappings manuels documentés. Renvoie True si résolu."""
    for (mfile, mkey), words in MANUAL_KEYWORDS.items():
        if mfile != fname:
            continue
        if mkey in tokens:
            ok = False
            for ctx in contexts:
                ctx_norm = normalize(ctx)
                if any(w in ctx_norm for w in words):
                    ok = True
                    break
            if ok:
                manual_hits.append((rm_no, label, mkey))
                return True
    return False


def check_citations(roadmap_path, audits_dir):
    with open(roadmap_path, "r", encoding="utf-8") as fh:
        rm_lines = fh.read().split("\n")

    sources = {}
    failures = []
    manual_hits = []
    total = 0

    def get_source(fname):
        if fname not in sources:
            path = os.path.join(audits_dir, fname)
            sources[fname] = (os.path.isfile(path), path)
        return sources[fname]

    for rm_no, rm_line in enumerate(rm_lines, 1):
        for m in CITATION_RE.finditer(rm_line):
            total += 1
            fname = m.group(1)
            start = int(m.group(2))
            end = int(m.group(3)) if m.group(3) else None
            label = ("%s:%d-%d" % (fname, start, end) if end
                     else "%s:%d" % (fname, start))
            where = "roadmap:L%d [%s]" % (rm_no, label)

            def fail(reason):
                failures.append((where, reason))

            exists, path = get_source(fname)
            if not exists:
                fail("fichier introuvable dans docs/audits/")
                continue

            if fname.endswith("logseq_audit_results.json"):
                src = LogseqContext(path)
            else:
                src = GenericJsonContext(path)
            n_lines = len(src.lines)

            error = None
            for b in ([start] if end is None else [start, end]):
                if b < 1 or b > n_lines:
                    error = "ligne %d hors fichier (1-%d)" % (b, n_lines)
                    break
                if is_structural(src.lines[b - 1]):
                    error = "ligne %d purement structurelle" % b
                    break
            if error:
                fail(error)
                continue

            subject = subject_for(rm_lines, rm_no)
            tokens = extract_tokens(subject)
            if not tokens:
                fail("sujet sans token exploitable")
                continue

            if end is not None:
                if isinstance(src, LogseqContext):
                    contexts = [" ".join(src.feature_names_in_range(start, end))]
                    contexts.append(src.context_for_line(start))
                else:
                    contexts = [src.context_for_range(start, end)]
            else:
                contexts = [src.context_for_line(start)]

            passed = eval_subject(tokens, contexts)
            if not passed:
                passed = try_manual(fname, tokens, contexts,
                                    manual_hits, rm_no, label)
            if not passed:
                fail("aucune correspondance sémantique entre le sujet de la "
                     "ligne et le bloc cité (fuzzy)")

    return total, failures, manual_hits


def main(argv):
    roadmap = argv[1] if len(argv) > 1 else DEFAULT_ROADMAP
    if not os.path.isfile(roadmap):
        print("ERREUR: roadmap introuvable : %s" % roadmap)
        return 2
    if not os.path.isdir(AUDITS_DIR):
        print("ERREUR: dossier docs/audits/ introuvable : %s" % AUDITS_DIR)
        return 2

    total, failures, manual_hits = check_citations(roadmap, AUDITS_DIR)
    ok = total - len(failures)

    print("Vérification des citations Source de : %s" % roadmap)
    print("  Citations .json:NNN (et plages) analysées : %d" % total)
    print("  OK : %d/%d" % (ok, total))
    if manual_hits:
        print("  Résolutions manuelles documentées (MANUAL_KEYWORDS) : %d"
              % len(manual_hits))
        for rm_no, label, key in manual_hits:
            print("    - roadmap:L%d [%s] <- mapping manuel '%s'"
                  % (rm_no, label, key))
    if failures:
        print("  ÉCHECS : %d" % len(failures))
        for where, reason in failures:
            print("    - %s : %s" % (where, reason))
        print("\nRÉCAP : OK %d/%d — exit 1" % (ok, total))
        return 1
    print("\nRÉCAP : OK %d/%d — exit 0" % (ok, total))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
