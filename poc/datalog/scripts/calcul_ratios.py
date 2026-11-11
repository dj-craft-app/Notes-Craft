#!/usr/bin/env python3
"""
Script de calcul des ratios crepe vs datafrog par requête.

Lit les médianes depuis les fichiers de benchmark et calcule:
ratio = mediane_crepe / mediane_datafrog

Unités:
- crepe: millisecondes (ms)
- datafrog: microsecondes (µs)

Conversion: 1 ms = 1000 µs
"""

import re

def extraire_mediane_crepe(fichier):
    """Extrait les médianes crepe depuis benchmark_crepe.md (Jeu Grand)"""
    with open(fichier, 'r') as f:
        content = f.read()
    
    # Chercher le tableau du Jeu Grand
    # Format: | Par tag | **192.03 ms** | 190.49 ms | 193.96 ms |
    pattern = r'\| (Par tag|Par lien|Transitivité|Par type) \|\*\*(\d+\.\d+) ms\*\*'
    matches = re.findall(pattern, content)
    
    result = {}
    for requete, mediane_ms in matches:
        result[requete] = float(mediane_ms) * 1000  # Convertir en µs
    
    # Si pas trouvé avec **, essayer sans
    if not result:
        pattern = r'\| (Par tag|Par lien|Transitivité|Par type) \|(\d+\.\d+) ms\|'
        matches = re.findall(pattern, content)
        for requete, mediane_ms in matches:
            result[requete] = float(mediane_ms) * 1000
    
    return result

def extraire_mediane_datafrog(fichier):
    """Extrait les médianes datafrog depuis benchmark_datafrog.md (Jeu Grand)"""
    with open(fichier, 'r') as f:
        content = f.read()
    
    # Chercher le tableau du Jeu Grand
    # Format: | Par tag | **16.848 µs** | ...
    pattern = r'\| (Par tag|Par lien|Transitivité|Par type) \|\*\*(\d+\.\d+) µs\*\*'
    matches = re.findall(pattern, content)
    
    result = {}
    for requete, mediane_us in matches:
        result[requete] = float(mediane_us)
    
    # Si pas trouvé avec **, essayer sans
    if not result:
        # Chercher dans la section "Jeu Grand"
        pattern = r'\| (Par tag|Par lien|Transitivité|Par type) \| (\d+\.\d+) µs'
        matches = re.findall(pattern, content)
        for requete, mediane_us in matches:
            result[requete] = float(mediane_us)
    
    return result

def main():
    # Chemins des fichiers
    crepe_file = 'poc/datalog/results/benchmark_crepe.md'
    datafrog_file = 'poc/datalog/results/benchmark_datafrog.md'
    
    # Extraire les médianes
    crepe_medianes = extraire_mediane_crepe(crepe_file)
    datafrog_medianes = extraire_mediane_datafrog(datafrog_file)
    
    print("=" * 80)
    print("CALCUL DES RATIOS crepe/datafrog PAR REQUIÊTE")
    print("=" * 80)
    print()
    print("Sources:")
    print(f"  - crepe:   {crepe_file}")
    print(f"  - datafrog: {datafrog_file}")
    print()
    print("Métriques:")
    print("  - crepe:   médiane en millisecondes (ms) -> convertie en µs")
    print("  - datafrog: médiane en microsecondes (µs)")
    print("  - ratio:   mediane_crepe(µs) / mediane_datafrog(µs)")
    print()
    print("-" * 80)
    
    requetes = ['Par tag', 'Par lien', 'Transitivité', 'Par type']
    
    for requete in requetes:
        crepe_us = crepe_medianes.get(requete)
        datafrog_us = datafrog_medianes.get(requete)
        
        if crepe_us is not None and datafrog_us is not None:
            ratio = crepe_us / datafrog_us
            print(f"| {requete:<15} | {crepe_us:>10.2f} µs | {datafrog_us:>10.2f} µs | {ratio:>12.2f}x |")
        else:
            print(f"| {requete:<15} | {'N/A':>10} | {'N/A':>10} | {'N/A':>12} |")
    
    print("-" * 80)
    print()
    print("Valeurs attendues (approximatives):")
    print("  - Par tag:        ~11 398x")
    print("  - Par lien:       ~191x")
    print("  - Transitivité:   ~353x")
    print("  - Par type:       ~22 118x")
    print()
    print("✅ Ratios calculés automatiquement à partir des fichiers de benchmark")
    print("=" * 80)

if __name__ == '__main__':
    main()
