//! Note Craft Storage - Couche de stockage SQLite
//!
//! Ce module fournit:
//! - Stockage des nœuds, liens, tags, types
//! - Requêtes SQL pour récupérer les données
//! - Intégration avec rusqlite

pub mod importer;
pub mod properties;
pub mod schema;

use note_craft_core::{Link, LinkId, Node, NodeId, TagId, TypeId};
use rusqlite::{params, Connection, Result};
use std::path::Path;

/// Structure principale de la base de données
pub struct Storage {
    conn: Connection,
}

impl Storage {
    /// Ouvre ou crée une base de données
    pub fn open(path: &Path) -> Result<Self> {
        let conn = Connection::open(path)?;

        // Créer les tables
        conn.execute_batch(
            "BEGIN;
             CREATE TABLE IF NOT EXISTS nodes (
                 id INTEGER PRIMARY KEY,
                 content TEXT NOT NULL,
                 node_type INTEGER NOT NULL,
                 created_at INTEGER NOT NULL,
                 updated_at INTEGER NOT NULL
             );
             
             CREATE TABLE IF NOT EXISTS tags (
                 id INTEGER PRIMARY KEY,
                 name TEXT NOT NULL UNIQUE
             );
             
             CREATE TABLE IF NOT EXISTS node_tags (
                 node_id INTEGER NOT NULL,
                 tag_id INTEGER NOT NULL,
                 PRIMARY KEY (node_id, tag_id),
                 FOREIGN KEY (node_id) REFERENCES nodes(id),
                 FOREIGN KEY (tag_id) REFERENCES tags(id)
             );
             
             CREATE TABLE IF NOT EXISTS types (
                 id INTEGER PRIMARY KEY,
                 name TEXT NOT NULL UNIQUE
             );
             
             CREATE TABLE IF NOT EXISTS links (
                 id INTEGER PRIMARY KEY,
                 from_node INTEGER NOT NULL,
                 to_node INTEGER NOT NULL,
                 link_type TEXT,
                 created_at INTEGER NOT NULL,
                 FOREIGN KEY (from_node) REFERENCES nodes(id),
                 FOREIGN KEY (to_node) REFERENCES nodes(id)
             );
             COMMIT;",
        )?;

        Ok(Storage { conn })
    }

    /// Sauvegarde un nœud
    pub fn save_node(&self, node: &Node) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO nodes (id, content, node_type, created_at, updated_at) 
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                node.id.0 as i64,
                node.content,
                node.node_type.0 as i64,
                node.created_at as i64,
                node.updated_at as i64
            ],
        )?;

        // Sauvegarder les tags
        for tag in &node.tags {
            self.conn.execute(
                "INSERT OR IGNORE INTO node_tags (node_id, tag_id) VALUES (?1, ?2)",
                params![node.id.0 as i64, tag.0 as i64],
            )?;
        }

        Ok(())
    }

    /// Charge un nœud par ID
    pub fn load_node(&self, id: NodeId) -> Result<Option<Node>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, content, node_type, created_at, updated_at 
             FROM nodes WHERE id = ?1",
        )?;

        let mut rows = stmt.query_map(params![id.0 as i64], |row| {
            Ok(Node {
                id: NodeId(row.get(0)?),
                content: row.get(1)?,
                node_type: TypeId(row.get(2)?),
                tags: vec![], // À charger séparément
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        })?;

        if let Some(row) = rows.next() {
            let mut node = row?;
            // Charger les tags
            let mut tag_stmt = self
                .conn
                .prepare("SELECT tag_id FROM node_tags WHERE node_id = ?1")?;
            let tag_rows =
                tag_stmt.query_map(params![id.0 as i64], |row| Ok(TagId(row.get(0)?)))?;

            for tag_row in tag_rows {
                node.tags.push(tag_row?);
            }

            Ok(Some(node))
        } else {
            Ok(None)
        }
    }

    /// Sauvegarde un lien
    pub fn save_link(&self, link: &Link) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO links (id, from_node, to_node, link_type, created_at) 
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                link.id.0 as i64,
                link.from.0 as i64,
                link.to.0 as i64,
                link.link_type.as_deref().unwrap_or(""),
                link.created_at as i64
            ],
        )?;

        Ok(())
    }

    /// Charge tous les nœuds
    pub fn load_all_nodes(&self) -> Result<Vec<Node>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, content, node_type, created_at, updated_at FROM nodes")?;

        let nodes: Vec<Node> = stmt
            .query_map(params![], |row| {
                Ok(Node {
                    id: NodeId(row.get(0)?),
                    content: row.get(1)?,
                    node_type: TypeId(row.get(2)?),
                    tags: vec![],
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                })
            })?
            .filter_map(|r| r.ok())
            .collect();

        // Charger les tags pour chaque nœud
        let mut result = Vec::new();
        for mut node in nodes {
            let mut tag_stmt = self
                .conn
                .prepare("SELECT tag_id FROM node_tags WHERE node_id = ?1")?;
            let tags: Vec<TagId> = tag_stmt
                .query_map(params![node.id.0 as i64], |row| Ok(TagId(row.get(0)?)))?
                .filter_map(|r| r.ok())
                .collect();
            node.tags = tags;
            result.push(node);
        }

        Ok(result)
    }

    /// Charge tous les liens
    pub fn load_all_links(&self) -> Result<Vec<Link>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, from_node, to_node, link_type, created_at FROM links")?;

        let links: Vec<Link> = stmt
            .query_map(params![], |row| {
                Ok(Link {
                    id: LinkId(row.get(0)?),
                    from: NodeId(row.get(1)?),
                    to: NodeId(row.get(2)?),
                    link_type: row.get(3).ok().map(|s: String| s),
                    created_at: row.get(4)?,
                })
            })?
            .filter_map(|r| r.ok())
            .collect();

        Ok(links)
    }
}

/// Tests de base
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_open() {
        let temp_dir = tempfile::tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let storage = Storage::open(&db_path).unwrap();
        assert!(storage.conn.is_autocommit());
    }

    #[test]
    fn test_save_and_load_node() {
        let temp_dir = tempfile::tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let storage = Storage::open(&db_path).unwrap();

        // Pour l'instant, testons sans tags pour éviter les contraintes FK
        let node = Node {
            id: NodeId(1),
            content: "Test content".to_string(),
            node_type: TypeId(1),
            tags: vec![], // Pas de tags pour éviter les contraintes FK
            created_at: 1234567890,
            updated_at: 1234567890,
        };

        storage.save_node(&node).unwrap();
        let loaded = storage.load_node(NodeId(1)).unwrap();

        assert!(loaded.is_some());
        let loaded_node = loaded.unwrap();
        assert_eq!(loaded_node.id, node.id);
        assert_eq!(loaded_node.content, node.content);
    }
}
