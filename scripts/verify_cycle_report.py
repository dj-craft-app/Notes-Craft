#!/usr/bin/env python3
"""
verify_cycle_report.py — Audit mécanique read-only d'une PR (Dossier de Décision).

Usage :
    python3 scripts/verify_cycle_report.py <numéro_PR>           # PR ouverte (gate propriétaire)
    python3 scripts/verify_cycle_report.py <numéro_PR> --merged  # audit a posteriori (calibration)

Vérifie, via gh api (strictement read-only) :
  1. PR existe, base = main, et (selon le mode) ouverte-non-mergée ou mergée
  2. CI du head de la PR : au moins un run completed/success, URL citée
  3. Fichiers changés vs liste réservée (AGENTS.md, .vibe/, docs/verification,
     docs/history, .github/) — tout hit = ÉCHEC (revue humaine obligatoire)
  4. Cargo.toml/Cargo.lock modifiés -> INFO (doit être justifié dans le rapport)
  5. Scan de secrets dans le diff (tokens, clés privées, mots de passe)
  6. scripts/verify_roadmap_citations.py : exit 0
  7. Au moins une issue référencée dans le corps de la PR

Sortie : lignes OK/ÉCHEC + verdict final PASS/FAIL. Exit 0 si PASS, 1 si FAIL.
Ce script n'écrit JAMAIS aucun fichier.
"""
import json
import os
import re
import subprocess
import sys

REPO = os.environ.get("GH_REPO", "dj-craft-app/Notes-Craft")
RESERVED = ["AGENTS.md", ".vibe/", "docs/verification/", "docs/history/", ".github/"]
SECRET_PATTERNS = [
    r"ghp_[A-Za-z0-9]{20,}",
    r"github_pat_[A-Za-z0-9_]{20,}",
    r"gho_[A-Za-z0-9]{20,}",
    r"AKIA[0-9A-Z]{16}",
    r"-----BEGIN [A-Z ]*PRIVATE KEY-----",
    r"(?i)(password|secret|token)\s*[:=]\s*[\"'][^\"']{8,}[\"']",
]


def gh(path):
    r = subprocess.run(["gh", "api", path], capture_output=True, text=True)
    if r.returncode != 0:
        print(f"ÉCHEC — gh api {path} : {r.stderr.strip()[:200]}")
        sys.exit(2)
    return json.loads(r.stdout)


def main():
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    merged_mode = "--merged" in sys.argv
    if not args or not args[0].isdigit():
        print(__doc__)
        sys.exit(2)
    n = args[0]

    pr = gh(f"repos/{REPO}/pulls/{n}")
    print(f"Audit mécanique — PR #{pr['number']} « {pr['title'][:70]} »")
    print(f"  head: {pr['head']['sha'][:9]} | base: {pr['base']['ref']}\n")

    results = []

    def check(name, ok, detail=""):
        results.append((name, ok))
        print(f"  {'OK   ' if ok else 'ÉCHEC'} — {name}" + (f" : {detail}" if detail else ""))

    if merged_mode:
        check("PR mergée (mode audit a posteriori)", bool(pr.get("merged")), str(pr.get("merged_at")))
    else:
        check("PR ouverte, NON mergée (gate propriétaire)", pr["state"] == "open" and not pr.get("merged"))
    check("base = main", pr["base"]["ref"] == "main")

    files = gh(f"repos/{REPO}/pulls/{n}/files")
    names = [f["filename"] for f in files]
    reserved_hits = [f for f in names if any(f == r.rstrip("/") or f.startswith(r) for r in RESERVED)]
    check("aucun fichier réservé", not reserved_hits, ", ".join(reserved_hits) or f"{len(names)} fichier(s)")
    deps = [f for f in names if f.endswith("Cargo.toml") or f == "Cargo.lock"]
    print(f"  INFO  — dépendances modifiées : {', '.join(deps) if deps else 'aucune'}")

    secrets = []
    for f in files:
        patch = f.get("patch") or ""
        for pat in SECRET_PATTERNS:
            if re.search(pat, patch):
                secrets.append(f"{f['filename']} ({pat[:20]}…)")
    check("aucun secret dans le diff", not secrets, ", ".join(secrets))

    sha = pr["head"]["sha"]
    runs = gh(f"repos/{REPO}/actions/runs?head_sha={sha}&per_page=10").get("workflow_runs", [])
    ok_run = next((r for r in runs if r["status"] == "completed" and r["conclusion"] == "success"), None)
    check("CI verte sur le head de la PR", ok_run is not None,
          ok_run["html_url"] if ok_run else f"{len(runs)} run(s), aucun succès")

    here = os.path.dirname(os.path.abspath(__file__))
    rc = subprocess.run([sys.executable, "-B", os.path.join(here, "verify_roadmap_citations.py")],
                        capture_output=True, text=True)
    check("citations roadmap (verify_roadmap_citations exit 0)", rc.returncode == 0)

    refs = sorted(set(re.findall(r"#(\d+)", pr.get("body") or "")))
    check("issue(s) référencée(s) dans la PR", bool(refs), ", ".join("#" + r for r in refs[:6]) or "aucune")

    fails = [name for name, ok in results if not ok]
    print()
    if fails:
        print(f"AUDIT MÉCANIQUE : FAIL — {len(fails)} échec(s)")
        sys.exit(1)
    print("AUDIT MÉCANIQUE : PASS")
    if deps:
        print("NOTE : Cargo.toml/Cargo.lock modifiés — justification exigée dans le rapport du chef.")
    sys.exit(0)


if __name__ == "__main__":
    main()
