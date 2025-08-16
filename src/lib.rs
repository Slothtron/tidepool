//! Tidepool GVM - A high-performance Go Version Manager toolkit
//!
//! This is a high-performance Go version management tool written in Rust,
//! providing a simple command-line interface to manage multiple Go versions.

// Core modules
pub mod cli;
pub mod commands;
pub mod config;

pub mod downloader;
pub mod error;
pub mod go;
pub mod platform;
pub mod symlink;

// Flattened UI and progress system
pub mod progress_flat;
pub mod ui_flat;

// Note: Deprecated nested modules have been removed.
// Please use the flattened modules directly: ui_flat.rs and progress_flat.rs

// Main type exports
pub use cli::Cli;
pub use config::Config;
pub use downloader::Downloader;
pub use error::{ErrorUtils, Result};
pub use go::{GoManager, GoVersionInfo};

// UI and progress system (flattened)
pub use progress_flat::{BasicProgress, InstallSteps};
pub use ui_flat::{format_duration, format_size, SimpleProgressBar, SimpleUI};

// Note: Deprecated backward-compatible exports have been removed.
// Please use the new flattened modules: ui_flat and progress_flat

// Public type definitions



/// Installation request
#[derive(Debug, Clone)]
pub struct InstallRequest {
    pub version: String,
    pub install_dir: std::path::PathBuf,
    pub download_dir: std::path::PathBuf,
    pub force: bool,
}

/// Switch request
#[derive(Debug, Clone)]
pub struct SwitchRequest {
    pub version: String,
    pub base_dir: std::path::PathBuf,
    pub global: bool,
    pub force: bool,
}

/// Uninstall request
#[derive(Debug, Clone)]
pub struct UninstallRequest {
    pub version: String,
    pub base_dir: std::path::PathBuf,
}

/// List installed versions request
#[derive(Debug, Clone)]
pub struct ListInstalledRequest {
    pub base_dir: std::path::PathBuf,
}

/// Status request
#[derive(Debug, Clone)]
pub struct StatusRequest {
    pub base_dir: Option<std::path::PathBuf>,
}

/// Runtime status
#[derive(Debug, Clone, Default)]
pub struct RuntimeStatus {
    pub current_version: Option<String>,
    pub current_path: Option<String>,
    pub environment_vars: std::collections::HashMap<String, String>,
}

/// Version list
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VersionList {
    pub versions: Vec<GoVersionInfo>,
    pub total_count: usize,
}
