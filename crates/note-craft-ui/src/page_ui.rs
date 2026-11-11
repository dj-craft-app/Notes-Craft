//! Page UI Components Module
//!
//! This module provides UI components for displaying and managing pages in Note Craft.
//! Pages contain blocks and provide the main content structure.

use note_craft_core::{Block, BlockId, Page, PageId};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::block_ui::{BlockRenderInfo, BlockStyleConfig, BlockUiManager};
use crate::sidebar::{SidebarItem, SidebarSection};

/// Represents the visual state of a page in the UI
#[derive(Debug, Clone, PartialEq)]
pub struct PageUiState {
    /// The page ID
    pub page_id: PageId,
    /// Whether the page is currently active/visible
    pub is_active: bool,
    /// Whether the page is being edited
    pub is_editing: bool,
    /// Whether the page is in read-only mode
    pub is_read_only: bool,
    /// Current scroll position
    pub scroll_position: f64,
    /// Current zoom level
    pub zoom_level: f64,
    /// Whether the page is in preview mode
    pub is_preview: bool,
}

impl PageUiState {
    /// Creates a new UI state for a page
    pub fn new(page_id: PageId) -> Self {
        Self {
            page_id,
            is_active: false,
            is_editing: false,
            is_read_only: false,
            scroll_position: 0.0,
            zoom_level: 1.0,
            is_preview: false,
        }
    }

    /// Creates a new UI state from u64
    pub fn new_from_u64(id: u64) -> Self {
        Self::new(PageId(id))
    }

    /// Sets the active state
    pub fn active(mut self, active: bool) -> Self {
        self.is_active = active;
        self
    }

    /// Sets the editing state
    pub fn editing(mut self, editing: bool) -> Self {
        self.is_editing = editing;
        self
    }

    /// Sets the read-only state
    pub fn read_only(mut self, read_only: bool) -> Self {
        self.is_read_only = read_only;
        self
    }

    /// Sets the preview mode
    pub fn preview(mut self, preview: bool) -> Self {
        self.is_preview = preview;
        self
    }
}

/// Page layout mode
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PageLayoutMode {
    /// Single column layout
    #[default]
    SingleColumn,
    /// Two columns layout
    TwoColumns,
    /// Three columns layout
    ThreeColumns,
    /// Split view (editor + preview)
    SplitView,
    /// Full screen
    FullScreen,
}

/// Page view mode
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PageViewMode {
    /// Edit mode (editable)
    #[default]
    Edit,
    /// Preview mode (rendered)
    Preview,
    /// Live preview (updates as you type)
    LivePreview,
}

/// Page configuration
#[derive(Debug, Clone)]
pub struct PageConfig {
    /// Layout mode
    pub layout_mode: PageLayoutMode,
    /// View mode
    pub view_mode: PageViewMode,
    /// Block style configuration
    pub block_style: BlockStyleConfig,
    /// Page background color
    pub background_color: String,
    /// Page text color
    pub text_color: String,
    /// Font size
    pub font_size: u32,
    /// Font family
    pub font_family: String,
    /// Line height
    pub line_height: f64,
    /// Whether to show line numbers
    pub show_line_numbers: bool,
    /// Whether to wrap text
    pub word_wrap: bool,
}

impl Default for PageConfig {
    fn default() -> Self {
        Self {
            layout_mode: PageLayoutMode::default(),
            view_mode: PageViewMode::default(),
            block_style: BlockStyleConfig::default(),
            background_color: "#1e1e1e".to_string(),
            text_color: "#dadada".to_string(),
            font_size: 14,
            font_family: "Inter, -apple-system, sans-serif".to_string(),
            line_height: 1.5,
            show_line_numbers: false,
            word_wrap: true,
        }
    }
}

impl PageConfig {
    /// Creates a dark theme configuration
    pub fn dark_theme() -> Self {
        Self::default()
    }

    /// Creates a light theme configuration
    pub fn light_theme() -> Self {
        Self {
            layout_mode: PageLayoutMode::default(),
            view_mode: PageViewMode::default(),
            block_style: BlockStyleConfig::light_theme(),
            background_color: "#ffffff".to_string(),
            text_color: "#333333".to_string(),
            font_size: 14,
            font_family: "Inter, -apple-system, sans-serif".to_string(),
            line_height: 1.5,
            show_line_numbers: false,
            word_wrap: true,
        }
    }

    /// Sets the layout mode
    pub fn with_layout_mode(mut self, mode: PageLayoutMode) -> Self {
        self.layout_mode = mode;
        self
    }

    /// Sets the view mode
    pub fn with_view_mode(mut self, mode: PageViewMode) -> Self {
        self.view_mode = mode;
        self
    }

    /// Sets the background color
    pub fn with_background_color(mut self, color: impl Into<String>) -> Self {
        self.background_color = color.into();
        self
    }
}

/// Event types for page UI interactions
#[derive(Debug, Clone, PartialEq)]
pub enum PageUiEvent {
    /// Page was loaded
    PageLoaded(PageId),
    /// Page was activated
    PageActivated(PageId),
    /// Page was deactivated
    PageDeactivated(PageId),
    /// Page content changed
    ContentChanged(PageId),
    /// Page title changed
    TitleChanged(PageId, String),
    /// Page was created
    PageCreated(PageId),
    /// Page was deleted
    PageDeleted(PageId),
    /// Navigation to page
    NavigateToPage(PageId),
    /// Layout mode changed
    LayoutModeChanged(PageLayoutMode),
    /// View mode changed
    ViewModeChanged(PageViewMode),
    /// Switch to edit mode
    SwitchToEdit,
    /// Switch to preview mode
    SwitchToPreview,
    /// Toggle preview
    TogglePreview,
    /// Zoom in
    ZoomIn,
    /// Zoom out
    ZoomOut,
    /// Reset zoom
    ResetZoom,
}

/// Page render information
#[derive(Debug, Clone)]
pub struct PageRenderInfo {
    /// The page to render
    pub page: Page,
    /// UI state for the page
    pub ui_state: PageUiState,
    /// Configuration
    pub config: PageConfig,
    /// List of blocks to render
    pub blocks: Vec<Block>,
    /// Map of block render infos
    pub block_infos: HashMap<BlockId, BlockRenderInfo>,
    /// Current sidebar section
    pub sidebar_section: SidebarSection,
}

impl PageRenderInfo {
    /// Creates a new render info
    pub fn new(page: Page, config: PageConfig) -> Self {
        let page_id = page.id;
        Self {
            page,
            ui_state: PageUiState::new(page_id),
            config,
            blocks: Vec::new(),
            block_infos: HashMap::new(),
            sidebar_section: SidebarSection::Explorer,
        }
    }

    /// Sets the UI state
    pub fn with_ui_state(mut self, state: PageUiState) -> Self {
        self.ui_state = state;
        self
    }

    /// Sets the blocks
    pub fn with_blocks(mut self, blocks: Vec<Block>) -> Self {
        self.blocks = blocks;
        self
    }

    /// Sets the sidebar section
    pub fn with_sidebar_section(mut self, section: SidebarSection) -> Self {
        self.sidebar_section = section;
        self
    }

    /// Gets the page title
    pub fn get_title(&self) -> String {
        self.page.metadata.title.clone()
    }

    /// Whether the page is active
    pub fn is_active(&self) -> bool {
        self.ui_state.is_active
    }

    /// Whether the page is in edit mode
    pub fn is_editing(&self) -> bool {
        self.ui_state.is_editing
    }

    /// Whether the page is in preview mode
    pub fn is_preview(&self) -> bool {
        self.ui_state.is_preview
    }
}

/// Page UI manager
pub struct PageUiManager {
    /// Map of pages to their UI states
    page_states: HashMap<PageId, PageUiState>,
    /// Configuration
    config: PageConfig,
    /// Block UI manager
    block_ui_manager: BlockUiManager,
    /// Currently active page
    active_page: Option<PageId>,
    /// Page history for navigation
    page_history: Vec<PageId>,
    /// Event handler
    event_handler: Option<Box<dyn Fn(PageUiEvent) + Send + Sync>>,
}

impl PageUiManager {
    /// Creates a new page UI manager
    pub fn new(config: PageConfig) -> Self {
        let block_style = config.block_style.clone();
        Self {
            page_states: HashMap::new(),
            config,
            block_ui_manager: BlockUiManager::new(block_style),
            active_page: None,
            page_history: Vec::new(),
            event_handler: None,
        }
    }

    /// Creates with default configuration
    pub fn with_defaults() -> Self {
        Self::new(PageConfig::default())
    }

    /// Sets the event handler
    pub fn set_event_handler<F>(&mut self, handler: F)
    where
        F: Fn(PageUiEvent) + Send + Sync + 'static,
    {
        self.event_handler = Some(Box::new(handler));
    }

    /// Returns the configuration
    pub fn config(&self) -> &PageConfig {
        &self.config
    }

    /// Sets the configuration
    pub fn set_config(&mut self, config: PageConfig) {
        self.config = config.clone();
        self.block_ui_manager.set_style(config.block_style);
    }

    /// Returns the block UI manager
    pub fn block_ui_manager(&self) -> &BlockUiManager {
        &self.block_ui_manager
    }

    /// Returns mutable reference to block UI manager
    pub fn block_ui_manager_mut(&mut self) -> &mut BlockUiManager {
        &mut self.block_ui_manager
    }

    /// Activates a page
    pub fn activate_page(&mut self, page_id: PageId) {
        // Deactivate current page
        if let Some(current) = self.active_page.take() {
            if let Some(state) = self.page_states.get_mut(&current) {
                state.is_active = false;
            }
            if let Some(handler) = &self.event_handler {
                handler(PageUiEvent::PageDeactivated(current));
            }
        }

        // Activate new page
        self.active_page = Some(page_id);
        self.page_states
            .entry(page_id)
            .or_insert_with(|| PageUiState::new(page_id))
            .is_active = true;

        // Add to history
        self.page_history.push(page_id);

        if let Some(handler) = &self.event_handler {
            handler(PageUiEvent::PageActivated(page_id));
        }
    }

    /// Deactivates the current page
    pub fn deactivate_page(&mut self) {
        if let Some(current) = self.active_page.take() {
            if let Some(state) = self.page_states.get_mut(&current) {
                state.is_active = false;
            }
            if let Some(handler) = &self.event_handler {
                handler(PageUiEvent::PageDeactivated(current));
            }
        }
    }

    /// Navigates to a page
    pub fn navigate_to(&mut self, page_id: PageId) {
        self.activate_page(page_id);
        if let Some(handler) = &self.event_handler {
            handler(PageUiEvent::NavigateToPage(page_id));
        }
    }

    /// Navigates back in history
    pub fn navigate_back(&mut self) -> Option<PageId> {
        if self.page_history.len() > 1 {
            self.page_history.pop();
            if let Some(&prev_page) = self.page_history.last() {
                self.activate_page(prev_page);
                return Some(prev_page);
            }
        }
        None
    }

    /// Navigates forward in history
    pub fn navigate_forward(&mut self) -> Option<PageId> {
        // For now, just reactivate the current page
        if let Some(current) = self.active_page {
            self.activate_page(current);
            return Some(current);
        }
        None
    }

    /// Creates render info for a page
    pub fn create_render_info(&self, page: Page, blocks: Vec<Block>) -> PageRenderInfo {
        let page_id = page.id;
        let ui_state = self
            .page_states
            .get(&page_id)
            .cloned()
            .unwrap_or_else(|| PageUiState::new(page_id));

        let mut render_info =
            PageRenderInfo::new(page, self.config.clone()).with_ui_state(ui_state);

        // Create block render infos
        for (index, block) in blocks.iter().enumerate() {
            let is_first = index == 0;
            let is_last = index == blocks.len() - 1;
            let block_info = self.block_ui_manager.create_render_info(
                block, 0, // depth
                is_first, is_last,
            );
            render_info.block_infos.insert(block.id, block_info);
        }

        render_info
    }

    /// Handles a page UI event
    pub fn handle_event(&mut self, event: PageUiEvent) {
        match event {
            PageUiEvent::PageActivated(page_id) => {
                self.activate_page(page_id);
            }
            PageUiEvent::PageDeactivated(page_id) => {
                if self.active_page == Some(page_id) {
                    self.deactivate_page();
                }
            }
            PageUiEvent::NavigateToPage(page_id) => {
                self.navigate_to(page_id);
            }
            PageUiEvent::SwitchToEdit => {
                if let Some(page_id) = self.active_page {
                    if let Some(state) = self.page_states.get_mut(&page_id) {
                        state.is_editing = true;
                        state.is_preview = false;
                    }
                    if let Some(handler) = &self.event_handler {
                        handler(PageUiEvent::ViewModeChanged(PageViewMode::Edit));
                    }
                }
            }
            PageUiEvent::SwitchToPreview => {
                if let Some(page_id) = self.active_page {
                    if let Some(state) = self.page_states.get_mut(&page_id) {
                        state.is_editing = false;
                        state.is_preview = true;
                    }
                    if let Some(handler) = &self.event_handler {
                        handler(PageUiEvent::ViewModeChanged(PageViewMode::Preview));
                    }
                }
            }
            PageUiEvent::TogglePreview => {
                if let Some(page_id) = self.active_page {
                    if let Some(state) = self.page_states.get_mut(&page_id) {
                        state.is_preview = !state.is_preview;
                        state.is_editing = !state.is_preview;
                        if let Some(handler) = &self.event_handler {
                            let mode = if state.is_preview {
                                PageViewMode::Preview
                            } else {
                                PageViewMode::Edit
                            };
                            handler(PageUiEvent::ViewModeChanged(mode));
                        }
                    }
                }
            }
            PageUiEvent::LayoutModeChanged(mode) => {
                self.config.layout_mode = mode.clone();
                if let Some(handler) = &self.event_handler {
                    handler(PageUiEvent::LayoutModeChanged(mode));
                }
            }
            PageUiEvent::ViewModeChanged(mode) => {
                self.config.view_mode = mode.clone();
                if let Some(handler) = &self.event_handler {
                    handler(PageUiEvent::ViewModeChanged(mode));
                }
            }
            _ => {}
        }
    }

    /// Gets the active page ID
    pub fn get_active_page(&self) -> Option<PageId> {
        self.active_page
    }

    /// Gets the UI state for a page
    pub fn get_ui_state(&self, page_id: &PageId) -> Option<&PageUiState> {
        self.page_states.get(page_id)
    }

    /// Updates the UI state for a page
    pub fn update_ui_state<F>(&mut self, page_id: PageId, f: F)
    where
        F: FnOnce(&mut PageUiState),
    {
        if let Some(state) = self.page_states.get_mut(&page_id) {
            f(state);
        }
    }
}

/// Builder for creating page UI managers
#[derive(Default)]
pub struct PageUiManagerBuilder {
    config: PageConfig,
}

impl PageUiManagerBuilder {
    /// Creates a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Uses dark theme
    pub fn dark_theme(mut self) -> Self {
        self.config = PageConfig::dark_theme();
        self
    }

    /// Uses light theme
    pub fn light_theme(mut self) -> Self {
        self.config = PageConfig::light_theme();
        self
    }

    /// Sets the layout mode
    pub fn layout_mode(mut self, mode: PageLayoutMode) -> Self {
        self.config.layout_mode = mode;
        self
    }

    /// Sets the view mode
    pub fn view_mode(mut self, mode: PageViewMode) -> Self {
        self.config.view_mode = mode;
        self
    }

    /// Builds the manager
    pub fn build(self) -> PageUiManager {
        PageUiManager::new(self.config)
    }
}

/// Page sidebar item builder
pub struct PageSidebarItemBuilder {
    page: Page,
}

impl PageSidebarItemBuilder {
    /// Creates a new builder
    pub fn new(page: Page) -> Self {
        Self { page }
    }

    /// Builds a sidebar item for this page
    pub fn build(self) -> SidebarItem {
        SidebarItem::file(
            self.page.id.0.to_string(),
            self.page.metadata.title,
            PathBuf::new(),
        )
        .with_icon("file-text")
    }

    /// Builds with custom icon
    pub fn with_icon(self, icon: impl Into<String>) -> SidebarItem {
        self.build().with_icon(icon)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_ui_state_creation() {
        let state = PageUiState::new_from_u64(1);

        assert_eq!(state.page_id.0, 1);
        assert!(!state.is_active);
        assert!(!state.is_editing);
    }

    #[test]
    fn test_page_config_dark_theme() {
        let config = PageConfig::dark_theme();

        assert_eq!(config.background_color, "#1e1e1e");
        assert_eq!(config.text_color, "#dadada");
    }

    #[test]
    fn test_page_config_light_theme() {
        let config = PageConfig::light_theme();

        assert_eq!(config.background_color, "#ffffff");
        assert_eq!(config.text_color, "#333333");
    }

    #[test]
    fn test_page_layout_mode_default() {
        let mode = PageLayoutMode::default();
        assert!(matches!(mode, PageLayoutMode::SingleColumn));
    }

    #[test]
    fn test_page_view_mode_default() {
        let mode = PageViewMode::default();
        assert!(matches!(mode, PageViewMode::Edit));
    }

    #[test]
    fn test_page_ui_manager_creation() {
        let manager = PageUiManager::with_defaults();

        assert!(!manager.config().background_color.is_empty());
    }

    #[test]
    fn test_page_ui_manager_activate_page() {
        let mut manager = PageUiManager::with_defaults();
        let page_id = PageId(1);

        manager.activate_page(page_id);

        assert_eq!(manager.get_active_page(), Some(page_id));
        assert!(manager.get_ui_state(&page_id).unwrap().is_active);
    }

    #[test]
    fn test_page_ui_manager_deactivate() {
        let mut manager = PageUiManager::with_defaults();
        let page_id = PageId(1);

        manager.activate_page(page_id);
        manager.deactivate_page();

        assert!(manager.get_active_page().is_none());
    }

    #[test]
    fn test_page_ui_manager_navigate_to() {
        let mut manager = PageUiManager::with_defaults();
        let page_id = PageId(1);

        manager.navigate_to(page_id);

        assert_eq!(manager.get_active_page(), Some(page_id));
    }

    #[test]
    fn test_page_ui_manager_handle_event() {
        let mut manager = PageUiManager::with_defaults();
        let page_id = PageId(1);

        manager.handle_event(PageUiEvent::PageActivated(page_id));

        assert_eq!(manager.get_active_page(), Some(page_id));
    }

    #[test]
    fn test_page_render_info_creation() {
        let page = Page::new("Test Page".to_string());
        let config = PageConfig::default();

        let info = PageRenderInfo::new(page, config);

        assert_eq!(info.get_title(), "Test Page");
    }

    #[test]
    fn test_page_ui_manager_builder() {
        let manager = PageUiManagerBuilder::new().dark_theme().build();

        assert_eq!(manager.config().background_color, "#1e1e1e");
    }

    #[test]
    fn test_page_sidebar_item_builder() {
        let page = Page::new("Test Page".to_string());
        let builder = PageSidebarItemBuilder::new(page);
        let item = builder.build();

        assert_eq!(item.name, "Test Page");
    }

    #[test]
    fn test_page_ui_manager_get_active_page() {
        let mut manager = PageUiManager::with_defaults();

        assert!(manager.get_active_page().is_none());

        manager.activate_page(PageId(1));
        assert_eq!(manager.get_active_page(), Some(PageId(1)));
    }

    #[test]
    fn test_page_ui_state_active() {
        let state = PageUiState::new_from_u64(1).active(true);

        assert!(state.is_active);
    }

    #[test]
    fn test_page_ui_state_editing() {
        let state = PageUiState::new_from_u64(1).editing(true);

        assert!(state.is_editing);
    }

    #[test]
    fn test_page_ui_state_read_only() {
        let state = PageUiState::new_from_u64(1).read_only(true);

        assert!(state.is_read_only);
    }

    #[test]
    fn test_page_config_with_layout() {
        let config = PageConfig::default().with_layout_mode(PageLayoutMode::TwoColumns);

        assert!(matches!(config.layout_mode, PageLayoutMode::TwoColumns));
    }

    #[test]
    fn test_page_config_with_view_mode() {
        let config = PageConfig::default().with_view_mode(PageViewMode::Preview);

        assert!(matches!(config.view_mode, PageViewMode::Preview));
    }

    #[test]
    fn test_page_render_info_is_active() {
        let page = Page::new("Test".to_string());
        let config = PageConfig::default();
        let mut info = PageRenderInfo::new(page, config);

        assert!(!info.is_active());

        info.ui_state.is_active = true;
        assert!(info.is_active());
    }

    #[test]
    fn test_page_render_info_is_editing() {
        let page = Page::new("Test".to_string());
        let config = PageConfig::default();
        let mut info = PageRenderInfo::new(page, config);

        assert!(!info.is_editing());

        info.ui_state.is_editing = true;
        assert!(info.is_editing());
    }
}
