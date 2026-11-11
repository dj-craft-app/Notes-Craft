//! Note Craft Editor - Composant d'édition Markdown
//!
//! Ce module fournit:
//! - Parsing Markdown complet avec pulldown-cmark
//! - Edition en temps réel avec preview
//! - Gestion des blocs et pages
//! - Intégration avec le système de stockage

pub mod code_block;
pub mod markdown;

use note_craft_core::{Node, NodeId, TypeId};

// Ré-export des types principaux
pub use code_block::{
    copy_javascript, generate_css, CodeBlock, CodeBlockMetadata, CodeBlockParser,
};
pub use markdown::{MarkdownConfig, MarkdownEditor, MarkdownError, MarkdownResult};

/// Parse un document Markdown simple et retourne les nœuds extraits
///
/// **Note:** Cette fonction est conservée pour la compatibilité ascendante.
/// Pour une implémentation complète, utiliser `MarkdownEditor`.
pub fn parse_markdown(content: &str) -> Vec<Node> {
    let mut editor = MarkdownEditor::new();
    editor.set_content(content.to_string());

    // Pour l'instant, créons un nœud par paragraphe (comme avant)
    let mut nodes = Vec::new();
    let mut current_node_id = 1u64;

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

/// Tests de base pour la compatibilité ascendante
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
