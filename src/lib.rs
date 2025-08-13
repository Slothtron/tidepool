//! Tidepool GVM - 高性能 Go 版本管理工具包
//!
//! 这是一个用 Rust 编写的高性能 Go 版本管理工具，提供简单易用的命令行界面
//! 来管理多个 Go 版本。

pub mod cli;
pub mod commands;
pub mod config;
pub mod downloader;
pub mod go;
pub mod symlink;
pub mod ui;

// 重新导出主要类型
pub use cli::Cli;
pub use downloader::Downloader;
pub use go::{GoManager, GoVersionInfo};
pub use ui::{GvmUI, UI, ProgressManager, InteractiveUI};

// 公共类型定义
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// 版本信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct VersionInfo {
    pub version: String,
    pub path: PathBuf,
    pub is_current: bool,
}

impl std::fmt::Display for VersionInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.version)
    }
}

/// 安装请求
#[derive(Debug, Clone)]
pub struct InstallRequest {
    pub version: String,
    pub force: bool,
    pub install_dir: PathBuf,
    pub download_dir: PathBuf,
}

/// 切换请求
#[derive(Debug, Clone)]
pub struct SwitchRequest {
    pub version: String,
    pub base_dir: PathBuf,
    pub global: bool,
    pub force: bool,
}

/// 卸载请求
#[derive(Debug, Clone)]
pub struct UninstallRequest {
    pub version: String,
    pub base_dir: PathBuf,
}

/// 列出已安装版本请求
#[derive(Debug, Clone)]
pub struct ListInstalledRequest {
    pub base_dir: PathBuf,
}

/// 状态请求
#[derive(Debug, Clone)]
pub struct StatusRequest {
    pub base_dir: Option<PathBuf>,
}

/// 运行时状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeStatus {
    pub current_version: Option<String>,
    pub go_path: Option<PathBuf>,
    pub is_installed: bool,
    pub install_path: Option<PathBuf>,
    pub environment_vars: std::collections::HashMap<String, String>,
    pub link_info: Option<String>,
}

/// 版本列表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionList {
    pub versions: Vec<VersionInfo>,
    pub total_count: usize,
}
