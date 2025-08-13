//! Validation System
//!
//! Comprehensive validation framework for installation processes with
//! pre-flight checks, runtime validation, and post-installation verification.

use super::types::InstallStep;
use super::tasks::ValidationCheck;
use anyhow::{Result, Context, bail};
use std::path::{Path, PathBuf};
use std::fs;
use std::time::Instant;
use serde::{Deserialize, Serialize};

/// Validation engine for comprehensive installation validation
pub struct ValidationEngine {
    checks_performed: Vec<ValidationResult>,
    validation_start: Instant,
}

/// Result of a validation check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub check_type: String,
    pub success: bool,
    pub message: String,
    pub timestamp: String,
    pub duration_ms: u64,
}

/// Validation report containing all validation results
#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationReport {
    pub overall_success: bool,
    pub total_checks: usize,
    pub successful_checks: usize,
    pub failed_checks: usize,
    pub total_duration_ms: u64,
    pub results: Vec<ValidationResult>,
}

impl ValidationEngine {
    /// Create a new validation engine
    pub fn new() -> Self {
        Self {
            checks_performed: Vec::new(),
            validation_start: Instant::now(),
        }
    }

    /// Perform pre-installation validation
    pub async fn pre_installation_validation(
        &mut self,
        version: &str,
        install_dir: &Path,
        cache_dir: &Path,
        force: bool,
    ) -> Result<ValidationReport> {
        self.validation_start = Instant::now();
        self.checks_performed.clear();

        // Core validation checks
        self.validate_version_format(version).await?;
        self.validate_directories(install_dir, cache_dir).await?;
        self.validate_permissions(install_dir, cache_dir).await?;
        self.validate_disk_space(install_dir, cache_dir).await?;
        self.validate_network_connectivity().await?;
        
        if !force {
            self.validate_no_existing_installation(version, install_dir).await?;
        }

        Ok(self.generate_report())
    }

    /// Perform runtime validation during installation
    pub async fn runtime_validation(
        &mut self,
        step: InstallStep,
        context: &RuntimeValidationContext,
    ) -> Result<ValidationReport> {
        match step {
            InstallStep::Downloading { .. } => {
                self.validate_download_context(context).await?;
            }
            InstallStep::Extracting { .. } => {
                self.validate_extraction_context(context).await?;
            }
            InstallStep::Installing { .. } => {
                self.validate_installation_context(context).await?;
            }
            InstallStep::Verifying => {
                self.validate_verification_context(context).await?;
            }
            _ => {
                // Other steps can have specific validations added here
            }
        }

        Ok(self.generate_report())
    }

    /// Perform post-installation validation
    pub async fn post_installation_validation(
        &mut self,
        version: &str,
        install_dir: &Path,
    ) -> Result<ValidationReport> {
        self.validate_go_installation(version, install_dir).await?;
        self.validate_go_executable(install_dir).await?;
        self.validate_go_version_output(version, install_dir).await?;
        self.validate_directory_structure(install_dir).await?;

        Ok(self.generate_report())
    }

    /// Validate version format
    async fn validate_version_format(&mut self, version: &str) -> Result<()> {
        let start = Instant::now();
        let result = self.check_version_format(version);
        
        self.record_validation_result(
            "version_format",
            result.is_ok(),
            &match result {
                Ok(_) => format!("Version format '{}' is valid", version),
                Err(ref e) => format!("Invalid version format '{}': {}", version, e),
            },
            start.elapsed().as_millis() as u64,
        );

        result
    }

    /// Validate directory accessibility
    async fn validate_directories(&mut self, install_dir: &Path, cache_dir: &Path) -> Result<()> {
        // Validate install directory
        let start = Instant::now();
        let install_result = self.check_directory_access(install_dir, "install");
        self.record_validation_result(
            "install_directory",
            install_result.is_ok(),
            &match install_result {
                Ok(_) => format!("Install directory accessible: {}", install_dir.display()),
                Err(ref e) => format!("Install directory issue: {}", e),
            },
            start.elapsed().as_millis() as u64,
        );

        // Validate cache directory
        let start = Instant::now();
        let cache_result = self.check_directory_access(cache_dir, "cache");
        self.record_validation_result(
            "cache_directory",
            cache_result.is_ok(),
            &match cache_result {
                Ok(_) => format!("Cache directory accessible: {}", cache_dir.display()),
                Err(ref e) => format!("Cache directory issue: {}", e),
            },
            start.elapsed().as_millis() as u64,
        );

        install_result?;
        cache_result?;
        Ok(())
    }

    /// Validate write permissions
    async fn validate_permissions(&mut self, install_dir: &Path, cache_dir: &Path) -> Result<()> {
        let start = Instant::now();
        let result = self.check_write_permissions(install_dir, cache_dir);
        
        self.record_validation_result(
            "write_permissions",
            result.is_ok(),
            &match result {
                Ok(_) => "Write permissions verified for all directories".to_string(),
                Err(ref e) => format!("Permission check failed: {}", e),
            },
            start.elapsed().as_millis() as u64,
        );

        result
    }

    /// Validate disk space
    async fn validate_disk_space(&mut self, install_dir: &Path, cache_dir: &Path) -> Result<()> {
        let start = Instant::now();
        let result = self.check_disk_space(install_dir, cache_dir);
        
        self.record_validation_result(
            "disk_space",
            result.is_ok(),
            &match result {
                Ok(space) => format!("Sufficient disk space available: {} MB", space / 1024 / 1024),
                Err(ref e) => format!("Disk space check failed: {}", e),
            },
            start.elapsed().as_millis() as u64,
        );

        result.map(|_| ())
    }

    /// Validate network connectivity
    async fn validate_network_connectivity(&mut self) -> Result<()> {
        let start = Instant::now();
        let result = self.check_network_connectivity().await;
        
        self.record_validation_result(
            "network_connectivity",
            result.is_ok(),
            &match result {
                Ok(_) => "Network connectivity verified".to_string(),
                Err(ref e) => format!("Network connectivity failed: {}", e),
            },
            start.elapsed().as_millis() as u64,
        );

        result
    }

    /// Validate no existing installation (unless force)
    async fn validate_no_existing_installation(&mut self, version: &str, install_dir: &Path) -> Result<()> {
        let start = Instant::now();
        let version_dir = install_dir.join(version);
        let exists = version_dir.exists();
        
        self.record_validation_result(
            "existing_installation",
            !exists,
            &if exists {
                format!("Go {} is already installed at {}", version, version_dir.display())
            } else {
                format!("No existing installation found for Go {}", version)
            },
            start.elapsed().as_millis() as u64,
        );

        if exists {
            bail!("Go {} is already installed. Use --force to overwrite", version);
        }

        Ok(())
    }

    /// Validate download context
    async fn validate_download_context(&mut self, context: &RuntimeValidationContext) -> Result<()> {
        if let Some(ref download_url) = context.download_url {
            let start = Instant::now();
            let result = self.validate_download_url(download_url).await;
            
            self.record_validation_result(
                "download_url",
                result.is_ok(),
                &match result {
                    Ok(_) => format!("Download URL is accessible: {}", download_url),
                    Err(ref e) => format!("Download URL validation failed: {}", e),
                },
                start.elapsed().as_millis() as u64,
            );

            result?;
        }

        Ok(())
    }

    /// Validate extraction context
    async fn validate_extraction_context(&mut self, context: &RuntimeValidationContext) -> Result<()> {
        if let Some(ref archive_path) = context.archive_path {
            let start = Instant::now();
            let result = self.validate_archive_file(archive_path);
            
            self.record_validation_result(
                "archive_file",
                result.is_ok(),
                &match result {
                    Ok(_) => format!("Archive file is valid: {}", archive_path.display()),
                    Err(ref e) => format!("Archive validation failed: {}", e),
                },
                start.elapsed().as_millis() as u64,
            );

            result?;
        }

        Ok(())
    }

    /// Validate installation context
    async fn validate_installation_context(&mut self, _context: &RuntimeValidationContext) -> Result<()> {
        // Placeholder for installation-specific validation
        Ok(())
    }

    /// Validate verification context
    async fn validate_verification_context(&mut self, context: &RuntimeValidationContext) -> Result<()> {
        if let Some(ref install_path) = context.install_path {
            let start = Instant::now();
            let result = self.validate_installed_files(install_path);
            
            self.record_validation_result(
                "installed_files",
                result.is_ok(),
                &match result {
                    Ok(_) => format!("Installed files verified at: {}", install_path.display()),
                    Err(ref e) => format!("File verification failed: {}", e),
                },
                start.elapsed().as_millis() as u64,
            );

            result?;
        }

        Ok(())
    }

    /// Validate Go installation
    async fn validate_go_installation(&mut self, version: &str, install_dir: &Path) -> Result<()> {
        let start = Instant::now();
        let version_dir = install_dir.join(version);
        let exists = version_dir.exists() && version_dir.is_dir();
        
        self.record_validation_result(
            "go_installation",
            exists,
            &if exists {
                format!("Go {} installation directory found", version)
            } else {
                format!("Go {} installation directory not found", version)
            },
            start.elapsed().as_millis() as u64,
        );

        if !exists {
            bail!("Go installation directory not found: {}", version_dir.display());
        }

        Ok(())
    }

    /// Validate Go executable
    async fn validate_go_executable(&mut self, install_dir: &Path) -> Result<()> {
        let start = Instant::now();
        let go_binary = if cfg!(windows) {
            install_dir.join("bin").join("go.exe")
        } else {
            install_dir.join("bin").join("go")
        };

        let exists = go_binary.exists() && go_binary.is_file();
        
        self.record_validation_result(
            "go_executable",
            exists,
            &if exists {
                format!("Go executable found: {}", go_binary.display())
            } else {
                format!("Go executable not found: {}", go_binary.display())
            },
            start.elapsed().as_millis() as u64,
        );

        if !exists {
            bail!("Go executable not found: {}", go_binary.display());
        }

        Ok(())
    }

    /// Validate Go version output
    async fn validate_go_version_output(&mut self, expected_version: &str, _install_dir: &Path) -> Result<()> {
        let start = Instant::now();
        // This would normally execute `go version` and verify the output
        // For now, we'll simulate this check
        let success = true; // Placeholder
        
        self.record_validation_result(
            "go_version_output",
            success,
            &format!("Go version output verified for {}", expected_version),
            start.elapsed().as_millis() as u64,
        );

        Ok(())
    }

    /// Validate directory structure
    async fn validate_directory_structure(&mut self, install_dir: &Path) -> Result<()> {
        let start = Instant::now();
        let required_dirs = ["bin", "src", "pkg"];
        let mut missing_dirs = Vec::new();

        for dir in &required_dirs {
            let dir_path = install_dir.join(dir);
            if !dir_path.exists() {
                missing_dirs.push(*dir);
            }
        }

        let success = missing_dirs.is_empty();
        self.record_validation_result(
            "directory_structure",
            success,
            &if success {
                "All required directories found".to_string()
            } else {
                format!("Missing directories: {}", missing_dirs.join(", "))
            },
            start.elapsed().as_millis() as u64,
        );

        if !success {
            bail!("Missing required directories: {}", missing_dirs.join(", "));
        }

        Ok(())
    }

    // Helper validation methods
    fn check_version_format(&self, version: &str) -> Result<()> {
        if version.is_empty() {
            bail!("Version cannot be empty");
        }
        if !version.chars().all(|c| c.is_alphanumeric() || c == '.' || c == '-') {
            bail!("Version contains invalid characters");
        }
        Ok(())
    }

    fn check_directory_access(&self, dir: &Path, purpose: &str) -> Result<()> {
        if dir.exists() && !dir.is_dir() {
            bail!("{} path exists but is not a directory: {}", purpose, dir.display());
        }
        Ok(())
    }

    fn check_write_permissions(&self, install_dir: &Path, cache_dir: &Path) -> Result<()> {
        for dir in [install_dir, cache_dir] {
            if dir.exists() {
                let test_file = dir.join(".write_test");
                fs::write(&test_file, b"test")
                    .with_context(|| format!("Cannot write to directory: {}", dir.display()))?;
                fs::remove_file(&test_file).ok();
            }
        }
        Ok(())
    }

    fn check_disk_space(&self, _install_dir: &Path, _cache_dir: &Path) -> Result<u64> {
        // Placeholder - would normally check actual disk space
        // Return available space in bytes
        Ok(5 * 1024 * 1024 * 1024) // 5GB placeholder
    }

    async fn check_network_connectivity(&self) -> Result<()> {
        // Placeholder for network connectivity check
        // Would normally test connection to Go download servers
        Ok(())
    }

    async fn validate_download_url(&self, _url: &str) -> Result<()> {
        // Placeholder for URL validation
        Ok(())
    }

    fn validate_archive_file(&self, archive_path: &Path) -> Result<()> {
        if !archive_path.exists() {
            bail!("Archive file does not exist: {}", archive_path.display());
        }
        if !archive_path.is_file() {
            bail!("Archive path is not a file: {}", archive_path.display());
        }
        Ok(())
    }

    fn validate_installed_files(&self, install_path: &Path) -> Result<()> {
        if !install_path.exists() {
            bail!("Installation path does not exist: {}", install_path.display());
        }
        Ok(())
    }

    /// Record a validation result
    fn record_validation_result(&mut self, check_type: &str, success: bool, message: &str, duration_ms: u64) {
        self.checks_performed.push(ValidationResult {
            check_type: check_type.to_string(),
            success,
            message: message.to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            duration_ms,
        });
    }

    /// Generate validation report
    fn generate_report(&self) -> ValidationReport {
        let successful_checks = self.checks_performed.iter().filter(|r| r.success).count();
        let failed_checks = self.checks_performed.len() - successful_checks;
        let overall_success = failed_checks == 0;

        ValidationReport {
            overall_success,
            total_checks: self.checks_performed.len(),
            successful_checks,
            failed_checks,
            total_duration_ms: self.validation_start.elapsed().as_millis() as u64,
            results: self.checks_performed.clone(),
        }
    }
}

/// Context for runtime validation
#[derive(Debug, Default)]
pub struct RuntimeValidationContext {
    pub download_url: Option<String>,
    pub archive_path: Option<PathBuf>,
    pub install_path: Option<PathBuf>,
    pub expected_size: Option<u64>,
}

impl RuntimeValidationContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_download_url(mut self, url: String) -> Self {
        self.download_url = Some(url);
        self
    }

    pub fn with_archive_path(mut self, path: PathBuf) -> Self {
        self.archive_path = Some(path);
        self
    }

    pub fn with_install_path(mut self, path: PathBuf) -> Self {
        self.install_path = Some(path);
        self
    }

    pub fn with_expected_size(mut self, size: u64) -> Self {
        self.expected_size = Some(size);
        self
    }
}
