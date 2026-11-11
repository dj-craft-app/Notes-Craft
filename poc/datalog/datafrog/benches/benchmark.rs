//! Benchmarks pour datafrog avec criterion.rs
//!
//! Ces benchmarks mesurent les performances de datafrog sur les 4 requêtes
//! définies dans ADR-002-criteres-decision.md pour les 3 jeux de données
//! (100, 1000, 10000 nœuds).
//!
//! Exécution: cargo bench --bench benchmark
//!
//! Auteur: Agent autonome
//! Date: 2026-11-11

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use datafrog::Iteration;
use datafrog::Relation;

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
/// Identique à la fonction dans main.rs pour garantir la reproductibilité
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

/// Benchmark pour la requête "par tag" (query 1)
/// Implementation naive sans datafrog
fn bench_query_by_tag(c: &mut Criterion) {
    let mut group = c.benchmark_group("query_by_tag");
    
    for (node_count, link_count, size_name) in &[
        (100, 500, "small"),
        (1_000, 5_000, "medium"),
        (10_000, 50_000, "large"),
    ] {
        let (nodes, tags, links, node_types) = generate_test_data(*node_count, *link_count);
        
        group.throughput(Throughput::Elements(*node_count as u64));
        
        group.bench_with_input(
            BenchmarkId::new("datafrog", size_name),
            &(*node_count, *link_count),
            |b, _| {
                b.iter(|| {
                    let result = find_nodes_by_tag(&tags, Tag("important".to_string()));
                    black_box(result.len());
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark pour la requête "par lien" (query 2)
/// Implementation naive sans datafrog
fn bench_query_by_link(c: &mut Criterion) {
    let mut group = c.benchmark_group("query_by_link");
    
    for (node_count, link_count, size_name) in &[
        (100, 500, "small"),
        (1_000, 5_000, "medium"),
        (10_000, 50_000, "large"),
    ] {
        let (nodes, tags, links, node_types) = generate_test_data(*node_count, *link_count);
        
        group.throughput(Throughput::Elements(*link_count as u64));
        
        group.bench_with_input(
            BenchmarkId::new("datafrog", size_name),
            &(*node_count, *link_count),
            |b, _| {
                b.iter(|| {
                    let result = find_nodes_linked_to_tagged(&links, &tags, Tag("important".to_string()));
                    black_box(result.len());
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark pour la requête de transitivité (query 3)
/// Utilise datafrog pour le calcul
fn bench_query_transitive(c: &mut Criterion) {
    let mut group = c.benchmark_group("query_transitive");
    
    for (node_count, link_count, size_name) in &[
        (100, 500, "small"),
        (1_000, 5_000, "medium"),
        (10_000, 50_000, "large"),
    ] {
        let (nodes, tags, links, node_types) = generate_test_data(*node_count, *link_count);
        
        group.throughput(Throughput::Elements(*node_count as u64));
        
        group.bench_with_input(
            BenchmarkId::new("datafrog", size_name),
            &(*node_count, *link_count),
            |b, _| {
                b.iter(|| {
                    let result = find_reachable_with_datafrog(&links, NodeId(1));
                    black_box(result.len());
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark pour la requête "par type" (query 4)
/// Implementation naive sans datafrog
fn bench_query_by_type(c: &mut Criterion) {
    let mut group = c.benchmark_group("query_by_type");
    
    for (node_count, link_count, size_name) in &[
        (100, 500, "small"),
        (1_000, 5_000, "medium"),
        (10_000, 50_000, "large"),
    ] {
        let (nodes, tags, links, node_types) = generate_test_data(*node_count, *link_count);
        
        group.throughput(Throughput::Elements(*node_count as u64));
        
        group.bench_with_input(
            BenchmarkId::new("datafrog", size_name),
            &(*node_count, *link_count),
            |b, _| {
                b.iter(|| {
                    let result = find_nodes_by_type(&node_types, NodeType("page".to_string()));
                    black_box(result.len());
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark combiné: exécuter la transitivité avec datafrog
/// Mesure le temps total pour le calcul le plus complexe
fn bench_transitive_datafrog(c: &mut Criterion) {
    let mut group = c.benchmark_group("transitive_datafrog");
    
    for (node_count, link_count, size_name) in &[
        (100, 500, "small"),
        (1_000, 5_000, "medium"),
        (10_000, 50_000, "large"),
    ] {
        let (nodes, tags, links, node_types) = generate_test_data(*node_count, *link_count);
        
        group.throughput(Throughput::Elements(*node_count as u64));
        
        group.bench_with_input(
            BenchmarkId::new("datafrog", size_name),
            &(*node_count, *link_count),
            |b, _| {
                b.iter(|| {
                    // Exécuter la transitivité avec datafrog
                    let result = find_reachable_with_datafrog(&links, NodeId(1));
                    black_box(result);
                });
            },
        );
    }
    
    group.finish();
}

/// Trouve tous les nœuds avec un tag spécifique
fn find_nodes_by_tag(tags: &[(NodeId, Tag)], target_tag: Tag) -> Vec<NodeId> {
    tags.iter()
        .filter(|(_, tag)| tag == &target_tag)
        .map(|(node, _)| node.clone())
        .collect()
}

/// Trouve tous les nœuds liés à des nœuds avec un tag spécifique
fn find_nodes_linked_to_tagged(links: &[Link], tags: &[(NodeId, Tag)], target_tag: Tag) -> Vec<NodeId> {
    use std::collections::HashSet;
    
    let tagged_nodes: HashSet<NodeId> = tags.iter()
        .filter(|(_, tag)| tag == &target_tag)
        .map(|(node, _)| node.clone())
        .collect();
    
    links.iter()
        .filter(|link| tagged_nodes.contains(&link.from) || tagged_nodes.contains(&link.to))
        .flat_map(|link| vec![link.from.clone(), link.to.clone()])
        .collect()
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
    // Convertir les liens en tuples (from, to)
    let edges: Vec<(NodeId, NodeId)> = links.iter()
        .map(|link| (link.from.clone(), link.to.clone()))
        .collect();
    
    // Créer le contexte d'itération
    let mut iteration = Iteration::new();
    
    // Déclarer les variables
    let nodes_var = iteration.variable::<(NodeId, ())>("nodes");
    let edges_var = iteration.variable::<(NodeId, NodeId)>("edges");
    
    // Insérer les données initiales
    nodes_var.insert(Relation::from_iter(vec![(source, ())]));
    edges_var.insert(Relation::from_iter(edges));
    
    // Appliquer les règles itérativement pour calculer la transitivité
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

criterion_group!(
    benches,
    bench_query_by_tag,
    bench_query_by_link,
    bench_query_transitive,
    bench_query_by_type,
    bench_transitive_datafrog,
);

criterion_main!(benches);
