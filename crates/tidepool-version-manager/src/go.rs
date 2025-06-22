// Go version management module
use crate::{
    downloader::{Downloader, ProgressReporter},
    InstallRequest, ListInstalledRequest, RuntimeStatus, StatusRequest, SwitchRequest,
    UninstallRequest, VersionInfo, VersionList, VersionManager,
};
use log::{debug, info, warn};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::io::AsyncReadExt;

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
    pub fn new() -> Self {
        Self {}
    }

    /// Extract archive to specified directory (public method)
    pub fn extract_archive(&self, archive_path: &Path, extract_to: &Path) -> Result<(), String> {
        #[cfg(target_os = "windows")]
        {
            self.extract_zip(archive_path, extract_to)
        }

        #[cfg(not(target_os = "windows"))]
        {
            self.extract_tar_gz(archive_path, extract_to)
        }
    }

    /// 跨平台版本切换实现（Windows 使用 Junction，Unix 使用符号链接）
    pub fn switch_version(&self, version: &str, base_dir: &Path) -> Result<(), String> {
        #[cfg(target_os = "windows")]
        {
            self.switch_version_windows(version, base_dir)
        }

        #[cfg(not(target_os = "windows"))]
        {
            self.switch_version_unix(version, base_dir)
        }
    }

    /// Windows Junction Point 版本切换实现（不需要管理员权限）
    #[cfg(target_os = "windows")]
    fn switch_version_windows(&self, version: &str, base_dir: &Path) -> Result<(), String> {
        let version_path = base_dir.join(version);
        let junction_path = base_dir.join("current");

        if !version_path.exists() {
            return Err(format!("Go version {} is not installed", version));
        }

        // 验证Go安装的完整性
        let go_exe = version_path.join("bin").join("go.exe");
        if !go_exe.exists() {
            return Err(format!(
                "Invalid Go installation: missing go.exe in {}",
                version_path.display()
            ));
        }

        debug!("Creating junction point for Go version {}", version);

        // 删除现有的junction或目录
        if junction_path.exists() && junction_path.is_dir() {
            std::fs::remove_dir_all(&junction_path)
                .map_err(|e| format!("Failed to remove existing directory: {}", e))?;
        }

        // 使用mklink命令创建junction (不需要管理员权限)
        let output = std::process::Command::new("cmd")
            .args([
                "/C",
                "mklink",
                "/J",
                &junction_path.to_string_lossy(),
                &version_path.to_string_lossy(),
            ])
            .output()
            .map_err(|e| format!("Failed to execute mklink: {}", e))?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to create junction: {}", error_msg));
        }

        info!("Successfully created junction point for Go version {}", version);

        // 验证junction是否正确创建
        if !junction_path.exists() {
            return Err("Junction does not exist after creation".to_string());
        }

        let junction_go_exe = junction_path.join("bin").join("go.exe");
        if !junction_go_exe.exists() {
            return Err("Junction target is invalid: missing go.exe".to_string());
        }
        debug!("Successfully created junction point for Go version {}", version);
        debug!("Environment variables updated for Go version {}", version);
        Ok(())
    }

    /// Unix Symlink 版本切换实现（使用符号链接）
    #[cfg(not(target_os = "windows"))]
    fn switch_version_unix(&self, version: &str, base_dir: &Path) -> Result<(), String> {
        let version_path = base_dir.join(version);
        let current_path = base_dir.join("current");

        if !version_path.exists() {
            return Err(format!("Go version {} is not installed", version));
        }

        // 验证Go安装的完整性
        let go_binary = version_path.join("bin").join("go");
        if !go_binary.exists() {
            return Err(format!(
                "Invalid Go installation: missing go binary in {}",
                version_path.display()
            ));
        }

        debug!("Creating symlink for Go version {}", version);

        // 删除现有的符号链接或目录
        if current_path.exists() {
            if current_path.is_symlink() {
                std::fs::remove_file(&current_path)
                    .map_err(|e| format!("Failed to remove existing symlink: {}", e))?;
            } else if current_path.is_dir() {
                std::fs::remove_dir_all(&current_path)
                    .map_err(|e| format!("Failed to remove existing directory: {}", e))?;
            }
        }

        // 创建符号链接
        #[cfg(not(target_os = "windows"))]
        {
            std::os::unix::fs::symlink(&version_path, &current_path)
                .map_err(|e| format!("Failed to create symlink: {}", e))?;
        }

        info!("Successfully created symlink for Go version {}", version);

        // 验证符号链接是否正确创建
        if !current_path.exists() {
            return Err("Symlink does not exist after creation".to_string());
        }

        let current_go_binary = current_path.join("bin").join("go");
        if !current_go_binary.exists() {
            return Err("Symlink target is invalid: missing go binary".to_string());
        }

        debug!("Successfully created symlink for Go version {}", version);
        Ok(())
    }

    /// 获取当前活跃的Go版本
    pub fn get_current_version(&self, base_dir: &Path) -> Option<String> {
        // 首先尝试从链接获取（跨平台支持junction和symlink）
        if let Some(target) = self.get_link_target(base_dir) {
            return target.file_name().and_then(|name| name.to_str()).map(|s| s.to_string());
        }

        // 最后尝试从环境变量推断
        if let Ok(goroot) = std::env::var("GOROOT") {
            let goroot_path = PathBuf::from(goroot);
            if goroot_path.starts_with(base_dir) {
                return goroot_path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .map(|s| s.to_string());
            }
        }

        None
    }

    /// 获取链接指向的目标路径（跨平台）
    pub fn get_link_target(&self, base_dir: &Path) -> Option<PathBuf> {
        let link_path = base_dir.join("current");

        if !link_path.exists() {
            return None;
        }

        // 检查是否为链接（适用于Windows junction和Unix symlink）
        if link_path.is_symlink() {
            // 使用标准库读取链接目标
            if let Ok(target) = std::fs::read_link(&link_path) {
                return Some(target);
            }
        }

        None
    }

    /// 获取junction指向的目标路径（Windows特定，向后兼容）
    #[cfg(target_os = "windows")]
    pub fn get_junction_target(&self, base_dir: &Path) -> Option<PathBuf> {
        self.get_link_target(base_dir)
    }

    /// 获取符号链接指向的目标路径（跨平台）
    pub fn get_symlink_target(&self, base_dir: &Path) -> Option<PathBuf> {
        self.get_link_target(base_dir)
    }

    /// 获取链接状态信息（跨平台）
    pub fn get_symlink_info(&self, base_dir: &Path) -> String {
        let link_path = base_dir.join("current");

        if !link_path.exists() {
            #[cfg(target_os = "windows")]
            return "No junction found".to_string();
            #[cfg(not(target_os = "windows"))]
            return "No symlink found".to_string();
        }

        if let Some(target) = self.get_link_target(base_dir) {
            #[cfg(target_os = "windows")]
            return format!("Junction: {} -> {}", link_path.display(), target.display());
            #[cfg(not(target_os = "windows"))]
            return format!("Symlink: {} -> {}", link_path.display(), target.display());
        } else {
            #[cfg(target_os = "windows")]
            return "Junction exists but target unknown".to_string();
            #[cfg(not(target_os = "windows"))]
            return "Symlink exists but target unknown".to_string();
        }
    }

    /// 解压 ZIP 文件 (Windows)
    #[cfg(target_os = "windows")]
    fn extract_zip(&self, zip_path: &Path, extract_to: &Path) -> Result<(), String> {
        use std::fs::File;
        use std::io::BufReader;

        let file = File::open(zip_path).map_err(|e| format!("Failed to open zip file: {}", e))?;
        let reader = BufReader::new(file);

        let mut archive = zip::ZipArchive::new(reader)
            .map_err(|e| format!("Failed to read zip archive: {}", e))?;

        for i in 0..archive.len() {
            let mut file = archive
                .by_index(i)
                .map_err(|e| format!("Failed to access file in archive: {}", e))?;

            let file_path = match file.enclosed_name() {
                Some(path) => path,
                None => continue,
            };

            // Skip the top-level "go" directory and extract its contents directly
            let relative_path = if let Ok(stripped) = file_path.strip_prefix("go") {
                stripped
            } else {
                continue; // Skip files not in the "go" directory
            };

            // Skip empty path (the "go" directory itself)
            if relative_path.as_os_str().is_empty() {
                continue;
            }

            let outpath = extract_to.join(relative_path);

            if file.name().ends_with('/') {
                // Directory
                std::fs::create_dir_all(&outpath)
                    .map_err(|e| format!("Failed to create directory: {}", e))?;
            } else {
                // File
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        std::fs::create_dir_all(p)
                            .map_err(|e| format!("Failed to create parent directory: {}", e))?;
                    }
                }

                let mut outfile = File::create(&outpath)
                    .map_err(|e| format!("Failed to create output file: {}", e))?;

                std::io::copy(&mut file, &mut outfile)
                    .map_err(|e| format!("Failed to extract file: {}", e))?;
            }
        }

        Ok(())
    }

    /// 解压 tar.gz 文件 (Unix)
    #[cfg(not(target_os = "windows"))]
    fn extract_tar_gz(&self, tar_gz_path: &Path, extract_to: &Path) -> Result<(), String> {
        use flate2::read::GzDecoder;
        use std::fs::File;
        use tar::Archive;

        let file =
            File::open(tar_gz_path).map_err(|e| format!("Failed to open tar.gz file: {}", e))?;

        let gz = GzDecoder::new(file);
        let mut archive = Archive::new(gz);

        // Extract all entries
        for entry in
            archive.entries().map_err(|e| format!("Failed to read archive entries: {}", e))?
        {
            let mut entry = entry.map_err(|e| format!("Failed to read archive entry: {}", e))?;

            let entry_path =
                entry.path().map_err(|e| format!("Failed to get entry path: {}", e))?;

            // Skip the top-level "go" directory and extract its contents directly
            let relative_path = if let Ok(stripped) = entry_path.strip_prefix("go") {
                stripped
            } else {
                continue; // Skip files not in the "go" directory
            };

            // Skip empty path (the "go" directory itself)
            if relative_path.as_os_str().is_empty() {
                continue;
            }

            let target_path = extract_to.join(relative_path);

            // Create parent directories if needed
            if let Some(parent) = target_path.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create parent directory: {}", e))?;
            }

            // Extract the entry to the target path
            entry.unpack(&target_path).map_err(|e| format!("Failed to extract entry: {}", e))?;
        }

        Ok(())
    }

    /// 计算文件的 SHA256 哈希值
    pub async fn calculate_file_hash(&self, file_path: &Path) -> Result<String, String> {
        let mut file = tokio::fs::File::open(file_path)
            .await
            .map_err(|e| format!("Failed to open file for hash calculation: {}", e))?;

        let mut hasher = Sha256::new();
        let mut buffer = vec![0u8; 8192]; // 8KB 缓冲区

        loop {
            let bytes_read =
                file.read(&mut buffer).await.map_err(|e| format!("Error reading file: {}", e))?;

            if bytes_read == 0 {
                break;
            }

            hasher.update(&buffer[..bytes_read]);
        }

        let result = hasher.finalize();
        Ok(format!("{:x}", result))
    }

    /// 获取 Go 版本的官方 SHA256 校验和
    async fn get_official_checksum(
        &self,
        version: &str,
        os: &str,
        arch: &str,
        extension: &str,
    ) -> Result<String, String> {
        let client = reqwest::Client::new();
        let checksums_url = "https://go.dev/dl/?mode=json&include=all".to_string();

        debug!("Fetching official checksums: {}", checksums_url);

        let response = client
            .get(&checksums_url)
            .send()
            .await
            .map_err(|e| format!("Failed to fetch checksums: {}", e))?;

        let versions: serde_json::Value =
            response.json().await.map_err(|e| format!("Failed to parse checksum data: {}", e))?;

        let filename = format!("go{}.{}-{}.{}", version, os, arch, extension);
        debug!("Looking for checksum for file: {}", filename);

        // 查找对应版本和文件的校验和
        if let Some(releases) = versions.as_array() {
            for release in releases {
                if let Some(version_str) = release.get("version").and_then(|v| v.as_str()) {
                    if version_str == format!("go{}", version) {
                        if let Some(files) = release.get("files").and_then(|f| f.as_array()) {
                            for file in files {
                                if let Some(file_name) =
                                    file.get("filename").and_then(|f| f.as_str())
                                {
                                    if file_name == filename {
                                        if let Some(sha256) =
                                            file.get("sha256").and_then(|s| s.as_str())
                                        {
                                            debug!("Found official checksum: {}", sha256);
                                            return Ok(sha256.to_string());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Err(format!("Official checksum not found for Go {} ({})", version, filename))
    }

    /// 校验下载文件的完整性
    async fn verify_file_integrity(
        &self,
        file_path: &Path,
        version: &str,
        os: &str,
        arch: &str,
        extension: &str,
    ) -> Result<(), String> {
        debug!("Starting file integrity verification: {}", file_path.display());

        // 计算下载文件的哈希值
        let file_hash = self.calculate_file_hash(file_path).await?;
        debug!("File hash: {}", file_hash);

        // 获取官方校验和
        let official_hash = self.get_official_checksum(version, os, arch, extension).await?;
        debug!("Official hash: {}", official_hash);

        // 比较哈希值
        if file_hash.to_lowercase() == official_hash.to_lowercase() {
            info!("File integrity verification passed");
            Ok(())
        } else {
            Err(format!(
                "File integrity verification failed!\nExpected: {}\nActual: {}",
                official_hash, file_hash
            ))
        }
    }

    /// 验证缓存文件的完整性
    /// 检查文件是否存在、非空，并且可以被读取
    pub fn validate_cache_file(&self, file_path: &Path) -> bool {
        if !file_path.exists() {
            return false;
        }

        match std::fs::metadata(file_path) {
            Ok(metadata) => {
                // 文件大小检查：至少应该有一些内容（比如大于1KB）
                if metadata.len() < 1024 {
                    return false;
                }

                // 尝试打开文件以确保它可读
                std::fs::File::open(file_path).is_ok()
            }
            Err(_) => false,
        }
    }

    /// 获取 Go 版本的详细信息
    pub async fn get_version_info(
        &self,
        version: &str,
        install_dir: &Path,
        cache_dir: &Path,
    ) -> Result<GoVersionInfo, String> {
        // 确定当前平台信息
        let (os, arch) = if cfg!(target_os = "windows") {
            ("windows", if cfg!(target_arch = "x86_64") { "amd64" } else { "386" })
        } else if cfg!(target_os = "macos") {
            ("darwin", if cfg!(target_arch = "x86_64") { "amd64" } else { "arm64" })
        } else {
            ("linux", if cfg!(target_arch = "x86_64") { "amd64" } else { "386" })
        };

        let extension = if cfg!(target_os = "windows") { "zip" } else { "tar.gz" };
        let filename = format!("go{}.{}-{}.{}", version, os, arch, extension);
        let download_url = format!("https://go.dev/dl/{}", filename);

        // 检查是否已安装
        let install_path = install_dir.join(version);
        let is_installed = install_path.exists() && {
            let go_binary = if cfg!(target_os = "windows") {
                install_path.join("bin").join("go.exe")
            } else {
                install_path.join("bin").join("go")
            };
            go_binary.exists()
        };

        // 检查是否已缓存
        let cache_path = cache_dir.join(&filename);
        let is_cached = cache_path.exists() && self.validate_cache_file(&cache_path);

        // 获取官方校验和
        let sha256 = self.get_official_checksum(version, os, arch, extension).await.ok();

        // 获取文件大小
        let size = if is_cached {
            // 如果已缓存，从本地文件获取大小
            debug!("Getting size from cached file: {}", cache_path.display());
            std::fs::metadata(&cache_path).ok().map(|m| m.len())
        } else {
            // 如果未缓存，尝试通过网络获取文件大小
            debug!("Getting size from network for: {}", download_url);
            use crate::downloader::Downloader;
            let downloader = Downloader::new();
            match downloader.get_file_size(&download_url).await {
                Ok(size) => {
                    debug!("Successfully got file size from network: {} bytes", size);
                    Some(size)
                }
                Err(e) => {
                    debug!("Failed to get file size from network: {}", e);
                    None
                }
            }
        };

        Ok(GoVersionInfo {
            version: version.to_string(),
            os: os.to_string(),
            arch: arch.to_string(),
            extension: extension.to_string(),
            filename,
            download_url,
            sha256,
            size,
            is_installed,
            is_cached,
            install_path: if is_installed { Some(install_path) } else { None },
            cache_path: if is_cached { Some(cache_path) } else { None },
        })
    }
}

#[async_trait::async_trait]
impl VersionManager for GoManager {
    /// 安装指定版本的Go
    async fn install(&self, request: InstallRequest) -> Result<VersionInfo, String> {
        let version = &request.version;
        let install_dir = &request.install_dir;
        let download_dir = &request.download_dir;
        let force = request.force;

        // Check if install directory exists and is accessible
        if !install_dir.exists() {
            return Err("Install directory does not exist".to_string());
        }

        if !install_dir.is_dir() {
            return Err("Install path is not a directory".to_string());
        }

        // 创建版本目录
        let version_dir = install_dir.join(version);
        if version_dir.exists() && !force {
            return Err(format!("Go version {} is already installed", version));
        }

        // 如果强制安装且版本目录存在，删除现有目录
        if force && version_dir.exists() {
            std::fs::remove_dir_all(&version_dir)
                .map_err(|e| format!("Failed to remove existing version directory: {}", e))?;
        }

        std::fs::create_dir_all(&version_dir)
            .map_err(|e| format!("Failed to create version directory: {}", e))?;

        // 构建下载URL
        let (os, arch) = if cfg!(target_os = "windows") {
            ("windows", if cfg!(target_arch = "x86_64") { "amd64" } else { "386" })
        } else if cfg!(target_os = "macos") {
            ("darwin", if cfg!(target_arch = "x86_64") { "amd64" } else { "arm64" })
        } else {
            ("linux", if cfg!(target_arch = "x86_64") { "amd64" } else { "386" })
        };

        let extension = if cfg!(target_os = "windows") { "zip" } else { "tar.gz" };
        let download_url = format!("https://go.dev/dl/go{}.{}-{}.{}", version, os, arch, extension);

        // 设置下载文件名和路径
        let archive_name = format!("go{}.{}-{}.{}", version, os, arch, extension);

        // 验证下载目录存在
        if !download_dir.exists() {
            return Err(format!(
                "Download directory does not exist: {}. Please ensure the download directory is created before installation.",
                download_dir.display()
            ));
        }

        if !download_dir.is_dir() {
            return Err(format!("Download path is not a directory: {}", download_dir.display()));
        }

        let download_path = download_dir.join(&archive_name);

        // 检查缓存文件是否已存在
        let need_download = if force {
            // 强制模式：删除现有缓存文件并重新下载
            if download_path.exists() {
                debug!("Force mode: removing existing cached file");
                std::fs::remove_file(&download_path)
                    .map_err(|e| format!("Failed to remove cached file: {}", e))?;
            }
            debug!("Force mode: downloading Go {} from {}", version, download_url);
            true
        } else if download_path.exists() {
            debug!("Found cached file: {}", download_path.display());

            // 使用更严格的完整性验证
            if self.validate_cache_file(&download_path) {
                let metadata = std::fs::metadata(&download_path).unwrap();
                debug!("Using valid cached file (size: {} bytes)", metadata.len());
                false
            } else {
                debug!("Cached file appears to be corrupted or incomplete, will re-download");
                true
            }
        } else {
            debug!("Downloading Go {} from {}", version, download_url);
            true
        };

        // 只有在需要时才下载文件
        if need_download {
            // 使用内置下载器下载
            let downloader = Downloader::new();

            // 先获取文件大小，然后创建正确大小的进度报告器
            let file_size =
                downloader.get_file_size(&download_url).await.map_err(|e| format!("{}", e))?;
            let progress_reporter = ProgressReporter::new(file_size);

            downloader
                .download(&download_url, &download_path, Some(progress_reporter))
                .await
                .map_err(|e| format!("{}", e))?;

            // 下载完成后立即校验文件完整性
            debug!("Verifying downloaded file integrity");
            self.verify_file_integrity(&download_path, version, os, arch, extension)
                .await
                .map_err(|e| {
                    // 校验失败时删除损坏的文件
                    if download_path.exists() {
                        let _ = std::fs::remove_file(&download_path);
                        warn!(
                            "Verification failed, deleted corrupted file: {}",
                            download_path.display()
                        );
                    }
                    format!("File integrity verification failed: {}", e)
                })?;
        } else {
            // 即使是缓存文件，也要进行完整性校验
            debug!("Verifying cached file integrity");
            self.verify_file_integrity(&download_path, version, os, arch, extension)
                .await
                .map_err(|e| {
                    // 校验失败时删除损坏的缓存文件
                    if download_path.exists() {
                        let _ = std::fs::remove_file(&download_path);
                        warn!(
                            "Cached file verification failed, deleted: {}",
                            download_path.display()
                        );
                    }
                    format!(
                        "Cached file integrity verification failed, recommend re-downloading: {}",
                        e
                    )
                })?;
        }

        debug!("Extracting Go {} to {}", version, version_dir.display());

        // 解压下载的文件
        self.extract_archive(&download_path, &version_dir)?;

        // 如果使用下载目录，下载完成后可以选择保留或删除压缩包
        // 这里我们保留压缩包以便下次使用（由调用方决定缓存策略）

        info!("Go {} installed successfully", version);

        // Return version info
        Ok(VersionInfo { version: version.to_string(), install_path: version_dir })
    }
    /// 切换到指定版本
    fn switch_to(&self, request: SwitchRequest) -> Result<(), String> {
        let version = &request.version;
        let base_dir = &request.base_dir;
        self.switch_version(version, base_dir)
    }

    /// 卸载指定版本
    fn uninstall(&self, request: UninstallRequest) -> Result<(), String> {
        let version = &request.version;
        let base_dir = &request.base_dir;
        let version_path = base_dir.join(version);

        if !version_path.exists() {
            return Err(format!("Go version {} is not installed", version));
        }

        // 检查要卸载的版本是否为当前活跃版本
        if let Some(current_version) = self.get_current_version(base_dir) {
            if current_version == *version {
                return Err(format!(
                    "Cannot uninstall Go {} as it is currently active. Please switch to another version or clear the current symlink first.",
                    version
                ));
            }
        }

        std::fs::remove_dir_all(&version_path)
            .map_err(|e| format!("Failed to remove Go {}: {}", version, e))?;

        Ok(())
    }

    /// 列出已安装的版本
    ///
    /// 扫描基础目录中的所有子目录，返回包含有效Go安装的版本目录列表。
    /// 自动排除：
    /// - `current` 目录（junction point）
    /// - 没有 `bin/go` 或 `bin/go.exe` 的目录
    /// - 非目录文件
    ///
    /// 返回的版本列表按字母顺序排序。
    fn list_installed(&self, request: ListInstalledRequest) -> Result<VersionList, String> {
        let base_dir = &request.base_dir;
        if !base_dir.exists() {
            return Err(format!("Base directory does not exist: {}", base_dir.display()));
        }

        let mut versions = Vec::new();

        match std::fs::read_dir(base_dir) {
            Ok(entries) => {
                for entry in entries.flatten() {
                    let name = entry.file_name().to_string_lossy().to_string();

                    // 跳过current目录（junction point）
                    if name == "current" {
                        continue;
                    }

                    // 检查是否为目录且包含Go二进制文件
                    if entry.path().is_dir() {
                        let go_binary_name =
                            if cfg!(target_os = "windows") { "go.exe" } else { "go" };
                        let go_binary = entry.path().join("bin").join(go_binary_name);
                        if go_binary.exists() {
                            versions.push(name);
                        }
                    }
                }
            }
            Err(e) => return Err(format!("Failed to read directory: {}", e)),
        }

        versions.sort();
        let total_count = versions.len();

        Ok(VersionList { versions, total_count })
    }

    /// 列出可用的版本（实时获取）
    async fn list_available(&self) -> Result<VersionList, String> {
        // 从Go官方API获取版本列表
        let url = "https://go.dev/dl/?mode=json";

        let resp = reqwest::get(url).await.map_err(|e| format!("{}", e))?;
        let releases: serde_json::Value = resp.json().await.map_err(|e| format!("{}", e))?;

        let mut versions = Vec::new();

        if let Some(array) = releases.as_array() {
            for release in array {
                if let Some(version_str) = release["version"].as_str() {
                    // 去掉 "go" 前缀
                    if let Some(version) = version_str.strip_prefix("go") {
                        versions.push(version.to_string());
                    }
                }
            }
        }

        // 只返回稳定版本，过滤掉 beta 和 rc 版本
        versions.retain(|v| !v.contains("beta") && !v.contains("rc"));

        // 倒序排列，最新版本在前
        versions.sort_by(|a, b| {
            // 简单的版本比较
            let a_parts: Vec<u32> = a.split('.').filter_map(|s| s.parse().ok()).collect();
            let b_parts: Vec<u32> = b.split('.').filter_map(|s| s.parse().ok()).collect();
            b_parts.cmp(&a_parts)
        });

        let total_count = versions.len();

        Ok(VersionList { versions, total_count })
    }

    /// 获取当前状态
    fn status(&self, request: StatusRequest) -> Result<RuntimeStatus, String> {
        let base_dir = request.base_dir.as_deref();
        let mut environment_vars = HashMap::new();

        let goroot = std::env::var("GOROOT").unwrap_or_else(|_| "Not set".to_string());
        let gopath = std::env::var("GOPATH").unwrap_or_else(|_| "Not set".to_string());

        environment_vars.insert("GOROOT".to_string(), goroot.clone());
        environment_vars.insert("GOPATH".to_string(), gopath);
        let mut current_version = None;
        let mut install_path = None;
        let mut link_info = None;

        if let Some(base_dir) = base_dir {
            current_version = self.get_current_version(base_dir);

            if let Some(ref version) = current_version {
                install_path = Some(base_dir.join(version));
            }
            #[cfg(target_os = "windows")]
            {
                link_info = Some(self.get_symlink_info(base_dir));
            }
        }

        Ok(RuntimeStatus { current_version, install_path, environment_vars, link_info })
    }
}
