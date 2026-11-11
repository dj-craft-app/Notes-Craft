//! Outliner System Module
//!
//! This module provides an outliner system for hierarchical organization of content.
//! It allows users to create, navigate, and manipulate outline structures (tree of items).

use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};

use note_craft_core::{BlockId, PageId};
use slint::SharedString;

/// Represents an item in the outline
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OutlineItem {
    /// Unique identifier
    pub id: String,
    /// Display text
    pub text: String,
    /// Optional associated block ID
    pub block_id: Option<BlockId>,
    /// Optional associated page ID
    pub page_id: Option<PageId>,
    /// Depth level in the outline (0 = root level)
    pub depth: usize,
    /// Whether the item is expanded (shows children)
    pub is_expanded: bool,
    /// Whether the item is selected
    pub is_selected: bool,
    /// Whether the item is editable
    pub is_editable: bool,
    /// Collapsed state (for lazy loading)
    pub has_children: bool,
}

impl OutlineItem {
    /// Creates a new outline item
    pub fn new(id: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            block_id: None,
            page_id: None,
            depth: 0,
            is_expanded: false,
            is_selected: false,
            is_editable: false,
            has_children: false,
        }
    }

    /// Creates a new outline item with block ID
    pub fn with_block(id: impl Into<String>, text: impl Into<String>, block_id: BlockId) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            block_id: Some(block_id),
            page_id: None,
            depth: 0,
            is_expanded: false,
            is_selected: false,
            is_editable: false,
            has_children: false,
        }
    }

    /// Creates a new outline item with page ID
    pub fn with_page(id: impl Into<String>, text: impl Into<String>, page_id: PageId) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            block_id: None,
            page_id: Some(page_id),
            depth: 0,
            is_expanded: false,
            is_selected: false,
            is_editable: false,
            has_children: false,
        }
    }

    /// Sets the depth level
    pub fn with_depth(mut self, depth: usize) -> Self {
        self.depth = depth;
        self
    }

    /// Sets expanded state
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.is_expanded = expanded;
        self
    }

    /// Sets selected state
    pub fn selected(mut self, selected: bool) -> Self {
        self.is_selected = selected;
        self
    }

    /// Sets editable state
    pub fn editable(mut self, editable: bool) -> Self {
        self.is_editable = editable;
        self
    }

    /// Sets has_children flag
    pub fn has_children(mut self, has_children: bool) -> Self {
        self.has_children = has_children;
        self
    }

    /// Gets the display text with indentation
    pub fn get_display_text(&self) -> String {
        let indent = "  ".repeat(self.depth);
        format!("{}{}", indent, self.text)
    }

    /// Checks if this item can have children
    pub fn can_have_children(&self) -> bool {
        // For now, all items can have children
        // In the future, this might depend on the type
        true
    }
}

/// Represents a path in the outline (for navigation)
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct OutlinePath {
    pub indices: Vec<usize>,
}

impl OutlinePath {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, index: usize) {
        self.indices.push(index);
    }

    pub fn pop(&mut self) -> Option<usize> {
        self.indices.pop()
    }

    pub fn len(&self) -> usize {
        self.indices.len()
    }

    pub fn is_empty(&self) -> bool {
        self.indices.is_empty()
    }

    pub fn parent(&self) -> Option<OutlinePath> {
        if self.indices.is_empty() {
            None
        } else {
            let mut parent = self.indices.clone();
            parent.pop();
            Some(OutlinePath { indices: parent })
        }
    }
}

/// Event related to the outliner
#[derive(Debug, Clone, PartialEq)]
pub enum OutlinerEvent {
    /// Item was selected
    ItemSelected(String),
    /// Item was expanded
    ItemExpanded(String, bool),
    /// Item was collapsed
    ItemCollapsed(String),
    /// Item was edited
    ItemEdited(String, String),
    /// Item was added
    ItemAdded(String, Option<String>), // (item_id, parent_id)
    /// Item was removed
    ItemRemoved(String),
    /// Item was moved
    ItemMoved(String, Option<String>, usize), // (item_id, parent_id, new_index)
    /// Outline was cleared
    Cleared,
    /// Outline was loaded
    Loaded,
}

/// Configuration for the outliner
#[derive(Debug, Clone)]
pub struct OutlinerConfig {
    /// Indentation per level (in spaces)
    pub indent_size: usize,
    /// Whether to show line numbers
    pub show_line_numbers: bool,
    /// Whether to show icons
    pub show_icons: bool,
    /// Whether to allow drag and drop
    pub allow_drag_drop: bool,
    /// Whether to auto-expand on selection
    pub auto_expand: bool,
    /// Maximum depth
    pub max_depth: Option<usize>,
}

impl Default for OutlinerConfig {
    fn default() -> Self {
        Self {
            indent_size: 2,
            show_line_numbers: false,
            show_icons: true,
            allow_drag_drop: true,
            auto_expand: false,
            max_depth: None,
        }
    }
}

impl OutlinerConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_indent_size(mut self, size: usize) -> Self {
        self.indent_size = size;
        self
    }

    pub fn with_line_numbers(mut self, show: bool) -> Self {
        self.show_line_numbers = show;
        self
    }

    pub fn with_icons(mut self, show: bool) -> Self {
        self.show_icons = show;
        self
    }

    pub fn with_drag_drop(mut self, allow: bool) -> Self {
        self.allow_drag_drop = allow;
        self
    }

    pub fn with_auto_expand(mut self, auto_expand: bool) -> Self {
        self.auto_expand = auto_expand;
        self
    }

    pub fn with_max_depth(mut self, depth: usize) -> Self {
        self.max_depth = Some(depth);
        self
    }
}

/// State of the outliner
#[derive(Debug, Clone, Default)]
pub struct OutlinerState {
    /// Currently selected item ID
    pub selected_id: Option<String>,
    /// Currently edited item ID
    pub edited_id: Option<String>,
    /// Whether the outliner is focused
    pub is_focused: bool,
}

impl OutlinerState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_selected(mut self, id: impl Into<String>) -> Self {
        self.selected_id = Some(id.into());
        self
    }

    pub fn with_edited(mut self, id: impl Into<String>) -> Self {
        self.edited_id = Some(id.into());
        self
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.is_focused = focused;
        self
    }
}

/// Manager for the outliner system
pub struct OutlinerManager {
    /// All outline items
    items: HashMap<String, OutlineItem>,
    /// Children mapping: parent_id -> [child_ids]
    children: HashMap<String, Vec<String>>,
    /// Root items (items with no parent)
    root_items: Vec<String>,
    /// Configuration
    config: OutlinerConfig,
    /// State
    state: OutlinerState,
    /// Event handler
    #[allow(clippy::type_complexity)]
    event_handler: Option<RefCell<Box<dyn FnMut(OutlinerEvent) + Send + Sync>>>,
}

impl OutlinerManager {
    /// Creates a new outliner manager
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
            children: HashMap::new(),
            root_items: Vec::new(),
            config: OutlinerConfig::default(),
            state: OutlinerState::default(),
            event_handler: None,
        }
    }

    /// Creates a manager with configuration
    pub fn with_config(config: OutlinerConfig) -> Self {
        Self {
            items: HashMap::new(),
            children: HashMap::new(),
            root_items: Vec::new(),
            config,
            state: OutlinerState::default(),
            event_handler: None,
        }
    }

    /// Adds an item to the outline
    pub fn add_item(&mut self, item: OutlineItem, parent_id: Option<&str>) -> &mut Self {
        let id = item.id.clone();
        let parent_id_str = parent_id.map(|s| s.to_string());
        self.items.insert(id.clone(), item);

        match parent_id {
            Some(parent) => {
                self.children
                    .entry(parent.to_string())
                    .or_default()
                    .push(id.clone());
            }
            None => {
                self.root_items.push(id.clone());
            }
        }

        if let Some(handler) = &self.event_handler {
            let mut handler = handler.borrow_mut();
            handler(OutlinerEvent::ItemAdded(id, parent_id_str));
        }

        self
    }

    /// Adds a root item
    pub fn add_root_item(&mut self, item: OutlineItem) -> &mut Self {
        self.add_item(item, None)
    }

    /// Adds a child item
    pub fn add_child_item(&mut self, parent_id: &str, item: OutlineItem) -> &mut Self {
        self.add_item(item, Some(parent_id))
    }

    /// Removes an item from the outline
    pub fn remove_item(&mut self, id: &str) -> Option<OutlineItem> {
        // Remove from items
        let item = self.items.remove(id)?;

        // Remove from parent's children
        if let Some(parent) = self.find_parent(id) {
            if let Some(children) = self.children.get_mut(&parent) {
                children.retain(|c| c != id);
            }
        } else {
            // Remove from root items
            self.root_items.retain(|r| r != id);
        }

        // Remove children (and their children recursively)
        self.remove_children_recursive(id);

        if let Some(handler) = &self.event_handler {
            let mut handler = handler.borrow_mut();
            handler(OutlinerEvent::ItemRemoved(id.to_string()));
        }

        // Clear selection if this was the selected item
        if self.state.selected_id == Some(id.to_string()) {
            self.state.selected_id = None;
        }

        Some(item)
    }

    /// Removes all children of an item recursively
    fn remove_children_recursive(&mut self, parent_id: &str) {
        if let Some(children) = self.children.remove(parent_id) {
            for child_id in children {
                self.items.remove(&child_id);
                self.remove_children_recursive(&child_id);
            }
        }
    }

    /// Finds the parent of an item
    pub fn find_parent(&self, id: &str) -> Option<String> {
        for (parent, children) in &self.children {
            if children.contains(&id.to_string()) {
                return Some(parent.clone());
            }
        }
        None
    }

    /// Gets all children of an item
    pub fn get_children(&self, id: &str) -> Vec<&OutlineItem> {
        self.children
            .get(id)
            .map(|child_ids| {
                child_ids
                    .iter()
                    .filter_map(|child_id| self.items.get(child_id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Gets all root items
    pub fn get_root_items(&self) -> Vec<&OutlineItem> {
        self.root_items
            .iter()
            .filter_map(|id| self.items.get(id))
            .collect()
    }

    /// Gets an item by ID
    pub fn get_item(&self, id: &str) -> Option<&OutlineItem> {
        self.items.get(id)
    }

    /// Gets an item by ID (mutable)
    pub fn get_item_mut(&mut self, id: &str) -> Option<&mut OutlineItem> {
        self.items.get_mut(id)
    }

    /// Selects an item
    pub fn select_item(&mut self, id: &str) -> &mut Self {
        if self.items.contains_key(id) {
            // Deselect previous
            if let Some(prev_id) = &self.state.selected_id {
                if let Some(prev_item) = self.items.get_mut(prev_id) {
                    prev_item.is_selected = false;
                }
            }

            // Select new
            if let Some(item) = self.items.get_mut(id) {
                item.is_selected = true;
            }

            self.state.selected_id = Some(id.to_string());

            if let Some(handler) = &self.event_handler {
                let mut handler = handler.borrow_mut();
                handler(OutlinerEvent::ItemSelected(id.to_string()));
            }

            if self.config.auto_expand {
                self.expand_item(id);
            }
        }
        self
    }

    /// Expands an item
    pub fn expand_item(&mut self, id: &str) -> &mut Self {
        if let Some(item) = self.items.get_mut(id) {
            item.is_expanded = true;
            if let Some(handler) = &self.event_handler {
                let mut handler = handler.borrow_mut();
                handler(OutlinerEvent::ItemExpanded(id.to_string(), true));
            }
        }
        self
    }

    /// Collapses an item
    pub fn collapse_item(&mut self, id: &str) -> &mut Self {
        if let Some(item) = self.items.get_mut(id) {
            item.is_expanded = false;
            if let Some(handler) = &self.event_handler {
                let mut handler = handler.borrow_mut();
                handler(OutlinerEvent::ItemCollapsed(id.to_string()));
            }
        }
        self
    }

    /// Toggles expand/collapse state
    pub fn toggle_expand(&mut self, id: &str) -> &mut Self {
        if let Some(item) = self.items.get_mut(id) {
            item.is_expanded = !item.is_expanded;
            if let Some(handler) = &self.event_handler {
                let mut handler = handler.borrow_mut();
                handler(OutlinerEvent::ItemExpanded(
                    id.to_string(),
                    item.is_expanded,
                ));
            }
        }
        self
    }

    /// Moves an item to a new parent and position
    pub fn move_item(
        &mut self,
        item_id: &str,
        new_parent_id: Option<&str>,
        new_index: usize,
    ) -> Result<&mut Self, String> {
        // Check if item exists
        if !self.items.contains_key(item_id) {
            return Err(format!("Item '{}' not found", item_id));
        }

        // Check if new parent exists (if specified)
        if let Some(parent_id) = new_parent_id {
            if !self.items.contains_key(parent_id) && !self.is_root_item(parent_id) {
                return Err(format!("Parent '{}' not found", parent_id));
            }
        }

        // Remove from old parent
        let old_parent = self.find_parent(item_id);
        if let Some(parent) = old_parent {
            if let Some(children) = self.children.get_mut(&parent) {
                children.retain(|c| c != item_id);
            }
        } else {
            self.root_items.retain(|r| r != item_id);
        }

        // Add to new parent
        match new_parent_id {
            Some(parent) => {
                self.children
                    .entry(parent.to_string())
                    .or_default()
                    .insert(new_index, item_id.to_string());
            }
            None => {
                self.root_items.insert(new_index, item_id.to_string());
            }
        }

        // Update depth
        let new_depth = match new_parent_id {
            Some(parent) => self.get_item(parent).map(|p| p.depth + 1).unwrap_or(1),
            None => 0,
        };
        if let Some(item) = self.items.get_mut(item_id) {
            item.depth = new_depth;
        }

        if let Some(handler) = &self.event_handler {
            let mut handler = handler.borrow_mut();
            handler(OutlinerEvent::ItemMoved(
                item_id.to_string(),
                new_parent_id.map(|s| s.to_string()),
                new_index,
            ));
        }

        Ok(self)
    }

    /// Checks if an item is a root item
    pub fn is_root_item(&self, id: &str) -> bool {
        self.root_items.contains(&id.to_string())
    }

    /// Clears all items
    pub fn clear(&mut self) -> &mut Self {
        self.items.clear();
        self.children.clear();
        self.root_items.clear();
        self.state.selected_id = None;
        self.state.edited_id = None;

        if let Some(handler) = &self.event_handler {
            let mut handler = handler.borrow_mut();
            handler(OutlinerEvent::Cleared);
        }

        self
    }

    /// Gets the current configuration
    pub fn config(&self) -> &OutlinerConfig {
        &self.config
    }

    /// Gets the current state
    pub fn state(&self) -> &OutlinerState {
        &self.state
    }

    /// Sets the event handler
    pub fn set_event_handler<F>(&mut self, handler: F) -> &mut Self
    where
        F: FnMut(OutlinerEvent) + Send + Sync + 'static,
    {
        self.event_handler = Some(RefCell::new(Box::new(handler)));
        self
    }

    /// Sets the configuration
    pub fn set_config(&mut self, config: OutlinerConfig) -> &mut Self {
        self.config = config;
        self
    }

    /// Gets all items as a flat list (for display)
    pub fn get_flat_items(&self) -> Vec<&OutlineItem> {
        let mut result = Vec::new();
        self.collect_items_recursive(&self.root_items, &mut result);
        result
    }

    /// Recursively collects items
    fn collect_items_recursive<'a>(&'a self, ids: &[String], result: &mut Vec<&'a OutlineItem>) {
        for id in ids {
            if let Some(item) = self.items.get(id) {
                result.push(item);
                if item.is_expanded || !item.has_children {
                    let children = self.get_children(id);
                    let child_ids: Vec<String> = children.iter().map(|c| c.id.clone()).collect();
                    self.collect_items_recursive(&child_ids, result);
                }
            }
        }
    }

    /// Gets the depth of an item
    pub fn get_depth(&self, id: &str) -> Option<usize> {
        self.items.get(id).map(|item| item.depth)
    }

    /// Gets the path from root to an item
    pub fn get_path(&self, id: &str) -> Option<Vec<String>> {
        let mut path = Vec::new();
        let mut current = id.to_string();

        loop {
            path.push(current.clone());
            match self.find_parent(&current) {
                Some(parent) => current = parent,
                None => break,
            }
        }

        path.reverse();
        Some(path)
    }

    /// Finds an item by text (search)
    pub fn find_by_text(&self, text: &str) -> Vec<&OutlineItem> {
        let text = text.to_lowercase();
        self.items
            .values()
            .filter(|item| item.text.to_lowercase().contains(&text))
            .collect()
    }

    /// Gets the count of items
    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    /// Checks if the outliner is empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl Default for OutlinerManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for creating outliners
#[derive(Default)]
pub struct OutlinerBuilder {
    items: Vec<OutlineItem>,
    config: OutlinerConfig,
}

impl OutlinerBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_item(mut self, item: OutlineItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn add_items(mut self, items: impl IntoIterator<Item = OutlineItem>) -> Self {
        self.items.extend(items);
        self
    }

    pub fn with_config(mut self, config: OutlinerConfig) -> Self {
        self.config = config;
        self
    }

    pub fn build(self) -> OutlinerManager {
        let mut manager = OutlinerManager::with_config(self.config);
        for item in self.items {
            manager.add_root_item(item);
        }
        manager
    }
}

/// Traversal order for outline items
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TraversalOrder {
    /// Depth-first
    DepthFirst,
    /// Breadth-first
    BreadthFirst,
}

/// Iterator for traversing outline items
pub struct OutlineIterator<'a> {
    manager: &'a OutlinerManager,
    order: TraversalOrder,
    stack: VecDeque<String>,
    visited: Vec<String>,
}

impl<'a> OutlineIterator<'a> {
    pub fn new(manager: &'a OutlinerManager, order: TraversalOrder) -> Self {
        let mut stack = VecDeque::new();
        for id in &manager.root_items {
            stack.push_back(id.clone());
        }

        Self {
            manager,
            order,
            stack,
            visited: Vec::new(),
        }
    }
}

impl<'a> Iterator for OutlineIterator<'a> {
    type Item = &'a OutlineItem;

    fn next(&mut self) -> Option<Self::Item> {
        match self.order {
            TraversalOrder::DepthFirst => {
                while let Some(id) = self.stack.pop_back() {
                    if !self.visited.contains(&id) {
                        self.visited.push(id.clone());
                        if let Some(item) = self.manager.items.get(&id) {
                            // Push children in reverse order to maintain order
                            let children = self.manager.get_children(&id);
                            for child in children.iter().rev() {
                                if !self.visited.contains(&child.id) {
                                    self.stack.push_back(child.id.clone());
                                }
                            }
                            return Some(item);
                        }
                    }
                }
                None
            }
            TraversalOrder::BreadthFirst => {
                while let Some(id) = self.stack.pop_front() {
                    if !self.visited.contains(&id) {
                        self.visited.push(id.clone());
                        if let Some(item) = self.manager.items.get(&id) {
                            // Push children to the end
                            let children = self.manager.get_children(&id);
                            for child in children.iter() {
                                if !self.visited.contains(&child.id) {
                                    self.stack.push_back(child.id.clone());
                                }
                            }
                            return Some(item);
                        }
                    }
                }
                None
            }
        }
    }
}

// ============================================================================
// Slint Integration
// ============================================================================

/// Creates a Slint-compatible model for the outliner
pub fn create_outliner_model(manager: &OutlinerManager) -> Vec<HashMap<String, SharedString>> {
    let items = manager.get_flat_items();
    items
        .into_iter()
        .map(|item| {
            let mut map = HashMap::new();
            map.insert("id".to_string(), SharedString::from(&item.id));
            map.insert("text".to_string(), SharedString::from(&item.text));
            map.insert(
                "depth".to_string(),
                SharedString::from(item.depth.to_string()),
            );
            map.insert(
                "is_expanded".to_string(),
                SharedString::from(item.is_expanded.to_string()),
            );
            map.insert(
                "is_selected".to_string(),
                SharedString::from(item.is_selected.to_string()),
            );
            if let Some(block_id) = &item.block_id {
                map.insert(
                    "block_id".to_string(),
                    SharedString::from(&block_id.0.to_string()),
                );
            }
            if let Some(page_id) = &item.page_id {
                map.insert(
                    "page_id".to_string(),
                    SharedString::from(&page_id.0.to_string()),
                );
            }
            map
        })
        .collect()
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_outline_item_creation() {
        let item = OutlineItem::new("item1", "Test Item");
        assert_eq!(item.id, "item1");
        assert_eq!(item.text, "Test Item");
        assert_eq!(item.depth, 0);
        assert!(!item.is_expanded);
        assert!(!item.is_selected);
    }

    #[test]
    fn test_outline_item_with_block() {
        let block_id = BlockId(1);
        let item = OutlineItem::with_block("item1", "Test", block_id);
        assert_eq!(item.block_id, Some(block_id));
    }

    #[test]
    fn test_outline_item_with_page() {
        let page_id = PageId(1);
        let item = OutlineItem::with_page("item1", "Test", page_id);
        assert_eq!(item.page_id, Some(page_id));
    }

    #[test]
    fn test_outline_item_display_text() {
        let item = OutlineItem::new("item1", "Test").with_depth(2);
        assert_eq!(item.get_display_text(), "    Test");
    }

    #[test]
    fn test_outliner_manager_new() {
        let manager = OutlinerManager::new();
        assert!(manager.is_empty());
        assert_eq!(manager.item_count(), 0);
    }

    #[test]
    fn test_outliner_add_root_item() {
        let mut manager = OutlinerManager::new();
        manager.add_root_item(OutlineItem::new("item1", "Test 1"));

        assert!(!manager.is_empty());
        assert_eq!(manager.item_count(), 1);
        assert_eq!(manager.get_root_items().len(), 1);
    }

    #[test]
    fn test_outliner_add_child_item() {
        let mut manager = OutlinerManager::new();
        manager.add_root_item(OutlineItem::new("parent", "Parent"));
        manager.add_child_item("parent", OutlineItem::new("child", "Child"));

        assert_eq!(manager.item_count(), 2);
        assert_eq!(manager.get_children("parent").len(), 1);
    }

    #[test]
    fn test_outliner_find_parent() {
        let mut manager = OutlinerManager::new();
        manager.add_root_item(OutlineItem::new("parent", "Parent"));
        manager.add_child_item("parent", OutlineItem::new("child", "Child"));

        assert_eq!(manager.find_parent("child"), Some("parent".to_string()));
        assert!(manager.find_parent("parent").is_none());
    }

    #[test]
    fn test_outliner_select_item() {
        let mut manager = OutlinerManager::new();
        manager.add_root_item(OutlineItem::new("item1", "Test"));

        manager.select_item("item1");

        assert_eq!(manager.state().selected_id, Some("item1".to_string()));
        let item = manager.get_item("item1").unwrap();
        assert!(item.is_selected);
    }

    #[test]
    fn test_outliner_expand_collapse() {
        let mut manager = OutlinerManager::new();
        manager.add_root_item(OutlineItem::new("item1", "Test"));

        manager.expand_item("item1");
        let item = manager.get_item("item1").unwrap();
        assert!(item.is_expanded);

        manager.collapse_item("item1");
        let item = manager.get_item("item1").unwrap();
        assert!(!item.is_expanded);

        manager.toggle_expand("item1");
        let item = manager.get_item("item1").unwrap();
        assert!(item.is_expanded);
    }

    #[test]
    fn test_outliner_remove_item() {
        let mut manager = OutlinerManager::new();
        manager.add_root_item(OutlineItem::new("item1", "Test"));

        let removed = manager.remove_item("item1");
        assert!(removed.is_some());
        assert!(manager.is_empty());
    }

    #[test]
    fn test_outliner_remove_with_children() {
        let mut manager = OutlinerManager::new();
        manager.add_root_item(OutlineItem::new("parent", "Parent"));
        manager.add_child_item("parent", OutlineItem::new("child", "Child"));

        let removed = manager.remove_item("parent");
        assert!(removed.is_some());
        assert!(manager.is_empty());
    }

    #[test]
    fn test_outliner_clear() {
        let mut manager = OutlinerManager::new();
        manager.add_root_item(OutlineItem::new("item1", "Test"));
        manager.add_root_item(OutlineItem::new("item2", "Test 2"));

        manager.clear();
        assert!(manager.is_empty());
    }

    #[test]
    fn test_outliner_move_item() {
        let mut manager = OutlinerManager::new();
        manager.add_root_item(OutlineItem::new("item1", "Test 1"));
        manager.add_root_item(OutlineItem::new("item2", "Test 2"));

        manager.move_item("item1", None, 0).unwrap();

        // Check order
        let root_items = manager.get_root_items();
        assert_eq!(root_items.len(), 2);
    }

    #[test]
    fn test_outliner_get_path() {
        let mut manager = OutlinerManager::new();
        manager.add_root_item(OutlineItem::new("root", "Root"));
        manager.add_child_item("root", OutlineItem::new("child", "Child"));
        manager.add_child_item("child", OutlineItem::new("grandchild", "Grandchild"));

        let path = manager.get_path("grandchild");
        assert!(path.is_some());
        let path = path.unwrap();
        assert_eq!(path, vec!["root", "child", "grandchild"]);
    }

    #[test]
    fn test_outliner_find_by_text() {
        let mut manager = OutlinerManager::new();
        manager.add_root_item(OutlineItem::new("item1", "Test Item"));
        manager.add_root_item(OutlineItem::new("item2", "Another Item"));

        let results = manager.find_by_text("Test");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "item1");
    }

    #[test]
    fn test_outliner_get_flat_items() {
        let mut manager = OutlinerManager::new();
        manager.add_root_item(OutlineItem::new("root", "Root"));
        manager.add_child_item("root", OutlineItem::new("child1", "Child 1"));
        manager.add_child_item("root", OutlineItem::new("child2", "Child 2"));

        // Children are not expanded by default, so only root should appear
        let flat = manager.get_flat_items();
        assert!(!flat.is_empty());
    }

    #[test]
    fn test_outliner_config() {
        let config = OutlinerConfig::new()
            .with_indent_size(4)
            .with_line_numbers(true)
            .with_icons(false);

        assert_eq!(config.indent_size, 4);
        assert!(config.show_line_numbers);
        assert!(!config.show_icons);
    }

    #[test]
    fn test_outliner_state() {
        let state = OutlinerState::new()
            .with_selected("item1")
            .with_edited("item2")
            .focused(true);

        assert_eq!(state.selected_id, Some("item1".to_string()));
        assert_eq!(state.edited_id, Some("item2".to_string()));
        assert!(state.is_focused);
    }

    #[test]
    fn test_outliner_event_handling() {
        use std::sync::{Arc, Mutex};

        let mut manager = OutlinerManager::new();
        let events = Arc::new(Mutex::new(Vec::new()));
        let events_clone = events.clone();

        manager.set_event_handler(move |event| {
            events_clone.lock().unwrap().push(event);
        });

        manager.add_root_item(OutlineItem::new("item1", "Test"));
        assert!(!events.lock().unwrap().is_empty());

        manager.select_item("item1");
        assert!(events
            .lock()
            .unwrap()
            .iter()
            .any(|e| matches!(e, OutlinerEvent::ItemSelected(_))));
    }

    #[test]
    fn test_outliner_iterator_depth_first() {
        let mut manager = OutlinerManager::new();
        manager.add_root_item(OutlineItem::new("root", "Root"));
        manager.add_child_item("root", OutlineItem::new("child1", "Child 1"));
        manager.add_child_item("root", OutlineItem::new("child2", "Child 2"));

        // Expand all for iteration
        manager.expand_item("root");

        let items: Vec<_> = OutlineIterator::new(&manager, TraversalOrder::DepthFirst).collect();
        assert!(!items.is_empty());
    }

    #[test]
    fn test_outliner_iterator_breadth_first() {
        let mut manager = OutlinerManager::new();
        manager.add_root_item(OutlineItem::new("root", "Root"));
        manager.add_child_item("root", OutlineItem::new("child1", "Child 1"));
        manager.add_child_item("root", OutlineItem::new("child2", "Child 2"));

        // Expand all for iteration
        manager.expand_item("root");

        let items: Vec<_> = OutlineIterator::new(&manager, TraversalOrder::BreadthFirst).collect();
        assert!(!items.is_empty());
    }

    #[test]
    fn test_outliner_builder() {
        let builder = OutlinerBuilder::new()
            .add_item(OutlineItem::new("item1", "Test 1"))
            .add_item(OutlineItem::new("item2", "Test 2"));

        let manager = builder.build();
        assert_eq!(manager.item_count(), 2);
    }

    #[test]
    fn test_outliner_outline_path() {
        let mut path = OutlinePath::new();
        assert!(path.is_empty());

        path.push(0);
        path.push(1);
        assert_eq!(path.len(), 2);

        let parent = path.parent();
        assert!(parent.is_some());
        assert_eq!(parent.unwrap().len(), 1);
    }

    #[test]
    fn test_outliner_is_root_item() {
        let mut manager = OutlinerManager::new();
        manager.add_root_item(OutlineItem::new("root", "Root"));
        manager.add_child_item("root", OutlineItem::new("child", "Child"));

        assert!(manager.is_root_item("root"));
        assert!(!manager.is_root_item("child"));
    }

    #[test]
    fn test_outliner_can_have_children() {
        let item = OutlineItem::new("item1", "Test");
        assert!(item.can_have_children());
    }
}
