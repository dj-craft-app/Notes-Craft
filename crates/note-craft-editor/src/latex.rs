//! LaTeX Support Module
//!
//! Ce module fournit le support pour les formules mathématiques LaTeX avec rendu
//! via KaTeX (CDN).
//!
//! Fonctionnalités:
//! - Parsing des expressions LaTeX inline ($...$) et display ($$...$$)
//! - Génération de HTML avec classes KaTeX
//! - Support des équations numérotées
//! - Génération du HTML pour l'intégration KaTeX

/// Type d'expression LaTeX
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LatexType {
    /// Expression inline: $...$
    Inline,
    /// Expression display: $$...$$
    Display,
}

/// Expression LaTeX avec contenu et type
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LatexExpression {
    /// Contenu LaTeX
    pub content: String,
    /// Type d'expression
    pub latex_type: LatexType,
    /// Si l'expression doit être numérotée
    pub numbered: bool,
}

impl LatexExpression {
    /// Crée une nouvelle expression LaTeX inline
    pub fn inline(content: String) -> Self {
        Self {
            content,
            latex_type: LatexType::Inline,
            numbered: false,
        }
    }

    /// Crée une nouvelle expression LaTeX display
    pub fn display(content: String) -> Self {
        Self {
            content,
            latex_type: LatexType::Display,
            numbered: false,
        }
    }

    /// Active la numérotation
    pub fn with_number(mut self) -> Self {
        self.numbered = true;
        self
    }

    /// Rend l'expression en HTML avec KaTeX
    pub fn render(&self) -> String {
        match self.latex_type {
            LatexType::Inline => self.render_inline(),
            LatexType::Display => self.render_display(),
        }
    }

    /// Rend une expression inline
    fn render_inline(&self) -> String {
        format!(
            "<span class=\"katex\" data-katex=\"{}\">{}</span>",
            Self::escape_html(&self.content),
            Self::escape_html(&self.content)
        )
    }

    /// Rend une expression display
    fn render_display(&self) -> String {
        if self.numbered {
            format!(
                "<div class=\"katex-display numbered\" data-katex=\"{}\">{}</div>",
                Self::escape_html(&self.content),
                Self::escape_html(&self.content)
            )
        } else {
            format!(
                "<div class=\"katex-display\" data-katex=\"{}\">{}</div>",
                Self::escape_html(&self.content),
                Self::escape_html(&self.content)
            )
        }
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

/// Parseur d'expressions LaTeX
pub struct LatexParser;

impl LatexParser {
    /// Parse un document pour trouver toutes les expressions LaTeX
    pub fn parse_document(document: &str) -> Vec<LatexExpression> {
        let mut expressions = Vec::new();
        let mut remaining = document;

        // Parser toutes les expressions (display et inline) dans une seule passe
        let mut pos = 0;
        while pos < remaining.len() {
            // Vérifier si on a une expression display ($$...$$)
            if remaining[pos..].starts_with("$$") {
                if let Some((display_expr, new_remaining, advance)) =
                    Self::parse_display_at(remaining, pos)
                {
                    expressions.push(display_expr);
                    pos += advance;
                    remaining = new_remaining;
                    continue;
                }
            }
            // Vérifier si on a une expression inline ($...$)
            else if remaining[pos..].starts_with("$") {
                if let Some((inline_expr, new_remaining, advance)) =
                    Self::parse_inline_at(remaining, pos)
                {
                    expressions.push(inline_expr);
                    pos += advance;
                    remaining = new_remaining;
                    continue;
                }
            }
            pos += 1;
        }

        expressions
    }

    /// Parse une expression display à une position donnée
    fn parse_display_at(
        document: &str,
        start_pos: usize,
    ) -> Option<(LatexExpression, &str, usize)> {
        let start = "$$";
        let end = "$$";

        if !document[start_pos..].starts_with(start) {
            return None;
        }

        let after_start = &document[start_pos + start.len()..];
        if let Some(end_idx) = after_start.find(end) {
            let content = &after_start[..end_idx];
            let expression = LatexExpression::display(content.trim().to_string());
            let advance = start_pos + start.len() + end_idx + end.len();
            return Some((expression, document[advance..].trim_start(), advance));
        }
        None
    }

    /// Parse une expression inline à une position donnée
    fn parse_inline_at(document: &str, start_pos: usize) -> Option<(LatexExpression, &str, usize)> {
        let start = "$";
        let end = "$";

        if !document[start_pos..].starts_with(start) {
            return None;
        }

        let after_start = &document[start_pos + start.len()..];
        if let Some(end_idx) = after_start.find(end) {
            // S'assurer que ce n'est pas une expression display
            if after_start[..end_idx].contains("$$") {
                return None;
            }
            let content = &after_start[..end_idx];
            let expression = LatexExpression::inline(content.trim().to_string());
            let advance = start_pos + start.len() + end_idx + end.len();
            return Some((expression, document[advance..].trim_start(), advance));
        }
        None
    }

    /// Parse toutes les expressions LaTeX inline dans un document
    pub fn parse_inline_all(document: &str) -> Vec<LatexExpression> {
        let all = Self::parse_document(document);
        all.into_iter()
            .filter(|e| e.latex_type == LatexType::Inline)
            .collect()
    }

    /// Parse toutes les expressions LaTeX display dans un document
    pub fn parse_display_all(document: &str) -> Vec<LatexExpression> {
        let all = Self::parse_document(document);
        all.into_iter()
            .filter(|e| e.latex_type == LatexType::Display)
            .collect()
    }

    /// Vérifie si un document contient des expressions LaTeX
    pub fn has_latex(document: &str) -> bool {
        document.contains("$$") || document.contains("$")
    }

    /// Compte le nombre d'expressions LaTeX dans un document
    pub fn count_latex(document: &str) -> usize {
        Self::parse_document(document).len()
    }
}

/// Génère le HTML pour intégrer KaTeX
pub fn generate_katex_html() -> String {
    // CDN KaTeX
    r#"<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.11/dist/katex.min.css">
<script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.11/dist/katex.min.js"></script>
<script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.11/dist/contrib/auto-render.min.js" onload="renderMathInElement(document.body);"></script>"#
    .to_string()
}

/// Génère le CSS pour les expressions LaTeX
pub fn generate_latex_css() -> String {
    r#"
/* LaTeX styling */
.katex {
    font-size: 1.1em;
    vertical-align: middle;
}

.katex-display {
    display: block;
    text-align: center;
    margin: 1em 0;
    padding: 0.5em;
    overflow-x: auto;
}

.katex-display.numbered {
    text-align: left;
}

.katex .katex-mathml {
    display: none;
}

/* Assure que KaTeX s'affiche correctement */
.katex-display .katex {
    display: inline-block;
}
"#
    .to_string()
}

/// Remplace les expressions LaTeX dans un document HTML par leur version rendu
pub fn render_latex_in_html(html: &str) -> String {
    let expressions = LatexParser::parse_document(html);
    let mut result = html.to_string();

    // Trouver et remplacer chaque expression par son rendu
    for expr in expressions {
        let rendered = expr.render();
        let delimiter = match expr.latex_type {
            LatexType::Inline => "$",
            LatexType::Display => "$$$",
        };
        let pattern = format!("${}{}{}", delimiter, expr.content, delimiter);
        result = result.replacen(&pattern, &rendered, 1);
    }

    result
}

/// Tests unitaires
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_latex_expression_inline() {
        let expr = LatexExpression::inline("E = mc^2".to_string());
        assert_eq!(expr.latex_type, LatexType::Inline);
        assert!(!expr.numbered);
    }

    #[test]
    fn test_latex_expression_display() {
        let expr = LatexExpression::display("\\frac{1}{2}".to_string());
        assert_eq!(expr.latex_type, LatexType::Display);
        assert!(!expr.numbered);
    }

    #[test]
    fn test_latex_expression_numbered() {
        let expr = LatexExpression::display("\\frac{1}{2}".to_string()).with_number();
        assert!(expr.numbered);
    }

    #[test]
    fn test_latex_render_inline() {
        let expr = LatexExpression::inline("E = mc^2".to_string());
        let html = expr.render();
        assert!(html.contains("katex"));
        assert!(html.contains("E = mc^2"));
    }

    #[test]
    fn test_latex_render_display() {
        let expr = LatexExpression::display("\\frac{1}{2}".to_string());
        let html = expr.render();
        assert!(html.contains("katex-display"));
        assert!(html.contains("\\frac{1}{2}"));
    }

    #[test]
    fn test_latex_render_display_numbered() {
        let expr = LatexExpression::display("\\frac{1}{2}".to_string()).with_number();
        let html = expr.render();
        assert!(html.contains("numbered"));
    }

    #[test]
    fn test_parser_parse_inline() {
        let document = "The formula $E = mc^2$ is famous.";
        let expressions = LatexParser::parse_document(document);
        assert_eq!(expressions.len(), 1);
        assert_eq!(expressions[0].content, "E = mc^2");
        assert_eq!(expressions[0].latex_type, LatexType::Inline);
    }

    #[test]
    fn test_parser_parse_display() {
        let document = "The formula:\n$$\\int_0^1 x^2 dx$$\nis an integral.";
        let expressions = LatexParser::parse_document(document);
        assert_eq!(expressions.len(), 1);
        assert_eq!(expressions[0].content, "\\int_0^1 x^2 dx");
        assert_eq!(expressions[0].latex_type, LatexType::Display);
    }

    #[test]
    fn test_parser_parse_multiple() {
        let document = "Here $a^2$ and $$\\sum_{i=1}^n i$$ and $b^2$.";
        let expressions = LatexParser::parse_document(document);
        assert!(expressions.len() >= 2);
    }

    #[test]
    fn test_parser_has_latex() {
        assert!(LatexParser::has_latex("Here $x$ and $$y$$"));
        assert!(!LatexParser::has_latex("No LaTeX here"));
    }

    #[test]
    fn test_parser_count_latex() {
        let document = "$a$ and $b$ and $$c$$ here.";
        let count = LatexParser::count_latex(document);
        assert!(count >= 2);
    }

    #[test]
    fn test_escape_html() {
        let expr = LatexExpression::inline("<script>alert('xss')</script>".to_string());
        let html = expr.render();
        assert!(!html.contains("<script>"));
        assert!(html.contains("&lt;script&gt;"));
    }

    #[test]
    fn test_generate_katex_html() {
        let html = generate_katex_html();
        assert!(html.contains("katex.min.css"));
        assert!(html.contains("katex.min.js"));
        assert!(html.contains("auto-render.min.js"));
    }

    #[test]
    fn test_generate_latex_css() {
        let css = generate_latex_css();
        assert!(css.contains(".katex"));
        assert!(css.contains(".katex-display"));
    }
}
