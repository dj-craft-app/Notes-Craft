//! Note Craft Storage - Data Import System
//!
//! Ce module fournit les fonctionnalités d'import de données depuis différents formats.
//! Il supporte l'import depuis Markdown, JSON, et OPML.
//!
//! Phase 4 - Étape 4.3 (Data Import) — docs/roadmap/roadmap_v3.md:814

use note_craft_core::block::{Block, BlockContent, BlockId, BlockType};
use note_craft_core::page::PageId;
use note_craft_core::property::{Property, PropertyType, PropertyValue};
use note_craft_core::{Link, LinkId, NodeId, TagId};
use rusqlite::{params, Connection};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

// ============================================================================
// ERREURS
// ============================================================================

/// Erreurs d'import
#[derive(Error, Debug)]
pub enum ImportError {
    #[error("Erreur IO: {0}")]
    IoError(String),
    #[error("Erreur de parsing: {0}")]
    ParseError(String),
    #[error("Erreur de stockage: {0}")]
    StorageError(String),
    #[error("Fichier non trouvé: {0}")]
    FileNotFound(String),
    #[error("Format non supporté: {0}")]
    UnsupportedFormat(String),
    #[error("Erreur de validation: {0}")]
    ValidationError(String),
}

impl From<std::io::Error> for ImportError {
    fn from(err: std::io::Error) -> Self {
        ImportError::IoError(err.to_string())
    }
}

impl From<rusqlite::Error> for ImportError {
    fn from(err: rusqlite::Error) -> Self {
        ImportError::StorageError(err.to_string())
    }
}

// ============================================================================
// STRUCTURES D'IMPORT
// ============================================================================

/// Format d'import supporté
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImportFormat {
    /// Fichier Markdown unique
    Markdown,
    /// Dossier de fichiers Markdown
    MarkdownDirectory,
    /// Fichier JSON (format Logseq / export générique)
    Json,
    /// Fichier OPML
    Opml,
}

impl ImportFormat {
    /// Détermine le format à partir de l'extension du fichier
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "md" | "markdown" => Some(ImportFormat::Markdown),
            "json" => Some(ImportFormat::Json),
            "opml" => Some(ImportFormat::Opml),
            _ => None,
        }
    }
}

/// Résultat d'un import
#[derive(Debug, Clone)]
pub struct ImportResult {
    /// Nombre de pages importées
    pub pages_imported: usize,
    /// Nombre de blocs importés
    pub blocks_imported: usize,
    /// Nombre de liens importés
    pub links_imported: usize,
    /// Nombre de propriétés importées
    pub properties_imported: usize,
    /// Nombre de tags importés
    pub tags_imported: usize,
    /// Avertissements générés pendant l'import
    pub warnings: Vec<String>,
    /// Erreurs non bloquantes
    pub errors: Vec<String>,
}

impl Default for ImportResult {
    fn default() -> Self {
        Self::new()
    }
}

impl ImportResult {
    /// Crée un nouveau résultat d'import
    pub fn new() -> Self {
        Self {
            pages_imported: 0,
            blocks_imported: 0,
            links_imported: 0,
            properties_imported: 0,
            tags_imported: 0,
            warnings: Vec::new(),
            errors: Vec::new(),
        }
    }

    /// Ajoute un avertissement
    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }

    /// Ajoute une erreur non bloquante
    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }
}

/// Lien extrait d'un contenu, avec ses métadonnées résolubles
#[derive(Debug, Clone)]
pub struct ImportLink {
    /// Le lien lui-même
    pub link: Link,
    /// Texte affiché du lien
    pub text: String,
    /// URL (pour les liens Markdown)
    pub url: Option<String>,
    /// Nom de la page cible (pour les wikilinks)
    pub target_name: Option<String>,
}

// ============================================================================
// IMPORTEUR PRINCIPAL
// ============================================================================

/// Importeur de données
pub struct DataImporter {
    conn: Connection,
    /// Map des noms de page à leurs IDs
    page_name_to_id: HashMap<String, PageId>,
    /// Map des noms de propriété à leurs IDs
    property_name_to_id: HashMap<String, u64>,
    /// Map des noms de tag à leurs IDs
    tag_name_to_id: HashMap<String, TagId>,
    /// Compteur pour les IDs
    next_page_id: u64,
    next_block_id: u64,
    next_link_id: u64,
}

impl DataImporter {
    /// Crée un nouvel importeur
    pub fn new(conn: Connection) -> Self {
        Self {
            conn,
            page_name_to_id: HashMap::new(),
            property_name_to_id: HashMap::new(),
            tag_name_to_id: HashMap::new(),
            next_page_id: 1,
            next_block_id: 1,
            next_link_id: 1,
        }
    }

    /// Importe depuis un fichier ou dossier
    pub fn import(
        &mut self,
        path: &Path,
        format: ImportFormat,
    ) -> Result<ImportResult, ImportError> {
        match format {
            ImportFormat::Markdown => self.import_markdown_file(path),
            ImportFormat::MarkdownDirectory => self.import_markdown_directory(path),
            ImportFormat::Json => self.import_json(path),
            ImportFormat::Opml => self.import_opml(path),
        }
    }

    /// Importe depuis un fichier (détecte le format automatiquement)
    pub fn import_auto(&mut self, path: &Path) -> Result<ImportResult, ImportError> {
        if path.is_dir() {
            self.import(path, ImportFormat::MarkdownDirectory)
        } else {
            let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
            let format = ImportFormat::from_extension(ext)
                .ok_or_else(|| ImportError::UnsupportedFormat(ext.to_string()))?;
            self.import(path, format)
        }
    }

    // ============================================================================
    // IMPORT JSON (format Logseq / export générique)
    // ============================================================================

    /// Importe depuis un fichier JSON.
    ///
    /// Formats supportés :
    /// - Tableau de pages : `[{"title": "...", "properties": {...}, "blocks": [...]}]`
    /// - Objet avec clé `pages` : `{"pages": [...]}`
    /// - Chaque bloc : `{"content": "...", "type": "paragraph|heading_1|code|...", "children": [...], "properties": {...}}`
    pub fn import_json(&mut self, path: &Path) -> Result<ImportResult, ImportError> {
        let content = fs::read_to_string(path)?;
        let value: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| ImportError::ParseError(format!("JSON invalide: {}", e)))?;

        let pages: Vec<serde_json::Value> = if let Some(arr) = value.as_array() {
            arr.clone()
        } else if let Some(arr) = value.get("pages").and_then(|v| v.as_array()) {
            arr.clone()
        } else {
            return Err(ImportError::ParseError(
                "Le JSON doit être un tableau de pages ou un objet avec une clé \"pages\""
                    .to_string(),
            ));
        };

        let mut result = ImportResult::new();
        for page_val in &pages {
            let title = page_val
                .get("title")
                .or_else(|| page_val.get("name"))
                .and_then(|v| v.as_str())
                .unwrap_or("untitled")
                .to_string();

            // Path synthétique unique par page (la colonne path est UNIQUE)
            let synthetic_path = PathBuf::from(format!("{}/{}", path.display(), title));
            let page_id = self.create_page(&title, &synthetic_path)?;
            result.pages_imported += 1;

            // Propriétés de la page (entity_type 0 = page)
            if let Some(props) = page_val.get("properties").and_then(|v| v.as_object()) {
                for (key, val) in props {
                    let pv = Self::json_value_to_property_value(val);
                    match self.create_property_value(page_id.0, 0, key, pv) {
                        Ok(()) => result.properties_imported += 1,
                        Err(e) => result.add_warning(format!("Propriété '{}' ignorée: {}", key, e)),
                    }
                }
            }

            // Blocs de la page
            if let Some(blocks) = page_val.get("blocks").and_then(|v| v.as_array()) {
                for block_val in blocks {
                    self.import_json_block(block_val, page_id, 0, &mut result)?;
                }
            }
        }

        Ok(result)
    }

    /// Importe un bloc JSON (récursif pour les enfants)
    fn import_json_block(
        &mut self,
        block_val: &serde_json::Value,
        page_id: PageId,
        indent: u32,
        result: &mut ImportResult,
    ) -> Result<(), ImportError> {
        let content = block_val
            .get("content")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let type_str = block_val
            .get("type")
            .and_then(|v| v.as_str())
            .unwrap_or("paragraph");

        let block_type = Self::json_type_to_block_type(type_str);

        let block = Block {
            id: BlockId(self.next_block_id),
            page_id: Some(page_id.0),
            content: BlockContent::Text(content.clone()),
            block_type,
            position: result.blocks_imported,
            parent_id: None,
            children: Vec::new(),
            created_at: chrono::Utc::now().timestamp() as u64,
            updated_at: chrono::Utc::now().timestamp() as u64,
            tags: Vec::new(),
        };
        let block_id = block.id;
        self.next_block_id += 1;
        self.create_block_with_indent(block, indent)?;
        result.blocks_imported += 1;

        // Extraire les liens du contenu du bloc, rattachés au bloc source
        let links = self.extract_links_from_markdown(&content, page_id);
        for import_link in links {
            self.create_link(import_link, block_id)?;
            result.links_imported += 1;
        }

        // Extraire les tags du contenu du bloc
        let tags = self.extract_tags_from_markdown(&content);
        for tag in tags {
            match self.register_tag(&tag) {
                Ok(_) => result.tags_imported += 1,
                Err(e) => result.add_warning(format!("Tag '{}' ignoré: {}", tag, e)),
            }
        }

        // Propriétés du bloc (entity_type 1 = bloc, entity_id = block_id)
        if let Some(props) = block_val.get("properties").and_then(|v| v.as_object()) {
            for (key, val) in props {
                let pv = Self::json_value_to_property_value(val);
                match self.create_property_value(block_id.0, 1, key, pv) {
                    Ok(()) => result.properties_imported += 1,
                    Err(e) => result.add_warning(format!("Propriété '{}' ignorée: {}", key, e)),
                }
            }
        }

        // Enfants (récursif)
        if let Some(children) = block_val.get("children").and_then(|v| v.as_array()) {
            for child in children {
                self.import_json_block(child, page_id, indent + 1, result)?;
            }
        }

        Ok(())
    }

    /// Convertit une valeur JSON en PropertyValue
    fn json_value_to_property_value(value: &serde_json::Value) -> PropertyValue {
        match value {
            serde_json::Value::String(s) => {
                if let Ok(num) = s.parse::<f64>() {
                    PropertyValue::Number(num)
                } else if let Ok(b) = s.parse::<bool>() {
                    PropertyValue::Boolean(b)
                } else {
                    PropertyValue::Text(s.clone())
                }
            }
            serde_json::Value::Number(n) => PropertyValue::Number(n.as_f64().unwrap_or(0.0)),
            serde_json::Value::Bool(b) => PropertyValue::Boolean(*b),
            serde_json::Value::Array(arr) => PropertyValue::MultiSelect(
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect(),
            ),
            serde_json::Value::Null => PropertyValue::None,
            _ => PropertyValue::Text(value.to_string()),
        }
    }

    /// Convertit une chaîne de type JSON en BlockType
    fn json_type_to_block_type(type_str: &str) -> BlockType {
        match type_str {
            "heading_1" | "heading1" | "h1" => BlockType::Heading(1),
            "heading_2" | "heading2" | "h2" => BlockType::Heading(2),
            "heading_3" | "heading3" | "h3" => BlockType::Heading(3),
            "heading_4" | "h4" => BlockType::Heading(4),
            "heading_5" | "h5" => BlockType::Heading(5),
            "heading_6" | "h6" => BlockType::Heading(6),
            "bulleted_list" | "bullet_list" | "bulleted-list" => BlockType::BulletList,
            "numbered_list" | "numbered-list" | "ordered_list" => BlockType::NumberedList,
            "code" | "code_block" => BlockType::Code,
            "quote" | "block_quote" => BlockType::Quote,
            "divider" | "hr" | "horizontal_rule" => BlockType::HorizontalRule,
            "to_do" | "todo" | "checkbox" => BlockType::BulletList,
            "toggle" => BlockType::Text,
            "page" | "page_reference" => BlockType::PageReference,
            "block_reference" | "block_ref" => BlockType::BlockReference(BlockId(0)),
            "tag" => BlockType::Tag,
            _ => BlockType::Text,
        }
    }

    // ============================================================================
    // IMPORT OPML
    // ============================================================================

    /// Importe depuis un fichier OPML.
    ///
    /// Les `<outline>` de premier niveau deviennent des pages ;
    /// leurs sous-outlines deviennent des blocs (listes imbriquées via indent_level).
    pub fn import_opml(&mut self, path: &Path) -> Result<ImportResult, ImportError> {
        use quick_xml::events::Event;

        let content = fs::read_to_string(path)?;
        let mut result = ImportResult::new();

        let filename = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("opml_import");
        let root_title = Path::new(filename)
            .with_extension("")
            .to_string_lossy()
            .to_string();

        // Profondeur courante dans l'arbre des outlines
        let mut depth: u32 = 0;
        // Page courante (dernier outline de niveau 0 rencontré)
        let mut current_page: Option<PageId> = None;

        let mut reader = quick_xml::Reader::from_str(&content);
        reader.trim_text(true);

        let mut buf = Vec::new();
        loop {
            let event = reader.read_event_into(&mut buf);
            match event {
                Ok(Event::Start(e)) => {
                    if e.name().as_ref() == b"outline" {
                        let text = Self::outline_text(&e);
                        if depth == 0 {
                            // Premier niveau : nouvelle page (path synthétique unique)
                            let synthetic_path =
                                PathBuf::from(format!("{}/{}", path.display(), text));
                            let page_id = self.create_page(&text, &synthetic_path)?;
                            result.pages_imported += 1;
                            current_page = Some(page_id);
                        } else if let Some(page_id) = current_page {
                            // Sous-outline : bloc de la page courante
                            let block = Block {
                                id: BlockId(self.next_block_id),
                                page_id: Some(page_id.0),
                                content: BlockContent::Text(text),
                                block_type: BlockType::BulletList,
                                position: result.blocks_imported,
                                parent_id: None,
                                children: Vec::new(),
                                created_at: chrono::Utc::now().timestamp() as u64,
                                updated_at: chrono::Utc::now().timestamp() as u64,
                                tags: Vec::new(),
                            };
                            self.next_block_id += 1;
                            self.create_block_with_indent(block, depth)?;
                            result.blocks_imported += 1;
                        }
                        depth += 1;
                    }
                }
                Ok(Event::Empty(e)) => {
                    // Élément self-closing : traité comme un nœud sans enfants
                    if e.name().as_ref() == b"outline" {
                        let text = Self::outline_text(&e);
                        if depth == 0 {
                            let synthetic_path =
                                PathBuf::from(format!("{}/{}", path.display(), text));
                            let page_id = self.create_page(&text, &synthetic_path)?;
                            result.pages_imported += 1;
                            current_page = Some(page_id);
                        } else if let Some(page_id) = current_page {
                            let block = Block {
                                id: BlockId(self.next_block_id),
                                page_id: Some(page_id.0),
                                content: BlockContent::Text(text),
                                block_type: BlockType::BulletList,
                                position: result.blocks_imported,
                                parent_id: None,
                                children: Vec::new(),
                                created_at: chrono::Utc::now().timestamp() as u64,
                                updated_at: chrono::Utc::now().timestamp() as u64,
                                tags: Vec::new(),
                            };
                            self.next_block_id += 1;
                            self.create_block_with_indent(block, depth)?;
                            result.blocks_imported += 1;
                        }
                        // Pas d'augmentation de profondeur pour un élément vide
                    }
                }
                Ok(Event::End(e)) => {
                    if e.name().as_ref() == b"outline" && depth > 0 {
                        depth -= 1;
                        if depth == 0 {
                            current_page = None;
                        }
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => {
                    return Err(ImportError::ParseError(format!("XML invalide: {}", e)));
                }
                _ => {}
            }
            buf.clear();
        }

        // Si aucun outline de niveau 1 : créer une page racine avec le nom du fichier
        if result.pages_imported == 0 {
            let page_id = self.create_page(&root_title, path)?;
            result.pages_imported += 1;
            result.add_warning(format!(
                "Aucun <outline> trouvé, page racine '{}' créée (id={})",
                root_title, page_id.0
            ));
        }

        Ok(result)
    }

    /// Extrait le texte d'un élément `<outline>` (attribut text ou title)
    fn outline_text(e: &quick_xml::events::BytesStart) -> String {
        e.attributes()
            .filter_map(|a| a.ok())
            .find(|a| a.key.as_ref() == b"text" || a.key.as_ref() == b"title")
            .and_then(|a| a.unescape_value().ok())
            .map(|s| s.to_string())
            .unwrap_or_default()
    }

    // ============================================================================
    // IMPORT MARKDOWN
    // ============================================================================

    /// Importe un fichier Markdown unique
    pub fn import_markdown_file(&mut self, path: &Path) -> Result<ImportResult, ImportError> {
        let content = fs::read_to_string(path)?;
        let filename = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("untitled");
        let page_name = Path::new(filename)
            .with_extension("")
            .to_string_lossy()
            .to_string();

        // Créer la page
        let page_id = self.create_page(&page_name, path)?;

        // Parser le contenu Markdown et créer des blocs
        let blocks = self.parse_markdown(&content, page_id)?;

        // Créer les blocs dans la base de données
        let mut result = ImportResult::new();
        result.pages_imported = 1;

        // Mémoriser (id, contenu texte) de chaque bloc pour rattacher les
        // liens au vrai bloc source (links.source_block_id est NOT NULL avec
        // une FK vers blocks(id), activée par PRAGMA foreign_keys=ON)
        let block_infos: Vec<(BlockId, String)> = blocks
            .iter()
            .filter_map(|b| match &b.content {
                BlockContent::Text(s) => Some((b.id, s.clone())),
                _ => None,
            })
            .collect();

        for block in blocks {
            self.create_block(block)?;
            result.blocks_imported += 1;
        }

        // Extraire les liens bloc par bloc et les créer avec le vrai source_block_id
        for (block_id, block_content) in block_infos {
            let links = self.extract_links_from_markdown(&block_content, page_id);
            for import_link in links {
                self.create_link(import_link, block_id)?;
                result.links_imported += 1;
            }
        }

        // Extraire et enregistrer les tags (#tag)
        let tags = self.extract_tags_from_markdown(&content);
        for tag in tags {
            match self.register_tag(&tag) {
                Ok(_) => result.tags_imported += 1,
                Err(e) => result.add_warning(format!("Tag '{}' ignoré: {}", tag, e)),
            }
        }

        // Extraire les propriétés frontmatter
        let properties = self.extract_frontmatter_properties(&content);
        for (name, value) in properties {
            match self.create_property_value(page_id.0, 0, &name, value) {
                Ok(()) => result.properties_imported += 1,
                Err(e) => result.add_warning(format!("Propriété '{}' ignorée: {}", name, e)),
            }
        }

        Ok(result)
    }

    /// Importe un dossier de fichiers Markdown (récursif)
    pub fn import_markdown_directory(&mut self, path: &Path) -> Result<ImportResult, ImportError> {
        let mut result = ImportResult::new();

        if !path.exists() {
            return Err(ImportError::FileNotFound(
                path.to_string_lossy().to_string(),
            ));
        }

        let entries = fs::read_dir(path)?;

        for entry in entries {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_file() {
                let ext = entry_path
                    .extension()
                    .and_then(|s| s.to_str())
                    .unwrap_or("");
                if ext.eq_ignore_ascii_case("md") || ext.eq_ignore_ascii_case("markdown") {
                    match self.import_markdown_file(&entry_path) {
                        Ok(sub_result) => {
                            result.pages_imported += sub_result.pages_imported;
                            result.blocks_imported += sub_result.blocks_imported;
                            result.links_imported += sub_result.links_imported;
                            result.properties_imported += sub_result.properties_imported;
                            result.tags_imported += sub_result.tags_imported;
                            result.warnings.extend(sub_result.warnings);
                            result.errors.extend(sub_result.errors);
                        }
                        Err(e) => {
                            result.add_error(format!("{}: {}", entry_path.display(), e));
                        }
                    }
                }
            } else if entry_path.is_dir() {
                // Importer récursivement
                let sub_result = self.import_markdown_directory(&entry_path)?;
                result.pages_imported += sub_result.pages_imported;
                result.blocks_imported += sub_result.blocks_imported;
                result.links_imported += sub_result.links_imported;
                result.properties_imported += sub_result.properties_imported;
                result.tags_imported += sub_result.tags_imported;
                result.warnings.extend(sub_result.warnings);
                result.errors.extend(sub_result.errors);
            }
        }

        Ok(result)
    }

    /// Parse le contenu Markdown et extrait les blocs
    fn parse_markdown(
        &mut self,
        content: &str,
        page_id: PageId,
    ) -> Result<Vec<Block>, ImportError> {
        let mut blocks = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        let mut current_block_type = BlockType::Text;
        let mut current_content = String::new();
        let mut in_code_block = false;
        let mut code_language = String::new();

        for line in lines.iter() {
            let trimmed = line.trim();

            // Sauter les lignes vides
            if trimmed.is_empty() && !in_code_block {
                // Sauvegarder le bloc courant s'il y a du contenu
                if !current_content.is_empty() {
                    blocks.push(self.create_text_block(
                        page_id,
                        &current_block_type,
                        &current_content,
                    ));
                    current_content.clear();
                }
                continue;
            }

            // Détecter les blocs de code
            if trimmed.starts_with("```") {
                if in_code_block {
                    // Fin du bloc de code
                    blocks.push(self.create_code_block(page_id, &code_language, &current_content));
                    current_content.clear();
                    in_code_block = false;
                    code_language.clear();
                    current_block_type = BlockType::Text;
                } else {
                    // Début du bloc de code
                    if !current_content.is_empty() {
                        blocks.push(self.create_text_block(
                            page_id,
                            &current_block_type,
                            &current_content,
                        ));
                        current_content.clear();
                    }
                    in_code_block = true;
                    code_language = trimmed.trim_start_matches("```").trim().to_string();
                    current_block_type = BlockType::Code;
                }
                continue;
            }

            if in_code_block {
                current_content.push_str(line);
                current_content.push('\n');
                continue;
            }

            // Détecter les titres
            if trimmed.starts_with('#') {
                if !current_content.is_empty() {
                    blocks.push(self.create_text_block(
                        page_id,
                        &current_block_type,
                        &current_content,
                    ));
                    current_content.clear();
                }

                let level = trimmed.chars().take_while(|c| *c == '#').count();
                let heading_content = trimmed[level..].trim();

                // Créer un bloc de titre
                let block = Block {
                    id: BlockId(self.next_block_id),
                    page_id: Some(page_id.0),
                    content: BlockContent::Text(heading_content.to_string()),
                    block_type: match level {
                        1 => BlockType::Heading(1),
                        2 => BlockType::Heading(2),
                        3 => BlockType::Heading(3),
                        4 => BlockType::Heading(4),
                        5 => BlockType::Heading(5),
                        6 => BlockType::Heading(6),
                        _ => BlockType::Heading(1),
                    },
                    position: blocks.len(),
                    parent_id: None,
                    children: Vec::new(),
                    created_at: chrono::Utc::now().timestamp() as u64,
                    updated_at: chrono::Utc::now().timestamp() as u64,
                    tags: Vec::new(),
                };

                self.next_block_id += 1;
                blocks.push(block);
                continue;
            }

            // Détecter les listes
            if trimmed.starts_with("* ") || trimmed.starts_with("- ") || trimmed.starts_with("+ ") {
                if current_block_type != BlockType::BulletList {
                    if !current_content.is_empty() {
                        blocks.push(self.create_text_block(
                            page_id,
                            &current_block_type,
                            &current_content,
                        ));
                        current_content.clear();
                    }
                    current_block_type = BlockType::BulletList;
                }
                current_content.push_str(trimmed[2..].trim_start());
                current_content.push('\n');
                continue;
            }

            // Détecter les listes numérotées
            if let Some(pos) = trimmed.find('.') {
                if pos > 0 && pos < 3 {
                    let prefix = &trimmed[..pos];
                    if prefix.chars().all(|c| c.is_ascii_digit()) {
                        if current_block_type != BlockType::NumberedList {
                            if !current_content.is_empty() {
                                blocks.push(self.create_text_block(
                                    page_id,
                                    &current_block_type,
                                    &current_content,
                                ));
                                current_content.clear();
                            }
                            current_block_type = BlockType::NumberedList;
                        }
                        current_content.push_str(trimmed[pos + 1..].trim_start());
                        current_content.push('\n');
                        continue;
                    }
                }
            }

            // Détecter les citations
            if let Some(quote) = trimmed.strip_prefix("> ") {
                if current_block_type != BlockType::Quote {
                    if !current_content.is_empty() {
                        blocks.push(self.create_text_block(
                            page_id,
                            &current_block_type,
                            &current_content,
                        ));
                        current_content.clear();
                    }
                    current_block_type = BlockType::Quote;
                }
                current_content.push_str(quote.trim_start());
                current_content.push('\n');
                continue;
            }

            // Paragraphe normal
            if current_block_type != BlockType::Text {
                if !current_content.is_empty() {
                    blocks.push(self.create_text_block(
                        page_id,
                        &current_block_type,
                        &current_content,
                    ));
                    current_content.clear();
                }
                current_block_type = BlockType::Text;
            }
            current_content.push_str(line);
            current_content.push('\n');
        }

        // Sauvegarder le dernier bloc
        if !current_content.is_empty() {
            blocks.push(self.create_text_block(page_id, &current_block_type, &current_content));
        }

        Ok(blocks)
    }

    /// Crée un bloc de texte
    fn create_text_block(
        &mut self,
        page_id: PageId,
        block_type: &BlockType,
        content: &str,
    ) -> Block {
        let block = Block {
            id: BlockId(self.next_block_id),
            page_id: Some(page_id.0),
            content: BlockContent::Text(content.to_string()),
            block_type: block_type.clone(),
            position: 0,
            parent_id: None,
            children: Vec::new(),
            created_at: chrono::Utc::now().timestamp() as u64,
            updated_at: chrono::Utc::now().timestamp() as u64,
            tags: Vec::new(),
        };
        self.next_block_id += 1;
        block
    }

    /// Crée un bloc de code
    fn create_code_block(&mut self, page_id: PageId, language: &str, content: &str) -> Block {
        let block = Block {
            id: BlockId(self.next_block_id),
            page_id: Some(page_id.0),
            content: BlockContent::Text(format!("```{}\n{}\n```", language, content.trim())),
            block_type: BlockType::Code,
            position: 0,
            parent_id: None,
            children: Vec::new(),
            created_at: chrono::Utc::now().timestamp() as u64,
            updated_at: chrono::Utc::now().timestamp() as u64,
            tags: Vec::new(),
        };
        self.next_block_id += 1;
        block
    }

    // ============================================================================
    // EXTRACTION DES LIENS
    // ============================================================================

    /// Extrait les liens depuis le contenu Markdown
    ///
    /// Supporte :
    /// - Liens Markdown : `[text](url)`
    /// - Wikilinks : `[[page]]` ou `[[page|text]]`
    fn extract_links_from_markdown(&mut self, content: &str, page_id: PageId) -> Vec<ImportLink> {
        let mut links = Vec::new();

        // Regexes compilées une seule fois, hors de la boucle (clippy: regex_in_loop)
        let link_re = regex::Regex::new(r"\[([^\]]+)\]\(([^)]+)\)").unwrap();
        let wikilink_re = regex::Regex::new(r"\[\[([^\]|]+)(?:\|([^\]]+))?\]\]").unwrap();

        for line in content.lines() {
            // Extraire les liens Markdown : [text](url)
            for cap in link_re.captures_iter(line) {
                let text = cap.get(1).unwrap().as_str().to_string();
                let url = cap.get(2).unwrap().as_str().to_string();

                let link = Link {
                    id: LinkId(self.next_link_id),
                    from: NodeId(page_id.0),
                    to: NodeId(0), // les liens externes n'ont pas de cible interne
                    link_type: Some("markdown".to_string()),
                    created_at: chrono::Utc::now().timestamp() as u64,
                };
                self.next_link_id += 1;
                links.push(ImportLink {
                    link,
                    text,
                    url: Some(url),
                    target_name: None,
                });
            }

            // Extraire les wikilinks : [[page]] ou [[page|text]]
            for cap in wikilink_re.captures_iter(line) {
                let target = cap.get(1).unwrap().as_str().to_string();
                let text = cap
                    .get(2)
                    .map_or(target.clone(), |m| m.as_str().to_string());

                let link = Link {
                    id: LinkId(self.next_link_id),
                    from: NodeId(page_id.0),
                    to: NodeId(0), // sera résolu dans create_link
                    link_type: Some("wikilink".to_string()),
                    created_at: chrono::Utc::now().timestamp() as u64,
                };
                self.next_link_id += 1;
                links.push(ImportLink {
                    link,
                    text,
                    url: None,
                    target_name: Some(target),
                });
            }
        }

        links
    }

    // ============================================================================
    // EXTRACTION DES TAGS
    // ============================================================================

    /// Extrait les tags (#tag) depuis le contenu Markdown
    fn extract_tags_from_markdown(&self, content: &str) -> Vec<String> {
        let tag_re = regex::Regex::new(r"(?:^|\s)#([A-Za-z0-9_/-]+)").unwrap();
        let mut tags = Vec::new();
        for line in content.lines() {
            for cap in tag_re.captures_iter(line) {
                let tag = cap.get(1).unwrap().as_str().to_string();
                if !tags.contains(&tag) {
                    tags.push(tag);
                }
            }
        }
        tags
    }

    /// Enregistre un tag dans la base de données (idempotent)
    fn register_tag(&mut self, name: &str) -> Result<TagId, ImportError> {
        if let Some(&id) = self.tag_name_to_id.get(name) {
            return Ok(id);
        }
        let tag_id = TagId(self.tag_name_to_id.len() as u64 + 1);
        let now = chrono::Utc::now().timestamp();
        self.conn.execute(
            "INSERT OR IGNORE INTO tags (id, name, normalized_name, created_at, updated_at) 
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![tag_id.0 as i64, name, name.to_lowercase(), now, now,],
        )?;
        self.tag_name_to_id.insert(name.to_string(), tag_id);
        Ok(tag_id)
    }

    // ============================================================================
    // EXTRACTION DES PROPRIÉTÉS (Frontmatter)
    // ============================================================================

    /// Extrait les propriétés depuis le frontmatter YAML (délimité par `---`)
    fn extract_frontmatter_properties(&self, content: &str) -> HashMap<String, PropertyValue> {
        let mut properties = HashMap::new();

        // Vérifier si le contenu commence par du frontmatter
        let lines: Vec<&str> = content.lines().collect();
        if lines.is_empty() || !lines[0].trim().starts_with("---") {
            return properties;
        }

        // Trouver la fin du frontmatter
        let mut end_idx = 1;
        while end_idx < lines.len() && !lines[end_idx].trim().starts_with("---") {
            end_idx += 1;
        }

        if end_idx >= lines.len() {
            return properties;
        }

        // Extraire le contenu du frontmatter
        let frontmatter = &lines[1..end_idx].join("\n");

        // Parser comme YAML (format le plus courant)
        if let Ok(serde_yaml::Value::Mapping(map)) =
            serde_yaml::from_str::<serde_yaml::Value>(frontmatter)
        {
            for (key, value) in map {
                if let (Some(k), Some(v)) = (key.as_str(), value.as_str()) {
                    // Détecter le type de la valeur
                    let property_value = match v.parse::<f64>() {
                        Ok(num) => PropertyValue::Number(num),
                        Err(_) => match v.parse::<bool>() {
                            Ok(b) => PropertyValue::Boolean(b),
                            Err(_) => PropertyValue::Text(v.to_string()),
                        },
                    };
                    properties.insert(k.to_string(), property_value);
                }
            }
        }

        properties
    }

    // ============================================================================
    // CRÉATION D'ENTITÉS
    // ============================================================================

    /// Crée une page dans la base de données
    fn create_page(&mut self, title: &str, path: &Path) -> Result<PageId, ImportError> {
        // Vérifier si la page existe déjà
        if let Some(&page_id) = self.page_name_to_id.get(title) {
            return Ok(page_id);
        }

        let filename = path.file_name().and_then(|s| s.to_str()).unwrap_or(title);
        let path_str = path.to_string_lossy().to_string();

        let page_id = PageId(self.next_page_id);
        self.next_page_id += 1;

        self.conn.execute(
            "INSERT INTO pages (id, uuid, path, title, filename, created_at, updated_at) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                page_id.0 as i64,
                uuid::Uuid::new_v4().to_string(),
                &path_str,
                title,
                filename,
                { chrono::Utc::now().timestamp() },
                { chrono::Utc::now().timestamp() },
            ],
        )?;

        self.page_name_to_id.insert(title.to_string(), page_id);

        Ok(page_id)
    }

    /// Crée un bloc dans la base de données (indentation 0)
    fn create_block(&mut self, block: Block) -> Result<(), ImportError> {
        self.create_block_with_indent(block, 0)
    }

    /// Crée un bloc dans la base de données avec un niveau d'indentation
    fn create_block_with_indent(
        &mut self,
        block: Block,
        indent_level: u32,
    ) -> Result<(), ImportError> {
        // Extraire le contenu comme String
        let content_str = match block.content {
            BlockContent::Text(s) => s,
            BlockContent::Children(_) => String::new(),
            BlockContent::RichText { text, .. } => text,
        };

        self.conn.execute(
            "INSERT INTO blocks (id, uuid, page_id, content, block_type, format, 
             order_index, indent_level, is_collapsed, created_at, updated_at, is_deleted) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            params![
                block.id.0 as i64,
                uuid::Uuid::new_v4().to_string(),
                block.page_id.unwrap_or(0) as i64,
                &content_str,
                self.block_type_to_string(&block.block_type),
                "markdown",
                block.position as i64,
                indent_level as i64,
                false as i32, // is_collapsed
                block.created_at as i64,
                block.updated_at as i64,
                false as i32, // is_deleted
            ],
        )?;

        Ok(())
    }

    /// Convertit BlockType en String pour le stockage
    fn block_type_to_string(&self, block_type: &BlockType) -> String {
        match block_type {
            BlockType::Text => "paragraph".to_string(),
            BlockType::Heading(level) => format!("heading_{}", level),
            BlockType::BulletList => "bullet_list".to_string(),
            BlockType::NumberedList => "numbered_list".to_string(),
            BlockType::Code => "code".to_string(),
            BlockType::Quote => "quote".to_string(),
            BlockType::HorizontalRule => "hr".to_string(),
            BlockType::Link => "link".to_string(),
            BlockType::BlockReference(_) => "block_ref".to_string(),
            BlockType::PageReference => "page_ref".to_string(),
            BlockType::Tag => "tag".to_string(),
        }
    }

    /// Crée un lien dans la base de données, avec résolution des wikilinks
    ///
    /// `source_block_id` doit référencer un bloc déjà inséré : la colonne
    /// `links.source_block_id` est NOT NULL avec une FK vers `blocks(id)`
    /// (schema.rs:98-109), et `PRAGMA foreign_keys=ON` est actif
    /// (schema.rs:642).
    fn create_link(
        &mut self,
        import_link: ImportLink,
        source_block_id: BlockId,
    ) -> Result<(), ImportError> {
        let link = import_link.link;

        // Résoudre la cible si c'est un wikilink vers une page déjà importée
        let (target_page_id, is_resolved): (Option<i64>, i32) = match &import_link.target_name {
            Some(name) => match self.page_name_to_id.get(name) {
                Some(&pid) => (Some(pid.0 as i64), 1),
                None => (None, 0),
            },
            None => (None, 1), // lien externe : considéré comme résolu
        };

        let now = chrono::Utc::now().timestamp();
        self.conn.execute(
            "INSERT INTO links (id, uuid, source_block_id, source_page_id, target_page_id, 
             target_block_id, link_type, link_text, url, anchor, is_resolved, created_at, updated_at) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            params![
                link.id.0 as i64,
                uuid::Uuid::new_v4().to_string(),
                source_block_id.0 as i64, // source_block_id (bloc contenant le lien)
                link.from.0 as i64,
                target_page_id,
                Option::<i64>::None, // target_block_id
                link.link_type.as_deref().unwrap_or("page"),
                import_link.text,
                import_link.url,
                Option::<String>::None, // anchor
                is_resolved,
                now,
                now,
            ],
        )?;

        Ok(())
    }

    /// Crée une valeur de propriété (INSERT OR REPLACE pour idempotence)
    fn create_property_value(
        &mut self,
        entity_id: u64,
        entity_type: u8,
        name: &str,
        value: PropertyValue,
    ) -> Result<(), ImportError> {
        // Vérifier si la propriété existe, sinon la créer
        let property_id = if let Some(&id) = self.property_name_to_id.get(name) {
            id
        } else {
            // Créer la propriété
            let property = Property::new(name.to_string(), PropertyType::Text, false);
            let property_id = self.create_property_internal(&property)?;
            self.property_name_to_id
                .insert(name.to_string(), property_id);
            property_id
        };

        // Sauvegarder la valeur
        self.conn.execute(
            "INSERT OR REPLACE INTO property_values (property_id, entity_id, entity_type, value_type, 
             string_value, number_value, boolean_value, date_value, created_at, updated_at) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                property_id as i64,
                entity_id as i64,
                entity_type as i64,
                value.value_type().to_string(),
                Self::value_to_string(&value),
                Self::value_to_number(&value),
                Self::value_to_boolean(&value),
                Self::value_to_date(&value),
                { chrono::Utc::now().timestamp() },
                { chrono::Utc::now().timestamp() },
            ],
        )?;

        Ok(())
    }

    /// Crée une propriété dans la base de données
    fn create_property_internal(&mut self, property: &Property) -> Result<u64, ImportError> {
        let id: i64 = self.conn.query_row(
            "INSERT INTO properties (name, normalized_name, property_type, default_value, 
             options, is_system, is_required, description, display_order, 
             created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11) 
             RETURNING id",
            params![
                &property.name,
                &property.normalized_name,
                property.property_type.to_string(),
                serde_json::to_string(&property.default_value)
                    .unwrap_or_else(|_| "null".to_string()),
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

        Ok(id as u64)
    }

    // ============================================================================
    // FONCTIONS UTILITAIRES
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
            PropertyValue::Number(v) => Some(v.to_string()),
            PropertyValue::Boolean(v) => Some(v.to_string()),
            PropertyValue::None => None,
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

    /// Convertit un PropertyValue en date pour le stockage
    fn value_to_date(value: &PropertyValue) -> Option<String> {
        match value {
            PropertyValue::Date(v) => Some(v.clone()),
            PropertyValue::DateTime(v) => Some(v.clone()),
            _ => None,
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema;
    use tempfile::tempdir;

    /// Helper : crée une base de données temporaire initialisée
    fn make_importer(dir: &tempfile::TempDir) -> DataImporter {
        let db_path = dir.path().join("test.db");
        let conn = schema::Schema::initialize_database(&db_path).unwrap();
        DataImporter::new(conn)
    }

    #[test]
    fn test_import_markdown_file() {
        let dir = tempdir().unwrap();
        let mut importer = make_importer(&dir);

        // Créer un fichier Markdown temporaire
        let md_file = dir.path().join("test.md");
        let content =
            "# Title\n\nThis is a paragraph.\n\n- Item 1\n- Item 2\n\n```rust\nfn main() {}\n```";
        fs::write(&md_file, content).unwrap();

        let result = importer.import_markdown_file(&md_file).unwrap();

        assert_eq!(result.pages_imported, 1);
        assert!(result.blocks_imported > 0);
    }

    #[test]
    fn test_import_markdown_directory() {
        let dir = tempdir().unwrap();
        let mut importer = make_importer(&dir);

        // Créer un dossier avec deux fichiers Markdown (et un sous-dossier)
        let md_dir = dir.path().join("notes");
        fs::create_dir(&md_dir).unwrap();
        fs::write(md_dir.join("a.md"), "# Page A\n\nContent A").unwrap();
        fs::write(md_dir.join("b.md"), "# Page B\n\nContent B").unwrap();
        let sub = md_dir.join("sub");
        fs::create_dir(&sub).unwrap();
        fs::write(sub.join("c.md"), "# Page C").unwrap();

        let result = importer.import_markdown_directory(&md_dir).unwrap();

        assert_eq!(result.pages_imported, 3);
        assert!(result.blocks_imported >= 3);
    }

    #[test]
    fn test_import_auto_detects_format() {
        let dir = tempdir().unwrap();
        let mut importer = make_importer(&dir);

        let md_file = dir.path().join("auto.md");
        fs::write(&md_file, "# Auto\n\nHello").unwrap();

        let result = importer.import_auto(&md_file).unwrap();
        assert_eq!(result.pages_imported, 1);

        // Extension inconnue -> erreur
        let txt_file = dir.path().join("file.txt");
        fs::write(&txt_file, "hello").unwrap();
        let err = importer.import_auto(&txt_file).unwrap_err();
        assert!(matches!(err, ImportError::UnsupportedFormat(_)));
    }

    #[test]
    fn test_import_json_array_of_pages() {
        let dir = tempdir().unwrap();
        let mut importer = make_importer(&dir);

        let json_file = dir.path().join("export.json");
        let json = r##"[
            {
                "title": "Projet Alpha",
                "properties": {"status": "active", "priority": 3, "urgent": true},
                "blocks": [
                    {"content": "# Introduction", "type": "heading_1"},
                    {"content": "Texte avec [[Projet Beta]] et [site](https://example.com)", "type": "paragraph",
                     "children": [
                        {"content": "Sous-élément", "type": "bulleted_list"}
                     ]},
                    {"content": "fn main() {}", "type": "code"}
                ]
            },
            {
                "title": "Projet Beta",
                "properties": {"status": "done"},
                "blocks": [
                    {"content": "Page liée", "type": "paragraph"}
                ]
            }
        ]"##;
        fs::write(&json_file, json).unwrap();

        let result = importer.import_json(&json_file).unwrap();

        assert_eq!(result.pages_imported, 2);
        assert_eq!(result.blocks_imported, 5); // 3 + 1 enfant + 1
        assert_eq!(result.properties_imported, 4); // 3 + 1
        assert_eq!(result.links_imported, 2); // wikilink + lien markdown
    }

    #[test]
    fn test_import_json_object_with_pages_key() {
        let dir = tempdir().unwrap();
        let mut importer = make_importer(&dir);

        let json_file = dir.path().join("export2.json");
        let json = r#"{"pages": [{"name": "My Page", "blocks": [{"content": "Hello", "type": "paragraph"}]}]}"#;
        fs::write(&json_file, json).unwrap();

        let result = importer.import_json(&json_file).unwrap();

        assert_eq!(result.pages_imported, 1);
        assert_eq!(result.blocks_imported, 1);
    }

    #[test]
    fn test_import_json_invalid() {
        let dir = tempdir().unwrap();
        let mut importer = make_importer(&dir);

        let json_file = dir.path().join("bad.json");
        fs::write(&json_file, "{not valid json").unwrap();

        let err = importer.import_json(&json_file).unwrap_err();
        assert!(matches!(err, ImportError::ParseError(_)));
    }

    #[test]
    fn test_import_opml() {
        let dir = tempdir().unwrap();
        let mut importer = make_importer(&dir);

        let opml_file = dir.path().join("outline.opml");
        let opml = r#"<?xml version="1.0" encoding="UTF-8"?>
<opml version="2.0">
  <head><title>My Outline</title></head>
  <body>
    <outline text="Chapitre 1">
      <outline text="Section 1.1"/>
      <outline text="Section 1.2">
        <outline text="Détail 1.2.1"/>
      </outline>
    </outline>
    <outline text="Chapitre 2"/>
  </body>
</opml>"#;
        fs::write(&opml_file, opml).unwrap();

        let result = importer.import_opml(&opml_file).unwrap();

        assert_eq!(result.pages_imported, 2); // Chapitre 1 et Chapitre 2
        assert_eq!(result.blocks_imported, 3); // 1.1, 1.2, 1.2.1
    }

    #[test]
    fn test_extract_frontmatter_properties() {
        let dir = tempdir().unwrap();
        let importer = make_importer(&dir);

        let content = "---\ntitle: My Page\nauthor: John\ndate: 2024-01-01\n---\n\n# My Page";
        let properties = importer.extract_frontmatter_properties(content);

        assert_eq!(properties.len(), 3);
        assert_eq!(
            properties.get("title"),
            Some(&PropertyValue::Text("My Page".to_string()))
        );
    }

    #[test]
    fn test_extract_links() {
        let dir = tempdir().unwrap();
        let mut importer = make_importer(&dir);

        let content = "This is a [link](https://example.com) and a [[wikilink]].";
        let links = importer.extract_links_from_markdown(content, PageId(1));

        assert_eq!(links.len(), 2);
        assert_eq!(links[0].url, Some("https://example.com".to_string()));
        assert_eq!(links[1].target_name, Some("wikilink".to_string()));
    }

    #[test]
    fn test_extract_tags() {
        let dir = tempdir().unwrap();
        let importer = make_importer(&dir);

        let content = "# Title\n\nSome #rust and #programming tags, plus #rust again.";
        let tags = importer.extract_tags_from_markdown(content);

        assert_eq!(tags.len(), 2);
        assert!(tags.contains(&"rust".to_string()));
        assert!(tags.contains(&"programming".to_string()));
    }

    #[test]
    fn test_register_tag_idempotent() {
        let dir = tempdir().unwrap();
        let mut importer = make_importer(&dir);

        let id1 = importer.register_tag("test").unwrap();
        let id2 = importer.register_tag("test").unwrap();
        assert_eq!(id1, id2);

        // Vérifier en base
        let count: i64 = importer
            .conn
            .query_row("SELECT COUNT(*) FROM tags WHERE name = 'test'", [], |row| {
                row.get(0)
            })
            .unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_parse_markdown() {
        let dir = tempdir().unwrap();
        let mut importer = make_importer(&dir);

        let content = "# Heading\n\nParagraph\n\n```rust\ncode\n```\n\n- List";
        let blocks = importer.parse_markdown(content, PageId(1)).unwrap();

        assert!(!blocks.is_empty());
        assert!(blocks
            .iter()
            .any(|b| matches!(b.block_type, BlockType::Heading(1))));
        assert!(blocks
            .iter()
            .any(|b| matches!(b.block_type, BlockType::Text)));
        assert!(blocks
            .iter()
            .any(|b| matches!(b.block_type, BlockType::Code)));
        assert!(blocks
            .iter()
            .any(|b| matches!(b.block_type, BlockType::BulletList)));
    }

    #[test]
    fn test_json_type_to_block_type() {
        assert!(matches!(
            DataImporter::json_type_to_block_type("heading_2"),
            BlockType::Heading(2)
        ));
        assert!(matches!(
            DataImporter::json_type_to_block_type("code"),
            BlockType::Code
        ));
        assert!(matches!(
            DataImporter::json_type_to_block_type("bulleted_list"),
            BlockType::BulletList
        ));
        assert!(matches!(
            DataImporter::json_type_to_block_type("unknown"),
            BlockType::Text
        ));
    }

    #[test]
    fn test_json_value_to_property_value() {
        assert_eq!(
            DataImporter::json_value_to_property_value(&serde_json::json!(42)),
            PropertyValue::Number(42.0)
        );
        assert_eq!(
            DataImporter::json_value_to_property_value(&serde_json::json!(true)),
            PropertyValue::Boolean(true)
        );
        assert_eq!(
            DataImporter::json_value_to_property_value(&serde_json::json!("hello")),
            PropertyValue::Text("hello".to_string())
        );
        assert_eq!(
            DataImporter::json_value_to_property_value(&serde_json::json!(["a", "b"])),
            PropertyValue::MultiSelect(vec!["a".to_string(), "b".to_string()])
        );
    }

    #[test]
    fn test_wikilink_resolution() {
        let dir = tempdir().unwrap();
        let mut importer = make_importer(&dir);

        // Créer deux pages markdown avec un wikilink de l'une vers l'autre
        let file_a = dir.path().join("PageA.md");
        let file_b = dir.path().join("PageB.md");
        fs::write(&file_a, "# A\n\nVoir [[PageB]]").unwrap();
        fs::write(&file_b, "# B\n\nContenu B").unwrap();

        importer.import_markdown_file(&file_b).unwrap();
        let result = importer.import_markdown_file(&file_a).unwrap();

        assert_eq!(result.links_imported, 1);

        // Le wikilink doit être résolu (target_page_id renseigné)
        let resolved: i64 = importer
            .conn
            .query_row(
                "SELECT COUNT(*) FROM links WHERE link_type = 'wikilink' AND is_resolved = 1 AND target_page_id IS NOT NULL",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(resolved, 1);
    }
}
