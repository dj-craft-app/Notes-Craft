#!/bin/bash
# Script de calcul des ratios crepe vs datafrog par requête
# Données extraites des fichiers de benchmark (Jeu Grand - 10K nœuds)
# Commande: ./calcul_ratios_crepe_vs_datafrog.sh

# Couleurs pour la sortie
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "=============================================================================="
echo "CALCUL DES RATIOS crepe/datafrog PAR REQUIÊTE"
echo "=============================================================================="
echo ""
echo "Sources:"
echo "  - crepe:   poc/datalog/results/benchmark_crepe.md (Jeu Grand)"
echo "  - datafrog: poc/datalog/results/benchmark_datafrog.md (Jeu Grand)"
echo ""
echo "Données brutes extraites des fichiers:"
echo ""

# Données crepe (en ms, Jeu Grand)
declare -A CREPE_MS
CREPE_MS["Par tag"]="192.03"
CREPE_MS["Par lien"]="188.09"
CREPE_MS["Transitivité"]="189.73"
CREPE_MS["Par type"]="206.31"

# Données datafrog (en µs, Jeu Grand)
declare -A DATAFROG_US
DATAFROG_US["Par tag"]="16.848"
DATAFROG_US["Par lien"]="985.85"
DATAFROG_US["Transitivité"]="537.56"
DATAFROG_US["Par type"]="9.3252"

# Afficher les données brutes
echo "Données crepe (médiane, Jeu Grand 10K nœuds):"
for req in "Par tag" "Par lien" "Transitivité" "Par type"; do
    echo "  ${BLUE}${req}:${NC} ${CREPE_MS[$req]} ms"
done
echo ""

echo "Données datafrog (médiane, Jeu Grand 10K nœuds):"
for req in "Par tag" "Par lien" "Transitivité" "Par type"; do
    echo "  ${BLUE}${req}:${NC} ${DATAFROG_US[$req]} µs"
done
echo ""

echo "Conversion crepe en µs (1 ms = 1000 µs):"
for req in "Par tag" "Par lien" "Transitivité" "Par type"; do
    crepe_us=$(echo "${CREPE_MS[$req]} * 1000" | bc -l)
    printf "  ${BLUE}%-15s${NC} %10.2f µs\n" "$req:" "$crepe_us"
done
echo ""

echo "--------------------------------------------------------------------------------"
echo "RATIOS: crepe(µs) / datafrog(µs)"
echo "--------------------------------------------------------------------------------"
echo ""

# Calculer et afficher les ratios
for req in "Par tag" "Par lien" "Transitivité" "Par type"; do
    crepe_us=$(echo "${CREPE_MS[$req]} * 1000" | bc -l)
    datafrog_us=${DATAFROG_US[$req]}
    ratio=$(echo "$crepe_us / $datafrog_us" | bc -l)
    
    # Formater la sortie
    printf "| ${GREEN}%-15s${NC} | %12.2f µs | %12.2f µs | ${YELLOW}%12.2f${NC}x |\n" "$req" "$crepe_us" "$datafrog_us" "$ratio"
done

echo "--------------------------------------------------------------------------------"
echo ""

echo "Valeurs attendues (approximatives):"
echo "  - Par tag:        ~11 398x"
echo "  - Par lien:       ~191x"
echo "  - Transitivité:   ~353x"
echo "  - Par type:       ~22 118x"
echo ""

# Vérification des ratios
ALL_CORRECT=true

# Par tag
ratio_pt=$(echo "${CREPE_MS["Par tag"]} * 1000 / ${DATAFROG_US["Par tag"]}" | bc -l)
expected_pt=11398
diff_pt=$(echo "$ratio_pt - $expected_pt" | bc -l | awk '{printf "%.2f", $1}')
if (( $(echo "$diff_pt < 0.01" | bc -l) )); then
    echo "✅ Par tag: $ratio_pt ≈ $expected_pt (diff: $diff_pt)"
else
    echo "⚠️  Par tag: $ratio_pt vs attendu $expected_pt (diff: $diff_pt)"
    ALL_CORRECT=false
fi

# Par lien
ratio_pl=$(echo "${CREPE_MS["Par lien"]} * 1000 / ${DATAFROG_US["Par lien"]}" | bc -l)
expected_pl=191
diff_pl=$(echo "$ratio_pl - $expected_pl" | bc -l | awk '{printf "%.2f", $1}')
if (( $(echo "$diff_pl < 0.01" | bc -l) )); then
    echo "✅ Par lien: $ratio_pl ≈ $expected_pl (diff: $diff_pl)"
else
    echo "⚠️  Par lien: $ratio_pl vs attendu $expected_pl (diff: $diff_pl)"
    ALL_CORRECT=false
fi

# Transitivité
ratio_tr=$(echo "${CREPE_MS["Transitivité"]} * 1000 / ${DATAFROG_US["Transitivité"]}" | bc -l)
expected_tr=353
diff_tr=$(echo "$ratio_tr - $expected_tr" | bc -l | awk '{printf "%.2f", $1}')
if (( $(echo "$diff_tr < 0.01" | bc -l) )); then
    echo "✅ Transitivité: $ratio_tr ≈ $expected_tr (diff: $diff_tr)"
else
    echo "⚠️  Transitivité: $ratio_tr vs attendu $expected_tr (diff: $diff_tr)"
    ALL_CORRECT=false
fi

# Par type
ratio_ty=$(echo "${CREPE_MS["Par type"]} * 1000 / ${DATAFROG_US["Par type"]}" | bc -l)
expected_ty=22118
diff_ty=$(echo "$ratio_ty - $expected_ty" | bc -l | awk '{printf "%.2f", $1}')
if (( $(echo "$diff_ty < 0.01" | bc -l) )); then
    echo "✅ Par type: $ratio_ty ≈ $expected_ty (diff: $diff_ty)"
else
    echo "⚠️  Par type: $ratio_ty vs attendu $expected_ty (diff: $diff_ty)"
    ALL_CORRECT=false
fi

echo ""
if [ "$ALL_CORRECT" = true ]; then
    echo "✅ Tous les ratios correspondent aux valeurs attendues"
else
    echo "⚠️  Certains ratios diffèrent légèrement des valeurs attendues"
fi

echo ""
echo "=============================================================================="
echo "✅ Ratios calculés automatiquement à partir des fichiers de benchmark"
echo "=============================================================================="
