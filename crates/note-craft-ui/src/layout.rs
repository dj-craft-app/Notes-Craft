//! Layout System Module
//!
//! This module provides a flexible layout system for Note Craft UI.
//! It manages the arrangement of UI components (panes, sidebar, content) in various layouts.
//!
//! Note: Phase 3 - Step 3.12 (Layout System)

use std::collections::HashMap;

// ============================================================================
// Layout Types
// ============================================================================

/// Unique identifier for a pane
pub type PaneId = String;

/// Layout direction for split views
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LayoutDirection {
    /// Horizontal split (left-right)
    #[default]
    Horizontal,
    /// Vertical split (top-bottom)
    Vertical,
}

impl std::fmt::Display for LayoutDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LayoutDirection::Horizontal => write!(f, "Horizontal"),
            LayoutDirection::Vertical => write!(f, "Vertical"),
        }
    }
}

/// Layout type for the application
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AppLayout {
    /// Single pane (maximized content)
    #[default]
    Single,
    /// Two panes split vertically (left-right)
    SplitVertical,
    /// Two panes split horizontally (top-bottom)
    SplitHorizontal,
    /// Three panes: left sidebar, main content, right sidebar
    ThreeColumns,
    /// Grid layout with customizable rows/columns
    Grid,
    /// Free-floating panes (like a tiling window manager)
    Floating,
}

impl std::fmt::Display for AppLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppLayout::Single => write!(f, "Single"),
            AppLayout::SplitVertical => write!(f, "SplitVertical"),
            AppLayout::SplitHorizontal => write!(f, "SplitHorizontal"),
            AppLayout::ThreeColumns => write!(f, "ThreeColumns"),
            AppLayout::Grid => write!(f, "Grid"),
            AppLayout::Floating => write!(f, "Floating"),
        }
    }
}

/// Position for docking
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DockPosition {
    /// Left sidebar
    #[default]
    Left,
    /// Right sidebar
    Right,
    /// Top
    Top,
    /// Bottom
    Bottom,
    /// Center (main content area)
    Center,
    /// Floating (not docked)
    Floating,
}

impl std::fmt::Display for DockPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DockPosition::Left => write!(f, "Left"),
            DockPosition::Right => write!(f, "Right"),
            DockPosition::Top => write!(f, "Top"),
            DockPosition::Bottom => write!(f, "Bottom"),
            DockPosition::Center => write!(f, "Center"),
            DockPosition::Floating => write!(f, "Floating"),
        }
    }
}

// ============================================================================
// Pane Types
// ============================================================================

/// Content type for a pane
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PaneContent {
    /// Editor pane (markdown editor)
    Editor(String),
    /// Preview pane (rendered markdown)
    Preview(String),
    /// Sidebar pane (navigation)
    Sidebar,
    /// Graph view pane
    GraphView,
    /// Journal view pane
    JournalView,
    /// Asset gallery pane
    AssetGallery,
    /// Command palette pane
    CommandPalette,
    /// Custom content (for plugins)
    Custom(String),
    /// Empty pane
    Empty,
}

/// Pane configuration
#[derive(Debug, Clone)]
pub struct PaneConfig {
    /// Pane identifier
    pub id: PaneId,
    /// Pane title
    pub title: String,
    /// Content type
    pub content: PaneContent,
    /// Minimum width (for horizontal splits)
    pub min_width: Option<u32>,
    /// Minimum height (for vertical splits)
    pub min_height: Option<u32>,
    /// Is the pane resizable
    pub resizable: bool,
    /// Is the pane collapsible
    pub collapsible: bool,
    /// Is the pane visible
    pub visible: bool,
    /// Is the pane active
    pub active: bool,
    /// Can the pane be closed
    pub closable: bool,
}

impl PaneConfig {
    /// Creates a new pane configuration
    pub fn new(id: impl Into<String>, title: impl Into<String>, content: PaneContent) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            content,
            min_width: None,
            min_height: None,
            resizable: true,
            collapsible: true,
            visible: true,
            active: false,
            closable: true,
        }
    }

    /// Sets the minimum width
    pub fn with_min_width(mut self, width: u32) -> Self {
        self.min_width = Some(width);
        self
    }

    /// Sets the minimum height
    pub fn with_min_height(mut self, height: u32) -> Self {
        self.min_height = Some(height);
        self
    }

    /// Sets if the pane is resizable
    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    /// Sets if the pane is collapsible
    pub fn collapsible(mut self, collapsible: bool) -> Self {
        self.collapsible = collapsible;
        self
    }

    /// Sets if the pane is visible
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// Sets if the pane is active
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    /// Sets if the pane can be closed
    pub fn closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }
}

/// Pane state (runtime information)
#[derive(Debug, Clone)]
pub struct PaneState {
    /// Pane configuration
    pub config: PaneConfig,
    /// Current width (in pixels)
    pub width: u32,
    /// Current height (in pixels)
    pub height: u32,
    /// X position (in pixels)
    pub x: u32,
    /// Y position (in pixels)
    pub y: u32,
    /// Z-index (for floating panes)
    pub z_index: u32,
}

impl PaneState {
    /// Creates a new pane state from config
    pub fn from_config(config: PaneConfig) -> Self {
        Self {
            config,
            width: 0,
            height: 0,
            x: 0,
            y: 0,
            z_index: 0,
        }
    }

    /// Updates the position and size
    pub fn update_geometry(&mut self, x: u32, y: u32, width: u32, height: u32) {
        self.x = x;
        self.y = y;
        self.width = width;
        self.height = height;
    }

    /// Sets the z-index
    pub fn set_z_index(&mut self, z_index: u32) {
        self.z_index = z_index;
    }
}

// ============================================================================
// Layout Configuration
// ============================================================================

/// Layout configuration
#[derive(Debug, Clone)]
pub struct LayoutConfig {
    /// Application layout type
    pub app_layout: AppLayout,
    /// Default split direction
    pub default_split_direction: LayoutDirection,
    /// Split ratio (0.0 to 1.0, where 0.5 is equal)
    pub split_ratio: f32,
    /// Minimum pane width
    pub min_pane_width: u32,
    /// Minimum pane height
    pub min_pane_height: u32,
    /// Enable responsive layout adjustments
    pub responsive: bool,
    /// Breakpoint for mobile layout (in pixels)
    pub mobile_breakpoint: u32,
    /// Breakpoint for tablet layout (in pixels)
    pub tablet_breakpoint: u32,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            app_layout: AppLayout::default(),
            default_split_direction: LayoutDirection::default(),
            split_ratio: 0.5,
            min_pane_width: 200,
            min_pane_height: 150,
            responsive: true,
            mobile_breakpoint: 768,
            tablet_breakpoint: 1024,
        }
    }
}

impl LayoutConfig {
    /// Creates a new layout configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the application layout
    pub fn with_app_layout(mut self, layout: AppLayout) -> Self {
        self.app_layout = layout;
        self
    }

    /// Sets the default split direction
    pub fn with_split_direction(mut self, direction: LayoutDirection) -> Self {
        self.default_split_direction = direction;
        self
    }

    /// Sets the split ratio
    pub fn with_split_ratio(mut self, ratio: f32) -> Self {
        self.split_ratio = ratio.clamp(0.0, 1.0);
        self
    }

    /// Sets the minimum pane width
    pub fn with_min_pane_width(mut self, width: u32) -> Self {
        self.min_pane_width = width;
        self
    }

    /// Sets the minimum pane height
    pub fn with_min_pane_height(mut self, height: u32) -> Self {
        self.min_pane_height = height;
        self
    }

    /// Enables or disables responsive layout
    pub fn responsive(mut self, responsive: bool) -> Self {
        self.responsive = responsive;
        self
    }

    /// Sets the mobile breakpoint
    pub fn with_mobile_breakpoint(mut self, breakpoint: u32) -> Self {
        self.mobile_breakpoint = breakpoint;
        self
    }

    /// Sets the tablet breakpoint
    pub fn with_tablet_breakpoint(mut self, breakpoint: u32) -> Self {
        self.tablet_breakpoint = breakpoint;
        self
    }

    /// Gets the current layout based on window width
    pub fn get_responsive_layout(&self, window_width: u32) -> AppLayout {
        if !self.responsive {
            return self.app_layout;
        }

        if window_width < self.mobile_breakpoint {
            AppLayout::Single
        } else if window_width < self.tablet_breakpoint {
            AppLayout::SplitVertical
        } else {
            self.app_layout
        }
    }
}

// ============================================================================
// Layout Events
// ============================================================================

/// Events for the layout system
#[derive(Debug, Clone, PartialEq)]
pub enum LayoutEvent {
    /// Layout was changed
    LayoutChanged(AppLayout),
    /// A pane was added
    PaneAdded(PaneId),
    /// A pane was removed
    PaneRemoved(PaneId),
    /// A pane was activated
    PaneActivated(PaneId),
    /// A pane was resized
    PaneResized(PaneId, u32, u32),
    /// A pane was moved
    PaneMoved(PaneId, u32, u32),
    /// Split ratio was changed
    SplitRatioChanged(f32),
    /// Layout was reset
    Reset,
}

// ============================================================================
// Layout Manager
// ============================================================================

/// Main layout manager
pub struct LayoutManager {
    /// Layout configuration
    pub config: LayoutConfig,
    /// Map of panes by ID
    pub panes: HashMap<PaneId, PaneState>,
    /// Active pane ID
    pub active_pane: Option<PaneId>,
    /// Last used pane ID counter
    pub next_pane_id: u32,
    /// Event handler
    pub event_handler: Option<Box<dyn Fn(LayoutEvent) + Send + Sync>>,
}

impl LayoutManager {
    /// Creates a new layout manager
    pub fn new() -> Self {
        Self {
            config: LayoutConfig::default(),
            panes: HashMap::new(),
            active_pane: None,
            next_pane_id: 1,
            event_handler: None,
        }
    }

    /// Creates a new layout manager with configuration
    pub fn with_config(config: LayoutConfig) -> Self {
        Self {
            config,
            panes: HashMap::new(),
            active_pane: None,
            next_pane_id: 1,
            event_handler: None,
        }
    }

    /// Sets the event handler
    pub fn set_event_handler<F>(&mut self, handler: F) -> &mut Self
    where
        F: Fn(LayoutEvent) + Send + Sync + 'static,
    {
        self.event_handler = Some(Box::new(handler));
        self
    }

    /// Sets the layout configuration
    pub fn set_config(&mut self, config: LayoutConfig) -> &mut Self {
        self.config = config;
        self.emit_event(LayoutEvent::LayoutChanged(self.config.app_layout));
        self
    }

    /// Sets the application layout
    pub fn set_layout(&mut self, layout: AppLayout) -> &mut Self {
        self.config.app_layout = layout;
        self.emit_event(LayoutEvent::LayoutChanged(layout));
        self
    }

    /// Gets the current layout
    pub fn layout(&self) -> AppLayout {
        self.config.app_layout
    }

    /// Gets the layout configuration
    pub fn config(&self) -> &LayoutConfig {
        &self.config
    }

    /// Adds a new pane
    pub fn add_pane(&mut self, pane_config: PaneConfig) -> PaneId {
        let id = pane_config.id.clone();
        let pane_state = PaneState::from_config(pane_config);
        self.panes.insert(id.clone(), pane_state);
        self.set_active_pane(&id);
        self.emit_event(LayoutEvent::PaneAdded(id.clone()));
        id
    }

    /// Removes a pane by ID
    pub fn remove_pane<P: AsRef<str>>(&mut self, pane_id: P) -> Option<PaneState> {
        let pane_id = pane_id.as_ref();
        let removed = self.panes.remove(pane_id);

        if removed.is_some() {
            if self.active_pane.as_deref() == Some(pane_id) {
                self.active_pane = None;
            }
            self.emit_event(LayoutEvent::PaneRemoved(pane_id.to_string()));
        }

        removed
    }

    /// Sets the active pane
    pub fn set_active_pane<P: AsRef<str>>(&mut self, pane_id: P) -> &mut Self {
        let pane_id = pane_id.as_ref();
        if self.panes.contains_key(pane_id) {
            self.active_pane = Some(pane_id.to_string());
            self.emit_event(LayoutEvent::PaneActivated(pane_id.to_string()));
        }
        self
    }

    /// Gets the active pane
    pub fn active_pane(&self) -> Option<&PaneState> {
        self.active_pane.as_ref().and_then(|id| self.get_pane(id))
    }

    /// Gets a pane by ID
    pub fn get_pane<P: AsRef<str>>(&self, pane_id: P) -> Option<&PaneState> {
        self.panes.get(pane_id.as_ref())
    }

    /// Gets a mutable pane by ID
    pub fn get_pane_mut<P: AsRef<str>>(&mut self, pane_id: P) -> Option<&mut PaneState> {
        self.panes.get_mut(pane_id.as_ref())
    }

    /// Gets all panes
    pub fn get_panes(&self) -> Vec<&PaneState> {
        self.panes.values().collect()
    }

    /// Clears all panes
    pub fn clear_panes(&mut self) -> &mut Self {
        self.panes.clear();
        self.active_pane = None;
        self.emit_event(LayoutEvent::Reset);
        self
    }

    /// Generates a unique pane ID
    pub fn generate_pane_id(&mut self) -> String {
        let id = format!("pane_{}", self.next_pane_id);
        self.next_pane_id += 1;
        id
    }

    /// Splits the current layout
    pub fn split(&mut self, direction: LayoutDirection, content: PaneContent) -> Option<PaneId> {
        // Store direction for potential future use
        let _ = direction;
        let pane_id = self.generate_pane_id();
        let title = match &content {
            PaneContent::Editor(_) => "Editor".to_string(),
            PaneContent::Preview(_) => "Preview".to_string(),
            PaneContent::Sidebar => "Sidebar".to_string(),
            PaneContent::GraphView => "Graph".to_string(),
            PaneContent::JournalView => "Journal".to_string(),
            PaneContent::AssetGallery => "Assets".to_string(),
            PaneContent::CommandPalette => "Commands".to_string(),
            PaneContent::Custom(s) => s.clone(),
            PaneContent::Empty => "Empty".to_string(),
        };

        let pane_config = PaneConfig::new(pane_id.clone(), title, content);
        self.add_pane(pane_config);
        Some(pane_id)
    }

    /// Calculates the layout for all panes
    pub fn calculate_layout(&self, window_width: u32, window_height: u32) -> Vec<LayoutRect> {
        let effective_layout = self.config.get_responsive_layout(window_width);

        let mut layouts: Vec<LayoutRect> = Vec::new();

        match effective_layout {
            AppLayout::Single => {
                // Single pane layout - maximize the active pane or first pane
                if let Some(pane) = self.active_pane() {
                    layouts.push(LayoutRect::new(
                        pane.config.id.clone(),
                        0,
                        0,
                        window_width,
                        window_height,
                    ));
                } else if !self.panes.is_empty() {
                    let first_pane = self.panes.values().next().unwrap();
                    layouts.push(LayoutRect::new(
                        first_pane.config.id.clone(),
                        0,
                        0,
                        window_width,
                        window_height,
                    ));
                }
            }
            AppLayout::SplitVertical => {
                // Two panes side by side
                if self.panes.len() >= 2 {
                    let mut panes: Vec<_> = self.panes.values().collect();
                    panes.sort_by(|a, b| a.config.id.cmp(&b.config.id));
                    let split_x = (window_width as f32 * self.config.split_ratio) as u32;

                    // Left pane
                    layouts.push(LayoutRect::new(
                        panes[0].config.id.clone(),
                        0,
                        0,
                        split_x,
                        window_height,
                    ));

                    // Right pane
                    layouts.push(LayoutRect::new(
                        panes[1].config.id.clone(),
                        split_x,
                        0,
                        window_width - split_x,
                        window_height,
                    ));
                }
            }
            AppLayout::SplitHorizontal => {
                // Two panes top and bottom
                if self.panes.len() >= 2 {
                    let mut panes: Vec<_> = self.panes.values().collect();
                    panes.sort_by(|a, b| a.config.id.cmp(&b.config.id));
                    let split_y = (window_height as f32 * self.config.split_ratio) as u32;

                    // Top pane
                    layouts.push(LayoutRect::new(
                        panes[0].config.id.clone(),
                        0,
                        0,
                        window_width,
                        split_y,
                    ));

                    // Bottom pane
                    layouts.push(LayoutRect::new(
                        panes[1].config.id.clone(),
                        0,
                        split_y,
                        window_width,
                        window_height - split_y,
                    ));
                }
            }
            AppLayout::ThreeColumns => {
                // Left sidebar, main content, right sidebar
                if self.panes.len() >= 3 {
                    let mut panes: Vec<_> = self.panes.values().collect();
                    panes.sort_by(|a, b| a.config.id.cmp(&b.config.id));

                    // Left sidebar (20% width)
                    let left_width = window_width / 5;
                    layouts.push(LayoutRect::new(
                        panes[0].config.id.clone(),
                        0,
                        0,
                        left_width,
                        window_height,
                    ));

                    // Main content (60% width)
                    let main_width = window_width * 3 / 5;
                    layouts.push(LayoutRect::new(
                        panes[1].config.id.clone(),
                        left_width,
                        0,
                        main_width,
                        window_height,
                    ));

                    // Right sidebar (20% width)
                    let right_width = window_width / 5;
                    layouts.push(LayoutRect::new(
                        panes[2].config.id.clone(),
                        left_width + main_width,
                        0,
                        right_width,
                        window_height,
                    ));
                }
            }
            AppLayout::Grid => {
                // Simple grid layout
                let cols = if window_width >= 1200 { 3 } else { 2 };
                let cell_width = window_width / cols;
                let cell_height =
                    window_height / ((self.panes.len() as f32 / cols as f32).ceil() as u32);

                let mut panes: Vec<_> = self.panes.values().collect();
                panes.sort_by(|a, b| a.config.id.cmp(&b.config.id));

                for (i, pane) in panes.into_iter().enumerate() {
                    let row = i as u32 / cols;
                    let col = i as u32 % cols;
                    layouts.push(LayoutRect::new(
                        pane.config.id.clone(),
                        col * cell_width,
                        row * cell_height,
                        cell_width,
                        cell_height,
                    ));
                }
            }
            AppLayout::Floating => {
                // For floating panes, use their stored positions
                for pane in self.panes.values() {
                    layouts.push(LayoutRect::new(
                        pane.config.id.clone(),
                        pane.x,
                        pane.y,
                        pane.width,
                        pane.height,
                    ));
                }
            }
        }

        layouts
    }

    /// Resizes a pane and emits resize event
    pub fn resize_pane<P: AsRef<str>>(&mut self, pane_id: P, width: u32, height: u32) -> bool {
        let pane_id = pane_id.as_ref();
        if let Some(pane) = self.panes.get_mut(pane_id) {
            // Apply minimum constraints
            let min_width = pane.config.min_width.unwrap_or(self.config.min_pane_width);
            let min_height = pane
                .config
                .min_height
                .unwrap_or(self.config.min_pane_height);

            let final_width = width.max(min_width);
            let final_height = height.max(min_height);

            pane.width = final_width;
            pane.height = final_height;

            self.emit_event(LayoutEvent::PaneResized(
                pane_id.to_string(),
                final_width,
                final_height,
            ));
            true
        } else {
            false
        }
    }

    /// Moves a pane and emits move event
    pub fn move_pane<P: AsRef<str>>(&mut self, pane_id: P, x: u32, y: u32) -> bool {
        let pane_id = pane_id.as_ref();
        if let Some(pane) = self.panes.get_mut(pane_id) {
            pane.x = x;
            pane.y = y;
            self.emit_event(LayoutEvent::PaneMoved(pane_id.to_string(), x, y));
            true
        } else {
            false
        }
    }

    /// Sets the split ratio
    pub fn set_split_ratio(&mut self, ratio: f32) -> &mut Self {
        self.config.split_ratio = ratio.clamp(0.0, 1.0);
        self.emit_event(LayoutEvent::SplitRatioChanged(ratio));
        self
    }

    /// Resets the layout
    pub fn reset(&mut self) -> &mut Self {
        self.clear_panes();
        self.config = LayoutConfig::default();
        self.emit_event(LayoutEvent::Reset);
        self
    }

    /// Emits an event to the event handler
    fn emit_event(&self, event: LayoutEvent) {
        if let Some(ref handler) = self.event_handler {
            handler(event);
        }
    }
}

impl Default for LayoutManager {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Layout Rectangle
// ============================================================================

/// Rectangle defining a pane's position and size
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutRect {
    /// Pane ID
    pub pane_id: PaneId,
    /// X position
    pub x: u32,
    /// Y position
    pub y: u32,
    /// Width
    pub width: u32,
    /// Height
    pub height: u32,
}

impl LayoutRect {
    /// Creates a new layout rectangle
    pub fn new(pane_id: PaneId, x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            pane_id,
            x,
            y,
            width,
            height,
        }
    }

    /// Checks if the rectangle contains a point
    pub fn contains(&self, x: u32, y: u32) -> bool {
        x >= self.x && x < self.x + self.width && y >= self.y && y < self.y + self.height
    }

    /// Gets the center point
    pub fn center(&self) -> (u32, u32) {
        (self.x + self.width / 2, self.y + self.height / 2)
    }

    /// Gets the area
    pub fn area(&self) -> u64 {
        self.width as u64 * self.height as u64
    }
}

// ============================================================================
// Dock Manager
// ============================================================================

/// Manages docked panes
pub struct DockManager {
    /// Left dock panes
    pub left_dock: Vec<PaneId>,
    /// Right dock panes
    pub right_dock: Vec<PaneId>,
    /// Top dock panes
    pub top_dock: Vec<PaneId>,
    /// Bottom dock panes
    pub bottom_dock: Vec<PaneId>,
    /// Center panes
    pub center_panes: Vec<PaneId>,
}

impl DockManager {
    /// Creates a new dock manager
    pub fn new() -> Self {
        Self {
            left_dock: Vec::new(),
            right_dock: Vec::new(),
            top_dock: Vec::new(),
            bottom_dock: Vec::new(),
            center_panes: Vec::new(),
        }
    }

    /// Docks a pane at a position
    pub fn dock<P: Into<PaneId>>(&mut self, pane_id: P, position: DockPosition) {
        let pane_id = pane_id.into();
        match position {
            DockPosition::Left => {
                if !self.left_dock.contains(&pane_id) {
                    self.left_dock.push(pane_id);
                }
            }
            DockPosition::Right => {
                if !self.right_dock.contains(&pane_id) {
                    self.right_dock.push(pane_id);
                }
            }
            DockPosition::Top => {
                if !self.top_dock.contains(&pane_id) {
                    self.top_dock.push(pane_id);
                }
            }
            DockPosition::Bottom => {
                if !self.bottom_dock.contains(&pane_id) {
                    self.bottom_dock.push(pane_id);
                }
            }
            DockPosition::Center => {
                if !self.center_panes.contains(&pane_id) {
                    self.center_panes.push(pane_id);
                }
            }
            DockPosition::Floating => {
                // Remove from all docks
                self.left_dock.retain(|id| id != &pane_id);
                self.right_dock.retain(|id| id != &pane_id);
                self.top_dock.retain(|id| id != &pane_id);
                self.bottom_dock.retain(|id| id != &pane_id);
                self.center_panes.retain(|id| id != &pane_id);
            }
        }
    }

    /// Undocks a pane
    pub fn undock<P: AsRef<str>>(&mut self, pane_id: P) {
        let pane_id = pane_id.as_ref();
        self.left_dock.retain(|id| id.as_str() != pane_id);
        self.right_dock.retain(|id| id.as_str() != pane_id);
        self.top_dock.retain(|id| id.as_str() != pane_id);
        self.bottom_dock.retain(|id| id.as_str() != pane_id);
        self.center_panes.retain(|id| id.as_str() != pane_id);
    }

    /// Gets the position of a docked pane
    pub fn get_dock_position<P: AsRef<str>>(&self, pane_id: P) -> Option<DockPosition> {
        let pane_id = pane_id.as_ref();
        if self.left_dock.iter().any(|id| id.as_str() == pane_id) {
            Some(DockPosition::Left)
        } else if self.right_dock.iter().any(|id| id.as_str() == pane_id) {
            Some(DockPosition::Right)
        } else if self.top_dock.iter().any(|id| id.as_str() == pane_id) {
            Some(DockPosition::Top)
        } else if self.bottom_dock.iter().any(|id| id.as_str() == pane_id) {
            Some(DockPosition::Bottom)
        } else if self.center_panes.iter().any(|id| id.as_str() == pane_id) {
            Some(DockPosition::Center)
        } else {
            None
        }
    }

    /// Gets all panes in a dock position
    pub fn get_docked_panes(&self, position: DockPosition) -> Vec<&PaneId> {
        match position {
            DockPosition::Left => self.left_dock.iter().collect(),
            DockPosition::Right => self.right_dock.iter().collect(),
            DockPosition::Top => self.top_dock.iter().collect(),
            DockPosition::Bottom => self.bottom_dock.iter().collect(),
            DockPosition::Center => self.center_panes.iter().collect(),
            DockPosition::Floating => Vec::new(),
        }
    }
}

impl Default for DockManager {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ========== LayoutConfig Tests ==========

    #[test]
    fn test_layout_config_default() {
        let config = LayoutConfig::default();
        assert_eq!(config.app_layout, AppLayout::Single);
        assert_eq!(config.default_split_direction, LayoutDirection::Horizontal);
        assert_eq!(config.split_ratio, 0.5);
        assert_eq!(config.min_pane_width, 200);
        assert_eq!(config.min_pane_height, 150);
        assert!(config.responsive);
        assert_eq!(config.mobile_breakpoint, 768);
        assert_eq!(config.tablet_breakpoint, 1024);
    }

    #[test]
    fn test_layout_config_with_app_layout() {
        let config = LayoutConfig::new().with_app_layout(AppLayout::SplitVertical);
        assert_eq!(config.app_layout, AppLayout::SplitVertical);
    }

    #[test]
    fn test_layout_config_with_split_ratio() {
        let config = LayoutConfig::new().with_split_ratio(0.75);
        assert_eq!(config.split_ratio, 0.75);

        // Test clamping
        let config = LayoutConfig::new().with_split_ratio(1.5);
        assert_eq!(config.split_ratio, 1.0);

        let config = LayoutConfig::new().with_split_ratio(-0.5);
        assert_eq!(config.split_ratio, 0.0);
    }

    #[test]
    fn test_layout_config_get_responsive_layout() {
        let config = LayoutConfig::new()
            .with_app_layout(AppLayout::ThreeColumns)
            .with_mobile_breakpoint(768)
            .with_tablet_breakpoint(1024);

        // Mobile
        assert_eq!(config.get_responsive_layout(500), AppLayout::Single);

        // Tablet
        assert_eq!(config.get_responsive_layout(800), AppLayout::SplitVertical);

        // Desktop
        assert_eq!(config.get_responsive_layout(1200), AppLayout::ThreeColumns);

        // Responsive disabled
        let config = config.responsive(false);
        assert_eq!(config.get_responsive_layout(500), AppLayout::ThreeColumns);
    }

    // ========== PaneConfig Tests ==========

    #[test]
    fn test_pane_config_new() {
        let config = PaneConfig::new(
            "pane1",
            "Test Pane",
            PaneContent::Editor("test.md".to_string()),
        );
        assert_eq!(config.id, "pane1");
        assert_eq!(config.title, "Test Pane");
        assert_eq!(config.content, PaneContent::Editor("test.md".to_string()));
        assert!(config.resizable);
        assert!(config.collapsible);
        assert!(config.visible);
        assert!(!config.active);
        assert!(config.closable);
    }

    #[test]
    fn test_pane_config_with_min_width() {
        let config = PaneConfig::new("pane1", "Test", PaneContent::Empty).with_min_width(300);
        assert_eq!(config.min_width, Some(300));
    }

    #[test]
    fn test_pane_config_active() {
        let config = PaneConfig::new("pane1", "Test", PaneContent::Empty).active(true);
        assert!(config.active);
    }

    // ========== PaneState Tests ==========

    #[test]
    fn test_pane_state_from_config() {
        let config = PaneConfig::new("pane1", "Test", PaneContent::Empty);
        let state = PaneState::from_config(config);
        assert_eq!(state.config.id, "pane1");
        assert_eq!(state.width, 0);
        assert_eq!(state.height, 0);
        assert_eq!(state.x, 0);
        assert_eq!(state.y, 0);
    }

    #[test]
    fn test_pane_state_update_geometry() {
        let config = PaneConfig::new("pane1", "Test", PaneContent::Empty);
        let mut state = PaneState::from_config(config);
        state.update_geometry(10, 20, 100, 200);
        assert_eq!(state.x, 10);
        assert_eq!(state.y, 20);
        assert_eq!(state.width, 100);
        assert_eq!(state.height, 200);
    }

    #[test]
    fn test_pane_state_set_z_index() {
        let config = PaneConfig::new("pane1", "Test", PaneContent::Empty);
        let mut state = PaneState::from_config(config);
        state.set_z_index(100);
        assert_eq!(state.z_index, 100);
    }

    // ========== LayoutManager Tests ==========

    #[test]
    fn test_layout_manager_new() {
        let manager = LayoutManager::new();
        assert_eq!(manager.config.app_layout, AppLayout::Single);
        assert!(manager.panes.is_empty());
        assert!(manager.active_pane.is_none());
    }

    #[test]
    fn test_layout_manager_with_config() {
        let config = LayoutConfig::new().with_app_layout(AppLayout::SplitVertical);
        let manager = LayoutManager::with_config(config);
        assert_eq!(manager.config.app_layout, AppLayout::SplitVertical);
    }

    #[test]
    fn test_layout_manager_set_layout() {
        let mut manager = LayoutManager::new();
        manager.set_layout(AppLayout::ThreeColumns);
        assert_eq!(manager.layout(), AppLayout::ThreeColumns);
    }

    #[test]
    fn test_layout_manager_add_pane() {
        let mut manager = LayoutManager::new();
        let pane_config = PaneConfig::new("pane1", "Test", PaneContent::Empty);
        let pane_id = manager.add_pane(pane_config);
        assert_eq!(pane_id, "pane1");
        assert!(manager.panes.contains_key("pane1"));
        assert_eq!(manager.active_pane, Some("pane1".to_string()));
    }

    #[test]
    fn test_layout_manager_remove_pane() {
        let mut manager = LayoutManager::new();
        let pane_config = PaneConfig::new("pane1", "Test", PaneContent::Empty);
        manager.add_pane(pane_config);
        let removed = manager.remove_pane("pane1");
        assert!(removed.is_some());
        assert!(!manager.panes.contains_key("pane1"));
        assert!(manager.active_pane.is_none());
    }

    #[test]
    fn test_layout_manager_set_active_pane() {
        let mut manager = LayoutManager::new();
        let pane_config1 = PaneConfig::new("pane1", "Test1", PaneContent::Empty);
        let pane_config2 = PaneConfig::new("pane2", "Test2", PaneContent::Empty);
        manager.add_pane(pane_config1);
        manager.add_pane(pane_config2);
        manager.set_active_pane("pane2");
        assert_eq!(manager.active_pane, Some("pane2".to_string()));
    }

    #[test]
    fn test_layout_manager_get_pane() {
        let mut manager = LayoutManager::new();
        let pane_config = PaneConfig::new("pane1", "Test", PaneContent::Empty);
        manager.add_pane(pane_config);
        let pane = manager.get_pane("pane1");
        assert!(pane.is_some());
        assert_eq!(pane.unwrap().config.id, "pane1");
    }

    #[test]
    fn test_layout_manager_clear_panes() {
        let mut manager = LayoutManager::new();
        let pane_config1 = PaneConfig::new("pane1", "Test1", PaneContent::Empty);
        let pane_config2 = PaneConfig::new("pane2", "Test2", PaneContent::Empty);
        manager.add_pane(pane_config1);
        manager.add_pane(pane_config2);
        manager.clear_panes();
        assert!(manager.panes.is_empty());
        assert!(manager.active_pane.is_none());
    }

    #[test]
    fn test_layout_manager_generate_pane_id() {
        let mut manager = LayoutManager::new();
        let id1 = manager.generate_pane_id();
        let id2 = manager.generate_pane_id();
        assert_eq!(id1, "pane_1");
        assert_eq!(id2, "pane_2");
    }

    #[test]
    fn test_layout_manager_split() {
        let mut manager = LayoutManager::new();
        let pane_id = manager.split(
            LayoutDirection::Horizontal,
            PaneContent::Editor("test.md".to_string()),
        );
        assert!(pane_id.is_some());
        assert_eq!(pane_id.unwrap(), "pane_1");
        assert!(manager.panes.contains_key("pane_1"));
    }

    #[test]
    fn test_layout_manager_active_pane() {
        let mut manager = LayoutManager::new();
        let pane_config = PaneConfig::new("pane1", "Test", PaneContent::Empty);
        let pane_id = manager.add_pane(pane_config);
        assert_eq!(pane_id, "pane1");

        let active = manager.active_pane();
        assert!(active.is_some());
        assert_eq!(active.unwrap().config.id, "pane1");
    }

    #[test]
    fn test_layout_manager_calculate_layout_single() {
        let mut manager = LayoutManager::new();
        manager.config.responsive = false; // Disable responsive for testing
        let pane_config = PaneConfig::new("pane1", "Test", PaneContent::Empty);
        manager.add_pane(pane_config);
        manager.set_layout(AppLayout::Single);

        let layouts = manager.calculate_layout(800, 600);
        assert_eq!(layouts.len(), 1);
        assert_eq!(layouts[0].pane_id, "pane1");
        assert_eq!(layouts[0].x, 0);
        assert_eq!(layouts[0].y, 0);
        assert_eq!(layouts[0].width, 800);
        assert_eq!(layouts[0].height, 600);
    }

    #[test]
    fn test_layout_manager_calculate_layout_split_vertical() {
        let mut manager = LayoutManager::new();
        manager.config.responsive = false; // Disable responsive for testing
        let pane_config1 = PaneConfig::new("pane1", "Left", PaneContent::Empty);
        let pane_config2 = PaneConfig::new("pane2", "Right", PaneContent::Empty);
        manager.add_pane(pane_config1);
        manager.add_pane(pane_config2);
        manager.set_layout(AppLayout::SplitVertical);
        manager.set_split_ratio(0.3);

        let layouts = manager.calculate_layout(800, 600);
        assert_eq!(layouts.len(), 2);

        // Left pane (30% of width)
        assert_eq!(layouts[0].pane_id, "pane1");
        assert_eq!(layouts[0].x, 0);
        assert_eq!(layouts[0].y, 0);
        assert_eq!(layouts[0].width, 240); // 800 * 0.3 = 240
        assert_eq!(layouts[0].height, 600);

        // Right pane (70% of width)
        assert_eq!(layouts[1].pane_id, "pane2");
        assert_eq!(layouts[1].x, 240);
        assert_eq!(layouts[1].y, 0);
        assert_eq!(layouts[1].width, 560); // 800 - 240 = 560
        assert_eq!(layouts[1].height, 600);
    }

    #[test]
    fn test_layout_manager_calculate_layout_split_horizontal() {
        let mut manager = LayoutManager::new();
        manager.config.responsive = false; // Disable responsive for testing
        let pane_config1 = PaneConfig::new("pane1", "Top", PaneContent::Empty);
        let pane_config2 = PaneConfig::new("pane2", "Bottom", PaneContent::Empty);
        manager.add_pane(pane_config1);
        manager.add_pane(pane_config2);
        manager.set_layout(AppLayout::SplitHorizontal);
        manager.set_split_ratio(0.4);

        let layouts = manager.calculate_layout(800, 600);
        assert_eq!(layouts.len(), 2);

        // Top pane (40% of height)
        assert_eq!(layouts[0].pane_id, "pane1");
        assert_eq!(layouts[0].x, 0);
        assert_eq!(layouts[0].y, 0);
        assert_eq!(layouts[0].width, 800);
        assert_eq!(layouts[0].height, 240); // 600 * 0.4 = 240

        // Bottom pane (60% of height)
        assert_eq!(layouts[1].pane_id, "pane2");
        assert_eq!(layouts[1].x, 0);
        assert_eq!(layouts[1].y, 240);
        assert_eq!(layouts[1].width, 800);
        assert_eq!(layouts[1].height, 360); // 600 - 240 = 360
    }

    #[test]
    fn test_layout_manager_calculate_layout_three_columns() {
        let mut manager = LayoutManager::new();
        manager.config.responsive = false; // Disable responsive for testing
        let pane_config1 = PaneConfig::new("pane1", "Left", PaneContent::Empty);
        let pane_config2 = PaneConfig::new("pane2", "Center", PaneContent::Empty);
        let pane_config3 = PaneConfig::new("pane3", "Right", PaneContent::Empty);
        manager.add_pane(pane_config1);
        manager.add_pane(pane_config2);
        manager.add_pane(pane_config3);
        manager.set_layout(AppLayout::ThreeColumns);

        let layouts = manager.calculate_layout(1000, 600);
        assert_eq!(layouts.len(), 3);

        // Left sidebar (20%)
        assert_eq!(layouts[0].pane_id, "pane1");
        assert_eq!(layouts[0].x, 0);
        assert_eq!(layouts[0].width, 200);

        // Center (60%)
        assert_eq!(layouts[1].pane_id, "pane2");
        assert_eq!(layouts[1].x, 200);
        assert_eq!(layouts[1].width, 600);

        // Right sidebar (20%)
        assert_eq!(layouts[2].pane_id, "pane3");
        assert_eq!(layouts[2].x, 800);
        assert_eq!(layouts[2].width, 200);
    }

    #[test]
    fn test_layout_manager_resize_pane() {
        let mut manager = LayoutManager::new();
        let pane_config = PaneConfig::new("pane1", "Test", PaneContent::Empty)
            .with_min_width(100)
            .with_min_height(100);
        manager.add_pane(pane_config);

        let success = manager.resize_pane("pane1", 50, 50);
        assert!(success);
        // Should be clamped to minimum
        let pane = manager.get_pane("pane1").unwrap();
        assert_eq!(pane.width, 100);
        assert_eq!(pane.height, 100);
    }

    #[test]
    fn test_layout_manager_move_pane() {
        let mut manager = LayoutManager::new();
        let pane_config = PaneConfig::new("pane1", "Test", PaneContent::Empty);
        manager.add_pane(pane_config);

        let success = manager.move_pane("pane1", 100, 200);
        assert!(success);
        let pane = manager.get_pane("pane1").unwrap();
        assert_eq!(pane.x, 100);
        assert_eq!(pane.y, 200);
    }

    #[test]
    fn test_layout_manager_set_split_ratio() {
        let mut manager = LayoutManager::new();
        manager.set_split_ratio(0.75);
        assert_eq!(manager.config.split_ratio, 0.75);
    }

    #[test]
    fn test_layout_manager_reset() {
        let mut manager = LayoutManager::new();
        let pane_config = PaneConfig::new("pane1", "Test", PaneContent::Empty);
        manager.add_pane(pane_config);
        manager.set_layout(AppLayout::ThreeColumns);
        manager.reset();
        assert!(manager.panes.is_empty());
        assert_eq!(manager.config.app_layout, AppLayout::Single);
    }

    // ========== LayoutRect Tests ==========

    #[test]
    fn test_layout_rect_new() {
        let rect = LayoutRect::new("pane1".to_string(), 10, 20, 100, 200);
        assert_eq!(rect.pane_id, "pane1");
        assert_eq!(rect.x, 10);
        assert_eq!(rect.y, 20);
        assert_eq!(rect.width, 100);
        assert_eq!(rect.height, 200);
    }

    #[test]
    fn test_layout_rect_contains() {
        let rect = LayoutRect::new("pane1".to_string(), 10, 20, 100, 200);
        assert!(rect.contains(50, 50));
        assert!(rect.contains(10, 20));
        assert!(rect.contains(109, 219)); // Just inside
        assert!(!rect.contains(110, 220)); // Just outside
        assert!(!rect.contains(5, 5)); // Outside
    }

    #[test]
    fn test_layout_rect_center() {
        let rect = LayoutRect::new("pane1".to_string(), 10, 20, 100, 200);
        let (cx, cy) = rect.center();
        assert_eq!(cx, 60); // 10 + 100/2 = 60
        assert_eq!(cy, 120); // 20 + 200/2 = 120
    }

    #[test]
    fn test_layout_rect_area() {
        let rect = LayoutRect::new("pane1".to_string(), 0, 0, 100, 200);
        assert_eq!(rect.area(), 20000);
    }

    // ========== DockManager Tests ==========

    #[test]
    fn test_dock_manager_new() {
        let manager = DockManager::new();
        assert!(manager.left_dock.is_empty());
        assert!(manager.right_dock.is_empty());
        assert!(manager.top_dock.is_empty());
        assert!(manager.bottom_dock.is_empty());
        assert!(manager.center_panes.is_empty());
    }

    #[test]
    fn test_dock_manager_dock() {
        let mut manager = DockManager::new();
        manager.dock("pane1".to_string(), DockPosition::Left);
        manager.dock("pane2".to_string(), DockPosition::Right);
        manager.dock("pane3".to_string(), DockPosition::Top);

        assert_eq!(manager.left_dock, vec!["pane1"]);
        assert_eq!(manager.right_dock, vec!["pane2"]);
        assert_eq!(manager.top_dock, vec!["pane3"]);
    }

    #[test]
    fn test_dock_manager_undock() {
        let mut manager = DockManager::new();
        manager.dock("pane1".to_string(), DockPosition::Left);
        manager.undock("pane1");
        assert!(manager.left_dock.is_empty());
    }

    #[test]
    fn test_dock_manager_get_dock_position() {
        let mut manager = DockManager::new();
        manager.dock("pane1".to_string(), DockPosition::Left);
        manager.dock("pane2".to_string(), DockPosition::Right);

        assert_eq!(manager.get_dock_position("pane1"), Some(DockPosition::Left));
        assert_eq!(
            manager.get_dock_position("pane2"),
            Some(DockPosition::Right)
        );
        assert_eq!(manager.get_dock_position("pane3"), None);
    }

    #[test]
    fn test_dock_manager_get_docked_panes() {
        let mut manager = DockManager::new();
        manager.dock("pane1".to_string(), DockPosition::Left);
        manager.dock("pane2".to_string(), DockPosition::Left);

        let panes = manager.get_docked_panes(DockPosition::Left);
        assert_eq!(panes.len(), 2);
        assert!(panes.contains(&&"pane1".to_string()));
        assert!(panes.contains(&&"pane2".to_string()));
    }

    #[test]
    fn test_dock_manager_floating() {
        let mut manager = DockManager::new();
        manager.dock("pane1".to_string(), DockPosition::Left);
        manager.dock("pane1".to_string(), DockPosition::Floating);

        assert!(manager.left_dock.is_empty());
        assert_eq!(manager.get_dock_position("pane1"), None);
    }

    // ========== Enum Tests ==========

    #[test]
    fn test_app_layout_display() {
        assert_eq!(format!("{}", AppLayout::Single), "Single");
        assert_eq!(format!("{}", AppLayout::SplitVertical), "SplitVertical");
        assert_eq!(format!("{}", AppLayout::SplitHorizontal), "SplitHorizontal");
        assert_eq!(format!("{}", AppLayout::ThreeColumns), "ThreeColumns");
        assert_eq!(format!("{}", AppLayout::Grid), "Grid");
        assert_eq!(format!("{}", AppLayout::Floating), "Floating");
    }

    #[test]
    fn test_layout_direction_display() {
        assert_eq!(format!("{}", LayoutDirection::Horizontal), "Horizontal");
        assert_eq!(format!("{}", LayoutDirection::Vertical), "Vertical");
    }

    #[test]
    fn test_dock_position_display() {
        assert_eq!(format!("{}", DockPosition::Left), "Left");
        assert_eq!(format!("{}", DockPosition::Right), "Right");
        assert_eq!(format!("{}", DockPosition::Top), "Top");
        assert_eq!(format!("{}", DockPosition::Bottom), "Bottom");
        assert_eq!(format!("{}", DockPosition::Center), "Center");
        assert_eq!(format!("{}", DockPosition::Floating), "Floating");
    }

    #[test]
    fn test_pane_content_equality() {
        let content1 = PaneContent::Editor("test.md".to_string());
        let content2 = PaneContent::Editor("test.md".to_string());
        let content3 = PaneContent::Editor("other.md".to_string());

        assert_eq!(content1, content2);
        assert_ne!(content1, content3);
    }

    #[test]
    fn test_layout_event_equality() {
        let event1 = LayoutEvent::LayoutChanged(AppLayout::Single);
        let event2 = LayoutEvent::LayoutChanged(AppLayout::Single);
        let event3 = LayoutEvent::LayoutChanged(AppLayout::SplitVertical);

        assert_eq!(event1, event2);
        assert_ne!(event1, event3);
    }

    // ========== Event Handler Tests ==========
}
