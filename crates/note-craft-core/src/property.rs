//! Note Craft Property System
//!
//! Ce module fournit le système de propriétés personnalisées pour Note Craft.
//! Il permet de définir et gérer des propriétés sur les pages et les blocs,
//! similaires à Logseq/Obsidian.
//!
//! Phase 4 - Étape 4.2 (Property System)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use thiserror::Error;

// ============================================================================
// IDENTIFIANTS
// ============================================================================

/// Identifiant unique pour une propriété
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PropertyId(pub u64);

/// Identifiant unique pour une valeur de propriété
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PropertyValueId(pub u64);

// ============================================================================
// TYPES DE PROPRIÉTÉS
// ============================================================================

/// Types de propriétés supportés par Note Craft
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum PropertyType {
    /// Texte simple
    #[default]
    Text,
    /// Nombre (entier ou décimal)
    Number,
    /// Booléen (vrai/faux)
    Boolean,
    /// Date (ISO 8601)
    Date,
    /// Date et heure (ISO 8601)
    DateTime,
    /// Sélection parmi des options prédéfinies
    Select,
    /// Multi-sélection parmi des options prédéfinies
    MultiSelect,
    /// URL
    Url,
    /// Email
    Email,
    /// Couleur (hex ou nom)
    Color,
}

impl fmt::Display for PropertyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PropertyType::Text => "text",
                PropertyType::Number => "number",
                PropertyType::Boolean => "boolean",
                PropertyType::Date => "date",
                PropertyType::DateTime => "datetime",
                PropertyType::Select => "select",
                PropertyType::MultiSelect => "multiselect",
                PropertyType::Url => "url",
                PropertyType::Email => "email",
                PropertyType::Color => "color",
            }
        )
    }
}

/// Convertit une chaîne en PropertyType
impl std::str::FromStr for PropertyType {
    type Err = PropertyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "text" => Ok(PropertyType::Text),
            "number" => Ok(PropertyType::Number),
            "boolean" | "bool" => Ok(PropertyType::Boolean),
            "date" => Ok(PropertyType::Date),
            "datetime" => Ok(PropertyType::DateTime),
            "select" => Ok(PropertyType::Select),
            "multiselect" => Ok(PropertyType::MultiSelect),
            "url" => Ok(PropertyType::Url),
            "email" => Ok(PropertyType::Email),
            "color" => Ok(PropertyType::Color),
            _ => Err(PropertyError::InvalidType(s.to_string())),
        }
    }
}

// ============================================================================
// VALEURS DE PROPRIÉTÉ
// ============================================================================

/// Valeur d'une propriété
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(Default)]
pub enum PropertyValue {
    /// Valeur texte
    Text(String),
    /// Valeur numérique
    Number(f64),
    /// Valeur booléenne
    Boolean(bool),
    /// Valeur date (ISO 8601)
    Date(String),
    /// Valeur date-heure (ISO 8601)
    DateTime(String),
    /// Sélection unique
    Select(String),
    /// Multi-sélection
    MultiSelect(Vec<String>),
    /// URL
    Url(String),
    /// Email
    Email(String),
    /// Couleur
    Color(String),
    /// Pas de valeur
    #[default]
    None,
}

impl PropertyValue {
    /// Convertit la valeur en chaîne de caractères
    pub fn as_string(&self) -> Option<String> {
        match self {
            PropertyValue::Text(v) => Some(v.clone()),
            PropertyValue::Number(v) => Some(v.to_string()),
            PropertyValue::Boolean(v) => Some(v.to_string()),
            PropertyValue::Date(v) => Some(v.clone()),
            PropertyValue::DateTime(v) => Some(v.clone()),
            PropertyValue::Select(v) => Some(v.clone()),
            PropertyValue::MultiSelect(v) => Some(v.join(", ")),
            PropertyValue::Url(v) => Some(v.clone()),
            PropertyValue::Email(v) => Some(v.clone()),
            PropertyValue::Color(v) => Some(v.clone()),
            PropertyValue::None => None,
        }
    }

    /// Convertit la valeur en nombre
    pub fn as_number(&self) -> Option<f64> {
        match self {
            PropertyValue::Number(v) => Some(*v),
            PropertyValue::Text(v) => v.parse::<f64>().ok(),
            _ => None,
        }
    }

    /// Convertit la valeur en booléen
    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            PropertyValue::Boolean(v) => Some(*v),
            PropertyValue::Text(v) => v.to_lowercase().parse::<bool>().ok(),
            _ => None,
        }
    }

    /// Retourne le type de la valeur
    pub fn value_type(&self) -> PropertyType {
        match self {
            PropertyValue::Text(_) => PropertyType::Text,
            PropertyValue::Number(_) => PropertyType::Number,
            PropertyValue::Boolean(_) => PropertyType::Boolean,
            PropertyValue::Date(_) => PropertyType::Date,
            PropertyValue::DateTime(_) => PropertyType::DateTime,
            PropertyValue::Select(_) => PropertyType::Select,
            PropertyValue::MultiSelect(_) => PropertyType::MultiSelect,
            PropertyValue::Url(_) => PropertyType::Url,
            PropertyValue::Email(_) => PropertyType::Email,
            PropertyValue::Color(_) => PropertyType::Color,
            PropertyValue::None => PropertyType::Text,
        }
    }
}

// ============================================================================
// DÉFINITION D'UNE PROPRIÉTÉ
// ============================================================================

/// Définition d'une propriété
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Property {
    /// Identifiant unique de la propriété
    pub id: PropertyId,
    /// Nom de la propriété
    pub name: String,
    /// Nom normalisé (minuscules, sans espaces)
    pub normalized_name: String,
    /// Type de la propriété
    pub property_type: PropertyType,
    /// Valeur par défaut
    pub default_value: Option<PropertyValue>,
    /// Options disponibles (pour Select et MultiSelect)
    pub options: Option<Vec<String>>,
    /// Indique si c'est une propriété système
    pub is_system: bool,
    /// Indique si la propriété est requise
    pub is_required: bool,
    /// Description de la propriété
    pub description: Option<String>,
    /// Ordre d'affichage
    pub display_order: u32,
    /// Date de création
    pub created_at: u64,
    /// Date de modification
    pub updated_at: u64,
}

impl Property {
    /// Crée une nouvelle propriété
    pub fn new(name: String, property_type: PropertyType, is_system: bool) -> Self {
        let now = chrono::Utc::now().timestamp() as u64;
        let normalized_name = Self::normalize_name(&name);

        Self {
            id: PropertyId(0), // sera généré par le stockage
            name,
            normalized_name,
            property_type,
            default_value: None,
            options: None,
            is_system,
            is_required: false,
            description: None,
            display_order: 0,
            created_at: now,
            updated_at: now,
        }
    }

    /// Crée une propriété avec des options (pour Select/MultiSelect)
    pub fn with_options(
        name: String,
        property_type: PropertyType,
        options: Vec<String>,
        is_system: bool,
    ) -> Self {
        let mut prop = Self::new(name, property_type, is_system);
        prop.options = Some(options);
        prop
    }

    /// Normalise le nom d'une propriété
    pub fn normalize_name(name: &str) -> String {
        name.to_lowercase()
            .replace(" ", "_")
            .replace("-", "_")
            .replace(".", "_")
    }

    /// Vérifie si la valeur est valide pour cette propriété
    pub fn is_valid_value(&self, value: &PropertyValue) -> bool {
        match (&self.property_type, value) {
            (PropertyType::Text, PropertyValue::Text(_)) => true,
            (PropertyType::Number, PropertyValue::Number(_)) => true,
            (PropertyType::Boolean, PropertyValue::Boolean(_)) => true,
            (PropertyType::Date, PropertyValue::Date(_)) => true,
            (PropertyType::DateTime, PropertyValue::DateTime(_)) => true,
            (PropertyType::Date, PropertyValue::DateTime(_)) => true, // compatible
            (PropertyType::Select, PropertyValue::Select(v)) => {
                self.options.as_ref().is_none_or(|opts| opts.contains(v))
            }
            (PropertyType::MultiSelect, PropertyValue::MultiSelect(v)) => self
                .options
                .as_ref()
                .is_none_or(|opts| v.iter().all(|item| opts.contains(item))),
            (PropertyType::Url, PropertyValue::Url(_)) => true,
            (PropertyType::Url, PropertyValue::Text(v)) => {
                v.starts_with("http://") || v.starts_with("https://")
            }
            (PropertyType::Email, PropertyValue::Email(_)) => true,
            (PropertyType::Email, PropertyValue::Text(v)) => v.contains("@"),
            (PropertyType::Color, PropertyValue::Color(_)) => true,
            (PropertyType::Color, PropertyValue::Text(v)) => {
                v.starts_with("#") || Self::is_valid_color_name(v)
            }
            _ => false,
        }
    }

    /// Vérifie si le nom de couleur est valide
    fn is_valid_color_name(name: &str) -> bool {
        let valid_colors = [
            "red", "orange", "yellow", "green", "blue", "indigo", "violet", "pink", "purple",
            "gray", "grey", "black", "white", "brown", "cyan", "magenta", "lime", "teal", "navy",
            "maroon",
        ];
        valid_colors.contains(&name.to_lowercase().as_str())
    }
}

impl Default for Property {
    fn default() -> Self {
        let now = chrono::Utc::now().timestamp() as u64;

        Self {
            id: PropertyId(0),
            name: String::new(),
            normalized_name: String::new(),
            property_type: PropertyType::Text,
            default_value: None,
            options: None,
            is_system: false,
            is_required: false,
            description: None,
            display_order: 0,
            created_at: now,
            updated_at: now,
        }
    }
}

// ============================================================================
// VALEUR DE PROPRIÉTÉ ASSOCIÉE À UNE ENTITÉ
// ============================================================================

/// Valeur de propriété associée à une entité (page ou block)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PropertyValueEntry {
    /// Identifiant unique
    pub id: PropertyValueId,
    /// Identifiant de la propriété
    pub property_id: PropertyId,
    /// Identifiant de l'entité (page ou block)
    pub entity_id: u64,
    /// Type de l'entité (0 = page, 1 = block)
    pub entity_type: PropertyEntityType,
    /// Valeur de la propriété
    pub value: PropertyValue,
    /// Date de création
    pub created_at: u64,
    /// Date de modification
    pub updated_at: u64,
}

impl PropertyValueEntry {
    /// Crée une nouvelle entrée de valeur de propriété
    pub fn new(
        property_id: PropertyId,
        entity_id: u64,
        entity_type: PropertyEntityType,
        value: PropertyValue,
    ) -> Self {
        let now = chrono::Utc::now().timestamp() as u64;

        Self {
            id: PropertyValueId(0), // sera généré par le stockage
            property_id,
            entity_id,
            entity_type,
            value,
            created_at: now,
            updated_at: now,
        }
    }
}

// ============================================================================
// TYPES D'ENTITÉS
// ============================================================================

/// Type d'entité pour les valeurs de propriétés
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum PropertyEntityType {
    /// Page
    #[default]
    Page = 0,
    /// Block
    Block = 1,
}

// ============================================================================
// ERREURS
// ============================================================================

/// Erreurs liées aux propriétés
#[derive(Error, Debug, Clone)]
pub enum PropertyError {
    #[error("Propriété non trouvée: {0}")]
    NotFound(String),
    #[error("Propriété déjà existante: {0}")]
    AlreadyExists(String),
    #[error("Type de propriété invalide: {0}")]
    InvalidType(String),
    #[error("Valeur invalide pour la propriété: {0}")]
    InvalidValue(String),
    #[error("Type incompatible: attendu {expected}, obtenu {actual}")]
    TypeMismatch {
        expected: PropertyType,
        actual: PropertyType,
    },
    #[error("Erreur de stockage: {0}")]
    StorageError(String),
    #[error("Option non valide pour Select: {0}")]
    InvalidOption(String),
    #[error("Valeur requise manquante: {0}")]
    RequiredValueMissing(String),
}

// ============================================================================
// SYSTÈME DE PROPRIÉTÉS
// ============================================================================

/// Système de gestion des propriétés
pub struct PropertySystem {
    /// Map des propriétés par ID
    properties: HashMap<PropertyId, Property>,
    /// Map des propriétés par nom normalisé
    properties_by_name: HashMap<String, PropertyId>,
    /// Map des valeurs de propriétés par entité
    property_values: HashMap<u64, Vec<PropertyValueEntry>>,
    /// Compteur pour les IDs
    next_property_id: u64,
    next_value_id: u64,
}

impl PropertySystem {
    /// Crée un nouveau système de propriétés
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
            properties_by_name: HashMap::new(),
            property_values: HashMap::new(),
            next_property_id: 1,
            next_value_id: 1,
        }
    }

    /// Crée une nouvelle propriété
    pub fn create_property(&mut self, property: Property) -> Result<PropertyId, PropertyError> {
        // Vérifier si la propriété existe déjà
        if self
            .properties_by_name
            .contains_key(&property.normalized_name)
        {
            return Err(PropertyError::AlreadyExists(property.name));
        }

        // Générer un ID
        let property_id = PropertyId(self.next_property_id);
        self.next_property_id += 1;

        // Créer la propriété avec l'ID
        let mut new_property = property;
        new_property.id = property_id;

        // Ajouter au stockage
        self.properties.insert(property_id, new_property.clone());
        self.properties_by_name
            .insert(new_property.normalized_name.clone(), property_id);

        Ok(property_id)
    }

    /// Récupère une propriété par ID
    pub fn get_property(&self, id: PropertyId) -> Option<&Property> {
        self.properties.get(&id)
    }

    /// Récupère une propriété par nom
    pub fn get_property_by_name(&self, name: &str) -> Option<&Property> {
        let normalized = Property::normalize_name(name);
        self.properties_by_name
            .get(&normalized)
            .and_then(|id| self.properties.get(id))
    }

    /// Liste toutes les propriétés
    pub fn list_properties(&self) -> Vec<&Property> {
        self.properties.values().collect()
    }

    /// Met à jour une propriété
    pub fn update_property(
        &mut self,
        id: PropertyId,
        updates: PropertyUpdate,
    ) -> Result<(), PropertyError> {
        if let Some(property) = self.properties.get_mut(&id) {
            if let Some(name) = updates.name {
                // Vérifier si le nouveau nom est déjà utilisé
                let normalized = Property::normalize_name(&name);
                if self
                    .properties_by_name
                    .get(&normalized)
                    .is_some_and(|&existing_id| existing_id != id)
                {
                    return Err(PropertyError::AlreadyExists(name.clone()));
                }

                // Mettre à jour le nom
                self.properties_by_name.remove(&property.normalized_name);
                property.name = name;
                property.normalized_name = normalized.clone();
                self.properties_by_name.insert(normalized, id);
            }

            if let Some(property_type) = updates.property_type {
                property.property_type = property_type;
            }
            if let Some(default_value) = updates.default_value {
                property.default_value = Some(default_value);
            }
            if let Some(options) = updates.options {
                property.options = Some(options);
            }
            if let Some(is_required) = updates.is_required {
                property.is_required = is_required;
            }
            if let Some(description) = updates.description {
                property.description = Some(description);
            }
            if let Some(display_order) = updates.display_order {
                property.display_order = display_order;
            }

            property.updated_at = chrono::Utc::now().timestamp() as u64;
            Ok(())
        } else {
            Err(PropertyError::NotFound(id.0.to_string()))
        }
    }

    /// Supprime une propriété
    pub fn delete_property(&mut self, id: PropertyId) -> Result<(), PropertyError> {
        if let Some(property) = self.properties.remove(&id) {
            self.properties_by_name.remove(&property.normalized_name);
            Ok(())
        } else {
            Err(PropertyError::NotFound(id.0.to_string()))
        }
    }

    /// Définit une valeur de propriété pour une entité
    pub fn set_property_value(
        &mut self,
        property_id: PropertyId,
        entity_id: u64,
        entity_type: PropertyEntityType,
        value: PropertyValue,
    ) -> Result<(), PropertyError> {
        // Vérifier que la propriété existe
        let property = self
            .properties
            .get(&property_id)
            .ok_or_else(|| PropertyError::NotFound(property_id.0.to_string()))?;

        // Vérifier que la valeur est valide
        if !property.is_valid_value(&value) {
            return Err(PropertyError::InvalidValue(format!(
                "Value {:?} is not valid for property {}",
                value, property.name
            )));
        }

        // Vérifier que la valeur n'est pas None pour une propriété requise
        if property.is_required && matches!(value, PropertyValue::None) {
            return Err(PropertyError::RequiredValueMissing(property.name.clone()));
        }

        // Générer un ID pour la valeur
        let value_id = PropertyValueId(self.next_value_id);
        self.next_value_id += 1;

        // Créer l'entrée
        let entry = PropertyValueEntry {
            id: value_id,
            property_id,
            entity_id,
            entity_type,
            value,
            created_at: chrono::Utc::now().timestamp() as u64,
            updated_at: chrono::Utc::now().timestamp() as u64,
        };

        // Ajouter au stockage
        self.property_values
            .entry(entity_id)
            .or_default()
            .push(entry);

        Ok(())
    }

    /// Récupère la valeur d'une propriété pour une entité
    pub fn get_property_value(
        &self,
        property_id: PropertyId,
        entity_id: u64,
        entity_type: PropertyEntityType,
    ) -> Option<&PropertyValue> {
        self.property_values.get(&entity_id).and_then(|values| {
            values
                .iter()
                .find(|v| v.property_id == property_id && v.entity_type == entity_type)
                .map(|v| &v.value)
        })
    }

    /// Récupère toutes les valeurs de propriétés pour une entité
    pub fn get_all_property_values(&self, entity_id: u64) -> Vec<&PropertyValueEntry> {
        self.property_values
            .get(&entity_id)
            .map_or(vec![], |values| {
                values.iter().filter(|v| v.entity_id == entity_id).collect()
            })
    }

    /// Supprime une valeur de propriété pour une entité
    pub fn remove_property_value(
        &mut self,
        property_id: PropertyId,
        entity_id: u64,
        entity_type: PropertyEntityType,
    ) -> Result<(), PropertyError> {
        if let Some(values) = self.property_values.get_mut(&entity_id) {
            let index = values.iter().position(|v| {
                v.property_id == property_id
                    && v.entity_id == entity_id
                    && v.entity_type == entity_type
            });

            if let Some(index) = index {
                values.remove(index);
                Ok(())
            } else {
                Err(PropertyError::NotFound(format!(
                    "Property {} not found for entity {}",
                    property_id.0, entity_id
                )))
            }
        } else {
            Err(PropertyError::NotFound(format!(
                "Entity {} not found",
                entity_id
            )))
        }
    }

    /// Filtre les entités par propriété
    pub fn filter_by_property(&self, property_id: PropertyId, filter: PropertyFilter) -> Vec<u64> {
        let mut results = Vec::new();

        for (entity_id, values) in &self.property_values {
            for value in values {
                if value.property_id == property_id && Self::matches_filter(&value.value, &filter) {
                    results.push(*entity_id);
                    break;
                }
            }
        }

        results
    }

    /// Vérifie si une valeur correspond au filtre
    fn matches_filter(value: &PropertyValue, filter: &PropertyFilter) -> bool {
        match filter {
            PropertyFilter::Equals(filter_value) => {
                PropertyValue::as_string(value) == PropertyValue::as_string(filter_value)
            }
            PropertyFilter::NotEquals(filter_value) => {
                PropertyValue::as_string(value) != PropertyValue::as_string(filter_value)
            }
            PropertyFilter::GreaterThan(filter_value) => {
                if let (Some(a), Some(b)) = (value.as_number(), filter_value.as_number()) {
                    a > b
                } else {
                    false
                }
            }
            PropertyFilter::LessThan(filter_value) => {
                if let (Some(a), Some(b)) = (value.as_number(), filter_value.as_number()) {
                    a < b
                } else {
                    false
                }
            }
            PropertyFilter::Contains(text) => {
                PropertyValue::as_string(value).is_some_and(|v| v.contains(text))
            }
            PropertyFilter::In(values) => values
                .iter()
                .any(|v| Self::matches_filter(value, &PropertyFilter::Equals(v.clone()))),
            PropertyFilter::IsEmpty => matches!(value, PropertyValue::None),
            PropertyFilter::IsNotEmpty => !matches!(value, PropertyValue::None),
        }
    }
}

impl Default for PropertySystem {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// FILTRES DE PROPRIÉTÉS
// ============================================================================

/// Filtre pour les valeurs de propriétés
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PropertyFilter {
    /// Égal à
    Equals(PropertyValue),
    /// Différent de
    NotEquals(PropertyValue),
    /// Supérieur à
    GreaterThan(PropertyValue),
    /// Inférieur à
    LessThan(PropertyValue),
    /// Contient (pour texte)
    Contains(String),
    /// Dans une liste de valeurs
    In(Vec<PropertyValue>),
    /// Est vide
    IsEmpty,
    /// N'est pas vide
    IsNotEmpty,
}

// ============================================================================
// MISE À JOUR D'UNE PROPRIÉTÉ
// ============================================================================

/// Mise à jour partielle d'une propriété
#[derive(Debug, Clone, Default)]
pub struct PropertyUpdate {
    pub name: Option<String>,
    pub property_type: Option<PropertyType>,
    pub default_value: Option<PropertyValue>,
    pub options: Option<Vec<String>>,
    pub is_required: Option<bool>,
    pub description: Option<String>,
    pub display_order: Option<u32>,
}

// ============================================================================
// PROPRIÉTÉS SYSTÈME
// ============================================================================

/// Crée les propriétés système par défaut
pub fn create_system_properties() -> Vec<Property> {
    vec![
        Property {
            id: PropertyId(1),
            name: "title".to_string(),
            normalized_name: "title".to_string(),
            property_type: PropertyType::Text,
            default_value: None,
            options: None,
            is_system: true,
            is_required: true,
            description: Some("Titre de l'entité".to_string()),
            display_order: 1,
            created_at: 0,
            updated_at: 0,
        },
        Property {
            id: PropertyId(2),
            name: "alias".to_string(),
            normalized_name: "alias".to_string(),
            property_type: PropertyType::Text,
            default_value: None,
            options: None,
            is_system: true,
            is_required: false,
            description: Some("Autres noms pour l'entité".to_string()),
            display_order: 2,
            created_at: 0,
            updated_at: 0,
        },
        Property {
            id: PropertyId(3),
            name: "tags".to_string(),
            normalized_name: "tags".to_string(),
            property_type: PropertyType::MultiSelect,
            default_value: None,
            options: None,
            is_system: true,
            is_required: false,
            description: Some("Tags associés à l'entité".to_string()),
            display_order: 3,
            created_at: 0,
            updated_at: 0,
        },
        Property {
            id: PropertyId(4),
            name: "created".to_string(),
            normalized_name: "created".to_string(),
            property_type: PropertyType::DateTime,
            default_value: None,
            options: None,
            is_system: true,
            is_required: false,
            description: Some("Date de création".to_string()),
            display_order: 4,
            created_at: 0,
            updated_at: 0,
        },
        Property {
            id: PropertyId(5),
            name: "updated".to_string(),
            normalized_name: "updated".to_string(),
            property_type: PropertyType::DateTime,
            default_value: None,
            options: None,
            is_system: true,
            is_required: false,
            description: Some("Date de dernière modification".to_string()),
            display_order: 5,
            created_at: 0,
            updated_at: 0,
        },
    ]
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_property_creation() {
        let mut system = PropertySystem::new();

        let property = Property::new("status".to_string(), PropertyType::Select, false);
        let property_id = system.create_property(property).unwrap();

        assert_eq!(property_id.0, 1);
        assert!(system.get_property(property_id).is_some());
    }

    #[test]
    fn test_property_name_normalization() {
        assert_eq!(Property::normalize_name("Status"), "status");
        assert_eq!(Property::normalize_name("My Property"), "my_property");
        assert_eq!(Property::normalize_name("my-property"), "my_property");
    }

    #[test]
    fn test_property_value_validation() {
        let property = Property::with_options(
            "status".to_string(),
            PropertyType::Select,
            vec![
                "todo".to_string(),
                "in_progress".to_string(),
                "done".to_string(),
            ],
            false,
        );

        assert!(property.is_valid_value(&PropertyValue::Select("todo".to_string())));
        assert!(property.is_valid_value(&PropertyValue::Select("done".to_string())));
        assert!(!property.is_valid_value(&PropertyValue::Select("invalid".to_string())));
    }

    #[test]
    fn test_set_and_get_property_value() {
        let mut system = PropertySystem::new();

        // Créer une propriété
        let property = Property::new("priority".to_string(), PropertyType::Number, false);
        let property_id = system.create_property(property).unwrap();

        // Définir une valeur
        system
            .set_property_value(
                property_id,
                1, // entity_id
                PropertyEntityType::Page,
                PropertyValue::Number(5.0),
            )
            .unwrap();

        // Récupérer la valeur
        let value = system.get_property_value(property_id, 1, PropertyEntityType::Page);
        assert!(value.is_some());

        if let PropertyValue::Number(v) = value.unwrap() {
            assert_eq!(*v, 5.0);
        } else {
            panic!("Expected Number value");
        }
    }

    #[test]
    fn test_filter_by_property() {
        let mut system = PropertySystem::new();

        // Créer une propriété
        let property = Property::new("priority".to_string(), PropertyType::Number, false);
        let property_id = system.create_property(property).unwrap();

        // Définir des valeurs pour différentes entités
        system
            .set_property_value(
                property_id,
                1,
                PropertyEntityType::Page,
                PropertyValue::Number(5.0),
            )
            .unwrap();
        system
            .set_property_value(
                property_id,
                2,
                PropertyEntityType::Page,
                PropertyValue::Number(3.0),
            )
            .unwrap();
        system
            .set_property_value(
                property_id,
                3,
                PropertyEntityType::Page,
                PropertyValue::Number(8.0),
            )
            .unwrap();

        // Filtrer par valeur > 4
        let mut results = system.filter_by_property(
            property_id,
            PropertyFilter::GreaterThan(PropertyValue::Number(4.0)),
        );
        results.sort();

        assert_eq!(results, vec![1, 3]);
    }

    #[test]
    fn test_system_properties() {
        let properties = create_system_properties();
        assert_eq!(properties.len(), 5);
        assert!(properties.iter().all(|p| p.is_system));
    }

    #[test]
    fn test_property_value_as_string() {
        let text_value = PropertyValue::Text("hello".to_string());
        assert_eq!(text_value.as_string(), Some("hello".to_string()));

        let number_value = PropertyValue::Number(42.0);
        assert_eq!(number_value.as_string(), Some("42".to_string()));

        let none_value = PropertyValue::None;
        assert_eq!(none_value.as_string(), None);
    }
}
