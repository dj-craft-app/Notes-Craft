//! SQLite Schema Module
//!
//! This module provides the complete SQLite schema for Note Craft.
//! It defines all tables, indexes, and constraints for the application database.
//!
//! Note: Phase 4 - Step 4.1 (SQLite Schema)

use chrono::Utc;
use rusqlite::{Connection, Result};
use std::fs;
use std::path::Path;

// ============================================================================
// Database Version
// ============================================================================

/// Current database schema version
pub const DB_VERSION: u32 = 1;

/// Database name
pub const DB_NAME: &str = "note_craft.db";

// ============================================================================
// Table Definitions
// ============================================================================

/// SQL statements for creating all tables
pub struct Schema;

impl Schema {
    /// Gets the complete schema creation SQL
    pub fn create_all_tables() -> String {
        format!(
            "{};\n{};\n{};\n{};\n{};\n{};\n{};\n{};\n{};\n{};\n{};\n{}",
            Self::create_pages_table(),
            Self::create_blocks_table(),
            Self::create_links_table(),
            Self::create_tags_table(),
            Self::create_properties_table(),
            Self::create_property_values_table(),
            Self::create_journals_table(),
            Self::create_assets_table(),
            Self::create_graph_nodes_table(),
            Self::create_graph_edges_table(),
            Self::create_search_index_table(),
            Self::create_metadata_table()
        )
    }

    /// Creates the pages table
    pub fn create_pages_table() -> String {
        r#"CREATE TABLE IF NOT EXISTS pages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            uuid TEXT NOT NULL UNIQUE,
            path TEXT NOT NULL UNIQUE,
            title TEXT NOT NULL,
            filename TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            accessed_at INTEGER,
            size INTEGER DEFAULT 0,
            word_count INTEGER DEFAULT 0,
            is_journal BOOLEAN DEFAULT FALSE,
            journal_date TEXT,
            parent_id INTEGER,
            order_index INTEGER DEFAULT 0,
            is_deleted BOOLEAN DEFAULT FALSE,
            deleted_at INTEGER,
            FOREIGN KEY (parent_id) REFERENCES pages(id) ON DELETE SET NULL
        )"#
        .to_string()
    }

    /// Creates the blocks table
    pub fn create_blocks_table() -> String {
        r#"CREATE TABLE IF NOT EXISTS blocks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            uuid TEXT NOT NULL UNIQUE,
            page_id INTEGER NOT NULL,
            content TEXT NOT NULL,
            block_type TEXT NOT NULL DEFAULT 'paragraph',
            format TEXT DEFAULT 'markdown',
            order_index INTEGER NOT NULL,
            indent_level INTEGER DEFAULT 0,
            is_collapsed BOOLEAN DEFAULT FALSE,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            is_deleted BOOLEAN DEFAULT FALSE,
            deleted_at INTEGER,
            FOREIGN KEY (page_id) REFERENCES pages(id) ON DELETE CASCADE
        )"#
        .to_string()
    }

    /// Creates the links table
    pub fn create_links_table() -> String {
        r#"CREATE TABLE IF NOT EXISTS links (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            uuid TEXT NOT NULL UNIQUE,
            source_block_id INTEGER NOT NULL,
            source_page_id INTEGER NOT NULL,
            target_page_id INTEGER,
            target_block_id INTEGER,
            link_type TEXT NOT NULL DEFAULT 'page',
            link_text TEXT,
            url TEXT,
            anchor TEXT,
            is_resolved BOOLEAN DEFAULT TRUE,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            FOREIGN KEY (source_block_id) REFERENCES blocks(id) ON DELETE CASCADE,
            FOREIGN KEY (source_page_id) REFERENCES pages(id) ON DELETE CASCADE,
            FOREIGN KEY (target_page_id) REFERENCES pages(id) ON DELETE SET NULL,
            FOREIGN KEY (target_block_id) REFERENCES blocks(id) ON DELETE SET NULL
        )"#
        .to_string()
    }

    /// Creates the tags table
    pub fn create_tags_table() -> String {
        r#"CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            normalized_name TEXT NOT NULL UNIQUE,
            color TEXT,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        )"#
        .to_string()
    }

    /// Creates the properties table
    pub fn create_properties_table() -> String {
        r#"CREATE TABLE IF NOT EXISTS properties (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            normalized_name TEXT NOT NULL UNIQUE,
            property_type TEXT NOT NULL DEFAULT 'text',
            default_value TEXT,
            options TEXT,
            is_system BOOLEAN DEFAULT FALSE,
            is_required BOOLEAN DEFAULT FALSE,
            description TEXT,
            display_order INTEGER DEFAULT 0,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        )"#
        .to_string()
    }

    /// Creates the property values table
    pub fn create_property_values_table() -> String {
        r#"CREATE TABLE IF NOT EXISTS property_values (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            property_id INTEGER NOT NULL,
            entity_id INTEGER NOT NULL,
            entity_type INTEGER NOT NULL,
            value_type TEXT NOT NULL,
            string_value TEXT,
            number_value REAL,
            boolean_value INTEGER,
            date_value TEXT,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            FOREIGN KEY (property_id) REFERENCES properties(id) ON DELETE CASCADE,
            UNIQUE(property_id, entity_id, entity_type)
        )"#
        .to_string()
    }

    /// Creates the journals table
    pub fn create_journals_table() -> String {
        r#"CREATE TABLE IF NOT EXISTS journals (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            page_id INTEGER NOT NULL UNIQUE,
            date TEXT NOT NULL UNIQUE,
            weekday INTEGER NOT NULL,
            week_number INTEGER NOT NULL,
            month INTEGER NOT NULL,
            year INTEGER NOT NULL,
            quarter INTEGER NOT NULL,
            created_at INTEGER NOT NULL,
            FOREIGN KEY (page_id) REFERENCES pages(id) ON DELETE CASCADE
        )"#
        .to_string()
    }

    /// Creates the assets table
    pub fn create_assets_table() -> String {
        r#"CREATE TABLE IF NOT EXISTS assets (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            uuid TEXT NOT NULL UNIQUE,
            filename TEXT NOT NULL,
            original_filename TEXT NOT NULL,
            path TEXT NOT NULL UNIQUE,
            asset_type TEXT NOT NULL,
            mime_type TEXT NOT NULL,
            size INTEGER NOT NULL,
            width INTEGER,
            height INTEGER,
            hash TEXT NOT NULL,
            embedded_block_id INTEGER,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            is_deleted BOOLEAN DEFAULT FALSE,
            deleted_at INTEGER,
            FOREIGN KEY (embedded_block_id) REFERENCES blocks(id) ON DELETE SET NULL
        )"#
        .to_string()
    }

    /// Creates the graph nodes table
    pub fn create_graph_nodes_table() -> String {
        r#"CREATE TABLE IF NOT EXISTS graph_nodes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            uuid TEXT NOT NULL UNIQUE,
            node_type TEXT NOT NULL DEFAULT 'page',
            page_id INTEGER,
            block_id INTEGER,
            tag_id INTEGER,
            title TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            FOREIGN KEY (page_id) REFERENCES pages(id) ON DELETE CASCADE,
            FOREIGN KEY (block_id) REFERENCES blocks(id) ON DELETE CASCADE,
            FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
        )"#
        .to_string()
    }

    /// Creates the graph edges table
    pub fn create_graph_edges_table() -> String {
        r#"CREATE TABLE IF NOT EXISTS graph_edges (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            source_node_id INTEGER NOT NULL,
            target_node_id INTEGER NOT NULL,
            edge_type TEXT NOT NULL DEFAULT 'link',
            weight REAL DEFAULT 1.0,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            FOREIGN KEY (source_node_id) REFERENCES graph_nodes(id) ON DELETE CASCADE,
            FOREIGN KEY (target_node_id) REFERENCES graph_nodes(id) ON DELETE CASCADE,
            UNIQUE(source_node_id, target_node_id, edge_type)
        )"#
        .to_string()
    }

    /// Creates the search index table
    pub fn create_search_index_table() -> String {
        r#"CREATE VIRTUAL TABLE IF NOT EXISTS search_index USING fts5(
            content,
            page_id,
            block_id,
            title,
            tags,
            properties,
            tokenize = "unicode61 remove_diacritics 2",
            prefix = "2 3 4 5 6",
            detail = "column"
        )"#
        .to_string()
    }

    /// Creates the metadata table
    pub fn create_metadata_table() -> String {
        r#"CREATE TABLE IF NOT EXISTS metadata (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            key TEXT NOT NULL UNIQUE,
            value TEXT,
            updated_at INTEGER NOT NULL
        )"#
        .to_string()
    }

    // ============================================================================
    // Indexes
    // ============================================================================

    /// Gets all index creation SQL statements
    pub fn create_all_indexes() -> Vec<String> {
        vec![
            // Pages indexes
            "CREATE INDEX IF NOT EXISTS idx_pages_path ON pages(path)".to_string(),
            "CREATE INDEX IF NOT EXISTS idx_pages_title ON pages(title)".to_string(),
            "CREATE INDEX IF NOT EXISTS idx_pages_parent_id ON pages(parent_id)".to_string(),
            "CREATE INDEX IF NOT EXISTS idx_pages_journal_date ON pages(journal_date)".to_string(),
            "CREATE INDEX IF NOT EXISTS idx_pages_is_journal ON pages(is_journal)".to_string(),
            "CREATE INDEX IF NOT EXISTS idx_pages_created_at ON pages(created_at)".to_string(),
            "CREATE INDEX IF NOT EXISTS idx_pages_updated_at ON pages(updated_at)".to_string(),

            // Blocks indexes
            "CREATE INDEX IF NOT EXISTS idx_blocks_page_id ON blocks(page_id)".to_string(),
            "CREATE INDEX IF NOT EXISTS idx_blocks_order_index ON blocks(page_id, order_index)".to_string(),
            "CREATE INDEX IF NOT EXISTS idx_blocks_type ON blocks(block_type)".to_string(),
            "CREATE INDEX IF NOT EXISTS idx_blocks_created_at ON blocks(created_at)".to_string(),
            "CREATE INDEX IF NOT EXISTS idx_blocks_updated_at ON blocks(updated_at)".to_string(),

            // Links indexes
            "CREATE INDEX IF NOT EXISTS idx_links_source_block_id ON links(source_block_id)".to_string(),
            "CREATE INDEX IF NOT EXISTS idx_links_source_page_id ON links(source_page_id)".to_string(),
            "CREATE INDEX IF NOT EXISTS idx_links_target_page_id ON links(target_page_id)".to_string(),
            "CREATE INDEX IF NOT EXISTS idx_links_target_block_id ON links(target_block_id)".to_string(),
            "CREATE INDEX IF NOT EXISTS idx_links_type ON links(link_type)".to_string(),
            "CREATE INDEX IF NOT EXISTS idx_links_is_resolved ON links(is_resolved)".to_string(),

            // Tags indexes
            "CREATE INDEX IF NOT EXISTS idx_tags_name ON tags(name)".to_string(),
            "CREATE INDEX IF NOT EXISTS idx_tags_normalized_name ON tags(normalized_name)".to_string(),

            // Properties indexes
            "CREATE INDEX IF NOT EXISTS idx_properties_name ON properties(name)".to_string(),
            "CREATE INDEX IF NOT EXISTS idx_properties_normalized_name ON properties(normalized_name)".to_string(),
            "CREATE INDEX IF NOT EXISTS idx_properties_type ON properties(property_type)".to_string(),

            // Property values indexes
            "CREATE INDEX IF NOT EXISTS idx_property_values_property_id ON property_values(property_id)".to_string(),
            "CREATE INDEX IF NOT EXISTS idx_property_values_entity_id ON property_values(entity_id)".to_string(),
            "CREATE INDEX IF NOT EXISTS idx_property_values_entity_type ON property_values(entity_type)".to_string(),
            "CREATE INDEX IF NOT EXISTS idx_property_values_value_type ON property_values(value_type)".to_string(),
            "CREATE INDEX IF NOT EXISTS idx_property_values_string ON property_values(string_value)".to_string(),
            "CREATE INDEX IF NOT EXISTS idx_property_values_number ON property_values(number_value)".to_string(),
            "CREATE INDEX IF NOT EXISTS idx_property_values_date ON property_values(date_value)".to_string(),
            "CREATE INDEX IF NOT EXISTS idx_property_values_entity ON property_values(entity_id, entity_type)".to_string(),

            // Journals indexes
            "CREATE INDEX IF NOT EXISTS idx_journals_date ON journals(date)".to_string(),
            "CREATE INDEX IF NOT EXISTS idx_journals_weekday ON journals(weekday)".to_string(),
            "CREATE INDEX IF NOT EXISTS idx_journals_month ON journals(month)".to_string(),
            "CREATE INDEX IF NOT EXISTS idx_journals_year ON journals(year)".to_string(),
            "CREATE INDEX IF NOT EXISTS idx_journals_week_number ON journals(week_number)".to_string(),
            "CREATE INDEX IF NOT EXISTS idx_journals_quarter ON journals(quarter)".to_string(),

            // Assets indexes
            "CREATE INDEX IF NOT EXISTS idx_assets_path ON assets(path)".to_string(),
            "CREATE INDEX IF NOT EXISTS idx_assets_filename ON assets(filename)".to_string(),
            "CREATE INDEX IF NOT EXISTS idx_assets_type ON assets(asset_type)".to_string(),
            "CREATE INDEX IF NOT EXISTS idx_assets_hash ON assets(hash)".to_string(),
            "CREATE INDEX IF NOT EXISTS idx_assets_embedded ON assets(embedded_block_id)".to_string(),

            // Graph nodes indexes
            "CREATE INDEX IF NOT EXISTS idx_graph_nodes_type ON graph_nodes(node_type)".to_string(),
            "CREATE INDEX IF NOT EXISTS idx_graph_nodes_title ON graph_nodes(title)".to_string(),

            // Graph edges indexes
            "CREATE INDEX IF NOT EXISTS idx_graph_edges_source ON graph_edges(source_node_id)".to_string(),
            "CREATE INDEX IF NOT EXISTS idx_graph_edges_target ON graph_edges(target_node_id)".to_string(),
            "CREATE INDEX IF NOT EXISTS idx_graph_edges_type ON graph_edges(edge_type)".to_string(),
            "CREATE INDEX IF NOT EXISTS idx_graph_edges_weight ON graph_edges(weight)".to_string(),

            // Metadata indexes
            "CREATE INDEX IF NOT EXISTS idx_metadata_key ON metadata(key)".to_string(),
        ]
    }

    // ============================================================================
    // Triggers
    // ============================================================================

    /// Gets all trigger creation SQL statements
    pub fn create_all_triggers() -> Vec<String> {
        vec![
            // Trigger for updating page updated_at when a block is updated
            r#"CREATE TRIGGER IF NOT EXISTS trg_blocks_update_page_updated
                AFTER UPDATE ON blocks
                FOR EACH ROW
                WHEN OLD.updated_at != NEW.updated_at
                BEGIN
                    UPDATE pages
                    SET updated_at = NEW.updated_at
                    WHERE id = NEW.page_id;
                END"#.to_string(),

            // Trigger for updating page word count
            r#"CREATE TRIGGER IF NOT EXISTS trg_blocks_update_page_word_count
                AFTER INSERT ON blocks
                FOR EACH ROW
                BEGIN
                    UPDATE pages
                    SET word_count = (
                        SELECT SUM(length(content) - length(replace(content, ' ', '')) + 1)
                        FROM blocks
                        WHERE page_id = NEW.page_id AND is_deleted = FALSE
                    )
                    WHERE id = NEW.page_id;
                END"#.to_string(),

            // Trigger for updating search index on block insert
            r#"CREATE TRIGGER IF NOT EXISTS trg_blocks_insert_search_index
                AFTER INSERT ON blocks
                FOR EACH ROW
                WHEN NEW.is_deleted = FALSE
                BEGIN
                    INSERT INTO search_index (content, page_id, block_id, title, tags, properties)
                    VALUES (NEW.content, NEW.page_id, NEW.id, '', '', '');
                END"#.to_string(),

            // Trigger for updating search index on block update
            r#"CREATE TRIGGER IF NOT EXISTS trg_blocks_update_search_index
                AFTER UPDATE ON blocks
                FOR EACH ROW
                WHEN NEW.is_deleted = FALSE
                BEGIN
                    UPDATE search_index
                    SET content = NEW.content
                    WHERE block_id = NEW.id;
                END"#.to_string(),

            // Trigger for deleting from search index on block delete
            r#"CREATE TRIGGER IF NOT EXISTS trg_blocks_delete_search_index
                AFTER UPDATE ON blocks
                FOR EACH ROW
                WHEN NEW.is_deleted = TRUE
                BEGIN
                    DELETE FROM search_index WHERE block_id = NEW.id;
                END"#.to_string(),

            // Trigger for creating journal entry
            r#"CREATE TRIGGER IF NOT EXISTS trg_pages_insert_journal
                AFTER INSERT ON pages
                FOR EACH ROW
                WHEN NEW.is_journal = TRUE AND NEW.journal_date IS NOT NULL
                BEGIN
                    INSERT INTO journals (page_id, date, weekday, week_number, month, year, quarter, created_at)
                    VALUES (
                        NEW.id,
                        NEW.journal_date,
                        strftime('%w', NEW.journal_date),
                        strftime('%W', NEW.journal_date),
                        strftime('%m', NEW.journal_date),
                        strftime('%Y', NEW.journal_date),
                        (strftime('%m', NEW.journal_date) - 1) / 3 + 1,
                        NEW.created_at
                    );
                END"#.to_string(),

            // Trigger for updating graph nodes on page insert
            r#"CREATE TRIGGER IF NOT EXISTS trg_pages_insert_graph_node
                AFTER INSERT ON pages
                FOR EACH ROW
                WHEN NEW.is_deleted = FALSE
                BEGIN
                    INSERT INTO graph_nodes (uuid, node_type, page_id, title, created_at, updated_at)
                    VALUES (NEW.uuid, 'page', NEW.id, NEW.title, NEW.created_at, NEW.updated_at);
                END"#.to_string(),

            // Trigger for updating graph nodes on page update
            r#"CREATE TRIGGER IF NOT EXISTS trg_pages_update_graph_node
                AFTER UPDATE ON pages
                FOR EACH ROW
                WHEN NEW.is_deleted = FALSE
                BEGIN
                    UPDATE graph_nodes
                    SET title = NEW.title, updated_at = NEW.updated_at
                    WHERE page_id = NEW.id;
                END"#.to_string(),

            // Trigger for creating graph edges on link insert
            r#"CREATE TRIGGER IF NOT EXISTS trg_links_insert_graph_edge
                AFTER INSERT ON links
                FOR EACH ROW
                WHEN NEW.target_page_id IS NOT NULL AND NEW.is_resolved = TRUE
                BEGIN
                    INSERT INTO graph_edges (source_node_id, target_node_id, edge_type, weight, created_at, updated_at)
                    VALUES (
                        (SELECT id FROM graph_nodes WHERE page_id = NEW.source_page_id),
                        (SELECT id FROM graph_nodes WHERE page_id = NEW.target_page_id),
                        'link',
                        1.0,
                        NEW.created_at,
                        NEW.updated_at
                    );
                END"#.to_string(),
        ]
    }

    // ============================================================================
    // Database Operations
    // ============================================================================

    /// Initializes a new database with the complete schema
    pub fn initialize_database(path: &Path) -> Result<Connection> {
        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).map_err(|e| {
                    rusqlite::Error::SqliteFailure(
                        rusqlite::ffi::Error::new(-1),
                        Some(e.to_string()),
                    )
                })?;
            }
        }

        let conn = Connection::open(path)?;

        // Enable WAL mode for better performance
        conn.pragma_update(None, "journal_mode", "WAL")?;

        // Enable foreign keys
        conn.pragma_update(None, "foreign_keys", "ON")?;

        // Enable recursive triggers
        conn.pragma_update(None, "recursive_triggers", "ON")?;

        // Set busy timeout
        conn.pragma_update(None, "busy_timeout", "5000")?;

        // Set cache size
        conn.pragma_update(None, "cache_size", "-20000")?; // 20MB cache

        // Create tables
        let create_tables_sql = Self::create_all_tables();
        conn.execute_batch(&create_tables_sql)?;

        // Create indexes
        for index_sql in Self::create_all_indexes() {
            conn.execute_batch(&index_sql)?;
        }

        // Create triggers
        for trigger_sql in Self::create_all_triggers() {
            conn.execute_batch(&trigger_sql)?;
        }

        // Initialize metadata
        Self::initialize_metadata(&conn)?;

        Ok(conn)
    }

    /// Initializes the metadata table
    pub fn initialize_metadata(conn: &Connection) -> Result<()> {
        let version = Self::get_version(conn)?;
        if version == 0 {
            conn.execute(
                "INSERT INTO metadata (key, value, updated_at) VALUES (?, ?, ?)",
                ("db_version", DB_VERSION.to_string(), Utc::now().timestamp()),
            )?;
        }
        Ok(())
    }

    /// Gets the current database version
    pub fn get_version(conn: &Connection) -> Result<u32> {
        conn.query_row(
            "SELECT value FROM metadata WHERE key = 'db_version'",
            [],
            |row| row.get::<_, String>(0),
        )
        .map(|v| v.parse().unwrap_or(0))
        .or_else(|_| Ok(0))
    }

    /// Checks if the database needs migration
    pub fn needs_migration(conn: &Connection) -> Result<bool> {
        let version = Self::get_version(conn)?;
        Ok(version < DB_VERSION)
    }

    /// Applies all pending migrations
    pub fn migrate(conn: &Connection) -> Result<()> {
        let version = Self::get_version(conn)?;

        if version >= DB_VERSION {
            return Ok(());
        }

        // Apply migrations sequentially
        for v in (version + 1)..=DB_VERSION {
            Self::apply_migration(conn, v)?;
            // Update version
            conn.execute(
                "UPDATE metadata SET value = ?, updated_at = ? WHERE key = 'db_version'",
                (v.to_string(), Utc::now().timestamp()),
            )?;
        }

        Ok(())
    }

    /// Applies a specific migration version
    fn apply_migration(conn: &Connection, version: u32) -> Result<()> {
        match version {
            1 => Self::migration_v1(conn),
            _ => Ok(()), // No migration needed for future versions
        }
    }

    /// Migration to version 1 (initial schema)
    fn migration_v1(_conn: &Connection) -> Result<()> {
        // This is the initial schema, nothing to migrate
        Ok(())
    }

    // ============================================================================
    // Validation
    // ============================================================================

    /// Validates the database schema
    pub fn validate_schema(conn: &Connection) -> Result<Vec<String>> {
        let mut errors = Vec::new();

        // Check required tables
        let required_tables = [
            "pages",
            "blocks",
            "links",
            "tags",
            "properties",
            "property_values",
            "journals",
            "assets",
            "graph_nodes",
            "graph_edges",
            "metadata",
        ];

        for table in &required_tables {
            let count: i32 = conn.query_row(
                &format!(
                    "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='{}'",
                    table
                ),
                [],
                |row| row.get(0),
            )?;

            if count == 0 {
                errors.push(format!("Missing table: {}", table));
            }
        }

        // Check required indexes
        let required_indexes = [
            "idx_pages_path",
            "idx_pages_title",
            "idx_blocks_page_id",
            "idx_links_source_block_id",
            "idx_links_target_page_id",
            "idx_tags_name",
            "idx_properties_name",
        ];

        for index in &required_indexes {
            let count: i32 = conn.query_row(
                &format!(
                    "SELECT COUNT(*) FROM sqlite_master WHERE type='index' AND name='{}'",
                    index
                ),
                [],
                |row| row.get(0),
            )?;

            if count == 0 {
                errors.push(format!("Missing index: {}", index));
            }
        }

        Ok(errors)
    }

    // ============================================================================
    // Utility Functions
    // ============================================================================

    /// Gets the complete schema as a single SQL string (for backup/restore)
    pub fn get_complete_schema() -> String {
        let mut sql = String::new();

        // Add PRAGMA statements
        sql.push_str("PRAGMA journal_mode=WAL;\n");
        sql.push_str("PRAGMA foreign_keys=ON;\n");
        sql.push_str("PRAGMA recursive_triggers=ON;\n");
        sql.push_str("PRAGMA busy_timeout=5000;\n");
        sql.push_str("PRAGMA cache_size=-20000;\n\n");

        // Add table creation
        sql.push_str(&Self::create_all_tables());
        sql.push('\n');

        // Add indexes
        for index_sql in Self::create_all_indexes() {
            sql.push_str(&index_sql);
            sql.push_str(";\n");
        }
        sql.push('\n');

        // Add triggers
        for trigger_sql in Self::create_all_triggers() {
            sql.push_str(&trigger_sql);
            sql.push_str(";\n");
        }
        sql.push('\n');

        // Add metadata initialization
        sql.push_str(&format!(
            "INSERT INTO metadata (key, value, updated_at) VALUES ('db_version', '{}', {});\n",
            DB_VERSION,
            Utc::now().timestamp()
        ));

        sql
    }

    /// Drops all tables (useful for testing)
    pub fn drop_all_tables() -> String {
        let tables = [
            "metadata",
            "search_index",
            "graph_edges",
            "graph_nodes",
            "assets",
            "journals",
            "property_values",
            "properties",
            "tags",
            "links",
            "blocks",
            "pages",
        ];

        let mut sql = String::new();
        for table in tables {
            sql.push_str(&format!("DROP TABLE IF EXISTS {};\n", table));
        }
        sql
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_schema_creation() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let conn = Schema::initialize_database(&db_path).unwrap();

        // Verify tables exist
        let tables = [
            "pages",
            "blocks",
            "links",
            "tags",
            "properties",
            "property_values",
            "journals",
            "assets",
            "graph_nodes",
            "graph_edges",
            "metadata",
        ];

        for table in tables {
            let count: i32 = conn
                .query_row(
                    &format!(
                        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='{}'",
                        table
                    ),
                    [],
                    |row| row.get(0),
                )
                .unwrap();
            assert_eq!(count, 1, "Table {} should exist", table);
        }

        // Verify version
        let version = Schema::get_version(&conn).unwrap();
        assert_eq!(version, DB_VERSION);
    }

    #[test]
    fn test_schema_version() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test_version.db");

        let conn = Schema::initialize_database(&db_path).unwrap();
        let version = Schema::get_version(&conn).unwrap();
        assert_eq!(version, DB_VERSION);
    }

    #[test]
    fn test_needs_migration_false() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test_no_migration.db");

        let conn = Schema::initialize_database(&db_path).unwrap();
        let needs_migration = Schema::needs_migration(&conn).unwrap();
        assert!(!needs_migration);
    }

    #[test]
    fn test_create_tables_sql() {
        let sql = Schema::create_all_tables();
        assert!(sql.contains("CREATE TABLE IF NOT EXISTS pages"));
        assert!(sql.contains("CREATE TABLE IF NOT EXISTS blocks"));
        assert!(sql.contains("CREATE TABLE IF NOT EXISTS links"));
        assert!(sql.contains("FOREIGN KEY"));
    }

    #[test]
    fn test_create_indexes_sql() {
        let indexes = Schema::create_all_indexes();
        assert!(!indexes.is_empty());
        assert!(indexes.iter().any(|s| s.contains("idx_pages_path")));
        assert!(indexes.iter().any(|s| s.contains("idx_blocks_page_id")));
    }

    #[test]
    fn test_create_triggers_sql() {
        let triggers = Schema::create_all_triggers();
        assert!(!triggers.is_empty());
        assert!(triggers.iter().any(|s| s.contains("TRIGGER")));
    }

    #[test]
    fn test_get_complete_schema() {
        let schema = Schema::get_complete_schema();
        assert!(schema.contains("PRAGMA"));
        assert!(schema.contains("CREATE TABLE"));
        assert!(schema.contains("CREATE INDEX"));
        assert!(schema.contains("CREATE TRIGGER"));
    }

    #[test]
    fn test_drop_all_tables() {
        let sql = Schema::drop_all_tables();
        assert!(sql.contains("DROP TABLE IF EXISTS"));
    }

    #[test]
    fn test_validate_schema() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test_validate.db");

        let conn = Schema::initialize_database(&db_path).unwrap();
        let errors = Schema::validate_schema(&conn).unwrap();
        assert!(
            errors.is_empty(),
            "Schema validation should pass: {:?}",
            errors
        );
    }

    #[test]
    fn test_database_path_creation() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("nested").join("dir").join("test.db");

        // Create directory structure
        fs::create_dir_all(dir.path().join("nested").join("dir")).unwrap();

        let conn = Schema::initialize_database(&db_path).unwrap();
        assert!(db_path.exists());

        let version = Schema::get_version(&conn).unwrap();
        assert_eq!(version, DB_VERSION);
    }
}
