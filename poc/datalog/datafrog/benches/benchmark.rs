//! Benchmarks pour datafrog avec criterion.rs
//!
//! Ces benchmarks mesurent **STRICTEMENT** l'exécution des requêtes
//! sur des données déjà chargées, selon ADR-002-criteres-decision.md Section 2.
//!
//! IMPORTANT: Le setup (génération des données) est EXCLU de la mesure.
//! Seule l'exécution de la fonction de requête est chronométrée.
//!
//! Méthodologie:
//! - Setup: Générer les données (Vec<...>) - EXCLU
//! - Benchmark: Exécuter la fonction de requête sur ces données - INCLUS
//!
//! Note: Pour datafrog, les fonctions de requête sont des itérations simples
//! sur les vecteurs de données, donc elles ne créent pas de runtime.
//!
//! Exécution: cargo bench --bench benchmark
//!
//! Auteur: Agent autonome
//! Date: 2026-11-11 (corrigé pour symétrie de mesure)

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

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

/// Génère des données de test de manière déterministe
/// CETTE FONCTION EST APPÉLÉE EN DEHORS DE LA MESURE
fn generate_test_data(node_count: usize, link_count: usize) -> (Vec<NodeId>, Vec<(NodeId, Tag)>, Vec<Link>, Vec<(NodeId, NodeType)>) {
    let nodes: Vec<NodeId> = (1..=node_count).map(|i| NodeId(i as u64)).collect();
    
    // Générer des tags (chaque nœud a 1-3 tags de manière déterministe)
    let mut tags = Vec::new();
    let tag_names = ["important", "rust", "datalog", "note-craft", "test"];
    for (idx, node) in nodes.iter().enumerate() {
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
        let to_idx = ((i + 13) * 7) % nodes.len();
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
/// CECI SEUL EST MESURÉ - les données sont déjà générées
fn find_nodes_by_tag(tags: &[(NodeId, Tag)], target_tag: &Tag) -> usize {
    tags.iter()
        .filter(|(_, tag)| tag == target_tag)
        .count()
}

/// Trouve tous les nœuds liés à des nœuds avec un tag spécifique
/// CECI SEUL EST MESURÉ - les données sont déjà générées
fn find_nodes_linked_to_tagged(links: &[Link], tags: &[(NodeId, Tag)], target_tag: &Tag) -> usize {
    use std::collections::HashSet;
    
    let tagged_nodes: HashSet<NodeId> = tags.iter()
        .filter(|(_, tag)| tag == target_tag)
        .map(|(node, _)| node.clone())
        .collect();
    
    links.iter()
        .filter(|link| tagged_nodes.contains(&link.from) || tagged_nodes.contains(&link.to))
        .flat_map(|link| vec![link.from.clone(), link.to.clone()])
        .count()
}

/// Trouve tous les nœuds de type spécifique
/// CECI SEUL EST MESURÉ - les données sont déjà générées
fn find_nodes_by_type(node_types: &[(NodeId, NodeType)], target_type: &NodeType) -> usize {
    node_types.iter()
        .filter(|(_, node_type)| node_type == target_type)
        .count()
}

/// Trouve tous les nœuds accessibles depuis un nœud source en utilisant datafrog
/// CECI SEUL EST MESURÉ - les données sont déjà générées
/// Note: datafrog utilise un algorithme itératif avec Iteration
fn find_reachable_with_datafrog(links: &[Link], source: NodeId) -> usize {
    use datafrog::{Iteration, Relation};
    
    // Convertir les liens en tuples (from, to) - EXCLU de la mesure
    let edges: Vec<(NodeId, NodeId)> = links.iter()
        .map(|link| (link.from.clone(), link.to.clone()))
        .collect();
    
    // Créer le contexte d'itération
    let mut iteration = Iteration::new();
    
    // Déclarer les variables
    let nodes_var = iteration.variable::<(NodeId, ())>("nodes");
    let edges_var = iteration.variable::<(NodeId, NodeId)>("edges");
    
    // Insérer les données initiales - EXCLU de la mesure
    nodes_var.insert(Relation::from_iter(vec![(source, ())]));
    edges_var.insert(Relation::from_iter(edges));
    
    // Appliquer les règles itérativement pour calculer la transitivité
    // CECI SEUL EST MESURÉ
    while iteration.changed() {
        nodes_var.from_join(&nodes_var, &edges_var, |&_node, &(), &to| (to, ()));
    }
    
    // Extraire les résultats
    nodes_var.complete()
        .into_iter()
        .count()
}

/// Benchmark pour la requête "par tag" (query 1)
/// MESURE: find_nodes_by_tag() UNIQUEMENT
fn bench_query_by_tag(c: &mut Criterion) {
    let mut group = c.benchmark_group("query_by_tag");
    
    for (node_count, link_count, size_name) in &[
        (100, 500, "small"),
        (1_000, 5_000, "medium"),
        (10_000, 50_000, "large"),
    ] {
        // SETUP: Générer les données - EXCLU de la mesure
        let (nodes, tags, links, node_types) = generate_test_data(*node_count, *link_count);
        let target_tag = Tag("important".to_string());
        
        group.throughput(Throughput::Elements(*node_count as u64));
        
        group.bench_with_input(
            BenchmarkId::new("datafrog", size_name),
            &tags,
            |b, tags| {
                // MESURE: Exécution de la requête UNIQUEMENT
                b.iter(|| {
                    let count = find_nodes_by_tag(tags, &target_tag);
                    black_box(count);
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark pour la requête "par lien" (query 2)
/// MESURE: find_nodes_linked_to_tagged() UNIQUEMENT
fn bench_query_by_link(c: &mut Criterion) {
    let mut group = c.benchmark_group("query_by_link");
    
    for (node_count, link_count, size_name) in &[
        (100, 500, "small"),
        (1_000, 5_000, "medium"),
        (10_000, 50_000, "large"),
    ] {
        let (nodes, tags, links, node_types) = generate_test_data(*node_count, *link_count);
        let target_tag = Tag("important".to_string());
        
        group.throughput(Throughput::Elements(*link_count as u64));
        
        group.bench_with_input(
            BenchmarkId::new("datafrog", size_name),
            &(links, tags),
            |b, (links, tags)| {
                b.iter(|| {
                    let count = find_nodes_linked_to_tagged(links, tags, &target_tag);
                    black_box(count);
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark pour la requête de transitivité (query 3)
/// MESURE: find_reachable_with_datafrog() UNIQUEMENT
fn bench_query_transitive(c: &mut Criterion) {
    let mut group = c.benchmark_group("query_transitive");
    
    for (node_count, link_count, size_name) in &[
        (100, 500, "small"),
        (1_000, 5_000, "medium"),
        (10_000, 50_000, "large"),
    ] {
        let (nodes, tags, links, node_types) = generate_test_data(*node_count, *link_count);
        let source = NodeId(1);
        
        group.throughput(Throughput::Elements(*node_count as u64));
        
        // SETUP: Créer edges et edges_var - EXCLU de la mesure
        let edges: Vec<(NodeId, NodeId)> = links.iter()
            .map(|link| (link.from.clone(), link.to.clone()))
            .collect();
        
        group.bench_with_input(
            BenchmarkId::new("datafrog", size_name),
            &edges,
            |b, edges| {
                b.iter(|| {
                    let count = find_reachable_with_datafrog_from_edges(edges, source.clone());
                    black_box(count);
                });
            },
        );
    }
    
    group.finish();
}

/// Version de find_reachable qui prend edges directement pour le benchmark
fn find_reachable_with_datafrog_from_edges(edges: &[(NodeId, NodeId)], source: NodeId) -> usize {
    use datafrog::{Iteration, Relation};
    
    let mut iteration = Iteration::new();
    let nodes_var = iteration.variable::<(NodeId, ())>("nodes");
    let edges_var = iteration.variable::<(NodeId, NodeId)>("edges");
    
    // Insérer les données initiales - partie du setup de l'itération
    nodes_var.insert(Relation::from_iter(vec![(source, ())]));
    edges_var.insert(Relation::from_iter(edges.to_vec()));
    
    while iteration.changed() {
        nodes_var.from_join(&nodes_var, &edges_var, |&_node, &(), &to| (to, ()));
    }
    
    nodes_var.complete()
        .into_iter()
        .count()
}

/// Benchmark pour la requête "par type" (query 4)
/// MESURE: find_nodes_by_type() UNIQUEMENT
fn bench_query_by_type(c: &mut Criterion) {
    let mut group = c.benchmark_group("query_by_type");
    
    for (node_count, link_count, size_name) in &[
        (100, 500, "small"),
        (1_000, 5_000, "medium"),
        (10_000, 50_000, "large"),
    ] {
        let (nodes, tags, links, node_types) = generate_test_data(*node_count, *link_count);
        let target_type = NodeType("page".to_string());
        
        group.throughput(Throughput::Elements(*node_count as u64));
        
        group.bench_with_input(
            BenchmarkId::new("datafrog", size_name),
            &node_types,
            |b, node_types| {
                b.iter(|| {
                    let count = find_nodes_by_type(node_types, &target_type);
                    black_box(count);
                });
            },
        );
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_query_by_tag,
    bench_query_by_link,
    bench_query_transitive,
    bench_query_by_type,
);

criterion_main!(benches);
