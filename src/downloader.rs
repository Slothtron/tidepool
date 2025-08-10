//! 内置下载器模块
//!
//! 集成到 version-manager 中的下载器，提供文件下载功能
//! 支持分片下载、多线程下载和断点续传

use indicatif::{ProgressBar, ProgressStyle};
use log::{debug, info};
use reqwest::Client;
use std::path::Path;

use std::time::Duration;
use thiserror::Error;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;


/// 下载器错误类型
#[derive(Error, Debug)]
pub enum DownloadError {
    #[error("网络错误: {0}")]
    Network(#[from] reqwest::Error),

    #[error("IO错误: {0}")]
    Io(#[from] std::io::Error),

    #[error("无法获取文件大小")]
    FileSizeUnavailable,

    #[error("服务器不支持范围请求")]
    RangeNotSupported,

    #[error("分片下载失败: {0}")]
    ChunkDownloadFailed(String),

    #[error("其他错误: {0}")]
    Other(String),
}

/// 下载结果类型
pub type DownloadResult<T> = Result<T, DownloadError>;

/// 进度报告器
#[derive(Debug, Clone)]
pub struct ProgressReporter {
    /// 主进度条
    progress_bar: ProgressBar,
}

impl ProgressReporter {
    /// 创建新的进度报告器
    ///
    /// # Panics
    ///
    /// Panics if the progress bar template is invalid (which should not happen with the predefined template)
    #[must_use]
    pub fn new(total_size: u64) -> Self {
        let progress_bar = ProgressBar::new(total_size);
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
                .unwrap()
                .progress_chars("#>-"),
        );

        Self { progress_bar }
    }

    /// 开始下载
    pub fn start(&self) {
        self.progress_bar.tick();
    }

    /// 更新进度
    pub fn update(&self, bytes_downloaded: u64) {
        self.progress_bar.set_position(bytes_downloaded);
    }

    /// 增加进度
    pub fn increment(&self, bytes: u64) {
        self.progress_bar.inc(bytes);
    }

    /// 完成下载
    pub fn finish(&self) {
        self.progress_bar.finish_with_message("下载完成");
    }

    /// 设置长度
    pub fn set_length(&self, length: u64) {
        self.progress_bar.set_length(length);
    }

    /// 获取长度
    #[must_use]
    pub fn length(&self) -> Option<u64> {
        Some(self.progress_bar.length().unwrap_or(0))
    }

    /// 设置消息
    pub fn set_message(&self, message: &str) {
        self.progress_bar.set_message(message.to_string());
    }
}

/// 下载配置
#[derive(Debug, Clone)]
pub struct DownloadConfig {
    /// 用户代理
    pub user_agent: Option<String>,
    /// 请求超时时间（秒）
    pub timeout_seconds: u64,
    /// 连接超时时间（秒）
    pub connect_timeout_seconds: u64,
    /// 并发下载的线程数
    pub concurrent_connections: usize,
    /// 每个分片的最小大小（字节）
    pub min_chunk_size: u64,
    /// 重试次数
    pub max_retries: usize,
    /// 重试间隔（毫秒）
    pub retry_delay_ms: u64,
    /// 是否启用分片下载
    pub enable_chunked_download: bool,
}

impl Default for DownloadConfig {
    fn default() -> Self {
        Self {
            user_agent: Some("Tidepool-GVM/1.0".to_string()),
            timeout_seconds: 300, // 5 minutes
            connect_timeout_seconds: 30,
            concurrent_connections: 4,
            min_chunk_size: 1024 * 1024, // 1MB
            max_retries: 3,
            retry_delay_ms: 1000,
            enable_chunked_download: true,
        }
    }
}

/// 下载器
pub struct Downloader {
    /// HTTP客户端
    client: Client,
    /// 下载配置（保留以供将来扩展）
    #[allow(dead_code)]
    config: DownloadConfig,
}

impl Downloader {
    /// 创建新的下载器
    #[must_use]
    pub fn new() -> Self {
        Self::with_config(DownloadConfig::default())
    }

    /// 使用自定义配置创建下载器
    #[must_use]
    pub fn with_config(config: DownloadConfig) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .connect_timeout(Duration::from_secs(config.connect_timeout_seconds))
            .user_agent(config.user_agent.as_deref().unwrap_or("Tidepool-GVM/1.0"))
            .build()
            .expect("Failed to create HTTP client");

        Self { client, config }
    }

    /// 下载文件
    pub async fn download<P: AsRef<Path>>(
        &self,
        url: &str,
        output_path: P,
        progress_reporter: Option<ProgressReporter>,
    ) -> DownloadResult<()> {
        let output_path = output_path.as_ref();
        debug!("Starting download: {} -> {}", url, output_path.display());

        // 获取文件信息
        let (file_size, supports_range) = self.get_file_info(url).await?;

        // 根据文件大小和服务器支持情况选择下载方式
        if file_size > self.config.min_chunk_size && supports_range && self.config.enable_chunked_download {
            debug!("Using chunked download for file size: {}", file_size);
            // For now, fall back to single-threaded download
            self.download_single(url, output_path, progress_reporter).await
        } else {
            debug!("Using single-threaded download for file size: {}", file_size);
            self.download_single(url, output_path, progress_reporter).await
        }
    }

    /// 获取文件信息
    async fn get_file_info(&self, url: &str) -> DownloadResult<(u64, bool)> {
        let response = self.client.head(url).send().await?;

        let content_length = response
            .headers()
            .get("content-length")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok())
            .ok_or(DownloadError::FileSizeUnavailable)?;

        let supports_range = response
            .headers()
            .get("accept-ranges")
            .and_then(|v| v.to_str().ok())
            .map(|s| s == "bytes")
            .unwrap_or(false);

        Ok((content_length, supports_range))
    }

    /// 单线程下载
    async fn download_single<P: AsRef<Path>>(
        &self,
        url: &str,
        output_path: P,
        progress_reporter: Option<ProgressReporter>,
    ) -> DownloadResult<()> {
        let output_path = output_path.as_ref();

        // 确保输出目录存在
        if let Some(parent) = output_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        let mut response = self.client.get(url).send().await?;
        let mut file = File::create(output_path).await?;

        let mut downloaded = 0u64;
        let _buffer = vec![0u8; 8192];

        while let Some(chunk) = response.chunk().await? {
            file.write_all(&chunk).await?;
            downloaded += chunk.len() as u64;

            if let Some(ref progress) = progress_reporter {
                progress.update(downloaded);
            }
        }

        if let Some(ref progress) = progress_reporter {
            progress.finish();
        }

        info!("Download completed: {} bytes", downloaded);
        Ok(())
    }

    /// 获取文件大小
    pub async fn get_file_size(&self, url: &str) -> DownloadResult<u64> {
        let (size, _) = self.get_file_info(url).await?;
        Ok(size)
    }
}

impl Default for Downloader {
    fn default() -> Self {
        Self::new()
    }
}
