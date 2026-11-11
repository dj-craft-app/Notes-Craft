//! Note Craft Configuration System
//!
//! Ce module fournit un système de configuration hiérarchique pour Note Craft.
//!
//! Phase 1 - Étape 1.5: Configuration System

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use thiserror::Error;

// ============================================================================
// TYPES DE CONFIGURATION
// ============================================================================

/// Niveau de configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ConfigLevel {
    /// Configuration par défaut (compile-time)
    Default = 0,
    /// Configuration système (fichier global)
    System = 1,
    /// Configuration utilisateur
    User = 2,
    /// Configuration projet
    Project = 3,
    /// Configuration temporaire (runtime)
    Runtime = 4,
}

impl fmt::Display for ConfigLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigLevel::Default => write!(f, "default"),
            ConfigLevel::System => write!(f, "system"),
            ConfigLevel::User => write!(f, "user"),
            ConfigLevel::Project => write!(f, "project"),
            ConfigLevel::Runtime => write!(f, "runtime"),
        }
    }
}

/// Format de fichier de configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ConfigFormat {
    /// JSON
    #[default]
    Json,
    /// TOML
    Toml,
    /// YAML
    Yaml,
}

impl ConfigFormat {
    /// Extension du fichier
    pub fn extension(&self) -> &'static str {
        match self {
            ConfigFormat::Json => "json",
            ConfigFormat::Toml => "toml",
            ConfigFormat::Yaml => "yaml",
        }
    }

    /// Charge la configuration depuis un chemin
    pub fn load_from_path<P: AsRef<Path>>(&self, path: P) -> Result<Config, ConfigError> {
        let content = fs::read_to_string(path)?;
        self.load_from_str(&content)
    }

    /// Charge la configuration depuis une string
    pub fn load_from_str(&self, content: &str) -> Result<Config, ConfigError> {
        match self {
            ConfigFormat::Json => serde_json::from_str(content)
                .map_err(|e| ConfigError::ParseError(format!("JSON parse error: {}", e))),
            ConfigFormat::Toml => toml::from_str(content)
                .map_err(|e| ConfigError::ParseError(format!("TOML parse error: {}", e))),
            ConfigFormat::Yaml => serde_yaml::from_str(content)
                .map_err(|e| ConfigError::ParseError(format!("YAML parse error: {}", e))),
        }
    }

    /// Sauvegarde la configuration dans un fichier
    pub fn save_to_path<P: AsRef<Path>>(
        &self,
        config: &Config,
        path: P,
    ) -> Result<(), ConfigError> {
        let content = self.save_to_str(config)?;
        fs::write(path, content)?;
        Ok(())
    }

    /// Convertit la configuration en string
    pub fn save_to_str(&self, config: &Config) -> Result<String, ConfigError> {
        match self {
            ConfigFormat::Json => serde_json::to_string_pretty(config)
                .map_err(|e| ConfigError::SerializeError(format!("JSON serialize error: {}", e))),
            ConfigFormat::Toml => toml::to_string_pretty(config)
                .map_err(|e| ConfigError::SerializeError(format!("TOML serialize error: {}", e))),
            ConfigFormat::Yaml => serde_yaml::to_string(config)
                .map_err(|e| ConfigError::SerializeError(format!("YAML serialize error: {}", e))),
        }
    }
}

// ============================================================================
// VALEURS DE CONFIGURATION
// ============================================================================

/// Valeur de configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConfigValue {
    /// Null
    Null,
    /// Booléen
    Bool(bool),
    /// Entier
    Int(i64),
    /// Flottant
    Float(f64),
    /// Chaîne de caractères
    String(String),
    /// Tableau
    Array(Vec<ConfigValue>),
    /// Objet
    Object(HashMap<String, ConfigValue>),
}

impl ConfigValue {
    /// Crée un ConfigValue depuis une valeur primitive
    pub fn from_value<T: Into<ConfigValue>>(value: T) -> Self {
        value.into()
    }

    /// Convertit en booléen
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            ConfigValue::Bool(v) => Some(*v),
            _ => None,
        }
    }

    /// Convertit en i64
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            ConfigValue::Int(v) => Some(*v),
            _ => None,
        }
    }

    /// Convertit en f64
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            ConfigValue::Float(v) => Some(*v),
            ConfigValue::Int(v) => Some(*v as f64),
            _ => None,
        }
    }

    /// Convertit en String
    pub fn as_str(&self) -> Option<&str> {
        match self {
            ConfigValue::String(v) => Some(v),
            _ => None,
        }
    }

    /// Convertit en Vec<ConfigValue>
    pub fn as_array(&self) -> Option<&Vec<ConfigValue>> {
        match self {
            ConfigValue::Array(v) => Some(v),
            _ => None,
        }
    }

    /// Convertit en HashMap<String, ConfigValue>
    pub fn as_object(&self) -> Option<&HashMap<String, ConfigValue>> {
        match self {
            ConfigValue::Object(v) => Some(v),
            _ => None,
        }
    }

    /// Obtient une valeur par clé (pour les objets)
    pub fn get(&self, key: &str) -> Option<&ConfigValue> {
        match self {
            ConfigValue::Object(map) => map.get(key),
            _ => None,
        }
    }

    /// Vérifie si la valeur est null
    pub fn is_null(&self) -> bool {
        matches!(self, ConfigValue::Null)
    }
}

// Implémentations From/Into pour les types primitifs
impl From<bool> for ConfigValue {
    fn from(value: bool) -> Self {
        ConfigValue::Bool(value)
    }
}

impl From<i64> for ConfigValue {
    fn from(value: i64) -> Self {
        ConfigValue::Int(value)
    }
}

impl From<f64> for ConfigValue {
    fn from(value: f64) -> Self {
        ConfigValue::Float(value)
    }
}

impl From<String> for ConfigValue {
    fn from(value: String) -> Self {
        ConfigValue::String(value)
    }
}

impl From<&str> for ConfigValue {
    fn from(value: &str) -> Self {
        ConfigValue::String(value.to_string())
    }
}

impl From<Vec<ConfigValue>> for ConfigValue {
    fn from(value: Vec<ConfigValue>) -> Self {
        ConfigValue::Array(value)
    }
}

impl From<HashMap<String, ConfigValue>> for ConfigValue {
    fn from(value: HashMap<String, ConfigValue>) -> Self {
        ConfigValue::Object(value)
    }
}

// ============================================================================
// CONFIGURATION
// ============================================================================

/// Configuration complète de Note Craft
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    /// Version de la configuration
    pub version: Option<String>,
    /// Niveau de cette configuration
    pub level: Option<ConfigLevel>,
    /// Chemin du fichier source
    pub source_path: Option<PathBuf>,
    /// Valeurs de configuration
    #[serde(flatten)]
    pub values: HashMap<String, ConfigValue>,
    /// Configurations imbriquées (pour les sous-sections)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nested: Option<HashMap<String, Config>>,
}

impl Config {
    /// Crée une nouvelle configuration vide
    pub fn new() -> Self {
        Config {
            version: None,
            level: None,
            source_path: None,
            values: HashMap::new(),
            nested: None,
        }
    }

    /// Crée une configuration avec un niveau
    pub fn with_level(level: ConfigLevel) -> Self {
        Config {
            level: Some(level),
            ..Default::default()
        }
    }

    /// Obtient une valeur par clé
    pub fn get(&self, key: &str) -> Option<&ConfigValue> {
        self.values.get(key)
    }

    /// Obtient une valeur mutable par clé
    pub fn get_mut(&mut self, key: &str) -> Option<&mut ConfigValue> {
        self.values.get_mut(key)
    }

    /// Insère une valeur
    pub fn insert(&mut self, key: String, value: ConfigValue) -> Option<ConfigValue> {
        self.values.insert(key, value)
    }

    /// Supprime une valeur
    pub fn remove(&mut self, key: &str) -> Option<ConfigValue> {
        self.values.remove(key)
    }

    /// Vérifie si une clé existe
    pub fn contains_key(&self, key: &str) -> bool {
        self.values.contains_key(key)
    }

    /// Obtient une valeur comme booléen
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.get(key).and_then(|v| v.as_bool())
    }

    /// Obtient une valeur comme i64
    pub fn get_i64(&self, key: &str) -> Option<i64> {
        self.get(key).and_then(|v| v.as_i64())
    }

    /// Obtient une valeur comme f64
    pub fn get_f64(&self, key: &str) -> Option<f64> {
        self.get(key).and_then(|v| v.as_f64())
    }

    /// Obtient une valeur comme String
    pub fn get_string(&self, key: &str) -> Option<String> {
        self.get(key)
            .and_then(|v| v.as_str().map(|s| s.to_string()))
    }

    /// Obtient une configuration imbriquée
    pub fn get_nested(&self, key: &str) -> Option<&Config> {
        self.nested.as_ref().and_then(|n| n.get(key))
    }

    /// Obtient une configuration imbriquée mutable
    pub fn get_nested_mut(&mut self, key: &str) -> Option<&mut Config> {
        self.nested.as_mut().and_then(|n| n.get_mut(key))
    }

    /// Crée ou obtient une configuration imbriquée
    pub fn nested_or_create(&mut self, key: &str) -> &mut Config {
        if self.nested.is_none() {
            self.nested = Some(HashMap::new());
        }
        if !self.nested.as_ref().unwrap().contains_key(key) {
            self.nested
                .as_mut()
                .unwrap()
                .insert(key.to_string(), Config::new());
        }
        self.nested.as_mut().unwrap().get_mut(key).unwrap()
    }

    /// Fusionne avec une autre configuration (les valeurs de l'autre écrasent celles-ci)
    pub fn merge(&mut self, other: &Config) {
        for (key, value) in &other.values {
            self.values.insert(key.clone(), value.clone());
        }
        if let Some(other_nested) = &other.nested {
            if self.nested.is_none() {
                self.nested = Some(HashMap::new());
            }
            for (key, nested_config) in other_nested {
                self.nested_or_create(key).merge(nested_config);
            }
        }
    }

    /// Obtient toutes les clés
    pub fn keys(&self) -> Vec<&String> {
        self.values.keys().collect()
    }

    /// Vérifie si la configuration est vide
    pub fn is_empty(&self) -> bool {
        self.values.is_empty() && self.nested.as_ref().is_none_or(|n| n.is_empty())
    }
}

// ============================================================================
// ERREURS DE CONFIGURATION
// ============================================================================

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Erreur IO: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Erreur de parsing: {0}")]
    ParseError(String),
    #[error("Erreur de sérialisation: {0}")]
    SerializeError(String),
    #[error("Clé de configuration introuvable: {0}")]
    KeyNotFound(String),
    #[error("Type de configuration invalide pour {0}")]
    InvalidType(String),
    #[error("Chemin de configuration invalide: {0}")]
    InvalidPath(String),
    #[error("Niveau de configuration invalide")]
    InvalidLevel,
}

// ============================================================================
// CHEMINS DE CONFIGURATION
// ============================================================================

/// Chemin vers un fichier de configuration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConfigPath {
    /// Niveau de configuration
    pub level: ConfigLevel,
    /// Format du fichier
    pub format: ConfigFormat,
    /// Chemin du fichier
    pub path: PathBuf,
}

impl ConfigPath {
    /// Crée un chemin de configuration
    pub fn new<P: AsRef<Path>>(level: ConfigLevel, format: ConfigFormat, path: P) -> Self {
        ConfigPath {
            level,
            format,
            path: path.as_ref().to_path_buf(),
        }
    }

    /// Crée un chemin avec le format par défaut (JSON)
    pub fn with_default_format<P: AsRef<Path>>(level: ConfigLevel, path: P) -> Self {
        ConfigPath::new(level, ConfigFormat::default(), path)
    }

    /// Charge la configuration depuis le fichier
    pub fn load(&self) -> Result<Config, ConfigError> {
        let mut config = self.format.load_from_path(&self.path)?;
        config.level = Some(self.level);
        config.source_path = Some(self.path.clone());
        Ok(config)
    }

    /// Sauvegarde la configuration dans le fichier
    pub fn save(&self, config: &Config) -> Result<(), ConfigError> {
        self.format.save_to_path(config, &self.path)
    }

    /// Vérifie si le fichier existe
    pub fn exists(&self) -> bool {
        self.path.exists()
    }
}

// ============================================================================
// CHEMINS PAR DÉFAUT
// ============================================================================

/// Fournit les chemins par défaut pour les configurations
pub struct ConfigPaths {
    /// Chemin système
    pub system_config: Option<ConfigPath>,
    /// Chemin utilisateur
    pub user_config: Option<ConfigPath>,
    /// Chemin projet
    pub project_config: Option<ConfigPath>,
    /// Format par défaut
    pub default_format: ConfigFormat,
}

impl Default for ConfigPaths {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigPaths {
    /// Crée un ConfigPaths avec les chemins par défaut
    pub fn new() -> Self {
        ConfigPaths {
            system_config: None,
            user_config: None,
            project_config: None,
            default_format: ConfigFormat::Json,
        }
    }

    /// Configure les chemins
    pub fn with_system_path(mut self, path: Option<ConfigPath>) -> Self {
        self.system_config = path;
        self
    }

    pub fn with_user_path(mut self, path: Option<ConfigPath>) -> Self {
        self.user_config = path;
        self
    }

    pub fn with_project_path(mut self, path: Option<ConfigPath>) -> Self {
        self.project_config = path;
        self
    }

    pub fn with_format(mut self, format: ConfigFormat) -> Self {
        self.default_format = format;
        self
    }

    /// Obtient le chemin pour un niveau donné
    pub fn get_path(&self, level: ConfigLevel) -> Option<&ConfigPath> {
        match level {
            ConfigLevel::System => self.system_config.as_ref(),
            ConfigLevel::User => self.user_config.as_ref(),
            ConfigLevel::Project => self.project_config.as_ref(),
            _ => None,
        }
    }

    /// Crée un ConfigPath pour un niveau avec le format par défaut
    pub fn create_path(&self, level: ConfigLevel, path: impl AsRef<Path>) -> ConfigPath {
        ConfigPath::new(level, self.default_format, path)
    }
}

// ============================================================================
// GESTIONNAIRE DE CONFIGURATION
// ============================================================================

/// Gestionnaire de configuration
///
/// Ce struct gère la configuration hiérarchique de Note Craft.
/// Il charge et fusionne les configurations de différents niveaux.
pub struct ConfigManager {
    /// Chemins des configurations
    paths: ConfigPaths,
    /// Configurations chargées
    configs: RwLock<HashMap<ConfigLevel, Config>>,
    /// Configuration fusionnée (cache)
    merged_config: RwLock<Option<Config>>,
    /// Écouteurs de changements
    listeners: RwLock<Vec<Arc<dyn ConfigListener>>>,
}

impl ConfigManager {
    /// Crée un nouveau ConfigManager
    pub fn new() -> Self {
        ConfigManager {
            paths: ConfigPaths::new(),
            configs: RwLock::new(HashMap::new()),
            merged_config: RwLock::new(None),
            listeners: RwLock::new(Vec::new()),
        }
    }

    /// Crée un ConfigManager avec des chemins personnalisés
    pub fn with_paths(paths: ConfigPaths) -> Self {
        ConfigManager {
            paths,
            configs: RwLock::new(HashMap::new()),
            merged_config: RwLock::new(None),
            listeners: RwLock::new(Vec::new()),
        }
    }

    /// Charge toutes les configurations
    pub fn load_all(&self) -> Result<(), ConfigError> {
        let mut configs = self.configs.write().unwrap();

        // Charger les configurations dans l'ordre de priorité
        for level in [ConfigLevel::System, ConfigLevel::User, ConfigLevel::Project] {
            if let Some(path) = self.paths.get_path(level) {
                if path.exists() {
                    let config = path.load()?;
                    configs.insert(level, config);
                }
            }
        }

        // Invalider le cache de la configuration fusionnée
        let mut merged = self.merged_config.write().unwrap();
        *merged = None;

        self.emit_event(ConfigEvent::new(ConfigEventType::ConfigLoaded));

        Ok(())
    }

    /// Charge une configuration spécifique
    pub fn load_level(&self, level: ConfigLevel) -> Result<(), ConfigError> {
        if let Some(path) = self.paths.get_path(level) {
            if path.exists() {
                let config = path.load()?;
                let mut configs = self.configs.write().unwrap();
                configs.insert(level, config);

                // Invalider le cache
                let mut merged = self.merged_config.write().unwrap();
                *merged = None;

                self.emit_event(ConfigEvent::new(ConfigEventType::ConfigLoaded).with_level(level));
            }
        }
        Ok(())
    }

    /// Sauvegarde une configuration
    pub fn save_level(&self, level: ConfigLevel) -> Result<(), ConfigError> {
        let configs = self.configs.read().unwrap();
        if let Some(config) = configs.get(&level) {
            if let Some(path) = self.paths.get_path(level) {
                path.save(config)?;
                self.emit_event(ConfigEvent::new(ConfigEventType::ConfigSaved).with_level(level));
            }
        }
        Ok(())
    }

    /// Sauvegarde une valeur spécifique
    pub fn save_value(
        &self,
        level: ConfigLevel,
        key: String,
        value: ConfigValue,
    ) -> Result<(), ConfigError> {
        let mut configs = self.configs.write().unwrap();
        if let Some(config) = configs.get_mut(&level) {
            config.insert(key.clone(), value);
            self.save_level(level)?;
            self.emit_event(
                ConfigEvent::new(ConfigEventType::ConfigChanged)
                    .with_level(level)
                    .with_key(key),
            );
        }
        Ok(())
    }

    /// Obtient la configuration fusionnée
    pub fn get_merged(&self) -> Config {
        let merged = self.merged_config.read().unwrap();
        if merged.is_none() {
            drop(merged);
            let mut merged = self.merged_config.write().unwrap();
            if merged.is_none() {
                let configs = self.configs.read().unwrap();
                let mut result = Config::new();

                // Fusionner dans l'ordre de priorité (du moins prioritaire au plus)
                for level in [
                    ConfigLevel::Default,
                    ConfigLevel::System,
                    ConfigLevel::User,
                    ConfigLevel::Project,
                    ConfigLevel::Runtime,
                ] {
                    if let Some(config) = configs.get(&level) {
                        result.merge(config);
                    }
                }

                *merged = Some(result.clone());
                result
            } else {
                merged.as_ref().unwrap().clone()
            }
        } else {
            merged.as_ref().unwrap().clone()
        }
    }

    /// Obtient une valeur de configuration
    pub fn get_value(&self, key: &str) -> Option<ConfigValue> {
        self.get_merged().get(key).cloned()
    }

    /// Obtient une valeur comme booléen
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.get_merged().get_bool(key)
    }

    /// Obtient une valeur comme i64
    pub fn get_i64(&self, key: &str) -> Option<i64> {
        self.get_merged().get_i64(key)
    }

    /// Obtient une valeur comme f64
    pub fn get_f64(&self, key: &str) -> Option<f64> {
        self.get_merged().get_f64(key)
    }

    /// Obtient une valeur comme String
    pub fn get_string(&self, key: &str) -> Option<String> {
        self.get_merged().get_string(key)
    }

    /// Définit une valeur à un niveau spécifique
    pub fn set_value(
        &self,
        level: ConfigLevel,
        key: String,
        value: ConfigValue,
    ) -> Result<(), ConfigError> {
        let mut configs = self.configs.write().unwrap();
        let config = configs
            .entry(level)
            .or_insert_with(|| Config::with_level(level));
        config.insert(key.clone(), value);

        // Invalider le cache
        let mut merged = self.merged_config.write().unwrap();
        *merged = None;

        self.emit_event(
            ConfigEvent::new(ConfigEventType::ConfigChanged)
                .with_level(level)
                .with_key(key),
        );

        Ok(())
    }

    /// Définit une valeur au niveau Runtime (temporaire)
    pub fn set_runtime_value(&self, key: String, value: ConfigValue) {
        let _ = self.set_value(ConfigLevel::Runtime, key.clone(), value);
    }

    /// Supprime une valeur
    pub fn remove_value(&self, level: ConfigLevel, key: &str) -> Result<(), ConfigError> {
        let mut configs = self.configs.write().unwrap();
        if let Some(config) = configs.get_mut(&level) {
            config.remove(key);

            // Invalider le cache
            let mut merged = self.merged_config.write().unwrap();
            *merged = None;

            self.emit_event(
                ConfigEvent::new(ConfigEventType::ConfigChanged)
                    .with_level(level)
                    .with_key(key.to_string()),
            );
        }
        Ok(())
    }

    /// Réinitialise la configuration
    pub fn reset(&self) {
        let mut configs = self.configs.write().unwrap();
        configs.clear();

        let mut merged = self.merged_config.write().unwrap();
        *merged = None;

        self.emit_event(ConfigEvent::new(ConfigEventType::ConfigReset));
    }

    /// Ajoute un écouteur
    pub fn add_listener(&self, listener: Arc<dyn ConfigListener>) {
        let mut listeners = self.listeners.write().unwrap();
        listeners.push(listener);
    }

    /// Supprime un écouteur
    pub fn remove_listener(&self, listener: &Arc<dyn ConfigListener>) {
        let mut listeners = self.listeners.write().unwrap();
        listeners.retain(|l| !Arc::ptr_eq(l, listener));
    }

    /// Émet un événement
    fn emit_event(&self, event: ConfigEvent) {
        let listeners = self.listeners.read().unwrap();
        for listener in listeners.iter() {
            listener.on_config_event(&event);
        }
    }

    /// Configure les chemins
    pub fn set_paths(&mut self, paths: ConfigPaths) {
        self.paths = paths;
    }

    /// Obtient les chemins actuels
    pub fn paths(&self) -> &ConfigPaths {
        &self.paths
    }
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// ÉVÉNEMENTS DE CONFIGURATION
// ============================================================================

/// Type d'événement de configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigEventType {
    /// Configuration chargée
    ConfigLoaded,
    /// Configuration sauvegardée
    ConfigSaved,
    /// Configuration changée
    ConfigChanged,
    /// Configuration réinitialisée
    ConfigReset,
}

/// Événement de configuration
#[derive(Debug, Clone)]
pub struct ConfigEvent {
    /// Type d'événement
    pub event_type: ConfigEventType,
    /// Niveau concerné
    pub level: Option<ConfigLevel>,
    /// Clé concernée
    pub key: Option<String>,
    /// Timestamp
    pub timestamp: u64,
}

impl ConfigEvent {
    pub fn new(event_type: ConfigEventType) -> Self {
        ConfigEvent {
            event_type,
            level: None,
            key: None,
            timestamp: Self::current_timestamp(),
        }
    }

    pub fn with_level(mut self, level: ConfigLevel) -> Self {
        self.level = Some(level);
        self
    }

    pub fn with_key(mut self, key: String) -> Self {
        self.key = Some(key);
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

// ============================================================================
// ÉCOUTEUR DE CONFIGURATION
// ============================================================================

/// Trait pour écouter les événements de configuration
pub trait ConfigListener: Send + Sync {
    /// Appelé lorsqu'un événement de configuration se produit
    fn on_config_event(&self, event: &ConfigEvent);
}

// ============================================================================
// CONFIGURATIONS PAR DÉFAUT
// ============================================================================

/// Fournit des configurations par défaut
pub mod defaults {
    use super::*;

    /// Configuration par défaut pour l'éditeur
    pub fn editor_config() -> Config {
        let mut config = Config::new();
        config.insert(
            "editor.font_family".to_string(),
            "JetBrains Mono, Fira Code, monospace".into(),
        );
        config.insert("editor.font_size".to_string(), 14i64.into());
        config.insert("editor.theme".to_string(), "dark".into());
        config.insert("editor.indent_size".to_string(), 2i64.into());
        config.insert("editor.word_wrap".to_string(), true.into());
        config.insert("editor.line_numbers".to_string(), true.into());
        config
    }

    /// Configuration par défaut pour l'UI
    pub fn ui_config() -> Config {
        let mut config = Config::new();
        config.insert("ui.language".to_string(), "en".into());
        config.insert("ui.sidebar_width".to_string(), 250i64.into());
        config.insert("ui.show_toolbar".to_string(), true.into());
        config.insert("ui.show_status_bar".to_string(), true.into());
        config.insert("ui.animation_enabled".to_string(), true.into());
        config
    }

    /// Configuration par défaut pour le graphe
    pub fn graph_config() -> Config {
        let mut config = Config::new();
        config.insert("graph.max_nodes".to_string(), 10000i64.into());
        config.insert("graph.render_mode".to_string(), "canvas".into());
        config.insert("graph.node_radius".to_string(), 10i64.into());
        config.insert("graph.link_thickness".to_string(), 1i64.into());
        config.insert("graph.jump_to_node_on_click".to_string(), true.into());
        config
    }

    /// Configuration par défaut pour le stockage
    pub fn storage_config() -> Config {
        let mut config = Config::new();
        config.insert("storage.auto_save".to_string(), true.into());
        config.insert("storage.auto_save_interval".to_string(), 30i64.into()); // secondes
        config.insert("storage.backup_enabled".to_string(), true.into());
        config.insert("storage.backup_count".to_string(), 5i64.into());
        config.insert("storage.compress_backups".to_string(), false.into());
        config
    }

    /// Configuration par défaut complète
    pub fn full_config() -> Config {
        let mut config = Config::new();

        // Éditeur
        for (key, value) in editor_config().values {
            config.insert(format!("editor.{}", key), value);
        }

        // UI
        for (key, value) in ui_config().values {
            config.insert(format!("ui.{}", key), value);
        }

        // Graphe
        for (key, value) in graph_config().values {
            config.insert(format!("graph.{}", key), value);
        }

        // Stockage
        for (key, value) in storage_config().values {
            config.insert(format!("storage.{}", key), value);
        }

        config
    }
}

// ============================================================================
// VALIDATION DE CONFIGURATION
// ============================================================================

/// Validateur de configuration
pub struct ConfigValidator {
    /// Règles de validation
    rules: HashMap<String, ValidationRule>,
}

/// Règle de validation
pub enum ValidationRule {
    /// Doit être un booléen
    Boolean,
    /// Doit être un entier
    Integer,
    /// Doit être un flottant
    Float,
    /// Doit être une chaîne
    String,
    /// Doit être dans une liste de valeurs
    Enum(Vec<ConfigValue>),
    /// Doit être supérieur à une valeur
    Min(f64),
    /// Doit être inférieur à une valeur
    Max(f64),
    /// Doit correspondre à un pattern regex
    Pattern(String),
    /// Configuration imbriquée
    Nested(Box<ConfigValidator>),
}

impl ConfigValidator {
    pub fn new() -> Self {
        ConfigValidator {
            rules: HashMap::new(),
        }
    }

    /// Ajoute une règle
    pub fn add_rule(&mut self, key: String, rule: ValidationRule) {
        self.rules.insert(key, rule);
    }

    /// Valide une configuration
    pub fn validate(&self, config: &Config) -> Result<Vec<ValidationError>, ConfigError> {
        let mut errors = Vec::new();

        for (key, value) in &config.values {
            if let Some(rule) = self.rules.get(key) {
                if let Some(error) = self.validate_rule(key, value, rule) {
                    errors.push(error);
                }
            }
        }

        // Valider les configurations imbriquées
        if let Some(nested) = &config.nested {
            for (key, nested_config) in nested {
                if let Some(ValidationRule::Nested(validator)) = self.rules.get(key) {
                    let nested_errors = validator.validate(nested_config)?;
                    errors.extend(nested_errors);
                }
            }
        }

        Ok(errors)
    }

    /// Valide une règle spécifique
    fn validate_rule(
        &self,
        key: &str,
        value: &ConfigValue,
        rule: &ValidationRule,
    ) -> Option<ValidationError> {
        match rule {
            ValidationRule::Boolean => {
                if value.as_bool().is_none() {
                    Some(ValidationError::new(key, "must be a boolean"))
                } else {
                    None
                }
            }
            ValidationRule::Integer => {
                if value.as_i64().is_none() {
                    Some(ValidationError::new(key, "must be an integer"))
                } else {
                    None
                }
            }
            ValidationRule::Float => {
                if value.as_f64().is_none() {
                    Some(ValidationError::new(key, "must be a float"))
                } else {
                    None
                }
            }
            ValidationRule::String => {
                if value.as_str().is_none() {
                    Some(ValidationError::new(key, "must be a string"))
                } else {
                    None
                }
            }
            ValidationRule::Enum(values) => {
                if !values.contains(value) {
                    Some(ValidationError::new(
                        key,
                        &format!("must be one of: {:?}", values),
                    ))
                } else {
                    None
                }
            }
            ValidationRule::Min(min) => {
                if let Some(v) = value.as_f64() {
                    if v < *min {
                        Some(ValidationError::new(key, &format!("must be >= {}", min)))
                    } else {
                        None
                    }
                } else {
                    Some(ValidationError::new(key, "must be a number"))
                }
            }
            ValidationRule::Max(max) => {
                if let Some(v) = value.as_f64() {
                    if v > *max {
                        Some(ValidationError::new(key, &format!("must be <= {}", max)))
                    } else {
                        None
                    }
                } else {
                    Some(ValidationError::new(key, "must be a number"))
                }
            }
            ValidationRule::Pattern(_) => {
                // TODO: Implémenter la validation regex
                None
            }
            ValidationRule::Nested(_) => None,
        }
    }
}

impl Default for ConfigValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Erreur de validation
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub key: String,
    pub message: String,
}

impl ValidationError {
    pub fn new(key: &str, message: &str) -> Self {
        ValidationError {
            key: key.to_string(),
            message: message.to_string(),
        }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.key, self.message)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_config_new() {
        let config = Config::new();
        assert!(config.is_empty());
        assert!(config.version.is_none());
        assert!(config.level.is_none());
    }

    #[test]
    fn test_config_with_level() {
        let config = Config::with_level(ConfigLevel::User);
        assert_eq!(config.level, Some(ConfigLevel::User));
    }

    #[test]
    fn test_config_insert_and_get() {
        let mut config = Config::new();
        config.insert(
            "test_key".to_string(),
            ConfigValue::String("test_value".to_string()),
        );

        let value = config.get("test_key");
        assert!(value.is_some());
        assert_eq!(value.unwrap().as_str(), Some("test_value"));
    }

    #[test]
    fn test_config_get_bool() {
        let mut config = Config::new();
        config.insert("bool_key".to_string(), true.into());

        assert_eq!(config.get_bool("bool_key"), Some(true));
        assert_eq!(config.get_bool("missing_key"), None);
    }

    #[test]
    fn test_config_get_i64() {
        let mut config = Config::new();
        config.insert("int_key".to_string(), 42i64.into());

        assert_eq!(config.get_i64("int_key"), Some(42));
        assert_eq!(config.get_i64("missing_key"), None);
    }

    #[test]
    fn test_config_get_string() {
        let mut config = Config::new();
        config.insert("string_key".to_string(), "hello".into());

        assert_eq!(config.get_string("string_key"), Some("hello".to_string()));
        assert_eq!(config.get_string("missing_key"), None);
    }

    #[test]
    fn test_config_nested() {
        let mut config = Config::new();
        let nested = config.nested_or_create("editor");
        nested.insert("font_size".to_string(), 14i64.into());

        let editor_config = config.get_nested("editor");
        assert!(editor_config.is_some());
        assert_eq!(editor_config.unwrap().get_i64("font_size"), Some(14));
    }

    #[test]
    fn test_config_merge() {
        let mut config1 = Config::new();
        config1.insert("key1".to_string(), "value1".into());
        config1.insert("key2".to_string(), "value2".into());

        let mut config2 = Config::new();
        config2.insert("key2".to_string(), "new_value2".into());
        config2.insert("key3".to_string(), "value3".into());

        config1.merge(&config2);

        assert_eq!(config1.get_string("key1"), Some("value1".to_string()));
        assert_eq!(config1.get_string("key2"), Some("new_value2".to_string()));
        assert_eq!(config1.get_string("key3"), Some("value3".to_string()));
    }

    #[test]
    fn test_config_value_as_bool() {
        let value = ConfigValue::Bool(true);
        assert_eq!(value.as_bool(), Some(true));
        assert!(value.as_i64().is_none());
    }

    #[test]
    fn test_config_value_as_i64() {
        let value = ConfigValue::Int(42);
        assert_eq!(value.as_i64(), Some(42));
        assert!(value.as_bool().is_none());
    }

    #[test]
    fn test_config_value_as_str() {
        let value = ConfigValue::String("test".to_string());
        assert_eq!(value.as_str(), Some("test"));
    }

    #[test]
    fn test_config_value_as_array() {
        let value = ConfigValue::Array(vec![1i64.into(), 2i64.into(), 3i64.into()]);
        assert!(value.as_array().is_some());
        assert_eq!(value.as_array().unwrap().len(), 3);
    }

    #[test]
    fn test_config_value_as_object() {
        let mut map = HashMap::new();
        map.insert("key1".to_string(), "value1".into());
        map.insert("key2".to_string(), "value2".into());
        let value = ConfigValue::Object(map);
        assert!(value.as_object().is_some());
        assert_eq!(value.as_object().unwrap().len(), 2);
    }

    #[test]
    fn test_config_format_json() {
        let mut config = Config::new();
        config.insert("key".to_string(), "value".into());

        let json = ConfigFormat::Json.save_to_str(&config).unwrap();
        let loaded = ConfigFormat::Json.load_from_str(&json).unwrap();

        assert_eq!(loaded.get_string("key"), Some("value".to_string()));
    }

    #[test]
    fn test_config_format_toml() {
        let mut config = Config::new();
        config.insert("key".to_string(), "value".into());

        let toml = ConfigFormat::Toml.save_to_str(&config).unwrap();
        let loaded = ConfigFormat::Toml.load_from_str(&toml).unwrap();

        assert_eq!(loaded.get_string("key"), Some("value".to_string()));
    }

    #[test]
    fn test_config_path() {
        // Créer un fichier temporaire
        let file = NamedTempFile::new().unwrap();
        let path = file.path().to_path_buf();

        // Sauvegarder une configuration
        let mut config = Config::new();
        config.insert("test".to_string(), "value".into());

        let config_path = ConfigPath::new(ConfigLevel::User, ConfigFormat::Json, &path);
        config_path.save(&config).unwrap();

        // Charger la configuration
        let loaded = config_path.load().unwrap();
        assert_eq!(loaded.get_string("test"), Some("value".to_string()));
    }

    #[test]
    fn test_config_manager_new() {
        let manager = ConfigManager::new();
        let config = manager.get_merged();
        assert!(config.is_empty());
    }

    #[test]
    fn test_config_manager_set_and_get() {
        let manager = ConfigManager::new();
        manager
            .set_value(
                ConfigLevel::Runtime,
                "test_key".to_string(),
                "test_value".into(),
            )
            .unwrap();

        assert_eq!(
            manager.get_string("test_key"),
            Some("test_value".to_string())
        );
    }

    #[test]
    fn test_config_manager_runtime_value() {
        let manager = ConfigManager::new();
        manager.set_runtime_value("temp_key".to_string(), 42i64.into());

        assert_eq!(manager.get_i64("temp_key"), Some(42));
    }

    #[test]
    fn test_config_manager_merge_levels() {
        let manager = ConfigManager::new();

        // Niveau Default
        manager
            .set_value(ConfigLevel::Default, "key1".to_string(), "default".into())
            .unwrap();

        // Niveau User (écrase le default)
        manager
            .set_value(ConfigLevel::User, "key1".to_string(), "user".into())
            .unwrap();

        // Niveau Project (écrase le user)
        manager
            .set_value(ConfigLevel::Project, "key1".to_string(), "project".into())
            .unwrap();

        // La valeur fusionnée doit être celle du project
        assert_eq!(manager.get_string("key1"), Some("project".to_string()));
    }

    #[test]
    fn test_config_level_display() {
        assert_eq!(format!("{}", ConfigLevel::Default), "default");
        assert_eq!(format!("{}", ConfigLevel::System), "system");
        assert_eq!(format!("{}", ConfigLevel::User), "user");
        assert_eq!(format!("{}", ConfigLevel::Project), "project");
        assert_eq!(format!("{}", ConfigLevel::Runtime), "runtime");
    }

    #[test]
    fn test_config_format_extension() {
        assert_eq!(ConfigFormat::Json.extension(), "json");
        assert_eq!(ConfigFormat::Toml.extension(), "toml");
        assert_eq!(ConfigFormat::Yaml.extension(), "yaml");
    }

    #[test]
    fn test_defaults_editor_config() {
        let config = defaults::editor_config();
        assert_eq!(
            config.get_string("editor.font_family"),
            Some("JetBrains Mono, Fira Code, monospace".to_string())
        );
        assert_eq!(config.get_i64("editor.font_size"), Some(14));
        assert_eq!(config.get_string("editor.theme"), Some("dark".to_string()));
    }

    #[test]
    fn test_config_value_is_null() {
        let value = ConfigValue::Null;
        assert!(value.is_null());
    }

    #[test]
    fn test_config_keys() {
        let mut config = Config::new();
        config.insert("key1".to_string(), "value1".into());
        config.insert("key2".to_string(), "value2".into());

        let keys = config.keys();
        assert_eq!(keys.len(), 2);
        assert!(keys.contains(&&"key1".to_string()));
        assert!(keys.contains(&&"key2".to_string()));
    }

    #[test]
    fn test_config_remove() {
        let mut config = Config::new();
        config.insert("key1".to_string(), "value1".into());

        let removed = config.remove("key1");
        assert!(removed.is_some());
        assert!(config.get("key1").is_none());
    }

    #[test]
    fn test_config_contains_key() {
        let mut config = Config::new();
        config.insert("key1".to_string(), "value1".into());

        assert!(config.contains_key("key1"));
        assert!(!config.contains_key("key2"));
    }

    #[test]
    fn test_config_from_primitives() {
        assert!(ConfigValue::from(true).as_bool().is_some());
        assert!(ConfigValue::from(42i64).as_i64().is_some());
        assert!(ConfigValue::from(std::f64::consts::PI).as_f64().is_some());
        assert!(ConfigValue::from("test".to_string()).as_str().is_some());
    }

    #[test]
    fn test_config_manager_reset() {
        let manager = ConfigManager::new();
        manager
            .set_value(ConfigLevel::Runtime, "key1".to_string(), "value1".into())
            .unwrap();

        assert!(manager.get_string("key1").is_some());

        manager.reset();

        assert!(manager.get_string("key1").is_none());
    }

    #[test]
    fn test_validation_rule_boolean() {
        let mut validator = ConfigValidator::new();
        validator.add_rule("bool_key".to_string(), ValidationRule::Boolean);

        let mut config = Config::new();
        config.insert("bool_key".to_string(), true.into());

        let errors = validator.validate(&config).unwrap();
        assert!(errors.is_empty());
    }

    #[test]
    fn test_validation_rule_integer() {
        let mut validator = ConfigValidator::new();
        validator.add_rule("int_key".to_string(), ValidationRule::Integer);

        let mut config = Config::new();
        config.insert("int_key".to_string(), "not an integer".into());

        let errors = validator.validate(&config).unwrap();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].key, "int_key");
    }

    #[test]
    fn test_validation_rule_enum() {
        let mut validator = ConfigValidator::new();
        validator.add_rule(
            "theme".to_string(),
            ValidationRule::Enum(vec![
                ConfigValue::String("dark".to_string()),
                ConfigValue::String("light".to_string()),
            ]),
        );

        let mut config = Config::new();
        config.insert("theme".to_string(), "dark".into());

        let errors = validator.validate(&config).unwrap();
        assert!(errors.is_empty());

        config.insert("theme".to_string(), "invalid".into());
        let errors = validator.validate(&config).unwrap();
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn test_validation_rule_min() {
        let mut validator = ConfigValidator::new();
        validator.add_rule("value".to_string(), ValidationRule::Min(0.0));

        let mut config = Config::new();
        config.insert("value".to_string(), (-1i64).into());

        let errors = validator.validate(&config).unwrap();
        assert_eq!(errors.len(), 1);
    }
}
