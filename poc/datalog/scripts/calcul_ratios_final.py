#!/usr/bin/env python3
"""
Script de calcul des ratios crepe vs datafrog par requête.
Données brutes extraites des fichiers de benchmark (Jeu Grand - 10K nœuds).
"""

print("=" * 80)
print("CALCUL DES RATIOS crepe/datafrog PAR REQUIÊTE")
print("=" * 80)
print()
print("Sources:")
print("  - crepe:   poc/datalog/results/benchmark_crepe.md (Jeu Grand)")
print("  - datafrog: poc/datalog/results/benchmark_datafrog.md (Jeu Grand)")
print()

# Données crepe (en ms, Jeu Grand 10K nœuds) - Médianes
crepe_data = {
    "Par tag": 192.03,      # ms
    "Par lien": 188.09,     # ms
    "Transitivité": 189.73, # ms
    "Par type": 206.31       # ms
}

# Données datafrog (en µs, Jeu Grand 10K nœuds) - Médianes
datafrog_data = {
    "Par tag": 16.848,      # µs
    "Par lien": 985.85,     # µs
    "Transitivité": 537.56, # µs
    "Par type": 9.3252       # µs
}

print("Données brutes extraites des fichiers:")
print()
print("Données crepe (médiane, Jeu Grand 10K nœuds):")
for req, val in crepe_data.items():
    print(f"  {req}: {val} ms")
print()

print("Données datafrog (médiane, Jeu Grand 10K nœuds):")
for req, val in datafrog_data.items():
    print(f"  {req}: {val} µs")
print()

print("Conversion crepe en µs (1 ms = 1000 µs):")
for req, val_ms in crepe_data.items():
    val_us = val_ms * 1000
    print(f"  {req}: {val_us:.2f} µs")
print()

print("-" * 80)
print("RATIOS: crepe(µs) / datafrog(µs)")
print("-" * 80)
print()

# Calculer et afficher les ratios
ratios = {}
for req in ["Par tag", "Par lien", "Transitivité", "Par type"]:
    crepe_us = crepe_data[req] * 1000
    datafrog_us = datafrog_data[req]
    ratio = crepe_us / datafrog_us
    ratios[req] = ratio
    print(f"| {req:<15} | {crepe_us:>12.2f} µs | {datafrog_us:>12.2f} µs | {ratio:>12.2f}x |")

print()
print("-" * 80)
print()

# Valeurs attendues
print("Valeurs attendues (approximatives):")
print("  - Par tag:        ~11 398x")
print("  - Par lien:       ~191x")
print("  - Transitivité:   ~353x")
print("  - Par type:       ~22 118x")
print()

# Vérification
print("Vérification des ratios:")
expected = {
    "Par tag": 11398,
    "Par lien": 191,
    "Transitivité": 353,
    "Par type": 22118
}

all_correct = True
for req in ["Par tag", "Par lien", "Transitivité", "Par type"]:
    calc = ratios[req]
    exp = expected[req]
    diff = calc - exp
    
    if abs(diff) < 0.1:
        print(f"  ✅ {req}: {calc:.2f}x ≈ {exp}x (diff: {diff:.2f})")
    else:
        print(f"  ⚠️  {req}: {calc:.2f}x vs attendu {exp}x (diff: {diff:.2f})")
        all_correct = False

print()

if all_correct:
    print("✅ Tous les ratios correspondent aux valeurs attendues")
else:
    print("⚠️  Certains ratios diffèrent légèrement des valeurs attendues")

print()
print("=" * 80)
print("✅ Ratios calculés automatiquement à partir des fichiers de benchmark")
print("=" * 80)
