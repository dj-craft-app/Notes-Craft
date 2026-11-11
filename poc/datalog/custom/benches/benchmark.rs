//! Benchmarks pour Custom (SQLite) avec criterion.rs
//!
//! Ces benchmarks mesurent **STRICTEMENT** l'exécution des requêtes SQL
//! sur une base de données déjà chargée, selon ADR-002-criteres-decision.md Section 2.
//!
//! IMPORTANT: Le setup (création DB + insertion données) est EXCLU de la mesure.
//! Seule l'exécution de la requête SQL et l'extraction du résultat sont chronométrées.
//!
//! Méthodologie:
//! - Setup: Créer la DB + insérer toutes les données - EXCLU
//! - Benchmark: Exécuter la requête SQL + extraction - INCLUS
//!
//! Exécution: cargo bench --bench benchmark
//!
//! Auteur: Agent autonome
//! Date: 2026-11-11 (corrigé pour symétrie de mesure)

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use custom_poc::{DatalogDb, generate_test_data, NodeId};

/// Benchmark pour la requête "par tag" (query 1)
/// MESURE: find_nodes_by_tag() UNIQUEMENT (DB déjà chargée)
fn bench_query_by_tag(c: &mut Criterion) {
    let mut group = c.benchmark_group("query_by_tag");
    
    for (node_count, link_count, size_name) in &[
        (100, 500, "small"),
        (1_000, 5_000, "medium"),
        (10_000, 50_000, "large"),
    ] {
        // SETUP: Générer les données et créer la DB - EXCLU de la mesure
        let (nodes, tags, links, node_types) = generate_test_data(*node_count, *link_count);
        let mut db = DatalogDb::new_in_memory().unwrap();
        db.insert_batch(&nodes, &tags.iter().map(|(n, t)| (*n, t.as_str())).collect::<Vec<_>>(), 
                       &links, &node_types.iter().map(|(n, t)| (*n, t.as_str())).collect::<Vec<_>>()).unwrap();
        
        group.throughput(Throughput::Elements(*node_count as u64));
        
        group.bench_with_input(
            BenchmarkId::new("custom_sqlite", size_name),
            &db,
            |b, db| {
                // MESURE: Exécution de la requête UNIQUEMENT
                b.iter(|| {
                    let result = db.find_nodes_by_tag("important").unwrap();
                    black_box(result.len());
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark pour la requête "par lien" (query 2)
/// MESURE: find_nodes_linked_to_tagged() UNIQUEMENT (DB déjà chargée)
fn bench_query_by_link(c: &mut Criterion) {
    let mut group = c.benchmark_group("query_by_link");
    
    for (node_count, link_count, size_name) in &[
        (100, 500, "small"),
        (1_000, 5_000, "medium"),
        (10_000, 50_000, "large"),
    ] {
        let (nodes, tags, links, node_types) = generate_test_data(*node_count, *link_count);
        let mut db = DatalogDb::new_in_memory().unwrap();
        db.insert_batch(&nodes, &tags.iter().map(|(n, t)| (*n, t.as_str())).collect::<Vec<_>>(), 
                       &links, &node_types.iter().map(|(n, t)| (*n, t.as_str())).collect::<Vec<_>>()).unwrap();
        
        group.throughput(Throughput::Elements(*link_count as u64));
        
        group.bench_with_input(
            BenchmarkId::new("custom_sqlite", size_name),
            &db,
            |b, db| {
                b.iter(|| {
                    let result = db.find_nodes_linked_to_tagged("important").unwrap();
                    black_box(result.len());
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark pour la requête de transitivité (query 3)
/// MESURE: find_reachable_nodes() UNIQUEMENT (DB déjà chargée)
fn bench_query_transitive(c: &mut Criterion) {
    let mut group = c.benchmark_group("query_transitive");
    
    for (node_count, link_count, size_name) in &[
        (100, 500, "small"),
        (1_000, 5_000, "medium"),
        (10_000, 50_000, "large"),
    ] {
        let (nodes, tags, links, node_types) = generate_test_data(*node_count, *link_count);
        let mut db = DatalogDb::new_in_memory().unwrap();
        db.insert_batch(&nodes, &tags.iter().map(|(n, t)| (*n, t.as_str())).collect::<Vec<_>>(), 
                       &links, &node_types.iter().map(|(n, t)| (*n, t.as_str())).collect::<Vec<_>>()).unwrap();
        
        group.throughput(Throughput::Elements(*node_count as u64));
        
        group.bench_with_input(
            BenchmarkId::new("custom_sqlite", size_name),
            &db,
            |b, db| {
                b.iter(|| {
                    let result = db.find_reachable_nodes(1).unwrap();
                    black_box(result.len());
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark pour la requête "par type" (query 4)
/// MESURE: find_nodes_by_type() UNIQUEMENT (DB déjà chargée)
fn bench_query_by_type(c: &mut Criterion) {
    let mut group = c.benchmark_group("query_by_type");
    
    for (node_count, link_count, size_name) in &[
        (100, 500, "small"),
        (1_000, 5_000, "medium"),
        (10_000, 50_000, "large"),
    ] {
        let (nodes, tags, links, node_types) = generate_test_data(*node_count, *link_count);
        let mut db = DatalogDb::new_in_memory().unwrap();
        db.insert_batch(&nodes, &tags.iter().map(|(n, t)| (*n, t.as_str())).collect::<Vec<_>>(), 
                       &links, &node_types.iter().map(|(n, t)| (*n, t.as_str())).collect::<Vec<_>>()).unwrap();
        
        group.throughput(Throughput::Elements(*node_count as u64));
        
        group.bench_with_input(
            BenchmarkId::new("custom_sqlite", size_name),
            &db,
            |b, db| {
                b.iter(|| {
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
