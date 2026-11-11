//! Benchmarks pour crepe avec criterion.rs
//!
//! Ces benchmarks mesurent l'exécution des requêtes Datalog sur crepe.
//!
//! IMPORTANT - LIMITATION CREPE:
//! crepe::run() consume le runtime et retourne un tuple de HashSet.
//! Il n'est PAS possible de séparer le setup (insertion des données) de
//! l'exécution car chaque appel à run() nécessite un nouveau runtime.
//!
//! Par conséquent, pour crepe, nous mesurons:
//! - Setup INCLUS: création du runtime + insertion des données
//! - Exécution INCLUSE: run() + extraction du résultat
//!
//! Cela diffère de datafrog et Custom où le setup est exclu.
//! Cette limitation est documentée dans les résultats.
//!
//! Exécution: cargo bench --bench benchmark
//!
//! Auteur: Agent autonome
//! Date: 2026-11-11 (limitation crepe documentée)

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

/// Constantes pour les IDs de tags et types
const TAG_IMPORTANT: i32 = 1;
const TYPE_PAGE: i32 = 1;

/// Génère des données de test de manière déterministe
fn generate_test_data(node_count: usize, link_count: usize) -> 
    (Vec<i32>, Vec<(i32, i32)>, Vec<(i32, i32)>, Vec<(i32, i32)>) {
    
    let nodes: Vec<i32> = (1..=node_count as i32).collect();
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
    
    let mut links = Vec::new();
    for i in 0..link_count {
        let from_idx = i % nodes.len();
        let to_idx = ((i + 13) * 7) % nodes.len();
        links.push((nodes[from_idx], nodes[to_idx]));
    }
    
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

/// Helper pour créer un runtime crepe avec les données
fn build_runtime(nodes: &[i32], tags: &[(i32, i32)], links: &[(i32, i32)], node_types: &[(i32, i32)]) -> Crepe {
    let mut runtime = Crepe::new();
    runtime.extend(nodes.iter().copied().map(|n| Node(n)));
    runtime.extend(tags.iter().copied().map(|(n, t)| Tag(n, t)));
    runtime.extend(links.iter().copied().map(|(f, t)| Link(f, t)));
    runtime.extend(node_types.iter().copied().map(|(n, t)| NodeType(n, t)));
    runtime
}

/// Benchmark pour la requête "par tag" (query 1)
/// MESURE: Setup (runtime + insertion) + run() + extraction
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
                    // CREPE LIMITATION: Doit recréer et tout réinsérer
                    let mut runtime = build_runtime(&nodes, &tags, &links, &node_types);
                    let result = runtime.run();
                    
                    // Extraire le résultat pour query_by_tag
                    let count: usize = result.0
                        .into_iter()
                        .filter(|x| x.1 == TAG_IMPORTANT)
                        .count();
                    black_box(count);
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
                    let mut runtime = build_runtime(&nodes, &tags, &links, &node_types);
                    let result = runtime.run();
                    
                    // Extraire le résultat pour query_by_link
                    let count: usize = result.1.into_iter().count();
                    black_box(count);
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
                    let mut runtime = build_runtime(&nodes, &tags, &links, &node_types);
                    let result = runtime.run();
                    
                    // Extraire le résultat pour query_transitive
                    let count: usize = result.2.into_iter().count();
                    black_box(count);
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
                    let mut runtime = build_runtime(&nodes, &tags, &links, &node_types);
                    let result = runtime.run();
                    
                    // Extraire le résultat pour query_by_type
                    let count: usize = result.3
                        .into_iter()
                        .filter(|x| x.1 == TYPE_PAGE)
                        .count();
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
