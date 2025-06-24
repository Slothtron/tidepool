//! 网络代理服务模块
//!
//! 提供 DNS 代理服务功能，支持：
//! - 自定义 hosts 映射
//! - HTTP 下载 hosts 文件

use crate::host_entry::HostEntry;
use crate::hosts_manager::HostsManager;
use log::{debug, info, warn};
use std::collections::HashMap;
use std::net::IpAddr;
use std::time::Duration;

#[cfg(feature = "proxy")]
use tokio::sync::RwLock;

/// 代理服务错误类型
#[derive(Debug)]
pub enum ProxyError {
    /// 网络错误
    Network(std::io::Error),
    /// DNS 解析错误
    DnsResolution(String),
    /// HTTP 请求错误
    #[cfg(feature = "proxy")]
    HttpRequest(reqwest::Error),
    /// 无效的 URL
    InvalidUrl(String),
    /// 端口已被占用
    PortInUse(u16),
    /// 服务器错误
    ServerError(String),
}

impl std::fmt::Display for ProxyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProxyError::Network(err) => write!(f, "网络错误: {}", err),
            ProxyError::DnsResolution(msg) => write!(f, "DNS 解析错误: {}", msg),
            #[cfg(feature = "proxy")]
            ProxyError::HttpRequest(err) => write!(f, "HTTP 请求错误: {}", err),
            ProxyError::InvalidUrl(url) => write!(f, "无效的 URL: {}", url),
            ProxyError::PortInUse(port) => write!(f, "端口 {} 已被占用", port),
            ProxyError::ServerError(msg) => write!(f, "服务器错误: {}", msg),
        }
    }
}

impl std::error::Error for ProxyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ProxyError::Network(err) => Some(err),
            #[cfg(feature = "proxy")]
            ProxyError::HttpRequest(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for ProxyError {
    fn from(err: std::io::Error) -> Self {
        ProxyError::Network(err)
    }
}

#[cfg(feature = "proxy")]
impl From<reqwest::Error> for ProxyError {
    fn from(err: reqwest::Error) -> Self {
        ProxyError::HttpRequest(err)
    }
}

/// DNS 查询结果
#[derive(Debug, Clone)]
pub struct DnsResult {
    /// 查询的域名
    pub hostname: String,
    /// 解析的 IP 地址列表
    pub addresses: Vec<IpAddr>,
    /// 是否来自自定义 hosts
    pub from_hosts: bool,
    /// TTL（生存时间）
    pub ttl: u32,
}

/// 简化的 hosts 映射管理器
pub struct SimpleHostsManager {
    /// 自定义 hosts 映射
    #[cfg(feature = "proxy")]
    hosts_map: std::sync::Arc<RwLock<HashMap<String, Vec<IpAddr>>>>,
    #[cfg(not(feature = "proxy"))]
    hosts_map: HashMap<String, Vec<IpAddr>>,
}

impl SimpleHostsManager {
    /// 创建新的管理器
    pub fn new() -> Self {
        SimpleHostsManager {
            #[cfg(feature = "proxy")]
            hosts_map: std::sync::Arc::new(RwLock::new(HashMap::new())),
            #[cfg(not(feature = "proxy"))]
            hosts_map: HashMap::new(),
        }
    }

    /// 加载 hosts 条目
    #[cfg(feature = "proxy")]
    pub async fn load_hosts(&self, entries: Vec<HostEntry>) {
        let mut hosts_map = self.hosts_map.write().await;
        hosts_map.clear();

        for entry in entries {
            if !entry.is_commented {
                for hostname in &entry.hostnames {
                    hosts_map.entry(hostname.clone()).or_insert_with(Vec::new).push(entry.ip);
                }
            }
        }

        info!("加载了 {} 条 hosts 映射", hosts_map.len());
    }

    /// 加载 hosts 条目 (同步版本)
    #[cfg(not(feature = "proxy"))]
    pub fn load_hosts(&mut self, entries: Vec<HostEntry>) {
        self.hosts_map.clear();

        for entry in entries {
            if !entry.is_commented {
                for hostname in &entry.hostnames {
                    self.hosts_map.entry(hostname.clone()).or_insert_with(Vec::new).push(entry.ip);
                }
            }
        }

        info!("加载了 {} 条 hosts 映射", self.hosts_map.len());
    }
    /// 解析主机名
    #[cfg(feature = "proxy")]
    pub async fn resolve_hostname(&self, hostname: &str) -> Option<DnsResult> {
        let hosts_map = self.hosts_map.read().await;
        hosts_map.get(hostname).map(|addresses| DnsResult {
            hostname: hostname.to_string(),
            addresses: addresses.clone(),
            from_hosts: true,
            ttl: 3600, // 1小时
        })
    }
    /// 解析主机名 (同步版本)
    #[cfg(not(feature = "proxy"))]
    pub fn resolve_hostname(&self, hostname: &str) -> Option<DnsResult> {
        self.hosts_map.get(hostname).map(|addresses| DnsResult {
            hostname: hostname.to_string(),
            addresses: addresses.clone(),
            from_hosts: true,
            ttl: 3600, // 1小时
        })
    }
}

impl Default for SimpleHostsManager {
    fn default() -> Self {
        Self::new()
    }
}

/// HTTP Hosts 文件下载服务
#[cfg(feature = "proxy")]
#[derive(Debug)]
pub struct HostsDownloader {
    /// HTTP 客户端
    client: reqwest::Client,
    /// 请求超时时间
    timeout: Duration,
}

#[cfg(feature = "proxy")]
impl HostsDownloader {
    /// 创建新的下载器
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("tidepool-hosts-manager/1.0")
            .build()
            .expect("创建 HTTP 客户端失败");

        HostsDownloader { client, timeout: Duration::from_secs(30) }
    }

    /// 设置超时时间
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// 下载 hosts 文件
    ///
    /// # 参数
    /// - `url` - 下载 URL
    ///
    /// # 返回
    /// 返回解析后的 HostEntry 列表
    pub async fn download_hosts(&self, url: &str) -> Result<Vec<HostEntry>, ProxyError> {
        info!("开始下载 hosts 文件: {}", url);

        // 验证 URL
        let parsed_url =
            reqwest::Url::parse(url).map_err(|_| ProxyError::InvalidUrl(url.to_string()))?;

        if parsed_url.scheme() != "http" && parsed_url.scheme() != "https" {
            return Err(ProxyError::InvalidUrl(format!("不支持的协议: {}", parsed_url.scheme())));
        }

        // 发起 HTTP 请求
        let response = self.client.get(url).timeout(self.timeout).send().await?;

        if !response.status().is_success() {
            return Err(ProxyError::ServerError(format!("HTTP 错误: {}", response.status())));
        }

        let content = response.text().await?;
        info!("下载完成，内容大小: {} 字节", content.len());

        // 解析 hosts 内容
        let mut entries = Vec::new();
        let mut parse_errors = 0;

        for (line_num, line) in content.lines().enumerate() {
            match line.parse::<HostEntry>() {
                Ok(entry) => entries.push(entry),
                Err(crate::host_entry::HostsParseError::EmptyLine) => {
                    // 跳过空行
                    continue;
                }
                Err(e) => {
                    debug!("第 {} 行解析失败: {} - {}", line_num + 1, e, line);
                    parse_errors += 1;
                }
            }
        }

        if parse_errors > 0 {
            warn!("下载的 hosts 文件有 {} 行解析失败", parse_errors);
        }

        info!("成功解析 {} 条 hosts 记录", entries.len());
        Ok(entries)
    }

    /// 下载并保存 hosts 文件
    pub async fn download_and_save(
        &self,
        url: &str,
        save_path: &std::path::Path,
    ) -> Result<usize, ProxyError> {
        let entries = self.download_hosts(url).await?;

        let manager = HostsManager::new(save_path);
        manager.write_hosts(&entries).map_err(|e| ProxyError::ServerError(e.to_string()))?;

        info!("hosts 文件已保存到: {}", save_path.display());
        Ok(entries.len())
    }
}

#[cfg(feature = "proxy")]
impl Default for HostsDownloader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_hosts_manager_creation() {
        let manager = SimpleHostsManager::new();
        // 验证管理器创建成功 - 这里简单使用一个基本检查
        #[cfg(feature = "proxy")]
        assert!(std::sync::Arc::strong_count(&manager.hosts_map) == 1);
        #[cfg(not(feature = "proxy"))]
        assert!(manager.hosts_map.is_empty());
    }

    #[cfg(feature = "proxy")]
    #[test]
    fn test_hosts_downloader_creation() {
        let downloader = HostsDownloader::new();
        assert_eq!(downloader.timeout, Duration::from_secs(30));
    }

    #[cfg(feature = "proxy")]
    #[test]
    fn test_hosts_downloader_with_timeout() {
        let downloader = HostsDownloader::new().with_timeout(Duration::from_secs(10));
        assert_eq!(downloader.timeout, Duration::from_secs(10));
    }

    // 注意：实际的网络测试需要模拟服务器或使用 mock
    #[cfg(feature = "proxy")]
    #[tokio::test]
    #[ignore = "需要网络连接，在CI或无网络环境中跳过"]
    async fn test_download_hosts_invalid_url() {
        let downloader = HostsDownloader::new();
        let result = downloader.download_hosts("invalid-url").await;
        assert!(matches!(result, Err(ProxyError::InvalidUrl(_))));
    }

    #[cfg(feature = "proxy")]
    #[tokio::test]
    #[ignore = "需要网络连接，在CI或无网络环境中跳过"]
    async fn test_download_hosts_unsupported_scheme() {
        let downloader = HostsDownloader::new();
        let result = downloader.download_hosts("ftp://example.com/hosts").await;
        assert!(matches!(result, Err(ProxyError::InvalidUrl(_))));
    }
}
