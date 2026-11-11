//! Note Craft Block System
//!
//! Ce module fournit la structure de base des blocs, élément fondamental
//! de Note Craft.
//!
//! Phase 1 - Étape 1.2: Block System (structure)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Identifiant unique pour un bloc
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BlockId(pub u64);

impl Default for BlockId {
    fn default() -> Self {
        Self::new()
    }
}

impl BlockId {
    /// Génère un nouvel identifiant (simplified pour la Phase 1)
    pub fn new() -> Self {
        // TODO: Utiliser un générateur d'ID plus robuste (ex: snowflake)
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        BlockId(COUNTER.fetch_add(1, Ordering::SeqCst))
    }
}

/// Type de bloc
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlockType {
    /// Bloc de texte simple
    Text,
    /// Bloc de titre
    Heading(u8), // Niveau 1-6
    /// Bloc de liste (non ordonnée)
    BulletList,
    /// Bloc de liste ordonnée
    NumberedList,
    /// Bloc de code
    Code,
    /// Bloc de citation
    Quote,
    /// Bloc horizontal (séparateur)
    HorizontalRule,
    /// Bloc de lien
    Link,
    /// Bloc de référence vers un autre bloc
    BlockReference(BlockId),
    /// Bloc de page (référence vers une page)
    PageReference,
    /// Bloc de tag
    Tag,
}

/// Contenu du bloc
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BlockContent {
    /// Texte simple
    Text(String),
    /// Liste de sous-blocs (pour les listes, quotes, etc.)
    Children(Vec<Block>),
    /// Contenu combiné (texte + métadonnées)
    RichText {
        text: String,
        formatting: TextFormatting,
    },
}

/// Formatage du texte
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct TextFormatting {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
    pub code: bool,
}

/// Un bloc Note Craft
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Block {
    /// Identifiant unique du bloc
    pub id: BlockId,
    /// Type du bloc
    pub block_type: BlockType,
    /// Contenu du bloc
    pub content: BlockContent,
    /// ID de la page parente (si applicable)
    pub page_id: Option<u64>,
    /// Position dans la page (pour l'ordre)
    pub position: usize,
    /// Bloc parent (pour les blocs imbriqués)
    pub parent_id: Option<BlockId>,
    /// Enfants directs (si ce bloc est un conteneur)
    pub children: Vec<BlockId>,
    /// Métadonnées: horodatage de création (timestamp)
    pub created_at: u64,
    /// Métadonnées: horodatage de modification (timestamp)
    pub updated_at: u64,
    /// Tags associés au bloc
    pub tags: Vec<String>,
}

impl Block {
    /// Crée un nouveau bloc de texte
    pub fn new_text(content: String) -> Self {
        let now = Self::current_timestamp();
        Block {
            id: BlockId::new(),
            block_type: BlockType::Text,
            content: BlockContent::Text(content),
            page_id: None,
            position: 0,
            parent_id: None,
            children: Vec::new(),
            created_at: now,
            updated_at: now,
            tags: Vec::new(),
        }
    }

    /// Crée un nouveau bloc de titre
    pub fn new_heading(level: u8, content: String) -> Self {
        let level = level.clamp(1, 6); // Limiter entre 1 et 6
        let now = Self::current_timestamp();
        Block {
            id: BlockId::new(),
            block_type: BlockType::Heading(level),
            content: BlockContent::Text(content),
            page_id: None,
            position: 0,
            parent_id: None,
            children: Vec::new(),
            created_at: now,
            updated_at: now,
            tags: Vec::new(),
        }
    }

    /// Crée un nouveau bloc de code
    pub fn new_code(content: String, language: Option<String>) -> Self {
        let now = Self::current_timestamp();
        Block {
            id: BlockId::new(),
            block_type: BlockType::Code,
            content: BlockContent::Text(format!(
                "{}{}{}",
                language.as_deref().unwrap_or(""),
                if language.is_some() { "\n" } else { "" },
                content
            )),
            page_id: None,
            position: 0,
            parent_id: None,
            children: Vec::new(),
            created_at: now,
            updated_at: now,
            tags: Vec::new(),
        }
    }

    /// Met à jour le contenu du bloc
    pub fn update_content(&mut self, new_content: BlockContent) {
        self.content = new_content;
        self.updated_at = Self::current_timestamp();
    }

    /// Ajoute un tag au bloc
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
            self.updated_at = Self::current_timestamp();
        }
    }

    /// Supprime un tag du bloc
    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.retain(|t| t != tag);
        self.updated_at = Self::current_timestamp();
    }

    /// Obtient le timestamp actuel en secondes depuis epoch
    fn current_timestamp() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
    }

    /// Obtient le texte du bloc (pour les blocs de type texte)
    pub fn get_text(&self) -> Option<String> {
        match &self.content {
            BlockContent::Text(text) => Some(text.clone()),
            BlockContent::RichText { text, .. } => Some(text.clone()),
            BlockContent::Children(_children) => {
                // TODO: Concaténer le texte de tous les enfants
                // Pour l'instant, retourner None car nous n'avons pas le store
                None
            }
        }
    }

    /// Vérifie si le bloc est un conteneur (a des enfants)
    pub fn is_container(&self) -> bool {
        !self.children.is_empty()
    }

    /// Ajoute un enfant au bloc
    pub fn add_child(&mut self, child_id: BlockId) {
        if !self.children.contains(&child_id) {
            self.children.push(child_id);
            self.updated_at = Self::current_timestamp();
        }
    }

    /// Supprime un enfant du bloc
    pub fn remove_child(&mut self, child_id: &BlockId) {
        self.children.retain(|&id| id != *child_id);
        self.updated_at = Self::current_timestamp();
    }
}

/// Système de gestion des blocs
///
/// Ce struct gère une collection de blocs avec des opérations CRUD.
pub struct BlockSystem {
    /// Map des blocs par ID
    blocks: HashMap<BlockId, Block>,
    // Note: Utilise BlockId::new() qui a son propre compteur
}

impl BlockSystem {
    pub fn new() -> Self {
        BlockSystem {
            blocks: HashMap::new(),
        }
    }

    /// Ajoute un bloc au système
    pub fn add_block(&mut self, mut block: Block) -> BlockId {
        let id = block.id;
        block.position = self.blocks.len();
        self.blocks.insert(id, block);
        id
    }

    /// Récupère un bloc par ID
    pub fn get_block(&self, id: BlockId) -> Option<&Block> {
        self.blocks.get(&id)
    }

    /// Récupère un bloc par ID (mutable)
    pub fn get_block_mut(&mut self, id: BlockId) -> Option<&mut Block> {
        self.blocks.get_mut(&id)
    }

    /// Supprime un bloc du système
    pub fn remove_block(&mut self, id: BlockId) -> Option<Block> {
        self.blocks.remove(&id)
    }

    /// Met à jour un bloc
    pub fn update_block(&mut self, id: BlockId, block: Block) -> bool {
        self.blocks.contains_key(&id) && {
            self.blocks.insert(id, block);
            true
        }
    }

    /// Liste tous les blocs
    pub fn list_blocks(&self) -> Vec<&Block> {
        self.blocks.values().collect()
    }

    /// Trouve les blocs par tag
    pub fn find_by_tag(&self, tag: &str) -> Vec<&Block> {
        self.blocks
            .values()
            .filter(|block| block.tags.contains(&tag.to_string()))
            .collect()
    }

    /// Trouve les blocs par type
    pub fn find_by_type(&self, block_type: &BlockType) -> Vec<&Block> {
        self.blocks
            .values()
            .filter(|block| &block.block_type == block_type)
            .collect()
    }

    /// Compte le nombre de blocs
    pub fn count(&self) -> usize {
        self.blocks.len()
    }

    /// Vérifie si un bloc existe
    pub fn contains(&self, id: BlockId) -> bool {
        self.blocks.contains_key(&id)
    }
}

impl Default for BlockSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_id_new() {
        let id1 = BlockId::new();
        let id2 = BlockId::new();
        assert_ne!(id1, id2, "Les IDs doivent être uniques");
    }

    #[test]
    fn test_block_new_text() {
        let content = "Hello, Note Craft!".to_string();
        let block = Block::new_text(content.clone());

        assert_eq!(block.block_type, BlockType::Text);
        assert_eq!(block.get_text(), Some(content));
        assert!(block.created_at > 0);
        assert!(block.updated_at > 0);
    }

    #[test]
    fn test_block_new_heading() {
        let block = Block::new_heading(1, "Title".to_string());

        assert_eq!(block.block_type, BlockType::Heading(1));
        assert_eq!(block.get_text(), Some("Title".to_string()));
    }

    #[test]
    fn test_block_new_heading_clamped() {
        // Test que le niveau est clampé entre 1 et 6
        let block_h7 = Block::new_heading(7, "Title".to_string());
        let block_h0 = Block::new_heading(0, "Title".to_string());

        assert_eq!(block_h7.block_type, BlockType::Heading(6));
        assert_eq!(block_h0.block_type, BlockType::Heading(1));
    }

    #[test]
    fn test_block_new_code() {
        let block = Block::new_code("fn main() {}".to_string(), Some("rust".to_string()));

        assert_eq!(block.block_type, BlockType::Code);
        let content = block.get_text().unwrap();
        assert!(content.contains("rust"));
        assert!(content.contains("fn main() {}"));
    }

    #[test]
    fn test_block_add_tag() {
        let mut block = Block::new_text("Test".to_string());
        let initial_len = block.tags.len();

        block.add_tag("tag1".to_string());
        assert_eq!(block.tags.len(), initial_len + 1);
        assert!(block.tags.contains(&"tag1".to_string()));

        // Ajouter le même tag ne doit pas le dupliquer
        block.add_tag("tag1".to_string());
        assert_eq!(block.tags.len(), initial_len + 1);
    }

    #[test]
    fn test_block_remove_tag() {
        let mut block = Block::new_text("Test".to_string());
        block.add_tag("tag1".to_string());
        block.add_tag("tag2".to_string());

        block.remove_tag("tag1");
        assert_eq!(block.tags.len(), 1);
        assert!(!block.tags.contains(&"tag1".to_string()));
        assert!(block.tags.contains(&"tag2".to_string()));
    }

    #[test]
    fn test_block_update_content() {
        let mut block = Block::new_text("Original".to_string());
        let original_updated_at = block.updated_at;

        // Attendre un peu pour que le timestamp change
        std::thread::sleep(std::time::Duration::from_millis(10));

        block.update_content(BlockContent::Text("Updated".to_string()));

        assert_eq!(block.get_text(), Some("Updated".to_string()));
        assert!(block.updated_at >= original_updated_at);
    }

    #[test]
    fn test_block_system_add_and_get() {
        let mut system = BlockSystem::new();
        let content = "Test block".to_string();
        let block = Block::new_text(content.clone());
        let id = block.id;

        system.add_block(block);

        assert_eq!(system.count(), 1);
        assert!(system.contains(id));

        let retrieved = system.get_block(id).unwrap();
        assert_eq!(retrieved.get_text(), Some(content));
    }

    #[test]
    fn test_block_system_remove() {
        let mut system = BlockSystem::new();
        let block = Block::new_text("Test".to_string());
        let id = block.id;

        system.add_block(block);
        assert_eq!(system.count(), 1);

        system.remove_block(id);
        assert_eq!(system.count(), 0);
        assert!(!system.contains(id));
    }

    #[test]
    fn test_block_system_find_by_tag() {
        let mut system = BlockSystem::new();

        let mut block1 = Block::new_text("Block 1".to_string());
        block1.add_tag("tag1".to_string());

        let mut block2 = Block::new_text("Block 2".to_string());
        block2.add_tag("tag2".to_string());

        let mut block3 = Block::new_text("Block 3".to_string());
        block3.add_tag("tag1".to_string());

        system.add_block(block1);
        system.add_block(block2);
        system.add_block(block3);

        let tagged_blocks = system.find_by_tag("tag1");
        assert_eq!(tagged_blocks.len(), 2);

        let tagged_blocks2 = system.find_by_tag("tag2");
        assert_eq!(tagged_blocks2.len(), 1);
    }

    #[test]
    fn test_block_system_find_by_type() {
        let mut system = BlockSystem::new();

        let block1 = Block::new_text("Text block".to_string());
        let block2 = Block::new_heading(1, "Heading block".to_string());
        let block3 = Block::new_code("code".to_string(), None);

        system.add_block(block1);
        system.add_block(block2);
        system.add_block(block3);

        let text_blocks = system.find_by_type(&BlockType::Text);
        assert_eq!(text_blocks.len(), 1);

        let heading_blocks = system.find_by_type(&BlockType::Heading(1));
        assert_eq!(heading_blocks.len(), 1);
    }

    #[test]
    fn test_block_system_list_blocks() {
        let mut system = BlockSystem::new();

        system.add_block(Block::new_text("Block 1".to_string()));
        system.add_block(Block::new_text("Block 2".to_string()));
        system.add_block(Block::new_text("Block 3".to_string()));

        let blocks = system.list_blocks();
        assert_eq!(blocks.len(), 3);
    }
}
