//! Note Craft Storage - Property System Storage
//!
//! Ce module fournit des fonctions de stockage pour les propriétés.
//! Il utilise directement SQLite et les types du module core.
//!
//! Phase 4 - Étape 4.2 (Property System)

use crate::schema;
use note_craft_core::property::{Property, PropertyEntityType, PropertyError, PropertyId, PropertyValue, PropertyValueId};
use rusqlite::{params, Connection, Result, Row};
use serde_json;

// ============================================================================
// FONCTIONS DE CONVERSION
// ============================================================================

/// Convertit un PropertyValue en chaîne pour le stockage
fn value_to_string(value: &PropertyValue) -> Option<String> {
    match value {
        PropertyValue::Text(v) => Some(v.clone()),
        PropertyValue::Select(v) => Some(v.clone()),
        PropertyValue::MultiSelect(v) => Some(v.join(",")),
        PropertyValue::Url(v) => Some(v.clone()),
        PropertyValue::Email(v) => Some(v.clone()),
        PropertyValue::Color(v) => Some(v.clone()),
        PropertyValue::Date(v) => Some(v.clone()),
        PropertyValue::DateTime(v) => Some(v.clone()),
        _ => None,
    }
}

/// Convertit un PropertyValue en nombre pour le stockage
fn value_to_number(value: &PropertyValue) -> Option<f64> {
    match value {
        PropertyValue::Number(v) => Some(*v),
        _ => None,
    }
}

/// Convertit un PropertyValue en booléen pour le stockage
fn value_to_boolean(value: &PropertyValue) -> Option<i32> {
    match value {
        PropertyValue::Boolean(v) => Some(if *v { 1 } else { 0 }),
        _ => None,
    }
}

/// Convertit une ligne en Property
fn row_to_property(row: &Row) -> Result<Property> {
    let id: i64 = row.get(0)?;
    let name: String = row.get(1)?;
    let normalized_name: String = row.get(2)?;
    let property_type_str: String = row.get(3)?;
    let default_value_json: String = row.get(4)?;
    let options_json: String = row.get(5)?;
    let is_system: i32 = row.get(6)?;
    let is_required: i32 = row.get(7)?;
    let description: Option<String> = row.get(8)?;
    let display_order: i64 = row.get(9)?;
    let created_at: i64 = row.get(10)?;
    let updated_at: i64 = row.get(11)?;

    let property_type: note_craft_core::property::PropertyType = property_type_str
        .parse()
        .unwrap_or(note_craft_core::property::PropertyType::Text);

    let default_value: Option<PropertyValue> = serde_json::from_str(&default_value_json).ok();
    let options: Option<Vec<String>> = serde_json::from_str(&options_json).ok();

    Ok(Property {
        id: PropertyId(id as u64),
        name,
        normalized_name,
        property_type,
        default_value,
        options,
        is_system: is_system != 0,
        is_required: is_required != 0,
        description,
        display_order: display_order as u32,
        created_at: created_at as u64,
        updated_at: updated_at as u64,
    })
}

// ============================================================================
// FONCTIONS DE STOCKAGE DES PROPRIÉTÉS
// ============================================================================

/// Crée une propriété dans la base de données
pub fn create_property(conn: &Connection, property: &Property) -> Result<PropertyId> {
    let id: i64 = conn.query_row(
        "INSERT INTO properties (name, normalized_name, property_type, default_value, 
         options, is_system, is_required, description, display_order, 
         created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11) 
         RETURNING id",
        params![
            &property.name,
            &property.normalized_name,
            property.property_type.to_string(),
            serde_json::to_string(&property.default_value).unwrap_or_else(|_| "null".to_string()),
            serde_json::to_string(&property.options).unwrap_or_else(|_| "null".to_string()),
            property.is_system as i32,
            property.is_required as i32,
            property.description.as_deref().unwrap_or(""),
            property.display_order as i64,
            property.created_at as i64,
            property.updated_at as i64,
        ],
        |row| row.get(0),
    )?;

    Ok(PropertyId(id as u64))
}

/// Récupère une propriété par ID
pub fn get_property(conn: &Connection, id: PropertyId) -> Result<Option<Property>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, normalized_name, property_type, default_value, 
         options, is_system, is_required, description, display_order, 
         created_at, updated_at FROM properties WHERE id = ?1",
    )?;

    stmt.query_row(params![id.0 as i64], |row| row_to_property(row))
        .map(Some)
        .or_else(|e| {
            if e.to_string().contains("Query returned no rows") {
                Ok(None)
            } else {
                Err(e)
            }
        })
}

/// Récupère une propriété par nom
pub fn get_property_by_name(conn: &Connection, name: &str) -> Result<Option<Property>> {
    let normalized = Property::normalize_name(name);
    let mut stmt = conn.prepare(
        "SELECT id, name, normalized_name, property_type, default_value, 
         options, is_system, is_required, description, display_order, 
         created_at, updated_at FROM properties WHERE normalized_name = ?1",
    )?;

    stmt.query_row(params![&normalized], |row| row_to_property(row))
        .map(Some)
        .or_else(|e| {
            if e.to_string().contains("Query returned no rows") {
                Ok(None)
            } else {
                Err(e)
            }
        })
}

/// Liste toutes les propriétés
pub fn list_properties(conn: &Connection) -> Result<Vec<Property>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, normalized_name, property_type, default_value, 
         options, is_system, is_required, description, display_order, 
         created_at, updated_at FROM properties ORDER BY display_order",
    )?;

    let properties: Vec<Property> = stmt
        .query_map(params![], |row| row_to_property(row))?
        .filter_map(|r| r.ok())
        .collect();

    Ok(properties)
}

/// Met à jour une propriété
pub fn update_property(conn: &Connection, id: PropertyId, updates: &Property) -> Result<()> {
    conn.execute(
        "UPDATE properties SET name = ?1, normalized_name = ?2, property_type = ?3, 
         default_value = ?4, options = ?5, is_system = ?6, is_required = ?7, 
         description = ?8, display_order = ?9, updated_at = ?10 
         WHERE id = ?11",
        params![
            &updates.name,
            &updates.normalized_name,
            updates.property_type.to_string(),
            serde_json::to_string(&updates.default_value).unwrap_or_else(|_| "null".to_string()),
            serde_json::to_string(&updates.options).unwrap_or_else(|_| "null".to_string()),
            updates.is_system as i32,
            updates.is_required as i32,
            updates.description.as_deref().unwrap_or(""),
            updates.display_order as i64,
            chrono::Utc::now().timestamp() as i64,
            id.0 as i64,
        ],
    )?;

    Ok(())
}

/// Supprime une propriété
pub fn delete_property(conn: &Connection, id: PropertyId) -> Result<()> {
    conn.execute("DELETE FROM properties WHERE id = ?1", params![id.0 as i64])?;
    Ok(())
}

// ============================================================================
// FONCTIONS DE STOCKAGE DES VALEURS DE PROPRIÉTÉS
// ============================================================================

/// Définit une valeur de propriété pour une entité
pub fn set_property_value(
    conn: &Connection,
    property_id: PropertyId,
    entity_id: u64,
    entity_type: PropertyEntityType,
    value: &PropertyValue,
) -> Result<PropertyValueId> {
    let id: i64 = conn.query_row(
        "INSERT INTO property_values (property_id, entity_id, entity_type, value_type, 
         string_value, number_value, boolean_value, date_value, created_at, updated_at) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10) 
         ON CONFLICT (property_id, entity_id, entity_type) 
         DO UPDATE SET string_value = ?5, number_value = ?6, boolean_value = ?7, 
                     date_value = ?8, updated_at = ?10 
         RETURNING id",
        params![
            property_id.0 as i64,
            entity_id as i64,
            entity_type as i64,
            value.value_type().to_string(),
            value_to_string(value),
            value_to_number(value),
            value_to_boolean(value),
            value_to_string(value), // date_value peut contenir date ou datetime
            chrono::Utc::now().timestamp() as i64,
            chrono::Utc::now().timestamp() as i64,
        ],
        |row| row.get(0),
    )?;

    Ok(PropertyValueId(id as u64))
}

/// Récupère la valeur d'une propriété pour une entité
pub fn get_property_value(
    conn: &Connection,
    property_id: PropertyId,
    entity_id: u64,
    entity_type: PropertyEntityType,
) -> Result<Option<PropertyValue>> {
    let mut stmt = conn.prepare(
        "SELECT value_type, string_value, number_value, boolean_value, date_value 
         FROM property_values WHERE property_id = ?1 AND entity_id = ?2 AND entity_type = ?3",
    )?;

    stmt.query_row(
        params![
            property_id.0 as i64,
            entity_id as i64,
            entity_type as i64,
        ],
        |row| {
            let value_type_str: String = row.get(0)?;
            let string_value: Option<String> = row.get(1)?;
            let number_value: Option<f64> = row.get(2)?;
            let boolean_value: Option<i32> = row.get(3)?;
            let date_value: Option<String> = row.get(4)?;

            let value_type: note_craft_core::property::PropertyType = value_type_str
                .parse()
                .unwrap_or(note_craft_core::property::PropertyType::Text);

            let value = match value_type {
                note_craft_core::property::PropertyType::Text => {
                    PropertyValue::Text(string_value.unwrap_or_default())
                }
                note_craft_core::property::PropertyType::Number => {
                    PropertyValue::Number(number_value.unwrap_or(0.0))
                }
                note_craft_core::property::PropertyType::Boolean => {
                    PropertyValue::Boolean(boolean_value.unwrap_or(0) != 0)
                }
                note_craft_core::property::PropertyType::Date => {
                    PropertyValue::Date(date_value.unwrap_or_default())
                }
                note_craft_core::property::PropertyType::DateTime => {
                    PropertyValue::DateTime(date_value.unwrap_or_default())
                }
                note_craft_core::property::PropertyType::Select => {
                    PropertyValue::Select(string_value.unwrap_or_default())
                }
                note_craft_core::property::PropertyType::MultiSelect => {
                    PropertyValue::MultiSelect(
                        string_value
                            .map(|s| s.split(",").map(String::from).collect())
                            .unwrap_or_default(),
                    )
                }
                note_craft_core::property::PropertyType::Url => {
                    PropertyValue::Url(string_value.unwrap_or_default())
                }
                note_craft_core::property::PropertyType::Email => {
                    PropertyValue::Email(string_value.unwrap_or_default())
                }
                note_craft_core::property::PropertyType::Color => {
                    PropertyValue::Color(string_value.unwrap_or_default())
                }
            };

            Ok(value)
        },
    )
    .map(Some)
    .or_else(|e| {
        if e.to_string().contains("Query returned no rows") {
            Ok(None)
        } else {
            Err(e)
        }
    })
}

/// Récupère toutes les valeurs de propriétés pour une entité
pub fn get_all_property_values(
    conn: &Connection,
    entity_id: u64,
    entity_type: PropertyEntityType,
) -> Result<Vec<(PropertyId, PropertyValue)>> {
    let mut stmt = conn.prepare(
        "SELECT property_id, value_type, string_value, number_value, boolean_value, date_value 
         FROM property_values WHERE entity_id = ?1 AND entity_type = ?2",
    )?;

    let rows = stmt.query_map(
        params![entity_id as i64, entity_type as i64],
        |row| {
            let property_id: i64 = row.get(0)?;
            let value_type_str: String = row.get(1)?;
            let string_value: Option<String> = row.get(2)?;
            let number_value: Option<f64> = row.get(3)?;
            let boolean_value: Option<i32> = row.get(4)?;
            let date_value: Option<String> = row.get(5)?;

            let value_type: note_craft_core::property::PropertyType = value_type_str
                .parse()
                .unwrap_or(note_craft_core::property::PropertyType::Text);

            let value = match value_type {
                note_craft_core::property::PropertyType::Text => {
                    PropertyValue::Text(string_value.unwrap_or_default())
                }
                note_craft_core::property::PropertyType::Number => {
                    PropertyValue::Number(number_value.unwrap_or(0.0))
                }
                note_craft_core::property::PropertyType::Boolean => {
                    PropertyValue::Boolean(boolean_value.unwrap_or(0) != 0)
                }
                note_craft_core::property::PropertyType::Date => {
                    PropertyValue::Date(date_value.unwrap_or_default())
                }
                note_craft_core::property::PropertyType::DateTime => {
                    PropertyValue::DateTime(date_value.unwrap_or_default())
                }
                note_craft_core::property::PropertyType::Select => {
                    PropertyValue::Select(string_value.unwrap_or_default())
                }
                note_craft_core::property::PropertyType::MultiSelect => {
                    PropertyValue::MultiSelect(
                        string_value
                            .map(|s| s.split(",").map(String::from).collect())
                            .unwrap_or_default(),
                    )
                }
                note_craft_core::property::PropertyType::Url => {
                    PropertyValue::Url(string_value.unwrap_or_default())
                }
                note_craft_core::property::PropertyType::Email => {
                    PropertyValue::Email(string_value.unwrap_or_default())
                }
                note_craft_core::property::PropertyType::Color => {
                    PropertyValue::Color(string_value.unwrap_or_default())
                }
            };

            Ok((PropertyId(property_id as u64), value))
        },
    )?;

    rows.collect()
}

/// Supprime une valeur de propriété
pub fn delete_property_value(
    conn: &Connection,
    property_id: PropertyId,
    entity_id: u64,
    entity_type: PropertyEntityType,
) -> Result<()> {
    conn.execute(
        "DELETE FROM property_values WHERE property_id = ?1 AND entity_id = ?2 AND entity_type = ?3",
        params![
            property_id.0 as i64,
            entity_id as i64,
            entity_type as i64,
        ],
    )?;

    Ok(())
}

/// Supprime toutes les valeurs de propriétés pour une entité
pub fn delete_all_property_values(conn: &Connection, entity_id: u64, entity_type: PropertyEntityType) -> Result<()> {
    conn.execute(
        "DELETE FROM property_values WHERE entity_id = ?1 AND entity_type = ?2",
        params![entity_id as i64, entity_type as i64],
    )?;

    Ok(())
}

/// Filtre les entités par valeur de propriété
pub fn filter_by_property(
    conn: &Connection,
    property_id: PropertyId,
    value: &PropertyValue,
) -> Result<Vec<u64>> {
    let mut results = Vec::new();

    // Pour les propriétés de type Text/Select/Date etc.
    if let Some(text_value) = value.as_string() {
        let mut stmt = conn.prepare(
            "SELECT DISTINCT entity_id FROM property_values WHERE property_id = ?1 AND string_value = ?2",
        )?;

        let entity_ids: Vec<u64> = stmt
            .query_map(params![property_id.0 as i64, &text_value], |row| {
                let entity_id: i64 = row.get(0)?;
                Ok(entity_id as u64)
            })?
            .filter_map(|r| r.ok())
            .collect();

        results.extend(entity_ids);
    }

    // Pour les nombres
    if let Some(number_value) = value.as_number() {
        let mut stmt = conn.prepare(
            "SELECT DISTINCT entity_id FROM property_values WHERE property_id = ?1 AND number_value = ?2",
        )?;

        let entity_ids: Vec<u64> = stmt
            .query_map(params![property_id.0 as i64, number_value], |row| {
                let entity_id: i64 = row.get(0)?;
                Ok(entity_id as u64)
            })?
            .filter_map(|r| r.ok())
            .collect();

        results.extend(entity_ids);
    }

    // Pour les booléens
    if let Some(boolean_value) = value.as_boolean() {
        let bool_int = if boolean_value { 1 } else { 0 };
        let mut stmt = conn.prepare(
            "SELECT DISTINCT entity_id FROM property_values WHERE property_id = ?1 AND boolean_value = ?2",
        )?;

        let entity_ids: Vec<u64> = stmt
            .query_map(params![property_id.0 as i64, bool_int], |row| {
                let entity_id: i64 = row.get(0)?;
                Ok(entity_id as u64)
            })?
            .filter_map(|r| r.ok())
            .collect();

        results.extend(entity_ids);
    }

    Ok(results)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use note_craft_core::property::{Property, PropertyType, PropertyValue};
    use tempfile::tempdir;

    #[test]
    fn test_create_and_get_property() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let conn = schema::Schema::initialize_database(&db_path).unwrap();

        let property = Property::new("status".to_string(), PropertyType::Text, false);
        let property_id = create_property(&conn, &property).unwrap();

        let retrieved = get_property(&conn, property_id).unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "status");
    }

    #[test]
    fn test_set_and_get_property_value() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let conn = schema::Schema::initialize_database(&db_path).unwrap();

        // Créer une propriété
        let property = Property::new("priority".to_string(), PropertyType::Number, false);
        let property_id = create_property(&conn, &property).unwrap();

        // Définir une valeur
        set_property_value(
            &conn,
            property_id,
            1,
            PropertyEntityType::Page,
            &PropertyValue::Number(5.0),
        )
        .unwrap();

        // Récupérer la valeur
        let value = get_property_value(&conn, property_id, 1, PropertyEntityType::Page)
            .unwrap()
            .unwrap();

        if let PropertyValue::Number(v) = value {
            assert_eq!(v, 5.0);
        } else {
            panic!("Expected Number value");
        }
    }

    #[test]
    fn test_list_properties() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let conn = schema::Schema::initialize_database(&db_path).unwrap();

        // Créer plusieurs propriétés
        let prop1 = Property::new("status".to_string(), PropertyType::Text, false);
        let prop2 = Property::new("priority".to_string(), PropertyType::Number, false);
        create_property(&conn, &prop1).unwrap();
        create_property(&conn, &prop2).unwrap();

        let properties = list_properties(&conn).unwrap();
        assert_eq!(properties.len(), 2);
    }

    #[test]
    fn test_delete_property() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let conn = schema::Schema::initialize_database(&db_path).unwrap();

        // Créer une propriété
        let property = Property::new("temp".to_string(), PropertyType::Text, false);
        let property_id = create_property(&conn, &property).unwrap();

        // Supprimer la propriété
        delete_property(&conn, property_id).unwrap();

        // Vérifier qu'elle n'existe plus
        let retrieved = get_property(&conn, property_id).unwrap();
        assert!(retrieved.is_none());
    }
}
