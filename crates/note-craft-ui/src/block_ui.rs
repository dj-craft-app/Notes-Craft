//! Block UI Components Module
//!
//! This module provides UI components for displaying and editing blocks in Note Craft.

use note_craft_core::{Block, BlockId, BlockType};
use std::path::PathBuf;
use std::sync::Arc;

/// Represents the visual state of a block in the UI
#[derive(Debug, Clone, PartialEq)]
pub struct BlockUiState {
    /// The block ID
    pub block_id: BlockId,
    /// Whether the block is currently selected
    pub is_selected: bool,
    /// Whether the block is being edited
    pub is_editing: bool,
    /// Whether the block is expanded (for containers)
    pub is_expanded: bool,
    /// Whether the block is hovered
    pub is_hovered: bool,
    /// Whether the block has focus
    pub has_focus: bool,
}

impl BlockUiState {
    /// Creates a new UI state for a block
    pub fn new(block_id: BlockId) -> Self {
        Self {
            block_id,
            is_selected: false,
            is_editing: false,
            is_expanded: true,
            is_hovered: false,
            has_focus: false,
        }
    }

    /// Creates a new UI state for a block from u64
    pub fn new_from_u64(id: u64) -> Self {
        Self::new(BlockId(id))
    }

    /// Sets the selected state
    pub fn selected(mut self, selected: bool) -> Self {
        self.is_selected = selected;
        self
    }

    /// Sets the editing state
    pub fn editing_state(mut self, editing: bool) -> Self {
        self.is_editing = editing;
        self
    }
}

/// Visual style configuration for blocks
#[derive(Debug, Clone)]
pub struct BlockStyleConfig {
    /// Background color for normal blocks
    pub background_color: String,
    /// Background color for selected blocks
    pub selected_background_color: String,
    /// Text color
    pub text_color: String,
    /// Text color for selected blocks
    pub selected_text_color: String,
    /// Font size
    pub font_size: u32,
    /// Font family
    pub font_family: String,
    /// Indent for nested blocks
    pub indent_size: u32,
    /// Padding
    pub padding: u32,
    /// Margin between blocks
    pub margin: u32,
}

impl Default for BlockStyleConfig {
    fn default() -> Self {
        Self {
            background_color: "#282828".to_string(),
            selected_background_color: "#3f3f3f".to_string(),
            text_color: "#dadada".to_string(),
            selected_text_color: "#ffffff".to_string(),
            font_size: 14,
            font_family: "Inter, -apple-system, sans-serif".to_string(),
            indent_size: 20,
            padding: 8,
            margin: 8,
        }
    }
}

impl BlockStyleConfig {
    /// Creates a dark theme configuration
    pub fn dark_theme() -> Self {
        Self::default()
    }

    /// Creates a light theme configuration
    pub fn light_theme() -> Self {
        Self {
            background_color: "#ffffff".to_string(),
            selected_background_color: "#e8f0fe".to_string(),
            text_color: "#333333".to_string(),
            selected_text_color: "#000000".to_string(),
            font_size: 14,
            font_family: "Inter, -apple-system, sans-serif".to_string(),
            indent_size: 20,
            padding: 8,
            margin: 8,
        }
    }

    /// Sets the background color
    pub fn with_background_color(mut self, color: impl Into<String>) -> Self {
        self.background_color = color.into();
        self
    }
}

/// Event types for block UI interactions
#[derive(Debug, Clone, PartialEq)]
pub enum BlockUiEvent {
    /// Block was selected
    BlockSelected(BlockId),
    /// Block was deselected
    BlockDeselected(BlockId),
    /// Block editing started
    EditStarted(BlockId),
    /// Block editing ended
    EditEnded(BlockId),
    /// Block content changed
    ContentChanged(BlockId, String),
    /// Navigation to block
    NavigateToBlock(BlockId),
    /// Focus next block
    FocusNext,
    /// Focus previous block
    FocusPrevious,
}

impl BlockUiEvent {
    /// Creates a BlockSelected event from u64
    pub fn select(id: u64) -> Self {
        BlockUiEvent::BlockSelected(BlockId(id))
    }

    /// Creates a BlockDeselected event from u64
    pub fn deselect(id: u64) -> Self {
        BlockUiEvent::BlockDeselected(BlockId(id))
    }

    /// Creates an EditStarted event from u64
    pub fn edit_start(id: u64) -> Self {
        BlockUiEvent::EditStarted(BlockId(id))
    }

    /// Creates an EditEnded event from u64
    pub fn edit_end(id: u64) -> Self {
        BlockUiEvent::EditEnded(BlockId(id))
    }
}

/// Block UI manager
pub struct BlockUiManager {
    /// Map of blocks to their UI states
    block_states: std::collections::HashMap<BlockId, BlockUiState>,
    /// Style configuration
    style: BlockStyleConfig,
    /// Currently selected block
    selected_block: Option<BlockId>,
    /// Currently edited block
    edited_block: Option<BlockId>,
}

impl Default for BlockUiManager {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl BlockUiManager {
    /// Creates a new block UI manager
    pub fn new(style: BlockStyleConfig) -> Self {
        Self {
            block_states: std::collections::HashMap::new(),
            style,
            selected_block: None,
            edited_block: None,
        }
    }

    /// Creates with default configuration
    pub fn with_defaults() -> Self {
        Self::new(BlockStyleConfig::default())
    }

    /// Returns the style configuration
    pub fn style(&self) -> &BlockStyleConfig {
        &self.style
    }

    /// Sets the style configuration
    pub fn set_style(&mut self, style: BlockStyleConfig) {
        self.style = style;
    }

    /// Selects a block
    pub fn select_block(&mut self, block_id: BlockId) {
        // Deselect current
        if let Some(current) = self.selected_block.take() {
            if let Some(state) = self.block_states.get_mut(&current) {
                state.is_selected = false;
            }
        }

        // Select new
        self.selected_block = Some(block_id);
        self.block_states
            .entry(block_id)
            .or_insert_with(|| BlockUiState::new(block_id))
            .is_selected = true;
    }

    /// Deselects the current block
    pub fn deselect_block(&mut self) {
        if let Some(current) = self.selected_block.take() {
            if let Some(state) = self.block_states.get_mut(&current) {
                state.is_selected = false;
            }
        }
    }

    /// Starts editing a block
    pub fn start_editing(&mut self, block_id: BlockId) {
        self.select_block(block_id);

        self.edited_block = Some(block_id);
        self.block_states
            .entry(block_id)
            .or_insert_with(|| BlockUiState::new(block_id))
            .is_editing = true;
    }

    /// Ends editing a block
    pub fn end_editing(&mut self) {
        if let Some(block_id) = self.edited_block.take() {
            if let Some(state) = self.block_states.get_mut(&block_id) {
                state.is_editing = false;
            }
        }
    }

    /// Gets the UI state for a block
    pub fn get_ui_state(&self, block_id: &BlockId) -> Option<&BlockUiState> {
        self.block_states.get(block_id)
    }

    /// Handles a block UI event
    pub fn handle_event(&mut self, event: BlockUiEvent) {
        match event {
            BlockUiEvent::BlockSelected(block_id) => {
                self.select_block(block_id);
            }
            BlockUiEvent::BlockDeselected(block_id) => {
                if self.selected_block == Some(block_id) {
                    self.deselect_block();
                }
            }
            BlockUiEvent::EditStarted(block_id) => {
                self.start_editing(block_id);
            }
            BlockUiEvent::EditEnded(block_id) => {
                if self.edited_block == Some(block_id) {
                    self.end_editing();
                }
            }
            BlockUiEvent::ContentChanged(_, _) => {
                // Content change handled elsewhere
            }
            _ => {}
        }
    }

    /// Creates render info for a block
    pub fn create_render_info(
        &self,
        block: &Block,
        depth: usize,
        is_first: bool,
        is_last: bool,
    ) -> BlockRenderInfo {
        let ui_state = self
            .block_states
            .get(&block.id)
            .cloned()
            .unwrap_or_else(|| BlockUiState::new(block.id));

        BlockRenderInfo {
            block: block.clone(),
            ui_state,
            depth,
            is_first,
            is_last,
            style: self.style.clone(),
        }
    }

    /// Gets the background color for a block
    pub fn get_background_color(&self, block_id: &BlockId) -> String {
        if let Some(state) = self.block_states.get(block_id) {
            if state.is_selected {
                return self.style.selected_background_color.clone();
            }
        }
        self.style.background_color.clone()
    }

    /// Gets the text color for a block
    pub fn get_text_color(&self, block_id: &BlockId) -> String {
        if let Some(state) = self.block_states.get(block_id) {
            if state.is_selected {
                return self.style.selected_text_color.clone();
            }
        }
        self.style.text_color.clone()
    }
}

/// Information needed to render a block
#[derive(Debug, Clone)]
pub struct BlockRenderInfo {
    /// The block to render
    pub block: Block,
    /// UI state for the block
    pub ui_state: BlockUiState,
    /// Depth in the hierarchy
    pub depth: usize,
    /// Whether this is the first block
    pub is_first: bool,
    /// Whether this is the last block
    pub is_last: bool,
    /// Style configuration
    pub style: BlockStyleConfig,
}

impl BlockRenderInfo {
    /// Gets the indent based on depth
    pub fn get_indent(&self) -> u32 {
        (self.depth as u32) * self.style.indent_size
    }

    /// Gets the prefix for list blocks
    pub fn get_list_prefix(&self) -> Option<String> {
        match &self.block.block_type {
            BlockType::BulletList => Some("• ".to_string()),
            BlockType::NumberedList => Some(format!("{} ", self.depth + 1)),
            _ => None,
        }
    }

    /// Gets the CSS class name based on block type
    pub fn get_css_class(&self) -> String {
        match &self.block.block_type {
            BlockType::Text => "block-text".to_string(),
            BlockType::Heading(level) => format!("block-heading h{}", level),
            BlockType::BulletList => "block-bullet-list".to_string(),
            BlockType::NumberedList => "block-numbered-list".to_string(),
            BlockType::Code => "block-code".to_string(),
            BlockType::Quote => "block-quote".to_string(),
            BlockType::HorizontalRule => "block-hr".to_string(),
            BlockType::Link => "block-link".to_string(),
            BlockType::BlockReference(_) => "block-reference".to_string(),
            BlockType::PageReference => "block-page-reference".to_string(),
            BlockType::Tag => "block-tag".to_string(),
        }
    }

    /// Whether this block should have a border
    pub fn show_border(&self) -> bool {
        self.ui_state.is_selected
    }
}

/// Builder for creating block UI managers
pub struct BlockUiManagerBuilder {
    style: BlockStyleConfig,
}

impl Default for BlockUiManagerBuilder {
    fn default() -> Self {
        Self {
            style: BlockStyleConfig::default(),
        }
    }
}

impl BlockUiManagerBuilder {
    /// Creates a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Uses dark theme
    pub fn dark_theme(mut self) -> Self {
        self.style = BlockStyleConfig::dark_theme();
        self
    }

    /// Uses light theme
    pub fn light_theme(mut self) -> Self {
        self.style = BlockStyleConfig::light_theme();
        self
    }

    /// Builds the manager
    pub fn build(self) -> BlockUiManager {
        BlockUiManager::new(self.style)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_ui_state_creation() {
        let state = BlockUiState::new_from_u64(1);

        assert_eq!(state.block_id.0, 1);
        assert!(!state.is_selected);
        assert!(!state.is_editing);
    }

    #[test]
    fn test_block_style_config_dark_theme() {
        let config = BlockStyleConfig::dark_theme();

        assert_eq!(config.background_color, "#282828");
        assert_eq!(config.text_color, "#dadada");
    }

    #[test]
    fn test_block_style_config_light_theme() {
        let config = BlockStyleConfig::light_theme();

        assert_eq!(config.background_color, "#ffffff");
        assert_eq!(config.text_color, "#333333");
    }

    #[test]
    fn test_block_ui_manager_select_block() {
        let mut manager = BlockUiManager::with_defaults();
        let block_id = BlockId(1);

        manager.select_block(block_id);

        assert_eq!(manager.selected_block, Some(block_id));
        assert!(manager.get_ui_state(&block_id).unwrap().is_selected);
    }

    #[test]
    fn test_block_ui_manager_deselect() {
        let mut manager = BlockUiManager::with_defaults();
        let block_id = BlockId(1);

        manager.select_block(block_id);
        manager.deselect_block();

        assert!(manager.selected_block.is_none());
    }

    #[test]
    fn test_block_ui_manager_start_editing() {
        let mut manager = BlockUiManager::with_defaults();
        let block_id = BlockId(1);

        manager.start_editing(block_id);

        assert_eq!(manager.edited_block, Some(block_id));
        assert!(manager.get_ui_state(&block_id).unwrap().is_editing);
    }

    #[test]
    fn test_block_ui_manager_end_editing() {
        let mut manager = BlockUiManager::with_defaults();
        let block_id = BlockId(1);

        manager.start_editing(block_id);
        manager.end_editing();

        assert!(manager.edited_block.is_none());
        assert!(!manager.get_ui_state(&block_id).unwrap().is_editing);
    }

    #[test]
    fn test_block_ui_manager_handle_event() {
        let mut manager = BlockUiManager::with_defaults();
        let block_id = BlockId(1);

        manager.handle_event(BlockUiEvent::BlockSelected(block_id));

        assert_eq!(manager.selected_block, Some(block_id));
    }

    #[test]
    fn test_block_render_info_get_indent() {
        let block = Block::new_text("Hello".to_string());
        let manager = BlockUiManager::with_defaults();
        let info = manager.create_render_info(&block, 2, false, false);

        assert_eq!(info.get_indent(), 40); // 2 * 20
    }

    #[test]
    fn test_block_render_info_get_list_prefix() {
        let mut block = Block::new_text("Item".to_string());
        block.block_type = BlockType::BulletList;

        let manager = BlockUiManager::with_defaults();
        let info = manager.create_render_info(&block, 0, false, false);

        assert_eq!(info.get_list_prefix(), Some("• ".to_string()));
    }

    #[test]
    fn test_block_render_info_get_css_class() {
        let text_block = Block::new_text("Hello".to_string());
        let heading_block = Block::new_heading(1, "Title".to_string());
        let manager = BlockUiManager::with_defaults();

        let text_info = manager.create_render_info(&text_block, 0, false, false);
        let heading_info = manager.create_render_info(&heading_block, 0, false, false);

        assert_eq!(text_info.get_css_class(), "block-text");
        assert_eq!(heading_info.get_css_class(), "block-heading h1");
    }

    #[test]
    fn test_block_ui_manager_get_colors() {
        let mut manager = BlockUiManager::with_defaults();
        let block_id = BlockId(1);

        // Not selected
        assert_eq!(manager.get_background_color(&block_id), "#282828");
        assert_eq!(manager.get_text_color(&block_id), "#dadada");

        // Selected
        manager.select_block(block_id);
        assert_eq!(manager.get_background_color(&block_id), "#3f3f3f");
        assert_eq!(manager.get_text_color(&block_id), "#ffffff");
    }

    #[test]
    fn test_block_ui_manager_builder() {
        let manager = BlockUiManagerBuilder::new().dark_theme().build();

        assert_eq!(manager.style().background_color, "#282828");
    }

    #[test]
    fn test_block_render_info_show_border() {
        let block = Block::new_text("Hello".to_string());
        let manager = BlockUiManager::with_defaults();
        let mut info = manager.create_render_info(&block, 0, false, false);

        assert!(!info.show_border());

        info.ui_state.is_selected = true;
        assert!(info.show_border());
    }

    #[test]
    fn test_block_render_info_is_first_last() {
        let block = Block::new_text("Hello".to_string());
        let manager = BlockUiManager::with_defaults();
        let info = manager.create_render_info(&block, 0, true, false);

        assert!(info.is_first);
        assert!(!info.is_last);
    }

    #[test]
    fn test_block_style_config_with_background() {
        let config = BlockStyleConfig::default().with_background_color("#000000");

        assert_eq!(config.background_color, "#000000");
    }
}
