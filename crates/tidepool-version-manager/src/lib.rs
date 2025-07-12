// 运行时版本管理模块入口
pub mod go;

// 内部下载器模块（原 tidepool-downloader）
pub mod downloader;

// Windows Junction 工具模块
#[cfg(target_os = "windows")]
pub mod junction_utils;

// 重新导出 GoVersionInfo
pub use go::GoVersionInfo;

// 未来可扩展其他语言版本管理
// pub mod python;
// pub mod nodejs;

use std::path::PathBuf;

pub enum Runtime {
    Go,
    // 未来可扩展: Python, Nodejs
}

/// 版本信息结构体
#[derive(Debug, Clone)]
pub struct VersionInfo {
    pub version: String,
    pub install_path: PathBuf,
}

/// 安装请求
#[derive(Debug, Clone)]
pub struct InstallRequest {
    /// 要安装的版本号
    pub version: String,
    /// 安装目录
    pub install_dir: PathBuf,
    /// 下载目录（由调用方管理，可能是缓存目录）
    pub download_dir: PathBuf,
    /// 是否强制重新安装
    pub force: bool,
}

/// 切换版本请求
#[derive(Debug, Clone)]
pub struct SwitchRequest {
    /// 目标版本号
    pub version: String,
    /// 版本管理根目录
    pub base_dir: PathBuf,
    /// 是否全局设置
    pub global: bool,
    /// 是否强制切换
    pub force: bool,
}

/// 卸载请求
#[derive(Debug, Clone)]
pub struct UninstallRequest {
    /// 要卸载的版本号
    pub version: String,
    /// 版本管理根目录
    pub base_dir: PathBuf,
}

/// 列出已安装版本请求
#[derive(Debug, Clone)]
pub struct ListInstalledRequest {
    /// 版本管理根目录
    pub base_dir: PathBuf,
}

/// 状态查询请求
#[derive(Debug, Clone)]
pub struct StatusRequest {
    /// 版本管理根目录（可选）
    pub base_dir: Option<PathBuf>,
}

/// 运行时状态信息
#[derive(Debug, Clone)]
pub struct RuntimeStatus {
    pub current_version: Option<String>,
    pub install_path: Option<PathBuf>,
    pub environment_vars: std::collections::HashMap<String, String>,
    pub link_info: Option<String>,
}

/// 版本列表响应
#[derive(Debug, Clone)]
pub struct VersionList {
    pub versions: Vec<String>,
    pub total_count: usize,
}

// 通用版本管理接口，将来可以为Python、Node.js等实现
#[async_trait::async_trait]
pub trait VersionManager {
    /// 安装指定版本
    ///
    /// # 参数
    /// - `request`: 安装请求，包含版本号、安装目录、下载目录等信息
    ///
    /// # 返回
    /// - `Ok(VersionInfo)`: 安装成功，返回版本信息
    /// - `Err(String)`: 安装失败，返回错误信息
    async fn install(&self, request: InstallRequest) -> Result<VersionInfo, String>;

    /// 切换到指定版本
    ///
    /// # 参数
    /// - `request`: 切换请求，包含版本号、根目录、配置等信息    ///
    /// # 返回
    /// - `Ok(())`: 切换成功
    /// - `Err(String)`: 切换失败，返回错误信息
    ///
    /// # Errors
    ///
    /// Returns an error if the version switch operation fails
    fn switch_to(&self, request: SwitchRequest) -> Result<(), String>;

    /// 卸载指定版本    ///
    /// # 参数
    /// - `request`: 卸载请求，包含版本号和根目录
    ///
    /// # 返回
    /// - `Ok(())`: 卸载成功
    /// - `Err(String)`: 卸载失败，返回错误信息
    ///
    /// # Errors
    ///
    /// Returns an error if the uninstall operation fails
    fn uninstall(&self, request: UninstallRequest) -> Result<(), String>;

    /// 列出已安装的版本
    ///
    /// # 参数    /// - `request`: 列出请求，包含根目录
    ///
    /// # 返回
    /// - `Ok(VersionList)`: 成功，返回版本列表
    /// - `Err(String)`: 失败，返回错误信息
    ///
    /// # Errors
    ///
    /// Returns an error if the list operation fails
    fn list_installed(&self, request: ListInstalledRequest) -> Result<VersionList, String>;

    /// 列出可用的版本
    ///
    /// # 返回
    /// - `Ok(VersionList)`: 成功，返回版本列表
    /// - `Err(String)`: 失败，返回错误信息
    async fn list_available(&self) -> Result<VersionList, String>;

    /// 获取当前运行时状态
    ///
    /// # 参数    /// - `request`: 状态查询请求，包含可选的根目录
    ///
    /// # 返回
    /// - `Ok(RuntimeStatus)`: 成功，返回状态信息
    /// - `Err(String)`: 失败，返回错误信息
    ///
    /// # Errors
    ///
    /// Returns an error if the status query fails
    fn status(&self, request: StatusRequest) -> Result<RuntimeStatus, String>;
}
