//! Note Craft State Management System
//!
//! Ce module fournit un système de gestion d'état centralisé pour Note Craft.
//!
//! Phase 1 - Étape 1.4: State Management

use crate::block::{BlockId, BlockSystem};
use crate::page::{PageId, PageSystem};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::{Arc, RwLock};

// ============================================================================
// TYPES D'ÉTAT
// ============================================================================

/// Identifiant unique pour une action d'historique
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HistoryId(pub u64);

impl Default for HistoryId {
    fn default() -> Self {
        Self::new()
    }
}

impl HistoryId {
    pub fn new() -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        HistoryId(COUNTER.fetch_add(1, Ordering::SeqCst))
    }
}

/// Type d'action pour l'historique
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HistoryActionType {
    /// Création
    Create,
    /// Modification
    Update,
    /// Suppression
    Delete,
    /// Déplacement
    Move,
    /// Fusion
    Merge,
}

/// Énumération des types d'entités qui peuvent avoir un état
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntityType {
    /// Bloc
    Block,
    /// Page
    Page,
    /// Graphe
    Graph,
    /// Configuration
    Config,
}

/// État d'un bloc
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockState {
    /// ID du bloc
    pub block_id: BlockId,
    /// Contenu (optionnel, peut être stocké séparément)
    pub content_hash: Option<u64>,
    /// Position dans la page
    pub position: usize,
    /// Visibilité
    pub is_visible: bool,
    /// Verrouillé (ne peut pas être modifié)
    pub is_locked: bool,
    /// Sélectionné
    pub is_selected: bool,
    /// Marqué comme modifié
    pub is_dirty: bool,
    /// Tags spécifiques au bloc
    pub tags: Vec<String>,
    /// Timestamp de création
    pub created_at: u64,
    /// Timestamp de modification
    pub updated_at: u64,
}

impl BlockState {
    pub fn new(block_id: BlockId) -> Self {
        let now = Self::current_timestamp();
        BlockState {
            block_id,
            content_hash: None,
            position: 0,
            is_visible: true,
            is_locked: false,
            is_selected: false,
            is_dirty: false,
            tags: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn mark_dirty(&mut self) {
        self.is_dirty = true;
        self.updated_at = Self::current_timestamp();
    }

    pub fn mark_clean(&mut self) {
        self.is_dirty = false;
    }

    fn current_timestamp() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
    }
}

/// État d'une page
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageState {
    /// ID de la page
    pub page_id: PageId,
    /// Position dans la hiérarchie
    pub position: usize,
    /// Visibilité
    pub is_visible: bool,
    /// Développée (pour l'UI)
    pub is_expanded: bool,
    /// Page active (sélectionnée)
    pub is_active: bool,
    /// Modifiée
    pub is_dirty: bool,
    /// Tags de la page
    pub tags: Vec<String>,
    /// Timestamp de création
    pub created_at: u64,
    /// Timestamp de modification
    pub updated_at: u64,
}

impl PageState {
    pub fn new(page_id: PageId) -> Self {
        let now = Self::current_timestamp();
        PageState {
            page_id,
            position: 0,
            is_visible: true,
            is_expanded: false,
            is_active: false,
            is_dirty: false,
            tags: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn mark_dirty(&mut self) {
        self.is_dirty = true;
        self.updated_at = Self::current_timestamp();
    }

    pub fn mark_clean(&mut self) {
        self.is_dirty = false;
    }

    fn current_timestamp() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
    }
}

/// État global du graphe
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GraphState {
    /// Nombre total de nœuds
    pub node_count: usize,
    /// Nombre total de liens
    pub link_count: usize,
    /// Nœuds sélectionnés
    pub selected_nodes: HashSet<PageId>,
    /// Nœud central (si applicable)
    pub center_node: Option<PageId>,
    /// Zoom niveau
    pub zoom_level: f32,
    /// Position de la caméra
    pub camera_position: (f32, f32),
    /// Mode d'affichage
    pub display_mode: GraphDisplayMode,
    /// Filtre actuel
    pub filter: Option<String>,
    /// Modifié
    pub is_dirty: bool,
    /// Timestamp de modification
    pub updated_at: u64,
}

impl Default for GraphState {
    fn default() -> Self {
        Self::new()
    }
}

impl GraphState {
    pub fn new() -> Self {
        GraphState {
            node_count: 0,
            link_count: 0,
            selected_nodes: HashSet::new(),
            center_node: None,
            zoom_level: 1.0,
            camera_position: (0.0, 0.0),
            display_mode: GraphDisplayMode::ForceDirected,
            filter: None,
            is_dirty: false,
            updated_at: Self::current_timestamp(),
        }
    }

    pub fn mark_dirty(&mut self) {
        self.is_dirty = true;
        self.updated_at = Self::current_timestamp();
    }

    fn current_timestamp() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
    }
}

/// Mode d'affichage du graphe
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GraphDisplayMode {
    /// Force-directed layout
    ForceDirected,
    /// Hiérarchique
    Hierarchical,
    /// Circulaire
    Circular,
    /// Grille
    Grid,
    /// Liste
    List,
}

// ============================================================================
// ACTIONS D'HISTORIQUE
// ============================================================================

/// Action stockée dans l'historique
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryAction {
    /// ID unique
    pub id: HistoryId,
    /// Type d'action
    pub action_type: HistoryActionType,
    /// Type d'entité
    pub entity_type: EntityType,
    /// ID de l'entité
    pub entity_id: u64,
    /// État avant l'action
    pub previous_state: Option<StateSnapshot>,
    /// État après l'action
    pub new_state: Option<StateSnapshot>,
    /// Timestamp
    pub timestamp: u64,
    /// Description
    pub description: Option<String>,
}

impl HistoryAction {
    pub fn new(action_type: HistoryActionType, entity_type: EntityType, entity_id: u64) -> Self {
        HistoryAction {
            id: HistoryId::new(),
            action_type,
            entity_type,
            entity_id,
            previous_state: None,
            new_state: None,
            timestamp: Self::current_timestamp(),
            description: None,
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn with_previous_state(mut self, state: StateSnapshot) -> Self {
        self.previous_state = Some(state);
        self
    }

    pub fn with_new_state(mut self, state: StateSnapshot) -> Self {
        self.new_state = Some(state);
        self
    }

    fn current_timestamp() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
    }
}

/// Snapshot d'un état (pour l'historique)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StateSnapshot {
    Block(BlockState),
    Page(PageState),
    Graph(GraphState),
    // Pour des états composites
    Composite(Vec<StateSnapshot>),
}

// ============================================================================
// ÉVÉNEMENTS D'ÉTAT
// ============================================================================

/// Type d'événement d'état
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateEventType {
    /// État changé
    StateChanged,
    /// État sauvegardé
    StateSaved,
    /// État chargé
    StateLoaded,
    /// Historique modifié
    HistoryChanged,
    /// État réinitialisé
    StateReset,
}

/// Événement émis par le State Manager
#[derive(Debug, Clone)]
pub struct StateEvent {
    /// Type d'événement
    pub event_type: StateEventType,
    /// Type d'entité concernée
    pub entity_type: Option<EntityType>,
    /// ID de l'entité concernée
    pub entity_id: Option<u64>,
    /// Données supplémentaires
    pub data: Option<String>,
    /// Timestamp
    pub timestamp: u64,
}

impl StateEvent {
    pub fn new(event_type: StateEventType) -> Self {
        StateEvent {
            event_type,
            entity_type: None,
            entity_id: None,
            data: None,
            timestamp: Self::current_timestamp(),
        }
    }

    pub fn with_entity(mut self, entity_type: EntityType, entity_id: u64) -> Self {
        self.entity_type = Some(entity_type);
        self.entity_id = Some(entity_id);
        self
    }

    pub fn with_data(mut self, data: String) -> Self {
        self.data = Some(data);
        self
    }

    fn current_timestamp() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
    }
}

/// Trait pour écouter les événements d'état
pub trait StateEventListener: Send + Sync {
    /// Appelé lorsqu'un événement se produit
    fn on_state_event(&self, event: &StateEvent);
}

// ============================================================================
// STATE MANAGER
// ============================================================================

/// Gestionnaire d'état principal
///
/// Ce struct gère l'état complet de l'application Note Craft.
/// Il est thread-safe et peut être partagé entre plusieurs threads.
pub struct StateManager {
    /// État des blocs
    block_states: RwLock<HashMap<BlockId, BlockState>>,
    /// État des pages
    page_states: RwLock<HashMap<PageId, PageState>>,
    /// État du graphe
    graph_state: RwLock<GraphState>,
    /// Historique des actions
    history: RwLock<Vec<HistoryAction>>,
    /// Position actuelle dans l'historique
    history_position: RwLock<usize>,
    /// Système de blocs
    block_system: Arc<RwLock<BlockSystem>>,
    /// Système de pages
    page_system: Arc<RwLock<PageSystem>>,
    /// Écouteurs d'événements
    listeners: RwLock<Vec<Arc<dyn StateEventListener>>>,
}

impl StateManager {
    /// Crée un nouveau StateManager
    pub fn new() -> Self {
        StateManager {
            block_states: RwLock::new(HashMap::new()),
            page_states: RwLock::new(HashMap::new()),
            graph_state: RwLock::new(GraphState::new()),
            history: RwLock::new(Vec::new()),
            history_position: RwLock::new(0),
            block_system: Arc::new(RwLock::new(BlockSystem::new())),
            page_system: Arc::new(RwLock::new(PageSystem::new())),
            listeners: RwLock::new(Vec::new()),
        }
    }

    /// Crée un StateManager avec des systèmes de blocs et pages existants
    pub fn with_systems(
        block_system: Arc<RwLock<BlockSystem>>,
        page_system: Arc<RwLock<PageSystem>>,
    ) -> Self {
        StateManager {
            block_states: RwLock::new(HashMap::new()),
            page_states: RwLock::new(HashMap::new()),
            graph_state: RwLock::new(GraphState::new()),
            history: RwLock::new(Vec::new()),
            history_position: RwLock::new(0),
            block_system,
            page_system,
            listeners: RwLock::new(Vec::new()),
        }
    }

    // ========================================================================
    // GESTION DES ÉTATS DE BLOC
    // ========================================================================

    /// Ajoute un état de bloc
    pub fn add_block_state(&self, block_id: BlockId, state: BlockState) {
        let mut states = self.block_states.write().unwrap();
        states.insert(block_id, state);

        let mut graph = self.graph_state.write().unwrap();
        graph.node_count += 1;
        graph.mark_dirty();

        self.emit_event(
            StateEvent::new(StateEventType::StateChanged)
                .with_entity(EntityType::Block, block_id.0),
        );
    }

    /// Récupère l'état d'un bloc
    pub fn get_block_state(&self, block_id: BlockId) -> Option<BlockState> {
        let states = self.block_states.read().unwrap();
        states.get(&block_id).cloned()
    }

    /// Met à jour l'état d'un bloc
    pub fn update_block_state(&self, block_id: BlockId, update_fn: impl Fn(&mut BlockState)) {
        let mut states = self.block_states.write().unwrap();
        if let Some(state) = states.get_mut(&block_id) {
            // Sauvegarder l'état précédent pour l'historique
            let previous = state.clone();

            // Appliquer la mise à jour
            update_fn(state);

            // Marquer comme modifié
            state.mark_dirty();

            // Ajouter à l'historique
            self.record_history(
                HistoryAction::new(HistoryActionType::Update, EntityType::Block, block_id.0)
                    .with_previous_state(StateSnapshot::Block(previous))
                    .with_new_state(StateSnapshot::Block(state.clone())),
            );

            // Notifier
            self.emit_event(
                StateEvent::new(StateEventType::StateChanged)
                    .with_entity(EntityType::Block, block_id.0),
            );
        }
    }

    /// Supprime l'état d'un bloc
    pub fn remove_block_state(&self, block_id: BlockId) -> Option<BlockState> {
        let mut states = self.block_states.write().unwrap();
        let mut graph = self.graph_state.write().unwrap();

        if let Some(state) = states.remove(&block_id) {
            graph.node_count = graph.node_count.saturating_sub(1);
            graph.mark_dirty();

            // Ajouter à l'historique
            self.record_history(
                HistoryAction::new(HistoryActionType::Delete, EntityType::Block, block_id.0)
                    .with_previous_state(StateSnapshot::Block(state.clone())),
            );

            // Notifier
            self.emit_event(
                StateEvent::new(StateEventType::StateChanged)
                    .with_entity(EntityType::Block, block_id.0),
            );

            Some(state)
        } else {
            None
        }
    }

    // ========================================================================
    // GESTION DES ÉTATS DE PAGE
    // ========================================================================

    /// Ajoute un état de page
    pub fn add_page_state(&self, page_id: PageId, state: PageState) {
        let mut states = self.page_states.write().unwrap();
        states.insert(page_id, state);

        let mut graph = self.graph_state.write().unwrap();
        graph.node_count += 1;
        graph.mark_dirty();

        self.emit_event(
            StateEvent::new(StateEventType::StateChanged).with_entity(EntityType::Page, page_id.0),
        );
    }

    /// Récupère l'état d'une page
    pub fn get_page_state(&self, page_id: PageId) -> Option<PageState> {
        let states = self.page_states.read().unwrap();
        states.get(&page_id).cloned()
    }

    /// Met à jour l'état d'une page
    pub fn update_page_state(&self, page_id: PageId, update_fn: impl Fn(&mut PageState)) {
        let mut states = self.page_states.write().unwrap();
        if let Some(state) = states.get_mut(&page_id) {
            let previous = state.clone();
            update_fn(state);
            state.mark_dirty();

            self.record_history(
                HistoryAction::new(HistoryActionType::Update, EntityType::Page, page_id.0)
                    .with_previous_state(StateSnapshot::Page(previous))
                    .with_new_state(StateSnapshot::Page(state.clone())),
            );

            self.emit_event(
                StateEvent::new(StateEventType::StateChanged)
                    .with_entity(EntityType::Page, page_id.0),
            );
        }
    }

    /// Supprime l'état d'une page
    pub fn remove_page_state(&self, page_id: PageId) -> Option<PageState> {
        let mut states = self.page_states.write().unwrap();
        let mut graph = self.graph_state.write().unwrap();

        if let Some(state) = states.remove(&page_id) {
            graph.node_count = graph.node_count.saturating_sub(1);
            graph.mark_dirty();

            self.record_history(
                HistoryAction::new(HistoryActionType::Delete, EntityType::Page, page_id.0)
                    .with_previous_state(StateSnapshot::Page(state.clone())),
            );

            self.emit_event(
                StateEvent::new(StateEventType::StateChanged)
                    .with_entity(EntityType::Page, page_id.0),
            );

            Some(state)
        } else {
            None
        }
    }

    // ========================================================================
    // GESTION DE L'ÉTAT DU GRAPHE
    // ========================================================================

    /// Récupère l'état du graphe
    pub fn get_graph_state(&self) -> GraphState {
        let state = self.graph_state.read().unwrap();
        state.clone()
    }

    /// Met à jour l'état du graphe
    pub fn update_graph_state(&self, update_fn: impl Fn(&mut GraphState)) {
        let mut state = self.graph_state.write().unwrap();
        let previous = state.clone();
        update_fn(&mut state);
        state.mark_dirty();

        self.record_history(
            HistoryAction::new(HistoryActionType::Update, EntityType::Graph, 0)
                .with_previous_state(StateSnapshot::Graph(previous))
                .with_new_state(StateSnapshot::Graph(state.clone())),
        );

        self.emit_event(
            StateEvent::new(StateEventType::StateChanged).with_entity(EntityType::Graph, 0),
        );
    }

    /// Définit le mode d'affichage du graphe
    pub fn set_graph_display_mode(&self, mode: GraphDisplayMode) {
        self.update_graph_state(|state| {
            state.display_mode = mode;
        });
    }

    /// Sélectionne un nœud dans le graphe
    pub fn select_graph_node(&self, node_id: PageId, selected: bool) {
        self.update_graph_state(|state| {
            if selected {
                state.selected_nodes.insert(node_id);
            } else {
                state.selected_nodes.remove(&node_id);
            }
        });
    }

    // ========================================================================
    // HISTORIQUE (UNDO/REDO)
    // ========================================================================

    /// Enregistre une action dans l'historique
    pub fn record_history(&self, action: HistoryAction) {
        let mut history = self.history.write().unwrap();
        let mut position = self.history_position.write().unwrap();

        // Si on n'est pas à la fin de l'historique, tronquer
        if *position < history.len() {
            history.truncate(*position);
        }

        // Ajouter la nouvelle action
        history.push(action);
        *position = history.len();

        self.emit_event(StateEvent::new(StateEventType::HistoryChanged));
    }

    /// Annule la dernière action
    pub fn undo(&self) -> Option<HistoryAction> {
        let mut position = self.history_position.write().unwrap();
        let history = self.history.read().unwrap();

        if *position == 0 {
            return None;
        }

        *position -= 1;
        let action = &history[*position];

        // Appliquer l'annulation
        self.apply_history_action(action, true);

        Some(action.clone())
    }

    /// Rétablit la dernière action annulée
    pub fn redo(&self) -> Option<HistoryAction> {
        let mut position = self.history_position.write().unwrap();
        let history = self.history.read().unwrap();

        if *position >= history.len() {
            return None;
        }

        let action = &history[*position];
        *position += 1;

        // Appliquer le rétablissement
        self.apply_history_action(action, false);

        Some(action.clone())
    }

    /// Applique une action de l'historique (undo ou redo)
    fn apply_history_action(&self, action: &HistoryAction, is_undo: bool) {
        match (
            action.entity_type,
            &action.previous_state,
            &action.new_state,
        ) {
            (
                EntityType::Block,
                Some(StateSnapshot::Block(prev)),
                Some(StateSnapshot::Block(new)),
            ) => {
                if is_undo {
                    let mut states = self.block_states.write().unwrap();
                    states.insert(BlockId(action.entity_id), prev.clone());
                } else {
                    let mut states = self.block_states.write().unwrap();
                    states.insert(BlockId(action.entity_id), new.clone());
                }
            }
            (EntityType::Page, Some(StateSnapshot::Page(prev)), Some(StateSnapshot::Page(new))) => {
                if is_undo {
                    let mut states = self.page_states.write().unwrap();
                    states.insert(PageId(action.entity_id), prev.clone());
                } else {
                    let mut states = self.page_states.write().unwrap();
                    states.insert(PageId(action.entity_id), new.clone());
                }
            }
            (
                EntityType::Graph,
                Some(StateSnapshot::Graph(prev)),
                Some(StateSnapshot::Graph(new)),
            ) => {
                if is_undo {
                    let mut state = self.graph_state.write().unwrap();
                    *state = prev.clone();
                } else {
                    let mut state = self.graph_state.write().unwrap();
                    *state = new.clone();
                }
            }
            _ => {}
        }

        self.emit_event(StateEvent::new(StateEventType::StateChanged));
    }

    /// Efface l'historique
    pub fn clear_history(&self) {
        let mut history = self.history.write().unwrap();
        let mut position = self.history_position.write().unwrap();
        history.clear();
        *position = 0;

        self.emit_event(StateEvent::new(StateEventType::HistoryChanged));
    }

    /// Obtient le nombre d'actions d'historique disponibles
    pub fn history_len(&self) -> usize {
        let history = self.history.read().unwrap();
        history.len()
    }

    /// Obtient la position actuelle dans l'historique
    pub fn history_position(&self) -> usize {
        let position = self.history_position.read().unwrap();
        *position
    }

    // ========================================================================
    // GESTION DES ÉCOUTEURS
    // ========================================================================

    /// Ajoute un écouteur d'événements
    pub fn add_listener(&self, listener: Arc<dyn StateEventListener>) {
        let mut listeners = self.listeners.write().unwrap();
        listeners.push(listener);
    }

    /// Supprime un écouteur d'événements
    pub fn remove_listener(&self, listener: &Arc<dyn StateEventListener>) {
        let mut listeners = self.listeners.write().unwrap();
        listeners.retain(|l| !Arc::ptr_eq(l, listener));
    }

    /// Émet un événement à tous les écouteurs
    fn emit_event(&self, event: StateEvent) {
        let listeners = self.listeners.read().unwrap();
        for listener in listeners.iter() {
            listener.on_state_event(&event);
        }
    }

    // ========================================================================
    // SAUVEGARDE ET CHARGEMENT
    // ========================================================================

    /// Sauvegarde l'état complet
    pub fn save_state(&self) -> AppState {
        let block_states = self.block_states.read().unwrap();
        let page_states = self.page_states.read().unwrap();
        let graph_state = self.graph_state.read().unwrap();

        AppState {
            block_states: block_states.clone(),
            page_states: page_states.clone(),
            graph_state: graph_state.clone(),
        }
    }

    /// Charge un état complet
    pub fn load_state(&self, state: AppState) {
        // Sauvegarder l'état actuel pour l'historique
        let current = self.save_state();

        {
            let mut block_states = self.block_states.write().unwrap();
            let mut page_states = self.page_states.write().unwrap();
            let mut graph_state = self.graph_state.write().unwrap();

            *block_states = state.block_states;
            *page_states = state.page_states;
            *graph_state = state.graph_state;
        }

        // Enregistrer dans l'historique
        self.record_history(
            HistoryAction::new(HistoryActionType::Update, EntityType::Graph, 0)
                .with_description("Full state loaded".to_string())
                .with_previous_state(StateSnapshot::Composite(vec![StateSnapshot::Graph(
                    current.graph_state,
                )])),
        );

        self.emit_event(StateEvent::new(StateEventType::StateLoaded));
    }

    // ========================================================================
    // UTILITAIRES
    // ========================================================================

    /// Réinitialise tout l'état
    pub fn reset(&self) {
        let mut block_states = self.block_states.write().unwrap();
        let mut page_states = self.page_states.write().unwrap();
        let mut graph_state = self.graph_state.write().unwrap();
        let mut history = self.history.write().unwrap();
        let mut position = self.history_position.write().unwrap();

        block_states.clear();
        page_states.clear();
        *graph_state = GraphState::new();
        history.clear();
        *position = 0;

        self.emit_event(StateEvent::new(StateEventType::StateReset));
    }

    /// Obtient le nombre total de blocs
    pub fn block_count(&self) -> usize {
        let states = self.block_states.read().unwrap();
        states.len()
    }

    /// Obtient le nombre total de pages
    pub fn page_count(&self) -> usize {
        let states = self.page_states.read().unwrap();
        states.len()
    }

    /// Vérifie s'il y a des changements non sauvegardés
    pub fn has_unsaved_changes(&self) -> bool {
        let block_states = self.block_states.read().unwrap();
        let page_states = self.page_states.read().unwrap();
        let graph_state = self.graph_state.read().unwrap();

        block_states.values().any(|s| s.is_dirty)
            || page_states.values().any(|s| s.is_dirty)
            || graph_state.is_dirty
    }

    /// Accès au BlockSystem sous-jacent
    pub fn block_system(&self) -> Arc<RwLock<BlockSystem>> {
        self.block_system.clone()
    }

    /// Accès au PageSystem sous-jacent
    pub fn page_system(&self) -> Arc<RwLock<PageSystem>> {
        self.page_system.clone()
    }
}

impl Default for StateManager {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for StateManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "StateManager {{ blocks: {}, pages: {}, graph_nodes: {} }}",
            self.block_count(),
            self.page_count(),
            self.get_graph_state().node_count
        )
    }
}

// ============================================================================
// ÉTAT COMPLET DE L'APPLICATION
// ============================================================================

/// État complet de l'application (pour la sérialisation)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    /// États des blocs
    pub block_states: HashMap<BlockId, BlockState>,
    /// États des pages
    pub page_states: HashMap<PageId, PageState>,
    /// État du graphe
    pub graph_state: GraphState,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            block_states: HashMap::new(),
            page_states: HashMap::new(),
            graph_state: GraphState::new(),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Mock listener pour les tests
    struct TestListener {
        events: RwLock<Vec<StateEvent>>,
    }

    impl TestListener {
        fn new() -> Arc<Self> {
            Arc::new(Self {
                events: RwLock::new(Vec::new()),
            })
        }

        fn get_events(&self) -> Vec<StateEvent> {
            let events = self.events.read().unwrap();
            events.clone()
        }

        #[allow(dead_code)]
        fn clear(&self) {
            let mut events = self.events.write().unwrap();
            events.clear();
        }
    }

    impl StateEventListener for TestListener {
        fn on_state_event(&self, event: &StateEvent) {
            let mut events = self.events.write().unwrap();
            events.push(event.clone());
        }
    }

    #[test]
    fn test_state_manager_new() {
        let manager = StateManager::new();
        assert_eq!(manager.block_count(), 0);
        assert_eq!(manager.page_count(), 0);
    }

    #[test]
    fn test_add_and_get_block_state() {
        let manager = StateManager::new();
        let block_id = BlockId::new();
        let state = BlockState::new(block_id);

        manager.add_block_state(block_id, state.clone());

        let retrieved = manager.get_block_state(block_id).unwrap();
        assert_eq!(retrieved.block_id, block_id);
        assert_eq!(manager.block_count(), 1);
    }

    #[test]
    fn test_update_block_state() {
        let manager = StateManager::new();
        let block_id = BlockId::new();
        let state = BlockState::new(block_id);

        manager.add_block_state(block_id, state);

        manager.update_block_state(block_id, |s| {
            s.is_visible = false;
        });

        let retrieved = manager.get_block_state(block_id).unwrap();
        assert!(!retrieved.is_visible);
        assert!(retrieved.is_dirty);
    }

    #[test]
    fn test_remove_block_state() {
        let manager = StateManager::new();
        let block_id = BlockId::new();
        let state = BlockState::new(block_id);

        manager.add_block_state(block_id, state);
        assert_eq!(manager.block_count(), 1);

        let removed = manager.remove_block_state(block_id);
        assert!(removed.is_some());
        assert_eq!(manager.block_count(), 0);
    }

    #[test]
    fn test_add_and_get_page_state() {
        let manager = StateManager::new();
        let page_id = PageId::new();
        let state = PageState::new(page_id);

        manager.add_page_state(page_id, state.clone());

        let retrieved = manager.get_page_state(page_id).unwrap();
        assert_eq!(retrieved.page_id, page_id);
        assert_eq!(manager.page_count(), 1);
    }

    #[test]
    fn test_update_page_state() {
        let manager = StateManager::new();
        let page_id = PageId::new();
        let state = PageState::new(page_id);

        manager.add_page_state(page_id, state);

        manager.update_page_state(page_id, |s| {
            s.is_expanded = true;
        });

        let retrieved = manager.get_page_state(page_id).unwrap();
        assert!(retrieved.is_expanded);
        assert!(retrieved.is_dirty);
    }

    #[test]
    fn test_graph_state() {
        let manager = StateManager::new();

        let state = manager.get_graph_state();
        assert_eq!(state.node_count, 0);
        assert_eq!(state.zoom_level, 1.0);

        manager.update_graph_state(|s| {
            s.zoom_level = 2.0;
        });

        let state = manager.get_graph_state();
        assert_eq!(state.zoom_level, 2.0);
    }

    #[test]
    fn test_graph_display_mode() {
        let manager = StateManager::new();

        manager.set_graph_display_mode(GraphDisplayMode::Hierarchical);

        let state = manager.get_graph_state();
        assert_eq!(state.display_mode, GraphDisplayMode::Hierarchical);
    }

    #[test]
    fn test_select_graph_node() {
        let manager = StateManager::new();
        let node_id = PageId::new();

        manager.select_graph_node(node_id, true);

        let state = manager.get_graph_state();
        assert!(state.selected_nodes.contains(&node_id));

        manager.select_graph_node(node_id, false);

        let state = manager.get_graph_state();
        assert!(!state.selected_nodes.contains(&node_id));
    }

    #[test]
    fn test_event_listener() {
        let manager = StateManager::new();
        let listener = TestListener::new();

        manager.add_listener(listener.clone());

        let block_id = BlockId::new();
        let state = BlockState::new(block_id);
        manager.add_block_state(block_id, state);

        let events = listener.get_events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type, StateEventType::StateChanged);
        assert_eq!(events[0].entity_type, Some(EntityType::Block));
    }

    #[test]
    fn test_remove_listener() {
        let manager = StateManager::new();
        let listener = TestListener::new();
        let listener_ref: Arc<dyn StateEventListener> = listener.clone();

        manager.add_listener(listener.clone());
        manager.remove_listener(&listener_ref);

        let block_id = BlockId::new();
        let state = BlockState::new(block_id);
        manager.add_block_state(block_id, state);

        let events = listener.get_events();
        assert_eq!(events.len(), 0);
    }

    #[test]
    fn test_history_record() {
        let manager = StateManager::new();

        let block_id = BlockId::new();
        let state = BlockState::new(block_id);
        manager.add_block_state(block_id, state);
        // add_block_state n'enregistre pas dans l'historique actuellement
        // Mais update_block_state oui
        manager.update_block_state(block_id, |s| {
            s.is_visible = false;
        });

        assert_eq!(manager.history_len(), 1);
    }

    #[test]
    fn test_undo_redo() {
        let manager = StateManager::new();
        let block_id = BlockId::new();
        let initial_state = BlockState::new(block_id);

        manager.add_block_state(block_id, initial_state);

        // Modifier l'état (c'est ça qui crée l'historique)
        manager.update_block_state(block_id, |s| {
            s.is_visible = false;
        });

        // 1 action dans l'historique (la modification)
        assert_eq!(manager.history_len(), 1);

        // Vérifier que l'état a changé
        let state = manager.get_block_state(block_id).unwrap();
        assert!(!state.is_visible);

        // Annuler
        let undone = manager.undo();
        assert!(undone.is_some());

        // Vérifier que la position a été modifiée
        assert_eq!(manager.history_position(), 0);

        // Rétablir
        let redone = manager.redo();
        assert!(redone.is_some());
        assert_eq!(manager.history_position(), 1);
    }

    #[test]
    fn test_save_and_load_state() {
        let manager = StateManager::new();
        let block_id = BlockId::new();
        let state = BlockState::new(block_id);
        manager.add_block_state(block_id, state);

        let saved = manager.save_state();
        assert_eq!(saved.block_states.len(), 1);

        // Créer un nouveau manager et charger l'état
        let manager2 = StateManager::new();
        manager2.load_state(saved);

        assert_eq!(manager2.block_count(), 1);
    }

    #[test]
    fn test_has_unsaved_changes() {
        let manager = StateManager::new();

        assert!(!manager.has_unsaved_changes());

        let block_id = BlockId::new();
        let mut state = BlockState::new(block_id);
        state.mark_dirty();
        manager.add_block_state(block_id, state);

        assert!(manager.has_unsaved_changes());
    }

    #[test]
    fn test_reset() {
        let manager = StateManager::new();
        let block_id = BlockId::new();
        let state = BlockState::new(block_id);
        manager.add_block_state(block_id, state);

        assert_eq!(manager.block_count(), 1);

        manager.reset();

        assert_eq!(manager.block_count(), 0);
        assert_eq!(manager.page_count(), 0);
        assert_eq!(manager.history_len(), 0);
    }

    #[test]
    fn test_block_state_mark_dirty_clean() {
        let mut state = BlockState::new(BlockId::new());
        assert!(!state.is_dirty);

        state.mark_dirty();
        assert!(state.is_dirty);

        state.mark_clean();
        assert!(!state.is_dirty);
    }

    #[test]
    fn test_page_state_mark_dirty_clean() {
        let mut state = PageState::new(PageId::new());
        assert!(!state.is_dirty);

        state.mark_dirty();
        assert!(state.is_dirty);

        state.mark_clean();
        assert!(!state.is_dirty);
    }

    #[test]
    fn test_graph_state_mark_dirty() {
        let mut state = GraphState::new();
        assert!(!state.is_dirty);

        state.mark_dirty();
        assert!(state.is_dirty);
    }

    #[test]
    fn test_app_state_new() {
        let state = AppState::new();
        assert!(state.block_states.is_empty());
        assert!(state.page_states.is_empty());
        assert_eq!(state.graph_state.node_count, 0);
    }

    #[test]
    fn test_history_action_new() {
        let action = HistoryAction::new(HistoryActionType::Create, EntityType::Block, 1);

        assert_eq!(action.action_type, HistoryActionType::Create);
        assert_eq!(action.entity_type, EntityType::Block);
        assert_eq!(action.entity_id, 1);
    }

    #[test]
    fn test_state_event_new() {
        let event = StateEvent::new(StateEventType::StateChanged);
        assert_eq!(event.event_type, StateEventType::StateChanged);
        assert!(event.entity_type.is_none());
        assert!(event.entity_id.is_none());
    }
}
