//! # tidepool-hosts-manager
//!
//! Hosts 文件管理工具库，提供以下核心功能：
//! - Hosts 映射解析与管理
//! - 多文件分组管理
//! - 服务内 hosts 映射解析
//! - 网络代理下载服务
//!
//! **注意：此模块仅在服务内映射 hosts 记录，不接管系统 hosts 文件**
//!
//! ## 功能模块
//!
//! - [`host_entry`] - HostEntry 结构体和 hosts 文件解析
//! - [`hosts_manager`] - HostsManager 核心管理功能
//! - [`group`] - 分组管理功能
//! - [`proxy`] - 服务内映射和网络下载服务

pub mod group;
pub mod host_entry;
pub mod hosts_manager;

#[cfg(feature = "proxy")]
pub mod proxy;

// 重新导出主要类型
pub use group::{GroupError, GroupManager};
pub use host_entry::{HostEntry, HostsParseError};
pub use hosts_manager::{HostsManager, HostsManagerError};

#[cfg(feature = "proxy")]
pub use proxy::{DnsResult, HostsDownloader, ProxyError, SimpleHostsManager};

/// 库的版本信息
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// 库级别的结果类型
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_exists() {
        assert!(!VERSION.is_empty());
    }
}
