//! Benchmarks pour Custom (SQLite) avec criterion.rs
//!
//! Ces benchmarks mesurent les performances de SQLite sur les 4 requêtes
//! définies dans ADR-002-criteres-decision.md pour les 3 jeux de données
//! (100, 1000, 10000 nœuds).
//!
//! Exécution: cargo bench --bench benchmark
//!
//! Auteur: Agent autonome
//! Date: 2026-11-11

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use custom_poc::{DatalogDb, generate_test_data};

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
            BenchmarkId::new("custom_sqlite", size_name),
            &(*node_count, *link_count),
            |b, _| {
                b.iter(|| {
                    let mut db = DatalogDb::new_in_memory().unwrap();
                    db.insert_batch(&nodes, &tags.iter().map(|(n, t)| (*n, t.as_str())).collect::<Vec<_>>(), 
                                   &links, &node_types.iter().map(|(n, t)| (*n, t.as_str())).collect::<Vec<_>>()).unwrap();
                    
                    let result = db.find_nodes_by_tag("important").unwrap();
                    black_box(result.len());
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
            BenchmarkId::new("custom_sqlite", size_name),
            &(*node_count, *link_count),
            |b, _| {
                b.iter(|| {
                    let mut db = DatalogDb::new_in_memory().unwrap();
                    db.insert_batch(&nodes, &tags.iter().map(|(n, t)| (*n, t.as_str())).collect::<Vec<_>>(), 
                                   &links, &node_types.iter().map(|(n, t)| (*n, t.as_str())).collect::<Vec<_>>()).unwrap();
                    
                    let result = db.find_nodes_linked_to_tagged("important").unwrap();
                    black_box(result.len());
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
            BenchmarkId::new("custom_sqlite", size_name),
            &(*node_count, *link_count),
            |b, _| {
                b.iter(|| {
                    let mut db = DatalogDb::new_in_memory().unwrap();
                    db.insert_batch(&nodes, &tags.iter().map(|(n, t)| (*n, t.as_str())).collect::<Vec<_>>(), 
                                   &links, &node_types.iter().map(|(n, t)| (*n, t.as_str())).collect::<Vec<_>>()).unwrap();
                    
                    let result = db.find_reachable_nodes(1).unwrap();
                    black_box(result.len());
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
            BenchmarkId::new("custom_sqlite", size_name),
            &(*node_count, *link_count),
            |b, _| {
                b.iter(|| {
                    let mut db = DatalogDb::new_in_memory().unwrap();
                    db.insert_batch(&nodes, &tags.iter().map(|(n, t)| (*n, t.as_str())).collect::<Vec<_>>(), 
                                   &links, &node_types.iter().map(|(n, t)| (*n, t.as_str())).collect::<Vec<_>>()).unwrap();
                    
                    let result = db.find_nodes_by_type("page").unwrap();
                    black_box(result.len());
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
