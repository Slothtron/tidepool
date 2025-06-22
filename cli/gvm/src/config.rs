use anyhow::Result;
use std::env;
use std::path::{Path, PathBuf};

/// Configuration manager for GVM
///
/// Handles configuration paths with the following priority:
/// 1. Environment variables  
/// 2. Default configuration
#[derive(Debug, Clone)]
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
    pub fn new() -> Result<Self> {
        let root_path = Self::resolve_root_path()?;
        let versions_path = Self::resolve_versions_path(&root_path)?;
        let cache_path = Self::resolve_cache_path(&root_path)?;
        Ok(Config { root_path, versions_path, cache_path })
    }

    /// Get the GVM root path
    ///
    /// Priority: Environment variable GVM_ROOT_PATH -> Default (~/.gvm)
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
    /// Priority: Environment variable GVM_VERSIONS_PATH -> Default ($GVM_ROOT_PATH/versions)
    fn resolve_versions_path(root_path: &Path) -> Result<PathBuf> {
        if let Ok(env_path) = env::var("GVM_VERSIONS_PATH") {
            return Ok(PathBuf::from(env_path));
        }
        Ok(root_path.join("versions"))
    }

    /// Get the cache directory path
    ///
    /// Priority: Environment variable GVM_CACHE_PATH -> Default ($GVM_ROOT_PATH/cache)
    fn resolve_cache_path(root_path: &Path) -> Result<PathBuf> {
        if let Ok(env_path) = env::var("GVM_CACHE_PATH") {
            return Ok(PathBuf::from(env_path));
        }
        Ok(root_path.join("cache"))
    }

    /// Get the versions path
    pub fn versions(&self) -> &PathBuf {
        &self.versions_path
    }

    /// Get the cache path
    pub fn cache(&self) -> &PathBuf {
        &self.cache_path
    }

    /// Ensure all configuration directories exist
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    #[test]
    fn test_config_initialization() {
        // 清除可能影响测试的环境变量
        env::remove_var("GVM_ROOT");

        // Test default configuration
        let config = Config::new().unwrap();
        assert!(config.root_path.to_string_lossy().contains(".gvm"));
        assert!(config.versions().to_string_lossy().contains("versions"));
        assert!(config.cache().to_string_lossy().contains("cache"));
    }

    #[test]
    fn test_env_var_priority() {
        // Test that environment variable takes precedence for root path
        let test_root = if cfg!(windows) { "C:\\test\\root" } else { "/test/root" };

        unsafe {
            env::set_var("GVM_ROOT_PATH", test_root);
        }
        let config = Config::new().unwrap();
        assert_eq!(config.root_path, PathBuf::from(test_root));
        unsafe {
            env::remove_var("GVM_ROOT_PATH");
        }

        // Test environment variable for versions path
        let test_versions = if cfg!(windows) { "C:\\env\\versions" } else { "/env/versions" };

        unsafe {
            env::set_var("GVM_VERSIONS_PATH", test_versions);
        }
        let config = Config::new().unwrap();
        assert_eq!(config.versions(), &PathBuf::from(test_versions));
        unsafe {
            env::remove_var("GVM_VERSIONS_PATH");
        }
    }
    #[test]
    fn test_config_paths() {
        // Ensure clean environment for this test
        unsafe {
            env::remove_var("GVM_ROOT_PATH");
            env::remove_var("GVM_VERSIONS_PATH");
            env::remove_var("GVM_CACHE_PATH");
        }

        let config = Config::new().unwrap(); // Test that basic path functionality works
        assert!(
            config.root_path.is_absolute(),
            "Config root path should be absolute, got: {}",
            config.root_path.display()
        );
        assert!(config.versions().to_string_lossy().contains("versions"));
        assert!(config.cache().to_string_lossy().contains("cache"));
    }
}
