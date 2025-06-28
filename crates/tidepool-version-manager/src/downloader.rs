//! 内置下载器模块
//!
//! 集成到 version-manager 中的下载器，提供文件下载功能
//! 支持分片下载、多线程下载和断点续传

use indicatif::{ProgressBar, ProgressStyle};
use log::{debug, info, warn};
use reqwest::Client;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;

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

impl ProgressReporter {    /// 创建新的进度报告器
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
    }    /// 获取长度
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
            user_agent: Some("tidepool-version-manager/0.1.0".to_string()),
            timeout_seconds: 120, // 增加超时时间
            connect_timeout_seconds: 30,
            concurrent_connections: 4,   // 默认4个并发连接
            min_chunk_size: 1024 * 1024, // 1MB 最小分片
            max_retries: 3,
            retry_delay_ms: 1000,
            enable_chunked_download: true,
        }
    }
}

/// 简化的下载器
#[derive(Debug, Clone)]
pub struct Downloader {
    /// HTTP客户端
    client: Client,
    /// 下载配置（保留以供将来扩展）
    #[allow(dead_code)]
    config: DownloadConfig,
}

impl Downloader {    /// 创建新的下载器
    #[must_use]
    pub fn new() -> Self {
        Self::with_config(DownloadConfig::default())
    }    /// 使用指定配置创建下载器
    ///
    /// # Panics
    ///
    /// Panics if the HTTP client cannot be created (should not happen with valid configuration)
    #[must_use]
    pub fn with_config(config: DownloadConfig) -> Self {
        let mut client_builder = Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .connect_timeout(Duration::from_secs(config.connect_timeout_seconds))
            .tcp_keepalive(Duration::from_secs(60))
            .pool_idle_timeout(Duration::from_secs(90))
            .pool_max_idle_per_host(config.concurrent_connections);

        if let Some(user_agent) = &config.user_agent {
            client_builder = client_builder.user_agent(user_agent);
        }

        let client = client_builder.build().expect("Failed to create HTTP client");

        Self { client, config }
    }    /// 下载文件（支持分片下载和多线程）
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Network request fails
    /// - File I/O operations fail
    /// - Download validation fails
    /// - Server does not support range requests when chunked download is attempted
    pub async fn download<P: AsRef<Path>>(
        &self,
        url: &str,
        output_path: P,
        progress_reporter: Option<ProgressReporter>,
    ) -> DownloadResult<()> {
        let output_path = output_path.as_ref();

        debug!("开始下载: {} -> {}", url, output_path.display());

        // 获取文件大小和检查是否支持范围请求
        let (file_size, supports_ranges) = self.get_file_info(url).await?;

        // 创建或更新进度报告器
        let reporter = if let Some(reporter) = progress_reporter {
            reporter.set_length(file_size);
            Some(reporter)
        } else {
            Some(ProgressReporter::new(file_size))
        };

        if let Some(ref reporter) = reporter {
            reporter.start();
        }

        // 决定使用分片下载还是单线程下载
        let use_chunked = self.config.enable_chunked_download
            && supports_ranges
            && file_size > self.config.min_chunk_size
            && self.config.concurrent_connections > 1;

        if use_chunked {
            info!("Using chunked download mode, file size: {file_size} bytes");
            self.download_chunked(url, output_path, file_size, reporter).await
        } else {
            info!("Using single-threaded download mode, file size: {file_size} bytes");
            self.download_single(url, output_path, reporter).await
        }
    }

    /// 获取文件信息（大小和是否支持范围请求）
    async fn get_file_info(&self, url: &str) -> DownloadResult<(u64, bool)> {
        debug!("Getting file info: {url}");

        let response = self.client.head(url).send().await?;

        if !response.status().is_success() {
            return Err(DownloadError::Other(format!(
                "Server returned error status: {}",
                response.status()
            )));
        }

        let file_size = response
            .headers()
            .get("content-length")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok())
            .ok_or(DownloadError::FileSizeUnavailable)?;        let supports_ranges = response
            .headers()
            .get("accept-ranges")
            .is_some_and(|v| v.to_str().unwrap_or("").to_lowercase() == "bytes");

        debug!("File size: {file_size} bytes, supports ranges: {supports_ranges}");
        Ok((file_size, supports_ranges))
    }

    /// 单线程下载
    async fn download_single<P: AsRef<Path>>(
        &self,
        url: &str,
        output_path: P,
        progress_reporter: Option<ProgressReporter>,
    ) -> DownloadResult<()> {
        for attempt in 1..=self.config.max_retries {
            match self.try_download_single(url, &output_path, progress_reporter.as_ref()).await {
                Ok(()) => {
                    if let Some(ref reporter) = progress_reporter {
                        reporter.finish();
                    }
                    info!("单线程下载完成: {}", output_path.as_ref().display());
                    return Ok(());
                }
                Err(e) => {
                    warn!("下载尝试 {}/{} 失败: {}", attempt, self.config.max_retries, e);
                    if attempt < self.config.max_retries {
                        tokio::time::sleep(Duration::from_millis(self.config.retry_delay_ms)).await;
                    } else {
                        return Err(e);
                    }
                }
            }
        }
        unreachable!()
    }    /// 单次下载尝试
    async fn try_download_single<P: AsRef<Path>>(
        &self,
        url: &str,
        output_path: P,
        progress_reporter: Option<&ProgressReporter>,
    ) -> DownloadResult<()> {
        use futures::stream::StreamExt;
        
        let output_path = output_path.as_ref();

        // 创建临时文件路径，添加 .tmp 后缀
        let temp_path = output_path.with_extension(match output_path.extension() {
            Some(ext) => format!("{}.tmp", ext.to_string_lossy()),
            None => "tmp".to_string(),
        });

        debug!("下载到临时文件: {}", temp_path.display());

        let response = self.client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(DownloadError::Other(format!(
                "Server returned error status: {}",
                response.status()
            )));
        }

        // 确保父目录存在
        if let Some(parent) = temp_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        let mut file = File::create(&temp_path).await?;
        let mut downloaded: u64 = 0;        let mut stream = response.bytes_stream();

        // 下载过程中如果出错，确保清理临时文件
        let download_result = async {
            while let Some(chunk_result) = stream.next().await {
                let chunk = chunk_result?;
                file.write_all(&chunk).await?;

                downloaded += chunk.len() as u64;                if let Some(reporter) = progress_reporter {
                    reporter.update(downloaded);
                }
            }

            file.flush().await?;
            file.sync_all().await?; // 确保数据写入磁盘
            Ok::<(), DownloadError>(())
        }
        .await;

        // 处理下载结果
        match download_result {
            Ok(()) => {
                // 下载成功，将临时文件重命名为目标文件
                debug!(
                    "下载完成，重命名文件: {} -> {}",
                    temp_path.display(),
                    output_path.display()
                );
                tokio::fs::rename(&temp_path, output_path).await?;
                info!("文件下载并重命名成功: {}", output_path.display());
                Ok(())
            }
            Err(e) => {
                // 下载失败，清理临时文件
                warn!("下载失败，清理临时文件: {}", temp_path.display());
                let _ = tokio::fs::remove_file(&temp_path).await; // 忽略删除错误
                Err(e)
            }
        }
    }    /// 分片下载
    #[allow(clippy::too_many_lines)]
    async fn download_chunked<P: AsRef<Path>>(
        &self,
        url: &str,
        output_path: P,
        file_size: u64,
        progress_reporter: Option<ProgressReporter>,
    ) -> DownloadResult<()> {
        use std::cmp::min;
        use std::sync::Arc;
        use tokio::fs::OpenOptions;
        use tokio::sync::Mutex;

        let output_path = output_path.as_ref();

        // 创建临时文件路径，添加 .tmp 后缀
        let temp_path = output_path.with_extension(match output_path.extension() {
            Some(ext) => format!("{}.tmp", ext.to_string_lossy()),
            None => "tmp".to_string(),
        });

        debug!("分片下载到临时文件: {}", temp_path.display());

        // 确保父目录存在
        if let Some(parent) = temp_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        // 创建临时文件
        let file = Arc::new(Mutex::new(
            OpenOptions::new().create(true).write(true).truncate(true).open(&temp_path).await?,
        ));

        // 预分配文件空间
        {
            #[allow(unused_mut)]
            let mut file_guard = file.lock().await;
            file_guard.set_len(file_size).await?;
        }

        // 计算分片大小和数量
        let chunk_size = std::cmp::max(
            file_size / self.config.concurrent_connections as u64,
            self.config.min_chunk_size,
        );

        let mut chunks = Vec::new();
        let mut start = 0;

        while start < file_size {
            let end = min(start + chunk_size - 1, file_size - 1);
            chunks.push((start, end));
            start = end + 1;
        }

        info!("分片下载: {} 个分片，每片大约 {} 字节", chunks.len(), chunk_size);

        // 共享进度计数器
        let progress_counter = Arc::new(Mutex::new(0u64));

        // 启动下载任务
        let mut handles = Vec::new();
        let semaphore = Arc::new(tokio::sync::Semaphore::new(self.config.concurrent_connections));

        for (chunk_start, chunk_end) in chunks {
            let client = self.client.clone();
            let url = url.to_string();
            let file = Arc::clone(&file);
            let progress_counter = Arc::clone(&progress_counter);
            let progress_reporter = progress_reporter.clone();
            let semaphore = Arc::clone(&semaphore);
            let max_retries = self.config.max_retries;
            let retry_delay = self.config.retry_delay_ms;

            let handle = tokio::spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();

                for attempt in 1..=max_retries {
                    match Self::download_chunk(
                        &client,
                        &url,
                        chunk_start,
                        chunk_end,
                        Arc::clone(&file),
                        Arc::clone(&progress_counter),
                        progress_reporter.as_ref(),
                    )
                    .await
                    {
                        Ok(()) => return Ok(()),
                        Err(e) => {
                            warn!(
                                "分片 {chunk_start}-{chunk_end} 下载尝试 {attempt}/{max_retries} 失败: {e}"
                            );
                            if attempt < max_retries {
                                tokio::time::sleep(Duration::from_millis(retry_delay)).await;
                            } else {
                                return Err(e);
                            }
                        }
                    }
                }
                unreachable!()
            });

            handles.push(handle);
        }

        // 等待所有分片下载完成
        let download_result = async {
            for handle in handles {
                handle.await.map_err(|e| DownloadError::Other(format!("任务执行错误: {e}")))??;
            }
            Ok::<(), DownloadError>(())
        }
        .await;

        // 处理下载结果
        match download_result {
            Ok(()) => {
                // 确保文件数据写入磁盘
                {
                    let mut file_guard = file.lock().await;
                    file_guard.flush().await?;
                    file_guard.sync_all().await?;
                }

                if let Some(ref reporter) = progress_reporter {
                    reporter.finish();
                }

                // 下载成功，将临时文件重命名为目标文件
                debug!(
                    "分片下载完成，重命名文件: {} -> {}",
                    temp_path.display(),
                    output_path.display()
                );
                tokio::fs::rename(&temp_path, output_path).await?;
                info!("分片文件下载并重命名成功: {}", output_path.display());
                Ok(())
            }
            Err(e) => {
                // 下载失败，清理临时文件
                warn!("分片下载失败，清理临时文件: {}", temp_path.display());
                let _ = tokio::fs::remove_file(&temp_path).await; // 忽略删除错误
                Err(e)
            }
        }
    }    /// 下载单个分片
    async fn download_chunk(
        client: &Client,
        url: &str,
        start: u64,
        end: u64,
        file: Arc<Mutex<File>>,
        progress_counter: Arc<Mutex<u64>>,
        progress_reporter: Option<&ProgressReporter>,
    ) -> DownloadResult<()> {
        use tokio::io::{AsyncSeekExt, AsyncWriteExt, SeekFrom};

        debug!("下载分片: {start}-{end}");

        let range_header = format!("bytes={start}-{end}");
        let response = client.get(url).header("Range", range_header).send().await?;

        if !response.status().is_success() && response.status().as_u16() != 206 {
            return Err(DownloadError::ChunkDownloadFailed(format!(
                "分片下载失败，状态码: {}",
                response.status()
            )));
        }

        let chunk_data = response.bytes().await?;

        // 写入文件
        {
            let mut file_guard = file.lock().await;
            file_guard.seek(SeekFrom::Start(start)).await?;
            file_guard.write_all(&chunk_data).await?;
            file_guard.flush().await?;
        }

        // 更新进度
        {
            let mut counter = progress_counter.lock().await;
            *counter += chunk_data.len() as u64;            if let Some(reporter) = progress_reporter {
                reporter.update(*counter);
            }
        }

        debug!("分片 {start}-{end} 下载完成");
        Ok(())
    }

    /// 获取文件大小
    /// 获取文件大小
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Network request fails
    /// - Server does not provide content-length header
    /// - Content-length value is invalid
    pub async fn get_file_size(&self, url: &str) -> DownloadResult<u64> {
        let (file_size, _) = self.get_file_info(url).await?;
        Ok(file_size)
    }
}

impl Default for Downloader {
    fn default() -> Self {
        Self::new()
    }
}
