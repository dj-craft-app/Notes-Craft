//! Note Craft File System Abstraction
//!
//! Ce module fournit une abstraction du système de fichiers pour gérer
//! les notes, pages et assets de manière portable et testable.
//!
//! Phase 1 - Étape 1.1: File System Abstraction

use std::path::{Path, PathBuf};
use thiserror::Error;

/// Erreurs liées au système de fichiers
#[derive(Error, Debug)]
pub enum FsError {
    #[error("Fichier non trouvé: {0}")]
    NotFound(PathBuf),
    #[error("Erreur de lecture: {0}")]
    ReadError(String),
    #[error("Erreur d'écriture: {0}")]
    WriteError(String),
    #[error("Erreur de création de répertoire: {0}")]
    CreateDirError(String),
    #[error("Erreur de suppression: {0}")]
    RemoveError(String),
    #[error("Chemin invalide: {0}")]
    InvalidPath(String),
    #[error("Erreur IO: {0}")]
    IoError(#[from] std::io::Error),
}

/// Type de résultat pour les opérations du filesystem
pub type FsResult<T> = Result<T, FsError>;

/// Trait d'abstraction du système de fichiers
pub trait FileSystem {
    /// Vérifie si un fichier existe
    fn exists(&self, path: &Path) -> FsResult<bool>;

    /// Lit le contenu d'un fichier
    fn read_file(&self, path: &Path) -> FsResult<Vec<u8>>;

    /// Écrit le contenu dans un fichier
    fn write_file(&self, path: &Path, content: &[u8]) -> FsResult<()>;

    /// Crée un répertoire
    fn create_dir(&self, path: &Path) -> FsResult<()>;

    /// Crée un répertoire et tous ses parents
    fn create_dir_all(&self, path: &Path) -> FsResult<()>;

    /// Supprime un fichier
    fn remove_file(&self, path: &Path) -> FsResult<()>;

    /// Supprime un répertoire (vide)
    fn remove_dir(&self, path: &Path) -> FsResult<()>;

    /// Liste les entrées d'un répertoire
    fn read_dir(&self, path: &Path) -> FsResult<Vec<PathBuf>>;

    /// Obtient le chemin canonique (absolu)
    fn canonicalize(&self, path: &Path) -> FsResult<PathBuf>;

    /// Vérifie si un chemin est un répertoire
    fn is_dir(&self, path: &Path) -> FsResult<bool>;

    /// Vérifie si un chemin est un fichier
    fn is_file(&self, path: &Path) -> FsResult<bool>;
}

/// Implémentation du filesystem pour le système local
pub struct LocalFileSystem;

impl FileSystem for LocalFileSystem {
    fn exists(&self, path: &Path) -> FsResult<bool> {
        Ok(path.exists())
    }

    fn read_file(&self, path: &Path) -> FsResult<Vec<u8>> {
        Ok(std::fs::read(path)?)
    }

    fn write_file(&self, path: &Path, content: &[u8]) -> FsResult<()> {
        Ok(std::fs::write(path, content)?)
    }

    fn create_dir(&self, path: &Path) -> FsResult<()> {
        Ok(std::fs::create_dir(path)?)
    }

    fn create_dir_all(&self, path: &Path) -> FsResult<()> {
        Ok(std::fs::create_dir_all(path)?)
    }

    fn remove_file(&self, path: &Path) -> FsResult<()> {
        Ok(std::fs::remove_file(path)?)
    }

    fn remove_dir(&self, path: &Path) -> FsResult<()> {
        Ok(std::fs::remove_dir(path)?)
    }

    fn read_dir(&self, path: &Path) -> FsResult<Vec<PathBuf>> {
        Ok(std::fs::read_dir(path)?
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                // Filtrer les fichiers cachés (commencant par .)
                entry
                    .file_name()
                    .to_str()
                    .map(|name| !name.starts_with('.'))
                    .unwrap_or(true)
            })
            .map(|entry| entry.path())
            .collect::<Vec<_>>())
    }

    fn canonicalize(&self, path: &Path) -> FsResult<PathBuf> {
        Ok(std::fs::canonicalize(path)?)
    }

    fn is_dir(&self, path: &Path) -> FsResult<bool> {
        Ok(path.is_dir())
    }

    fn is_file(&self, path: &Path) -> FsResult<bool> {
        Ok(path.is_file())
    }
}

impl Default for LocalFileSystem {
    fn default() -> Self {
        LocalFileSystem
    }
}

/// Fournit une instance par défaut de LocalFileSystem
pub fn local_fs() -> LocalFileSystem {
    LocalFileSystem
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_local_fs_exists_current_dir() {
        let fs = LocalFileSystem;
        let current_dir = env::current_dir().unwrap();
        assert!(fs.exists(&current_dir).unwrap());
    }

    #[test]
    fn test_local_fs_not_exists_fake_path() {
        let fs = LocalFileSystem;
        let non_existent = Path::new("/non/existent/path/that/should/not/exist/12345");
        assert!(!fs.exists(non_existent).unwrap());
    }

    #[test]
    fn test_local_fs_is_dir_current_dir() {
        let fs = LocalFileSystem;
        let current_dir = env::current_dir().unwrap();
        assert!(fs.is_dir(&current_dir).unwrap());
        assert!(!fs.is_file(&current_dir).unwrap());
    }

    #[test]
    fn test_local_fs_canonicalize() {
        let fs = LocalFileSystem;
        let current_dir = env::current_dir().unwrap();
        let canonical = fs.canonicalize(&current_dir).unwrap();
        assert!(canonical.is_absolute());
    }
}
