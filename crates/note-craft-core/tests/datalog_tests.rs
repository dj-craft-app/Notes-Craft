//! Tests d'intégration pour le moteur Datalog
//!
//! Ces tests vérifient que l'abstraction du moteur Datalog fonctionne
//! correctement avec datafrog 2.0.1

use note_craft_core::{
    DatafrogEngine, DatalogEngine, NodeId, QueryByLink, QueryByTag, QueryByType, QueryTransitive,
    TagId, TypeId,
};

#[test]
fn test_datafrog_engine_creation() {
    let _engine = DatafrogEngine::new();
    // DatafrogEngine créé avec succès
}

#[test]
fn test_query_by_tag_empty() {
    let engine = DatafrogEngine::new();
    let query = QueryByTag { tag_id: TagId(1) };

    let result = engine.query_by_tag(&query);
    assert!(result.is_ok());
}

#[test]
fn test_query_by_link_empty() {
    let engine = DatafrogEngine::new();
    let query = QueryByLink {
        from_node: Some(NodeId(1)),
        to_node: None,
    };

    let result = engine.query_by_link(&query);
    assert!(result.is_ok());
}

#[test]
fn test_query_transitive_empty() {
    let engine = DatafrogEngine::new();
    let query = QueryTransitive {
        start_node: NodeId(1),
        max_depth: Some(10),
    };

    let result = engine.query_transitive(&query);
    assert!(result.is_ok());
}

#[test]
fn test_query_by_type_empty() {
    let engine = DatafrogEngine::new();
    let query = QueryByType {
        node_type: TypeId(1),
    };

    let result = engine.query_by_type(&query);
    assert!(result.is_ok());
}

#[test]
fn test_load_nodes_empty() {
    let mut engine = DatafrogEngine::new();
    let nodes = vec![];

    let result = engine.load_nodes(nodes);
    assert!(result.is_ok());
}

#[test]
fn test_load_links_empty() {
    let mut engine = DatafrogEngine::new();
    let links = vec![];

    let result = engine.load_links(links);
    assert!(result.is_ok());
}
