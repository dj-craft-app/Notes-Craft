//! Quick Add Module - Ajout rapide de contenu
//!
//! Ce module fournit:
//! - Ajout rapide de blocs avec raccourcis
//! - Ajout rapide de pages
//! - Ajout rapide avec templates pré-définis
//! - Gestion des templates personnalisés

use std::collections::HashMap;

use crate::markdown::MarkdownEditor;

/// Erreurs possibles lors de l'ajout rapide
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum QuickAddError {
    #[error("Template '{0}' not found")]
    TemplateNotFound(String),
    #[error("Invalid block type: {0}")]
    InvalidBlockType(String),
    #[error("Empty content not allowed")]
    EmptyContent,
    #[error("Invalid template name: {0}")]
    InvalidTemplateName(String),
}

pub type QuickAddResult<T> = Result<T, QuickAddError>;

/// Type de bloc pour l'ajout rapide
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QuickBlockType {
    /// Paragraphe
    Paragraph,
    /// Titre niveau 1
    Heading1,
    /// Titre niveau 2
    Heading2,
    /// Titre niveau 3
    Heading3,
    /// Liste à puces
    BulletList,
    /// Liste numérotée
    NumberedList,
    /// Bloc de code
    CodeBlock,
    /// Citation
    Blockquote,
    /// Ligne horizontale
    HorizontalRule,
    /// Table
    Table,
    /// Image
    Image,
    /// Lien
    Link,
}

impl std::str::FromStr for QuickBlockType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "p" | "paragraph" => Ok(QuickBlockType::Paragraph),
            "h1" | "heading1" | "title" => Ok(QuickBlockType::Heading1),
            "h2" | "heading2" | "subtitle" => Ok(QuickBlockType::Heading2),
            "h3" | "heading3" => Ok(QuickBlockType::Heading3),
            "ul" | "list" | "bullet" | "bullets" => Ok(QuickBlockType::BulletList),
            "ol" | "numbered" | "numberedlist" => Ok(QuickBlockType::NumberedList),
            "code" | "codeblock" => Ok(QuickBlockType::CodeBlock),
            "quote" | "blockquote" => Ok(QuickBlockType::Blockquote),
            "hr" | "rule" | "horizontal" | "horizontalrule" => Ok(QuickBlockType::HorizontalRule),
            "table" => Ok(QuickBlockType::Table),
            "img" | "image" => Ok(QuickBlockType::Image),
            "link" | "url" | "a" => Ok(QuickBlockType::Link),
            _ => Err(()),
        }
    }
}

impl QuickBlockType {
    /// Retourne le préfixe Markdown pour ce type de bloc
    pub fn markdown_prefix(&self) -> &'static str {
        match self {
            QuickBlockType::Paragraph => "",
            QuickBlockType::Heading1 => "# ",
            QuickBlockType::Heading2 => "## ",
            QuickBlockType::Heading3 => "### ",
            QuickBlockType::BulletList => "- ",
            QuickBlockType::NumberedList => "1. ",
            QuickBlockType::CodeBlock => "```\n",
            QuickBlockType::Blockquote => "> ",
            QuickBlockType::HorizontalRule => "---",
            QuickBlockType::Table => "| ",
            QuickBlockType::Image => "![",
            QuickBlockType::Link => "[",
        }
    }

    /// Retourne le suffixe Markdown pour ce type de bloc
    pub fn markdown_suffix(&self) -> &'static str {
        match self {
            QuickBlockType::CodeBlock => "\n```",
            QuickBlockType::Image | QuickBlockType::Link => ")",
            _ => "",
        }
    }
}

/// Template pour l'ajout rapide
#[derive(Debug, Clone)]
pub struct QuickTemplate {
    /// Nom du template
    pub name: String,
    /// Contenu du template (Markdown)
    pub content: String,
    /// Description du template
    pub description: Option<String>,
    /// Catégorie du template
    pub category: Option<String>,
}

impl QuickTemplate {
    pub fn new(name: &str, content: &str) -> Self {
        Self {
            name: name.to_string(),
            content: content.to_string(),
            description: None,
            category: None,
        }
    }

    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = Some(desc.to_string());
        self
    }

    pub fn with_category(mut self, category: &str) -> Self {
        self.category = Some(category.to_string());
        self
    }

    /// Applique le template avec des substitutions de variables
    pub fn apply(&self, substitutions: &HashMap<String, String>) -> String {
        let mut result = self.content.clone();

        for (key, value) in substitutions {
            let placeholder = format!("${}", key);
            result = result.replace(&placeholder, value);
        }

        result
    }
}

/// Gestionnaire de l'ajout rapide
#[derive(Default)]
pub struct QuickAddManager {
    /// Templates personnalisés
    templates: HashMap<String, QuickTemplate>,
    /// Raccourcis pour les types de blocs
    block_shortcuts: HashMap<String, QuickBlockType>,
    /// Raccourcis pour les templates
    template_shortcuts: HashMap<String, String>,
}

impl QuickAddManager {
    pub fn new() -> Self {
        Self {
            templates: HashMap::new(),
            block_shortcuts: Self::create_default_shortcuts(),
            template_shortcuts: HashMap::new(),
        }
    }

    /// Crée les raccourcis par défaut pour les types de blocs
    fn create_default_shortcuts() -> HashMap<String, QuickBlockType> {
        let mut shortcuts = HashMap::new();

        // Raccourcis simples
        shortcuts.insert("p".to_string(), QuickBlockType::Paragraph);
        shortcuts.insert("h1".to_string(), QuickBlockType::Heading1);
        shortcuts.insert("h2".to_string(), QuickBlockType::Heading2);
        shortcuts.insert("h3".to_string(), QuickBlockType::Heading3);
        shortcuts.insert("ul".to_string(), QuickBlockType::BulletList);
        shortcuts.insert("ol".to_string(), QuickBlockType::NumberedList);
        shortcuts.insert("code".to_string(), QuickBlockType::CodeBlock);
        shortcuts.insert("quote".to_string(), QuickBlockType::Blockquote);
        shortcuts.insert("hr".to_string(), QuickBlockType::HorizontalRule);
        shortcuts.insert("table".to_string(), QuickBlockType::Table);
        shortcuts.insert("img".to_string(), QuickBlockType::Image);
        shortcuts.insert("link".to_string(), QuickBlockType::Link);

        shortcuts
    }

    /// Ajoute un nouveau template
    pub fn add_template(&mut self, template: QuickTemplate) -> QuickAddResult<()> {
        if !is_valid_template_name(&template.name) {
            return Err(QuickAddError::InvalidTemplateName(template.name));
        }
        self.templates.insert(template.name.clone(), template);
        Ok(())
    }

    /// Supprime un template
    pub fn remove_template(&mut self, name: &str) -> QuickAddResult<()> {
        if self.templates.remove(name).is_none() {
            return Err(QuickAddError::TemplateNotFound(name.to_string()));
        }
        Ok(())
    }

    /// Récupère un template par son nom
    pub fn get_template(&self, name: &str) -> Option<&QuickTemplate> {
        self.templates.get(name)
    }

    /// Liste tous les templates
    pub fn list_templates(&self) -> Vec<&QuickTemplate> {
        self.templates.values().collect()
    }

    /// Ajoute un raccourci pour un type de bloc
    pub fn add_block_shortcut(&mut self, shortcut: &str, block_type: QuickBlockType) {
        self.block_shortcuts
            .insert(shortcut.to_string(), block_type);
    }

    /// Ajoute un raccourci pour un template
    pub fn add_template_shortcut(
        &mut self,
        shortcut: &str,
        template_name: &str,
    ) -> QuickAddResult<()> {
        if self.templates.contains_key(template_name) {
            self.template_shortcuts
                .insert(shortcut.to_string(), template_name.to_string());
            Ok(())
        } else {
            Err(QuickAddError::TemplateNotFound(template_name.to_string()))
        }
    }

    /// Ajoute un bloc rapidement
    pub fn add_block(&self, block_type: QuickBlockType, content: &str) -> QuickAddResult<String> {
        if content.is_empty() {
            return Err(QuickAddError::EmptyContent);
        }

        let markdown = match block_type {
            QuickBlockType::Paragraph => content.to_string(),
            QuickBlockType::Heading1 => format!("# {}", content),
            QuickBlockType::Heading2 => format!("## {}", content),
            QuickBlockType::Heading3 => format!("### {}", content),
            QuickBlockType::BulletList => content
                .lines()
                .map(|line| {
                    if line.starts_with('-') || line.starts_with('*') || line.starts_with('+') {
                        line.to_string()
                    } else {
                        format!("- {}", line)
                    }
                })
                .collect::<Vec<String>>()
                .join("\n"),
            QuickBlockType::NumberedList => {
                content
                    .lines()
                    .map(|line| {
                        let trimmed = line.trim_start();
                        if trimmed.parse::<u32>().is_ok() && trimmed.contains('.') {
                            // Déjà numéroté, normaliser
                            let pos = trimmed.find('.').unwrap_or(0);
                            format!("1. {}", trimmed[pos + 1..].trim_start())
                        } else {
                            format!("1. {}", line)
                        }
                    })
                    .collect::<Vec<String>>()
                    .join("\n")
            }
            QuickBlockType::CodeBlock => format!("```\n{}\n```", content),
            QuickBlockType::Blockquote => content
                .lines()
                .map(|line| format!("> {}", line))
                .collect::<Vec<String>>()
                .join("\n"),
            QuickBlockType::HorizontalRule => "---".to_string(),
            QuickBlockType::Table => {
                // Format basique de table
                format!("| {} |\n|-----|", content)
            }
            QuickBlockType::Image => {
                // Format: ![alt](url) - on sépare par espace ou tab
                let parts: Vec<&str> = content.split_whitespace().collect();
                if parts.len() >= 2 {
                    format!("![{}]({})", parts[0], parts[1])
                } else {
                    format!("![{}]({})", parts[0], parts[0])
                }
            }
            QuickBlockType::Link => {
                // Format: [text](url)
                let parts: Vec<&str> = content.split_whitespace().collect();
                if parts.len() >= 2 {
                    format!("[{}]({})", parts[0], parts[1])
                } else {
                    format!("[{}]({})", parts[0], parts[0])
                }
            }
        };

        Ok(markdown)
    }

    /// Ajoute un bloc rapidement avec un raccourci
    pub fn add_block_by_shortcut(&self, shortcut: &str, content: &str) -> QuickAddResult<String> {
        let block_type = self
            .block_shortcuts
            .get(shortcut)
            .ok_or_else(|| QuickAddError::InvalidBlockType(shortcut.to_string()))?;

        self.add_block(block_type.clone(), content)
    }

    /// Ajoute du contenu à partir d'un template
    pub fn add_from_template(
        &self,
        template_name: &str,
        substitutions: &HashMap<String, String>,
    ) -> QuickAddResult<String> {
        let template = self
            .templates
            .get(template_name)
            .ok_or_else(|| QuickAddError::TemplateNotFound(template_name.to_string()))?;

        Ok(template.apply(substitutions))
    }

    /// Ajoute du contenu à partir d'un raccourci de template
    pub fn add_from_template_shortcut(
        &self,
        shortcut: &str,
        substitutions: &HashMap<String, String>,
    ) -> QuickAddResult<String> {
        let template_name = self
            .template_shortcuts
            .get(shortcut)
            .ok_or_else(|| QuickAddError::TemplateNotFound(shortcut.to_string()))?;

        self.add_from_template(template_name, substitutions)
    }

    /// Ajoute plusieurs blocs rapidement
    pub fn add_multiple_blocks(
        &self,
        blocks: &[(QuickBlockType, &str)],
    ) -> QuickAddResult<Vec<String>> {
        let mut results = Vec::new();

        for (block_type, content) in blocks {
            if content.is_empty() {
                return Err(QuickAddError::EmptyContent);
            }
            results.push(self.add_block(block_type.clone(), content)?);
        }

        Ok(results)
    }
}

/// Vérifie si un nom de template est valide
fn is_valid_template_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }
    let chars: Vec<char> = name.chars().collect();
    if !chars[0].is_alphabetic() && chars[0] != '_' {
        return false;
    }
    chars
        .iter()
        .all(|&c| c.is_alphanumeric() || c == '_' || c == '-' || c == '.')
}

/// Crée un gestionnaire QuickAdd avec des templates par défaut
pub fn create_default_quick_add_manager() -> QuickAddManager {
    let mut manager = QuickAddManager::new();

    // Ajouter des templates par défaut
    manager
        .add_template(
            QuickTemplate::new("todo", "- [ ] $task")
                .with_description("Tâche à faire")
                .with_category("productivity"),
        )
        .ok();

    manager
        .add_template(
            QuickTemplate::new("note", "### $title\n\n$content")
                .with_description("Note avec titre")
                .with_category("notes"),
        )
        .ok();

    manager
        .add_template(
            QuickTemplate::new("code-snippet", "```$language\n$code\n```")
                .with_description("Extra de code")
                .with_category("code"),
        )
        .ok();

    manager
        .add_template(
            QuickTemplate::new("quote", "> $text\n> — $author")
                .with_description("Citation")
                .with_category("formatting"),
        )
        .ok();

    manager
        .add_template(
            QuickTemplate::new("link-button", "[$text]($url)")
                .with_description("Bouton lien")
                .with_category("formatting"),
        )
        .ok();

    manager
}

/// Builder pour QuickTemplate
pub struct QuickTemplateBuilder {
    name: String,
    content: String,
    description: Option<String>,
    category: Option<String>,
}

impl QuickTemplateBuilder {
    pub fn new(name: &str, content: &str) -> Self {
        Self {
            name: name.to_string(),
            content: content.to_string(),
            description: None,
            category: None,
        }
    }

    pub fn description(mut self, desc: &str) -> Self {
        self.description = Some(desc.to_string());
        self
    }

    pub fn category(mut self, cat: &str) -> Self {
        self.category = Some(cat.to_string());
        self
    }

    pub fn build(self) -> QuickTemplate {
        QuickTemplate {
            name: self.name,
            content: self.content,
            description: self.description,
            category: self.category,
        }
    }
}

/// Intégration avec MarkdownEditor
impl QuickAddManager {
    /// Ajoute du contenu à un éditeur Markdown existant
    pub fn add_to_editor(&self, editor: &mut MarkdownEditor, content: &str) -> QuickAddResult<()> {
        let current = editor.get_content().to_string();
        let new_content = if current.is_empty() {
            content.to_string()
        } else {
            format!("{}\n\n{}", current, content)
        };
        editor.set_content(new_content);
        Ok(())
    }

    /// Ajoute un bloc à un éditeur Markdown existant
    pub fn add_block_to_editor(
        &self,
        editor: &mut MarkdownEditor,
        block_type: QuickBlockType,
        content: &str,
    ) -> QuickAddResult<()> {
        let markdown = self.add_block(block_type, content)?;
        self.add_to_editor(editor, &markdown)
    }

    /// Ajoute du contenu depuis un template à un éditeur
    pub fn add_template_to_editor(
        &self,
        editor: &mut MarkdownEditor,
        template_name: &str,
        substitutions: &HashMap<String, String>,
    ) -> QuickAddResult<()> {
        let markdown = self.add_from_template(template_name, substitutions)?;
        self.add_to_editor(editor, &markdown)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_type_markdown_prefix() {
        assert_eq!(QuickBlockType::Heading1.markdown_prefix(), "# ");
        assert_eq!(QuickBlockType::Heading2.markdown_prefix(), "## ");
        assert_eq!(QuickBlockType::Heading3.markdown_prefix(), "### ");
        assert_eq!(QuickBlockType::BulletList.markdown_prefix(), "- ");
        assert_eq!(QuickBlockType::NumberedList.markdown_prefix(), "1. ");
        assert_eq!(QuickBlockType::CodeBlock.markdown_prefix(), "```\n");
    }

    #[test]
    fn test_block_type_markdown_suffix() {
        assert_eq!(QuickBlockType::CodeBlock.markdown_suffix(), "\n```");
        assert_eq!(QuickBlockType::Image.markdown_suffix(), ")");
        assert_eq!(QuickBlockType::Link.markdown_suffix(), ")");
        assert_eq!(QuickBlockType::Paragraph.markdown_suffix(), "");
    }

    #[test]
    fn test_block_type_from_str() {
        use std::str::FromStr;
        assert_eq!(QuickBlockType::from_str("h1"), Ok(QuickBlockType::Heading1));
        assert_eq!(QuickBlockType::from_str("H1"), Ok(QuickBlockType::Heading1));
        assert_eq!(
            QuickBlockType::from_str("heading1"),
            Ok(QuickBlockType::Heading1)
        );
        assert_eq!(QuickBlockType::from_str("p"), Ok(QuickBlockType::Paragraph));
        assert_eq!(
            QuickBlockType::from_str("code"),
            Ok(QuickBlockType::CodeBlock)
        );
        assert!(QuickBlockType::from_str("unknown").is_err());
    }

    #[test]
    fn test_quick_add_manager_add_block_paragraph() {
        let manager = QuickAddManager::new();
        let result = manager
            .add_block(QuickBlockType::Paragraph, "Hello World")
            .unwrap();
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn test_quick_add_manager_add_block_heading1() {
        let manager = QuickAddManager::new();
        let result = manager
            .add_block(QuickBlockType::Heading1, "Main Title")
            .unwrap();
        assert_eq!(result, "# Main Title");
    }

    #[test]
    fn test_quick_add_manager_add_block_code() {
        let manager = QuickAddManager::new();
        let result = manager
            .add_block(QuickBlockType::CodeBlock, "fn main() {}")
            .unwrap();
        assert_eq!(result, "```\nfn main() {}\n```");
    }

    #[test]
    fn test_quick_add_manager_add_block_bullet_list() {
        let manager = QuickAddManager::new();
        let result = manager
            .add_block(QuickBlockType::BulletList, "Item 1\nItem 2\nItem 3")
            .unwrap();
        assert_eq!(result, "- Item 1\n- Item 2\n- Item 3");
    }

    #[test]
    fn test_quick_add_manager_add_block_numbered_list() {
        let manager = QuickAddManager::new();
        let result = manager
            .add_block(QuickBlockType::NumberedList, "First\nSecond\nThird")
            .unwrap();
        assert_eq!(result, "1. First\n1. Second\n1. Third");
    }

    #[test]
    fn test_quick_add_manager_add_block_blockquote() {
        let manager = QuickAddManager::new();
        let result = manager
            .add_block(QuickBlockType::Blockquote, "Quote text")
            .unwrap();
        assert_eq!(result, "> Quote text");
    }

    #[test]
    fn test_quick_add_manager_add_block_horizontal_rule() {
        let manager = QuickAddManager::new();
        // HorizontalRule ignore le contenu et retourne toujours "---"
        let result = manager
            .add_block(QuickBlockType::HorizontalRule, "-")
            .unwrap();
        assert_eq!(result, "---");
    }

    #[test]
    fn test_quick_add_manager_add_block_image() {
        let manager = QuickAddManager::new();
        let result = manager
            .add_block(QuickBlockType::Image, "alt image_url")
            .unwrap();
        assert_eq!(result, "![alt](image_url)");
    }

    #[test]
    fn test_quick_add_manager_add_block_link() {
        let manager = QuickAddManager::new();
        let result = manager.add_block(QuickBlockType::Link, "link url").unwrap();
        assert_eq!(result, "[link](url)");
    }

    #[test]
    fn test_quick_add_manager_add_block_by_shortcut() {
        let manager = QuickAddManager::new();
        let result = manager.add_block_by_shortcut("h1", "Title").unwrap();
        assert_eq!(result, "# Title");
    }

    #[test]
    fn test_quick_add_manager_add_block_empty_content() {
        let manager = QuickAddManager::new();
        let result = manager.add_block(QuickBlockType::Paragraph, "");
        assert!(matches!(result, Err(QuickAddError::EmptyContent)));
    }

    #[test]
    fn test_quick_add_manager_add_block_invalid_shortcut() {
        let manager = QuickAddManager::new();
        let result = manager.add_block_by_shortcut("invalid", "content");
        assert!(matches!(result, Err(QuickAddError::InvalidBlockType(_))));
    }

    #[test]
    fn test_quick_template_new() {
        let template = QuickTemplate::new("test", "Hello $name");
        assert_eq!(template.name, "test");
        assert_eq!(template.content, "Hello $name");
    }

    #[test]
    fn test_quick_template_apply() {
        let template = QuickTemplate::new("greeting", "Hello $name, welcome to $place!");
        let mut substitutions = HashMap::new();
        substitutions.insert("name".to_string(), "Alice".to_string());
        substitutions.insert("place".to_string(), "Note Craft".to_string());

        let result = template.apply(&substitutions);
        assert_eq!(result, "Hello Alice, welcome to Note Craft!");
    }

    #[test]
    fn test_quick_template_apply_missing_substitution() {
        let template = QuickTemplate::new("greeting", "Hello $name!");
        let substitutions = HashMap::new();

        let result = template.apply(&substitutions);
        assert_eq!(result, "Hello $name!");
    }

    #[test]
    fn test_quick_add_manager_add_template() {
        let mut manager = QuickAddManager::new();
        let template = QuickTemplate::new("todo", "- [ ] $task");
        manager.add_template(template).unwrap();

        assert!(manager.get_template("todo").is_some());
    }

    #[test]
    fn test_quick_add_manager_remove_template() {
        let mut manager = QuickAddManager::new();
        let template = QuickTemplate::new("todo", "- [ ] $task");
        manager.add_template(template).unwrap();

        manager.remove_template("todo").unwrap();
        assert!(manager.get_template("todo").is_none());
    }

    #[test]
    fn test_quick_add_manager_remove_nonexistent_template() {
        let mut manager = QuickAddManager::new();
        let result = manager.remove_template("nonexistent");
        assert!(matches!(result, Err(QuickAddError::TemplateNotFound(_))));
    }

    #[test]
    fn test_quick_add_manager_add_from_template() {
        let mut manager = QuickAddManager::new();
        let template = QuickTemplate::new("greeting", "Hello $name!");
        manager.add_template(template).unwrap();

        let mut substitutions = HashMap::new();
        substitutions.insert("name".to_string(), "Bob".to_string());

        let result = manager
            .add_from_template("greeting", &substitutions)
            .unwrap();
        assert_eq!(result, "Hello Bob!");
    }

    #[test]
    fn test_quick_add_manager_add_from_nonexistent_template() {
        let manager = QuickAddManager::new();
        let result = manager.add_from_template("nonexistent", &HashMap::new());
        assert!(matches!(result, Err(QuickAddError::TemplateNotFound(_))));
    }

    #[test]
    fn test_quick_add_manager_add_template_shortcut() {
        let mut manager = QuickAddManager::new();
        let template = QuickTemplate::new("greeting", "Hello $name!");
        manager.add_template(template).unwrap();
        manager.add_template_shortcut("g", "greeting").unwrap();

        let mut substitutions = HashMap::new();
        substitutions.insert("name".to_string(), "Charlie".to_string());

        let result = manager
            .add_from_template_shortcut("g", &substitutions)
            .unwrap();
        assert_eq!(result, "Hello Charlie!");
    }

    #[test]
    fn test_quick_add_manager_add_multiple_blocks() {
        let manager = QuickAddManager::new();
        let blocks = [
            (QuickBlockType::Heading1, "Title"),
            (QuickBlockType::Paragraph, "Content"),
            (QuickBlockType::CodeBlock, "code here"),
        ];

        let results = manager.add_multiple_blocks(&blocks).unwrap();
        assert_eq!(results.len(), 3);
        assert_eq!(results[0], "# Title");
        assert_eq!(results[1], "Content");
        assert_eq!(results[2], "```\ncode here\n```");
    }

    #[test]
    fn test_quick_add_manager_add_multiple_blocks_empty_content() {
        let manager = QuickAddManager::new();
        let blocks = [
            (QuickBlockType::Heading1, "Title"),
            (QuickBlockType::Paragraph, ""),
        ];

        let result = manager.add_multiple_blocks(&blocks);
        assert!(matches!(result, Err(QuickAddError::EmptyContent)));
    }

    #[test]
    fn test_quick_template_builder() {
        let template = QuickTemplateBuilder::new("note", "# $title\n$content")
            .description("Note template")
            .category("notes")
            .build();

        assert_eq!(template.name, "note");
        assert_eq!(template.description, Some("Note template".to_string()));
        assert_eq!(template.category, Some("notes".to_string()));
    }

    #[test]
    fn test_default_quick_add_manager() {
        let manager = create_default_quick_add_manager();
        assert!(manager.get_template("todo").is_some());
        assert!(manager.get_template("note").is_some());
        assert!(manager.get_template("code-snippet").is_some());
        assert!(manager.get_template("quote").is_some());
        assert!(manager.get_template("link-button").is_some());
    }

    #[test]
    fn test_invalid_template_name() {
        assert!(!is_valid_template_name(""));
        assert!(!is_valid_template_name("123invalid"));
        assert!(is_valid_template_name("valid-name"));
        assert!(is_valid_template_name("valid_name"));
        assert!(is_valid_template_name("valid.name"));
    }

    #[test]
    fn test_invalid_template_name_error() {
        let mut manager = QuickAddManager::new();
        let template = QuickTemplate::new("123invalid", "content");
        let result = manager.add_template(template);
        assert!(matches!(result, Err(QuickAddError::InvalidTemplateName(_))));
    }

    #[test]
    fn test_add_to_editor() {
        let manager = QuickAddManager::new();
        let mut editor = MarkdownEditor::new();

        manager.add_to_editor(&mut editor, "New content").unwrap();
        assert_eq!(editor.get_content(), "New content");
    }

    #[test]
    fn test_add_to_editor_existing_content() {
        let manager = QuickAddManager::new();
        let mut editor = MarkdownEditor::new();
        editor.set_content("Existing content".to_string());

        manager.add_to_editor(&mut editor, "New content").unwrap();
        assert_eq!(editor.get_content(), "Existing content\n\nNew content");
    }

    #[test]
    fn test_add_block_to_editor() {
        let manager = QuickAddManager::new();
        let mut editor = MarkdownEditor::new();

        manager
            .add_block_to_editor(&mut editor, QuickBlockType::Heading1, "Title")
            .unwrap();
        assert_eq!(editor.get_content(), "# Title");
    }

    #[test]
    fn test_add_template_to_editor() {
        let mut manager = QuickAddManager::new();
        let template = QuickTemplate::new("todo", "- [ ] $task");
        manager.add_template(template).unwrap();

        let mut editor = MarkdownEditor::new();
        let mut substitutions = HashMap::new();
        substitutions.insert("task".to_string(), "Write tests".to_string());

        manager
            .add_template_to_editor(&mut editor, "todo", &substitutions)
            .unwrap();
        assert_eq!(editor.get_content(), "- [ ] Write tests");
    }
}
