//! Drag and Drop Module
//!
//! Ce module fournit le support pour le drag and drop avec:
//! - Réorganisation de blocs
//! - Réorganisation de pages
//! - Réorganisation d'assets
//! - Feedback visuel pendant le drag
//! - Génération de JavaScript pour le drag and drop HTML5

use note_craft_core::{BlockId, PageId};
use std::collections::HashMap;

/// Type d'élément qui peut être draggé
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DraggableType {
    /// Bloc (note, paragraphe, code, etc.)
    Block,
    /// Page
    Page,
    /// Asset (image, fichier)
    Asset,
}

/// Position d'insertion
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InsertPosition {
    /// Avant l'élément cible
    Before,
    /// Après l'élément cible
    After,
    /// À l'intérieur (pour les blocs enfants)
    Inside,
}

/// Événement de drag and drop
#[derive(Debug, Clone)]
pub struct DragDropEvent {
    /// Type de l'élément draggé
    pub draggable_type: DraggableType,
    /// ID de l'élément draggé
    pub dragged_id: String,
    /// ID de l'élément cible
    pub target_id: Option<String>,
    /// Position d'insertion
    pub position: InsertPosition,
    /// Coordonnées X
    pub x: f64,
    /// Coordonnées Y
    pub y: f64,
}

/// État du drag and drop
#[derive(Debug, Clone)]
pub struct DragDropState {
    /// Si un drag est en cours
    pub is_dragging: bool,
    /// ID de l'élément draggé
    pub dragged_id: Option<String>,
    /// Type de l'élément draggé
    pub dragged_type: Option<DraggableType>,
    /// ID de l'élément cible actuel
    pub target_id: Option<String>,
    /// Position d'insertion actuelle
    pub position: Option<InsertPosition>,
    /// Offset X du drag
    pub offset_x: f64,
    /// Offset Y du drag
    pub offset_y: f64,
}

impl Default for DragDropState {
    fn default() -> Self {
        Self {
            is_dragging: false,
            dragged_id: None,
            dragged_type: None,
            target_id: None,
            position: None,
            offset_x: 0.0,
            offset_y: 0.0,
        }
    }
}

/// Gestionnaire du drag and drop
pub struct DragDropManager {
    /// État actuel
    state: DragDropState,
    /// Callbacks pour les différents types
    callbacks: HashMap<DraggableType, Box<dyn DragDropCallback>>,
}

/// Callback pour le drag and drop
pub trait DragDropCallback: Send + Sync {
    /// Appelé quand un élément est draggé
    fn on_drag_start(&self, id: &str);
    /// Appelé quand un élément est déplacé au-dessus d'une cible
    fn on_drag_over(&self, dragged_id: &str, target_id: Option<&str>, position: InsertPosition);
    /// Appelé quand un élément est lâché
    fn on_drop(&self, dragged_id: &str, target_id: Option<&str>, position: InsertPosition) -> bool;
    /// Appelé quand le drag est annulé
    fn on_drag_end(&self, dragged_id: &str);
}

impl DragDropManager {
    /// Crée un nouveau gestionnaire
    pub fn new() -> Self {
        Self {
            state: DragDropState::default(),
            callbacks: HashMap::new(),
        }
    }

    /// Enregistre un callback pour un type d'élément
    pub fn register_callback(
        &mut self,
        draggable_type: DraggableType,
        callback: Box<dyn DragDropCallback>,
    ) {
        self.callbacks.insert(draggable_type, callback);
    }

    /// Démarre un drag
    pub fn start_drag(&mut self, draggable_type: DraggableType, id: String, x: f64, y: f64) {
        self.state.is_dragging = true;
        self.state.dragged_id = Some(id.clone());
        self.state.dragged_type = Some(draggable_type);
        self.state.offset_x = x;
        self.state.offset_y = y;

        if let Some(callback) = self.callbacks.get(&draggable_type) {
            callback.on_drag_start(&id);
        }
    }

    /// Met à jour la position du drag
    pub fn update_drag(
        &mut self,
        target_id: Option<String>,
        position: InsertPosition,
        _x: f64,
        _y: f64,
    ) {
        self.state.target_id = target_id.clone();
        self.state.position = Some(position);

        if let (Some(dragged_id), Some(dragged_type)) =
            (&self.state.dragged_id, &self.state.dragged_type)
        {
            if let Some(callback) = self.callbacks.get(dragged_type) {
                callback.on_drag_over(dragged_id, target_id.as_deref(), position);
            }
        }
    }

    /// Termine un drag (drop ou annulation)
    pub fn end_drag(&mut self, _x: f64, _y: f64) -> bool {
        let result = if let (Some(dragged_id), Some(dragged_type)) =
            (&self.state.dragged_id, &self.state.dragged_type)
        {
            if let Some(callback) = self.callbacks.get(dragged_type) {
                let target_id = self.state.target_id.clone();
                let position = self.state.position.unwrap_or(InsertPosition::After);
                callback.on_drop(dragged_id, target_id.as_deref(), position)
            } else {
                false
            }
        } else {
            false
        };

        if let (Some(dragged_id), Some(dragged_type)) =
            (&self.state.dragged_id, &self.state.dragged_type)
        {
            if let Some(callback) = self.callbacks.get(dragged_type) {
                callback.on_drag_end(dragged_id);
            }
        }

        self.state.is_dragging = false;
        self.state.dragged_id = None;
        self.state.dragged_type = None;
        self.state.target_id = None;
        self.state.position = None;

        result
    }

    /// Annule un drag
    pub fn cancel_drag(&mut self) {
        if let (Some(dragged_id), Some(dragged_type)) =
            (&self.state.dragged_id, &self.state.dragged_type)
        {
            if let Some(callback) = self.callbacks.get(dragged_type) {
                callback.on_drag_end(dragged_id);
            }
        }

        self.state.is_dragging = false;
        self.state.dragged_id = None;
        self.state.dragged_type = None;
        self.state.target_id = None;
        self.state.position = None;
    }

    /// Obtient l'état actuel
    pub fn state(&self) -> &DragDropState {
        &self.state
    }

    /// Vérifie si un drag est en cours
    pub fn is_dragging(&self) -> bool {
        self.state.is_dragging
    }
}

impl Default for DragDropManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Callback pour la réorganisation de blocs
pub struct BlockDragDropCallback {
    /// Fonction pour réorganiser les blocs
    pub on_reorder: Box<dyn Fn(BlockId, Option<BlockId>, InsertPosition) -> bool + Send + Sync>,
}

impl DragDropCallback for BlockDragDropCallback {
    fn on_drag_start(&self, _id: &str) {
        // Optionnel: feedback visuel
    }

    fn on_drag_over(&self, _dragged_id: &str, _target_id: Option<&str>, _position: InsertPosition) {
        // Optionnel: feedback visuel
    }

    fn on_drop(&self, dragged_id: &str, target_id: Option<&str>, position: InsertPosition) -> bool {
        if let (Ok(dragged), target) = (
            dragged_id.parse::<u64>(),
            target_id.and_then(|s| s.parse::<u64>().ok()),
        ) {
            (self.on_reorder)(BlockId(dragged), target.map(BlockId), position)
        } else {
            false
        }
    }

    fn on_drag_end(&self, _dragged_id: &str) {
        // Optionnel: nettoyage
    }
}

/// Callback pour la réorganisation de pages
pub struct PageDragDropCallback {
    /// Fonction pour réorganiser les pages
    pub on_reorder: Box<dyn Fn(PageId, Option<PageId>, InsertPosition) -> bool + Send + Sync>,
}

impl DragDropCallback for PageDragDropCallback {
    fn on_drag_start(&self, _id: &str) {
        // Optionnel: feedback visuel
    }

    fn on_drag_over(&self, _dragged_id: &str, _target_id: Option<&str>, _position: InsertPosition) {
        // Optionnel: feedback visuel
    }

    fn on_drop(&self, dragged_id: &str, target_id: Option<&str>, position: InsertPosition) -> bool {
        if let (Ok(dragged), target) = (
            dragged_id.parse::<u64>(),
            target_id.and_then(|s| s.parse::<u64>().ok()),
        ) {
            (self.on_reorder)(PageId(dragged), target.map(PageId), position)
        } else {
            false
        }
    }

    fn on_drag_end(&self, _dragged_id: &str) {
        // Optionnel: nettoyage
    }
}

/// Callback pour la réorganisation d'assets
pub struct AssetDragDropCallback {
    /// Fonction pour réorganiser les assets
    pub on_reorder: Box<dyn Fn(String, Option<String>, InsertPosition) -> bool + Send + Sync>,
}

impl DragDropCallback for AssetDragDropCallback {
    fn on_drag_start(&self, _id: &str) {
        // Optionnel: feedback visuel
    }

    fn on_drag_over(&self, _dragged_id: &str, _target_id: Option<&str>, _position: InsertPosition) {
        // Optionnel: feedback visuel
    }

    fn on_drop(&self, dragged_id: &str, target_id: Option<&str>, position: InsertPosition) -> bool {
        (self.on_reorder)(
            dragged_id.to_string(),
            target_id.map(|s| s.to_string()),
            position,
        )
    }

    fn on_drag_end(&self, _dragged_id: &str) {
        // Optionnel: nettoyage
    }
}

/// Génère le JavaScript pour le drag and drop
pub fn generate_drag_drop_js() -> String {
    r#"
// Drag and Drop Manager
class DragDropManager {
    constructor(options = {}) {
        this.draggableSelector = options.draggableSelector || '.draggable';
        this.dropZoneSelector = options.dropZoneSelector || '.drop-zone';
        this.onDragStart = options.onDragStart || (() => {});
        this.onDragOver = options.onDragOver || (() => {});
        this.onDrop = options.onDrop || (() => {});
        this.onDragEnd = options.onDragEnd || (() => {});
        
        this.draggedElement = null;
        this.dropZones = [];
        this.currentTarget = null;
        this.position = null;
        
        this.init();
    }

    init() {
        // Setup draggable elements
        document.querySelectorAll(this.draggableSelector).forEach(el => {
            el.draggable = true;
            el.addEventListener('dragstart', (e) => this.handleDragStart(e, el));
            el.addEventListener('dragend', (e) => this.handleDragEnd(e, el));
        });

        // Setup drop zones
        document.querySelectorAll(this.dropZoneSelector).forEach(zone => {
            this.dropZones.push(zone);
            zone.addEventListener('dragover', (e) => this.handleDragOver(e, zone));
            zone.addEventListener('dragleave', (e) => this.handleDragLeave(e, zone));
            zone.addEventListener('drop', (e) => this.handleDrop(e, zone));
        });
    }

    handleDragStart(e, element) {
        this.draggedElement = element;
        e.dataTransfer.setData('text/plain', element.dataset.id || element.id);
        e.dataTransfer.effectAllowed = 'move';
        
        // Add visual feedback
        element.classList.add('dragging');
        
        // Store offset
        const rect = element.getBoundingClientRect();
        this.offsetX = e.clientX - rect.left;
        this.offsetY = e.clientY - rect.top;
        
        this.onDragStart(element.dataset.id || element.id, element.dataset.type);
        e.stopPropagation();
    }

    handleDragOver(e, zone) {
        e.preventDefault();
        e.dataTransfer.dropEffect = 'move';
        
        const rect = zone.getBoundingClientRect();
        const midY = rect.top + rect.height / 2;
        
        let position = 'after';
        if (e.clientY < midY) {
            position = 'before';
        }
        
        this.currentTarget = zone;
        this.position = position;
        
        // Add visual feedback
        zone.classList.add('drag-over');
        zone.classList.add('drag-over-' + position);
        
        this.onDragOver(
            this.draggedElement.dataset.id || this.draggedElement.id,
            zone.dataset.id || zone.id,
            position
        );
        e.stopPropagation();
    }

    handleDragLeave(e, zone) {
        e.preventDefault();
        
        // Remove visual feedback
        zone.classList.remove('drag-over');
        zone.classList.remove('drag-over-before', 'drag-over-after', 'drag-over-inside');
        
        if (this.currentTarget === zone) {
            this.currentTarget = null;
            this.position = null;
        }
    }

    handleDrop(e, zone) {
        e.preventDefault();
        
        // Remove visual feedback
        zone.classList.remove('drag-over');
        zone.classList.remove('drag-over-before', 'drag-over-after', 'drag-over-inside');
        
        const draggedId = this.draggedElement.dataset.id || this.draggedElement.id;
        const targetId = zone.dataset.id || zone.id;
        const position = this.position || 'after';
        
        const success = this.onDrop(draggedId, targetId, position);
        
        this.draggedElement.classList.remove('dragging');
        this.draggedElement = null;
        this.currentTarget = null;
        this.position = null;
        
        e.stopPropagation();
        return success;
    }

    handleDragEnd(e, element) {
        e.preventDefault();
        
        // Remove visual feedback
        element.classList.remove('dragging');
        
        // Remove feedback from all drop zones
        document.querySelectorAll(this.dropZoneSelector).forEach(zone => {
            zone.classList.remove('drag-over');
            zone.classList.remove('drag-over-before', 'drag-over-after', 'drag-over-inside');
        });
        
        const draggedId = element.dataset.id || element.id;
        this.onDragEnd(draggedId);
        
        this.draggedElement = null;
        this.currentTarget = null;
        this.position = null;
        e.stopPropagation();
    }

    refresh() {
        // Re-initialize if DOM has changed
        this.dropZones = [];
        this.init();
    }
}

// CSS Classes for visual feedback
const dragDropStyles = `
.dragging {
    opacity: 0.5;
    border: 2px solid #2196F3;
    box-shadow: 0 4px 8px rgba(33, 150, 243, 0.3);
    z-index: 1000;
}

.drop-zone {
    min-height: 20px;
}

.drop-zone.drag-over {
    border-top: 2px dashed #2196F3;
}

.drop-zone.drag-over-before {
    border-top: 3px solid #2196F3;
    background: rgba(33, 150, 243, 0.05);
}

.drop-zone.drag-over-after {
    border-bottom: 3px solid #2196F3;
    background: rgba(33, 150, 243, 0.05);
}

.drop-zone.drag-over-inside {
    border: 2px dashed #2196F3;
    background: rgba(33, 150, 243, 0.1);
}
`;

// Inject styles
const styleElement = document.createElement('style');
styleElement.textContent = dragDropStyles;
document.head.appendChild(styleElement);

// Export for use in other modules
window.DragDropManager = DragDropManager;
"#
    .to_string()
}

/// Génère le HTML pour un élément dragable
pub fn generate_draggable_html(id: &str, draggable_type: DraggableType, content: &str) -> String {
    let type_str = match draggable_type {
        DraggableType::Block => "block",
        DraggableType::Page => "page",
        DraggableType::Asset => "asset",
    };

    format!(
        "<div class=\"draggable\" draggable=\"true\" data-id=\"{}\" data-type=\"{}\">{}</div>",
        id, type_str, content
    )
}

/// Génère le HTML pour une zone de drop
pub fn generate_drop_zone_html(id: &str, draggable_type: DraggableType, content: &str) -> String {
    let type_str = match draggable_type {
        DraggableType::Block => "block",
        DraggableType::Page => "page",
        DraggableType::Asset => "asset",
    };

    format!(
        "<div class=\"drop-zone\" data-id=\"{}\" data-type=\"{}\">{}</div>",
        id, type_str, content
    )
}

/// Génère le CSS pour le drag and drop
pub fn generate_drag_drop_css() -> String {
    r#"
/* Drag and Drop styling */
.draggable {
    cursor: move;
    user-select: none;
    transition: all 0.2s ease;
}

.dragging {
    opacity: 0.5;
    border: 2px solid #2196F3;
    box-shadow: 0 4px 8px rgba(33, 150, 243, 0.3);
    z-index: 1000;
}

.drop-zone {
    min-height: 20px;
    transition: all 0.2s ease;
}

.drop-zone.drag-over {
    border-top: 2px dashed #2196F3;
}

.drop-zone.drag-over-before {
    border-top: 3px solid #2196F3 !important;
    background: rgba(33, 150, 243, 0.05);
}

.drop-zone.drag-over-after {
    border-bottom: 3px solid #2196F3 !important;
    background: rgba(33, 150, 243, 0.05);
}

.drop-zone.drag-over-inside {
    border: 2px dashed #2196F3;
    background: rgba(33, 150, 243, 0.1);
}

/* Drag handle for smaller draggable areas */
.drag-handle {
    cursor: move;
    padding: 4px 8px;
    color: #666;
    display: inline-block;
}

.drag-handle:hover {
    color: #2196F3;
    background: rgba(33, 150, 243, 0.1);
}
"#
    .to_string()
}

/// Tests unitaires
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drag_drop_state_default() {
        let state = DragDropState::default();
        assert!(!state.is_dragging);
        assert!(state.dragged_id.is_none());
        assert!(state.dragged_type.is_none());
        assert!(state.target_id.is_none());
        assert!(state.position.is_none());
    }

    #[test]
    fn test_drag_drop_manager_new() {
        let manager = DragDropManager::new();
        assert!(!manager.is_dragging());
    }

    #[test]
    fn test_drag_drop_manager_start_drag() {
        let mut manager = DragDropManager::new();
        manager.start_drag(DraggableType::Block, "block-1".to_string(), 10.0, 20.0);

        assert!(manager.is_dragging());
        assert_eq!(manager.state().dragged_id, Some("block-1".to_string()));
        assert_eq!(manager.state().dragged_type, Some(DraggableType::Block));
    }

    #[test]
    fn test_drag_drop_manager_end_drag() {
        let mut manager = DragDropManager::new();
        manager.start_drag(DraggableType::Block, "block-1".to_string(), 10.0, 20.0);

        let result = manager.end_drag(100.0, 200.0);

        assert!(!manager.is_dragging());
        assert!(manager.state().dragged_id.is_none());
        assert!(!result); // Aucun callback enregistré
    }

    #[test]
    fn test_drag_drop_manager_cancel_drag() {
        let mut manager = DragDropManager::new();
        manager.start_drag(DraggableType::Block, "block-1".to_string(), 10.0, 20.0);

        manager.cancel_drag();

        assert!(!manager.is_dragging());
        assert!(manager.state().dragged_id.is_none());
    }

    #[test]
    fn test_drag_drop_manager_with_callback() {
        let mut manager = DragDropManager::new();

        let callback = BlockDragDropCallback {
            on_reorder: Box::new(|_dragged, _target, _position| {
                true // Toujours succes pour le test
            }),
        };

        manager.register_callback(DraggableType::Block, Box::new(callback));
        manager.start_drag(DraggableType::Block, "1".to_string(), 10.0, 20.0);
        manager.update_drag(Some("2".to_string()), InsertPosition::After, 100.0, 200.0);

        let result = manager.end_drag(100.0, 200.0);
        assert!(result);
    }

    #[test]
    fn test_insert_position() {
        assert_eq!(InsertPosition::Before, InsertPosition::Before);
        assert_eq!(InsertPosition::After, InsertPosition::After);
        assert_eq!(InsertPosition::Inside, InsertPosition::Inside);
    }

    #[test]
    fn test_draggable_type() {
        assert_eq!(DraggableType::Block, DraggableType::Block);
        assert_eq!(DraggableType::Page, DraggableType::Page);
        assert_eq!(DraggableType::Asset, DraggableType::Asset);
    }

    #[test]
    fn test_generate_draggable_html() {
        let html = generate_draggable_html("block-1", DraggableType::Block, "Content");
        assert!(html.contains("draggable=\"true\""));
        assert!(html.contains("block-1"));
        assert!(html.contains("block"));
        assert!(html.contains("Content"));
    }

    #[test]
    fn test_generate_drop_zone_html() {
        let html = generate_drop_zone_html("zone-1", DraggableType::Block, "Drop here");
        assert!(html.contains("drop-zone"));
        assert!(html.contains("zone-1"));
        assert!(html.contains("block"));
        assert!(html.contains("Drop here"));
    }

    #[test]
    fn test_generate_drag_drop_css() {
        let css = generate_drag_drop_css();
        assert!(css.contains(".draggable"));
        assert!(css.contains(".drag-over"));
        assert!(css.contains(".drag-over-before"));
        assert!(css.contains(".drag-over-after"));
    }

    #[test]
    fn test_generate_drag_drop_js() {
        let js = generate_drag_drop_js();
        assert!(js.contains("DragDropManager"));
        assert!(js.contains("dragstart"));
        assert!(js.contains("dragover"));
        assert!(js.contains("drop"));
        assert!(js.contains("dragend"));
    }
}
