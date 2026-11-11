//! Asset Gallery Module
//!
//! This module provides a gallery view for assets (images, files) in Note Craft.
//! It displays assets in a grid layout with thumbnails and metadata.
//!
//! This is a UI-only module that works with generic asset data.
//!
//! Note: Phase 3 - Step 3.11 (Asset Gallery)

use std::collections::HashMap;

// ============================================================================
// Asset Types
// ============================================================================

/// Asset type for gallery items
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AssetType {
    /// Image (PNG, JPEG, GIF, WebP, SVG)
    Image,
    /// Document PDF
    Pdf,
    /// Video
    Video,
    /// Audio
    Audio,
    /// Other file
    Other,
}

impl std::fmt::Display for AssetType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssetType::Image => write!(f, "Image"),
            AssetType::Pdf => write!(f, "PDF"),
            AssetType::Video => write!(f, "Video"),
            AssetType::Audio => write!(f, "Audio"),
            AssetType::Other => write!(f, "Other"),
        }
    }
}

impl AssetType {
    /// Checks if this is an image type
    pub fn is_image(&self) -> bool {
        matches!(self, AssetType::Image)
    }

    /// Gets the icon for this asset type
    pub fn icon(&self) -> char {
        match self {
            AssetType::Image => '🖼',
            AssetType::Pdf => '📄',
            AssetType::Video => '🎥',
            AssetType::Audio => '🎵',
            AssetType::Other => '📁',
        }
    }
}

/// Asset identifier type
pub type AssetId = String;

/// Asset metadata for gallery display
#[derive(Debug, Clone)]
pub struct AssetMetadata {
    /// Original file name
    pub name: String,
    /// Asset type
    pub asset_type: AssetType,
    /// File extension
    pub extension: String,
    /// File size in bytes
    pub size: u64,
    /// URL to access the asset
    pub url: String,
    /// Optional width (for images)
    pub width: Option<u32>,
    /// Optional height (for images)
    pub height: Option<u32>,
    /// Creation timestamp
    pub created_at: u64,
    /// Optional tags
    pub tags: Vec<String>,
    /// Optional description
    pub description: Option<String>,
}

impl AssetMetadata {
    /// Creates new metadata
    pub fn new(name: impl Into<String>, asset_type: AssetType) -> Self {
        let name = name.into();
        let extension = name
            .rsplit('.')
            .next()
            .map(|s| s.to_lowercase())
            .unwrap_or_default();

        Self {
            name,
            asset_type,
            extension,
            size: 0,
            url: String::new(),
            width: None,
            height: None,
            created_at: 0,
            tags: Vec::new(),
            description: None,
        }
    }

    /// Sets the file size
    pub fn with_size(mut self, size: u64) -> Self {
        self.size = size;
        self
    }

    /// Sets the URL
    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = url.into();
        self
    }

    /// Sets dimensions (for images)
    pub fn with_dimensions(mut self, width: u32, height: u32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    /// Sets creation timestamp
    pub fn with_created_at(mut self, timestamp: u64) -> Self {
        self.created_at = timestamp;
        self
    }

    /// Adds a tag
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// Sets description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

/// Gallery asset item
#[derive(Debug, Clone)]
pub struct GalleryAsset {
    /// Unique identifier
    pub id: AssetId,
    /// Metadata
    pub metadata: AssetMetadata,
    /// Whether this asset is selected
    pub selected: bool,
    /// Whether this asset is highlighted
    pub highlighted: bool,
}

impl GalleryAsset {
    /// Creates a new gallery asset
    pub fn new(id: impl Into<AssetId>, metadata: AssetMetadata) -> Self {
        Self {
            id: id.into(),
            metadata,
            selected: false,
            highlighted: false,
        }
    }

    /// Sets whether this asset is selected
    pub fn set_selected(&mut self, selected: bool) -> &mut Self {
        self.selected = selected;
        self
    }

    /// Sets whether this asset is highlighted
    pub fn set_highlighted(&mut self, highlighted: bool) -> &mut Self {
        self.highlighted = highlighted;
        self
    }

    /// Gets the display title
    pub fn title(&self) -> &str {
        &self.metadata.name
    }

    /// Gets the display subtitle
    pub fn subtitle(&self) -> String {
        let mut parts = Vec::new();

        // Add size
        parts.push(format_size(self.metadata.size));

        // Add dimensions for images
        if let (Some(w), Some(h)) = (self.metadata.width, self.metadata.height) {
            parts.push(format!("{}x{}", w, h));
        }

        // Add type
        parts.push(format!("{}", self.metadata.asset_type));

        parts.join(" • ")
    }

    /// Gets the file extension
    pub fn extension(&self) -> &str {
        &self.metadata.extension
    }

    /// Gets the file size
    pub fn size(&self) -> u64 {
        self.metadata.size
    }

    /// Gets the file size formatted
    pub fn size_formatted(&self) -> String {
        format_size(self.metadata.size)
    }

    /// Checks if this is an image
    pub fn is_image(&self) -> bool {
        self.metadata.asset_type.is_image()
    }

    /// Gets dimensions if available
    pub fn dimensions(&self) -> Option<String> {
        match (self.metadata.width, self.metadata.height) {
            (Some(w), Some(h)) => Some(format!("{}x{}", w, h)),
            _ => None,
        }
    }

    /// Gets the icon for this asset
    pub fn icon(&self) -> char {
        self.metadata.asset_type.icon()
    }
}

// ============================================================================
// Gallery Layout
// ============================================================================

/// Layout mode for the gallery
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GalleryLayout {
    /// Grid layout (multiple columns)
    #[default]
    Grid,
    /// List layout (single column)
    List,
    /// Masonry layout (variable height)
    Masonry,
}

impl std::fmt::Display for GalleryLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GalleryLayout::Grid => write!(f, "Grid"),
            GalleryLayout::List => write!(f, "List"),
            GalleryLayout::Masonry => write!(f, "Masonry"),
        }
    }
}

/// Grid layout configuration
#[derive(Debug, Clone)]
pub struct GridConfig {
    /// Number of columns
    pub columns: usize,
    /// Gap between items in pixels
    pub gap: usize,
    /// Minimum item width
    pub min_item_width: usize,
    /// Maximum item width
    pub max_item_width: usize,
}

impl Default for GridConfig {
    fn default() -> Self {
        Self {
            columns: 4,
            gap: 16,
            min_item_width: 120,
            max_item_width: 300,
        }
    }
}

/// Masonry layout configuration
#[derive(Debug, Clone)]
pub struct MasonryConfig {
    /// Number of columns
    pub columns: usize,
    /// Gap between items in pixels
    pub gap: usize,
    /// Minimum item width
    pub min_item_width: usize,
}

impl Default for MasonryConfig {
    fn default() -> Self {
        Self {
            columns: 4,
            gap: 16,
            min_item_width: 120,
        }
    }
}

// ============================================================================
// Gallery Sort Order
// ============================================================================

/// Sort order for gallery items
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GallerySortOrder {
    /// Newest first (by creation date)
    #[default]
    NewestFirst,
    /// Oldest first (by creation date)
    OldestFirst,
    /// By name (A-Z)
    NameAscending,
    /// By name (Z-A)
    NameDescending,
    /// By size (largest first)
    SizeDescending,
    /// By size (smallest first)
    SizeAscending,
    /// By type (grouped)
    TypeGrouped,
}

impl std::fmt::Display for GallerySortOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GallerySortOrder::NewestFirst => write!(f, "Newest First"),
            GallerySortOrder::OldestFirst => write!(f, "Oldest First"),
            GallerySortOrder::NameAscending => write!(f, "Name (A-Z)"),
            GallerySortOrder::NameDescending => write!(f, "Name (Z-A)"),
            GallerySortOrder::SizeDescending => write!(f, "Largest First"),
            GallerySortOrder::SizeAscending => write!(f, "Smallest First"),
            GallerySortOrder::TypeGrouped => write!(f, "By Type"),
        }
    }
}

// ============================================================================
// Gallery Filter
// ============================================================================

/// Filter for gallery items
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GalleryFilter {
    /// Filter by asset type(s)
    pub asset_types: Vec<AssetType>,
    /// Filter by extension(s)
    pub extensions: Vec<String>,
    /// Minimum size in bytes
    pub min_size: Option<u64>,
    /// Maximum size in bytes
    pub max_size: Option<u64>,
    /// Search query (matches name, description, tags)
    pub query: Option<String>,
    /// Filter by tags
    pub tags: Vec<String>,
}

impl GalleryFilter {
    /// Creates a new empty filter
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the asset types to filter by
    pub fn with_asset_type(mut self, asset_type: AssetType) -> Self {
        if !self.asset_types.contains(&asset_type) {
            self.asset_types.push(asset_type);
        }
        self
    }

    /// Sets the extensions to filter by
    pub fn with_extension(mut self, extension: impl Into<String>) -> Self {
        let ext = extension.into().to_lowercase();
        if !self.extensions.contains(&ext) {
            self.extensions.push(ext);
        }
        self
    }

    /// Sets the minimum size
    pub fn with_min_size(mut self, size: u64) -> Self {
        self.min_size = Some(size);
        self
    }

    /// Sets the maximum size
    pub fn with_max_size(mut self, size: u64) -> Self {
        self.max_size = Some(size);
        self
    }

    /// Sets the search query
    pub fn with_query(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
        self
    }

    /// Adds a tag to filter by
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// Checks if an asset matches this filter
    pub fn matches(&self, asset: &GalleryAsset) -> bool {
        let metadata = &asset.metadata;

        // Check asset type
        if !self.asset_types.is_empty() && !self.asset_types.contains(&metadata.asset_type) {
            return false;
        }

        // Check extension
        if !self.extensions.is_empty() && !self.extensions.contains(&metadata.extension) {
            return false;
        }

        // Check size
        if let Some(min_size) = self.min_size {
            if metadata.size < min_size {
                return false;
            }
        }

        if let Some(max_size) = self.max_size {
            if metadata.size > max_size {
                return false;
            }
        }

        // Check query
        if let Some(ref query) = self.query {
            let query_lower = query.to_lowercase();
            let name_lower = metadata.name.to_lowercase();
            let description_lower = metadata
                .description
                .clone()
                .map(|d| d.to_lowercase())
                .unwrap_or_default();
            let tags_lower: Vec<String> = metadata.tags.iter().map(|t| t.to_lowercase()).collect();

            if !name_lower.contains(&query_lower)
                && !description_lower.contains(&query_lower)
                && !tags_lower.iter().any(|t| t.contains(&query_lower))
            {
                return false;
            }
        }

        // Check tags
        if !self.tags.is_empty() {
            let has_tag = self.tags.iter().any(|tag| metadata.tags.contains(tag));
            if !has_tag {
                return false;
            }
        }

        true
    }

    /// Clears all filters
    pub fn clear(&mut self) -> &mut Self {
        self.asset_types.clear();
        self.extensions.clear();
        self.min_size = None;
        self.max_size = None;
        self.query = None;
        self.tags.clear();
        self
    }
}

// ============================================================================
// Thumbnail Size
// ============================================================================

/// Thumbnail size options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThumbnailSize {
    /// Small thumbnails (96x96)
    Small,
    /// Medium thumbnails (192x192)
    #[default]
    Medium,
    /// Large thumbnails (384x384)
    Large,
    /// Original size
    Original,
}

impl ThumbnailSize {
    /// Gets the size in pixels
    pub fn size_px(&self) -> u32 {
        match self {
            ThumbnailSize::Small => 96,
            ThumbnailSize::Medium => 192,
            ThumbnailSize::Large => 384,
            ThumbnailSize::Original => 0, // Will use original
        }
    }
}

// ============================================================================
// Gallery Theme
// ============================================================================

/// Gallery theme
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GalleryTheme {
    /// Default theme
    #[default]
    Default,
    /// Dark theme
    Dark,
    /// Light theme
    Light,
    /// Compact theme
    Compact,
}

// ============================================================================
// Gallery Events
// ============================================================================

/// Events for the asset gallery
#[derive(Debug, Clone, PartialEq)]
pub enum GalleryEvent {
    /// An asset was clicked
    AssetClicked(AssetId),
    /// An asset was double-clicked
    AssetDoubleClicked(AssetId),
    /// An asset was selected
    AssetSelected(AssetId),
    /// An asset was deselected
    AssetDeselected(AssetId),
    /// All assets were selected
    AllSelected,
    /// All assets were deselected
    AllDeselected,
    /// The selection changed
    SelectionChanged(Vec<AssetId>),
    /// An asset was dragged
    AssetDragged(AssetId),
    /// An asset was dropped
    AssetDropped(AssetId, f64, f64),
    /// The layout changed
    LayoutChanged(GalleryLayout),
    /// The sort order changed
    SortOrderChanged(GallerySortOrder),
    /// The filter changed
    FilterChanged(GalleryFilter),
    /// An asset was previewed
    AssetPreviewed(AssetId),
    /// The gallery was refreshed
    Refreshed,
}

// ============================================================================
// Gallery Configuration
// ============================================================================

/// Configuration for the asset gallery
#[derive(Debug, Clone)]
pub struct GalleryConfig {
    /// Layout mode
    pub layout: GalleryLayout,
    /// Sort order
    pub sort_order: GallerySortOrder,
    /// Grid configuration
    pub grid_config: GridConfig,
    /// Masonry configuration
    pub masonry_config: MasonryConfig,
    /// Whether to show thumbnails
    pub show_thumbnails: bool,
    /// Whether to show metadata
    pub show_metadata: bool,
    /// Whether to show file names
    pub show_file_names: bool,
    /// Whether to allow multi-selection
    pub allow_multi_select: bool,
    /// Whether to show selection checkboxes
    pub show_selection: bool,
    /// Thumbnail size
    pub thumbnail_size: ThumbnailSize,
    /// Theme
    pub theme: GalleryTheme,
}

impl Default for GalleryConfig {
    fn default() -> Self {
        Self {
            layout: GalleryLayout::Grid,
            sort_order: GallerySortOrder::NewestFirst,
            grid_config: GridConfig::default(),
            masonry_config: MasonryConfig::default(),
            show_thumbnails: true,
            show_metadata: true,
            show_file_names: true,
            allow_multi_select: false,
            show_selection: false,
            thumbnail_size: ThumbnailSize::Medium,
            theme: GalleryTheme::Default,
        }
    }
}

impl GalleryConfig {
    /// Creates a new configuration with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the layout mode
    pub fn with_layout(mut self, layout: GalleryLayout) -> Self {
        self.layout = layout;
        self
    }

    /// Sets the sort order
    pub fn with_sort_order(mut self, sort_order: GallerySortOrder) -> Self {
        self.sort_order = sort_order;
        self
    }

    /// Sets whether to show thumbnails
    pub fn with_thumbnails(mut self, show: bool) -> Self {
        self.show_thumbnails = show;
        self
    }

    /// Sets whether to show metadata
    pub fn with_metadata(mut self, show: bool) -> Self {
        self.show_metadata = show;
        self
    }

    /// Sets whether to allow multi-selection
    pub fn with_multi_select(mut self, allow: bool) -> Self {
        self.allow_multi_select = allow;
        self
    }

    /// Sets the thumbnail size
    pub fn with_thumbnail_size(mut self, size: ThumbnailSize) -> Self {
        self.thumbnail_size = size;
        self
    }

    /// Sets the theme
    pub fn with_theme(mut self, theme: GalleryTheme) -> Self {
        self.theme = theme;
        self
    }
}

// ============================================================================
// Gallery Section
// ============================================================================

/// A section in the gallery (for grouped views)
#[derive(Debug, Clone)]
pub struct GallerySection {
    /// Section title
    pub title: String,
    /// Section ID
    pub id: String,
    /// Items in this section
    pub items: Vec<GalleryAsset>,
    /// Whether this section is expanded
    pub expanded: bool,
    /// Section type
    pub section_type: GallerySectionType,
}

impl GallerySection {
    /// Creates a new gallery section
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        section_type: GallerySectionType,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            items: Vec::new(),
            expanded: true,
            section_type,
        }
    }

    /// Adds an item to this section
    pub fn add_item(&mut self, item: GalleryAsset) -> &mut Self {
        self.items.push(item);
        self
    }

    /// Gets the number of items
    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    /// Gets all asset IDs
    pub fn asset_ids(&self) -> Vec<AssetId> {
        self.items.iter().map(|item| item.id.clone()).collect()
    }

    /// Clears all items
    pub fn clear(&mut self) -> &mut Self {
        self.items.clear();
        self
    }
}

/// Type of gallery section
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GallerySectionType {
    /// All assets
    All,
    /// By asset type
    ByType,
    /// By date
    ByDate,
    /// By tag
    ByTag,
    /// By extension
    ByExtension,
}

impl std::fmt::Display for GallerySectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GallerySectionType::All => write!(f, "All"),
            GallerySectionType::ByType => write!(f, "By Type"),
            GallerySectionType::ByDate => write!(f, "By Date"),
            GallerySectionType::ByTag => write!(f, "By Tag"),
            GallerySectionType::ByExtension => write!(f, "By Extension"),
        }
    }
}

// ============================================================================
// Main Gallery Structure
// ============================================================================

/// Main asset gallery structure
pub struct AssetGallery {
    /// List of all assets (not filtered)
    pub all_assets: Vec<GalleryAsset>,
    /// List of assets currently visible (filtered)
    pub assets: Vec<GalleryAsset>,
    /// List of sections (for grouped views)
    pub sections: Vec<GallerySection>,
    /// Configuration
    pub config: GalleryConfig,
    /// Filter
    pub filter: GalleryFilter,
    /// Selected asset IDs
    pub selected_ids: Vec<AssetId>,
    /// Previewed asset ID
    pub previewed_id: Option<AssetId>,
    /// Event handler
    pub event_handler: Option<Box<dyn Fn(GalleryEvent) + Send + Sync>>,
    /// Thumbnail cache
    pub thumbnail_cache: HashMap<AssetId, String>,
}

impl AssetGallery {
    /// Creates a new asset gallery
    pub fn new() -> Self {
        Self {
            all_assets: Vec::new(),
            assets: Vec::new(),
            sections: Vec::new(),
            config: GalleryConfig::default(),
            filter: GalleryFilter::default(),
            selected_ids: Vec::new(),
            previewed_id: None,
            event_handler: None,
            thumbnail_cache: HashMap::new(),
        }
    }

    /// Creates a new asset gallery with configuration
    pub fn with_config(config: GalleryConfig) -> Self {
        Self {
            all_assets: Vec::new(),
            assets: Vec::new(),
            sections: Vec::new(),
            config,
            filter: GalleryFilter::default(),
            selected_ids: Vec::new(),
            previewed_id: None,
            event_handler: None,
            thumbnail_cache: HashMap::new(),
        }
    }

    /// Sets the event handler
    pub fn set_event_handler<F>(&mut self, handler: F) -> &mut Self
    where
        F: Fn(GalleryEvent) + Send + Sync + 'static,
    {
        self.event_handler = Some(Box::new(handler));
        self
    }

    /// Loads assets into the gallery
    pub fn load_assets(&mut self, assets: Vec<GalleryAsset>) -> &mut Self {
        self.all_assets = assets.clone();
        self.assets.clear();
        self.sections.clear();
        self.selected_ids.clear();
        self.previewed_id = None;

        // Apply filter
        self.assets = assets
            .into_iter()
            .filter(|a| self.filter.matches(a))
            .collect();

        // Sort items
        self.sort_assets();

        // Build sections
        self.build_sections();

        self
    }

    /// Adds a single asset
    pub fn add_asset(&mut self, asset: GalleryAsset) -> &mut Self {
        self.all_assets.push(asset.clone());
        if self.filter.matches(&asset) {
            self.assets.push(asset);
            self.sort_assets();
            self.rebuild_sections();
        }
        self
    }

    /// Removes an asset by ID
    pub fn remove_asset(&mut self, id: &AssetId) -> Option<GalleryAsset> {
        if let Some(index) = self.all_assets.iter().position(|a| a.id == *id) {
            let asset = self.all_assets.remove(index);

            // Remove from filtered assets
            self.assets.retain(|a| a.id != *id);

            // Remove from selection
            self.selected_ids.retain(|s| s != id);

            // Remove from sections
            for section in &mut self.sections {
                section.items.retain(|a| a.id != *id);
            }

            // Clear preview if it was this asset
            if self.previewed_id.as_ref() == Some(id) {
                self.previewed_id = None;
            }

            Some(asset)
        } else {
            None
        }
    }

    /// Sorts assets based on configuration
    fn sort_assets(&mut self) {
        match self.config.sort_order {
            GallerySortOrder::NewestFirst => {
                self.assets
                    .sort_by_key(|a| std::cmp::Reverse(a.metadata.created_at));
            }
            GallerySortOrder::OldestFirst => {
                self.assets.sort_by_key(|a| a.metadata.created_at);
            }
            GallerySortOrder::NameAscending => {
                self.assets.sort_by_key(|a| a.metadata.name.clone());
            }
            GallerySortOrder::NameDescending => {
                self.assets
                    .sort_by_key(|a| std::cmp::Reverse(a.metadata.name.clone()));
            }
            GallerySortOrder::SizeDescending => {
                self.assets
                    .sort_by_key(|a| std::cmp::Reverse(a.metadata.size));
            }
            GallerySortOrder::SizeAscending => {
                self.assets.sort_by_key(|a| a.metadata.size);
            }
            GallerySortOrder::TypeGrouped => {
                self.assets.sort_by(|a, b| {
                    a.metadata
                        .asset_type
                        .cmp(&b.metadata.asset_type)
                        .then(a.metadata.name.cmp(&b.metadata.name))
                });
            }
        }
    }

    /// Builds sections based on configuration
    fn build_sections(&mut self) {
        self.sections.clear();

        // For now, create a single section with all assets
        // In the future, this can be enhanced to group by type, date, etc.
        let mut section = GallerySection::new("all", "All Assets", GallerySectionType::All);
        for asset in &self.assets {
            section.add_item(asset.clone());
        }
        self.sections.push(section);
    }

    /// Rebuilds sections (keeps existing configuration)
    fn rebuild_sections(&mut self) {
        self.sections.clear();

        let mut section = GallerySection::new("all", "All Assets", GallerySectionType::All);
        for asset in &self.assets {
            section.add_item(asset.clone());
        }
        self.sections.push(section);
    }

    /// Handles an asset click
    pub fn handle_asset_click(&mut self, id: &AssetId) {
        if self.config.allow_multi_select {
            // Toggle selection
            if let Some(index) = self.selected_ids.iter().position(|s| s == id) {
                self.selected_ids.remove(index);
                if let Some(handler) = &self.event_handler {
                    handler(GalleryEvent::AssetDeselected(id.clone()));
                }
            } else {
                self.selected_ids.push(id.clone());
                if let Some(handler) = &self.event_handler {
                    handler(GalleryEvent::AssetSelected(id.clone()));
                }
            }

            if let Some(handler) = &self.event_handler {
                handler(GalleryEvent::SelectionChanged(self.selected_ids.clone()));
            }
        } else {
            // Single selection
            self.selected_ids.clear();
            self.selected_ids.push(id.clone());

            if let Some(handler) = &self.event_handler {
                handler(GalleryEvent::AssetClicked(id.clone()));
            }
        }

        // Update asset selection state
        for asset in &mut self.assets {
            asset.selected = self.selected_ids.contains(&asset.id);
        }
    }

    /// Handles an asset double click
    pub fn handle_asset_double_click(&mut self, id: &AssetId) {
        if let Some(handler) = &self.event_handler {
            handler(GalleryEvent::AssetDoubleClicked(id.clone()));
        }
    }

    /// Sets the layout
    pub fn set_layout(&mut self, layout: GalleryLayout) -> &mut Self {
        self.config.layout = layout;
        if let Some(handler) = &self.event_handler {
            handler(GalleryEvent::LayoutChanged(layout));
        }
        self
    }

    /// Sets the sort order
    pub fn set_sort_order(&mut self, sort_order: GallerySortOrder) -> &mut Self {
        self.config.sort_order = sort_order;
        self.sort_assets();
        if let Some(handler) = &self.event_handler {
            handler(GalleryEvent::SortOrderChanged(sort_order));
        }
        self
    }

    /// Sets the filter
    pub fn set_filter(&mut self, filter: GalleryFilter) -> &mut Self {
        self.filter = filter.clone();
        if let Some(handler) = &self.event_handler {
            handler(GalleryEvent::FilterChanged(filter));
        }
        self
    }

    /// Gets the number of assets
    pub fn asset_count(&self) -> usize {
        self.assets.len()
    }

    /// Gets all asset IDs
    pub fn asset_ids(&self) -> Vec<AssetId> {
        self.assets.iter().map(|a| a.id.clone()).collect()
    }

    /// Gets selected asset IDs
    pub fn selected_asset_ids(&self) -> Vec<AssetId> {
        self.selected_ids.clone()
    }

    /// Gets the first selected asset ID
    pub fn first_selected(&self) -> Option<&AssetId> {
        self.selected_ids.first()
    }

    /// Clears the selection
    pub fn clear_selection(&mut self) -> &mut Self {
        self.selected_ids.clear();
        for asset in &mut self.assets {
            asset.selected = false;
        }
        if let Some(handler) = &self.event_handler {
            handler(GalleryEvent::AllDeselected);
        }
        self
    }

    /// Selects all assets
    pub fn select_all(&mut self) -> &mut Self {
        self.selected_ids.clear();
        for asset in &mut self.assets {
            asset.selected = true;
            self.selected_ids.push(asset.id.clone());
        }
        if let Some(handler) = &self.event_handler {
            handler(GalleryEvent::AllSelected);
            handler(GalleryEvent::SelectionChanged(self.selected_ids.clone()));
        }
        self
    }

    /// Gets an asset by ID
    pub fn get_asset(&self, id: &AssetId) -> Option<&GalleryAsset> {
        self.assets.iter().find(|a| a.id == *id)
    }

    /// Gets a mutable asset by ID
    pub fn get_asset_mut(&mut self, id: &AssetId) -> Option<&mut GalleryAsset> {
        self.assets.iter_mut().find(|a| a.id == *id)
    }

    /// Clears all assets
    pub fn clear(&mut self) -> &mut Self {
        self.all_assets.clear();
        self.assets.clear();
        self.sections.clear();
        self.selected_ids.clear();
        self.previewed_id = None;
        self.thumbnail_cache.clear();
        self
    }

    /// Refreshes the gallery (re-applies filter and re-sorts)
    pub fn refresh(&mut self) -> &mut Self {
        // Re-filter assets from all_assets
        self.assets = self
            .all_assets
            .iter()
            .filter(|a| self.filter.matches(a))
            .cloned()
            .collect();

        // Re-sort
        self.sort_assets();

        // Rebuild sections
        self.rebuild_sections();

        if let Some(handler) = &self.event_handler {
            handler(GalleryEvent::Refreshed);
        }

        self
    }
}

impl Default for AssetGallery {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Utility Functions
// ============================================================================

/// Formats a size in bytes to human-readable format
pub fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = 1024 * KB;
    const GB: u64 = 1024 * MB;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Formats a timestamp to a date string
pub fn format_date(timestamp: u64) -> String {
    // Convert timestamp to date (simplified)
    let days = timestamp / 86400;
    let year = 1970 + (days / 365) as i32;
    let day_of_year = (days % 365) as u32;

    let month = day_of_year / 30 + 1;
    let day = day_of_year % 30 + 1;

    format!("{:04}-{:02}-{:02}", year, month, day)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Creates a test asset
    fn create_test_asset(id: u64, name: &str, asset_type: AssetType) -> GalleryAsset {
        let stored_name = format!("stored_{}_{}", id, name);
        let metadata = AssetMetadata::new(name, asset_type)
            .with_size(1024 * id)
            .with_url(format!("/assets/{}", name));
        let metadata = if asset_type.is_image() {
            metadata.with_dimensions(800, 600)
        } else {
            metadata
        };
        let metadata = metadata.with_created_at(1000 * id);

        GalleryAsset::new(stored_name, metadata)
    }

    #[test]
    fn test_gallery_asset_new() {
        let asset = create_test_asset(1, "test.png", AssetType::Image);
        assert_eq!(asset.title(), "test.png");
        assert!(asset.is_image());
        assert!(!asset.selected);
        assert!(!asset.highlighted);
    }

    #[test]
    fn test_gallery_asset_subtitle() {
        let asset = create_test_asset(1, "test.png", AssetType::Image);
        let subtitle = asset.subtitle();
        assert!(subtitle.contains("1.00 KB"));
        assert!(subtitle.contains("800x600"));
        assert!(subtitle.contains("Image"));
    }

    #[test]
    fn test_gallery_asset_dimensions() {
        let image_asset = create_test_asset(1, "test.png", AssetType::Image);
        let pdf_asset = create_test_asset(2, "test.pdf", AssetType::Pdf);

        assert_eq!(image_asset.dimensions(), Some("800x600".to_string()));
        assert!(pdf_asset.dimensions().is_none());
    }

    #[test]
    fn test_gallery_asset_icon() {
        let image_asset = create_test_asset(1, "test.png", AssetType::Image);
        let pdf_asset = create_test_asset(2, "test.pdf", AssetType::Pdf);

        assert_eq!(image_asset.icon(), '🖼');
        assert_eq!(pdf_asset.icon(), '📄');
    }

    #[test]
    fn test_asset_type_display() {
        assert_eq!(format!("{}", AssetType::Image), "Image");
        assert_eq!(format!("{}", AssetType::Pdf), "PDF");
        assert_eq!(format!("{}", AssetType::Video), "Video");
        assert_eq!(format!("{}", AssetType::Audio), "Audio");
        assert_eq!(format!("{}", AssetType::Other), "Other");
    }

    #[test]
    fn test_asset_type_is_image() {
        assert!(AssetType::Image.is_image());
        assert!(!AssetType::Pdf.is_image());
        assert!(!AssetType::Video.is_image());
        assert!(!AssetType::Audio.is_image());
        assert!(!AssetType::Other.is_image());
    }

    #[test]
    fn test_gallery_layout_display() {
        assert_eq!(format!("{}", GalleryLayout::Grid), "Grid");
        assert_eq!(format!("{}", GalleryLayout::List), "List");
        assert_eq!(format!("{}", GalleryLayout::Masonry), "Masonry");
    }

    #[test]
    fn test_gallery_sort_order_display() {
        assert_eq!(format!("{}", GallerySortOrder::NewestFirst), "Newest First");
        assert_eq!(format!("{}", GallerySortOrder::OldestFirst), "Oldest First");
        assert_eq!(format!("{}", GallerySortOrder::NameAscending), "Name (A-Z)");
        assert_eq!(
            format!("{}", GallerySortOrder::NameDescending),
            "Name (Z-A)"
        );
        assert_eq!(
            format!("{}", GallerySortOrder::SizeDescending),
            "Largest First"
        );
        assert_eq!(
            format!("{}", GallerySortOrder::SizeAscending),
            "Smallest First"
        );
        assert_eq!(format!("{}", GallerySortOrder::TypeGrouped), "By Type");
    }

    #[test]
    fn test_gallery_filter_new() {
        let filter = GalleryFilter::new();
        assert!(filter.asset_types.is_empty());
        assert!(filter.extensions.is_empty());
        assert!(filter.min_size.is_none());
        assert!(filter.max_size.is_none());
    }

    #[test]
    fn test_gallery_filter_with_asset_type() {
        let filter = GalleryFilter::new().with_asset_type(AssetType::Image);
        assert_eq!(filter.asset_types.len(), 1);
        assert!(filter.asset_types.contains(&AssetType::Image));
    }

    #[test]
    fn test_gallery_filter_with_extension() {
        let filter = GalleryFilter::new().with_extension("png");
        assert_eq!(filter.extensions.len(), 1);
        assert!(filter.extensions.contains(&"png".to_string()));
    }

    #[test]
    fn test_gallery_filter_with_query() {
        let filter = GalleryFilter::new().with_query("test");
        assert_eq!(filter.query, Some("test".to_string()));
    }

    #[test]
    fn test_gallery_filter_with_tag() {
        let filter = GalleryFilter::new().with_tag("photo");
        assert_eq!(filter.tags.len(), 1);
        assert!(filter.tags.contains(&"photo".to_string()));
    }

    #[test]
    fn test_gallery_filter_matches() {
        let asset = create_test_asset(1, "test.png", AssetType::Image);

        // Test matching by type
        let filter = GalleryFilter::new().with_asset_type(AssetType::Image);
        assert!(filter.matches(&asset));

        let filter = GalleryFilter::new().with_asset_type(AssetType::Pdf);
        assert!(!filter.matches(&asset));

        // Test matching by extension
        let filter = GalleryFilter::new().with_extension("png");
        assert!(filter.matches(&asset));

        let filter = GalleryFilter::new().with_extension("jpg");
        assert!(!filter.matches(&asset));

        // Test matching by query
        let filter = GalleryFilter::new().with_query("test");
        assert!(filter.matches(&asset));

        let filter = GalleryFilter::new().with_query("other");
        assert!(!filter.matches(&asset));
    }

    #[test]
    fn test_gallery_filter_clear() {
        let mut filter = GalleryFilter::new()
            .with_asset_type(AssetType::Image)
            .with_extension("png")
            .with_min_size(1024)
            .with_query("test");

        filter.clear();

        assert!(filter.asset_types.is_empty());
        assert!(filter.extensions.is_empty());
        assert!(filter.min_size.is_none());
        assert!(filter.max_size.is_none());
        assert!(filter.query.is_none());
        assert!(filter.tags.is_empty());
    }

    #[test]
    fn test_gallery_config_default() {
        let config = GalleryConfig::default();
        assert_eq!(config.layout, GalleryLayout::Grid);
        assert_eq!(config.sort_order, GallerySortOrder::NewestFirst);
        assert!(config.show_thumbnails);
        assert!(config.show_metadata);
        assert!(config.show_file_names);
    }

    #[test]
    fn test_gallery_config_with_layout() {
        let config = GalleryConfig::new().with_layout(GalleryLayout::List);
        assert_eq!(config.layout, GalleryLayout::List);
    }

    #[test]
    fn test_gallery_config_with_sort_order() {
        let config = GalleryConfig::new().with_sort_order(GallerySortOrder::NameAscending);
        assert_eq!(config.sort_order, GallerySortOrder::NameAscending);
    }

    #[test]
    fn test_asset_gallery_new() {
        let gallery = AssetGallery::new();
        assert!(gallery.assets.is_empty());
        assert!(gallery.sections.is_empty());
        assert!(gallery.selected_ids.is_empty());
        assert!(gallery.previewed_id.is_none());
    }

    #[test]
    fn test_asset_gallery_load_assets() {
        let mut gallery = AssetGallery::new();
        let assets = vec![
            create_test_asset(1, "test1.png", AssetType::Image),
            create_test_asset(2, "test2.png", AssetType::Image),
        ];

        gallery.load_assets(assets);
        assert_eq!(gallery.asset_count(), 2);
    }

    #[test]
    fn test_asset_gallery_add_asset() {
        let mut gallery = AssetGallery::new();
        let asset = create_test_asset(1, "test.png", AssetType::Image);

        gallery.add_asset(asset);
        assert_eq!(gallery.asset_count(), 1);
    }

    #[test]
    fn test_asset_gallery_remove_asset() {
        let mut gallery = AssetGallery::new();
        let asset1 = create_test_asset(1, "test1.png", AssetType::Image);
        let asset2 = create_test_asset(2, "test2.png", AssetType::Image);
        let stored_name_1 = asset1.id.clone();

        gallery.add_asset(asset1);
        gallery.add_asset(asset2);
        assert_eq!(gallery.asset_count(), 2);

        let removed = gallery.remove_asset(&stored_name_1);
        assert!(removed.is_some());
        assert_eq!(gallery.asset_count(), 1);
    }

    #[test]
    fn test_asset_gallery_clear() {
        let mut gallery = AssetGallery::new();
        let asset = create_test_asset(1, "test.png", AssetType::Image);

        gallery.add_asset(asset);
        assert_eq!(gallery.asset_count(), 1);

        gallery.clear();
        assert_eq!(gallery.asset_count(), 0);
        assert!(gallery.selected_ids.is_empty());
        assert!(gallery.previewed_id.is_none());
    }

    #[test]
    fn test_asset_gallery_handle_asset_click() {
        let mut gallery = AssetGallery::new();
        let asset = create_test_asset(1, "test.png", AssetType::Image);
        let id = asset.id.clone();

        gallery.add_asset(asset);
        gallery.handle_asset_click(&id);

        assert_eq!(gallery.selected_ids.len(), 1);
        assert_eq!(gallery.first_selected(), Some(&id));
    }

    #[test]
    fn test_asset_gallery_handle_asset_click_multi_select() {
        let mut gallery = AssetGallery::new();
        gallery.config.allow_multi_select = true;

        let asset1 = create_test_asset(1, "test1.png", AssetType::Image);
        let asset2 = create_test_asset(2, "test2.png", AssetType::Image);
        let id1 = asset1.id.clone();
        let id2 = asset2.id.clone();

        gallery.add_asset(asset1);
        gallery.add_asset(asset2);

        gallery.handle_asset_click(&id1);
        gallery.handle_asset_click(&id2);

        assert_eq!(gallery.selected_ids.len(), 2);
    }

    #[test]
    fn test_asset_gallery_clear_selection() {
        let mut gallery = AssetGallery::new();
        let asset = create_test_asset(1, "test.png", AssetType::Image);
        let id = asset.id.clone();

        gallery.add_asset(asset);
        gallery.handle_asset_click(&id);
        assert_eq!(gallery.selected_ids.len(), 1);

        gallery.clear_selection();
        assert!(gallery.selected_ids.is_empty());
    }

    #[test]
    fn test_asset_gallery_select_all() {
        let mut gallery = AssetGallery::new();
        let asset1 = create_test_asset(1, "test1.png", AssetType::Image);
        let asset2 = create_test_asset(2, "test2.png", AssetType::Image);

        gallery.add_asset(asset1);
        gallery.add_asset(asset2);

        gallery.select_all();
        assert_eq!(gallery.selected_ids.len(), 2);
    }

    #[test]
    fn test_asset_gallery_set_layout() {
        let mut gallery = AssetGallery::new();
        gallery.set_layout(GalleryLayout::List);
        assert_eq!(gallery.config.layout, GalleryLayout::List);
    }

    #[test]
    fn test_asset_gallery_set_sort_order() {
        let mut gallery = AssetGallery::new();
        let asset1 = create_test_asset(1, "test1.png", AssetType::Image);
        let asset2 = create_test_asset(2, "test2.png", AssetType::Image);

        gallery.add_asset(asset1);
        gallery.add_asset(asset2);

        gallery.set_sort_order(GallerySortOrder::NameAscending);
        assert_eq!(gallery.config.sort_order, GallerySortOrder::NameAscending);
    }

    #[test]
    fn test_asset_gallery_set_filter() {
        let mut gallery = AssetGallery::new();
        let filter = GalleryFilter::new().with_asset_type(AssetType::Image);

        gallery.set_filter(filter);
        assert_eq!(gallery.filter.asset_types.len(), 1);
    }

    #[test]
    fn test_asset_gallery_get_first_selected() {
        let mut gallery = AssetGallery::new();
        let asset = create_test_asset(1, "test.png", AssetType::Image);
        let id = asset.id.clone();

        gallery.add_asset(asset);
        gallery.handle_asset_click(&id);

        assert_eq!(gallery.first_selected(), Some(&id));
    }

    #[test]
    fn test_asset_gallery_selected_asset_ids() {
        let mut gallery = AssetGallery::new();
        let asset1 = create_test_asset(1, "test1.png", AssetType::Image);
        let asset2 = create_test_asset(2, "test2.png", AssetType::Image);
        let id1 = asset1.id.clone();
        let id2 = asset2.id.clone();

        gallery.add_asset(asset1);
        gallery.add_asset(asset2);
        gallery.config.allow_multi_select = true;

        gallery.handle_asset_click(&id1);
        gallery.handle_asset_click(&id2);

        let selected = gallery.selected_asset_ids();
        assert_eq!(selected.len(), 2);
        assert!(selected.contains(&id1));
        assert!(selected.contains(&id2));
    }

    #[test]
    fn test_asset_gallery_asset_ids() {
        let mut gallery = AssetGallery::new();
        let asset1 = create_test_asset(1, "test1.png", AssetType::Image);
        let asset2 = create_test_asset(2, "test2.png", AssetType::Image);
        let id1 = asset1.id.clone();
        let id2 = asset2.id.clone();

        gallery.add_asset(asset1);
        gallery.add_asset(asset2);

        let ids = gallery.asset_ids();
        assert_eq!(ids.len(), 2);
        assert!(ids.contains(&id1));
        assert!(ids.contains(&id2));
    }

    #[test]
    fn test_asset_gallery_get_asset() {
        let mut gallery = AssetGallery::new();
        let asset = create_test_asset(1, "test.png", AssetType::Image);
        let id = asset.id.clone();

        gallery.add_asset(asset);

        let retrieved = gallery.get_asset(&id);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().title(), "test.png");
    }

    #[test]
    fn test_gallery_section_new() {
        let section = GallerySection::new("all", "All Assets", GallerySectionType::All);
        assert_eq!(section.id, "all");
        assert_eq!(section.title, "All Assets");
        assert_eq!(section.section_type, GallerySectionType::All);
        assert!(section.items.is_empty());
    }

    #[test]
    fn test_gallery_section_add_item() {
        let mut section = GallerySection::new("all", "All Assets", GallerySectionType::All);
        let asset = create_test_asset(1, "test.png", AssetType::Image);

        section.add_item(asset);
        assert_eq!(section.item_count(), 1);
    }

    #[test]
    fn test_gallery_section_asset_ids() {
        let mut section = GallerySection::new("all", "All Assets", GallerySectionType::All);
        let asset1 = create_test_asset(1, "test1.png", AssetType::Image);
        let asset2 = create_test_asset(2, "test2.png", AssetType::Image);
        let id1 = asset1.id.clone();
        let id2 = asset2.id.clone();

        section.add_item(asset1);
        section.add_item(asset2);

        let ids = section.asset_ids();
        assert_eq!(ids.len(), 2);
        assert!(ids.contains(&id1));
        assert!(ids.contains(&id2));
    }

    #[test]
    fn test_gallery_section_type_display() {
        assert_eq!(format!("{}", GallerySectionType::All), "All");
        assert_eq!(format!("{}", GallerySectionType::ByType), "By Type");
        assert_eq!(format!("{}", GallerySectionType::ByDate), "By Date");
        assert_eq!(format!("{}", GallerySectionType::ByTag), "By Tag");
        assert_eq!(
            format!("{}", GallerySectionType::ByExtension),
            "By Extension"
        );
    }

    #[test]
    fn test_gallery_event_types() {
        let id = "test_id".to_string();
        assert_eq!(
            format!("{:?}", GalleryEvent::AssetClicked(id.clone())),
            "AssetClicked(\"test_id\")"
        );
        assert_eq!(
            format!("{:?}", GalleryEvent::LayoutChanged(GalleryLayout::Grid)),
            "LayoutChanged(Grid)"
        );
    }

    #[test]
    fn test_asset_gallery_event_handler() {
        use std::sync::{Arc, Mutex};

        let mut gallery = AssetGallery::new();
        let events = Arc::new(Mutex::new(Vec::new()));
        let events_clone = events.clone();

        gallery.set_event_handler(move |event| {
            events_clone.lock().unwrap().push(event);
        });

        let asset = create_test_asset(1, "test.png", AssetType::Image);
        let id = asset.id.clone();
        gallery.add_asset(asset);
        gallery.handle_asset_click(&id);

        let events = events.lock().unwrap();
        assert!(!events.is_empty());
        assert!(matches!(events[0], GalleryEvent::AssetClicked(_)));
    }

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(100), "100 B");
        assert_eq!(format_size(1024), "1.00 KB");
        assert_eq!(format_size(1024 * 1024), "1.00 MB");
        assert_eq!(format_size(1024 * 1024 * 1024), "1.00 GB");
    }

    #[test]
    fn test_asset_gallery_with_config() {
        let config = GalleryConfig::new().with_layout(GalleryLayout::List);
        let gallery = AssetGallery::with_config(config);
        assert_eq!(gallery.config.layout, GalleryLayout::List);
    }

    #[test]
    fn test_asset_gallery_handle_asset_double_click() {
        use std::sync::{Arc, Mutex};

        let mut gallery = AssetGallery::new();
        let events = Arc::new(Mutex::new(Vec::new()));
        let events_clone = events.clone();

        gallery.set_event_handler(move |event| {
            events_clone.lock().unwrap().push(event);
        });

        let asset = create_test_asset(1, "test.png", AssetType::Image);
        let id = asset.id.clone();
        gallery.add_asset(asset);
        gallery.handle_asset_double_click(&id);

        let events = events.lock().unwrap();
        assert!(!events.is_empty());
        assert!(matches!(events[0], GalleryEvent::AssetDoubleClicked(_)));
    }

    #[test]
    fn test_asset_gallery_set_layout_with_handler() {
        use std::sync::{Arc, Mutex};

        let mut gallery = AssetGallery::new();
        let events = Arc::new(Mutex::new(Vec::new()));
        let events_clone = events.clone();

        gallery.set_event_handler(move |event| {
            events_clone.lock().unwrap().push(event);
        });

        gallery.set_layout(GalleryLayout::List);

        let events = events.lock().unwrap();
        assert!(!events.is_empty());
        assert!(matches!(events[0], GalleryEvent::LayoutChanged(_)));
    }

    #[test]
    fn test_asset_gallery_refresh() {
        let mut gallery = AssetGallery::new();
        let asset1 = create_test_asset(1, "test1.png", AssetType::Image);
        let asset2 = create_test_asset(2, "test2.pdf", AssetType::Pdf);

        // Filter to only images
        gallery.filter = GalleryFilter::new().with_asset_type(AssetType::Image);

        gallery.add_asset(asset1);
        gallery.add_asset(asset2);

        // Should only have 1 asset (the image)
        assert_eq!(gallery.asset_count(), 1);

        // Change filter to allow all
        gallery.filter = GalleryFilter::new();
        gallery.refresh();

        // Should now have 2 assets
        assert_eq!(gallery.asset_count(), 2);
    }

    #[test]
    fn test_thumbnail_size() {
        assert_eq!(ThumbnailSize::Small.size_px(), 96);
        assert_eq!(ThumbnailSize::Medium.size_px(), 192);
        assert_eq!(ThumbnailSize::Large.size_px(), 384);
        assert_eq!(ThumbnailSize::Original.size_px(), 0);
    }

    #[test]
    fn test_gallery_theme() {
        assert_eq!(format!("{:?}", GalleryTheme::Default), "Default");
        assert_eq!(format!("{:?}", GalleryTheme::Dark), "Dark");
        assert_eq!(format!("{:?}", GalleryTheme::Light), "Light");
        assert_eq!(format!("{:?}", GalleryTheme::Compact), "Compact");
    }

    #[test]
    fn test_grid_config() {
        let config = GridConfig::default();
        assert_eq!(config.columns, 4);
        assert_eq!(config.gap, 16);
        assert_eq!(config.min_item_width, 120);
        assert_eq!(config.max_item_width, 300);
    }

    #[test]
    fn test_masonry_config() {
        let config = MasonryConfig::default();
        assert_eq!(config.columns, 4);
        assert_eq!(config.gap, 16);
        assert_eq!(config.min_item_width, 120);
    }

    #[test]
    fn test_asset_metadata_new() {
        let metadata = AssetMetadata::new("test.png", AssetType::Image);
        assert_eq!(metadata.name, "test.png");
        assert_eq!(metadata.asset_type, AssetType::Image);
        assert_eq!(metadata.extension, "png");
    }

    #[test]
    fn test_asset_metadata_with_size() {
        let metadata = AssetMetadata::new("test.png", AssetType::Image).with_size(2048);
        assert_eq!(metadata.size, 2048);
    }

    #[test]
    fn test_asset_metadata_with_url() {
        let metadata =
            AssetMetadata::new("test.png", AssetType::Image).with_url("/assets/test.png");
        assert_eq!(metadata.url, "/assets/test.png");
    }

    #[test]
    fn test_asset_metadata_with_dimensions() {
        let metadata = AssetMetadata::new("test.png", AssetType::Image).with_dimensions(800, 600);
        assert_eq!(metadata.width, Some(800));
        assert_eq!(metadata.height, Some(600));
    }

    #[test]
    fn test_asset_metadata_with_tag() {
        let metadata = AssetMetadata::new("test.png", AssetType::Image).with_tag("photo");
        assert_eq!(metadata.tags.len(), 1);
        assert!(metadata.tags.contains(&"photo".to_string()));
    }

    #[test]
    fn test_gallery_asset_set_selected() {
        let mut asset = create_test_asset(1, "test.png", AssetType::Image);

        asset.set_selected(true);
        assert!(asset.selected);

        asset.set_selected(false);
        assert!(!asset.selected);
    }

    #[test]
    fn test_gallery_asset_set_highlighted() {
        let mut asset = create_test_asset(1, "test.png", AssetType::Image);

        asset.set_highlighted(true);
        assert!(asset.highlighted);

        asset.set_highlighted(false);
        assert!(!asset.highlighted);
    }

    #[test]
    fn test_format_date() {
        // Simple test - just verify it doesn't panic
        let date = format_date(1000000);
        assert!(!date.is_empty());
        assert!(date.contains('-'));
    }
}
