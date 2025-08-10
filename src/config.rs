use anyhow::Result;
use std::env;
use std::path::{Path, PathBuf};

/// Configuration manager for GVM
///
/// Handles configuration paths with the following priority:
/// 1. Environment variables
/// 2. Default configuration
#[derive(Debug, Clone)]
#[allow(clippy::struct_field_names)]
pub struct Config {
    /// Root directory for GVM installation
    pub root_path: PathBuf,
    /// Directory containing installed Go versions
    pub versions_path: PathBuf,
    /// Directory for cached downloads
    pub cache_path: PathBuf,
}

impl Config {
    /// Initialize configuration from environment variables and defaults
    /// Creates a new configuration.
    ///
    /// # Errors
    /// Returns an error if the configuration directories cannot be created.
    pub fn new() -> Result<Self> {
        let root_path = Self::resolve_root_path()?;
        let versions_path = Self::resolve_versions_path(&root_path);
        let cache_path = Self::resolve_cache_path(&root_path);
        Ok(Config { root_path, versions_path, cache_path })
    }

    /// Get the GVM root path
    ///
    /// Priority: Environment variable `GVM_ROOT_PATH` -> Default (~/.gvm)
    fn resolve_root_path() -> Result<PathBuf> {
        if let Ok(env_path) = env::var("GVM_ROOT_PATH") {
            return Ok(PathBuf::from(env_path));
        }

        let home_dir = std::env::var("USERPROFILE")
            .or_else(|_| std::env::var("HOME"))
            .map(PathBuf::from)
            .map_err(|_| anyhow::anyhow!("Unable to get user home directory"))?;
        Ok(home_dir.join(".gvm"))
    }

    /// Get the versions directory path
    ///
    /// Priority: Environment variable `GVM_VERSIONS_PATH` -> Default (`$GVM_ROOT_PATH/versions`)
    fn resolve_versions_path(root_path: &Path) -> PathBuf {
        if let Ok(env_path) = env::var("GVM_VERSIONS_PATH") {
            return PathBuf::from(env_path);
        }
        root_path.join("versions")
    }

    /// Get the cache directory path
    ///
    /// Priority: Environment variable `GVM_CACHE_PATH` -> Default (`$GVM_ROOT_PATH/cache`)
    fn resolve_cache_path(root_path: &Path) -> PathBuf {
        if let Ok(env_path) = env::var("GVM_CACHE_PATH") {
            return PathBuf::from(env_path);
        }
        root_path.join("cache")
    }

    /// Get the versions path
    #[must_use]
    pub fn versions(&self) -> &PathBuf {
        &self.versions_path
    }

    /// Get the cache path
    #[must_use]
    pub fn cache(&self) -> &PathBuf {
        &self.cache_path
    }

    /// Ensure all configuration directories exist
    /// Ensure that required directories exist.
    ///
    /// # Errors
    /// Returns an error if the directories cannot be created.
    pub fn ensure_directories(&self) -> Result<()> {
        use std::fs;

        // Create root directory
        if !self.root_path.exists() {
            fs::create_dir_all(&self.root_path).map_err(|e| {
                anyhow::anyhow!(
                    "Failed to create root directory {}: {}",
                    self.root_path.display(),
                    e
                )
            })?;
        }

        // Create versions directory
        if !self.versions_path.exists() {
            fs::create_dir_all(&self.versions_path).map_err(|e| {
                anyhow::anyhow!(
                    "Failed to create versions directory {}: {}",
                    self.versions_path.display(),
                    e
                )
            })?;
        }

        // Create cache directory
        if !self.cache_path.exists() {
            fs::create_dir_all(&self.cache_path).map_err(|e| {
                anyhow::anyhow!(
                    "Failed to create cache directory {}: {}",
                    self.cache_path.display(),
                    e
                )
            })?;
        }

        Ok(())
    }
}
