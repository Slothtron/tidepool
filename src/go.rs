// Go version management module
use crate::{
    downloader::Downloader,
    symlink::{create_symlink, is_symlink, read_link, remove_symlink},
    InstallRequest, ListInstalledRequest, RuntimeStatus, StatusRequest, SwitchRequest,
    UninstallRequest, VersionInfo, VersionList,
};
use anyhow::{anyhow, Result};
use log::info;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Detailed information about a Go version
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct GoVersionInfo {
    /// Version number (e.g., "1.21.0")
    pub version: String,
    /// Operating system (e.g., "linux", "windows", "darwin")
    pub os: String,
    /// Architecture (e.g., "amd64", "arm64", "386")
    pub arch: String,
    /// File extension (e.g., "tar.gz", "zip")
    pub extension: String,
    /// Complete filename (e.g., "go1.21.0.linux-amd64.tar.gz")
    pub filename: String,
    /// Download URL
    pub download_url: String,
    /// Official SHA256 checksum
    pub sha256: Option<String>,
    /// File size in bytes
    pub size: Option<u64>,
    /// Whether it's installed
    pub is_installed: bool,
    /// Whether it's cached
    pub is_cached: bool,
    /// Local installation path (if installed)
    pub install_path: Option<PathBuf>,
    /// Cache file path (if cached)
    pub cache_path: Option<PathBuf>,
}

pub struct GoManager {}

impl Default for GoManager {
    fn default() -> Self {
        Self::new()
    }
}

impl GoManager {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }

    /// Extract archive to specified directory
    #[cfg(target_os = "windows")]
    pub fn extract_archive(&self, archive_path: &Path, extract_to: &Path) -> Result<()> {
        let file = std::fs::File::open(archive_path)?;
        let mut archive = zip::ZipArchive::new(file)?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;

            let outpath = extract_to.join(file.name());

            if file.name().ends_with('/') {
                std::fs::create_dir_all(&outpath)?;
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        std::fs::create_dir_all(p)?;
                    }
                }
                let mut outfile = std::fs::File::create(&outpath)?;
                std::io::copy(&mut file, &mut outfile)?;
            }
        }

        Ok(())
    }

    /// Extract archive to specified directory
    #[cfg(not(target_os = "windows"))]
    pub fn extract_archive(&self, archive_path: &Path, extract_to: &Path) -> Result<()> {
        let file = std::fs::File::open(archive_path)?;

        let gz = flate2::read::GzDecoder::new(file);
        let mut tar = tar::Archive::new(gz);

        tar.unpack(extract_to)?;

        Ok(())
    }

    /// Switch to a specific Go version
    pub fn switch_version(&self, version: &str, base_dir: &Path) -> Result<()> {
        let version_path = base_dir.join(version);
        let current_path = base_dir.join("current");

        if !version_path.exists() {
            return Err(anyhow!("Go version {} is not installed", version));
        }

        // Remove existing symlink if it exists
        if current_path.exists() {
            remove_symlink(&current_path)?;
        }

        // Create new symlink
        create_symlink(&version_path, &current_path)?;

        info!("Switched to Go version {version}");
        Ok(())
    }

    /// Get current version
    pub fn get_current_version(&self, base_dir: &Path) -> Option<String> {
        let current_path = base_dir.join("current");
        if current_path.exists() && is_symlink(&current_path) {
            if let Ok(target) = read_link(&current_path) {
                if let Some(name) = target.file_name() {
                    return name.to_str().map(|s| s.to_string());
                }
            }
        }
        None
    }

    /// Get symlink target
    pub fn get_link_target(&self, base_dir: &Path) -> Option<PathBuf> {
        let current_path = base_dir.join("current");
        if current_path.exists() && is_symlink(&current_path) {
            read_link(&current_path).ok()
        } else {
            None
        }
    }

    /// Get symlink information
    pub fn get_symlink_info(&self, base_dir: &Path) -> String {
        let current_path = base_dir.join("current");
        if current_path.exists() && is_symlink(&current_path) {
            if let Ok(target) = read_link(&current_path) {
                return format!("{} -> {}", current_path.display(), target.display());
            }
        }
        "No symlink found".to_string()
    }

    /// Install Go version
    pub async fn install(&self, request: InstallRequest) -> Result<VersionInfo> {
        let version = &request.version;
        let install_dir = &request.install_dir;
        let download_dir = &request.download_dir;

        // Determine platform information
        let platform = crate::platform::PlatformInfo::detect();
        let filename = platform.archive_filename(version);
        let download_url = format!("https://go.dev/dl/{filename}");
        let archive_path = download_dir.join(&filename);

        // Download if not cached
        if !archive_path.exists() {
            info!("Downloading Go {version} from {download_url}");

            let downloader = Downloader::new();

            downloader
                .download_with_simple_progress(&download_url, &archive_path, &filename)
                .await
                .map_err(|e| anyhow::anyhow!("Download failed: {}", e))?;
        }

        // Extract archive
        let version_dir = install_dir.join(version);
        if version_dir.exists() && !request.force {
            return Err(anyhow::anyhow!("Go version {} is already installed", version));
        }

        if version_dir.exists() {
            std::fs::remove_dir_all(&version_dir)
                .map_err(|e| anyhow::anyhow!("Failed to remove existing installation: {}", e))?;
        }

        // Create a temporary directory for extraction
        let temp_extract_dir = install_dir.join(format!("{version}_temp"));

        if temp_extract_dir.exists() {
            std::fs::remove_dir_all(&temp_extract_dir)
                .map_err(|e| anyhow::anyhow!("Failed to remove temp directory: {}", e))?;
        }

        std::fs::create_dir_all(&temp_extract_dir)
            .map_err(|e| anyhow::anyhow!("Failed to create temp directory: {}", e))?;

        // Extract to the temporary directory
        info!("Extracting archive to {}", temp_extract_dir.display());
        self.extract_archive(&archive_path, &temp_extract_dir)?;

        // The official Go archive extracts into a "go" directory, which we need to rename to the version number
        let extracted_go_dir = temp_extract_dir.join("go");
        if !extracted_go_dir.exists() {
            // Clean up the temporary directory
            let _ = std::fs::remove_dir_all(&temp_extract_dir);
            return Err(anyhow::anyhow!("Expected 'go' directory not found after extraction"));
        }

        // Rename the 'go' directory to the version directory
        std::fs::rename(&extracted_go_dir, &version_dir).map_err(|e| {
            anyhow::anyhow!("Failed to rename go directory to version directory: {}", e)
        })?;

        // Clean up the temporary directory
        std::fs::remove_dir_all(&temp_extract_dir)
            .map_err(|e| anyhow::anyhow!("Failed to remove temp directory: {}", e))?;

        // Verify installation - the Go binary should now be in the bin subdirectory of the version directory
        let go_binary =
            version_dir.join("bin").join(crate::platform::PlatformInfo::go_executable_name());

        if !go_binary.exists() {
            return Err(anyhow::anyhow!(
                "Go binary not found after extraction at {}",
                go_binary.display()
            ));
        }

        info!("Successfully installed Go version {version}");

        Ok(VersionInfo {
            version: version.to_string(),
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
            url: String::new(),
            sha256: String::new(),
            is_installed: true,
            path: version_dir,
        })
    }

    /// Switch to a version
    pub fn switch_to(&self, request: SwitchRequest) -> Result<()> {
        self.switch_version(&request.version, &request.base_dir)
    }

    /// Uninstall a version
    pub fn uninstall(&self, request: UninstallRequest) -> Result<()> {
        let version = &request.version;
        let base_dir = &request.base_dir;
        let version_path = base_dir.join(version);

        if !version_path.exists() {
            return Err(anyhow::anyhow!("Go version {} is not installed", version));
        }

        // Check if this is the current version
        let current_path = base_dir.join("current");
        if current_path.exists() && is_symlink(&current_path) {
            if let Ok(target) = read_link(&current_path) {
                if target == version_path {
                    return Err(anyhow::anyhow!(
                        "Cannot uninstall Go {} as it is currently active. Please switch to another version first.",
                        version
                    ));
                }
            }
        }

        // Remove the version directory
        std::fs::remove_dir_all(&version_path)
            .map_err(|e| anyhow::anyhow!("Failed to remove version directory: {}", e))?;

        info!("Successfully uninstalled Go version {version}");
        Ok(())
    }

    /// List installed versions
    pub fn list_installed(&self, request: ListInstalledRequest) -> Result<VersionList> {
        let base_dir = &request.base_dir;
        let mut versions = Vec::new();

        if !base_dir.exists() {
            return Ok(VersionList { versions, total_count: 0 });
        }

        let current_version = self.get_current_version(base_dir);

        for entry in std::fs::read_dir(base_dir)
            .map_err(|e| anyhow::anyhow!("Failed to read directory: {}", e))?
        {
            let entry =
                entry.map_err(|e| anyhow::anyhow!("Failed to read directory entry: {}", e))?;
            let path = entry.path();

            if path.is_dir() && path.file_name().is_some() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name != "current" {
                        let _is_current = current_version.as_ref().is_some_and(|cv| cv == name);
                        versions.push(GoVersionInfo {
                            version: name.to_string(),
                            os: std::env::consts::OS.to_string(),
                            arch: std::env::consts::ARCH.to_string(),
                            extension: String::new(),
                            filename: String::new(),
                            download_url: String::new(),
                            sha256: None,
                            size: None,
                            is_installed: true,
                            is_cached: false,
                            install_path: Some(path.clone()),
                            cache_path: None,
                        });
                    }
                }
            }
        }

        versions.sort();
        let total_count = versions.len();

        Ok(VersionList { versions, total_count })
    }

    /// List available versions
    pub fn list_available(&self) -> Result<VersionList> {
        // This is a simplified implementation
        // In a real implementation, you would fetch the list from Go's official API
        let versions = vec![
            GoVersionInfo {
                version: "1.21.3".to_string(),
                os: "linux".to_string(),
                arch: "amd64".to_string(),
                extension: "tar.gz".to_string(),
                filename: "go1.21.3.linux-amd64.tar.gz".to_string(),
                download_url: String::new(),
                sha256: None,
                size: None,
                is_installed: false,
                is_cached: false,
                install_path: None,
                cache_path: None,
            },
            GoVersionInfo {
                version: "1.21.2".to_string(),
                os: "linux".to_string(),
                arch: "amd64".to_string(),
                extension: "tar.gz".to_string(),
                filename: "go1.21.2.linux-amd64.tar.gz".to_string(),
                download_url: String::new(),
                sha256: None,
                size: None,
                is_installed: false,
                is_cached: false,
                install_path: None,
                cache_path: None,
            },
            GoVersionInfo {
                version: "1.21.1".to_string(),
                os: "linux".to_string(),
                arch: "amd64".to_string(),
                extension: "tar.gz".to_string(),
                filename: "go1.21.1.linux-amd64.tar.gz".to_string(),
                download_url: String::new(),
                sha256: None,
                size: None,
                is_installed: false,
                is_cached: false,
                install_path: None,
                cache_path: None,
            },
        ];

        let total_count = versions.len();
        Ok(VersionList { versions, total_count })
    }

    /// Get status
    pub fn status(&self, request: StatusRequest) -> Result<RuntimeStatus> {
        let base_dir = request.base_dir.unwrap_or_else(|| {
            dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")).join(".gvm").join("versions")
        });

        let current_version = self.get_current_version(&base_dir);
        let mut environment_vars = HashMap::new();

        if let Some(version) = &current_version {
            let version_path = base_dir.join(version);
            if version_path.exists() {
                environment_vars.insert("GOROOT".to_string(), version_path.display().to_string());
                environment_vars.insert(
                    "PATH".to_string(),
                    format!(
                        "{};{}",
                        version_path.join("bin").display(),
                        std::env::var("PATH").unwrap_or_default()
                    ),
                );
            }
        }

        let _link_info = if base_dir.join("current").exists() {
            Some(self.get_symlink_info(&base_dir))
        } else {
            None
        };

        let _is_installed = current_version.is_some();
        Ok(RuntimeStatus {
            current_version,
            current_path: self.get_link_target(&base_dir).map(|p| p.display().to_string()),
            environment_vars,
        })
    }

    /// Get version info
    pub fn get_version_info(
        &self,
        version: &str,
        install_dir: &Path,
        cache_dir: &Path,
    ) -> Result<GoVersionInfo> {
        let platform = crate::platform::PlatformInfo::detect();
        let filename = platform.archive_filename(version);
        let download_url = format!("https://go.dev/dl/{filename}");

        let install_path = install_dir.join(version);
        let cache_path = cache_dir.join(&filename);

        Ok(GoVersionInfo {
            version: version.to_string(),
            os: platform.os,
            arch: platform.arch,
            extension: platform.extension,
            filename: filename.clone(),
            download_url,
            sha256: None,
            size: None,
            is_installed: install_path.exists(),
            is_cached: cache_path.exists(),
            install_path: if install_path.exists() { Some(install_path) } else { None },
            cache_path: if cache_path.exists() { Some(cache_path) } else { None },
        })
    }
}
