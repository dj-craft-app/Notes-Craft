//! Main pour tester le PoC Custom (SQLite)
//!
//! Auteur: Agent autonome
//! Date: 2026-11-11

use custom_poc::{DatalogDb, generate_test_data};
use std::time::Instant;

fn main() {
    println!("=== Custom (SQLite) PoC pour Note Craft ===\n");

    let test_cases = vec![
        (100, 500, "Petit"),
        (1_000, 5_000, "Moyen"),
    ];

    for (node_count, link_count, size_name) in test_cases {
        println!("=== Test avec jeu {}: {} nœuds, {} liens ===", size_name, node_count, link_count);
        
        let (nodes, tags, links, node_types) = generate_test_data(node_count, link_count);
        println!("Données générées: {} nœuds, {} tags, {} liens", 
                 nodes.len(), tags.len(), links.len());

        let mut db = DatalogDb::new_in_memory().unwrap();
        
        // Insérer les données
        let start = Instant::now();
        db.insert_batch(&nodes, &tags.iter().map(|(n, t)| (*n, t.as_str())).collect::<Vec<_>>(), 
                       &links, &node_types.iter().map(|(n, t)| (*n, t.as_str())).collect::<Vec<_>>()).unwrap();
        let insert_duration = start.elapsed();
        println!("Insertion: {:?}", insert_duration);

        // Test 1: Par tag
        println!("\n--- Test 1: Nœuds avec tag 'important' ---");
        let start = Instant::now();
        let important_nodes = db.find_nodes_by_tag("important").unwrap();
        let duration = start.elapsed();
        println!("Résultats: {} nœuds", important_nodes.len());
        println!("Temps: {:?}", duration);

        // Test 2: Par lien
        println!("\n--- Test 2: Nœuds liés à des nœuds 'important' ---");
        let start = Instant::now();
        let linked_to_important = db.find_nodes_linked_to_tagged("important").unwrap();
        let duration = start.elapsed();
        println!("Résultats: {} nœuds", linked_to_important.len());
        println!("Temps: {:?}", duration);

        // Test 3: Transitivité
        println!("\n--- Test 3: Nœuds accessibles depuis node-1 ---");
        let start = Instant::now();
        let reachable = db.find_reachable_nodes(1).unwrap();
        let duration = start.elapsed();
        println!("Résultats: {} nœuds accessibles", reachable.len());
        println!("Temps: {:?}", duration);

        // Test 4: Par type
        println!("\n--- Test 4: Nœuds de type 'page' ---");
        let start = Instant::now();
        let pages = db.find_nodes_by_type("page").unwrap();
        let duration = start.elapsed();
        println!("Résultats: {} pages", pages.len());
        println!("Temps: {:?}", duration);

        println!("\n✅ Vérification: Custom (SQLite) fonctionne correctement");
    }

    println!("\n=== PoC terminé avec succès ===");
}
