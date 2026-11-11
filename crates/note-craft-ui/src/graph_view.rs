//! Graph View Module
//!
//! This module provides a basic graph visualization system for Note Craft.
//! It displays nodes and edges representing the knowledge graph structure.

use std::collections::{HashMap, HashSet};

/// Represents a node in the graph
#[derive(Debug, Clone, PartialEq)]
pub struct GraphNode {
    /// Unique identifier for the node
    pub id: String,
    /// Display name of the node
    pub label: String,
    /// X coordinate for positioning
    pub x: f32,
    /// Y coordinate for positioning
    pub y: f32,
    /// Radius of the node
    pub radius: f32,
    /// Color of the node (in hex format, e.g., "#FF0000")
    pub color: String,
    /// Whether the node is selected
    pub selected: bool,
    /// Whether the node is hovered
    pub hovered: bool,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl GraphNode {
    /// Creates a new graph node
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            x: 0.0,
            y: 0.0,
            radius: 20.0,
            color: "#4A90D9".to_string(),
            selected: false,
            hovered: false,
            metadata: HashMap::new(),
        }
    }

    /// Sets the position of the node
    pub fn with_position(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    /// Sets the radius of the node
    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    /// Sets the color of the node
    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.color = color.into();
        self
    }

    /// Adds metadata to the node
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Checks if the node contains a point
    pub fn contains_point(&self, px: f32, py: f32) -> bool {
        let dx = px - self.x;
        let dy = py - self.y;
        (dx * dx + dy * dy) <= (self.radius * self.radius)
    }

    /// Gets the distance to another node
    pub fn distance_to(&self, other: &Self) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        (dx * dx + dy * dy).sqrt()
    }
}

/// Represents an edge between two nodes
#[derive(Debug, Clone, PartialEq)]
pub struct GraphEdge {
    /// Source node ID
    pub source: String,
    /// Target node ID
    pub target: String,
    /// Edge weight (for weighted graphs)
    pub weight: f32,
    /// Color of the edge
    pub color: String,
    /// Width of the edge
    pub width: f32,
    /// Whether the edge is directed
    pub directed: bool,
    /// Whether the edge is selected
    pub selected: bool,
    /// Whether the edge is hovered
    pub hovered: bool,
    /// Edge label
    pub label: Option<String>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl GraphEdge {
    /// Creates a new graph edge
    pub fn new(source: impl Into<String>, target: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            target: target.into(),
            weight: 1.0,
            color: "#999999".to_string(),
            width: 1.5,
            directed: true,
            selected: false,
            hovered: false,
            label: None,
            metadata: HashMap::new(),
        }
    }

    /// Creates a new undirected edge
    pub fn undirected(source: impl Into<String>, target: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            target: target.into(),
            weight: 1.0,
            color: "#999999".to_string(),
            width: 1.5,
            directed: false,
            selected: false,
            hovered: false,
            label: None,
            metadata: HashMap::new(),
        }
    }

    /// Sets the weight of the edge
    pub fn with_weight(mut self, weight: f32) -> Self {
        self.weight = weight;
        self
    }

    /// Sets the color of the edge
    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.color = color.into();
        self
    }

    /// Sets the width of the edge
    pub fn with_width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Sets the label of the edge
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Adds metadata to the edge
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Gets the reverse edge (swapped source and target)
    pub fn reverse(&self) -> Self {
        Self {
            source: self.target.clone(),
            target: self.source.clone(),
            weight: self.weight,
            color: self.color.clone(),
            width: self.width,
            directed: self.directed,
            selected: self.selected,
            hovered: self.hovered,
            label: self.label.clone(),
            metadata: self.metadata.clone(),
        }
    }
}

/// Represents a graph with nodes and edges
#[derive(Debug, Clone)]
pub struct Graph {
    /// All nodes in the graph
    pub nodes: HashMap<String, GraphNode>,
    /// All edges in the graph
    pub edges: Vec<GraphEdge>,
    /// Whether the graph is directed
    pub directed: bool,
    /// Layout algorithm to use
    pub layout_algorithm: LayoutAlgorithm,
}

impl Graph {
    /// Creates a new empty graph
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
            directed: true,
            layout_algorithm: LayoutAlgorithm::ForceDirected,
        }
    }

    /// Creates a new directed graph
    pub fn directed() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
            directed: true,
            layout_algorithm: LayoutAlgorithm::ForceDirected,
        }
    }

    /// Creates a new undirected graph
    pub fn undirected() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
            directed: false,
            layout_algorithm: LayoutAlgorithm::ForceDirected,
        }
    }

    /// Sets the layout algorithm
    pub fn with_layout(mut self, algorithm: LayoutAlgorithm) -> Self {
        self.layout_algorithm = algorithm;
        self
    }

    /// Adds a node to the graph
    pub fn add_node(&mut self, node: GraphNode) -> &mut Self {
        self.nodes.insert(node.id.clone(), node);
        self
    }

    /// Removes a node from the graph
    pub fn remove_node(&mut self, node_id: &str) -> Option<GraphNode> {
        self.nodes.remove(node_id)
    }

    /// Adds an edge to the graph
    pub fn add_edge(&mut self, edge: GraphEdge) -> &mut Self {
        self.edges.push(edge);
        self
    }

    /// Removes an edge from the graph
    pub fn remove_edge(&mut self, source: &str, target: &str) -> Option<GraphEdge> {
        let idx = self
            .edges
            .iter()
            .position(|e| e.source == source && e.target == target);
        idx.map(|i| self.edges.remove(i))
    }

    /// Gets a node by ID
    pub fn get_node(&self, node_id: &str) -> Option<&GraphNode> {
        self.nodes.get(node_id)
    }

    /// Gets a mutable node by ID
    pub fn get_node_mut(&mut self, node_id: &str) -> Option<&mut GraphNode> {
        self.nodes.get_mut(node_id)
    }

    /// Checks if a node exists
    pub fn has_node(&self, node_id: &str) -> bool {
        self.nodes.contains_key(node_id)
    }

    /// Gets all edges from a node
    pub fn get_edges_from(&self, node_id: &str) -> Vec<&GraphEdge> {
        self.edges.iter().filter(|e| e.source == node_id).collect()
    }

    /// Gets all edges to a node
    pub fn get_edges_to(&self, node_id: &str) -> Vec<&GraphEdge> {
        self.edges.iter().filter(|e| e.target == node_id).collect()
    }

    /// Gets all neighbors of a node
    pub fn get_neighbors(&self, node_id: &str) -> Vec<String> {
        let mut neighbors = HashSet::new();

        for edge in &self.edges {
            if edge.source == node_id {
                neighbors.insert(edge.target.clone());
            }
            if !self.directed && edge.target == node_id {
                neighbors.insert(edge.source.clone());
            }
        }

        neighbors.into_iter().collect()
    }

    /// Gets the degree of a node (number of edges)
    pub fn get_degree(&self, node_id: &str) -> usize {
        self.get_neighbors(node_id).len()
    }

    /// Clears all nodes and edges
    pub fn clear(&mut self) -> &mut Self {
        self.nodes.clear();
        self.edges.clear();
        self
    }

    /// Gets the number of nodes
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Gets the number of edges
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Applies a layout algorithm to position the nodes
    pub fn apply_layout(&mut self) {
        match self.layout_algorithm {
            LayoutAlgorithm::ForceDirected => self.apply_force_directed_layout(),
            LayoutAlgorithm::Circular => self.apply_circular_layout(),
            LayoutAlgorithm::Grid => self.apply_grid_layout(),
            LayoutAlgorithm::Hierarchical => self.apply_hierarchical_layout(),
        }
    }

    /// Applies force-directed layout (Fruchterman-Reingold)
    fn apply_force_directed_layout(&mut self) {
        if self.nodes.is_empty() {
            return;
        }

        // Force-directed layout parameters
        let width = 800.0;
        let height = 600.0;
        let k = 2.0; // Repulsion constant
        let iterations = 50;
        let temperature = 100.0;

        // Initialize positions if not set (deterministic layout)
        if !self.nodes.is_empty() {
            let node_ids: Vec<String> = self.nodes.keys().cloned().collect();
            for (i, node_id) in node_ids.iter().enumerate() {
                if let Some(node) = self.nodes.get_mut(node_id) {
                    if node.x == 0.0 && node.y == 0.0 {
                        let angle = 2.0 * std::f32::consts::PI * i as f32 / node_ids.len() as f32;
                        node.x = 0.5 * width * angle.cos();
                        node.y = 0.5 * height * angle.sin();
                    }
                }
            }
        }

        for _ in 0..iterations {
            let mut displacements: HashMap<String, (f32, f32)> = HashMap::new();

            // Calculate repulsive forces between all nodes
            let node_ids: Vec<String> = self.nodes.keys().cloned().collect();
            for i in 0..node_ids.len() {
                for j in i + 1..node_ids.len() {
                    let id1 = &node_ids[i];
                    let id2 = &node_ids[j];
                    let node1 = self.nodes.get(id1).unwrap();
                    let node2 = self.nodes.get(id2).unwrap();

                    let dx = node2.x - node1.x;
                    let dy = node2.y - node1.y;
                    let distance = (dx * dx + dy * dy).sqrt();

                    if distance > 0.0 {
                        let repulsion = k * k / distance;
                        let fx = (dx / distance) * repulsion;
                        let fy = (dy / distance) * repulsion;

                        let entry1 = displacements.entry(id1.clone()).or_insert((0.0, 0.0));
                        entry1.0 -= fx;
                        entry1.1 -= fy;

                        let entry2 = displacements.entry(id2.clone()).or_insert((0.0, 0.0));
                        entry2.0 += fx;
                        entry2.1 += fy;
                    }
                }
            }

            // Calculate attractive forces along edges
            for edge in &self.edges {
                if let (Some(node1), Some(node2)) =
                    (self.nodes.get(&edge.source), self.nodes.get(&edge.target))
                {
                    let dx = node2.x - node1.x;
                    let dy = node2.y - node1.y;
                    let distance = (dx * dx + dy * dy).sqrt();

                    if distance > 0.0 {
                        let attraction = (distance * distance) / k;
                        let fx = (dx / distance) * attraction * edge.weight;
                        let fy = (dy / distance) * attraction * edge.weight;

                        let entry_source = displacements
                            .entry(edge.source.clone())
                            .or_insert((0.0, 0.0));
                        entry_source.0 += fx;
                        entry_source.1 += fy;

                        let entry_target = displacements
                            .entry(edge.target.clone())
                            .or_insert((0.0, 0.0));
                        entry_target.0 -= fx;
                        entry_target.1 -= fy;
                    }
                }
            }

            // Apply displacements with temperature limiting
            for (node_id, node) in &mut self.nodes {
                if let Some((dx, dy)) = displacements.get(node_id) {
                    let displacement = (dx * dx + dy * dy).sqrt();
                    if displacement > 0.0 {
                        let scale = (temperature / displacement).min(1.0);
                        node.x += dx * scale;
                        node.y += dy * scale;

                        // Keep within bounds
                        node.x = node.x.clamp(-width / 2.0, width / 2.0);
                        node.y = node.y.clamp(-height / 2.0, height / 2.0);
                    }
                }
            }

            // Cool down
            // temperature *= 0.95;
        }
    }

    /// Applies circular layout
    fn apply_circular_layout(&mut self) {
        if self.nodes.is_empty() {
            return;
        }

        let center_x = 0.0;
        let center_y = 0.0;
        let radius = 200.0;

        let node_ids: Vec<String> = self.nodes.keys().cloned().collect();
        for (i, node_id) in node_ids.iter().enumerate() {
            let angle = 2.0 * std::f32::consts::PI * i as f32 / node_ids.len() as f32;
            if let Some(node) = self.nodes.get_mut(node_id) {
                node.x = center_x + radius * angle.cos();
                node.y = center_y + radius * angle.sin();
            }
        }
    }

    /// Applies grid layout
    fn apply_grid_layout(&mut self) {
        if self.nodes.is_empty() {
            return;
        }

        let spacing = 100.0;
        let node_ids: Vec<String> = self.nodes.keys().cloned().collect();

        for (i, node_id) in node_ids.iter().enumerate() {
            let cols = (node_ids.len() as f32).sqrt().ceil() as usize;
            let row = i / cols;
            let col = i % cols;

            if let Some(node) = self.nodes.get_mut(node_id) {
                node.x = col as f32 * spacing;
                node.y = row as f32 * spacing;
            }
        }
    }

    /// Applies hierarchical layout (for tree-like structures)
    fn apply_hierarchical_layout(&mut self) {
        // Simple implementation: place root at top, children below
        // This is a placeholder for a more sophisticated implementation
        if self.nodes.is_empty() {
            return;
        }

        let node_ids: Vec<String> = self.nodes.keys().cloned().collect();
        let spacing = 100.0;

        for (i, node_id) in node_ids.iter().enumerate() {
            if let Some(node) = self.nodes.get_mut(node_id) {
                node.x = 0.0;
                node.y = i as f32 * spacing;
            }
        }
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

/// Layout algorithms for graph visualization
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutAlgorithm {
    /// Force-directed layout (Fruchterman-Reingold)
    ForceDirected,
    /// Circular layout
    Circular,
    /// Grid layout
    Grid,
    /// Hierarchical layout
    Hierarchical,
}

impl std::fmt::Display for LayoutAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LayoutAlgorithm::ForceDirected => write!(f, "Force Directed"),
            LayoutAlgorithm::Circular => write!(f, "Circular"),
            LayoutAlgorithm::Grid => write!(f, "Grid"),
            LayoutAlgorithm::Hierarchical => write!(f, "Hierarchical"),
        }
    }
}

/// Configuration for the graph view
#[derive(Debug, Clone)]
pub struct GraphViewConfig {
    /// Background color
    pub background_color: String,
    /// Default node color
    pub default_node_color: String,
    /// Default edge color
    pub default_edge_color: String,
    /// Node radius
    pub node_radius: f32,
    /// Edge width
    pub edge_width: f32,
    /// Whether to show labels
    pub show_labels: bool,
    /// Whether to show edge labels
    pub show_edge_labels: bool,
    /// Whether to enable zoom
    pub enable_zoom: bool,
    /// Whether to enable pan
    pub enable_pan: bool,
    /// Whether to enable node selection
    pub enable_selection: bool,
    /// Whether to enable hover effects
    pub enable_hover: bool,
    /// Zoom level (1.0 = 100%)
    pub zoom: f32,
    /// Pan offset
    pub pan_offset: (f32, f32),
}

impl Default for GraphViewConfig {
    fn default() -> Self {
        Self {
            background_color: "#1E1E1E".to_string(),
            default_node_color: "#4A90D9".to_string(),
            default_edge_color: "#999999".to_string(),
            node_radius: 20.0,
            edge_width: 1.5,
            show_labels: true,
            show_edge_labels: false,
            enable_zoom: true,
            enable_pan: true,
            enable_selection: true,
            enable_hover: true,
            zoom: 1.0,
            pan_offset: (0.0, 0.0),
        }
    }
}

impl GraphViewConfig {
    /// Creates a new configuration with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the background color
    pub fn with_background_color(mut self, color: impl Into<String>) -> Self {
        self.background_color = color.into();
        self
    }

    /// Sets the default node color
    pub fn with_node_color(mut self, color: impl Into<String>) -> Self {
        self.default_node_color = color.into();
        self
    }

    /// Sets the default edge color
    pub fn with_edge_color(mut self, color: impl Into<String>) -> Self {
        self.default_edge_color = color.into();
        self
    }

    /// Sets the node radius
    pub fn with_node_radius(mut self, radius: f32) -> Self {
        self.node_radius = radius;
        self
    }

    /// Sets the edge width
    pub fn with_edge_width(mut self, width: f32) -> Self {
        self.edge_width = width;
        self
    }

    /// Enables or disables labels
    pub fn with_labels(mut self, show: bool) -> Self {
        self.show_labels = show;
        self
    }

    /// Enables or disables edge labels
    pub fn with_edge_labels(mut self, show: bool) -> Self {
        self.show_edge_labels = show;
        self
    }

    /// Sets zoom level
    pub fn with_zoom(mut self, zoom: f32) -> Self {
        self.zoom = zoom.clamp(0.1, 10.0);
        self
    }

    /// Sets pan offset
    pub fn with_pan_offset(mut self, x: f32, y: f32) -> Self {
        self.pan_offset = (x, y);
        self
    }
}

/// Interaction events for the graph view
#[derive(Debug, Clone, PartialEq)]
pub enum GraphViewEvent {
    /// Node was clicked
    NodeClicked(String),
    /// Edge was clicked
    EdgeClicked(String, String),
    /// Node was selected
    NodeSelected(String),
    /// Edge was selected
    EdgeSelected(String, String),
    /// Node was hovered
    NodeHovered(String),
    /// Edge was hovered
    EdgeHovered(String, String),
    /// Background was clicked
    BackgroundClicked,
    /// Zoom changed
    ZoomChanged(f32),
    /// Pan changed
    PanChanged(f32, f32),
    /// Layout changed
    LayoutChanged(LayoutAlgorithm),
    /// Node was dragged
    NodeDragged(String, f32, f32),
}

/// Manager for the graph view
pub struct GraphView {
    /// The graph to display
    pub graph: Graph,
    /// Configuration
    pub config: GraphViewConfig,
    /// Selected node IDs
    pub selected_nodes: HashSet<String>,
    /// Selected edge indices
    pub selected_edges: HashSet<usize>,
    /// Hovered node ID
    pub hovered_node: Option<String>,
    /// Hovered edge index
    pub hovered_edge: Option<usize>,
    /// Event handler
    pub event_handler: Option<Box<dyn Fn(GraphViewEvent) + Send + Sync>>,
}

impl GraphView {
    /// Creates a new graph view
    pub fn new(graph: Graph) -> Self {
        Self {
            graph,
            config: GraphViewConfig::default(),
            selected_nodes: HashSet::new(),
            selected_edges: HashSet::new(),
            hovered_node: None,
            hovered_edge: None,
            event_handler: None,
        }
    }

    /// Creates a new graph view with a configuration
    pub fn with_config(graph: Graph, config: GraphViewConfig) -> Self {
        Self {
            graph,
            config,
            selected_nodes: HashSet::new(),
            selected_edges: HashSet::new(),
            hovered_node: None,
            hovered_edge: None,
            event_handler: None,
        }
    }

    /// Sets the event handler
    pub fn set_event_handler<F>(&mut self, handler: F) -> &mut Self
    where
        F: Fn(GraphViewEvent) + Send + Sync + 'static,
    {
        self.event_handler = Some(Box::new(handler));
        self
    }

    /// Handles a node click
    pub fn handle_node_click(&mut self, node_id: &str) {
        if !self.config.enable_selection {
            return;
        }

        if self.selected_nodes.contains(node_id) {
            self.selected_nodes.remove(node_id);
        } else {
            // Clear selection if Ctrl is not pressed (simulated)
            self.selected_nodes.clear();
            self.selected_nodes.insert(node_id.to_string());
        }

        // Update node state
        if let Some(node) = self.graph.nodes.get_mut(node_id) {
            node.selected = self.selected_nodes.contains(node_id);
        }

        if let Some(handler) = &self.event_handler {
            handler(GraphViewEvent::NodeClicked(node_id.to_string()));
            if self.selected_nodes.contains(node_id) {
                handler(GraphViewEvent::NodeSelected(node_id.to_string()));
            }
        }
    }

    /// Handles an edge click
    pub fn handle_edge_click(&mut self, source: &str, target: &str) {
        if !self.config.enable_selection {
            return;
        }

        // Find the edge
        for (i, edge) in self.graph.edges.iter_mut().enumerate() {
            if edge.source == source && edge.target == target {
                if self.selected_edges.contains(&i) {
                    self.selected_edges.remove(&i);
                } else {
                    self.selected_edges.clear();
                    self.selected_edges.insert(i);
                }

                edge.selected = self.selected_edges.contains(&i);

                if let Some(handler) = &self.event_handler {
                    handler(GraphViewEvent::EdgeClicked(
                        source.to_string(),
                        target.to_string(),
                    ));
                    if self.selected_edges.contains(&i) {
                        handler(GraphViewEvent::EdgeSelected(
                            source.to_string(),
                            target.to_string(),
                        ));
                    }
                }
                break;
            }
        }
    }

    /// Handles a background click
    pub fn handle_background_click(&mut self) {
        if self.config.enable_selection {
            // Clear selection
            for node in self.graph.nodes.values_mut() {
                node.selected = false;
            }
            for edge in self.graph.edges.iter_mut() {
                edge.selected = false;
            }
            self.selected_nodes.clear();
            self.selected_edges.clear();
        }

        if let Some(handler) = &self.event_handler {
            handler(GraphViewEvent::BackgroundClicked);
        }
    }

    /// Handles a node hover
    pub fn handle_node_hover(&mut self, node_id: &str) {
        if !self.config.enable_hover {
            return;
        }

        self.hovered_node = Some(node_id.to_string());

        if let Some(node) = self.graph.nodes.get_mut(node_id) {
            node.hovered = true;
        }

        if let Some(handler) = &self.event_handler {
            handler(GraphViewEvent::NodeHovered(node_id.to_string()));
        }
    }

    /// Handles the end of a node hover
    pub fn handle_node_hover_end(&mut self, node_id: &str) {
        if !self.config.enable_hover {
            return;
        }

        self.hovered_node = None;

        if let Some(node) = self.graph.nodes.get_mut(node_id) {
            node.hovered = false;
        }
    }

    /// Handles zoom
    pub fn handle_zoom(&mut self, delta: f32) {
        if !self.config.enable_zoom {
            return;
        }

        self.config.zoom = (self.config.zoom + delta).clamp(0.1, 10.0);

        if let Some(handler) = &self.event_handler {
            handler(GraphViewEvent::ZoomChanged(self.config.zoom));
        }
    }

    /// Handles pan
    pub fn handle_pan(&mut self, dx: f32, dy: f32) {
        if !self.config.enable_pan {
            return;
        }

        self.config.pan_offset.0 += dx;
        self.config.pan_offset.1 += dy;

        if let Some(handler) = &self.event_handler {
            handler(GraphViewEvent::PanChanged(dx, dy));
        }
    }

    /// Sets the layout algorithm
    pub fn set_layout(&mut self, algorithm: LayoutAlgorithm) -> &mut Self {
        self.graph.layout_algorithm = algorithm;
        self.graph.apply_layout();

        if let Some(handler) = &self.event_handler {
            handler(GraphViewEvent::LayoutChanged(algorithm));
        }

        self
    }

    /// Applies the current layout
    pub fn apply_layout(&mut self) -> &mut Self {
        self.graph.apply_layout();
        self
    }

    /// Adds a node
    pub fn add_node(&mut self, node: GraphNode) -> &mut Self {
        self.graph.add_node(node);
        self
    }

    /// Adds an edge
    pub fn add_edge(&mut self, edge: GraphEdge) -> &mut Self {
        self.graph.add_edge(edge);
        self
    }

    /// Gets the graph
    pub fn graph(&self) -> &Graph {
        &self.graph
    }

    /// Gets the mutable graph
    pub fn graph_mut(&mut self) -> &mut Graph {
        &mut self.graph
    }

    /// Gets the configuration
    pub fn config(&self) -> &GraphViewConfig {
        &self.config
    }

    /// Gets the mutable configuration
    pub fn config_mut(&mut self) -> &mut GraphViewConfig {
        &mut self.config
    }
}

impl Default for GraphView {
    fn default() -> Self {
        Self::new(Graph::new())
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_node_new() {
        let node = GraphNode::new("node1", "Node 1");
        assert_eq!(node.id, "node1");
        assert_eq!(node.label, "Node 1");
        assert_eq!(node.x, 0.0);
        assert_eq!(node.y, 0.0);
        assert_eq!(node.radius, 20.0);
    }

    #[test]
    fn test_graph_node_with_position() {
        let node = GraphNode::new("node1", "Node 1").with_position(100.0, 200.0);
        assert_eq!(node.x, 100.0);
        assert_eq!(node.y, 200.0);
    }

    #[test]
    fn test_graph_node_contains_point() {
        let node = GraphNode::new("node1", "Node 1")
            .with_position(100.0, 100.0)
            .with_radius(20.0);

        // Point inside the node
        assert!(node.contains_point(100.0, 100.0));
        assert!(node.contains_point(110.0, 100.0));

        // Point outside the node
        assert!(!node.contains_point(150.0, 100.0));
    }

    #[test]
    fn test_graph_node_distance_to() {
        let node1 = GraphNode::new("node1", "Node 1").with_position(0.0, 0.0);
        let node2 = GraphNode::new("node2", "Node 2").with_position(3.0, 4.0);

        assert_eq!(node1.distance_to(&node2), 5.0);
    }

    #[test]
    fn test_graph_edge_new() {
        let edge = GraphEdge::new("node1", "node2");
        assert_eq!(edge.source, "node1");
        assert_eq!(edge.target, "node2");
        assert_eq!(edge.weight, 1.0);
        assert!(edge.directed);
    }

    #[test]
    fn test_graph_edge_undirected() {
        let edge = GraphEdge::undirected("node1", "node2");
        assert_eq!(edge.source, "node1");
        assert_eq!(edge.target, "node2");
        assert!(!edge.directed);
    }

    #[test]
    fn test_graph_edge_reverse() {
        let edge = GraphEdge::new("node1", "node2").with_weight(2.5);
        let reversed = edge.reverse();

        assert_eq!(reversed.source, "node2");
        assert_eq!(reversed.target, "node1");
        assert_eq!(reversed.weight, 2.5);
    }

    #[test]
    fn test_graph_new() {
        let graph = Graph::new();
        assert!(graph.nodes.is_empty());
        assert!(graph.edges.is_empty());
        assert!(graph.directed);
    }

    #[test]
    fn test_graph_undirected() {
        let graph = Graph::undirected();
        assert!(!graph.directed);
    }

    #[test]
    fn test_graph_add_node() {
        let mut graph = Graph::new();
        let node = GraphNode::new("node1", "Node 1");

        graph.add_node(node);

        assert!(graph.has_node("node1"));
        assert_eq!(graph.node_count(), 1);
    }

    #[test]
    fn test_graph_add_edge() {
        let mut graph = Graph::new();
        graph.add_node(GraphNode::new("node1", "Node 1"));
        graph.add_node(GraphNode::new("node2", "Node 2"));

        let edge = GraphEdge::new("node1", "node2");
        graph.add_edge(edge);

        assert_eq!(graph.edge_count(), 1);
    }

    #[test]
    fn test_graph_get_neighbors() {
        let mut graph = Graph::new();
        graph.add_node(GraphNode::new("node1", "Node 1"));
        graph.add_node(GraphNode::new("node2", "Node 2"));
        graph.add_node(GraphNode::new("node3", "Node 3"));

        graph.add_edge(GraphEdge::new("node1", "node2"));
        graph.add_edge(GraphEdge::new("node1", "node3"));

        let neighbors = graph.get_neighbors("node1");
        assert_eq!(neighbors.len(), 2);
        assert!(neighbors.contains(&"node2".to_string()));
        assert!(neighbors.contains(&"node3".to_string()));
    }

    #[test]
    fn test_graph_get_degree() {
        let mut graph = Graph::new();
        graph.add_node(GraphNode::new("node1", "Node 1"));
        graph.add_node(GraphNode::new("node2", "Node 2"));
        graph.add_node(GraphNode::new("node3", "Node 3"));

        graph.add_edge(GraphEdge::new("node1", "node2"));
        graph.add_edge(GraphEdge::new("node1", "node3"));

        assert_eq!(graph.get_degree("node1"), 2);
    }

    #[test]
    fn test_graph_clear() {
        let mut graph = Graph::new();
        graph.add_node(GraphNode::new("node1", "Node 1"));
        graph.add_edge(GraphEdge::new("node1", "node2"));

        graph.clear();

        assert_eq!(graph.node_count(), 0);
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn test_graph_remove_node() {
        let mut graph = Graph::new();
        graph.add_node(GraphNode::new("node1", "Node 1"));

        let removed = graph.remove_node("node1");
        assert!(removed.is_some());
        assert_eq!(graph.node_count(), 0);
    }

    #[test]
    fn test_graph_remove_edge() {
        let mut graph = Graph::new();
        graph.add_node(GraphNode::new("node1", "Node 1"));
        graph.add_node(GraphNode::new("node2", "Node 2"));
        graph.add_edge(GraphEdge::new("node1", "node2"));

        let removed = graph.remove_edge("node1", "node2");
        assert!(removed.is_some());
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn test_graph_view_new() {
        let graph = Graph::new();
        let view = GraphView::new(graph);

        assert_eq!(view.graph.node_count(), 0);
        assert_eq!(view.graph.edge_count(), 0);
    }

    #[test]
    fn test_graph_view_handle_node_click() {
        let mut view = GraphView::new(Graph::new());
        view.add_node(GraphNode::new("node1", "Node 1"));

        view.handle_node_click("node1");

        assert!(view.selected_nodes.contains("node1"));
    }

    #[test]
    fn test_graph_view_handle_background_click() {
        let mut view = GraphView::new(Graph::new());
        view.add_node(GraphNode::new("node1", "Node 1"));
        view.handle_node_click("node1");

        view.handle_background_click();

        assert!(view.selected_nodes.is_empty());
    }

    #[test]
    fn test_graph_view_set_layout() {
        let mut view = GraphView::new(Graph::new());
        view.set_layout(LayoutAlgorithm::Circular);

        assert_eq!(view.graph.layout_algorithm, LayoutAlgorithm::Circular);
    }

    #[test]
    fn test_graph_view_config() {
        let config = GraphViewConfig::new()
            .with_background_color("#000000")
            .with_node_color("#FF0000")
            .with_node_radius(30.0);

        assert_eq!(config.background_color, "#000000");
        assert_eq!(config.default_node_color, "#FF0000");
        assert_eq!(config.node_radius, 30.0);
    }

    #[test]
    fn test_layout_algorithm_display() {
        assert_eq!(
            format!("{}", LayoutAlgorithm::ForceDirected),
            "Force Directed"
        );
        assert_eq!(format!("{}", LayoutAlgorithm::Circular), "Circular");
        assert_eq!(format!("{}", LayoutAlgorithm::Grid), "Grid");
        assert_eq!(format!("{}", LayoutAlgorithm::Hierarchical), "Hierarchical");
    }

    #[test]
    fn test_graph_apply_circular_layout() {
        let mut graph = Graph::new();
        graph.add_node(GraphNode::new("node1", "Node 1"));
        graph.add_node(GraphNode::new("node2", "Node 2"));
        graph.add_node(GraphNode::new("node3", "Node 3"));
        graph.layout_algorithm = LayoutAlgorithm::Circular;

        graph.apply_layout();

        // After layout, nodes should be positioned
        for node in graph.nodes.values() {
            assert!(node.x != 0.0 || node.y != 0.0);
        }
    }

    #[test]
    fn test_graph_apply_grid_layout() {
        let mut graph = Graph::new();
        graph.add_node(GraphNode::new("node1", "Node 1"));
        graph.add_node(GraphNode::new("node2", "Node 2"));
        graph.add_node(GraphNode::new("node3", "Node 3"));
        graph.layout_algorithm = LayoutAlgorithm::Grid;

        graph.apply_layout();

        // After layout, nodes should be positioned in a grid
        // Note: Some nodes may be at (0,0) depending on grid arrangement
        let positions: Vec<(f32, f32)> = graph.nodes.values().map(|n| (n.x, n.y)).collect();
        assert!(positions.len() >= 2);
        // At least one node should have non-zero position
        assert!(positions.iter().any(|&(x, y)| x != 0.0 || y != 0.0));
    }

    #[test]
    fn test_graph_view_event_handler() {
        use std::sync::{Arc, Mutex};

        let mut view = GraphView::new(Graph::new());
        let events = Arc::new(Mutex::new(Vec::new()));
        let events_clone = events.clone();

        view.set_event_handler(move |event| {
            events_clone.lock().unwrap().push(event);
        });

        view.add_node(GraphNode::new("node1", "Node 1"));
        view.handle_node_click("node1");

        let events = events.lock().unwrap();
        assert!(!events.is_empty());
        assert!(matches!(events[0], GraphViewEvent::NodeClicked(_)));
    }
}
