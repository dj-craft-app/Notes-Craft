//! Rendering Pipeline Module - Pipeline de rendu Markdown vers HTML
//!
//! Ce module fournit:
//! - Pipeline de rendu Markdown -> HTML avec pulldown-cmark
//! - Optimisation du rendu (cache, batch processing)
//! - Gestion du cache de rendu
//! - Support des extensions Markdown
//! - Rendu incrémental

use std::collections::HashMap;
use std::sync::{Arc, OnceLock, RwLock};
use std::time::Instant;

use pulldown_cmark::{html, Options, Parser};

use crate::markdown::MarkdownEditor;

/// Erreurs possibles lors du rendu
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum RenderingError {
    #[error("Rendering error: {0}")]
    RenderingError(String),
    #[error("Cache error: {0}")]
    CacheError(String),
    #[error("Extension not supported: {0}")]
    ExtensionNotSupported(String),
}

pub type RenderingResult<T> = Result<T, RenderingError>;

/// Configuration du pipeline de rendu
#[derive(Debug, Clone)]
pub struct RenderingConfig {
    /// Utiliser le cache
    pub use_cache: bool,
    /// Taille maximale du cache (en nombre d'entrées)
    pub cache_size: usize,
    /// Durée de vie maximale des entrées du cache (en secondes)
    pub cache_ttl: u64,
    /// Activer l'optimisation du rendu
    pub optimize: bool,
    /// Extensions activées
    pub extensions: Vec<String>,
}

impl Default for RenderingConfig {
    fn default() -> Self {
        Self {
            use_cache: true,
            cache_size: 1000,
            cache_ttl: 300, // 5 minutes
            optimize: true,
            extensions: vec![
                "tables".to_string(),
                "task_lists".to_string(),
                "strikethrough".to_string(),
                "footnotes".to_string(),
            ],
        }
    }
}

/// Entrée du cache de rendu
#[derive(Debug, Clone)]
struct CacheEntry {
    /// Contenu Markdown (réservé pour futur usage)
    #[allow(dead_code)]
    content: String,
    /// HTML rendu
    html: String,
    /// Horodatage de la création
    created_at: Instant,
    /// Taille du contenu (en octets) (réservé pour futur usage)
    #[allow(dead_code)]
    size: usize,
}

/// Cache de rendu thread-safe
#[derive(Debug, Clone)]
pub struct RenderingCache {
    /// Stockage du cache
    entries: Arc<RwLock<HashMap<String, CacheEntry>>>,
    /// Configuration du cache
    config: RenderingConfig,
}

impl RenderingCache {
    pub fn new(config: RenderingConfig) -> Self {
        Self {
            entries: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }

    /// Récupère une entrée du cache
    pub fn get(&self, key: &str) -> Option<String> {
        let entries = self.entries.read().ok()?;

        if let Some(entry) = entries.get(key) {
            // Vérifier la durée de vie
            if entry.created_at.elapsed().as_secs() > self.config.cache_ttl {
                return None;
            }
            Some(entry.html.clone())
        } else {
            None
        }
    }

    /// Ajoute une entrée au cache
    pub fn set(&self, key: &str, content: &str, html: &str) -> RenderingResult<()> {
        let mut entries = self.entries.write().map_err(|e| {
            RenderingError::CacheError(format!("Failed to acquire write lock: {}", e))
        })?;

        // Vérifier la taille maximale du cache
        if entries.len() >= self.config.cache_size {
            // Nettoyer les entrées les plus anciennes
            self.cleanup_old_entries(&mut entries);
        }

        entries.insert(
            key.to_string(),
            CacheEntry {
                content: content.to_string(),
                html: html.to_string(),
                created_at: Instant::now(),
                size: content.len(),
            },
        );

        Ok(())
    }

    /// Nettoie les entrées expirées
    fn cleanup_old_entries(&self, entries: &mut HashMap<String, CacheEntry>) {
        let now = Instant::now();
        entries.retain(|_, entry| {
            now.duration_since(entry.created_at).as_secs() <= self.config.cache_ttl
        });
    }

    /// Nettoie toutes les entrées du cache
    pub fn clear(&self) -> RenderingResult<()> {
        let mut entries = self.entries.write().map_err(|e| {
            RenderingError::CacheError(format!("Failed to acquire write lock: {}", e))
        })?;
        entries.clear();
        Ok(())
    }

    /// Retourne le nombre d'entrées dans le cache
    pub fn len(&self) -> usize {
        self.entries.read().map_or(0, |e| e.len())
    }

    /// Vérifie si le cache est vide
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Pipeline de rendu Markdown -> HTML
pub struct RenderingPipeline {
    /// Configuration du rendu
    config: RenderingConfig,
    /// Cache de rendu
    cache: Option<RenderingCache>,
    /// Options pulldown-cmark
    pulldown_options: Options,
}

impl RenderingPipeline {
    pub fn new(config: RenderingConfig) -> Self {
        let mut options = Options::empty();

        // Activer les extensions supportées
        for ext in &config.extensions {
            match ext.to_lowercase().as_str() {
                "tables" => options.insert(Options::ENABLE_TABLES),
                "task_lists" => options.insert(Options::ENABLE_TASKLISTS),
                "strikethrough" => options.insert(Options::ENABLE_STRIKETHROUGH),
                "footnotes" => options.insert(Options::ENABLE_FOOTNOTES),
                "heading_attributes" => options.insert(Options::ENABLE_HEADING_ATTRIBUTES),
                _ => {}
            };
        }

        let cache = if config.use_cache {
            Some(RenderingCache::new(config.clone()))
        } else {
            None
        };

        Self {
            config,
            cache,
            pulldown_options: options,
        }
    }

    pub fn with_config(config: RenderingConfig) -> Self {
        Self::new(config)
    }

    /// Rendu de base Markdown -> HTML
    pub fn render_markdown(&self, content: &str) -> RenderingResult<String> {
        // Vérifier le cache
        if let Some(ref cache) = self.cache {
            if let Some(html) = cache.get(content) {
                return Ok(html);
            }
        }

        // Rendu avec pulldown-cmark
        let parser = Parser::new_ext(content, self.pulldown_options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        // Stocker dans le cache
        if let Some(ref cache) = self.cache {
            cache.set(content, content, &html_output)?;
        }

        Ok(html_output)
    }

    /// Rendu avec optimisation
    pub fn render_optimized(&self, content: &str) -> RenderingResult<String> {
        let html = self.render_markdown(content)?;

        if self.config.optimize {
            Ok(self.optimize_html(&html))
        } else {
            Ok(html)
        }
    }

    /// Optimisation du HTML
    fn optimize_html(&self, html: &str) -> String {
        // Optimisations basiques du HTML
        let mut optimized = html.to_string();

        // Supprimer les espaces superflus
        optimized = optimized.replace(">\n<", "><");
        optimized = optimized.replace("  ", " ");

        // Supprimer les nouveaux sauts de ligne superflus
        while optimized.contains("\n\n\n") {
            optimized = optimized.replace("\n\n\n", "\n\n");
        }

        optimized
    }

    /// Rendu avec gestion du cache explicite
    pub fn render_with_cache_key(&self, key: &str, content: &str) -> RenderingResult<String> {
        // Vérifier le cache
        if let Some(ref cache) = self.cache {
            if let Some(html) = cache.get(key) {
                return Ok(html);
            }
        }

        // Rendu
        let html = self.render_markdown(content)?;

        // Stocker dans le cache
        if let Some(ref cache) = self.cache {
            cache.set(key, content, &html)?;
        }

        Ok(html)
    }

    /// Rendu d'un éditeur Markdown
    pub fn render_editor(&self, editor: &MarkdownEditor) -> RenderingResult<String> {
        self.render_markdown(editor.get_content())
    }

    /// Rendu incrémental (seulement les parties modifiées)
    pub fn render_incremental(
        &self,
        _old_content: &str,
        new_content: &str,
    ) -> RenderingResult<(String, Vec<usize>)> {
        // Pour l'instant, on rend tout le contenu
        // Une implémentation plus sophistiquée pourrait identifier
        // les parties modifiées et ne rendre que celles-ci
        let html = self.render_markdown(new_content)?;

        // Retourner le HTML et les positions modifiées (tout pour l'instant)
        Ok((html, vec![]))
    }

    /// Rendu par lots (plusieurs contenus)
    pub fn render_batch(&self, contents: &[&str]) -> RenderingResult<Vec<String>> {
        let mut results = Vec::with_capacity(contents.len());

        for &content in contents {
            results.push(self.render_markdown(content)?);
        }

        Ok(results)
    }

    /// Rendu avec des options personnalisées
    pub fn render_with_options(&self, content: &str, options: Options) -> RenderingResult<String> {
        let parser = Parser::new_ext(content, options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        Ok(html_output)
    }

    /// Vérifie si une extension est supportée
    pub fn is_extension_supported(&self, extension: &str) -> bool {
        let supported = [
            "tables",
            "task_lists",
            "strikethrough",
            "footnotes",
            "heading_attributes",
        ];
        supported.contains(&extension.to_lowercase().as_str())
    }

    /// Active une extension
    pub fn enable_extension(&mut self, extension: &str) -> RenderingResult<()> {
        if !self.is_extension_supported(extension) {
            return Err(RenderingError::ExtensionNotSupported(extension.to_string()));
        }

        if !self.config.extensions.contains(&extension.to_string()) {
            self.config.extensions.push(extension.to_string());

            // Mettre à jour les options pulldown-cmark
            match extension.to_lowercase().as_str() {
                "tables" => self.pulldown_options.insert(Options::ENABLE_TABLES),
                "task_lists" => self.pulldown_options.insert(Options::ENABLE_TASKLISTS),
                "strikethrough" => self.pulldown_options.insert(Options::ENABLE_STRIKETHROUGH),
                "footnotes" => self.pulldown_options.insert(Options::ENABLE_FOOTNOTES),
                "heading_attributes" => self
                    .pulldown_options
                    .insert(Options::ENABLE_HEADING_ATTRIBUTES),
                _ => {}
            };
        }

        Ok(())
    }

    /// Désactive une extension
    pub fn disable_extension(&mut self, extension: &str) -> RenderingResult<()> {
        if self.is_extension_supported(extension) {
            if let Some(pos) = self.config.extensions.iter().position(|e| e == extension) {
                self.config.extensions.remove(pos);
            }

            // Mettre à jour les options pulldown-cmark
            match extension.to_lowercase().as_str() {
                "tables" => self.pulldown_options.remove(Options::ENABLE_TABLES),
                "task_lists" => self.pulldown_options.remove(Options::ENABLE_TASKLISTS),
                "strikethrough" => self.pulldown_options.remove(Options::ENABLE_STRIKETHROUGH),
                "footnotes" => self.pulldown_options.remove(Options::ENABLE_FOOTNOTES),
                "heading_attributes" => self
                    .pulldown_options
                    .remove(Options::ENABLE_HEADING_ATTRIBUTES),
                _ => {}
            };
        }

        Ok(())
    }

    /// Retourne la configuration actuelle
    pub fn config(&self) -> &RenderingConfig {
        &self.config
    }

    /// Met à jour la configuration
    pub fn set_config(&mut self, config: RenderingConfig) {
        self.config = config;

        // Reconstruire les options pulldown-cmark
        self.pulldown_options = Options::empty();
        for ext in &self.config.extensions {
            match ext.to_lowercase().as_str() {
                "tables" => self.pulldown_options.insert(Options::ENABLE_TABLES),
                "task_lists" => self.pulldown_options.insert(Options::ENABLE_TASKLISTS),
                "strikethrough" => self.pulldown_options.insert(Options::ENABLE_STRIKETHROUGH),
                "footnotes" => self.pulldown_options.insert(Options::ENABLE_FOOTNOTES),
                "heading_attributes" => self
                    .pulldown_options
                    .insert(Options::ENABLE_HEADING_ATTRIBUTES),
                _ => {}
            };
        }
    }

    /// Retourne le cache (si activé)
    pub fn cache(&self) -> Option<&RenderingCache> {
        self.cache.as_ref()
    }

    /// Efface le cache
    pub fn clear_cache(&self) -> RenderingResult<()> {
        if let Some(ref cache) = self.cache {
            cache.clear()?;
        }
        Ok(())
    }
}

/// Implémentation par défaut du pipeline
impl Default for RenderingPipeline {
    fn default() -> Self {
        Self::new(RenderingConfig::default())
    }
}

/// Builder pour RenderingConfig
pub struct RenderingConfigBuilder {
    config: RenderingConfig,
}

impl Default for RenderingConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderingConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: RenderingConfig::default(),
        }
    }

    pub fn use_cache(mut self, use_cache: bool) -> Self {
        self.config.use_cache = use_cache;
        self
    }

    pub fn cache_size(mut self, size: usize) -> Self {
        self.config.cache_size = size;
        self
    }

    pub fn cache_ttl(mut self, ttl: u64) -> Self {
        self.config.cache_ttl = ttl;
        self
    }

    pub fn optimize(mut self, optimize: bool) -> Self {
        self.config.optimize = optimize;
        self
    }

    pub fn add_extension(mut self, extension: &str) -> Self {
        self.config.extensions.push(extension.to_string());
        self
    }

    pub fn extensions(mut self, extensions: Vec<&str>) -> Self {
        self.config.extensions = extensions.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn build(self) -> RenderingConfig {
        self.config
    }
}

/// Type alias pour le trait object du post-processeur
pub type HtmlProcessor = Box<dyn Fn(&str) -> String + Send + Sync>;

/// Post-processeur pour le HTML
pub struct HtmlPostProcessor {
    /// Fonctions de post-processing
    processors: Vec<HtmlProcessor>,
}

impl HtmlPostProcessor {
    pub fn new() -> Self {
        Self {
            processors: Vec::new(),
        }
    }

    /// Ajoute un post-processeur
    pub fn add_processor<F>(&mut self, processor: F)
    where
        F: Fn(&str) -> String + 'static + Send + Sync,
    {
        self.processors.push(Box::new(processor));
    }

    /// Applique tous les post-processeurs
    pub fn process(&self, html: &str) -> String {
        let mut result = html.to_string();
        for processor in &self.processors {
            result = processor(&result);
        }
        result
    }
}

impl Default for HtmlPostProcessor {
    fn default() -> Self {
        Self::new()
    }
}

/// Intégration avec MarkdownEditor
impl MarkdownEditor {
    /// Rendu avec un pipeline de rendu personnalisé
    pub fn render_with_pipeline(&self, pipeline: &RenderingPipeline) -> RenderingResult<String> {
        pipeline.render_editor(self)
    }

    /// Rendu avec cache
    pub fn render_with_cache(
        &self,
        pipeline: &RenderingPipeline,
        cache_key: &str,
    ) -> RenderingResult<String> {
        pipeline.render_with_cache_key(cache_key, self.get_content())
    }
}

/// Pipeline de rendu global (pour usage simple)
static GLOBAL_PIPELINE: OnceLock<RenderingPipeline> = OnceLock::new();

/// Fonction de rendu global simple
pub fn render_markdown(content: &str) -> RenderingResult<String> {
    let pipeline = GLOBAL_PIPELINE.get_or_init(RenderingPipeline::default);
    pipeline.render_markdown(content)
}

/// Fonction de rendu global avec cache
pub fn render_markdown_cached(content: &str) -> RenderingResult<String> {
    // Utiliser le hash du contenu comme clé de cache
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let pipeline = GLOBAL_PIPELINE.get_or_init(RenderingPipeline::default);

    let mut hasher = DefaultHasher::new();
    content.hash(&mut hasher);
    let hash = hasher.finish();
    let cache_key = format!("global_{}", hash);

    pipeline.render_with_cache_key(&cache_key, content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_markdown_simple() {
        let pipeline = RenderingPipeline::default();
        let result = pipeline.render_markdown("# Hello World").unwrap();
        assert!(result.contains("<h1>"));
        assert!(result.contains("Hello World"));
    }

    #[test]
    fn test_render_markdown_paragraph() {
        let pipeline = RenderingPipeline::default();
        let result = pipeline.render_markdown("This is a paragraph").unwrap();
        assert!(result.contains("<p>"));
        assert!(result.contains("This is a paragraph"));
    }

    #[test]
    fn test_render_markdown_bold() {
        let pipeline = RenderingPipeline::default();
        let result = pipeline.render_markdown("**Bold text**").unwrap();
        assert!(result.contains("<strong>"));
        assert!(result.contains("Bold text"));
    }

    #[test]
    fn test_render_markdown_italic() {
        let pipeline = RenderingPipeline::default();
        let result = pipeline.render_markdown("*Italic text*").unwrap();
        assert!(result.contains("<em>"));
        assert!(result.contains("Italic text"));
    }

    #[test]
    fn test_render_markdown_code() {
        let pipeline = RenderingPipeline::default();
        let result = pipeline.render_markdown("`code`").unwrap();
        assert!(result.contains("<code>"));
        assert!(result.contains("code"));
    }

    #[test]
    fn test_render_markdown_code_block() {
        let pipeline = RenderingPipeline::default();
        let result = pipeline.render_markdown("```\nfn main() {}\n```").unwrap();
        assert!(result.contains("<pre>"));
        assert!(result.contains("<code>"));
        assert!(result.contains("fn main() {}"));
    }

    #[test]
    fn test_render_markdown_list() {
        let pipeline = RenderingPipeline::default();
        let result = pipeline.render_markdown("- Item 1\n- Item 2").unwrap();
        assert!(result.contains("<ul>"));
        assert!(result.contains("<li>"));
        assert!(result.contains("Item 1"));
        assert!(result.contains("Item 2"));
    }

    #[test]
    fn test_render_markdown_link() {
        let pipeline = RenderingPipeline::default();
        let result = pipeline
            .render_markdown("[Link](https://example.com)")
            .unwrap();
        assert!(result.contains("<a"));
        assert!(result.contains("href=\"https://example.com\""));
        assert!(result.contains("Link"));
    }

    #[test]
    fn test_render_markdown_image() {
        let pipeline = RenderingPipeline::default();
        let result = pipeline.render_markdown("![Alt text](image.png)").unwrap();
        assert!(result.contains("<img"));
        assert!(result.contains("alt=\"Alt text\""));
        assert!(result.contains("src=\"image.png\""));
    }

    #[test]
    fn test_render_markdown_table() {
        let config = RenderingConfig {
            extensions: vec!["tables".to_string()],
            ..Default::default()
        };
        let pipeline = RenderingPipeline::with_config(config);

        let result = pipeline
            .render_markdown("| Header | Value |\n|--------|-------|\n| Row 1 | Data 1 |")
            .unwrap();
        assert!(result.contains("<table>"));
        assert!(result.contains("<th>"));
        assert!(result.contains("Header"));
        assert!(result.contains("Value"));
    }

    #[test]
    fn test_render_markdown_task_list() {
        let config = RenderingConfig {
            extensions: vec!["task_lists".to_string()],
            ..Default::default()
        };
        let pipeline = RenderingPipeline::with_config(config);

        let result = pipeline
            .render_markdown("- [x] Task 1\n- [ ] Task 2")
            .unwrap();
        assert!(result.contains("<input"));
        assert!(result.contains("checked"));
    }

    #[test]
    fn test_render_markdown_strikethrough() {
        let config = RenderingConfig {
            extensions: vec!["strikethrough".to_string()],
            ..Default::default()
        };
        let pipeline = RenderingPipeline::with_config(config);

        let result = pipeline.render_markdown("~~strikethrough~~").unwrap();
        assert!(result.contains("<del>"));
        assert!(result.contains("strikethrough"));
    }

    #[test]
    fn test_rendering_cache() {
        let config = RenderingConfig {
            use_cache: true,
            cache_size: 100,
            cache_ttl: 60,
            optimize: false,
            extensions: vec![],
        };
        let cache = RenderingCache::new(config);

        cache.set("test", "content", "<p>test</p>").unwrap();
        assert_eq!(cache.get("test"), Some("<p>test</p>".to_string()));
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_rendering_cache_expiration() {
        use std::time::Duration;

        let config = RenderingConfig {
            use_cache: true,
            cache_size: 100,
            cache_ttl: 1, // TTL de 1 seconde
            optimize: false,
            extensions: vec![],
        };
        let cache = RenderingCache::new(config);

        cache.set("test", "content", "<p>test</p>").unwrap();
        assert_eq!(cache.get("test"), Some("<p>test</p>".to_string()));

        // Attendre plus que le TTL
        std::thread::sleep(Duration::from_secs(2));
        assert_eq!(cache.get("test"), None);
    }

    #[test]
    fn test_rendering_cache_clear() {
        let config = RenderingConfig::default();
        let cache = RenderingCache::new(config);

        cache.set("test1", "content1", "<p>test1</p>").unwrap();
        cache.set("test2", "content2", "<p>test2</p>").unwrap();
        assert_eq!(cache.len(), 2);

        cache.clear().unwrap();
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn test_rendering_config_builder() {
        let config = RenderingConfigBuilder::new()
            .use_cache(true)
            .cache_size(500)
            .cache_ttl(600)
            .optimize(true)
            .add_extension("tables")
            .add_extension("task_lists")
            .build();

        assert!(config.use_cache);
        assert_eq!(config.cache_size, 500);
        assert_eq!(config.cache_ttl, 600);
        assert!(config.optimize);
        assert!(config.extensions.contains(&"tables".to_string()));
        assert!(config.extensions.contains(&"task_lists".to_string()));
    }

    #[test]
    fn test_pipeline_enable_extension() {
        let mut pipeline = RenderingPipeline::default();
        pipeline.enable_extension("tables").unwrap();

        assert!(pipeline.is_extension_supported("tables"));
    }

    #[test]
    fn test_pipeline_disable_extension() {
        let config = RenderingConfig {
            extensions: vec!["tables".to_string()],
            ..Default::default()
        };
        let mut pipeline = RenderingPipeline::with_config(config);

        pipeline.disable_extension("tables").unwrap();
        assert!(!pipeline.config.extensions.contains(&"tables".to_string()));
    }

    #[test]
    fn test_pipeline_enable_unsupported_extension() {
        let mut pipeline = RenderingPipeline::default();
        let result = pipeline.enable_extension("unsupported");
        assert!(matches!(
            result,
            Err(RenderingError::ExtensionNotSupported(_))
        ));
    }

    #[test]
    fn test_pipeline_render_editor() {
        let pipeline = RenderingPipeline::default();
        let mut editor = MarkdownEditor::new();
        editor.set_content("# Title\n\nContent".to_string());

        let result = pipeline.render_editor(&editor).unwrap();
        assert!(result.contains("<h1>"));
        assert!(result.contains("Title"));
        assert!(result.contains("<p>"));
        assert!(result.contains("Content"));
    }

    #[test]
    fn test_pipeline_render_batch() {
        let pipeline = RenderingPipeline::default();
        let contents = ["# Title 1", "# Title 2", "# Title 3"];

        let results = pipeline.render_batch(&contents).unwrap();
        assert_eq!(results.len(), 3);
        for result in results {
            assert!(result.contains("<h1>"));
        }
    }

    #[test]
    fn test_html_post_processor() {
        let mut processor = HtmlPostProcessor::new();

        // Ajouter un processeur qui remplace &nbsp; par une espace normale
        processor.add_processor(|html| html.replace("&nbsp;", " "));

        let input = "Text with &nbsp; non-breaking space";
        let result = processor.process(input);
        assert_eq!(result, "Text with   non-breaking space");
    }

    #[test]
    fn test_global_render() {
        let result = render_markdown("# Global Test").unwrap();
        assert!(result.contains("<h1>"));
        assert!(result.contains("Global Test"));
    }

    #[test]
    fn test_global_render_cached() {
        let result1 = render_markdown_cached("# Cached Test").unwrap();
        let result2 = render_markdown_cached("# Cached Test").unwrap();

        assert_eq!(result1, result2);
    }

    #[test]
    fn test_optimize_html() {
        let pipeline = RenderingPipeline::default();
        let html = "<p>Text</p>\n\n\n<p>More text</p>";
        let optimized = pipeline.optimize_html(html);

        // Vérifier que les multiples sauts de ligne sont réduits
        assert!(!optimized.contains("\n\n\n"));
    }

    #[test]
    fn test_pipeline_with_cache_key() {
        let pipeline = RenderingPipeline::default();
        let result1 = pipeline.render_with_cache_key("key1", "# Test").unwrap();
        let result2 = pipeline.render_with_cache_key("key1", "# Test").unwrap();

        assert_eq!(result1, result2);
    }

    #[test]
    fn test_clear_cache() {
        let pipeline = RenderingPipeline::default();
        pipeline.render_markdown("# Test 1").unwrap();
        pipeline.render_markdown("# Test 2").unwrap();

        if let Some(cache) = pipeline.cache() {
            assert!(!cache.is_empty());
            pipeline.clear_cache().unwrap();
            assert!(cache.is_empty());
        }
    }

    #[test]
    fn test_pipeline_config() {
        let pipeline = RenderingPipeline::default();
        let config = pipeline.config();

        assert!(config.use_cache);
        assert!(config.optimize);
        assert!(!config.extensions.is_empty());
    }

    #[test]
    fn test_pipeline_set_config() {
        let mut pipeline = RenderingPipeline::default();
        let new_config = RenderingConfig {
            use_cache: false,
            cache_size: 200,
            cache_ttl: 120,
            optimize: false,
            extensions: vec![],
        };

        pipeline.set_config(new_config);
        assert!(!pipeline.config().use_cache);
        assert!(!pipeline.config().optimize);
    }
}
