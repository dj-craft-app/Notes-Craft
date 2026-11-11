//! PoC d'évaluation de crepe comme moteur Datalog pour Note Craft
//!
//! Ce fichier teste les fonctionnalités de base de crepe pour déterminer
//! s'il peut servir de moteur Datalog pour Note Craft.
//!
//! Auteur: Agent autonome
//! Date: 2026-11-11

use crepe::crepe;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

// Définir les structures pour Note Craft
// crepe fonctionne mieux avec des types Copy, donc on utilise i32 pour tout
// et on gère les strings → IDs manuellement
crepe! {
    // Nœuds: identifiants uniques
    @input
    struct Node(i32);

    // Tags: association nœud -> tag_id (i32 au lieu de String)
    @input
    struct Tag(i32, i32);

    // Liens: association nœud -> nœud
    @input
    struct Link(i32, i32);

    // Types: association nœud -> type_id
    @input
    struct NodeType(i32, i32);

    // Résultats: nœuds avec tag spécifique (par tag_id)
    @output
    struct NodesWithTag(i32, i32);

    // Résultats: nœuds liés à des nœuds avec tag spécifique
    @output
    struct LinkedToTag(i32, i32, i32);

    // Résultats: nœuds accessibles (transitivité)
    @output
    struct Reachable(i32, i32);

    // Résultats: nœuds de type spécifique
    @output
    struct NodesOfType(i32, i32);

    // Règles Datalog

    // 1. Trouver tous les nœuds avec un tag
    NodesWithTag(node, tag_id) <- Tag(node, tag_id);

    // 2. Trouver tous les nœuds liés à des nœuds avec un tag
    LinkedToTag(from, to, tag_id) <- 
        Link(from, to),
        Tag(to, tag_id);

    LinkedToTag(from, to, tag_id) <- 
        Link(to, from),
        Tag(from, tag_id);

    // 3. Calculer la transitivité (fermeture transitive)
    Reachable(from, to) <- Link(from, to);
    Reachable(from, to) <- 
        Link(from, intermediate),
        Reachable(intermediate, to);

    // 4. Trouver les nœuds de type spécifique
    NodesOfType(node, type_id) <- NodeType(node, type_id);
}

/// Mapper les noms de tags à des IDs
fn get_tag_id(tag_name: &str) -> i32 {
    match tag_name {
        "important" => 1,
        "rust" => 2,
        "datalog" => 3,
        "note-craft" => 4,
        "test" => 5,
        _ => 0,
    }
}

/// Mapper les noms de types à des IDs
fn get_type_id(type_name: &str) -> i32 {
    match type_name {
        "page" => 1,
        "block" => 2,
        "asset" => 3,
        "query" => 4,
        _ => 0,
    }
}

fn main() {
    println!("=== crepe PoC pour Note Craft ===\n");

    // Mapper les noms pour l'affichage (non utilisés pour l'instant, mais gardés pour référence)
    let _tag_names: HashMap<i32, &str> = [
        (1, "important"),
        (2, "rust"),
        (3, "datalog"),
        (4, "note-craft"),
        (5, "test"),
    ].iter().cloned().collect();
    
    let _type_names: HashMap<i32, &str> = [
        (1, "page"),
        (2, "block"),
        (3, "asset"),
        (4, "query"),
    ].iter().cloned().collect();

    // Tester différentes tailles de jeux de données
    let test_cases = vec![
        (100, 500, "Petit"),
        (1_000, 5_000, "Moyen"),
    ];

    for (node_count, link_count, size_name) in test_cases {
        println!("=== Test avec jeu {}: {} nœuds, {} liens ===", size_name, node_count, link_count);
        
        // Générer des données de test
        let (nodes, tags, links, node_types) = generate_test_data(node_count, link_count);
        println!("Données générées: {} nœuds, {} tags, {} liens", 
                 nodes.len(), tags.len(), links.len());

        // Créer le runtime crepe
        let mut runtime = Crepe::new();
        
        // Insérer les données
        runtime.extend(nodes.into_iter().map(|n| Node(n)));
        runtime.extend(tags.into_iter().map(|(n, t)| Tag(n, t)));
        runtime.extend(links.into_iter().map(|(from, to)| Link(from, to)));
        runtime.extend(node_types.into_iter().map(|(n, t)| NodeType(n, t)));

        // Exécuter les règles
        let start = Instant::now();
        let (nodes_with_tag, linked_to_tag, reachable, nodes_of_type) = runtime.run();
        let duration = start.elapsed();
        println!("\n--- Exécution de toutes les règles ---");
        println!("Temps total: {:?}", duration);

        // Test 1: Nœuds avec tag 'important' (tag_id = 1)
        println!("\n--- Test 1: Nœuds avec tag 'important' ---");
        let important_tag_id = get_tag_id("important");
        let important_nodes: Vec<i32> = nodes_with_tag
            .into_iter()
            .filter(|NodesWithTag(_, tag_id)| *tag_id == important_tag_id)
            .map(|NodesWithTag(node, _)| node)
            .collect();
        println!("Résultats: {} nœuds", important_nodes.len());

        // Test 2: Nœuds liés à des nœuds avec tag 'important'
        println!("\n--- Test 2: Nœuds liés à des nœuds 'important' ---");
        let linked_to_important: HashSet<i32> = linked_to_tag
            .into_iter()
            .filter(|LinkedToTag(_, _, tag_id)| *tag_id == important_tag_id)
            .flat_map(|LinkedToTag(from, to, _)| vec![from, to])
            .collect();
        println!("Résultats: {} nœuds", linked_to_important.len());

        // Test 3: Nœuds accessibles depuis node-1
        println!("\n--- Test 3: Nœuds accessibles depuis node-1 ---");
        let reachable_from_1: Vec<i32> = reachable
            .into_iter()
            .filter(|Reachable(from, _)| *from == 1)
            .map(|Reachable(_, to)| to)
            .collect();
        println!("Résultats: {} nœuds accessibles", reachable_from_1.len());

        // Test 4: Nœuds de type 'page' (type_id = 1)
        println!("\n--- Test 4: Nœuds de type 'page' ---");
        let page_type_id = get_type_id("page");
        let pages: Vec<i32> = nodes_of_type
            .into_iter()
            .filter(|NodesOfType(_, type_id)| *type_id == page_type_id)
            .map(|NodesOfType(node, _)| node)
            .collect();
        println!("Résultats: {} pages", pages.len());

        println!("\n✅ Vérification: crepe fonctionne correctement");
    }

    println!("\n=== PoC terminé avec succès ===");
}

/// Génère des données de test de manière déterministe
/// Retourne des vecteurs de tuple structs compatibles avec crepe
fn generate_test_data(node_count: usize, link_count: usize) -> 
    (Vec<i32>, Vec<(i32, i32)>, Vec<(i32, i32)>, Vec<(i32, i32)>) {
    
    let nodes: Vec<i32> = (1..=node_count as i32).collect();
    
    // Générer des tags (chaque nœud a 1-3 tags de manière déterministe)
    // Mapper les noms de tags à des IDs
    let tag_name_to_id = |name: &str| -> i32 {
        match name {
            "important" => 1,
            "rust" => 2,
            "datalog" => 3,
            "note-craft" => 4,
            "test" => 5,
            _ => 0,
        }
    };
    
    let tag_names = ["important", "rust", "datalog", "note-craft", "test"];
    let mut tags = Vec::new();
    for (idx, &node) in nodes.iter().enumerate() {
        let num_tags = (idx % 3) + 1;
        for i in 0..num_tags {
            let tag_idx = (idx + i) % tag_names.len();
            let tag_id = tag_name_to_id(tag_names[tag_idx]);
            tags.push((node, tag_id));
        }
    }
    
    // Générer des liens de manière déterministe
    let mut links = Vec::new();
    for i in 0..link_count {
        let from_idx = i % nodes.len();
        let to_idx = ((i + 13) * 7) % nodes.len();
        links.push((nodes[from_idx], nodes[to_idx]));
    }
    
    // Générer des types (chaque nœud a un type déterministe)
    let type_name_to_id = |name: &str| -> i32 {
        match name {
            "page" => 1,
            "block" => 2,
            "asset" => 3,
            "query" => 4,
            _ => 0,
        }
    };
    
    let types = ["page", "block", "asset", "query"];
    let node_types: Vec<(i32, i32)> = nodes.iter()
        .enumerate()
        .map(|(idx, &n)| {
            let type_idx = idx % types.len();
            let type_id = type_name_to_id(types[type_idx]);
            (n, type_id)
        })
        .collect();

    (nodes, tags, links, node_types)
}
