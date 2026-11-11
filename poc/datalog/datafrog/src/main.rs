//! PoC d'évaluation de datafrog comme moteur Datalog pour Note Craft
//!
//! Ce fichier teste les fonctionnalités de base de datafrog pour déterminer
//! s'il peut servir de moteur Datalog pour Note Craft.
//!
//! Auteur: Agent autonome
//! Date: 2026-11-11

use datafrog::Iteration;
use std::collections::{HashSet, VecDeque};
use std::time::Instant;

/// Représente un identifiant de nœud
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct NodeId(u64);

/// Représente un tag
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Tag(String);

/// Représente un type de nœud
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct NodeType(String);

/// Représente un lien entre nœuds
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Link {
    from: NodeId,
    to: NodeId,
}

fn main() {
    println!("=== datafrog PoC pour Note Craft ===\n");

    // Générer des données de test (déterministe pour reproductibilité)
    let (nodes, tags, links, node_types) = generate_test_data(100, 500);
    println!("Données générées: {} nœuds, {} tags, {} liens", 
             nodes.len(), tags.len(), links.len());

    // Test 1: Requête simple - Trouver tous les nœuds avec un tag spécifique
    println!("\n--- Test 1: Nœuds avec tag 'important' ---");
    let start = Instant::now();
    let important_nodes = find_nodes_by_tag(&tags, Tag("important".to_string()));
    let duration = start.elapsed();
    println!("Résultats: {} nœuds", important_nodes.len());
    println!("Temps: {:?}", duration);

    // Test 2: Requête avec jointure - Trouver tous les nœuds liés à des nœuds tagués "important"
    println!("\n--- Test 2: Nœuds liés à des nœuds 'important' ---");
    let start = Instant::now();
    let linked_to_important = find_nodes_linked_to_tagged(&links, &tags, Tag("important".to_string()));
    let duration = start.elapsed();
    println!("Résultats: {} nœuds", linked_to_important.len());
    println!("Temps: {:?}", duration);

    // Test 3: Requête récursive - Trouver tous les nœuds accessibles depuis node-1
    println!("\n--- Test 3: Nœuds accessibles depuis node-1 (algorithme naif) ---");
    let start = Instant::now();
    let reachable = find_reachable_nodes(&links, NodeId(1));
    let duration = start.elapsed();
    println!("Résultats: {} nœuds accessibles", reachable.len());
    println!("Temps: {:?}", duration);

    // Test 4: Requête par type - Trouver tous les nœuds de type "page"
    println!("\n--- Test 4: Nœuds de type 'page' ---");
    let start = Instant::now();
    let pages = find_nodes_by_type(&node_types, NodeType("page".to_string()));
    let duration = start.elapsed();
    println!("Résultats: {} pages", pages.len());
    println!("Temps: {:?}", duration);

    // Test 5: Avec datafrog - Calcul de la transitivité
    println!("\n--- Test 5: Calcul de la transitivité avec datafrog ---");
    let start = Instant::now();
    let reachable_datafrog = find_reachable_with_datafrog(&links, NodeId(1));
    let duration = start.elapsed();
    println!("Résultats: {} nœuds accessibles", reachable_datafrog.len());
    println!("Temps: {:?}", duration);

    println!("\n=== PoC terminé avec succès ===");
}

/// Génère des données de test de manière déterministe
/// 
/// * `node_count`: Nombre de nœuds à générer
/// * `link_count`: Nombre de liens à générer
fn generate_test_data(node_count: usize, link_count: usize) -> (Vec<NodeId>, Vec<(NodeId, Tag)>, Vec<Link>, Vec<(NodeId, NodeType)>) {
    let nodes: Vec<NodeId> = (1..=node_count).map(|i| NodeId(i as u64)).collect();
    
    // Générer des tags (chaque nœud a 1-3 tags de manière déterministe)
    let mut tags = Vec::new();
    let tag_names = ["important", "rust", "datalog", "note-craft", "test"];
    for (idx, node) in nodes.iter().enumerate() {
        // Générer 1-3 tags par nœud (déterministe basé sur l'index)
        let num_tags = (idx % 3) + 1;
        for i in 0..num_tags {
            let tag_idx = (idx + i) % tag_names.len();
            tags.push((node.clone(), Tag(tag_names[tag_idx].to_string())));
        }
    }
    
    // Générer des liens de manière déterministe
    let mut links = Vec::new();
    for i in 0..link_count {
        let from_idx = i % nodes.len();
        let to_idx = ((i + 13) * 7) % nodes.len(); // Simple déterminisme
        links.push(Link {
            from: nodes[from_idx].clone(),
            to: nodes[to_idx].clone(),
        });
    }
    
    // Générer des types (chaque nœud a un type déterministe)
    let node_types: Vec<(NodeId, NodeType)> = nodes.iter()
        .enumerate()
        .map(|(idx, n)| {
            let types = ["page", "block", "asset", "query"];
            let type_idx = idx % types.len();
            (n.clone(), NodeType(types[type_idx].to_string()))
        })
        .collect();

    (nodes, tags, links, node_types)
}

/// Trouve tous les nœuds avec un tag spécifique
/// 
/// Implementation naive sans datafrog pour comparaison
fn find_nodes_by_tag(tags: &[(NodeId, Tag)], target_tag: Tag) -> Vec<NodeId> {
    tags.iter()
        .filter(|(_, tag)| tag == &target_tag)
        .map(|(node, _)| node.clone())
        .collect()
}

/// Trouve tous les nœuds liés à des nœuds avec un tag spécifique
fn find_nodes_linked_to_tagged(links: &[Link], tags: &[(NodeId, Tag)], target_tag: Tag) -> Vec<NodeId> {
    let tagged_nodes: HashSet<NodeId> = tags.iter()
        .filter(|(_, tag)| tag == &target_tag)
        .map(|(node, _)| node.clone())
        .collect();
    
    links.iter()
        .filter(|link| tagged_nodes.contains(&link.from) || tagged_nodes.contains(&link.to))
        .flat_map(|link| vec![link.from.clone(), link.to.clone()])
        .collect()
}

/// Trouve tous les nœuds accessibles depuis un nœud source (algorithme naif)
fn find_reachable_nodes(links: &[Link], source: NodeId) -> Vec<NodeId> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    
    visited.insert(source.clone());
    queue.push_back(source);
    
    while let Some(current) = queue.pop_front() {
        for link in links {
            if link.from == current && !visited.contains(&link.to) {
                visited.insert(link.to.clone());
                queue.push_back(link.to.clone());
            }
        }
    }
    
    visited.into_iter().collect()
}

/// Trouve tous les nœuds de type spécifique
fn find_nodes_by_type(node_types: &[(NodeId, NodeType)], target_type: NodeType) -> Vec<NodeId> {
    node_types.iter()
        .filter(|(_, node_type)| node_type == &target_type)
        .map(|(node, _)| node.clone())
        .collect()
}

/// Trouve tous les nœuds accessibles depuis un nœud source en utilisant datafrog
fn find_reachable_with_datafrog(links: &[Link], source: NodeId) -> Vec<NodeId> {
    use datafrog::Relation;
    
    // Convertir les liens en tuples (from, to)
    let edges: Vec<(NodeId, NodeId)> = links.iter()
        .map(|link| (link.from.clone(), link.to.clone()))
        .collect();
    
    // Créer le contexte d'itération
    let mut iteration = Iteration::new();
    
    // Déclarer les variables
    // nodes_var: contient les nœuds accessibles, sous forme (node, ())
    // Nous utilisons () comme dummy pour avoir un tuple à 2 éléments
    let nodes_var = iteration.variable::<(NodeId, ())>("nodes");
    // edges_var: contient tous les edges, sous forme (from, to)
    let edges_var = iteration.variable::<(NodeId, NodeId)>("edges");
    
    // Insérer les données initiales
    nodes_var.insert(Relation::from_iter(vec![(source, ())]));
    edges_var.insert(Relation::from_iter(edges));
    
    // Appliquer les règles itérativement pour calculer la transitivité
    // Règle: nodes(b, ()) <- nodes(a, ()), edges(a, b)
    // La jointure match nodes_var.0 avec edges_var.0
    // La closure reçoit (&K, &V1, &V2) = (&NodeId, &(), &NodeId)
    // Et retourne (NodeId, ()) pour le nouveau nœud
    while iteration.changed() {
        nodes_var.from_join(&nodes_var, &edges_var, |&_node, &(), &to| (to, ()));
    }
    
    // Extraire les résultats
    nodes_var.complete()
        .into_iter()
        .map(|(node, _)| node)
        .cloned()
        .collect()
}
