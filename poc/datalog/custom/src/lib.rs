//! PoC d'un moteur Datalog custom utilisant SQLite pour Note Craft
//!
//! Ce module implémente un simple moteur de requêtage Datalog-like
//! en utilisant SQLite comme backend de stockage.
//!
//! Auteur: Agent autonome
//! Date: 2026-11-11

use rusqlite::{Connection, params, Result};
use std::path::Path;

/// Représente un identifiant de nœud
pub type NodeId = i64;

/// Structure pour gérer la base de données
pub struct DatalogDb {
    conn: Connection,
}

impl DatalogDb {
    /// Crée une nouvelle base de données en mémoire ou dans un fichier
    pub fn new_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        Self::initialize_schema(&conn)?;
        Ok(Self { conn })
    }

    /// Crée une nouvelle base de données dans un fichier
    pub fn new(file_path: &Path) -> Result<Self> {
        let conn = Connection::open(file_path)?;
        Self::initialize_schema(&conn)?;
        Ok(Self { conn })
    }

    /// Initialise le schéma de la base de données
    fn initialize_schema(conn: &Connection) -> Result<()> {
        conn.execute_batch(
            "BEGIN;
            
            -- Nœuds
            CREATE TABLE IF NOT EXISTS nodes (
                id INTEGER PRIMARY KEY
            );
            
            -- Tags
            CREATE TABLE IF NOT EXISTS tags (
                node_id INTEGER NOT NULL,
                tag TEXT NOT NULL,
                FOREIGN KEY (node_id) REFERENCES nodes(id)
            );
            
            -- Liens
            CREATE TABLE IF NOT EXISTS links (
                from_node INTEGER NOT NULL,
                to_node INTEGER NOT NULL,
                FOREIGN KEY (from_node) REFERENCES nodes(id),
                FOREIGN KEY (to_node) REFERENCES nodes(id)
            );
            
            -- Types de nœuds
            CREATE TABLE IF NOT EXISTS node_types (
                node_id INTEGER NOT NULL,
                type_name TEXT NOT NULL,
                FOREIGN KEY (node_id) REFERENCES nodes(id)
            );
            
            -- Index pour optimiser les requêtes
            CREATE INDEX IF NOT EXISTS idx_tags_node ON tags(node_id);
            CREATE INDEX IF NOT EXISTS idx_tags_tag ON tags(tag);
            CREATE INDEX IF NOT EXISTS idx_links_from ON links(from_node);
            CREATE INDEX IF NOT EXISTS idx_links_to ON links(to_node);
            CREATE INDEX IF NOT EXISTS idx_node_types_node ON node_types(node_id);
            CREATE INDEX IF NOT EXISTS idx_node_types_type ON node_types(type_name);
            
            COMMIT;",
        )?;
        Ok(())
    }

    /// Insère un nœud
    pub fn insert_node(&self, id: NodeId) -> Result<()> {
        self.conn.execute(
            "INSERT OR IGNORE INTO nodes (id) VALUES (?1)",
            params![id],
        )?;
        Ok(())
    }

    /// Insère un tag
    pub fn insert_tag(&self, node_id: NodeId, tag: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO tags (node_id, tag) VALUES (?1, ?2)",
            params![node_id, tag],
        )?;
        Ok(())
    }

    /// Insère un lien
    pub fn insert_link(&self, from: NodeId, to: NodeId) -> Result<()> {
        self.conn.execute(
            "INSERT INTO links (from_node, to_node) VALUES (?1, ?2)",
            params![from, to],
        )?;
        Ok(())
    }

    /// Insère un type de nœud
    pub fn insert_node_type(&self, node_id: NodeId, type_name: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO node_types (node_id, type_name) VALUES (?1, ?2)",
            params![node_id, type_name],
        )?;
        Ok(())
    }

    /// Trouve les nœuds avec un tag spécifique (Requête 1: Par tag)
    pub fn find_nodes_by_tag(&self, tag: &str) -> Result<Vec<NodeId>> {
        let mut stmt = self.conn.prepare(
            "SELECT node_id FROM tags WHERE tag = ?1"
        )?;
        let mut rows = stmt.query(params![tag])?;
        let mut result = Vec::new();
        while let Some(row) = rows.next()? {
            result.push(row.get(0)?);
        }
        Ok(result)
    }

    /// Trouve les nœuds liés à des nœuds avec un tag spécifique (Requête 2: Par lien)
    pub fn find_nodes_linked_to_tagged(&self, tag: &str) -> Result<Vec<NodeId>> {
        let mut stmt = self.conn.prepare(
            "SELECT DISTINCT l.from_node, l.to_node 
             FROM links l 
             JOIN tags t ON l.from_node = t.node_id OR l.to_node = t.node_id
             WHERE t.tag = ?1"
        )?;
        let mut rows = stmt.query(params![tag])?;
        let mut result = Vec::new();
        while let Some(row) = rows.next()? {
            result.push(row.get::<_, NodeId>(0)?);
            result.push(row.get::<_, NodeId>(1)?);
        }
        Ok(result)
    }

    /// Trouve tous les nœuds accessibles depuis un nœud source (Requête 3: Transitivité)
    /// Utilise une requête récursive avec CTE (Common Table Expression)
    pub fn find_reachable_nodes(&self, source: NodeId) -> Result<Vec<NodeId>> {
        // SQLite supporte les CTE récursives depuis la version 3.8.3
        let mut stmt = self.conn.prepare(
            "WITH RECURSIVE reachable AS (
                SELECT to_node FROM links WHERE from_node = ?1
                UNION
                SELECT l.to_node FROM links l JOIN reachable r ON l.from_node = r.to_node
             )
             SELECT DISTINCT to_node FROM reachable"
        )?;
        let mut rows = stmt.query(params![source])?;
        let mut result = Vec::new();
        while let Some(row) = rows.next()? {
            result.push(row.get(0)?);
        }
        // Ajouter le nœud source lui-même
        result.push(source);
        Ok(result)
    }

    /// Trouve les nœuds de type spécifique (Requête 4: Par type)
    pub fn find_nodes_by_type(&self, type_name: &str) -> Result<Vec<NodeId>> {
        let mut stmt = self.conn.prepare(
            "SELECT node_id FROM node_types WHERE type_name = ?1"
        )?;
        let mut rows = stmt.query(params![type_name])?;
        let mut result = Vec::new();
        while let Some(row) = rows.next()? {
            result.push(row.get(0)?);
        }
        Ok(result)
    }

    /// Exécute une transaction pour insérer plusieurs données
    pub fn insert_batch(&mut self, nodes: &[NodeId], tags: &[(NodeId, &str)], links: &[(NodeId, NodeId)], node_types: &[(NodeId, &str)]) -> Result<()> {
        let tx = self.conn.transaction()?;
        
        for &node in nodes {
            tx.execute("INSERT OR IGNORE INTO nodes (id) VALUES (?1)", params![node])?;
        }
        
        for &(node, tag) in tags {
            tx.execute("INSERT INTO tags (node_id, tag) VALUES (?1, ?2)", params![node, tag])?;
        }
        
        for &(from, to) in links {
            tx.execute("INSERT INTO links (from_node, to_node) VALUES (?1, ?2)", params![from, to])?;
        }
        
        for &(node, type_name) in node_types {
            tx.execute("INSERT INTO node_types (node_id, type_name) VALUES (?1, ?2)", params![node, type_name])?;
        }
        
        tx.commit()?;
        Ok(())
    }

    /// Vider la base de données
    pub fn clear(&self) -> Result<()> {
        self.conn.execute_batch(
            "BEGIN;
            DELETE FROM node_types;
            DELETE FROM links;
            DELETE FROM tags;
            DELETE FROM nodes;
            COMMIT;"
        )?;
        Ok(())
    }
}

impl Default for DatalogDb {
    fn default() -> Self {
        Self::new_in_memory().expect("Failed to create in-memory database")
    }
}

/// Génère des données de test de manière déterministe
pub fn generate_test_data(node_count: usize, link_count: usize) -> (Vec<NodeId>, Vec<(NodeId, String)>, Vec<(NodeId, NodeId)>, Vec<(NodeId, String)>) {
    let nodes: Vec<NodeId> = (1..=node_count as NodeId).collect();
    
    // Générer des tags
    let tag_names = ["important", "rust", "datalog", "note-craft", "test"];
    let mut tags = Vec::new();
    for (idx, &node) in nodes.iter().enumerate() {
        let num_tags = (idx % 3) + 1;
        for i in 0..num_tags {
            let tag_idx = (idx + i) % tag_names.len();
            tags.push((node, tag_names[tag_idx].to_string()));
        }
    }
    
    // Générer des liens de manière déterministe
    let mut links = Vec::new();
    for i in 0..link_count {
        let from_idx = i % nodes.len();
        let to_idx = ((i + 13) * 7) % nodes.len();
        links.push((nodes[from_idx], nodes[to_idx]));
    }
    
    // Générer des types
    let types = ["page", "block", "asset", "query"];
    let node_types: Vec<(NodeId, String)> = nodes.iter()
        .enumerate()
        .map(|(idx, &n)| {
            let type_idx = idx % types.len();
            (n, types[type_idx].to_string())
        })
        .collect();

    (nodes, tags, links, node_types)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let db = DatalogDb::new_in_memory().unwrap();
        
        // Insérer des données
        db.insert_node(1).unwrap();
        db.insert_node(2).unwrap();
        db.insert_tag(1, "important").unwrap();
        db.insert_link(1, 2).unwrap();
        db.insert_node_type(1, "page").unwrap();
        
        // Tester requête par tag
        let nodes = db.find_nodes_by_tag("important").unwrap();
        assert!(nodes.contains(&1));
        
        // Tester requête par type
        let pages = db.find_nodes_by_type("page").unwrap();
        assert!(pages.contains(&1));
        
        // Tester requête de transitivité
        let reachable = db.find_reachable_nodes(1).unwrap();
        assert!(reachable.contains(&2));
    }
}
