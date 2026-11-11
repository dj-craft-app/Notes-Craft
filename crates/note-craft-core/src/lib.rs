//! Note Craft Core - Types et abstractions du moteur Datalog
//!
//! Ce module fournit:
//! - Les types de base pour représenter les nœuds, liens, tags, etc.
//! - Une abstraction du moteur Datalog (datafrog) pour isoler l'API
//! - Une abstraction du système de fichiers (FileSystem trait)
//! - Des tests de non-régression sur les 4 requêtes du PoC
//!
//! Décision: datafrog 2.0.1 retenu (voir ADR-002, tag v0.2.0-datalog-decision)
use serde::{Deserialize, Serialize};
use thiserror::Error;

// Abstraction du système de fichiers (Phase 1 - Étape 1.1)
pub mod fs;

// Block System (Phase 1 - Étape 1.2)
pub mod block;
pub use block::{Block, BlockContent, BlockId, BlockSystem, BlockType, TextFormatting};

// ============================================================================
// TYPES PROPRES À NOTE CRAFT
// ============================================================================

/// Identifiant unique pour un nœud (Note, Page, Block)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub u64);

/// Identifiant unique pour un tag
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TagId(pub u64);

/// Identifiant unique pour un type de nœud
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TypeId(pub u64);

/// Identifiant unique pour un lien
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LinkId(pub u64);

/// Représente un nœud dans Note Craft
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: NodeId,
    pub content: String,
    pub node_type: TypeId,
    pub tags: Vec<TagId>,
    pub created_at: u64,
    pub updated_at: u64,
}

/// Représente un lien entre deux nœuds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    pub id: LinkId,
    pub from: NodeId,
    pub to: NodeId,
    pub link_type: Option<String>,
    pub created_at: u64,
}

// ============================================================================
// TYPES DE REQUÊTES (Correspondent aux 4 requêtes du PoC)
// ============================================================================

/// Requête 1: Trouver les nœuds avec un tag spécifique
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryByTag {
    pub tag_id: TagId,
}

/// Requête 2: Trouver les nœuds liés à un nœud donné
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryByLink {
    pub from_node: Option<NodeId>,
    pub to_node: Option<NodeId>,
}

/// Requête 3: Requête de transitivité (tous les nœuds accessibles)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryTransitive {
    pub start_node: NodeId,
    pub max_depth: Option<usize>,
}

/// Requête 4: Trouver les nœuds d'un type spécifique
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryByType {
    pub node_type: TypeId,
}

/// Résultat générique d'une requête
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueryResult {
    NodesByTag(Vec<NodeId>),
    NodesByLink(Vec<(NodeId, NodeId)>),
    NodesTransitive(Vec<NodeId>),
    NodesByType(Vec<NodeId>),
}

// ============================================================================
// ERREURS
// ============================================================================

#[derive(Error, Debug)]
pub enum DatalogError {
    #[error("Erreur de requête Datalog: {0}")]
    QueryError(String),
    #[error("Erreur d'exécution: {0}")]
    ExecutionError(String),
    #[error("Aucun résultat trouvé")]
    NoResults,
}

// ============================================================================
// ABSTRACTION DU MOTEUR DATALOG
// ============================================================================

/// Trait d'abstraction pour le moteur Datalog
///
/// Cette abstraction permet d'isoler l'API spécifique de datafrog
/// et de faciliter un éventuel changement de moteur à l'avenir.
pub trait DatalogEngine {
    /// Exécute une requête "par tag"
    fn query_by_tag(&self, query: &QueryByTag) -> Result<QueryResult, DatalogError>;

    /// Exécute une requête "par lien"
    fn query_by_link(&self, query: &QueryByLink) -> Result<QueryResult, DatalogError>;

    /// Exécute une requête de transitivité
    fn query_transitive(&self, query: &QueryTransitive) -> Result<QueryResult, DatalogError>;

    /// Exécute une requête "par type"
    fn query_by_type(&self, query: &QueryByType) -> Result<QueryResult, DatalogError>;
}

// ============================================================================
// IMPLÉMENTATION AVEC DATAFROG 2.0.1
// ============================================================================

/// Implémentation concrète utilisant datafrog 2.0.1
///
/// Décision: datafrog retenu après PoC (Phase 0.13)
/// Score: 96.00/100
/// Tag: v0.2.0-datalog-decision
#[derive(Default)]
pub struct DatafrogEngine {
    // Stockage des données dans datafrog
    // Note: datafrog utilise un système de relations bas niveau
    // Nous avons une couche d'abstraction pour les types Note Craft
}

impl DatafrogEngine {
    pub fn new() -> Self {
        Self::default()
    }

    /// Charge les nœuds dans le moteur
    pub fn load_nodes(&mut self, _nodes: Vec<Node>) -> Result<(), DatalogError> {
        // TODO: Implémentation avec datafrog
        // Pour l'instant, nous retournons Ok car c'est un stub
        Ok(())
    }

    /// Charge les liens dans le moteur
    pub fn load_links(&mut self, _links: Vec<Link>) -> Result<(), DatalogError> {
        // TODO: Implémentation avec datafrog
        Ok(())
    }
}

impl DatalogEngine for DatafrogEngine {
    fn query_by_tag(&self, _query: &QueryByTag) -> Result<QueryResult, DatalogError> {
        // TODO: Implémentation avec datafrog
        // Stub pour compilation
        Ok(QueryResult::NodesByTag(vec![]))
    }

    fn query_by_link(&self, _query: &QueryByLink) -> Result<QueryResult, DatalogError> {
        // TODO: Implémentation avec datafrog
        Ok(QueryResult::NodesByLink(vec![]))
    }

    fn query_transitive(&self, _query: &QueryTransitive) -> Result<QueryResult, DatalogError> {
        // TODO: Implémentation avec datafrog
        Ok(QueryResult::NodesTransitive(vec![]))
    }

    fn query_by_type(&self, _query: &QueryByType) -> Result<QueryResult, DatalogError> {
        // TODO: Implémentation avec datafrog
        Ok(QueryResult::NodesByType(vec![]))
    }
}

// ============================================================================
// TESTS DE NON-RÉGRESSION (4 requêtes du PoC)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Test 1: Requête par tag - doit compiler et retourner un résultat
    #[test]
    fn test_query_by_tag_structure() {
        let engine = DatafrogEngine::new();
        let query = QueryByTag { tag_id: TagId(1) };

        let result = engine.query_by_tag(&query);
        assert!(result.is_ok());
    }

    /// Test 2: Requête par lien - doit compiler et retourner un résultat
    #[test]
    fn test_query_by_link_structure() {
        let engine = DatafrogEngine::new();
        let query = QueryByLink {
            from_node: Some(NodeId(1)),
            to_node: None,
        };

        let result = engine.query_by_link(&query);
        assert!(result.is_ok());
    }

    /// Test 3: Requête de transitivité - doit compiler et retourner un résultat
    #[test]
    fn test_query_transitive_structure() {
        let engine = DatafrogEngine::new();
        let query = QueryTransitive {
            start_node: NodeId(1),
            max_depth: Some(10),
        };

        let result = engine.query_transitive(&query);
        assert!(result.is_ok());
    }

    /// Test 4: Requête par type - doit compiler et retourner un résultat
    #[test]
    fn test_query_by_type_structure() {
        let engine = DatafrogEngine::new();
        let query = QueryByType {
            node_type: TypeId(1),
        };

        let result = engine.query_by_type(&query);
        assert!(result.is_ok());
    }

    /// Test 5: Types de base - sérialisation/désérialisation
    #[test]
    fn test_types_serialization() {
        let node = Node {
            id: NodeId(1),
            content: "Test content".to_string(),
            node_type: TypeId(1),
            tags: vec![TagId(1), TagId(2)],
            created_at: 1234567890,
            updated_at: 1234567890,
        };

        // Test serialization
        let serialized = serde_json::to_string(&node).unwrap();

        // Test deserialization
        let deserialized: Node = serde_json::from_str(&serialized).unwrap();

        assert_eq!(node.id, deserialized.id);
        assert_eq!(node.content, deserialized.content);
    }
}
