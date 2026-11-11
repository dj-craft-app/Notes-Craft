//! Note Craft Editor - Composant d'édition Markdown
//!
//! Ce module fournit:
//! - Parsing Markdown de base
//! - Gestion des blocs et pages
//! - Intégration avec le système de stockage

use note_craft_core::{Node, NodeId, TypeId};

/// Parse un document Markdown simple et retourne les nœuds extraits
///
/// Note: Cette implémentation est simplifiée pour la Phase 0.
/// Une implémentation complète avec pulldown-cmark viendra plus tard.
pub fn parse_markdown(content: &str) -> Vec<Node> {
    let mut nodes = Vec::new();
    let mut current_node_id = 1u64;

    // Split par doubles sauts de ligne pour séparer les paragraphes
    for paragraph in content.split("\n\n") {
        if paragraph.trim().is_empty() {
            continue;
        }

        nodes.push(Node {
            id: NodeId(current_node_id),
            content: paragraph.trim().to_string(),
            node_type: TypeId(1), // Type par défaut: paragraph
            tags: vec![],
            created_at: 0,
            updated_at: 0,
        });
        current_node_id += 1;
    }

    nodes
}

/// Tests de base
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_markdown() {
        let markdown = "# Title\n\nContent here";
        let nodes = parse_markdown(markdown);

        // Doit retourner au moins un nœud
        assert!(!nodes.is_empty());
    }

    #[test]
    fn test_parse_empty() {
        let markdown = "";
        let nodes = parse_markdown(markdown);

        // Peut retourner vide
        assert!(nodes.is_empty());
    }

    #[test]
    fn test_parse_multiple_paragraphs() {
        let markdown = "First\n\nSecond\n\nThird";
        let nodes = parse_markdown(markdown);

        assert_eq!(nodes.len(), 3);
        assert_eq!(nodes[0].content, "First");
        assert_eq!(nodes[1].content, "Second");
        assert_eq!(nodes[2].content, "Third");
    }
}
