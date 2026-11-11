//! Sidebar System Module
//!
//! This module provides the sidebar UI components for Note Craft.
//! The sidebar enables navigation between pages and files.

use std::path::PathBuf;
use std::sync::Arc;

/// Represents a navigation item in the sidebar
#[derive(Debug, Clone, PartialEq)]
pub struct SidebarItem {
    /// Unique identifier for the item
    pub id: String,
    /// Display name
    pub name: String,
    /// Icon name (for future icon support)
    pub icon: Option<String>,
    /// Path to the file or page
    pub path: PathBuf,
    /// Whether this item is a directory/folder
    pub is_directory: bool,
    /// Child items (for nested navigation)
    pub children: Vec<SidebarItem>,
    /// Whether this item is currently selected
    pub is_selected: bool,
    /// Whether this item is expanded (for directories)
    pub is_expanded: bool,
}

impl SidebarItem {
    /// Creates a new sidebar item
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        path: impl Into<PathBuf>,
        is_directory: bool,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            icon: None,
            path: path.into(),
            is_directory,
            children: Vec::new(),
            is_selected: false,
            is_expanded: false,
        }
    }

    /// Creates a new file item
    pub fn file(id: impl Into<String>, name: impl Into<String>, path: impl Into<PathBuf>) -> Self {
        Self::new(id, name, path, false)
    }

    /// Creates a new directory item
    pub fn directory(
        id: impl Into<String>,
        name: impl Into<String>,
        path: impl Into<PathBuf>,
    ) -> Self {
        Self::new(id, name, path, true)
    }

    /// Adds a child item
    pub fn with_child(mut self, child: SidebarItem) -> Self {
        self.children.push(child);
        self
    }

    /// Sets the icon
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Sets the selected state
    pub fn selected(mut self, selected: bool) -> Self {
        self.is_selected = selected;
        self
    }

    /// Sets the expanded state
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.is_expanded = expanded;
        self
    }
}

/// Navigation event types
#[derive(Debug, Clone, PartialEq)]
pub enum SidebarEvent {
    /// Item was selected
    ItemSelected(String),
    /// Directory was expanded
    DirectoryExpanded(String),
    /// Directory was collapsed
    DirectoryCollapsed(String),
    /// Navigation requested
    Navigate(PathBuf),
    /// New file requested
    NewFile(PathBuf),
    /// New directory requested
    NewDirectory(PathBuf),
    /// Delete item requested
    DeleteItem(String),
    /// Rename item requested
    RenameItem(String, String),
}

/// Sidebar configuration
#[derive(Debug, Clone)]
pub struct SidebarConfig {
    /// Width of the sidebar in pixels
    pub width: u32,
    /// Background color
    pub background_color: String,
    /// Text color
    pub text_color: String,
    /// Selected item background color
    pub selected_background_color: String,
    /// Selected item text color
    pub selected_text_color: String,
    /// Hover background color
    pub hover_background_color: String,
    /// Border color
    pub border_color: String,
    /// Font size
    pub font_size: u32,
    /// Icon size
    pub icon_size: u32,
    /// Whether to show icons
    pub show_icons: bool,
}

impl Default for SidebarConfig {
    fn default() -> Self {
        Self {
            width: 250,
            background_color: "#282828".to_string(),
            text_color: "#dadada".to_string(),
            selected_background_color: "#027aff".to_string(),
            selected_text_color: "#ffffff".to_string(),
            hover_background_color: "#3f3f3f".to_string(),
            border_color: "#3f3f3f".to_string(),
            font_size: 14,
            icon_size: 16,
            show_icons: true,
        }
    }
}

impl SidebarConfig {
    /// Creates a new sidebar configuration with dark theme (default)
    pub fn dark_theme() -> Self {
        Self::default()
    }

    /// Creates a new sidebar configuration with light theme
    pub fn light_theme() -> Self {
        Self {
            width: 250,
            background_color: "#ffffff".to_string(),
            text_color: "#333333".to_string(),
            selected_background_color: "#0066cc".to_string(),
            selected_text_color: "#ffffff".to_string(),
            hover_background_color: "#f0f0f0".to_string(),
            border_color: "#e0e0e0".to_string(),
            font_size: 14,
            icon_size: 16,
            show_icons: true,
        }
    }

    /// Sets the width
    pub fn with_width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    /// Sets the background color
    pub fn with_background_color(mut self, color: impl Into<String>) -> Self {
        self.background_color = color.into();
        self
    }
}

/// Sidebar section type
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SidebarSection {
    /// File explorer / navigation
    Explorer,
    /// Recent files
    Recent,
    /// Favorites / pinned items
    Favorites,
    /// Tags
    Tags,
    /// Search
    Search,
}

impl Default for SidebarSection {
    fn default() -> Self {
        SidebarSection::Explorer
    }
}

/// State of the sidebar - using Arc for thread-safe sharing
#[derive(Debug, Clone)]
pub struct SidebarState {
    /// List of items in the sidebar
    pub items: Arc<std::sync::Mutex<Vec<SidebarItem>>>,
    /// Currently selected item ID
    pub selected_item_id: Arc<std::sync::Mutex<Option<String>>>,
    /// Map of expanded directory IDs
    pub expanded_directories: Arc<std::sync::Mutex<std::collections::HashSet<String>>>,
    /// Search query
    pub search_query: Arc<std::sync::Mutex<String>>,
    /// Filter for the sidebar
    pub filter: Arc<std::sync::Mutex<Option<String>>>,
    /// Whether the sidebar is visible
    pub is_visible: Arc<std::sync::Mutex<bool>>,
    /// Current section
    pub current_section: Arc<std::sync::Mutex<SidebarSection>>,
}

impl Default for SidebarState {
    fn default() -> Self {
        Self::new()
    }
}

impl SidebarState {
    /// Creates a new sidebar state
    pub fn new() -> Self {
        Self {
            items: Arc::new(std::sync::Mutex::new(Vec::new())),
            selected_item_id: Arc::new(std::sync::Mutex::new(None)),
            expanded_directories: Arc::new(std::sync::Mutex::new(
                std::collections::HashSet::new(),
            )),
            search_query: Arc::new(std::sync::Mutex::new(String::new())),
            filter: Arc::new(std::sync::Mutex::new(None)),
            is_visible: Arc::new(std::sync::Mutex::new(true)),
            current_section: Arc::new(std::sync::Mutex::new(SidebarSection::default())),
        }
    }

    /// Creates a new sidebar state with initial items
    pub fn with_items(items: Vec<SidebarItem>) -> Self {
        Self {
            items: Arc::new(std::sync::Mutex::new(items)),
            selected_item_id: Arc::new(std::sync::Mutex::new(None)),
            expanded_directories: Arc::new(std::sync::Mutex::new(
                std::collections::HashSet::new(),
            )),
            search_query: Arc::new(std::sync::Mutex::new(String::new())),
            filter: Arc::new(std::sync::Mutex::new(None)),
            is_visible: Arc::new(std::sync::Mutex::new(true)),
            current_section: Arc::new(std::sync::Mutex::new(SidebarSection::default())),
        }
    }

    /// Selects an item by ID
    pub fn select_item(&self, item_id: impl Into<String>) {
        let item_id = item_id.into();
        let mut selected = self.selected_item_id.lock().unwrap();
        let mut items = self.items.lock().unwrap();
        
        // Deselect all
        Self::deselect_all(&mut items);
        
        // Select target
        *selected = Some(item_id.clone());
        Self::set_item_selected(&mut items, &item_id, true);
    }

    /// Deselects all items
    fn deselect_all(items: &mut Vec<SidebarItem>) {
        for item in items.iter_mut() {
            item.is_selected = false;
            Self::deselect_all(&mut item.children);
        }
    }

    /// Sets an item's selected state
    fn set_item_selected(items: &mut Vec<SidebarItem>, target_id: &str, selected: bool) {
        for item in items.iter_mut() {
            if item.id == target_id {
                item.is_selected = selected;
                return;
            }
            if !item.children.is_empty() {
                Self::set_item_selected(&mut item.children, target_id, selected);
            }
        }
    }

    /// Toggles directory expansion
    pub fn toggle_directory(&self, directory_id: impl Into<String>) {
        let directory_id = directory_id.into();
        let mut expanded = self.expanded_directories.lock().unwrap();
        
        if expanded.contains(&directory_id) {
            expanded.remove(&directory_id);
        } else {
            expanded.insert(directory_id);
        }
    }

    /// Expands all directories
    pub fn expand_all(&self) {
        let items = self.items.lock().unwrap();
        let mut expanded = self.expanded_directories.lock().unwrap();
        Self::collect_directory_ids(&items, &mut expanded);
    }

    /// Collects all directory IDs
    fn collect_directory_ids(items: &Vec<SidebarItem>, set: &mut std::collections::HashSet<String>) {
        for item in items {
            if item.is_directory {
                set.insert(item.id.clone());
            }
            Self::collect_directory_ids(&item.children, set);
        }
    }

    /// Collapses all directories
    pub fn collapse_all(&self) {
        let mut expanded = self.expanded_directories.lock().unwrap();
        expanded.clear();
    }

    /// Sets the visibility
    pub fn set_visible(&self, visible: bool) {
        let mut is_visible = self.is_visible.lock().unwrap();
        *is_visible = visible;
    }

    /// Toggles visibility
    pub fn toggle_visibility(&self) {
        let mut is_visible = self.is_visible.lock().unwrap();
        *is_visible = !*is_visible;
    }

    /// Sets the current section
    pub fn set_section(&self, section: SidebarSection) {
        let mut current = self.current_section.lock().unwrap();
        *current = section;
    }

    /// Gets the current section
    pub fn get_section(&self) -> SidebarSection {
        let current = self.current_section.lock().unwrap();
        current.clone()
    }

    /// Gets the visibility
    pub fn get_visibility(&self) -> bool {
        let is_visible = self.is_visible.lock().unwrap();
        *is_visible
    }

    /// Gets the selected item ID
    pub fn get_selected_item_id(&self) -> Option<String> {
        let selected = self.selected_item_id.lock().unwrap();
        selected.clone()
    }

    /// Gets the items
    pub fn get_items(&self) -> Vec<SidebarItem> {
        let items = self.items.lock().unwrap();
        items.clone()
    }

    /// Gets the expanded directories
    pub fn get_expanded_directories(&self) -> std::collections::HashSet<String> {
        let expanded = self.expanded_directories.lock().unwrap();
        expanded.clone()
    }
}

/// Sidebar navigation service
#[derive(Clone)]
pub struct SidebarService {
    /// Current state
    state: SidebarState,
    /// Configuration
    config: SidebarConfig,
}

impl SidebarService {
    /// Creates a new sidebar service
    pub fn new(config: SidebarConfig) -> Self {
        Self {
            state: SidebarState::new(),
            config,
        }
    }

    /// Creates a new sidebar service with default configuration
    pub fn with_defaults() -> Self {
        Self::new(SidebarConfig::default())
    }

    /// Returns the current state
    pub fn state(&self) -> &SidebarState {
        &self.state
    }

    /// Returns the current configuration
    pub fn config(&self) -> &SidebarConfig {
        &self.config
    }

    /// Returns mutable reference to state (not possible with Arc, so we return the state itself)
    pub fn state_mut(&mut self) -> &mut SidebarState {
        &mut self.state
    }

    /// Loads navigation items from a repository
    pub fn load_navigation_items(&mut self, _repository_path: &PathBuf) -> Vec<SidebarItem> {
        vec![]
    }

    /// Selects an item by path
    pub fn select_by_path(&self, path: &PathBuf) {
        let items = self.state.get_items();
        if let Some(item_id) = Self::find_item_id_by_path(&items, path) {
            self.state.select_item(item_id);
        }
    }

    /// Finds an item ID by path
    fn find_item_id_by_path(items: &Vec<SidebarItem>, path: &PathBuf) -> Option<String> {
        for item in items {
            if &item.path == path {
                return Some(item.id.clone());
            }
            if let Some(child_id) = Self::find_item_id_by_path(&item.children, path) {
                return Some(child_id);
            }
        }
        None
    }

    /// Handles a sidebar event
    pub fn handle_event(&self, event: SidebarEvent) {
        match event {
            SidebarEvent::ItemSelected(item_id) => {
                self.state.select_item(item_id);
            }
            SidebarEvent::DirectoryExpanded(dir_id) => {
                self.state.toggle_directory(dir_id);
            }
            SidebarEvent::DirectoryCollapsed(dir_id) => {
                self.state.toggle_directory(dir_id);
            }
            SidebarEvent::Navigate(path) => {
                self.select_by_path(&path);
            }
            _ => {}
        }
    }

    /// Navigates to a specific path
    pub fn navigate_to(&self, path: PathBuf) -> Vec<SidebarEvent> {
        let mut events = Vec::new();
        
        let items = self.state.get_items();
        if let Some(item_id) = Self::find_item_id_by_path(&items, &path) {
            if let Some(current_id) = self.state.get_selected_item_id() {
                events.push(SidebarEvent::ItemSelected(current_id));
            }
            
            self.state.select_item(item_id.clone());
            events.push(SidebarEvent::ItemSelected(item_id));
            events.push(SidebarEvent::Navigate(path));
        }
        
        events
    }
}

/// Builder for creating sidebar items
pub struct SidebarItemBuilder {
    item: SidebarItem,
}

impl SidebarItemBuilder {
    /// Creates a new builder for a file
    pub fn file(id: impl Into<String>, name: impl Into<String>, path: impl Into<PathBuf>) -> Self {
        Self {
            item: SidebarItem::file(id, name, path),
        }
    }

    /// Creates a new builder for a directory
    pub fn directory(id: impl Into<String>, name: impl Into<String>, path: impl Into<PathBuf>) -> Self {
        Self {
            item: SidebarItem::directory(id, name, path),
        }
    }

    /// Creates a new builder with default type (file)
    pub fn new(id: impl Into<String>, name: impl Into<String>, path: impl Into<PathBuf>) -> Self {
        Self::file(id, name, path)
    }

    /// Sets as directory
    pub fn as_directory(mut self) -> Self {
        self.item.is_directory = true;
        self
    }

    /// Sets as file
    pub fn as_file(mut self) -> Self {
        self.item.is_directory = false;
        self
    }

    /// Sets the icon
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.item.icon = Some(icon.into());
        self
    }

    /// Adds a child
    pub fn with_child(mut self, child: SidebarItem) -> Self {
        self.item.children.push(child);
        self
    }

    /// Adds multiple children
    pub fn with_children(mut self, children: Vec<SidebarItem>) -> Self {
        self.item.children.extend(children);
        self
    }

    /// Sets selected state
    pub fn selected(mut self, selected: bool) -> Self {
        self.item.is_selected = selected;
        self
    }

    /// Sets expanded state
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.item.is_expanded = expanded;
        self
    }

    /// Builds the item
    pub fn build(self) -> SidebarItem {
        self.item
    }
}

/// Default sidebar sections
pub fn default_sidebar_sections() -> Vec<SidebarSection> {
    vec![
        SidebarSection::Explorer,
        SidebarSection::Recent,
        SidebarSection::Favorites,
        SidebarSection::Tags,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_sidebar_item_creation() {
        let item = SidebarItem::file("1", "Test File", PathBuf::from("/test/file.md"));
        
        assert_eq!(item.id, "1");
        assert_eq!(item.name, "Test File");
        assert_eq!(item.path, PathBuf::from("/test/file.md"));
        assert!(!item.is_directory);
        assert!(item.children.is_empty());
    }

    #[test]
    fn test_sidebar_item_directory() {
        let item = SidebarItem::directory("1", "Test Dir", PathBuf::from("/test"));
        
        assert!(item.is_directory);
    }

    #[test]
    fn test_sidebar_item_with_child() {
        let child = SidebarItem::file("2", "Child", PathBuf::from("/test/child.md"));
        let parent = SidebarItem::directory("1", "Parent", PathBuf::from("/test"))
            .with_child(child);
        
        assert_eq!(parent.children.len(), 1);
        assert_eq!(parent.children[0].id, "2");
    }

    #[test]
    fn test_sidebar_state_select_item() {
        let items = vec![
            SidebarItem::file("1", "File 1", PathBuf::from("/file1.md")),
            SidebarItem::file("2", "File 2", PathBuf::from("/file2.md")),
        ];
        
        let state = SidebarState::with_items(items);
        state.select_item("2");
        
        assert_eq!(state.get_selected_item_id(), Some("2".to_string()));
    }

    #[test]
    fn test_sidebar_state_toggle_directory() {
        let items = vec![
            SidebarItem::directory("1", "Dir 1", PathBuf::from("/dir1")),
        ];
        
        let state = SidebarState::with_items(items);
        
        state.toggle_directory("1");
        assert!(state.get_expanded_directories().contains("1"));
        
        state.toggle_directory("1");
        assert!(!state.get_expanded_directories().contains("1"));
    }

    #[test]
    fn test_sidebar_state_visibility() {
        let state = SidebarState::new();
        
        assert!(state.get_visibility());
        
        state.set_visible(false);
        assert!(!state.get_visibility());
        
        state.toggle_visibility();
        assert!(state.get_visibility());
    }

    #[test]
    fn test_sidebar_config_dark_theme() {
        let config = SidebarConfig::dark_theme();
        
        assert_eq!(config.background_color, "#282828");
        assert_eq!(config.text_color, "#dadada");
    }

    #[test]
    fn test_sidebar_config_light_theme() {
        let config = SidebarConfig::light_theme();
        
        assert_eq!(config.background_color, "#ffffff");
        assert_eq!(config.text_color, "#333333");
    }

    #[test]
    fn test_sidebar_service_navigate_to() {
        let mut service = SidebarService::with_defaults();
        
        let items = vec![
            SidebarItem::file("1", "File 1", PathBuf::from("/file1.md")),
            SidebarItem::file("2", "File 2", PathBuf::from("/file2.md")),
        ];
        
        // Replace state with items
        service.state = SidebarState::with_items(items);
        
        let events = service.navigate_to(PathBuf::from("/file2.md"));
        
        assert_eq!(service.state.get_selected_item_id(), Some("2".to_string()));
        assert!(!events.is_empty());
    }

    #[test]
    fn test_sidebar_service_handle_event() {
        let items = vec![
            SidebarItem::file("1", "File 1", PathBuf::from("/file1.md")),
        ];
        
        // Create a new service with items
        let service = SidebarService::new(SidebarConfig::default());
        let mut service = SidebarService {
            state: SidebarState::with_items(items),
            config: service.config.clone(),
        };
        
        service.handle_event(SidebarEvent::ItemSelected("1".to_string()));
        
        assert_eq!(service.state.get_selected_item_id(), Some("1".to_string()));
    }

    #[test]
    fn test_default_sidebar_sections() {
        let sections = default_sidebar_sections();
        
        assert!(sections.contains(&SidebarSection::Explorer));
        assert!(sections.contains(&SidebarSection::Recent));
        assert!(sections.contains(&SidebarSection::Favorites));
        assert!(sections.contains(&SidebarSection::Tags));
    }

    #[test]
    fn test_sidebar_item_builder() {
        let item = SidebarItemBuilder::file("1", "Test", PathBuf::from("/test.md"))
            .icon("file-icon")
            .selected(true)
            .build();
        
        assert_eq!(item.id, "1");
        assert_eq!(item.icon, Some("file-icon".to_string()));
        assert!(item.is_selected);
    }

    #[test]
    fn test_sidebar_item_with_children() {
        let child1 = SidebarItem::file("2", "Child 1", PathBuf::from("/dir/child1.md"));
        let child2 = SidebarItem::file("3", "Child 2", PathBuf::from("/dir/child2.md"));
        
        let parent = SidebarItem::directory("1", "Parent", PathBuf::from("/dir"))
            .with_child(child1)
            .with_child(child2);
        
        assert_eq!(parent.children.len(), 2);
    }

    #[test]
    fn test_sidebar_state_set_section() {
        let state = SidebarState::new();
        
        state.set_section(SidebarSection::Tags);
        assert!(matches!(state.get_section(), SidebarSection::Tags));
    }

    #[test]
    fn test_sidebar_state_expand_all() {
        let items = vec![
            SidebarItem::directory("1", "Dir 1", PathBuf::from("/dir1")),
            SidebarItem::directory("2", "Dir 2", PathBuf::from("/dir2")),
        ];
        
        let state = SidebarState::with_items(items);
        state.expand_all();
        
        let expanded = state.get_expanded_directories();
        assert_eq!(expanded.len(), 2);
        assert!(expanded.contains("1"));
        assert!(expanded.contains("2"));
    }

    #[test]
    fn test_sidebar_state_collapse_all() {
        let items = vec![
            SidebarItem::directory("1", "Dir 1", PathBuf::from("/dir1")),
            SidebarItem::directory("2", "Dir 2", PathBuf::from("/dir2")),
        ];
        
        let state = SidebarState::with_items(items);
        
        state.expand_all();
        assert_eq!(state.get_expanded_directories().len(), 2);
        
        state.collapse_all();
        assert!(state.get_expanded_directories().is_empty());
    }
}
