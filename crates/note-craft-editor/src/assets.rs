//! Asset Management Module
//!
//! Ce module fournit le support pour la gestion des assets (images, fichiers) avec:
//! - Upload et stockage des assets
//! - Génération de previews (base64 pour les images)
//! - Gestion des métadonnées
//! - Support du drag and drop

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

/// Type d'asset
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AssetType {
    /// Image (PNG, JPEG, GIF, WebP, SVG)
    Image,
    /// Document PDF
    Pdf,
    /// Vidéo
    Video,
    /// Audio
    Audio,
    /// Autre fichier
    Other,
}

impl std::fmt::Display for AssetType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssetType::Image => write!(f, "image"),
            AssetType::Pdf => write!(f, "pdf"),
            AssetType::Video => write!(f, "video"),
            AssetType::Audio => write!(f, "audio"),
            AssetType::Other => write!(f, "other"),
        }
    }
}

impl AssetType {
    /// Détermine le type d'asset à partir de l'extension
    pub fn from_extension(ext: &str) -> Self {
        let ext_lower = ext.to_lowercase();
        match ext_lower.as_str() {
            "png" | "jpg" | "jpeg" | "gif" | "webp" | "svg" | "bmp" | "ico" => AssetType::Image,
            "pdf" => AssetType::Pdf,
            "mp4" | "webm" | "mov" | "avi" | "mkv" => AssetType::Video,
            "mp3" | "wav" | "ogg" | "flac" | "aac" => AssetType::Audio,
            _ => AssetType::Other,
        }
    }

    /// Obtient les extensions supportées pour ce type
    pub fn extensions(&self) -> &'static [&'static str] {
        match self {
            AssetType::Image => &["png", "jpg", "jpeg", "gif", "webp", "svg", "bmp", "ico"],
            AssetType::Pdf => &["pdf"],
            AssetType::Video => &["mp4", "webm", "mov", "avi", "mkv"],
            AssetType::Audio => &["mp3", "wav", "ogg", "flac", "aac"],
            AssetType::Other => &[],
        }
    }

    /// Vérifie si le type est une image
    pub fn is_image(&self) -> bool {
        matches!(self, AssetType::Image)
    }
}

/// Métadonnées d'un asset
#[derive(Debug, Clone)]
pub struct AssetMetadata {
    /// Nom original du fichier
    pub original_name: String,
    /// Nom stocké (avec hash pour éviter les conflits)
    pub stored_name: String,
    /// Type d'asset
    pub asset_type: AssetType,
    /// Extension du fichier
    pub extension: String,
    /// Taille en octets
    pub size: u64,
    /// Chemin de stockage relatif
    pub path: PathBuf,
    /// URL relative pour accéder à l'asset
    pub url: String,
    /// Largeur (pour les images)
    pub width: Option<u32>,
    /// Hauteur (pour les images)
    pub height: Option<u32>,
    /// Date de création (timestamp)
    pub created_at: u64,
    /// Date de modification (timestamp)
    pub updated_at: u64,
    /// Tags optionnels
    pub tags: Vec<String>,
    /// Description optionnelle
    pub description: Option<String>,
}

impl AssetMetadata {
    /// Crée de nouvelles métadonnées avec une URL par défaut
    pub fn new(
        original_name: String,
        stored_name: String,
        asset_type: AssetType,
        extension: String,
        size: u64,
        path: PathBuf,
    ) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let url = format!("/assets/{}", stored_name);

        Self {
            original_name,
            stored_name,
            asset_type,
            extension,
            size,
            path,
            url,
            width: None,
            height: None,
            created_at: now,
            updated_at: now,
            tags: vec![],
            description: None,
        }
    }

    /// Crée de nouvelles métadonnées avec une URL personnalisée
    pub fn new_with_url(
        original_name: String,
        stored_name: String,
        asset_type: AssetType,
        extension: String,
        size: u64,
        path: PathBuf,
        url: String,
    ) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            original_name,
            stored_name,
            asset_type,
            extension,
            size,
            path,
            url,
            width: None,
            height: None,
            created_at: now,
            updated_at: now,
            tags: vec![],
            description: None,
        }
    }

    /// Ajoute un tag
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }

    /// Supprime un tag
    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.retain(|t| t != tag);
    }

    /// Définit la description
    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    /// Définit les dimensions (pour les images)
    pub fn set_dimensions(&mut self, width: u32, height: u32) {
        self.width = Some(width);
        self.height = Some(height);
    }

    /// Obtient l'extension avec le point
    pub fn extension_with_dot(&self) -> String {
        format!(".{}", self.extension)
    }

    /// Génère un nom de fichier unique avec hash
    pub fn generate_stored_name(original_name: &str) -> String {
        use std::hash::{DefaultHasher, Hasher};

        let mut hasher = DefaultHasher::new();
        hasher.write(original_name.as_bytes());
        hasher.write(
            &std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos()
                .to_be_bytes(),
        );

        let hash = format!("{:x}", hasher.finish());
        let ext = Path::new(original_name)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();

        format!("{}_{}", &hash[..16], ext)
    }
}

/// Représente un asset
#[derive(Debug, Clone)]
pub struct Asset {
    /// Métadonnées
    pub metadata: AssetMetadata,
    /// Contenu binaire (pour les petits fichiers ou le preview)
    pub content: Option<Vec<u8>>,
    /// Preview base64 (pour les images)
    pub preview: Option<String>,
}

impl Asset {
    /// Crée un nouvel asset
    pub fn new(metadata: AssetMetadata, content: Option<Vec<u8>>) -> Self {
        Self {
            metadata,
            content,
            preview: None,
        }
    }

    /// Génère un preview base64 (pour les images)
    pub fn generate_preview(&mut self) -> Option<String> {
        if !self.metadata.asset_type.is_image() {
            return None;
        }

        if let Some(ref content) = self.content {
            // Pour les petits fichiers, nous pouvons générer un preview base64
            if content.len() < 10 * 1024 * 1024 {
                // 10MB max
                use base64::Engine;
                let base64 = base64::engine::general_purpose::STANDARD.encode(content);
                let data_uri = match self.metadata.extension.to_lowercase().as_str() {
                    "png" => format!("data:image/png;base64,{}", base64),
                    "jpg" | "jpeg" => format!("data:image/jpeg;base64,{}", base64),
                    "gif" => format!("data:image/gif;base64,{}", base64),
                    "webp" => format!("data:image/webp;base64,{}", base64),
                    "svg" => format!("data:image/svg+xml;base64,{}", base64),
                    _ => format!("data:image/{};base64,{}", self.metadata.extension, base64),
                };
                self.preview = Some(data_uri);
                return Some(self.preview.clone().unwrap());
            }
        }
        None
    }

    /// Obtient le preview ou le chemin de l'asset
    pub fn get_display_url(&self) -> String {
        self.preview
            .clone()
            .unwrap_or_else(|| self.metadata.url.clone())
    }

    /// Vérifie si l'asset a un preview
    pub fn has_preview(&self) -> bool {
        self.preview.is_some()
    }
}

/// Gestionnaire des assets
pub struct AssetManager {
    /// Répertoire de stockage des assets
    storage_path: PathBuf,
    /// URL base pour accéder aux assets
    base_url: String,
    /// Cache des assets chargés
    assets: Arc<RwLock<HashMap<String, Asset>>>,
    /// Compteur d'assets
    asset_count: Arc<RwLock<u64>>,
}

impl AssetManager {
    /// Crée un nouveau gestionnaire d'assets
    pub fn new(storage_path: impl AsRef<Path>, base_url: &str) -> Self {
        let storage_path = storage_path.as_ref().to_path_buf();

        // Créer le répertoire de stockage s'il n'existe pas
        if let Some(parent) = storage_path.parent() {
            std::fs::create_dir_all(parent).ok();
        }

        Self {
            storage_path,
            base_url: base_url.to_string(),
            assets: Arc::new(RwLock::new(HashMap::new())),
            asset_count: Arc::new(RwLock::new(0)),
        }
    }

    /// Upload un asset à partir de son contenu
    pub fn upload(&self, filename: &str, content: Vec<u8>) -> Result<Asset, AssetError> {
        let asset_type = AssetType::from_extension(
            Path::new(filename)
                .extension()
                .and_then(|s| s.to_str())
                .unwrap_or(""),
        );

        let extension = Path::new(filename)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();

        let stored_name = AssetMetadata::generate_stored_name(filename);
        let path = self.storage_path.join(&stored_name);

        // Générer l'URL complète
        let url = if self.base_url.ends_with('/') {
            format!("{}{}", self.base_url, stored_name)
        } else {
            format!("{}/{}", self.base_url, stored_name)
        };

        // Sauvegarder le fichier
        std::fs::write(&path, &content)
            .map_err(|e| AssetError::StorageError(format!("Failed to write asset: {}", e)))?;

        let metadata = AssetMetadata::new_with_url(
            filename.to_string(),
            stored_name.clone(),
            asset_type,
            extension,
            content.len() as u64,
            path,
            url,
        );

        let mut asset = Asset::new(metadata, Some(content));

        // Générer un preview pour les images
        if asset_type.is_image() {
            asset.generate_preview();
        }

        // Ajouter au cache
        {
            let mut assets = self.assets.write().unwrap();
            assets.insert(stored_name.clone(), asset.clone());
        }

        // Incrémenter le compteur
        {
            let mut count = self.asset_count.write().unwrap();
            *count += 1;
        }

        Ok(asset)
    }

    /// Upload un asset à partir d'un chemin de fichier
    pub fn upload_from_path(&self, file_path: impl AsRef<Path>) -> Result<Asset, AssetError> {
        let file_path = file_path.as_ref();
        let filename = file_path
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or(AssetError::InvalidFilename)?;

        let content = std::fs::read(file_path)
            .map_err(|e| AssetError::StorageError(format!("Failed to read file: {}", e)))?;

        self.upload(filename, content)
    }

    /// Obtient un asset par son nom stocké
    pub fn get(&self, stored_name: &str) -> Option<Asset> {
        let assets = self.assets.read().unwrap();
        assets.get(stored_name).cloned()
    }

    /// Supprime un asset
    pub fn delete(&self, stored_name: &str) -> Result<(), AssetError> {
        let mut assets = self.assets.write().unwrap();
        if let Some(asset) = assets.remove(stored_name) {
            // Supprimer le fichier physique
            std::fs::remove_file(&asset.metadata.path).ok();
            Ok(())
        } else {
            Err(AssetError::NotFound(stored_name.to_string()))
        }
    }

    /// Liste tous les assets
    pub fn list(&self) -> Vec<Asset> {
        let assets = self.assets.read().unwrap();
        assets.values().cloned().collect()
    }

    /// Liste les assets par type
    pub fn list_by_type(&self, asset_type: AssetType) -> Vec<Asset> {
        let assets = self.assets.read().unwrap();
        assets
            .values()
            .filter(|a| a.metadata.asset_type == asset_type)
            .cloned()
            .collect()
    }

    /// Liste les assets par tag
    pub fn list_by_tag(&self, tag: &str) -> Vec<Asset> {
        let assets = self.assets.read().unwrap();
        assets
            .values()
            .filter(|a| a.metadata.tags.contains(&tag.to_string()))
            .cloned()
            .collect()
    }

    /// Compte le nombre d'assets
    pub fn count(&self) -> u64 {
        let count = self.asset_count.read().unwrap();
        *count
    }

    /// Nettoie les assets non utilisés (à implémenter selon la logique métier)
    pub fn cleanup_unused(&self) -> Result<u64, AssetError> {
        // Pour l'instant, on ne fait rien
        Ok(0)
    }

    /// Génère le HTML pour afficher un asset
    pub fn render_html(&self, asset: &Asset) -> String {
        match asset.metadata.asset_type {
            AssetType::Image => self.render_image_html(asset),
            AssetType::Pdf => self.render_pdf_html(asset),
            AssetType::Video => self.render_video_html(asset),
            AssetType::Audio => self.render_audio_html(asset),
            AssetType::Other => self.render_file_html(asset),
        }
    }

    /// Génère le HTML pour une image
    fn render_image_html(&self, asset: &Asset) -> String {
        let url = asset.get_display_url();
        let alt = asset.metadata.original_name.clone();

        if let (Some(width), Some(height)) = (asset.metadata.width, asset.metadata.height) {
            format!(
                "<img src=\"{}\" alt=\"{}\" width=\"{}\" height=\"{}\" class=\"asset-image\" />",
                url, alt, width, height
            )
        } else {
            format!(
                "<img src=\"{}\" alt=\"{}\" class=\"asset-image\" />",
                url, alt
            )
        }
    }

    /// Génère le HTML pour un PDF
    fn render_pdf_html(&self, asset: &Asset) -> String {
        let url = asset.metadata.url.clone();
        format!(
            "<a href=\"{}\" class=\"asset-pdf\" target=\"_blank\">PDF: {}</a>",
            url, asset.metadata.original_name
        )
    }

    /// Génère le HTML pour une vidéo
    fn render_video_html(&self, asset: &Asset) -> String {
        let url = asset.metadata.url.clone();
        format!(
            "<video src=\"{}\" controls class=\"asset-video\"></video>",
            url
        )
    }

    /// Génère le HTML pour un audio
    fn render_audio_html(&self, asset: &Asset) -> String {
        let url = asset.metadata.url.clone();
        format!(
            "<audio src=\"{}\" controls class=\"asset-audio\"></audio>",
            url
        )
    }

    /// Génère le HTML pour un fichier générique
    fn render_file_html(&self, asset: &Asset) -> String {
        let url = asset.metadata.url.clone();
        format!(
            "<a href=\"{}\" class=\"asset-file\" download>Download: {}</a>",
            url, asset.metadata.original_name
        )
    }

    /// Génère le CSS pour les assets
    pub fn generate_css() -> String {
        r#"
/* Asset styling */
.asset-image {
    max-width: 100%;
    height: auto;
    display: block;
    margin: 1em 0;
    border-radius: 4px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.asset-pdf {
    display: inline-block;
    padding: 0.5em 1em;
    background: #f0f0f0;
    border-radius: 4px;
    color: #333;
    text-decoration: none;
    margin: 0.5em 0;
}

.asset-video {
    max-width: 100%;
    display: block;
    margin: 1em 0;
    border-radius: 4px;
}

.asset-audio {
    max-width: 100%;
    display: block;
    margin: 1em 0;
}

.asset-file {
    display: inline-block;
    padding: 0.5em 1em;
    background: #f0f0f0;
    border-radius: 4px;
    color: #333;
    text-decoration: none;
    margin: 0.5em 0;
}

.asset-drag-over {
    border: 2px dashed #4CAF50;
    background: rgba(76, 175, 80, 0.1);
    padding: 2em;
    text-align: center;
    cursor: pointer;
}

.asset-preview {
    max-width: 200px;
    max-height: 200px;
    object-fit: contain;
    margin: 0.5em;
}
"#
        .to_string()
    }

    /// Génère le JavaScript pour le drag and drop
    pub fn generate_drag_drop_js() -> String {
        r#"
function setupAssetDragDrop(selector, uploadCallback) {
    const dropZone = document.querySelector(selector);
    if (!dropZone) return;

    dropZone.addEventListener('dragover', (e) => {
        e.preventDefault();
        dropZone.classList.add('asset-drag-over');
    });

    dropZone.addEventListener('dragleave', (e) => {
        e.preventDefault();
        dropZone.classList.remove('asset-drag-over');
    });

    dropZone.addEventListener('drop', (e) => {
        e.preventDefault();
        dropZone.classList.remove('asset-drag-over');

        const files = e.dataTransfer.files;
        if (files.length > 0) {
            for (let i = 0; i < files.length; i++) {
                const file = files[i];
                // Appeler le callback avec le fichier
                if (uploadCallback) {
                    uploadCallback(file);
                }
            }
        }
    });

    // Also allow click to select files
    dropZone.addEventListener('click', () => {
        const input = document.createElement('input');
        input.type = 'file';
        input.multiple = true;
        input.accept = 'image/*,video/*,audio/*,.pdf';
        input.addEventListener('change', (e) => {
            const files = e.target.files;
            if (files.length > 0 && uploadCallback) {
                for (let i = 0; i < files.length; i++) {
                    uploadCallback(files[i]);
                }
            }
        });
        input.click();
    });
}
"#
        .to_string()
    }
}

/// Erreurs d'asset
#[derive(Debug, Clone)]
pub enum AssetError {
    /// Fichier introuvable
    NotFound(String),
    /// Nom de fichier invalide
    InvalidFilename,
    /// Erreur de stockage
    StorageError(String),
    /// Type non supporté
    UnsupportedType(String),
    /// Fichier trop grand
    FileTooLarge(u64),
}

impl std::fmt::Display for AssetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssetError::NotFound(name) => write!(f, "Asset not found: {}", name),
            AssetError::InvalidFilename => write!(f, "Invalid filename"),
            AssetError::StorageError(msg) => write!(f, "Storage error: {}", msg),
            AssetError::UnsupportedType(typ) => write!(f, "Unsupported type: {}", typ),
            AssetError::FileTooLarge(size) => write!(f, "File too large: {} bytes", size),
        }
    }
}

impl std::error::Error for AssetError {}

/// Gestionnaire d'assets par défaut (thread-safe)
pub type DefaultAssetManager = AssetManager;

/// Tests unitaires
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asset_type_from_extension() {
        assert_eq!(AssetType::from_extension("png"), AssetType::Image);
        assert_eq!(AssetType::from_extension("jpg"), AssetType::Image);
        assert_eq!(AssetType::from_extension("pdf"), AssetType::Pdf);
        assert_eq!(AssetType::from_extension("mp4"), AssetType::Video);
        assert_eq!(AssetType::from_extension("mp3"), AssetType::Audio);
        assert_eq!(AssetType::from_extension("txt"), AssetType::Other);
    }

    #[test]
    fn test_asset_type_is_image() {
        assert!(AssetType::Image.is_image());
        assert!(!AssetType::Pdf.is_image());
    }

    #[test]
    fn test_asset_type_extensions() {
        let image_exts = AssetType::Image.extensions();
        assert!(image_exts.contains(&"png"));
        assert!(image_exts.contains(&"jpg"));
        assert!(image_exts.contains(&"gif"));
    }

    #[test]
    fn test_metadata_generate_stored_name() {
        let name1 = AssetMetadata::generate_stored_name("test1.png");
        let name2 = AssetMetadata::generate_stored_name("test2.png");
        // Les noms devraient être différents car les fichiers sont différents
        // Même avec le même timestamp, le hash du nom diffère
        assert_ne!(name1, name2);
    }

    #[test]
    fn test_metadata_new() {
        let metadata = AssetMetadata::new(
            "test.png".to_string(),
            "abc123.png".to_string(),
            AssetType::Image,
            "png".to_string(),
            1024,
            PathBuf::from("/assets/abc123.png"),
        );
        assert_eq!(metadata.original_name, "test.png");
        assert_eq!(metadata.stored_name, "abc123.png");
        assert_eq!(metadata.asset_type, AssetType::Image);
        assert_eq!(metadata.size, 1024);
    }

    #[test]
    fn test_asset_new() {
        let metadata = AssetMetadata::new(
            "test.png".to_string(),
            "abc123.png".to_string(),
            AssetType::Image,
            "png".to_string(),
            1024,
            PathBuf::from("/assets/abc123.png"),
        );
        let content = vec![1, 2, 3, 4];
        let asset = Asset::new(metadata, Some(content.clone()));
        assert_eq!(asset.content, Some(content));
        assert!(asset.preview.is_none());
    }

    #[test]
    fn test_asset_manager_new() {
        let manager = AssetManager::new("/tmp/assets", "http://localhost:3000");
        assert!(manager.storage_path.ends_with("assets"));
    }

    #[test]
    fn test_asset_manager_upload() {
        let test_dir = format!("/tmp/test_assets_{}", std::process::id());
        let manager = AssetManager::new(&test_dir, "http://localhost:3000");
        std::fs::create_dir_all(&test_dir).ok();

        let content = b"test image content";
        let result = manager.upload("test.png", content.to_vec());

        assert!(result.is_ok());
        let asset = result.unwrap();
        assert_eq!(asset.metadata.original_name, "test.png");
        assert_eq!(asset.metadata.asset_type, AssetType::Image);
        // Cleanup
        std::fs::remove_dir_all(&test_dir).ok();
    }

    #[test]
    fn test_asset_manager_get() {
        let test_dir = format!("/tmp/test_assets_get_{}", std::process::id());
        let manager = AssetManager::new(&test_dir, "http://localhost:3000");
        std::fs::create_dir_all(&test_dir).ok();

        let content = b"test image content";
        let asset = manager.upload("test2.png", content.to_vec()).unwrap();

        let retrieved = manager.get(&asset.metadata.stored_name);
        assert!(retrieved.is_some());
        assert_eq!(
            retrieved.unwrap().metadata.stored_name,
            asset.metadata.stored_name
        );
        // Cleanup
        std::fs::remove_dir_all(&test_dir).ok();
    }

    #[test]
    fn test_asset_manager_list() {
        let test_dir = format!("/tmp/test_assets_list_{}", std::process::id());
        let manager = AssetManager::new(&test_dir, "http://localhost:3000");
        std::fs::create_dir_all(&test_dir).ok();

        manager.upload("test1.png", b"content1".to_vec()).ok();
        manager.upload("test2.png", b"content2".to_vec()).ok();

        let assets = manager.list();
        assert!(assets.len() >= 2);
        // Cleanup
        std::fs::remove_dir_all(&test_dir).ok();
    }

    #[test]
    fn test_asset_manager_count() {
        let test_dir = format!("/tmp/test_assets_count_{}", std::process::id());
        let manager = AssetManager::new(&test_dir, "http://localhost:3000");
        std::fs::create_dir_all(&test_dir).ok();

        let initial_count = manager.count();
        manager.upload("count_test.png", b"content".to_vec()).ok();
        let new_count = manager.count();
        assert_eq!(new_count, initial_count + 1);
        // Cleanup
        std::fs::remove_dir_all(&test_dir).ok();
    }

    #[test]
    fn test_asset_render_html() {
        let manager = AssetManager::new("/tmp/test_assets", "http://localhost:3000");
        std::fs::create_dir_all("/tmp/test_assets").ok();

        let asset = manager
            .upload("render_test.png", b"image data".to_vec())
            .unwrap();
        let html = manager.render_html(&asset);

        assert!(html.contains("img"));
        assert!(html.contains("asset-image"));
    }

    #[test]
    fn test_asset_generate_css() {
        let css = AssetManager::generate_css();
        assert!(css.contains(".asset-image"));
        assert!(css.contains(".asset-pdf"));
    }

    #[test]
    fn test_asset_generate_drag_drop_js() {
        let js = AssetManager::generate_drag_drop_js();
        assert!(js.contains("setupAssetDragDrop"));
        assert!(js.contains("dragover"));
        assert!(js.contains("drop"));
    }

    #[test]
    fn test_asset_display() {
        let error = AssetError::NotFound("test.png".to_string());
        assert_eq!(format!("{}", error), "Asset not found: test.png");
    }
}
