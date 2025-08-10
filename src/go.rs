// Go version management module
use crate::{
    downloader::{Downloader, ProgressReporter},
    symlink::{get_symlink_target, is_symlink, remove_symlink_dir, symlink_dir},
    InstallRequest, ListInstalledRequest, RuntimeStatus, StatusRequest, SwitchRequest,
    UninstallRequest, VersionInfo, VersionList,
};
use log::info;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Detailed information about a Go version
#[derive(Debug, Clone)]
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
    pub fn extract_archive(&self, archive_path: &Path, extract_to: &Path) -> Result<(), String> {
        #[cfg(target_os = "windows")]
        {
            Self::extract_zip(archive_path, extract_to)
        }

        #[cfg(not(target_os = "windows"))]
        {
            self.extract_tar_gz(archive_path, extract_to)
        }
    }

    /// Switch to a specific Go version
    pub fn switch_version(&self, version: &str, base_dir: &Path) -> Result<(), String> {
        let version_path = base_dir.join(version);
        let current_path = base_dir.join("current");

        if !version_path.exists() {
            return Err(format!("Go version {version} is not installed"));
        }

        // Remove existing symlink if it exists
        if current_path.exists() {
            remove_symlink_dir(&current_path)
                .map_err(|e| format!("Failed to remove existing symlink: {}", e))?;
        }

        // Create new symlink
        symlink_dir(&version_path, &current_path)
            .map_err(|e| format!("Failed to create symlink: {}", e))?;

        info!("Switched to Go version {}", version);
        Ok(())
    }

    /// Get current version
    pub fn get_current_version(&self, base_dir: &Path) -> Option<String> {
        let current_path = base_dir.join("current");
        if current_path.exists() && is_symlink(&current_path) {
            if let Some(target) = get_symlink_target(&current_path) {
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
            get_symlink_target(&current_path)
        } else {
            None
        }
    }

    /// Get symlink target
    pub fn get_symlink_target(&self, base_dir: &Path) -> Option<PathBuf> {
        self.get_link_target(base_dir)
    }

    /// Get symlink information
    pub fn get_symlink_info(&self, base_dir: &Path) -> String {
        let current_path = base_dir.join("current");
        if current_path.exists() && is_symlink(&current_path) {
            if let Some(target) = get_symlink_target(&current_path) {
                return format!("{} -> {}", current_path.display(), target.display());
            }
        }
        "No symlink found".to_string()
    }

    /// Extract ZIP archive
    fn extract_zip(zip_path: &Path, extract_to: &Path) -> Result<(), String> {
        let file =
            std::fs::File::open(zip_path).map_err(|e| format!("Failed to open ZIP file: {}", e))?;

        let mut archive =
            zip::ZipArchive::new(file).map_err(|e| format!("Failed to read ZIP archive: {}", e))?;

        for i in 0..archive.len() {
            let mut file =
                archive.by_index(i).map_err(|e| format!("Failed to access file in ZIP: {}", e))?;

            let outpath = extract_to.join(file.name());

            if file.name().ends_with('/') {
                std::fs::create_dir_all(&outpath)
                    .map_err(|e| format!("Failed to create directory: {}", e))?;
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        std::fs::create_dir_all(p)
                            .map_err(|e| format!("Failed to create parent directory: {}", e))?;
                    }
                }
                let mut outfile = std::fs::File::create(&outpath)
                    .map_err(|e| format!("Failed to create file: {}", e))?;
                std::io::copy(&mut file, &mut outfile)
                    .map_err(|e| format!("Failed to write file: {}", e))?;
            }
        }

        Ok(())
    }

    /// Extract TAR.GZ archive
    #[cfg(not(target_os = "windows"))]
    fn extract_tar_gz(&self, tar_gz_path: &Path, extract_to: &Path) -> Result<(), String> {
        let file = std::fs::File::open(tar_gz_path)
            .map_err(|e| format!("Failed to open TAR.GZ file: {}", e))?;

        let gz = flate2::read::GzDecoder::new(file);
        let mut tar = tar::Archive::new(gz);

        tar.unpack(extract_to).map_err(|e| format!("Failed to extract TAR.GZ: {}", e))?;

        Ok(())
    }

    /// Extract TAR.GZ archive (Windows placeholder)
    #[cfg(target_os = "windows")]
    #[allow(dead_code)]
    fn extract_tar_gz(&self, _tar_gz_path: &Path, _extract_to: &Path) -> Result<(), String> {
        Err("TAR.GZ extraction not supported on Windows".to_string())
    }

    /// Install Go version
    pub async fn install(&self, request: InstallRequest) -> anyhow::Result<VersionInfo> {
        let version = &request.version;
        let install_dir = &request.install_dir;
        let download_dir = &request.download_dir;

        // Determine platform information
        let (os, arch) = if cfg!(target_os = "windows") {
            ("windows", if cfg!(target_arch = "x86_64") { "amd64" } else { "386" })
        } else if cfg!(target_os = "macos") {
            ("darwin", if cfg!(target_arch = "x86_64") { "amd64" } else { "arm64" })
        } else {
            ("linux", if cfg!(target_arch = "x86_64") { "amd64" } else { "386" })
        };

        let extension = if cfg!(target_os = "windows") { "zip" } else { "tar.gz" };
        let filename = format!("go{version}.{os}-{arch}.{extension}");
        let download_url = format!("https://go.dev/dl/{}", filename);
        let archive_path = download_dir.join(&filename);

        // Download if not cached
        if !archive_path.exists() {
            info!("Downloading Go {} from {}", version, download_url);

            let downloader = Downloader::new();
            let file_size = downloader
                .get_file_size(&download_url)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to get file size: {}", e))?;

            let progress_reporter = ProgressReporter::new(file_size);
            progress_reporter.start();

            downloader
                .download(&download_url, &archive_path, Some(progress_reporter))
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

        std::fs::create_dir_all(&version_dir)
            .map_err(|e| anyhow::anyhow!("Failed to create version directory: {}", e))?;

        self.extract_archive(&archive_path, &version_dir)
            .map_err(|e| anyhow::anyhow!("Failed to extract archive: {}", e))?;

        // Verify installation
        #[cfg(target_os = "windows")]
        let go_binary = version_dir.join("bin").join("go.exe");
        #[cfg(not(target_os = "windows"))]
        let go_binary = version_dir.join("bin").join("go");

        if !go_binary.exists() {
            return Err(anyhow::anyhow!(
                "Go binary not found after extraction at {}",
                go_binary.display()
            ));
        }

        info!("Successfully installed Go version {}", version);

        Ok(VersionInfo { version: version.to_string(), path: version_dir, is_current: false })
    }

    /// Switch to a version
    pub async fn switch_to(&self, request: SwitchRequest) -> anyhow::Result<()> {
        self.switch_version(&request.version, &request.base_dir)
            .map_err(|e| anyhow::anyhow!("{}", e))
    }

    /// Uninstall a version
    pub async fn uninstall(&self, request: UninstallRequest) -> anyhow::Result<()> {
        let version = &request.version;
        let base_dir = &request.base_dir;
        let version_path = base_dir.join(version);

        if !version_path.exists() {
            return Err(anyhow::anyhow!("Go version {} is not installed", version));
        }

        // Check if this is the current version
        let current_path = base_dir.join("current");
        if current_path.exists() && is_symlink(&current_path) {
            if let Some(target) = get_symlink_target(&current_path) {
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

        info!("Successfully uninstalled Go version {}", version);
        Ok(())
    }

    /// List installed versions
    pub async fn list_installed(
        &self,
        request: ListInstalledRequest,
    ) -> anyhow::Result<VersionList> {
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
                        let is_current = current_version.as_ref().map_or(false, |cv| cv == name);
                        versions.push(VersionInfo {
                            version: name.to_string(),
                            path: path.clone(),
                            is_current,
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
    pub async fn list_available(&self) -> anyhow::Result<VersionList> {
        // This is a simplified implementation
        // In a real implementation, you would fetch the list from Go's official API
        let versions = vec![
            VersionInfo { version: "1.21.3".to_string(), path: PathBuf::new(), is_current: false },
            VersionInfo { version: "1.21.2".to_string(), path: PathBuf::new(), is_current: false },
            VersionInfo { version: "1.21.1".to_string(), path: PathBuf::new(), is_current: false },
        ];

        let total_count = versions.len();
        Ok(VersionList { versions, total_count })
    }

    /// Get status
    pub async fn status(&self, request: StatusRequest) -> anyhow::Result<RuntimeStatus> {
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

        let link_info = if base_dir.join("current").exists() {
            Some(self.get_symlink_info(&base_dir))
        } else {
            None
        };

        let is_installed = current_version.is_some();
        Ok(RuntimeStatus {
            current_version,
            go_path: self.get_link_target(&base_dir),
            is_installed,
            install_path: self.get_link_target(&base_dir),
            environment_vars,
            link_info,
        })
    }

    /// Get version info
    pub async fn get_version_info(
        &self,
        version: &str,
        install_dir: &Path,
        cache_dir: &Path,
    ) -> anyhow::Result<GoVersionInfo> {
        let (os, arch) = if cfg!(target_os = "windows") {
            ("windows", if cfg!(target_arch = "x86_64") { "amd64" } else { "386" })
        } else if cfg!(target_os = "macos") {
            ("darwin", if cfg!(target_arch = "x86_64") { "amd64" } else { "arm64" })
        } else {
            ("linux", if cfg!(target_arch = "x86_64") { "amd64" } else { "386" })
        };

        let extension = if cfg!(target_os = "windows") { "zip" } else { "tar.gz" };
        let filename = format!("go{version}.{os}-{arch}.{extension}");
        let download_url = format!("https://go.dev/dl/{}", filename);

        let install_path = install_dir.join(version);
        let cache_path = cache_dir.join(&filename);

        Ok(GoVersionInfo {
            version: version.to_string(),
            os: os.to_string(),
            arch: arch.to_string(),
            extension: extension.to_string(),
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
