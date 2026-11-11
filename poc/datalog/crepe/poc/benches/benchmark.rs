//! Benchmarks pour crepe avec criterion.rs
//!
//! Ces benchmarks mesurent les performances de crepe sur les 4 requêtes
//! définies dans ADR-002-criteres-decision.md pour les 3 jeux de données
//! (100, 1000, 10000 nœuds).
//!
//! Exécution: cargo bench --bench benchmark
//!
//! Auteur: Agent autonome
//! Date: 2026-11-11

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use crepe::crepe;

// Définir les structures pour Note Craft
crepe! {
    @input
    struct Node(i32);

    @input
    struct Tag(i32, i32);

    @input
    struct Link(i32, i32);

    @input
    struct NodeType(i32, i32);

    @output
    struct NodesWithTag(i32, i32);

    @output
    struct LinkedToTag(i32, i32, i32);

    @output
    struct Reachable(i32, i32);

    @output
    struct NodesOfType(i32, i32);

    // Règles Datalog
    NodesWithTag(node, tag_id) <- Tag(node, tag_id);

    LinkedToTag(from, to, tag_id) <- 
        Link(from, to),
        Tag(to, tag_id);

    LinkedToTag(from, to, tag_id) <- 
        Link(to, from),
        Tag(from, tag_id);

    Reachable(from, to) <- Link(from, to);
    Reachable(from, to) <- 
        Link(from, intermediate),
        Reachable(intermediate, to);

    NodesOfType(node, type_id) <- NodeType(node, type_id);
}

/// Constantes pour les IDs de tags et types (correspondent à generate_test_data)
const TAG_IMPORTANT: i32 = 1;
const TAG_RUST: i32 = 2;
const TYPE_PAGE: i32 = 1;

/// Génère des données de test de manière déterministe
/// Identique à la fonction dans main.rs pour garantir la reproductibilité
fn generate_test_data(node_count: usize, link_count: usize) -> 
    (Vec<i32>, Vec<(i32, i32)>, Vec<(i32, i32)>, Vec<(i32, i32)>) {
    
    let nodes: Vec<i32> = (1..=node_count as i32).collect();
    
    // Générer des tags
    let tag_names = ["important", "rust", "datalog", "note-craft", "test"];
    let mut tags = Vec::new();
    for (idx, &node) in nodes.iter().enumerate() {
        let num_tags = (idx % 3) + 1;
        for i in 0..num_tags {
            let tag_idx = (idx + i) % tag_names.len();
            let tag_id = match tag_names[tag_idx] {
                "important" => 1,
                "rust" => 2,
                "datalog" => 3,
                "note-craft" => 4,
                "test" => 5,
                _ => 0,
            };
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
    
    // Générer des types
    let types = ["page", "block", "asset", "query"];
    let node_types: Vec<(i32, i32)> = nodes.iter()
        .enumerate()
        .map(|(idx, &n)| {
            let type_idx = idx % types.len();
            let type_id = match types[type_idx] {
                "page" => 1,
                "block" => 2,
                "asset" => 3,
                "query" => 4,
                _ => 0,
            };
            (n, type_id)
        })
        .collect();

    (nodes, tags, links, node_types)
}

/// Benchmark pour la requête "par tag" (query 1)
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
            BenchmarkId::new("crepe", size_name),
            &(*node_count, *link_count),
            |b, _| {
                b.iter(|| {
                    let mut runtime = Crepe::new();
                    runtime.extend(nodes.iter().copied().map(Node));
                    runtime.extend(tags.iter().copied().map(|(n, t)| Tag(n, t)));
                    runtime.extend(links.iter().copied().map(|(f, t)| Link(f, t)));
                    runtime.extend(node_types.iter().copied().map(|(n, t)| NodeType(n, t)));
                    
                    let (nodes_with_tag, _, _, _) = runtime.run();
                    
                    // Filtrer pour tag "important" (TAG_IMPORTANT = 1)
                    black_box(
                        nodes_with_tag
                            .into_iter()
                            .filter(|NodesWithTag(_, tag_id)| *tag_id == TAG_IMPORTANT)
                            .count()
                    );
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark pour la requête "par lien" (query 2)
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
            BenchmarkId::new("crepe", size_name),
            &(*node_count, *link_count),
            |b, _| {
                b.iter(|| {
                    let mut runtime = Crepe::new();
                    runtime.extend(nodes.iter().copied().map(Node));
                    runtime.extend(tags.iter().copied().map(|(n, t)| Tag(n, t)));
                    runtime.extend(links.iter().copied().map(|(f, t)| Link(f, t)));
                    runtime.extend(node_types.iter().copied().map(|(n, t)| NodeType(n, t)));
                    
                    let (_, linked_to_tag, _, _) = runtime.run();
                    
                    // Compter tous les liens avec tags
                    black_box(linked_to_tag.into_iter().count());
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark pour la requête de transitivité (query 3)
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
            BenchmarkId::new("crepe", size_name),
            &(*node_count, *link_count),
            |b, _| {
                b.iter(|| {
                    let mut runtime = Crepe::new();
                    runtime.extend(nodes.iter().copied().map(Node));
                    runtime.extend(tags.iter().copied().map(|(n, t)| Tag(n, t)));
                    runtime.extend(links.iter().copied().map(|(f, t)| Link(f, t)));
                    runtime.extend(node_types.iter().copied().map(|(n, t)| NodeType(n, t)));
                    
                    let (_, _, reachable, _) = runtime.run();
                    
                    // Compter tous les nœuds accessibles
                    black_box(reachable.into_iter().count());
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark pour la requête "par type" (query 4)
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
            BenchmarkId::new("crepe", size_name),
            &(*node_count, *link_count),
            |b, _| {
                b.iter(|| {
                    let mut runtime = Crepe::new();
                    runtime.extend(nodes.iter().copied().map(Node));
                    runtime.extend(tags.iter().copied().map(|(n, t)| Tag(n, t)));
                    runtime.extend(links.iter().copied().map(|(f, t)| Link(f, t)));
                    runtime.extend(node_types.iter().copied().map(|(n, t)| NodeType(n, t)));
                    
                    let (_, _, _, nodes_of_type) = runtime.run();
                    
                    // Filtrer pour type "page" (TYPE_PAGE = 1)
                    black_box(
                        nodes_of_type
                            .into_iter()
                            .filter(|NodesOfType(_, type_id)| *type_id == TYPE_PAGE)
                            .count()
                    );
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark combiné: exécuter toutes les requêtes ensemble
/// Mesure le temps total pour toutes les règles Datalog
fn bench_all_queries(c: &mut Criterion) {
    let mut group = c.benchmark_group("all_queries");
    
    for (node_count, link_count, size_name) in &[
        (100, 500, "small"),
        (1_000, 5_000, "medium"),
        (10_000, 50_000, "large"),
    ] {
        let (nodes, tags, links, node_types) = generate_test_data(*node_count, *link_count);
        
        group.throughput(Throughput::Elements(*node_count as u64));
        
        group.bench_with_input(
            BenchmarkId::new("crepe", size_name),
            &(*node_count, *link_count),
            |b, _| {
                b.iter(|| {
                    let mut runtime = Crepe::new();
                    runtime.extend(nodes.iter().copied().map(Node));
                    runtime.extend(tags.iter().copied().map(|(n, t)| Tag(n, t)));
                    runtime.extend(links.iter().copied().map(|(f, t)| Link(f, t)));
                    runtime.extend(node_types.iter().copied().map(|(n, t)| NodeType(n, t)));
                    
                    let (nodes_with_tag, linked_to_tag, reachable, nodes_of_type) = runtime.run();
                    
                    // Consommer tous les résultats pour éviter l'optimisation
                    black_box((
                        nodes_with_tag.into_iter().count(),  // NodesWithTag
                        linked_to_tag.into_iter().count(),  // LinkedToTag
                        reachable.into_iter().count(),  // Reachable
                        nodes_of_type.into_iter().count(),  // NodesOfType
                    ));
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
    bench_all_queries,
);

criterion_main!(benches);
