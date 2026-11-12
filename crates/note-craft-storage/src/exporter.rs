//! Note Craft Storage - Data Export System
//!
//! Ce module fournit les fonctionnalités d'export de données vers différents formats.
//! Il supporte l'export vers Markdown (fichier ou dossier), JSON et HTML.
//!
//! Il est le miroir architectural de `importer.rs` (issue #96) : les requêtes
//! SELECT lisent directement les tables du schéma 4.1 (`schema.rs`) via le
//! `Connection` passé à [`DataExporter::new`]. Les lignes `is_deleted = TRUE`
//! (pages et blocs) sont exclues ; les tables dérivées ou annexes
//! (`search_index`, `graph_nodes`, `graph_edges`, `journals`, `assets`,
//! `metadata`) ne sont jamais exportées.
//!
//! Phase 4 - Étape 4.4 (Data Export) — docs/roadmap/roadmap_v3.md:815

use rusqlite::{params, Connection};
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use thiserror::Error;

// ============================================================================
// ERREURS
// ============================================================================

/// Erreurs d'export (miroir de `ImportError`, importer.rs:24)
#[derive(Error, Debug)]
pub enum ExportError {
    #[error("Erreur IO: {0}")]
    IoError(String),
    #[error("Erreur de sérialisation: {0}")]
    SerializationError(String),
    #[error("Erreur de stockage: {0}")]
    StorageError(String),
    #[error("Fichier non trouvé: {0}")]
    FileNotFound(String),
    #[error("Format non supporté: {0}")]
    UnsupportedFormat(String),
}

impl From<std::io::Error> for ExportError {
    fn from(err: std::io::Error) -> Self {
        ExportError::IoError(err.to_string())
    }
}

impl From<rusqlite::Error> for ExportError {
    fn from(err: rusqlite::Error) -> Self {
        ExportError::StorageError(err.to_string())
    }
}

// ============================================================================
// STRUCTURES D'EXPORT
// ============================================================================

/// Format d'export supporté (miroir de `ImportFormat`, importer.rs:57-80).
///
/// Pas d'OPML : hors critère de la roadmap pour l'export
/// (« Export vers Markdown, JSON, HTML », docs/roadmap/roadmap_v3.md:829).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    /// Fichier Markdown unique
    Markdown,
    /// Dossier de fichiers Markdown (un fichier par page)
    MarkdownDirectory,
    /// Fichier JSON (format d'entrée documenté de `import_json`, importer.rs:212)
    Json,
    /// Fichier HTML autonome
    Html,
}

impl ExportFormat {
    /// Détermine le format à partir de l'extension du fichier.
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "md" | "markdown" => Some(ExportFormat::Markdown),
            "json" => Some(ExportFormat::Json),
            "html" => Some(ExportFormat::Html),
            _ => None,
        }
    }
}

/// Résultat d'un export (miroir de `ImportResult`, importer.rs:82-127)
#[derive(Debug, Clone)]
pub struct ExportResult {
    /// Nombre de pages exportées
    pub pages_exported: usize,
    /// Nombre de blocs exportés
    pub blocks_exported: usize,
    /// Nombre de liens exportés
    pub links_exported: usize,
    /// Nombre de propriétés exportées
    pub properties_exported: usize,
    /// Nombre de tags exportés
    pub tags_exported: usize,
    /// Avertissements générés pendant l'export
    pub warnings: Vec<String>,
    /// Erreurs non bloquantes
    pub errors: Vec<String>,
}

impl Default for ExportResult {
    fn default() -> Self {
        Self::new()
    }
}

impl ExportResult {
    /// Crée un nouveau résultat d'export
    pub fn new() -> Self {
        Self {
            pages_exported: 0,
            blocks_exported: 0,
            links_exported: 0,
            properties_exported: 0,
            tags_exported: 0,
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

/// Ligne de page lue depuis la table `pages` (schema.rs:52)
struct PageRow {
    id: i64,
    title: String,
}

/// Ligne de bloc lue depuis la table `blocks` (schema.rs:76)
struct BlockRow {
    id: i64,
    content: String,
    block_type: String,
    indent_level: i64,
}

// ============================================================================
// EXPORTEUR PRINCIPAL
// ============================================================================

/// Exporteur de données.
///
/// Le JSON produit est exactement le format d'entrée documenté de
/// [`crate::importer::DataImporter::import_json`] (importer.rs:206-213) :
/// objet `{"pages": [...]}`, chaque page `{title, properties, blocks}`, chaque
/// bloc `{content, type, children, properties}` — garantissant un roundtrip
/// `export_json` → `import_json` sans perte.
pub struct DataExporter {
    conn: Connection,
}

impl DataExporter {
    /// Crée un nouvel exporteur
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }

    /// Exporte vers un fichier ou dossier
    pub fn export(&self, path: &Path, format: ExportFormat) -> Result<ExportResult, ExportError> {
        match format {
            ExportFormat::Markdown => self.export_markdown_file(path),
            ExportFormat::MarkdownDirectory => self.export_markdown_directory(path),
            ExportFormat::Json => self.export_json(path),
            ExportFormat::Html => self.export_html(path),
        }
    }

    /// Exporte vers un fichier (détecte le format automatiquement : un
    /// dossier existant déclenche l'export Markdown par dossier, sinon
    /// le format est déduit de l'extension).
    pub fn export_auto(&self, path: &Path) -> Result<ExportResult, ExportError> {
        if path.is_dir() {
            self.export(path, ExportFormat::MarkdownDirectory)
        } else {
            let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
            let format = ExportFormat::from_extension(ext)
                .ok_or_else(|| ExportError::UnsupportedFormat(ext.to_string()))?;
            self.export(path, format)
        }
    }

    // =========================================================================
    // LECTURE DES DONNÉES (schéma 4.1 : SELECT directs, jamais Storage)
    // =========================================================================

    /// Récupère les pages non supprimées, triées par id
    fn fetch_pages(&self) -> Result<Vec<PageRow>, ExportError> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, title FROM pages WHERE is_deleted = FALSE ORDER BY id")?;
        let rows = stmt.query_map([], |row| {
            Ok(PageRow {
                id: row.get(0)?,
                title: row.get(1)?,
            })
        })?;
        let mut pages = Vec::new();
        for row in rows {
            pages.push(row?);
        }
        Ok(pages)
    }

    /// Récupère les blocs non supprimés d'une page, triés par order_index
    fn fetch_blocks(&self, page_id: i64) -> Result<Vec<BlockRow>, ExportError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, content, block_type, indent_level FROM blocks \
             WHERE page_id = ?1 AND is_deleted = FALSE ORDER BY order_index",
        )?;
        let rows = stmt.query_map(params![page_id], |row| {
            Ok(BlockRow {
                id: row.get(0)?,
                content: row.get(1)?,
                block_type: row.get(2)?,
                indent_level: row.get(3)?,
            })
        })?;
        let mut blocks = Vec::new();
        for row in rows {
            blocks.push(row?);
        }
        Ok(blocks)
    }

    /// Récupère les valeurs de propriété d'une entité (entity_type 0 = page,
    /// 1 = bloc : miroir des écritures de l'importeur), triées par nom
    fn fetch_properties(
        &self,
        entity_id: i64,
        entity_type: i64,
    ) -> Result<Vec<(String, serde_json::Value)>, ExportError> {
        let mut stmt = self.conn.prepare(
            "SELECT p.name, pv.value_type, pv.string_value, pv.number_value, \
             pv.boolean_value, pv.date_value \
             FROM property_values pv JOIN properties p ON p.id = pv.property_id \
             WHERE pv.entity_id = ?1 AND pv.entity_type = ?2 ORDER BY p.name",
        )?;
        let rows = stmt.query_map(params![entity_id, entity_type], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, Option<String>>(1)?.unwrap_or_default(),
                row.get::<_, Option<String>>(2)?,
                row.get::<_, Option<f64>>(3)?,
                row.get::<_, Option<i64>>(4)?,
                row.get::<_, Option<String>>(5)?,
            ))
        })?;
        let mut props = Vec::new();
        for row in rows {
            let (name, value_type, string_value, number_value, boolean_value, date_value) = row?;
            props.push((
                name,
                Self::stored_value_to_json(
                    &value_type,
                    string_value,
                    number_value,
                    boolean_value,
                    date_value,
                ),
            ));
        }
        Ok(props)
    }

    /// Récupère les noms de tags du registre global, triés
    fn fetch_tag_names(&self) -> Result<Vec<String>, ExportError> {
        let mut stmt = self.conn.prepare("SELECT name FROM tags ORDER BY name")?;
        let rows = stmt.query_map([], |row| row.get::<_, String>(0))?;
        let mut tags = Vec::new();
        for row in rows {
            tags.push(row?);
        }
        Ok(tags)
    }

    /// Construit le résultat d'export : métriques issues de requêtes
    /// d'agrégation sur les données vivantes (pages et blocs non
    /// supprimés) ; les liens sont comptés uniquement si leur bloc et
    /// leur page sources sont vivants.
    fn build_export_result(&self) -> Result<ExportResult, ExportError> {
        let count = |sql: &str| -> Result<usize, ExportError> {
            Ok(self.conn.query_row(sql, [], |r| r.get::<_, i64>(0))? as usize)
        };
        Ok(ExportResult {
            pages_exported: count("SELECT COUNT(*) FROM pages WHERE is_deleted = FALSE")?,
            blocks_exported: count(
                "SELECT COUNT(*) FROM blocks b JOIN pages p ON p.id = b.page_id \
                 WHERE b.is_deleted = FALSE AND p.is_deleted = FALSE",
            )?,
            links_exported: count(
                "SELECT COUNT(*) FROM links l \
                 JOIN pages p ON p.id = l.source_page_id \
                 JOIN blocks b ON b.id = l.source_block_id \
                 WHERE p.is_deleted = FALSE AND b.is_deleted = FALSE",
            )?,
            properties_exported: count(
                "SELECT COUNT(*) FROM property_values pv WHERE \
                 (pv.entity_type = 0 AND pv.entity_id IN \
                  (SELECT id FROM pages WHERE is_deleted = FALSE)) OR \
                 (pv.entity_type = 1 AND pv.entity_id IN \
                  (SELECT b.id FROM blocks b JOIN pages p ON p.id = b.page_id \
                   WHERE b.is_deleted = FALSE AND p.is_deleted = FALSE))",
            )?,
            tags_exported: count("SELECT COUNT(*) FROM tags")?,
            warnings: Vec::new(),
            errors: Vec::new(),
        })
    }

    // =========================================================================
    // EXPORT JSON
    // =========================================================================

    /// Exporte toutes les pages vers un fichier JSON, exactement au format
    /// d'entrée documenté de `import_json` (importer.rs:206-213) :
    /// `{"pages": [{"title", "properties", "blocks": [...]}]}`.
    ///
    /// Les blocs sont exportés triés par order_index et imbriqués en
    /// `children` selon leur `indent_level` (miroir de la reconstruction de
    /// l'indentation par `import_json_block`, importer.rs:295-327).
    pub fn export_json(&self, path: &Path) -> Result<ExportResult, ExportError> {
        let pages = self.fetch_pages()?;
        let mut pages_json: Vec<serde_json::Value> = Vec::new();
        for page in &pages {
            let props = self.fetch_properties(page.id, 0)?;
            let mut properties = serde_json::Map::new();
            for (name, value) in props {
                properties.insert(name, value);
            }
            let blocks = self.fetch_blocks(page.id)?;
            let mut pos = 0;
            let blocks_json = self.blocks_to_json(&blocks, &mut pos, 0)?;
            pages_json.push(serde_json::json!({
                "title": page.title,
                "properties": serde_json::Value::Object(properties),
                "blocks": blocks_json,
            }));
        }
        let doc = serde_json::json!({ "pages": pages_json });
        let serialized = serde_json::to_string_pretty(&doc)
            .map_err(|e| ExportError::SerializationError(e.to_string()))?;
        fs::write(path, serialized)?;
        self.build_export_result()
    }

    /// Construit les blocs JSON récursivement : un bloc dont l'`indent_level`
    /// est strictement supérieur à celui du bloc précédent devient un de ses
    /// `children`.
    fn blocks_to_json(
        &self,
        rows: &[BlockRow],
        pos: &mut usize,
        indent: i64,
    ) -> Result<Vec<serde_json::Value>, ExportError> {
        let mut nodes = Vec::new();
        while *pos < rows.len() {
            let row = &rows[*pos];
            if row.indent_level < indent {
                break;
            }
            *pos += 1;
            let children = self.blocks_to_json(rows, pos, row.indent_level + 1)?;
            let props = self.fetch_properties(row.id, 1)?;
            let mut properties = serde_json::Map::new();
            for (name, value) in props {
                properties.insert(name, value);
            }
            nodes.push(serde_json::json!({
                "content": row.content,
                "type": Self::json_type_from_db(&row.block_type),
                "properties": serde_json::Value::Object(properties),
                "children": children,
            }));
        }
        Ok(nodes)
    }

    // =========================================================================
    // EXPORT MARKDOWN
    // =========================================================================

    /// Exporte toutes les pages vers un fichier Markdown unique : les pages
    /// sont concaténées ; le frontmatter (propriétés de page) n'est émis que
    /// si la base contient exactement une page (miroir du lecteur de
    /// frontmatter, qui ne lit que l'en-tête de fichier, importer.rs:885).
    pub fn export_markdown_file(&self, path: &Path) -> Result<ExportResult, ExportError> {
        let pages = self.fetch_pages()?;
        let include_frontmatter = pages.len() == 1;
        let mut out = String::new();
        for (index, page) in pages.iter().enumerate() {
            if index > 0 {
                out.push('\n');
            }
            out.push_str(&self.page_to_markdown(page, include_frontmatter)?);
        }
        fs::write(path, out)?;
        let mut result = self.build_export_result()?;
        if pages.len() > 1 {
            result.add_warning(format!(
                "Mode fichier unique : {} pages concaténées, propriétés de page non exportées",
                pages.len()
            ));
        }
        Ok(result)
    }

    /// Exporte chaque page vers un fichier `<titre>.md` du dossier (nom de
    /// fichier dérivé du titre de page, sanitizé et dédupliqué)
    pub fn export_markdown_directory(&self, dir: &Path) -> Result<ExportResult, ExportError> {
        if dir.exists() && !dir.is_dir() {
            return Err(ExportError::FileNotFound(format!(
                "{} est un fichier, pas un dossier",
                dir.display()
            )));
        }
        let pages = self.fetch_pages()?;
        fs::create_dir_all(dir)?;
        let mut used: HashSet<String> = HashSet::new();
        let mut result = self.build_export_result()?;
        for page in &pages {
            let mut filename = Self::sanitize_filename(&page.title);
            if !used.insert(filename.clone()) {
                let mut suffix = 2;
                while !used.insert(format!("{filename}-{suffix}")) {
                    suffix += 1;
                }
                filename = format!("{filename}-{suffix}");
            }
            let file_path = dir.join(format!("{filename}.md"));
            match self.page_to_markdown(page, true) {
                Ok(markdown) => {
                    if let Err(e) = fs::write(&file_path, markdown) {
                        result.add_error(format!("{}: {e}", file_path.display()));
                    }
                }
                Err(e) => {
                    result.add_error(format!("{}: {e}", file_path.display()));
                }
            }
        }
        Ok(result)
    }

    /// Génère le Markdown d'une page (frontmatter optionnel + blocs), avec
    /// les mêmes patterns que le parseur maison de l'importeur :
    /// `# titres`, `- listes`, `1. listes ordonnées`, `> citations`,
    /// des clôtures « ``` » pour les blocs de code (importer.rs:527-620).
    fn page_to_markdown(
        &self,
        page: &PageRow,
        include_frontmatter: bool,
    ) -> Result<String, ExportError> {
        let mut out = String::new();
        if include_frontmatter {
            let props = self.fetch_properties(page.id, 0)?;
            if !props.is_empty() {
                out.push_str("---\n");
                for (name, value) in props {
                    out.push_str(&format!("{name}: {}\n", Self::yaml_scalar(&value)));
                }
                out.push_str("---\n\n");
            }
        }
        let blocks = self.fetch_blocks(page.id)?;
        for block in &blocks {
            Self::block_to_markdown(block, &mut out);
        }
        Ok(out)
    }

    /// Écrit un bloc en Markdown selon le même dialecte que le parseur
    /// maison de l'importeur ; une ligne vide sépare les blocs.
    fn block_to_markdown(row: &BlockRow, out: &mut String) {
        let level = row
            .block_type
            .strip_prefix("heading_")
            .and_then(|lvl| lvl.parse::<usize>().ok())
            .unwrap_or(0);
        match row.block_type.as_str() {
            _ if (1..=6).contains(&level) => {
                out.push_str(&"#".repeat(level));
                out.push(' ');
                out.push_str(row.content.trim());
                out.push('\n');
            }
            "bullet_list" => {
                for line in row.content.lines() {
                    out.push_str("- ");
                    out.push_str(line);
                    out.push('\n');
                }
            }
            "numbered_list" => {
                for (index, line) in row.content.lines().enumerate() {
                    out.push_str(&format!("{}. ", index + 1));
                    out.push_str(line);
                    out.push('\n');
                }
            }
            "quote" => {
                for line in row.content.lines() {
                    out.push_str("> ");
                    out.push_str(line);
                    out.push('\n');
                }
            }
            "code" => {
                let trimmed = row.content.trim();
                // Le contenu des blocs de code importés contient déjà les
                // clôtures ``` (importer.rs : create_code_block)
                if trimmed.starts_with("```") {
                    out.push_str(trimmed);
                } else {
                    out.push_str("```\n");
                    out.push_str(trimmed);
                    out.push_str("\n```");
                }
                out.push('\n');
            }
            "hr" => {
                out.push_str("---\n");
            }
            // paragraph, page_ref, block_ref, tag, link, types inconnus :
            // contenu brut (le parseur lit/produit des paragraphes)
            _ => {
                out.push_str(&row.content);
                if !row.content.ends_with('\n') {
                    out.push('\n');
                }
            }
        }
        out.push('\n');
    }

    /// Écrit un scalaire de frontmatter YAML. Toujours entre guillemets :
    /// le lecteur de frontmatter de l'importeur ne garde que les scalaires
    /// chaînes (importer.rs : extract_frontmatter_properties) et re-typer
    /// lui-même nombres/booléens.
    fn yaml_scalar(value: &serde_json::Value) -> String {
        match value {
            serde_json::Value::Number(n) => format!("\"{n}\""),
            serde_json::Value::Bool(b) => format!("\"{b}\""),
            serde_json::Value::Array(items) => {
                let joined = items
                    .iter()
                    .map(|item| item.as_str().unwrap_or_default())
                    .collect::<Vec<&str>>()
                    .join(", ");
                format!("\"{joined}\"")
            }
            serde_json::Value::String(s) => {
                format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
            }
            _ => "\"\"".to_string(),
        }
    }

    // =========================================================================
    // EXPORT HTML
    // =========================================================================

    /// Exporte toutes les pages vers un fichier HTML autonome : `<meta
    /// charset>` et `<title>` dans `<head>`, une `<section>` par page avec
    /// `<h1>` titre, blocs mappés selon `block_type`. L'échappement est
    /// écrit à la main ([`Self::escape_html`]), sans crate ajoutée.
    pub fn export_html(&self, path: &Path) -> Result<ExportResult, ExportError> {
        let pages = self.fetch_pages()?;
        let tags = self.fetch_tag_names()?;
        let mut out = String::new();
        out.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        out.push_str("<meta charset=\"utf-8\">\n");
        out.push_str("<title>Note Craft Export</title>\n");
        if !tags.is_empty() {
            out.push_str(&format!(
                "<meta name=\"keywords\" content=\"{}\">\n",
                Self::escape_html(&tags.join(", "))
            ));
        }
        out.push_str("</head>\n<body>\n");
        for page in &pages {
            out.push_str(&format!("<section id=\"page-{}\">\n", page.id));
            out.push_str(&format!("<h1>{}</h1>\n", Self::escape_html(&page.title)));
            let blocks = self.fetch_blocks(page.id)?;
            for block in &blocks {
                Self::block_to_html(block, &mut out);
            }
            out.push_str("</section>\n");
        }
        out.push_str("</body>\n</html>\n");
        fs::write(path, out)?;
        self.build_export_result()
    }

    /// Écrit un bloc en HTML selon son `block_type`
    fn block_to_html(row: &BlockRow, out: &mut String) {
        let level = row
            .block_type
            .strip_prefix("heading_")
            .and_then(|lvl| lvl.parse::<usize>().ok())
            .unwrap_or(0);
        match row.block_type.as_str() {
            _ if (1..=6).contains(&level) => {
                out.push_str(&format!(
                    "<h{level}>{}</h{level}>\n",
                    Self::escape_html(row.content.trim())
                ));
            }
            "bullet_list" => {
                out.push_str("<ul>\n");
                for line in row.content.lines() {
                    out.push_str(&format!("<li>{}</li>\n", Self::escape_html(line)));
                }
                out.push_str("</ul>\n");
            }
            "numbered_list" => {
                out.push_str("<ol>\n");
                for line in row.content.lines() {
                    out.push_str(&format!("<li>{}</li>\n", Self::escape_html(line)));
                }
                out.push_str("</ol>\n");
            }
            "code" => {
                out.push_str(&format!(
                    "<pre><code>{}</code></pre>\n",
                    Self::escape_html(&Self::strip_code_fences(&row.content))
                ));
            }
            "quote" => {
                out.push_str(&format!(
                    "<blockquote>{}</blockquote>\n",
                    Self::escape_html(row.content.trim())
                ));
            }
            "hr" => {
                out.push_str("<hr>\n");
            }
            // paragraph, page_ref, block_ref, tag, link, types inconnus
            _ => {
                out.push_str(&format!(
                    "<p>{}</p>\n",
                    Self::escape_html(row.content.trim())
                ));
            }
        }
    }

    // =========================================================================
    // CONVERSIONS ET UTILITAIRES
    // =========================================================================

    /// Convertit une valeur stockée (`value_type` + colonnes typées, comme
    /// les écrit l'importer : `PropertyValue::value_type().to_string()`,
    /// note-craft-core/src/property.rs:171) en valeur JSON du format
    /// d'import.
    fn stored_value_to_json(
        value_type: &str,
        string_value: Option<String>,
        number_value: Option<f64>,
        boolean_value: Option<i64>,
        date_value: Option<String>,
    ) -> serde_json::Value {
        match value_type {
            "number" => serde_json::json!(number_value.unwrap_or(0.0)),
            "boolean" => serde_json::json!(boolean_value.map(|b| b != 0).unwrap_or(false)),
            "multiselect" => serde_json::json!(string_value
                .map(|s| s
                    .split(',')
                    .map(|item| item.trim().to_string())
                    .collect::<Vec<String>>())
                .unwrap_or_default()),
            "date" | "datetime" => {
                serde_json::json!(date_value.or(string_value).unwrap_or_default())
            }
            // text, select, url, email, color
            _ => serde_json::json!(string_value.unwrap_or_default()),
        }
    }

    /// Convertit le `block_type` stocké (chaînes écrites par
    /// `DataImporter::block_type_to_string`, importer.rs) en chaîne `type`
    /// du format JSON d'import (`json_type_to_block_type`, importer.rs).
    ///
    /// « page_ref » est converti en « page_reference », la seule chaîne
    /// que `import_json` mappe vers `BlockType::PageReference`, pour un
    /// roundtrip sans perte. « link » n'a pas d'équivalent côté import
    /// (retombe en paragraphe).
    fn json_type_from_db(block_type: &str) -> String {
        match block_type {
            "paragraph" | "heading_1" | "heading_2" | "heading_3" | "heading_4" | "heading_5"
            | "heading_6" | "bullet_list" | "numbered_list" | "code" | "quote" | "hr"
            | "block_ref" | "tag" => block_type.to_string(),
            "page_ref" => "page_reference".to_string(),
            _ => "paragraph".to_string(),
        }
    }

    /// Extrait le contenu d'un bloc de code des clôtures « ``` » que l'importeur
    /// stocke dans le contenu (importer.rs : create_code_fences), pour le
    /// rendu HTML.
    fn strip_code_fences(content: &str) -> String {
        let trimmed = content.trim();
        let inner = match trimmed.strip_prefix("```") {
            Some(rest) => match rest.find('\n') {
                Some(pos) => &rest[pos + 1..],
                None => "",
            },
            None => return trimmed.to_string(),
        };
        match inner.strip_suffix("```") {
            Some(body) => body.trim_end().to_string(),
            None => inner.trim_end().to_string(),
        }
    }

    /// Échappe `& < > " '` en entités HTML, à la main (aucune crate)
    fn escape_html(input: &str) -> String {
        let mut out = String::with_capacity(input.len());
        for c in input.chars() {
            match c {
                '&' => out.push_str("&amp;"),
                '<' => out.push_str("&lt;"),
                '>' => out.push_str("&gt;"),
                '"' => out.push_str("&quot;"),
                '\'' => out.push_str("&#39;"),
                _ => out.push(c),
            }
        }
        out
    }

    /// Sanitise un titre de page en nom de fichier sûr : les caractères
    /// interdits ou fragiles d'un nom de fichier (`/ \ : * ? " < > |` et
    /// les caractères de contrôle) deviennent `_` ; un titre vide donne
    /// « untitled ».
    fn sanitize_filename(title: &str) -> String {
        let mut name = String::with_capacity(title.len());
        for c in title.chars() {
            match c {
                '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => name.push('_'),
                c if (c as u32) < 32 => name.push('_'),
                c => name.push(c),
            }
        }
        let name = name.trim().to_string();
        if name.is_empty() {
            "untitled".to_string()
        } else {
            name
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::importer::DataImporter;
    use crate::schema;
    use tempfile::tempdir;

    /// Helper : crée une base de données temporaire initialisée + exporteur
    fn make_exporter(dir: &tempfile::TempDir) -> DataExporter {
        let db_path = dir.path().join("test.db");
        let conn = schema::Schema::initialize_database(&db_path).unwrap();
        DataExporter::new(conn)
    }

    /// Insère une page vivante directement en SQL (schéma 4.1)
    fn insert_page(ex: &DataExporter, id: i64, title: &str) {
        ex.conn
            .execute(
                "INSERT INTO pages (id, uuid, path, title, filename, created_at, updated_at) \
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    id,
                    format!("uuid-page-{id}"),
                    format!("pages/{id}.md"),
                    title,
                    format!("{title}.md"),
                    0,
                    0
                ],
            )
            .unwrap();
    }

    /// Insère un bloc vivant directement en SQL (schéma 4.1)
    #[allow(clippy::too_many_arguments)]
    fn insert_block(
        ex: &DataExporter,
        id: i64,
        page_id: i64,
        content: &str,
        block_type: &str,
        order_index: i64,
        indent_level: i64,
    ) {
        ex.conn
            .execute(
                "INSERT INTO blocks (id, uuid, page_id, content, block_type, format, \
                 order_index, indent_level, is_collapsed, created_at, updated_at, is_deleted) \
                 VALUES (?1, ?2, ?3, ?4, ?5, 'markdown', ?6, ?7, 0, 0, 0, 0)",
                params![
                    id,
                    format!("uuid-block-{id}"),
                    page_id,
                    content,
                    block_type,
                    order_index,
                    indent_level
                ],
            )
            .unwrap();
    }

    /// Insère une propriété (définition + valeur) ; miroir des écritures de
    /// l'importeur (`entity_type` 0 = page, 1 = bloc)
    #[allow(clippy::too_many_arguments)]
    fn insert_property(
        ex: &DataExporter,
        entity_id: i64,
        entity_type: i64,
        name: &str,
        value_type: &str,
        string_value: Option<&str>,
        number_value: Option<f64>,
        boolean_value: Option<i64>,
    ) {
        ex.conn
            .execute(
                "INSERT OR IGNORE INTO properties (name, normalized_name, property_type, \
                 default_value, options, is_system, is_required, description, display_order, \
                 created_at, updated_at) VALUES (?1, ?1, 'text', NULL, NULL, 0, 0, '', 0, 0, 0)",
                params![name],
            )
            .unwrap();
        let property_id: i64 = ex
            .conn
            .query_row(
                "SELECT id FROM properties WHERE name = ?1",
                params![name],
                |r| r.get(0),
            )
            .unwrap();
        ex.conn
            .execute(
                "INSERT OR REPLACE INTO property_values (property_id, entity_id, entity_type, \
                 value_type, string_value, number_value, boolean_value, date_value, \
                 created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, NULL, 0, 0)",
                params![
                    property_id,
                    entity_id,
                    entity_type,
                    value_type,
                    string_value,
                    number_value,
                    boolean_value
                ],
            )
            .unwrap();
    }

    /// Insère un lien (source = bloc existant, miroir du format de l'importeur)
    fn insert_link(
        ex: &DataExporter,
        id: i64,
        source_block_id: i64,
        source_page_id: i64,
        target_page_id: Option<i64>,
    ) {
        ex.conn
            .execute(
                "INSERT INTO links (id, uuid, source_block_id, source_page_id, target_page_id, \
                 target_block_id, link_type, link_text, url, anchor, is_resolved, \
                 created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, NULL, 'wikilink', \
                 'cible', NULL, NULL, 1, 0, 0)",
                params![
                    id,
                    format!("uuid-link-{id}"),
                    source_block_id,
                    source_page_id,
                    target_page_id
                ],
            )
            .unwrap();
    }

    /// Insère un tag dans le registre global
    fn insert_tag(ex: &DataExporter, id: i64, name: &str) {
        ex.conn
            .execute(
                "INSERT OR IGNORE INTO tags (id, name, normalized_name, created_at, \
                 updated_at) VALUES (?1, ?2, ?3, 0, 0)",
                params![id, name, name.to_lowercase()],
            )
            .unwrap();
    }

    #[test]
    fn test_from_extension() {
        assert_eq!(
            ExportFormat::from_extension("md"),
            Some(ExportFormat::Markdown)
        );
        assert_eq!(
            ExportFormat::from_extension("markdown"),
            Some(ExportFormat::Markdown)
        );
        assert_eq!(
            ExportFormat::from_extension("MD"),
            Some(ExportFormat::Markdown)
        );
        assert_eq!(
            ExportFormat::from_extension("json"),
            Some(ExportFormat::Json)
        );
        assert_eq!(
            ExportFormat::from_extension("html"),
            Some(ExportFormat::Html)
        );
        assert_eq!(ExportFormat::from_extension("txt"), None);
        assert_eq!(ExportFormat::from_extension(""), None);
    }

    #[test]
    fn test_escape_html() {
        assert_eq!(
            DataExporter::escape_html("a<b>c&d\"e'f"),
            "a&lt;b&gt;c&amp;d&quot;e&#39;f"
        );
    }

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(
            DataExporter::sanitize_filename("a/b\\c:d*e?f\"g<h>i|j"),
            "a_b_c_d_e_f_g_h_i_j"
        );
        assert_eq!(DataExporter::sanitize_filename("   "), "untitled");
    }

    #[test]
    fn test_export_json_structure() {
        let dir = tempdir().unwrap();
        let ex = make_exporter(&dir);

        insert_page(&ex, 1, "Ma Page");
        insert_block(&ex, 1, 1, "Titre de section", "heading_2", 0, 0);
        insert_block(&ex, 2, 1, "Intro\n", "paragraph", 1, 0);
        insert_block(&ex, 3, 1, "Sous élément\n", "bullet_list", 2, 1);
        insert_property(&ex, 1, 0, "status", "text", Some("active"), None, None);
        insert_property(&ex, 1, 0, "priority", "number", None, Some(3.0), None);
        insert_property(&ex, 1, 0, "urgent", "boolean", None, None, Some(1));
        insert_property(&ex, 2, 1, "done", "boolean", None, None, Some(0));

        let json_path = dir.path().join("export.json");
        let result = ex.export_json(&json_path).unwrap();
        assert_eq!(result.pages_exported, 1);
        assert_eq!(result.blocks_exported, 3);
        assert_eq!(result.properties_exported, 4);

        let content = fs::read_to_string(&json_path).unwrap();
        let value: serde_json::Value = serde_json::from_str(&content).unwrap();
        let pages = value["pages"].as_array().unwrap();
        assert_eq!(pages.len(), 1);
        assert_eq!(pages[0]["title"], serde_json::json!("Ma Page"));

        let props = &pages[0]["properties"];
        assert_eq!(props["status"], serde_json::json!("active"));
        assert_eq!(props["priority"].as_f64(), Some(3.0));
        assert_eq!(props["urgent"], serde_json::json!(true));

        let blocks = pages[0]["blocks"].as_array().unwrap();
        assert_eq!(blocks.len(), 2);
        assert_eq!(blocks[0]["type"], serde_json::json!("heading_2"));
        assert_eq!(blocks[0]["content"], serde_json::json!("Titre de section"));
        assert_eq!(blocks[1]["type"], serde_json::json!("paragraph"));
        assert_eq!(blocks[1]["properties"]["done"], serde_json::json!(false));

        let children = blocks[1]["children"].as_array().unwrap();
        assert_eq!(children.len(), 1);
        assert_eq!(children[0]["type"], serde_json::json!("bullet_list"));
        assert_eq!(children[0]["content"], serde_json::json!("Sous élément\n"));
    }

    #[test]
    fn test_roundtrip_json_counts() {
        let dir = tempdir().unwrap();
        let ex = make_exporter(&dir);

        insert_page(&ex, 1, "Alpha");
        insert_page(&ex, 2, "Beta");
        insert_block(&ex, 1, 1, "Intro", "heading_1", 0, 0);
        insert_block(&ex, 2, 1, "Voir [[Beta]] #rust\n", "paragraph", 1, 0);
        insert_block(&ex, 3, 1, "Sous\n", "bullet_list", 2, 1);
        insert_block(&ex, 4, 2, "Fin\n", "paragraph", 0, 0);
        insert_property(&ex, 1, 0, "status", "text", Some("active"), None, None);
        insert_property(&ex, 4, 1, "flag", "boolean", None, None, Some(1));
        insert_link(&ex, 1, 2, 1, Some(2));
        insert_tag(&ex, 1, "rust");

        let json_path = dir.path().join("roundtrip.json");
        let export_result = ex.export_json(&json_path).unwrap();
        assert_eq!(export_result.pages_exported, 2);
        assert_eq!(export_result.blocks_exported, 4);
        assert_eq!(export_result.links_exported, 1);
        assert_eq!(export_result.properties_exported, 2);
        assert_eq!(export_result.tags_exported, 1);

        // Ré-import du fichier exporté dans une base vierge
        let dir2 = tempdir().unwrap();
        let db2_path = dir2.path().join("roundtrip.db");
        let conn2 = schema::Schema::initialize_database(&db2_path).unwrap();
        let mut importer = DataImporter::new(conn2);
        let import_result = importer.import_json(&json_path).unwrap();

        assert_eq!(import_result.pages_imported, export_result.pages_exported);
        assert_eq!(import_result.blocks_imported, export_result.blocks_exported);
        assert_eq!(import_result.links_imported, export_result.links_exported);
        assert_eq!(
            import_result.properties_imported,
            export_result.properties_exported
        );
        assert_eq!(import_result.tags_imported, export_result.tags_exported);
    }

    #[test]
    fn test_roundtrip_json_rebuilds_indentation() {
        let dir = tempdir().unwrap();
        let ex = make_exporter(&dir);

        insert_page(&ex, 1, "Page");
        insert_block(&ex, 1, 1, "Parent\n", "paragraph", 0, 0);
        insert_block(&ex, 2, 1, "Enfant\n", "paragraph", 1, 1);
        insert_block(&ex, 3, 1, "Frère\n", "paragraph", 2, 0);

        let json_path = dir.path().join("indent.json");
        ex.export_json(&json_path).unwrap();

        let dir2 = tempdir().unwrap();
        let db2_path = dir2.path().join("indent.db");
        let conn2 = schema::Schema::initialize_database(&db2_path).unwrap();
        let mut importer = DataImporter::new(conn2);
        importer.import_json(&json_path).unwrap();

        // Vérification directe en base : l'indentation est reconstruite
        // par l'imbrication des `children`
        let conn2 = rusqlite::Connection::open(&db2_path).unwrap();
        let mut stmt = conn2
            .prepare("SELECT indent_level FROM blocks ORDER BY page_id, order_index")
            .unwrap();
        let indents: Vec<i64> = stmt
            .query_map([], |row| row.get(0))
            .unwrap()
            .filter_map(|r| r.ok())
            .collect();
        assert_eq!(indents, vec![0, 1, 0]);
    }

    #[test]
    fn test_roundtrip_markdown_file() {
        let dir = tempdir().unwrap();
        let ex = make_exporter(&dir);

        insert_page(&ex, 1, "Ma Page");
        insert_block(&ex, 1, 1, "Titre", "heading_1", 0, 0);
        insert_block(&ex, 2, 1, "Texte\n", "paragraph", 1, 0);
        insert_block(&ex, 3, 1, "a\nb\n", "bullet_list", 2, 0);
        insert_block(&ex, 4, 1, "x\ny\n", "numbered_list", 3, 0);
        insert_block(&ex, 5, 1, "q\n", "quote", 4, 0);
        insert_block(&ex, 6, 1, "```rust\nfn main() {}\n```", "code", 5, 0);

        let md_path = dir.path().join("Ma Page.md");
        let export_result = ex.export_markdown_file(&md_path).unwrap();
        assert_eq!(export_result.pages_exported, 1);
        assert_eq!(export_result.blocks_exported, 6);

        let dir2 = tempdir().unwrap();
        let db2_path = dir2.path().join("md.db");
        let conn2 = schema::Schema::initialize_database(&db2_path).unwrap();
        let mut importer = DataImporter::new(conn2);
        let import_result = importer.import_markdown_file(&md_path).unwrap();

        assert_eq!(import_result.pages_imported, 1);
        assert_eq!(import_result.blocks_imported, export_result.blocks_exported);
        assert_eq!(
            import_result.properties_imported,
            export_result.properties_exported
        );
    }

    #[test]
    fn test_export_markdown_directory_multiple_pages() {
        let dir = tempdir().unwrap();
        let ex = make_exporter(&dir);

        insert_page(&ex, 1, "Alpha");
        insert_page(&ex, 2, "Beta");
        insert_page(&ex, 3, "a/b:c?");
        insert_block(&ex, 1, 1, "Contenu A\n", "paragraph", 0, 0);
        insert_block(&ex, 2, 2, "Contenu B\n", "paragraph", 0, 0);
        insert_block(&ex, 3, 3, "Contenu C\n", "paragraph", 0, 0);

        let out_dir = dir.path().join("out");
        let result = ex.export_markdown_directory(&out_dir).unwrap();
        assert_eq!(result.pages_exported, 3);
        assert_eq!(result.blocks_exported, 3);
        assert!(out_dir.is_dir());

        let mut names: Vec<String> = Vec::new();
        for entry in fs::read_dir(&out_dir).unwrap() {
            let entry = entry.unwrap();
            names.push(entry.file_name().to_string_lossy().to_string());
        }
        names.sort();
        assert_eq!(names, vec!["Alpha.md", "Beta.md", "a_b_c_.md"]);
    }

    #[test]
    fn test_roundtrip_markdown_directory_counts() {
        let dir = tempdir().unwrap();
        let ex = make_exporter(&dir);

        insert_page(&ex, 1, "Premier");
        insert_page(&ex, 2, "Deuxieme");
        insert_block(&ex, 1, 1, "Titre", "heading_1", 0, 0);
        insert_block(&ex, 2, 1, "Voir [[Deuxieme]] #rust\n", "paragraph", 1, 0);
        insert_block(&ex, 3, 1, "a\nb\n", "bullet_list", 2, 0);
        insert_block(&ex, 4, 1, "x\ny\n", "numbered_list", 3, 0);
        insert_block(&ex, 5, 1, "q\n", "quote", 4, 0);
        insert_block(&ex, 6, 1, "```rust\nfn f() {}\n```", "code", 5, 0);
        insert_block(&ex, 7, 2, "Contenu\n", "paragraph", 0, 0);
        insert_link(&ex, 1, 2, 1, Some(2));
        insert_tag(&ex, 1, "rust");

        let out_dir = dir.path().join("export_md");
        let export_result = ex.export_markdown_directory(&out_dir).unwrap();
        assert_eq!(export_result.pages_exported, 2);
        assert_eq!(export_result.blocks_exported, 7);
        assert_eq!(export_result.links_exported, 1);
        assert_eq!(export_result.tags_exported, 1);

        let dir2 = tempdir().unwrap();
        let db2_path = dir2.path().join("mddir.db");
        let conn2 = schema::Schema::initialize_database(&db2_path).unwrap();
        let mut importer = DataImporter::new(conn2);
        let import_result = importer.import_markdown_directory(&out_dir).unwrap();

        assert_eq!(import_result.pages_imported, 2);
        assert_eq!(import_result.blocks_imported, export_result.blocks_exported);
        assert_eq!(import_result.links_imported, export_result.links_exported);
        assert_eq!(
            import_result.properties_imported,
            export_result.properties_exported
        );
        assert_eq!(import_result.tags_imported, export_result.tags_exported);
    }

    #[test]
    fn test_export_html() {
        let dir = tempdir().unwrap();
        let ex = make_exporter(&dir);

        insert_page(&ex, 1, "Titre & <b>");
        insert_page(&ex, 2, "Deux");
        insert_block(&ex, 1, 1, "Sous-titre", "heading_2", 0, 0);
        insert_block(
            &ex,
            2,
            1,
            "<script>alert(\"&\");</script>\n",
            "paragraph",
            1,
            0,
        );
        insert_block(&ex, 3, 1, "a\nb\n", "bullet_list", 2, 0);
        insert_block(&ex, 4, 1, "```rust\nfn f() {}\n```", "code", 3, 0);
        insert_block(&ex, 5, 1, "citation\n", "quote", 4, 0);
        insert_block(&ex, 6, 1, "---", "hr", 5, 0);
        insert_block(&ex, 7, 2, "Texte page 2\n", "paragraph", 0, 0);
        insert_tag(&ex, 1, "rust");

        let html_path = dir.path().join("export.html");
        let result = ex.export_html(&html_path).unwrap();
        assert_eq!(result.pages_exported, 2);
        assert_eq!(result.blocks_exported, 7);

        let html = fs::read_to_string(&html_path).unwrap();
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("<meta charset=\"utf-8\">"));
        assert!(html.contains("<title>Note Craft Export</title>"));
        assert_eq!(html.matches("<section").count(), 2);
        assert!(html.contains("<h1>Titre &amp; &lt;b&gt;</h1>"));
        assert!(html.contains("<h2>Sous-titre</h2>"));
        assert!(html.contains("&lt;script&gt;alert(&quot;&amp;&quot;);&lt;/script&gt;"));
        assert!(!html.contains("<script>alert"));
        assert!(html.contains("<li>a</li>"));
        assert!(html.contains("<li>b</li>"));
        assert!(html.contains("<pre><code>fn f() {}</code></pre>"));
        assert!(html.contains("<blockquote>citation</blockquote>"));
        assert!(html.contains("<hr>"));
    }

    #[test]
    fn test_export_auto_detects_format_by_extension() {
        let dir = tempdir().unwrap();
        let ex = make_exporter(&dir);

        insert_page(&ex, 1, "Auto");
        insert_block(&ex, 1, 1, "Contenu\n", "paragraph", 0, 0);

        let json_path = dir.path().join("auto.json");
        let result = ex.export_auto(&json_path).unwrap();
        assert_eq!(result.pages_exported, 1);
        let value: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&json_path).unwrap()).unwrap();
        assert!(value.get("pages").is_some());

        let md_path = dir.path().join("auto.md");
        ex.export_auto(&md_path).unwrap();
        assert!(fs::read_to_string(&md_path).unwrap().contains("Contenu"));

        let html_path = dir.path().join("auto.html");
        ex.export_auto(&html_path).unwrap();
        assert!(fs::read_to_string(&html_path).unwrap().contains("<section"));
    }

    #[test]
    fn test_export_auto_directory() {
        let dir = tempdir().unwrap();
        let ex = make_exporter(&dir);

        insert_page(&ex, 1, "Page Un");
        insert_page(&ex, 2, "Page Deux");
        insert_block(&ex, 1, 1, "A\n", "paragraph", 0, 0);
        insert_block(&ex, 2, 2, "B\n", "paragraph", 0, 0);

        let out_dir = dir.path().join("auto_out");
        fs::create_dir(&out_dir).unwrap();
        let result = ex.export_auto(&out_dir).unwrap();
        assert_eq!(result.pages_exported, 2);

        let md_count = fs::read_dir(&out_dir)
            .unwrap()
            .filter(|e| {
                let p = e.as_ref().unwrap().path();
                p.extension().and_then(|x| x.to_str()) == Some("md")
            })
            .count();
        assert_eq!(md_count, 2);
    }

    #[test]
    fn test_export_auto_unknown_extension() {
        let dir = tempdir().unwrap();
        let ex = make_exporter(&dir);

        let txt_path = dir.path().join("fichier.txt");
        let err = ex.export_auto(&txt_path).unwrap_err();
        assert!(matches!(err, ExportError::UnsupportedFormat(ext) if ext == "txt"));
    }

    #[test]
    fn test_export_json_io_error_nonexistent_parent() {
        let dir = tempdir().unwrap();
        let ex = make_exporter(&dir);

        let bad_path = dir.path().join("inexistant").join("x.json");
        let err = ex.export_json(&bad_path).unwrap_err();
        assert!(matches!(err, ExportError::IoError(_)));
    }

    #[test]
    fn test_export_excludes_deleted() {
        let dir = tempdir().unwrap();
        let ex = make_exporter(&dir);

        insert_page(&ex, 1, "Alpha");
        insert_page(&ex, 2, "Beta");
        insert_block(&ex, 1, 1, "A1\n", "paragraph", 0, 0);
        insert_block(&ex, 2, 1, "A2\n", "paragraph", 1, 0);
        insert_block(&ex, 3, 2, "B1\n", "paragraph", 0, 0);
        insert_block(&ex, 4, 2, "B2\n", "paragraph", 1, 0);
        insert_property(&ex, 1, 0, "k", "text", Some("v"), None, None);
        insert_property(&ex, 2, 0, "k", "text", Some("v"), None, None);
        insert_property(&ex, 4, 1, "k", "text", Some("v"), None, None);
        insert_link(&ex, 1, 1, 1, None);
        insert_link(&ex, 2, 3, 2, None);

        // Marquage de suppression logique (pages et blocs)
        ex.conn
            .execute("UPDATE pages SET is_deleted = TRUE WHERE id = 2", [])
            .unwrap();
        ex.conn
            .execute("UPDATE blocks SET is_deleted = TRUE WHERE id = 2", [])
            .unwrap();

        let json_path = dir.path().join("deleted.json");
        let result = ex.export_json(&json_path).unwrap();
        assert_eq!(result.pages_exported, 1);
        assert_eq!(result.blocks_exported, 1);
        assert_eq!(result.links_exported, 1);
        assert_eq!(result.properties_exported, 1);

        let value: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&json_path).unwrap()).unwrap();
        let pages = value["pages"].as_array().unwrap();
        assert_eq!(pages.len(), 1);
        assert_eq!(pages[0]["title"], serde_json::json!("Alpha"));
        assert_eq!(pages[0]["blocks"].as_array().unwrap().len(), 1);
    }

    #[test]
    fn test_export_json_empty_database() {
        let dir = tempdir().unwrap();
        let ex = make_exporter(&dir);

        let json_path = dir.path().join("empty.json");
        let result = ex.export_json(&json_path).unwrap();
        assert_eq!(result.pages_exported, 0);
        assert_eq!(result.blocks_exported, 0);
        assert_eq!(result.links_exported, 0);
        assert_eq!(result.properties_exported, 0);
        assert_eq!(result.tags_exported, 0);
        assert!(result.warnings.is_empty());
        assert!(result.errors.is_empty());

        let value: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&json_path).unwrap()).unwrap();
        assert!(value["pages"].as_array().unwrap().is_empty());
    }

    #[test]
    fn test_export_markdown_file_multi_page_warning() {
        let dir = tempdir().unwrap();
        let ex = make_exporter(&dir);

        insert_page(&ex, 1, "Un");
        insert_page(&ex, 2, "Deux");
        insert_block(&ex, 1, 1, "A\n", "paragraph", 0, 0);
        insert_block(&ex, 2, 2, "B\n", "paragraph", 0, 0);

        let md_path = dir.path().join("multi.md");
        let result = ex.export_markdown_file(&md_path).unwrap();
        assert_eq!(result.pages_exported, 2);
        assert!(!result.warnings.is_empty());
    }

    #[test]
    fn test_export_markdown_frontmatter_roundtrip() {
        let dir = tempdir().unwrap();
        let ex = make_exporter(&dir);

        insert_page(&ex, 1, "Props");
        insert_property(&ex, 1, 0, "status", "text", Some("active"), None, None);
        insert_block(&ex, 1, 1, "Corps\n", "paragraph", 0, 0);

        let md_path = dir.path().join("Props.md");
        let export_result = ex.export_markdown_file(&md_path).unwrap();
        assert_eq!(export_result.pages_exported, 1);
        assert_eq!(export_result.properties_exported, 1);

        // Le frontmatter (propriétés de page) est bien émis
        let content = fs::read_to_string(&md_path).unwrap();
        assert!(content.starts_with("---\n"));
        assert!(content.contains("status: \"active\""));

        // Les propriétés passent le roundtrip via le frontmatter
        let dir2 = tempdir().unwrap();
        let db2_path = dir2.path().join("props.db");
        let conn2 = schema::Schema::initialize_database(&db2_path).unwrap();
        let mut importer = DataImporter::new(conn2);
        let import_result = importer.import_markdown_file(&md_path).unwrap();
        assert_eq!(
            import_result.properties_imported,
            export_result.properties_exported
        );

        // NB (dette préexistante de l'importeur, hors périmètre #102) : le
        // parseur maison reçoit le fichier ENTIER, frontmatter inclus
        // (importer.rs:542), donc les lignes du frontmatter ressortent aussi
        // comme un bloc paragraphe supplémentaire côté import.
        assert_eq!(
            import_result.blocks_imported,
            export_result.blocks_exported + 1
        );
    }
}
