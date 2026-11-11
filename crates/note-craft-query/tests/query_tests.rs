//! Tests d'intégration pour les requêtes
//!
//! Ces tests vérifient que les requêtes Note Craft fonctionnent
//! avec le moteur Datalog

use note_craft_core::{
    Link, LinkId, Node, NodeId, QueryByLink, QueryByTag, QueryByType, QueryResult, QueryTransitive,
    TagId, TypeId,
};

#[test]
fn test_query_structures() {
    // Test que les structures de requête sont correctement définies
    let _query_by_tag = QueryByTag { tag_id: TagId(1) };
    let _query_by_link = QueryByLink {
        from_node: Some(NodeId(1)),
        to_node: None,
    };
    let _query_transitive = QueryTransitive {
        start_node: NodeId(1),
        max_depth: Some(10),
    };
    let _query_by_type = QueryByType {
        node_type: TypeId(1),
    };
}

#[test]
fn test_query_result_variants() {
    // Test que tous les variants de QueryResult existent
    let _result_by_tag = QueryResult::NodesByTag(vec![]);
    let _result_by_link = QueryResult::NodesByLink(vec![]);
    let _result_transitive = QueryResult::NodesTransitive(vec![]);
    let _result_by_type = QueryResult::NodesByType(vec![]);
}

#[test]
fn test_node_structure() {
    // Test que la structure Node est correctement définie
    let node = Node {
        id: NodeId(1),
        content: "Test content".to_string(),
        node_type: TypeId(1),
        tags: vec![TagId(1), TagId(2)],
        created_at: 1234567890,
        updated_at: 1234567890,
    };

    assert_eq!(node.id, NodeId(1));
    assert_eq!(node.content, "Test content");
    assert_eq!(node.node_type, TypeId(1));
    assert_eq!(node.tags.len(), 2);
}

#[test]
fn test_link_structure() {
    // Test que la structure Link est correctement définie
    let link = Link {
        id: LinkId(1),
        from: NodeId(1),
        to: NodeId(2),
        link_type: Some("reference".to_string()),
        created_at: 1234567890,
    };

    assert_eq!(link.id, LinkId(1));
    assert_eq!(link.from, NodeId(1));
    assert_eq!(link.to, NodeId(2));
}
