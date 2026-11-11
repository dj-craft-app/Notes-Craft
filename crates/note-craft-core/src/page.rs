//! Note Craft Page System
//!
//! Ce module fournit le système de pages, qui sont des conteneurs de blocs.
//!
//! Phase 1 - Étape 1.3: Page System

use crate::block::{Block, BlockId, BlockSystem};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Identifiant unique pour une page
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PageId(pub u64);

impl Default for PageId {
    fn default() -> Self {
        Self::new()
    }
}

impl PageId {
    /// Génère un nouvel identifiant
    pub fn new() -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        PageId(COUNTER.fetch_add(1, Ordering::SeqCst))
    }
}

/// Type de page
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PageType {
    /// Page normale
    Normal,
    /// Page journal (avec date)
    Journal,
    /// Page de documentation
    Documentation,
    /// Page de template
    Template,
}

/// Métadonnées d'une page
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageMetadata {
    /// Titre de la page
    pub title: String,
    /// Description de la page
    pub description: Option<String>,
    /// Type de la page
    pub page_type: PageType,
    /// Tags associés à la page
    pub tags: Vec<String>,
    /// Date de création (timestamp)
    pub created_at: u64,
    /// Date de modification (timestamp)
    pub updated_at: u64,
    /// Auteur de la page
    pub author: Option<String>,
}

impl PageMetadata {
    /// Crée de nouvelles métadonnées
    pub fn new(title: String) -> Self {
        let now = Self::current_timestamp();
        PageMetadata {
            title,
            description: None,
            page_type: PageType::Normal,
            tags: Vec::new(),
            created_at: now,
            updated_at: now,
            author: None,
        }
    }

    /// Met à jour le titre
    pub fn set_title(&mut self, title: String) {
        self.title = title;
        self.updated_at = Self::current_timestamp();
    }

    /// Met à jour la description
    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
        self.updated_at = Self::current_timestamp();
    }

    /// Ajoute un tag
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
            self.updated_at = Self::current_timestamp();
        }
    }

    /// Supprime un tag
    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.retain(|t| t != tag);
        self.updated_at = Self::current_timestamp();
    }

    /// Obtient le timestamp actuel
    fn current_timestamp() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
    }
}

/// Une page Note Craft
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    /// Identifiant unique de la page
    pub id: PageId,
    /// Métadonnées de la page
    pub metadata: PageMetadata,
    /// IDs des blocs appartenant à cette page
    pub block_ids: Vec<BlockId>,
    /// Page parente (pour les sous-pages)
    pub parent_id: Option<PageId>,
    /// Pages enfants
    pub children: Vec<PageId>,
    /// Position dans la hiérarchie
    pub position: usize,
}

impl Page {
    /// Crée une nouvelle page
    pub fn new(title: String) -> Self {
        Page {
            id: PageId::new(),
            metadata: PageMetadata::new(title),
            block_ids: Vec::new(),
            parent_id: None,
            children: Vec::new(),
            position: 0,
        }
    }

    /// Ajoute un bloc à la page
    pub fn add_block(&mut self, block_id: BlockId) {
        if !self.block_ids.contains(&block_id) {
            self.block_ids.push(block_id);
            self.metadata.updated_at = PageMetadata::current_timestamp();
        }
    }

    /// Supprime un bloc de la page
    pub fn remove_block(&mut self, block_id: &BlockId) {
        self.block_ids.retain(|&id| id != *block_id);
        self.metadata.updated_at = PageMetadata::current_timestamp();
    }

    /// Vérifie si la page contient un bloc
    pub fn contains_block(&self, block_id: BlockId) -> bool {
        self.block_ids.contains(&block_id)
    }

    /// Renomme la page
    pub fn rename(&mut self, new_title: String) {
        self.metadata.set_title(new_title);
    }

    /// Ajoute une page enfant
    pub fn add_child(&mut self, child_id: PageId) {
        if !self.children.contains(&child_id) {
            self.children.push(child_id);
            self.metadata.updated_at = PageMetadata::current_timestamp();
        }
    }

    /// Supprime une page enfant
    pub fn remove_child(&mut self, child_id: &PageId) {
        self.children.retain(|&id| id != *child_id);
        self.metadata.updated_at = PageMetadata::current_timestamp();
    }

    /// Vérifie si la page a des enfants
    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    /// Obtient le nombre de blocs
    pub fn block_count(&self) -> usize {
        self.block_ids.len()
    }
}

/// Système de gestion des pages
pub struct PageSystem {
    /// Map des pages par ID
    pages: HashMap<PageId, Page>,
    /// Système de blocs associé
    block_system: BlockSystem,
}

impl PageSystem {
    /// Crée un nouveau système de pages
    pub fn new() -> Self {
        PageSystem {
            pages: HashMap::new(),
            block_system: BlockSystem::new(),
        }
    }

    /// Crée une nouvelle page
    pub fn create_page(&mut self, title: String) -> PageId {
        let mut page = Page::new(title);
        page.position = self.pages.len();
        let id = page.id;
        self.pages.insert(id, page);
        id
    }

    /// Récupère une page par ID
    pub fn get_page(&self, id: PageId) -> Option<&Page> {
        self.pages.get(&id)
    }

    /// Récupère une page par ID (mutable)
    pub fn get_page_mut(&mut self, id: PageId) -> Option<&mut Page> {
        self.pages.get_mut(&id)
    }

    /// Supprime une page
    pub fn delete_page(&mut self, id: PageId) -> Option<Page> {
        self.pages.remove(&id)
    }

    /// Renomme une page
    pub fn rename_page(&mut self, id: PageId, new_title: String) -> bool {
        if let Some(page) = self.pages.get_mut(&id) {
            page.rename(new_title);
            true
        } else {
            false
        }
    }

    /// Ajoute un bloc à une page
    pub fn add_block_to_page(&mut self, page_id: PageId, block: Block) -> Option<BlockId> {
        // Ajouter le bloc au block system
        let block_id = self.block_system.add_block(block);

        // Ajouter le bloc à la page
        if let Some(page) = self.pages.get_mut(&page_id) {
            page.add_block(block_id);
            Some(block_id)
        } else {
            // Si la page n'existe pas, supprimer le bloc du block system
            self.block_system.remove_block(block_id);
            None
        }
    }

    /// Récupère les blocs d'une page
    pub fn get_page_blocks(&self, page_id: PageId) -> Vec<&Block> {
        if let Some(page) = self.pages.get(&page_id) {
            page.block_ids
                .iter()
                .filter_map(|&id| self.block_system.get_block(id))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Supprime un bloc d'une page
    pub fn remove_block_from_page(&mut self, page_id: PageId, block_id: BlockId) -> bool {
        if let Some(page) = self.pages.get_mut(&page_id) {
            page.remove_block(&block_id);
            true
        } else {
            false
        }
    }

    /// Liste toutes les pages
    pub fn list_pages(&self) -> Vec<&Page> {
        self.pages.values().collect()
    }

    /// Trouve les pages par tag
    pub fn find_pages_by_tag(&self, tag: &str) -> Vec<&Page> {
        self.pages
            .values()
            .filter(|page| page.metadata.tags.contains(&tag.to_string()))
            .collect()
    }

    /// Trouve les pages par type
    pub fn find_pages_by_type(&self, page_type: PageType) -> Vec<&Page> {
        self.pages
            .values()
            .filter(|page| page.metadata.page_type == page_type)
            .collect()
    }

    /// Compte le nombre de pages
    pub fn count(&self) -> usize {
        self.pages.len()
    }

    /// Vérifie si une page existe
    pub fn contains(&self, id: PageId) -> bool {
        self.pages.contains_key(&id)
    }

    /// Obtient l'accès au BlockSystem sous-jacent
    pub fn block_system(&self) -> &BlockSystem {
        &self.block_system
    }

    /// Obtient l'accès mutable au BlockSystem sous-jacent
    pub fn block_system_mut(&mut self) -> &mut BlockSystem {
        &mut self.block_system
    }
}

impl Default for PageSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_id_new() {
        let id1 = PageId::new();
        let id2 = PageId::new();
        assert_ne!(id1, id2, "Les IDs doivent être uniques");
    }

    #[test]
    fn test_page_new() {
        let title = "Ma Page".to_string();
        let page = Page::new(title.clone());

        assert_eq!(page.metadata.title, title);
        assert_eq!(page.block_ids.len(), 0);
        assert!(page.children.is_empty());
        assert!(page.parent_id.is_none());
        assert_eq!(page.position, 0);
    }

    #[test]
    fn test_page_add_block() {
        let mut page = Page::new("Test Page".to_string());
        let block_id = BlockId::new();

        page.add_block(block_id);

        assert_eq!(page.block_count(), 1);
        assert!(page.contains_block(block_id));
    }

    #[test]
    fn test_page_remove_block() {
        let mut page = Page::new("Test Page".to_string());
        let block_id = BlockId::new();

        page.add_block(block_id);
        page.remove_block(&block_id);

        assert_eq!(page.block_count(), 0);
        assert!(!page.contains_block(block_id));
    }

    #[test]
    fn test_page_rename() {
        let mut page = Page::new("Original".to_string());
        let original_updated_at = page.metadata.updated_at;

        // Attendre un peu
        std::thread::sleep(std::time::Duration::from_millis(10));

        page.rename("Renamed".to_string());

        assert_eq!(page.metadata.title, "Renamed");
        assert!(page.metadata.updated_at >= original_updated_at);
    }

    #[test]
    fn test_page_add_child() {
        let mut page = Page::new("Parent".to_string());
        let child_id = PageId::new();

        page.add_child(child_id);

        assert_eq!(page.children.len(), 1);
        assert!(page.has_children());
    }

    #[test]
    fn test_page_remove_child() {
        let mut page = Page::new("Parent".to_string());
        let child_id = PageId::new();

        page.add_child(child_id);
        page.remove_child(&child_id);

        assert_eq!(page.children.len(), 0);
        assert!(!page.has_children());
    }

    #[test]
    fn test_page_system_create_page() {
        let mut system = PageSystem::new();
        let title = "Test Page".to_string();

        let id = system.create_page(title.clone());

        assert_eq!(system.count(), 1);
        assert!(system.contains(id));

        let page = system.get_page(id).unwrap();
        assert_eq!(page.metadata.title, title);
    }

    #[test]
    fn test_page_system_rename_page() {
        let mut system = PageSystem::new();
        let id = system.create_page("Original".to_string());

        let result = system.rename_page(id, "Renamed".to_string());
        assert!(result);

        let page = system.get_page(id).unwrap();
        assert_eq!(page.metadata.title, "Renamed");
    }

    #[test]
    fn test_page_system_rename_nonexistent() {
        let mut system = PageSystem::new();
        let fake_id = PageId::new();

        let result = system.rename_page(fake_id, "Renamed".to_string());
        assert!(!result);
    }

    #[test]
    fn test_page_system_add_block_to_page() {
        let mut system = PageSystem::new();
        let page_id = system.create_page("Test Page".to_string());
        let block = Block::new_text("Test content".to_string());

        let result = system.add_block_to_page(page_id, block);
        assert!(result.is_some());

        let blocks = system.get_page_blocks(page_id);
        assert_eq!(blocks.len(), 1);
    }

    #[test]
    fn test_page_system_add_block_to_nonexistent_page() {
        let mut system = PageSystem::new();
        let fake_page_id = PageId::new();
        let block = Block::new_text("Test content".to_string());

        let result = system.add_block_to_page(fake_page_id, block);
        assert!(result.is_none());
    }

    #[test]
    fn test_page_system_get_page_blocks() {
        let mut system = PageSystem::new();
        let page_id = system.create_page("Test Page".to_string());

        let block1 = Block::new_text("Block 1".to_string());
        let block2 = Block::new_heading(1, "Block 2".to_string());

        system.add_block_to_page(page_id, block1);
        system.add_block_to_page(page_id, block2);

        let blocks = system.get_page_blocks(page_id);
        assert_eq!(blocks.len(), 2);
    }

    #[test]
    fn test_page_system_list_pages() {
        let mut system = PageSystem::new();

        system.create_page("Page 1".to_string());
        system.create_page("Page 2".to_string());
        system.create_page("Page 3".to_string());

        let pages = system.list_pages();
        assert_eq!(pages.len(), 3);
    }

    #[test]
    fn test_page_system_find_by_tag() {
        let mut system = PageSystem::new();

        let mut page1 = Page::new("Page 1".to_string());
        page1.metadata.add_tag("tag1".to_string());

        let mut page2 = Page::new("Page 2".to_string());
        page2.metadata.add_tag("tag2".to_string());

        let mut page3 = Page::new("Page 3".to_string());
        page3.metadata.add_tag("tag1".to_string());

        let id1 = page1.id;
        let id2 = page2.id;
        let id3 = page3.id;

        // Ajouter les pages directement (contournement pour le test)
        system.pages.insert(id1, page1);
        system.pages.insert(id2, page2);
        system.pages.insert(id3, page3);

        let tagged_pages = system.find_pages_by_tag("tag1");
        assert_eq!(tagged_pages.len(), 2);
    }

    #[test]
    fn test_page_system_delete_page() {
        let mut system = PageSystem::new();
        let id = system.create_page("Test Page".to_string());

        assert_eq!(system.count(), 1);

        system.delete_page(id);

        assert_eq!(system.count(), 0);
        assert!(!system.contains(id));
    }

    #[test]
    fn test_page_metadata_add_tag() {
        let mut metadata = PageMetadata::new("Test".to_string());

        metadata.add_tag("tag1".to_string());
        assert_eq!(metadata.tags.len(), 1);
        assert!(metadata.tags.contains(&"tag1".to_string()));

        // Ajouter le même tag ne doit pas le dupliquer
        metadata.add_tag("tag1".to_string());
        assert_eq!(metadata.tags.len(), 1);
    }

    #[test]
    fn test_page_metadata_remove_tag() {
        let mut metadata = PageMetadata::new("Test".to_string());
        metadata.add_tag("tag1".to_string());
        metadata.add_tag("tag2".to_string());

        metadata.remove_tag("tag1");
        assert_eq!(metadata.tags.len(), 1);
        assert!(!metadata.tags.contains(&"tag1".to_string()));
    }
}
