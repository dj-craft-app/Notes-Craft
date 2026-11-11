//! Advanced Paste Module - Gestion du collage avancé
//!
//! Ce module fournit:
//! - Collage d'images depuis le clipboard
//! - Collage de texte avec formatage Markdown
//! - Collage de listes (numérotées et à puces)
//! - Collage de code avec syntax highlighting
//! - Détection automatique du type de contenu

use std::path::Path;

/// Erreurs possibles lors du collage
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum PasteError {
    #[error("Clipboard empty")]
    EmptyClipboard,
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),
    #[error("Image save error: {0}")]
    ImageSaveError(String),
    #[error("Invalid image data")]
    InvalidImageData,
}

pub type PasteResult<T> = Result<T, PasteError>;

/// Type de contenu collé
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PasteType {
    /// Texte brut
    PlainText,
    /// Texte avec formatage Markdown
    FormattedText,
    /// Image (chemin ou binaire)
    Image,
    /// Liste à puces
    BulletList,
    /// Liste numérotée
    NumberedList,
    /// Bloc de code
    CodeBlock,
    /// Table
    Table,
    /// Inconnu
    Unknown,
}

/// Contenu collé avec métadonnées
#[derive(Debug, Clone)]
pub struct PasteContent {
    /// Type de contenu
    pub content_type: PasteType,
    /// Contenu brut
    pub raw_content: String,
    /// Contenu formaté (Markdown)
    pub formatted_content: String,
    /// Langage pour les blocs de code
    pub language: Option<String>,
    /// Nom de fichier pour les images
    pub filename: Option<String>,
    /// Largeur de l'image (si applicable)
    pub width: Option<u32>,
    /// Hauteur de l'image (si applicable)
    pub height: Option<u32>,
}

impl PasteContent {
    pub fn new(content_type: PasteType, raw_content: String) -> Self {
        Self {
            content_type,
            raw_content: raw_content.clone(),
            formatted_content: raw_content,
            language: None,
            filename: None,
            width: None,
            height: None,
        }
    }

    pub fn with_formatted(mut self, formatted: String) -> Self {
        self.formatted_content = formatted;
        self
    }

    pub fn with_language(mut self, lang: String) -> Self {
        self.language = Some(lang);
        self
    }

    pub fn with_image_info(mut self, filename: String, width: u32, height: u32) -> Self {
        self.filename = Some(filename);
        self.width = Some(width);
        self.height = Some(height);
        self
    }
}

/// Détecte le type de contenu à partir du texte brut
pub fn detect_paste_type(content: &str) -> PasteType {
    if content.is_empty() {
        return PasteType::PlainText;
    }

    // Vérifier si c'est du code (indenté ou bloc de code)
    if is_code_block(content) {
        return PasteType::CodeBlock;
    }

    // Vérifier si c'est une liste numérotée
    if is_numbered_list(content) {
        return PasteType::NumberedList;
    }

    // Vérifier si c'est une liste à puces
    if is_bullet_list(content) {
        return PasteType::BulletList;
    }

    // Vérifier si c'est une table
    if is_table(content) {
        return PasteType::Table;
    }

    // Vérifier si c'est du Markdown formaté
    if is_formatted_text(content) {
        return PasteType::FormattedText;
    }

    PasteType::PlainText
}

/// Vérifie si le contenu est un bloc de code
fn is_code_block(content: &str) -> bool {
    // Bloc de code Markdown avec triple backticks
    if content.starts_with("```") {
        return true;
    }

    // Indentation de 4 espaces sur la plupart des lignes
    let lines: Vec<&str> = content.lines().collect();
    if lines.is_empty() {
        return false;
    }

    // Compter les lignes non vides qui sont indentées de 4 espaces
    let non_empty_lines: Vec<&str> = lines
        .iter()
        .filter(|line| !line.trim().is_empty())
        .copied()
        .collect();

    if non_empty_lines.is_empty() {
        return false;
    }

    // Compter les lignes indentées
    let indented_count = non_empty_lines
        .iter()
        .filter(|line| line.starts_with("    "))
        .count();

    // Si au moins 50% des lignes non vides sont indentées de 4 espaces,
    // on considère comme code (pour être tolérant)
    // Ou si au moins 2 lignes sont indentées
    indented_count > 0 && (indented_count * 2 >= non_empty_lines.len() || indented_count >= 2)
}

/// Vérifie si le contenu est une liste numérotée
fn is_numbered_list(content: &str) -> bool {
    let lines: Vec<&str> = content.lines().filter(|l| !l.trim().is_empty()).collect();
    if lines.is_empty() {
        return false;
    }

    let mut first_number = None;
    let mut count = 0;

    for line in &lines {
        let trimmed = line.trim_start();
        // Vérifier si la ligne commence par un nombre suivi d'un point
        if let Some(pos) = trimmed.find('.') {
            let number_part = &trimmed[..pos];
            if let Ok(num) = number_part.parse::<u32>() {
                if first_number.is_none() {
                    first_number = Some(num);
                }
                // Vérifier que les nombres sont consécutifs
                if first_number.unwrap() + count == num {
                    count += 1;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

    // Au moins 2 éléments pour être une liste
    count >= 2
}

/// Vérifie si le contenu est une liste à puces
fn is_bullet_list(content: &str) -> bool {
    let lines: Vec<&str> = content.lines().filter(|l| !l.trim().is_empty()).collect();
    if lines.is_empty() {
        return false;
    }

    lines.iter().all(|line| {
        let trimmed = line.trim_start();
        trimmed.starts_with("-") || trimmed.starts_with("*") || trimmed.starts_with("+")
    }) && lines.len() >= 2
}

/// Vérifie si le contenu est une table
fn is_table(content: &str) -> bool {
    let lines: Vec<&str> = content.lines().filter(|l| !l.trim().is_empty()).collect();
    if lines.is_empty() {
        return false;
    }

    // Une table Markdown a au moins une ligne avec |
    let has_pipes = lines.iter().any(|line| line.contains('|'));
    if !has_pipes {
        return false;
    }

    // Vérifier la présence d'une ligne de séparation
    let has_separator = lines.iter().any(|line| {
        let trimmed = line.trim();
        trimmed.starts_with('|')
            && trimmed.ends_with('|')
            && trimmed.chars().filter(|&c| c == '-').count() > 0
    });

    has_separator
}

/// Vérifie si le contenu est du texte formaté
fn is_formatted_text(content: &str) -> bool {
    content.contains("**")
        || content.contains("*")
        || content.contains("__")
        || content.contains("_")
        || content.contains("#")
        || content.contains("[") && content.contains("]")
        || content.contains("<") && content.contains(">")
}

/// Structure pour gérer le collage avancé
pub struct PasteManager {
    /// Chemin de stockage des images
    image_storage_path: String,
    /// Préfixe pour les noms de fichiers images
    image_prefix: String,
    /// Compteur pour les noms uniques
    image_counter: u64,
}

impl PasteManager {
    pub fn new(storage_path: &str) -> Self {
        Self {
            image_storage_path: storage_path.to_string(),
            image_prefix: "paste".to_string(),
            image_counter: 0,
        }
    }

    pub fn with_prefix(mut self, prefix: &str) -> Self {
        self.image_prefix = prefix.to_string();
        self
    }

    /// Colle du contenu depuis le clipboard
    pub fn paste(&mut self, clipboard_content: &str) -> PasteResult<PasteContent> {
        if clipboard_content.is_empty() {
            return Err(PasteError::EmptyClipboard);
        }

        let paste_type = detect_paste_type(clipboard_content);

        match paste_type {
            PasteType::Image => self.handle_image_paste(clipboard_content),
            PasteType::CodeBlock => self.handle_code_paste(clipboard_content),
            PasteType::BulletList => self.handle_bullet_list_paste(clipboard_content),
            PasteType::NumberedList => self.handle_numbered_list_paste(clipboard_content),
            PasteType::Table => self.handle_table_paste(clipboard_content),
            PasteType::FormattedText => self.handle_formatted_text_paste(clipboard_content),
            PasteType::PlainText => Ok(PasteContent::new(
                PasteType::PlainText,
                clipboard_content.to_string(),
            )),
            PasteType::Unknown => Ok(PasteContent::new(
                PasteType::PlainText,
                clipboard_content.to_string(),
            )),
        }
    }

    /// Gère le collage d'images
    fn handle_image_paste(&mut self, content: &str) -> PasteResult<PasteContent> {
        // Si content est un chemin de fichier
        if Path::new(content).exists() {
            let filename = Path::new(content)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("paste_image")
                .to_string();

            // Générer un nom unique
            self.image_counter += 1;
            let new_filename = format!("{}_{}_{}", self.image_prefix, self.image_counter, filename);

            // Copier le fichier vers le stockage
            // (En pratique, on utiliserait std::fs::copy ou une fonction dédiée)
            // Pour l'instant, on génère juste le markdown

            let markdown = format!(
                "![{}]({}/{})",
                filename, self.image_storage_path, new_filename
            );

            return Ok(PasteContent::new(PasteType::Image, content.to_string())
                .with_formatted(markdown)
                .with_image_info(new_filename, 0, 0));
        }

        // Si content est du data: URI
        if content.starts_with("data:image/") {
            // Extraire le type et les données
            if let Some(_comma_pos) = content.find(',') {
                // TODO: Décoder le base64 et sauvegarder
                // Pour l'instant, on retourne une erreur
                return Err(PasteError::ImageSaveError(
                    "Base64 image decoding not yet implemented".to_string(),
                ));
            }
        }

        // Contenu non reconnu comme image
        Err(PasteError::UnsupportedFormat(
            "Image paste requires file path or data URI".to_string(),
        ))
    }

    /// Gère le collage de blocs de code
    fn handle_code_paste(&mut self, content: &str) -> PasteResult<PasteContent> {
        // Détecter le langage si présent
        let (language, code_content) = if let Some(stripped) = content.strip_prefix("```") {
            if let Some(end_pos) = stripped.find("\n") {
                let lang_line = &stripped[..end_pos];
                let lang = if lang_line.trim().is_empty() {
                    None
                } else {
                    Some(lang_line.trim().to_string())
                };
                let code = &stripped[end_pos + 1..];
                // Enlever les backticks finaux
                let code = if let Some(last_pos) = code.rfind("```") {
                    &code[..last_pos]
                } else {
                    code
                };
                (lang, code.trim().to_string())
            } else {
                (None, stripped.trim().to_string())
            }
        } else {
            // Code indenté
            let lines: Vec<&str> = content.lines().collect();
            let code: String = lines
                .iter()
                .map(|line| {
                    if let Some(stripped_line) = line.strip_prefix("    ") {
                        stripped_line
                    } else {
                        line
                    }
                })
                .collect::<Vec<&str>>()
                .join("\n");
            (None, code)
        };

        let markdown = if let Some(lang) = &language {
            format!("```{}\n{}\n```", lang, code_content)
        } else {
            format!("```\n{}\n```", code_content)
        };

        let mut result =
            PasteContent::new(PasteType::CodeBlock, content.to_string()).with_formatted(markdown);

        if let Some(lang) = language {
            result = result.with_language(lang);
        }

        Ok(result)
    }

    /// Gère le collage de listes à puces
    fn handle_bullet_list_paste(&mut self, content: &str) -> PasteResult<PasteContent> {
        // Vérifier si le contenu est déjà bien formaté
        let lines: Vec<&str> = content.lines().collect();

        // Normaliser les puces vers "-"
        let normalized: String = lines
            .iter()
            .map(|line| {
                let trimmed = line.trim_start();
                if trimmed.starts_with("-") || trimmed.starts_with("*") || trimmed.starts_with("+")
                {
                    // Conserver l'indentation mais normaliser la puce
                    let indent = line.len() - trimmed.len();
                    format!("{}- {}", " ".repeat(indent), trimmed[1..].trim_start())
                } else if !trimmed.is_empty() {
                    // Ligne sans puce -> ajouter une puce
                    format!("- {}", trimmed)
                } else {
                    line.to_string()
                }
            })
            .collect::<Vec<String>>()
            .join("\n");

        Ok(
            PasteContent::new(PasteType::BulletList, content.to_string())
                .with_formatted(normalized),
        )
    }

    /// Gère le collage de listes numérotées
    fn handle_numbered_list_paste(&mut self, content: &str) -> PasteResult<PasteContent> {
        let lines: Vec<&str> = content.lines().collect();

        // Normaliser la numérotation à partir de 1
        let normalized: String = lines
            .iter()
            .map(|line| {
                let trimmed = line.trim_start();
                if let Some(pos) = trimmed.find('.') {
                    let content_part = &trimmed[pos + 1..].trim_start();
                    format!("1. {}", content_part)
                } else if !trimmed.is_empty() {
                    format!("1. {}", trimmed)
                } else {
                    line.to_string()
                }
            })
            .collect::<Vec<String>>()
            .join("\n");

        Ok(
            PasteContent::new(PasteType::NumberedList, content.to_string())
                .with_formatted(normalized),
        )
    }

    /// Gère le collage de tables
    fn handle_table_paste(&mut self, content: &str) -> PasteResult<PasteContent> {
        // Pour l'instant, on retourne le contenu tel quel
        // L'amélioration future pourrait inclure la normalisation
        Ok(PasteContent::new(PasteType::Table, content.to_string()))
    }

    /// Gère le collage de texte formaté
    fn handle_formatted_text_paste(&mut self, content: &str) -> PasteResult<PasteContent> {
        // Nettoyer les espaces superflus
        let cleaned = content.split_whitespace().collect::<Vec<&str>>().join(" ");

        Ok(
            PasteContent::new(PasteType::FormattedText, content.to_string())
                .with_formatted(cleaned),
        )
    }
}

/// Utilitaires pour le collage depuis HTML
pub mod html_paste {
    use super::{PasteContent, PasteType};

    /// Convertit du HTML vers Markdown pour le collage
    pub fn html_to_markdown(html: &str) -> PasteContent {
        // Implémentation améliorée de conversion HTML -> Markdown
        let mut markdown = html.to_string();

        // Nettoyer d'abord les balises HTML en les remplaçant par leurs équivalents Markdown
        // Gras et fort
        markdown = markdown
            .replace("<strong>", "**")
            .replace("</strong>", "**")
            .replace("<b>", "**")
            .replace("</b>", "**")
            // Italique et emphase
            .replace("<em>", "*")
            .replace("</em>", "*")
            .replace("<i>", "*")
            .replace("</i>", "*")
            // Code inline
            .replace("<code>", "`")
            .replace("</code>", "`")
            // Bloc de code
            .replace("<pre>", "\n\n```\n")
            .replace("</pre>", "\n```\n")
            // Paragraphes
            .replace("<p>", "\n\n")
            .replace("</p>", "\n")
            // Sauts de ligne
            .replace("<br>", "\n")
            .replace("<br/>", "\n")
            .replace("<br />", "\n")
            // Titres
            .replace("<h1>", "\n\n# ")
            .replace("</h1>", "\n")
            .replace("<h2>", "\n\n## ")
            .replace("</h2>", "\n")
            .replace("<h3>", "\n\n### ")
            .replace("</h3>", "\n")
            .replace("<h4>", "\n\n#### ")
            .replace("</h4>", "\n")
            .replace("<h5>", "\n\n##### ")
            .replace("</h5>", "\n")
            .replace("<h6>", "\n\n###### ")
            .replace("</h6>", "\n")
            // Listes
            .replace("<ul>", "\n")
            .replace("</ul>", "\n")
            .replace("<ol>", "\n")
            .replace("</ol>", "\n")
            .replace("<li>", "\n- ")
            .replace("</li>", "")
            // Liens
            .replace("<a href=\"", "[")
            .replace("\">", "](");

        // Supprimer les balises restantes
        markdown = remove_html_tags(&markdown);

        // Nettoyer les espaces multiples
        markdown = markdown.split_whitespace().collect::<Vec<&str>>().join(" ");

        // Nettoyer les multiples sauts de ligne
        while markdown.contains("\n\n\n") {
            markdown = markdown.replace("\n\n\n", "\n\n");
        }

        // Nettoyer les espaces avant les sauts de ligne
        markdown = markdown.replace(" \n", "\n");

        PasteContent::new(PasteType::FormattedText, html.to_string()).with_formatted(markdown)
    }

    /// Supprime les balises HTML restantes
    fn remove_html_tags(text: &str) -> String {
        let mut result = String::new();
        let mut in_tag = false;

        for c in text.chars() {
            match c {
                '<' => {
                    in_tag = true;
                }
                '>' => {
                    in_tag = false;
                }
                _ if !in_tag => {
                    result.push(c);
                }
                _ => {}
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_plain_text() {
        let text = "Simple text content";
        assert_eq!(detect_paste_type(text), PasteType::PlainText);
    }

    #[test]
    fn test_detect_code_block_with_backticks() {
        let code = "```rust\nfn main() {}\n```";
        assert_eq!(detect_paste_type(code), PasteType::CodeBlock);
    }

    #[test]
    fn test_detect_indented_code() {
        let code = "    fn main() {\n    println!(\"Hello\");\n}";
        assert_eq!(detect_paste_type(code), PasteType::CodeBlock);
    }

    #[test]
    fn test_detect_bullet_list() {
        let list = "- Item 1\n- Item 2\n- Item 3";
        assert_eq!(detect_paste_type(list), PasteType::BulletList);
    }

    #[test]
    fn test_detect_bullet_list_with_stars() {
        let list = "* Item 1\n* Item 2\n* Item 3";
        assert_eq!(detect_paste_type(list), PasteType::BulletList);
    }

    #[test]
    fn test_detect_numbered_list() {
        let list = "1. Item 1\n2. Item 2\n3. Item 3";
        assert_eq!(detect_paste_type(list), PasteType::NumberedList);
    }

    #[test]
    fn test_detect_table() {
        let table = "| Header 1 | Header 2 |\n|----------|----------|\n| Cell 1 | Cell 2 |";
        assert_eq!(detect_paste_type(table), PasteType::Table);
    }

    #[test]
    fn test_detect_formatted_text() {
        let text = "**Bold** and *italic* text";
        assert_eq!(detect_paste_type(text), PasteType::FormattedText);
    }

    #[test]
    fn test_paste_manager_code_block() {
        let mut manager = PasteManager::new("/storage");
        let code = "```rust\nfn main() {}\n```";
        let result = manager.paste(code).unwrap();

        assert_eq!(result.content_type, PasteType::CodeBlock);
        assert!(result.formatted_content.contains("```rust"));
        assert_eq!(result.language, Some("rust".to_string()));
    }

    #[test]
    fn test_paste_manager_bullet_list() {
        let mut manager = PasteManager::new("/storage");
        let list = "* Item 1\n* Item 2\n* Item 3";
        let result = manager.paste(list).unwrap();

        assert_eq!(result.content_type, PasteType::BulletList);
        assert!(result.formatted_content.contains("- Item"));
    }

    #[test]
    fn test_paste_manager_numbered_list() {
        let mut manager = PasteManager::new("/storage");
        let list = "3. Item 1\n4. Item 2\n5. Item 3";
        let result = manager.paste(list).unwrap();

        assert_eq!(result.content_type, PasteType::NumberedList);
        assert!(result.formatted_content.starts_with("1. Item"));
    }

    #[test]
    fn test_paste_manager_plain_text() {
        let mut manager = PasteManager::new("/storage");
        let text = "Simple text";
        let result = manager.paste(text).unwrap();

        assert_eq!(result.content_type, PasteType::PlainText);
        assert_eq!(result.raw_content, text);
        assert_eq!(result.formatted_content, text);
    }

    #[test]
    fn test_paste_error_empty_clipboard() {
        let mut manager = PasteManager::new("/storage");
        let result = manager.paste("");
        assert!(matches!(result, Err(PasteError::EmptyClipboard)));
    }

    #[test]
    fn test_html_to_markdown() {
        let html = "<p>Hello <b>World</b></p>";
        let result = html_paste::html_to_markdown(html);
        assert_eq!(result.content_type, PasteType::FormattedText);
        assert!(result.formatted_content.contains("**World**"));
    }

    #[test]
    fn test_paste_content_builder() {
        let content = PasteContent::new(PasteType::PlainText, "test".to_string())
            .with_formatted("formatted".to_string())
            .with_language("rust".to_string());

        assert_eq!(content.content_type, PasteType::PlainText);
        assert_eq!(content.raw_content, "test");
        assert_eq!(content.formatted_content, "formatted");
        assert_eq!(content.language, Some("rust".to_string()));
    }

    #[test]
    fn test_detect_single_line_not_list() {
        // Une seule ligne avec une puce n'est pas considérée comme une liste
        let text = "- Single item";
        assert_eq!(detect_paste_type(text), PasteType::PlainText);
    }

    #[test]
    fn test_detect_mixed_bullets() {
        let list = "- Item 1\n* Item 2\n+ Item 3";
        assert_eq!(detect_paste_type(list), PasteType::BulletList);
    }
}
