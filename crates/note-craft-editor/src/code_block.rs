//! Code Block Module
//!
//! Ce module fournit le support pour les blocs de code avec:
//! - Coloration syntaxique (via classes CSS)
//! - Numérotation des lignes
//! - Support des métadonnées
//! - Génération de HTML

/// Métadonnées d'un bloc de code
#[derive(Debug, Clone)]
pub struct CodeBlockMetadata {
    /// Langage de programmation
    pub language: Option<String>,
    /// Numéro de ligne de départ
    pub start_line: usize,
    /// Si la numérotation des lignes est activée
    pub show_line_numbers: bool,
    /// Si la copie est autorisée
    pub allow_copy: bool,
}

impl Default for CodeBlockMetadata {
    fn default() -> Self {
        Self {
            language: None,
            start_line: 1,
            show_line_numbers: true,
            allow_copy: true,
        }
    }
}

impl CodeBlockMetadata {
    /// Crée des métadonnées avec un langage
    pub fn with_language(language: String) -> Self {
        Self {
            language: Some(language),
            ..Default::default()
        }
    }

    /// Active/Désactive la numérotation des lignes
    pub fn with_line_numbers(mut self, show: bool) -> Self {
        self.show_line_numbers = show;
        self
    }

    /// Définit le numéro de ligne de départ
    pub fn with_start_line(mut self, line: usize) -> Self {
        self.start_line = line;
        self
    }
}

/// Bloc de code avec contenu et métadonnées
#[derive(Debug, Clone)]
pub struct CodeBlock {
    /// Contenu du code
    pub content: String,
    /// Métadonnées
    pub metadata: CodeBlockMetadata,
}

impl CodeBlock {
    /// Crée un nouveau bloc de code
    pub fn new(content: String) -> Self {
        Self {
            content,
            metadata: CodeBlockMetadata::default(),
        }
    }

    /// Crée un bloc de code avec un langage
    pub fn with_language(content: String, language: String) -> Self {
        Self {
            content,
            metadata: CodeBlockMetadata::with_language(language),
        }
    }

    /// Définit les métadonnées
    pub fn with_metadata(mut self, metadata: CodeBlockMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    /// Rend le bloc de code en HTML
    pub fn render(&self) -> String {
        self.render_html()
    }

    /// Rend le bloc de code en HTML
    pub fn render_html(&self) -> String {
        let mut html = String::new();

        // Début du bloc
        let mut classes: Vec<String> = vec!["code-block".to_string()];
        if let Some(ref lang) = self.metadata.language {
            classes.push(format!("language-{}", lang.to_lowercase()));
        }
        if self.metadata.show_line_numbers {
            classes.push("with-line-numbers".to_string());
        }
        html.push_str(&format!("<pre class=\"{}\">", classes.join(" ")));

        // Contenu
        if self.metadata.show_line_numbers {
            let lines: Vec<&str> = self.content.lines().collect();
            for (line_num, line) in (self.metadata.start_line..).zip(lines) {
                html.push_str(&format!(
                    "<span class=\"line\" data-line=\"{}\"><span class=\"line-number\">{:>4}</span> <span class=\"line-content\">{}</span></span>\n",
                    line_num, line_num, Self::escape_html(line)
                ));
            }
        } else {
            html.push_str(&format!(
                "<code class=\"code-content\">{}</code>",
                Self::escape_html(&self.content)
            ));
        }

        // Bouton de copie si autorisé
        if self.metadata.allow_copy {
            html.push_str("<button class=\"copy-button\" onclick=\"copyCode(this)\">Copy</button>");
        }

        // Fin du bloc
        html.push_str("</pre>");
        html
    }

    /// Rend le bloc de code en HTML simple (sans line numbers)
    pub fn render_simple(&self) -> String {
        let mut html = String::new();
        html.push_str("<pre><code>");
        html.push_str(&Self::escape_html(&self.content));
        html.push_str("</code></pre>");
        html
    }

    /// Obtient le langage
    pub fn language(&self) -> Option<&String> {
        self.metadata.language.as_ref()
    }

    /// Échappe les caractères HTML
    fn escape_html(text: &str) -> String {
        text.replace("&", "&amp;")
            .replace("<", "&lt;")
            .replace(">", "&gt;")
            .replace("\"", "&quot;")
            .replace("'", "&#039;")
    }
}

impl Default for CodeBlock {
    fn default() -> Self {
        Self::new(String::new())
    }
}

/// Parseur de blocs de code depuis du Markdown
pub struct CodeBlockParser;

impl CodeBlockParser {
    /// Parse un bloc de code Markdown (```lang\ncode\n```)
    pub fn parse_markdown_block(code: &str) -> Option<CodeBlock> {
        let trimmed = code.trim();

        if !trimmed.starts_with("```") && !trimmed.starts_with("~~~") {
            return None;
        }

        // Extraire le langage et le contenu
        let lines: Vec<&str> = trimmed.lines().collect();
        if lines.is_empty() {
            return None;
        }

        let first_line = lines[0].trim();
        let language = if first_line.len() > 3 {
            Some(first_line[3..].trim().to_string())
        } else {
            None
        };

        // Trouver la fin du bloc
        let mut content_lines = Vec::new();
        let mut in_code = false;
        for line in lines.iter().skip(1) {
            let trimmed_line = line.trim();
            if trimmed_line.starts_with("```") || trimmed_line.starts_with("~~~") {
                break;
            }
            if !in_code && trimmed_line.is_empty() {
                continue;
            }
            in_code = true;
            content_lines.push(*line);
        }

        let content = content_lines.join("\n");

        Some(CodeBlock::with_language(
            content,
            language.unwrap_or_default(),
        ))
    }

    /// Parse un bloc de code inline ( `code` )
    pub fn parse_inline_code(code: &str) -> Option<String> {
        if !code.starts_with('`') || !code.ends_with('`') {
            return None;
        }

        let content = code[1..code.len() - 1].to_string();
        Some(content)
    }

    /// Parse plusieurs blocs de code depuis un document
    pub fn parse_all_markdown(document: &str) -> Vec<CodeBlock> {
        let mut blocks = Vec::new();
        let mut in_code_block = false;
        let mut current_code = String::new();

        for line in document.lines() {
            let trimmed = line.trim();

            if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
                if in_code_block {
                    // Fin du bloc
                    if let Some(block) = Self::parse_markdown_block(&current_code) {
                        blocks.push(block);
                    }
                    current_code.clear();
                    in_code_block = false;
                } else {
                    // Début du bloc
                    current_code.push_str(line);
                    current_code.push('\n');
                    in_code_block = true;
                }
            } else if in_code_block {
                current_code.push_str(line);
                current_code.push('\n');
            }
        }

        // Si on est encore dans un bloc
        if in_code_block {
            if let Some(block) = Self::parse_markdown_block(&current_code) {
                blocks.push(block);
            }
        }

        blocks
    }
}

/// Génère le CSS pour les blocs de code
pub fn generate_css() -> String {
    r#"
/* Code block styling */
.code-block {
    background-color: #282c34;
    color: #abb2bf;
    padding: 1em;
    border-radius: 4px;
    overflow-x: auto;
    font-family: Consolas, Monaco, "Courier New", monospace;
    font-size: 14px;
    line-height: 1.5;
    position: relative;
    margin: 1em 0;
}

.code-block.with-line-numbers {
    counter-reset: line-numbering;
}

.line {
    display: block;
    white-space: pre;
}

.line-number {
    color: #636d83;
    margin-right: 1em;
    user-select: none;
    display: inline-block;
    min-width: 40px;
    text-align: right;
    padding-right: 10px;
    border-right: 1px solid #404756;
}

.line-content {
    display: inline;
}

.code-content {
    white-space: pre;
}

.copy-button {
    position: absolute;
    top: 0.5em;
    right: 0.5em;
    background: #404756;
    color: #abb2bf;
    border: none;
    border-radius: 3px;
    padding: 0.25em 0.5em;
    cursor: pointer;
    font-size: 0.8em;
    opacity: 0.7;
    transition: opacity 0.2s;
}

.copy-button:hover {
    opacity: 1;
}

/* Syntax highlighting classes */
.code-block .token.comment { color: #5c6370; font-style: italic; }
.code-block .token.keyword { color: #c678dd; font-weight: bold; }
.code-block .token.string { color: #98c379; }
.code-block .token.number { color: #d19a66; }
.code-block .token.type { color: #e5c07b; }
.code-block .token.function { color: #61afef; }
.code-block .token.operator { color: #abb2bf; }
.code-block .token.punctuation { color: #abb2bf; }

/* Language-specific styling */
.code-block.language-rust { }
.code-block.language-python { }
.code-block.language-javascript { }
.code-block.language-html { }
.code-block.language-css { }
.code-block.language-bash { }
.code-block.language-sql { }
.code-block.language-json { }
.code-block.language-yaml { }
.code-block.language-markdown { }
"#
    .to_string()
}

/// JavaScript pour la copie de code (à inclure dans le HTML)
pub fn copy_javascript() -> String {
    r#"
function copyCode(button) {
    const pre = button.parentElement;
    const code = pre.querySelector('.code-content') || pre;
    const text = code.textContent || code.innerText;
    
    navigator.clipboard.writeText(text).then(() => {
        const originalText = button.textContent;
        button.textContent = 'Copied!';
        button.style.background = '#4caf50';
        
        setTimeout(() => {
            button.textContent = originalText;
            button.style.background = '#404756';
        }, 2000);
    }).catch(err => {
        console.error('Failed to copy: ', err);
    });
}
"#
    .to_string()
}

/// Tests unitaires
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_block_new() {
        let block = CodeBlock::new("fn main() {}".to_string());
        assert_eq!(block.content, "fn main() {}");
    }

    #[test]
    fn test_code_block_with_language() {
        let block = CodeBlock::with_language("fn main() {}".to_string(), "rust".to_string());
        assert_eq!(block.content, "fn main() {}");
        assert_eq!(block.language().unwrap(), "rust");
    }

    #[test]
    fn test_code_block_render() {
        let block = CodeBlock::new("fn main() {}".to_string());
        let html = block.render();
        assert!(html.contains("<pre"));
        assert!(html.contains("fn main()"));
        // Le contenu est escape, donc vérifions qu'il est présent
        assert!(html.contains("fn main"));
    }

    #[test]
    fn test_code_block_render_with_line_numbers() {
        let block = CodeBlock::new("fn main()\n{\n}".to_string());
        let html = block.render();
        assert!(html.contains("line-number"));
        assert!(html.contains("1"));
        assert!(html.contains("2"));
    }

    #[test]
    fn test_code_block_render_simple() {
        let block = CodeBlock::new("fn main() {}".to_string());
        let html = block.render_simple();
        assert!(html.contains("<pre><code>"));
        assert!(html.contains("fn main()"));
    }

    #[test]
    fn test_code_block_escape_html() {
        let block = CodeBlock::new("<script>alert('xss')</script>".to_string());
        let html = block.render();
        assert!(!html.contains("<script>"));
        assert!(html.contains("&lt;script&gt;"));
    }

    #[test]
    fn test_parser_parse_markdown_block() {
        let code = "```rust\nfn main() {}\n```";
        let block = CodeBlockParser::parse_markdown_block(code).unwrap();
        assert_eq!(block.language().unwrap(), "rust");
        assert_eq!(block.content, "fn main() {}");
    }

    #[test]
    fn test_parser_parse_inline_code() {
        let code = "`fn main()`";
        let content = CodeBlockParser::parse_inline_code(code).unwrap();
        assert_eq!(content, "fn main()");
    }

    #[test]
    fn test_parser_parse_all_markdown() {
        let document = "Some text\n\n```rust\nfn main() {}\n```\n\nMore text\n\n```python\ndef hello():\n    pass\n```";
        let blocks = CodeBlockParser::parse_all_markdown(document);
        assert_eq!(blocks.len(), 2);
        assert_eq!(blocks[0].language().unwrap(), "rust");
        assert_eq!(blocks[1].language().unwrap(), "python");
    }

    #[test]
    fn test_generate_css() {
        let css = generate_css();
        assert!(css.contains(".code-block"));
        assert!(css.contains(".line-number"));
    }

    #[test]
    fn test_copy_javascript() {
        let js = copy_javascript();
        assert!(js.contains("function copyCode"));
    }

    #[test]
    fn test_code_block_metadata() {
        let metadata = CodeBlockMetadata::with_language("rust".to_string())
            .with_line_numbers(false)
            .with_start_line(10);

        assert_eq!(metadata.language, Some("rust".to_string()));
        assert!(!metadata.show_line_numbers);
        assert_eq!(metadata.start_line, 10);
    }

    #[test]
    fn test_code_block_with_metadata() {
        let block = CodeBlock::new("code".to_string())
            .with_metadata(CodeBlockMetadata::with_language("rust".to_string()));
        assert_eq!(block.language().unwrap(), "rust");
    }
}
