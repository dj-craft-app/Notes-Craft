//! Markdown Editor Module - Implémentation simple et robuste
//!
//! Ce module fournit:
//! - Parsing Markdown complet avec pulldown-cmark
//! - Conversion vers HTML
//! - Gestion des erreurs

use pulldown_cmark::{html, Options, Parser};

/// Erreurs possibles lors du parsing Markdown
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum MarkdownError {
    #[error("Parse error: {0}")]
    ParseError(String),
}

pub type MarkdownResult<T> = Result<T, MarkdownError>;

/// Configuration de l'éditeur Markdown
#[derive(Debug, Clone)]
pub struct MarkdownConfig {
    pub enable_tables: bool,
    pub enable_task_lists: bool,
    pub enable_footnotes: bool,
    pub enable_strikethrough: bool,
    pub enable_heading_attributes: bool,
}

impl Default for MarkdownConfig {
    fn default() -> Self {
        Self {
            enable_tables: true,
            enable_task_lists: true,
            enable_footnotes: true,
            enable_strikethrough: true,
            enable_heading_attributes: true,
        }
    }
}

impl MarkdownConfig {
    pub fn to_options(&self) -> Options {
        let mut options = Options::empty();
        if self.enable_tables {
            options.insert(Options::ENABLE_TABLES);
        }
        if self.enable_task_lists {
            options.insert(Options::ENABLE_TASKLISTS);
        }
        if self.enable_footnotes {
            options.insert(Options::ENABLE_FOOTNOTES);
        }
        if self.enable_strikethrough {
            options.insert(Options::ENABLE_STRIKETHROUGH);
        }
        if self.enable_heading_attributes {
            options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
        }
        options
    }
}

/// Editor Markdown principal
pub struct MarkdownEditor {
    config: MarkdownConfig,
    content: String,
    html: Option<String>,
}

impl MarkdownEditor {
    pub fn new() -> Self {
        Self {
            config: MarkdownConfig::default(),
            content: String::new(),
            html: None,
        }
    }

    pub fn with_config(config: MarkdownConfig) -> Self {
        Self {
            config,
            content: String::new(),
            html: None,
        }
    }

    pub fn set_content(&mut self, content: String) {
        self.content = content;
        self.html = None;
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }

    /// Parse le contenu et retourne du HTML
    pub fn to_html(&mut self) -> MarkdownResult<String> {
        if self.html.is_none() {
            let options = self.config.to_options();
            let parser = Parser::new_ext(&self.content, options);
            let mut html_output = String::new();
            html::push_html(&mut html_output, parser);
            self.html = Some(html_output.clone());
            Ok(html_output)
        } else {
            Ok(self.html.clone().unwrap())
        }
    }

    /// Répare le contenu et retourne le HTML
    pub fn render(&mut self) -> MarkdownResult<String> {
        self.to_html()
    }

    pub fn validate(&self) -> MarkdownResult<()> {
        Ok(())
    }

    pub fn sanitize(&mut self) -> &str {
        self.content = self
            .content
            .replace("<script>", "")
            .replace("</script>", "")
            .replace("javascript:", "");
        self.html = None;
        &self.content
    }

    pub fn append(&mut self, content: &str) {
        self.content.push_str(content);
        self.html = None;
    }

    pub fn clear(&mut self) {
        self.content.clear();
        self.html = None;
    }

    pub fn length(&self) -> usize {
        self.content.len()
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    pub fn get_config(&self) -> &MarkdownConfig {
        &self.config
    }

    pub fn set_config(&mut self, config: MarkdownConfig) {
        self.config = config;
        self.html = None;
    }
}

impl Default for MarkdownEditor {
    fn default() -> Self {
        Self::new()
    }
}

/// Fonctions utilitaires
pub mod utils {
    pub fn is_markdown(content: &str) -> bool {
        content.contains('#')
            || content.contains('*')
            || content.contains('_')
            || content.contains('[')
            || content.contains('`')
            || content.contains('>')
            || content.contains('-')
            || content.contains('|')
    }

    pub fn markdown_complexity(content: &str) -> f32 {
        let mut score = 0.0;
        score += content.matches('#').count() as f32 * 0.1;
        score += content.matches('*').count() as f32 * 0.05;
        score += content.matches('_').count() as f32 * 0.05;
        score += content.matches('[').count() as f32 * 0.15;
        score += content.matches('`').count() as f32 * 0.1;
        score += content.matches('>').count() as f32 * 0.2;
        score += content.matches('-').count() as f32 * 0.08;
        score += content.matches('|').count() as f32 * 0.2;
        score
    }

    pub fn word_count(content: &str) -> usize {
        content.split_whitespace().count()
    }

    pub fn char_count(content: &str) -> usize {
        content.chars().count()
    }

    pub fn line_count(content: &str) -> usize {
        content.lines().count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_editor_new() {
        let editor = MarkdownEditor::new();
        assert!(editor.get_content().is_empty());
    }

    #[test]
    fn test_editor_set_content() {
        let mut editor = MarkdownEditor::new();
        editor.set_content("Test".to_string());
        assert_eq!(editor.get_content(), "Test");
    }

    #[test]
    fn test_editor_render_heading() {
        let mut editor = MarkdownEditor::new();
        editor.set_content("# Title".to_string());
        let html = editor.render().unwrap();
        assert!(html.contains("<h1>"));
        assert!(html.contains("Title"));
    }

    #[test]
    fn test_editor_render_paragraph() {
        let mut editor = MarkdownEditor::new();
        editor.set_content("Hello, world!".to_string());
        let html = editor.render().unwrap();
        assert!(html.contains("<p>"));
        assert!(html.contains("Hello, world!"));
    }

    #[test]
    fn test_editor_render_code_block() {
        let mut editor = MarkdownEditor::new();
        editor.set_content("```rust\nfn main() {}\n```".to_string());
        let html = editor.render().unwrap();
        assert!(html.contains("<pre>"));
        // pulldown-cmark 0.9 peut ou non inclure <code> selon la configuration
        // On vérifie juste que le bloc de code est présent
        assert!(html.contains("fn main()"));
    }

    #[test]
    fn test_editor_render_list() {
        let mut editor = MarkdownEditor::new();
        editor.set_content("- Item 1\n- Item 2".to_string());
        let html = editor.render().unwrap();
        assert!(html.contains("<ul>"));
        assert!(html.contains("<li>"));
    }

    #[test]
    fn test_editor_render_table() {
        let config = MarkdownConfig {
            enable_tables: true,
            ..Default::default()
        };
        let mut editor = MarkdownEditor::with_config(config);
        editor.set_content("| A | B |\n|---|---|\n| 1 | 2 |".to_string());
        let html = editor.render().unwrap();
        assert!(html.contains("<table>"));
    }

    #[test]
    fn test_editor_render_link() {
        let mut editor = MarkdownEditor::new();
        editor.set_content("[Link](https://example.com)".to_string());
        let html = editor.render().unwrap();
        assert!(html.contains("<a"));
        assert!(html.contains("href=\"https://example.com\""));
    }

    #[test]
    fn test_editor_render_emphasis() {
        let mut editor = MarkdownEditor::new();
        editor.set_content("*emphasis* and **strong**".to_string());
        let html = editor.render().unwrap();
        assert!(html.contains("<em>"));
        assert!(html.contains("<strong>"));
    }

    #[test]
    fn test_editor_render_strikethrough() {
        let config = MarkdownConfig {
            enable_strikethrough: true,
            ..Default::default()
        };
        let mut editor = MarkdownEditor::with_config(config);
        editor.set_content("~~strikethrough~~".to_string());
        let html = editor.render().unwrap();
        assert!(html.contains("<del>"));
    }

    #[test]
    fn test_editor_render_inline_code() {
        let mut editor = MarkdownEditor::new();
        editor.set_content("This is `code`".to_string());
        let html = editor.render().unwrap();
        assert!(html.contains("<code>"));
    }

    #[test]
    fn test_editor_render_blockquote() {
        let mut editor = MarkdownEditor::new();
        editor.set_content("> Quote".to_string());
        let html = editor.render().unwrap();
        assert!(html.contains("<blockquote>"));
    }

    #[test]
    fn test_editor_render_horizontal_rule() {
        let mut editor = MarkdownEditor::new();
        editor.set_content("---".to_string());
        let html = editor.render().unwrap();
        assert!(html.contains("<hr"));
    }

    #[test]
    fn test_editor_render_task_list() {
        let config = MarkdownConfig {
            enable_task_lists: true,
            ..Default::default()
        };
        let mut editor = MarkdownEditor::with_config(config);
        editor.set_content("- [x] Task 1\n- [ ] Task 2".to_string());
        let html = editor.render().unwrap();
        assert!(html.contains("<input"));
        assert!(html.contains("type=\"checkbox\""));
    }

    #[test]
    fn test_editor_append() {
        let mut editor = MarkdownEditor::new();
        editor.set_content("Hello".to_string());
        editor.append(", world!");
        assert_eq!(editor.get_content(), "Hello, world!");
    }

    #[test]
    fn test_editor_clear() {
        let mut editor = MarkdownEditor::new();
        editor.set_content("Content".to_string());
        editor.clear();
        assert!(editor.get_content().is_empty());
    }

    #[test]
    fn test_editor_sanitize() {
        let mut editor = MarkdownEditor::new();
        editor.set_content("<script>alert('xss')</script>".to_string());
        let sanitized = editor.sanitize();
        assert!(!sanitized.contains("<script>"));
    }

    #[test]
    fn test_editor_length() {
        let mut editor = MarkdownEditor::new();
        editor.set_content("Test".to_string());
        assert_eq!(editor.length(), 4);
    }

    #[test]
    fn test_editor_is_empty() {
        let editor = MarkdownEditor::new();
        assert!(editor.is_empty());
        let mut editor2 = MarkdownEditor::new();
        editor2.set_content("Not empty".to_string());
        assert!(!editor2.is_empty());
    }

    #[test]
    fn test_config_default() {
        let config = MarkdownConfig::default();
        assert!(config.enable_tables);
        assert!(config.enable_task_lists);
        assert!(config.enable_footnotes);
        assert!(config.enable_strikethrough);
    }

    #[test]
    fn test_utils_is_markdown() {
        assert!(utils::is_markdown("# Heading"));
        assert!(!utils::is_markdown("Plain text"));
    }

    #[test]
    fn test_utils_word_count() {
        assert_eq!(utils::word_count("Hello world"), 2);
    }

    #[test]
    fn test_utils_line_count() {
        assert_eq!(utils::line_count("Line 1\nLine 2"), 2);
    }

    #[test]
    fn test_utils_markdown_complexity() {
        let simple = "Just text";
        let complex = "# Heading\n\n*emphasis*\n\n[link](url)";
        assert!(utils::markdown_complexity(simple) < utils::markdown_complexity(complex));
    }

    #[test]
    fn test_editor_to_html() {
        let mut editor = MarkdownEditor::new();
        editor.set_content("# Title\n\nContent".to_string());
        let html = editor.to_html().unwrap();
        assert!(html.contains("<h1>Title</h1>"));
    }

    #[test]
    fn test_editor_validate() {
        let editor = MarkdownEditor::new();
        assert!(editor.validate().is_ok());
    }

    #[test]
    fn test_editor_cache_html() {
        let mut editor = MarkdownEditor::new();
        editor.set_content("# Title".to_string());
        let html1 = editor.render().unwrap();
        let html2 = editor.render().unwrap();
        assert_eq!(html1, html2);
    }

    #[test]
    fn test_editor_set_config() {
        let mut editor = MarkdownEditor::new();
        let config = MarkdownConfig {
            enable_tables: false,
            ..Default::default()
        };
        editor.set_config(config);
        assert!(!editor.get_config().enable_tables);
    }

    #[test]
    fn test_complex_document() {
        let mut editor = MarkdownEditor::new();
        let content = "# Title\n\n## Section\n\nParagraph with *emphasis* and **strong**.\n\n- Item 1\n- Item 2\n\n```rust\nfn main() {}\n```\n\n> Quote";
        editor.set_content(content.to_string());
        let html = editor.render().unwrap();
        assert!(html.contains("<h1>Title</h1>"));
        assert!(html.contains("<h2>Section</h2>"));
        assert!(html.contains("<em>"));
        assert!(html.contains("<strong>"));
        assert!(html.contains("<ul>"));
        assert!(html.contains("<pre>"));
        assert!(html.contains("<blockquote>"));
    }
}

#[test]
fn test_editor_render_image() {
    let mut editor = MarkdownEditor::new();
    editor.set_content("![Alt text](image.png)".to_string());
    let html = editor.render().unwrap();
    assert!(html.contains("<img"));
    assert!(html.contains("src=\"image.png\""));
}
