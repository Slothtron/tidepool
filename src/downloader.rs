//! Built-in downloader module
//!
//! Integrated downloader for version-manager, providing file download functionality.
//! Uses a simple text-based progress display, independent of `indicatif`.

use crate::progress_flat::BasicProgress;
use futures::future::try_join_all;
use futures::StreamExt;
use log::{debug, info, warn};
use reqwest::Client;
use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, Instant};
use thiserror::Error;
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncSeekExt, AsyncWriteExt, SeekFrom};
use tokio::sync::Semaphore;

/// Downloader error types
#[derive(Error, Debug)]
pub enum DownloadError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not get file size")]
    FileSize,
    #[error("Server does not support range requests")]
    RangeNotSupported,
    #[error("Chunk download failed: {0}")]
    ChunkDownloadFailed(String),
    #[error("Other error: {0}")]
    Other(String),
}

/// Download result type
pub type DownloadResult<T> = Result<T, DownloadError>;

/// Simplified progress callback type
pub type ProgressCallback = Box<dyn Fn(u64, u64) + Send + Sync>;

/// Chunk download information
#[derive(Debug, Clone)]
pub struct ChunkInfo {
    /// Chunk index
    pub index: usize,
    /// Start position
    pub start: u64,
    /// End position
    pub end: u64,
    /// Chunk size
    pub size: u64,
}

impl ChunkInfo {
    /// Creates new chunk information
    pub fn new(index: usize, start: u64, end: u64) -> Self {
        Self { index, start, end, size: end - start + 1 }
    }
}

/// Download configuration
#[derive(Debug, Clone)]
pub struct DownloadConfig {
    /// User-Agent
    pub user_agent: String,
    /// Request timeout in seconds
    pub timeout: Duration,
    /// Connection timeout in seconds
    pub connect_timeout: Duration,
    /// Number of concurrent download threads
    pub concurrent_connections: usize,
    /// Minimum size of each chunk in bytes
    pub min_chunk_size: u64,
    /// Number of retries
    pub max_retries: u32,
    /// Retry interval in milliseconds
    pub retry_delay_ms: u64,
    /// Whether to enable chunked downloading
    pub enable_chunked_download: bool,
}

impl Default for DownloadConfig {
    fn default() -> Self {
        Self {
            user_agent: "Tidepool-GVM/1.0".to_string(),
            timeout: Duration::from_secs(300), // 5 minutes
            connect_timeout: Duration::from_secs(30),
            concurrent_connections: 4,
            min_chunk_size: 1024 * 1024, // 1MB
            max_retries: 3,
            retry_delay_ms: 1000,
            enable_chunked_download: true,
        }
    }
}

/// Downloader
pub struct Downloader {
    /// HTTP client
    client: Client,
    /// Download configuration (reserved for future extension)
    #[allow(dead_code)]
    config: DownloadConfig,
}

impl Downloader {
    /// Creates a new downloader
    #[must_use]
    pub fn new() -> Self {
        let config = DownloadConfig::default();
        Self::with_config(config)
    }

    /// Creates a downloader with custom configuration
    #[must_use]
    pub fn with_config(config: DownloadConfig) -> Self {
        let client = Client::builder()
            .user_agent(&config.user_agent)
            .timeout(config.timeout)
            .connect_timeout(config.connect_timeout)
            .build()
            .unwrap();
        Self { client, config }
    }

    /// Downloads a file (simplified version, without using `indicatif`)
    pub async fn download(&self, url: &str, output_path: impl AsRef<Path>) -> DownloadResult<()> {
        let output_path = output_path.as_ref();

        // Get file information
        let total_size = self.get_file_size(url).await?;

        // Choose download method based on file size and server support
        if self.config.enable_chunked_download && total_size > self.config.min_chunk_size {
            self.download_chunked(url, output_path, total_size, None).await
        } else {
            self.download_single_threaded(url, output_path, None).await
        }
    }

    /// Downloads a file and displays simple progress
    pub async fn download_with_simple_progress(
        &self,
        url: &str,
        output_path: impl AsRef<Path>,
        filename: &str,
    ) -> DownloadResult<()> {
        let output_path = output_path.as_ref();
        // Get file information
        let total_size = self.get_file_size(url).await?;

        // Create progress bar display
        let progress = BasicProgress::new(format!("Downloading {filename}"));
        let progress_clone = progress.clone();

        // Ensure the output directory exists
        if let Some(parent) = output_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        let callback: ProgressCallback = Box::new(move |downloaded, total| {
            if total > 0 {
                progress_clone.show_download(downloaded, total);
            }
        });

        if self.config.enable_chunked_download && total_size > self.config.min_chunk_size {
            self.download_chunked(url, output_path, total_size, Some(callback)).await?;
        } else {
            self.download_single_threaded(url, output_path, Some(callback)).await?;
        }

        // Display final message upon completion
        progress.done(&format!("Downloaded {filename}"));

        Ok(())
    }

    /// Gets the file size
    async fn get_file_size(&self, url: &str) -> DownloadResult<u64> {
        let response = self.client.head(url).send().await?;
        response
            .headers()
            .get(reqwest::header::CONTENT_LENGTH)
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse().ok())
            .ok_or(DownloadError::FileSize)
    }

    /// Single-threaded download
    async fn download_single_threaded(
        &self,
        url: &str,
        output_path: impl AsRef<Path>,
        progress_callback: Option<ProgressCallback>,
    ) -> DownloadResult<()> {
        let output_path = output_path.as_ref();

        // Ensure the output directory exists
        if let Some(parent) = output_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        let response = self.client.get(url).send().await?;
        let file_size = response.content_length().unwrap_or(0);
        let mut file = File::create(output_path).await?;
        let mut downloaded: u64 = 0;
        let mut stream = response.bytes_stream();
        let last_update = Instant::now();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            file.write_all(&chunk).await?;
            downloaded += chunk.len() as u64;

            // Limit update frequency: update every 100ms or on completion
            let now = Instant::now();
            if now.duration_since(last_update).as_millis() >= 100 || downloaded == file_size {
                if let Some(ref callback) = progress_callback {
                    callback(downloaded, file_size);
                }
            }
        }
        Ok(())
    }

    /// Concurrent chunked download
    async fn download_chunked(
        &self,
        url: &str,
        output_path: impl AsRef<Path>,
        total_size: u64,
        _progress_callback: Option<ProgressCallback>,
    ) -> DownloadResult<()> {
        let output_path = output_path.as_ref();

        // Ensure the output directory exists
        if let Some(parent) = output_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        // Calculate number and size of chunks
        let chunk_size = self.config.min_chunk_size;
        let num_chunks = ((total_size + chunk_size - 1) / chunk_size)
            .min(self.config.concurrent_connections as u64) as usize;
        let actual_chunk_size = total_size / num_chunks as u64;

        debug!("Downloading {total_size} bytes in {num_chunks} chunks of ~{actual_chunk_size} bytes each");

        // Create chunk information
        let chunks: Vec<ChunkInfo> = (0..num_chunks)
            .map(|i| {
                let start = i as u64 * actual_chunk_size;
                let end = if i == num_chunks - 1 {
                    total_size - 1
                } else {
                    start + actual_chunk_size - 1
                };
                ChunkInfo::new(i, start, end)
            })
            .collect();

        // Create the output file
        {
            let file = File::create(output_path).await?;
            file.set_len(total_size).await?;
        }

        // Use a semaphore to limit concurrency
        let semaphore = Arc::new(Semaphore::new(self.config.concurrent_connections));
        let downloaded_bytes = Arc::new(std::sync::atomic::AtomicU64::new(0));

        // Simplified version: no progress callback for now to avoid lifetime issues
        // TODO: Implement a better progress callback mechanism in a future version

        // Concurrently download all chunks
        let download_tasks: Vec<_> = chunks
            .into_iter()
            .map(|chunk| {
                let client = self.client.clone();
                let url = url.to_string();
                let output_path = output_path.to_path_buf();
                let semaphore = semaphore.clone();
                let downloaded_bytes = downloaded_bytes.clone();

                let max_retries = self.config.max_retries;
                let retry_delay = Duration::from_millis(self.config.retry_delay_ms);

                tokio::spawn(async move {
                    let _permit = semaphore.acquire().await.unwrap();

                    for attempt in 1..=max_retries {
                        match Self::download_chunk(
                            &client,
                            &url,
                            &output_path,
                            &chunk,
                            &downloaded_bytes,
                            total_size,
                        )
                        .await
                        {
                            Ok(()) => return Ok(()),
                            Err(e) => {
                                warn!(
                                    "Chunk {} download attempt {}/{} failed: {}",
                                    chunk.index, attempt, max_retries, e
                                );
                                if attempt < max_retries {
                                    tokio::time::sleep(retry_delay).await;
                                } else {
                                    return Err(e);
                                }
                            }
                        }
                    }

                    Err(DownloadError::ChunkDownloadFailed(format!(
                        "Chunk {} failed after {} attempts",
                        chunk.index, max_retries
                    )))
                })
            })
            .collect();

        // Wait for all download tasks to complete
        let results: Result<Vec<_>, _> = try_join_all(download_tasks)
            .await
            .map_err(|e| DownloadError::Other(format!("Task join error: {e}")))?
            .into_iter()
            .collect();

        results?;

        info!("Chunked download completed: {total_size} bytes in {num_chunks} chunks");
        Ok(())
    }

    /// Downloads a single chunk
    async fn download_chunk(
        client: &Client,
        url: &str,
        output_path: &Path,
        chunk: &ChunkInfo,
        downloaded_bytes: &Arc<std::sync::atomic::AtomicU64>,
        total_size: u64,
    ) -> DownloadResult<()> {
        let _total_size = total_size;
        // Create a range request
        let range_header = format!("bytes={}-{}", chunk.start, chunk.end);
        let response = client.get(url).header("Range", range_header).send().await?;

        if !response.status().is_success() {
            return Err(DownloadError::ChunkDownloadFailed(format!(
                "HTTP {}: {}",
                response.status(),
                response.status().canonical_reason().unwrap_or("Unknown")
            )));
        }

        // Open the file and seek to the chunk's position
        let mut file = OpenOptions::new().write(true).open(output_path).await?;

        file.seek(SeekFrom::Start(chunk.start)).await?;

        // Download the chunk data
        let mut chunk_downloaded = 0u64;
        let mut stream = response.bytes_stream();

        while let Some(bytes_result) = StreamExt::next(&mut stream).await {
            let bytes = bytes_result.map_err(DownloadError::Network)?;
            file.write_all(&bytes).await?;

            chunk_downloaded += bytes.len() as u64;
            // Update the global downloaded byte count
            downloaded_bytes.fetch_add(bytes.len() as u64, std::sync::atomic::Ordering::Relaxed);
        }

        if chunk_downloaded != chunk.size {
            Err(DownloadError::ChunkDownloadFailed(format!(
                "Chunk {} downloaded {} bytes, expected {}",
                chunk.index, chunk_downloaded, chunk.size
            )))
        } else {
            Ok(())
        }
    }
}

/// Helper function to format file sizes in human-readable format
pub fn format_file_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut value = bytes as f64;
    let mut unit_index = 0;

    while value >= 1024.0 && unit_index < UNITS.len() - 1 {
        value /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1} {}", value, UNITS[unit_index])
    }
}

impl Default for Downloader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_info_creation() {
        let chunk = ChunkInfo::new(0, 0, 1023);
        assert_eq!(chunk.index, 0);
        assert_eq!(chunk.start, 0);
        assert_eq!(chunk.end, 1023);
        assert_eq!(chunk.size, 1024);
    }

    #[test]
    fn test_downloader_creation() {
        let downloader = Downloader::new();
        assert!(downloader.config.enable_chunked_download);
        assert_eq!(downloader.config.concurrent_connections, 4);
        assert_eq!(downloader.config.min_chunk_size, 1024 * 1024);
    }

    #[test]
    fn test_custom_config() {
        let config = DownloadConfig {
            concurrent_connections: 8,
            min_chunk_size: 512 * 1024,
            enable_chunked_download: false,
            ..Default::default()
        };

        let downloader = Downloader::with_config(config.clone());
        assert_eq!(downloader.config.concurrent_connections, 8);
        assert_eq!(downloader.config.min_chunk_size, 512 * 1024);
        assert!(!downloader.config.enable_chunked_download);
    }

    #[test]
    fn test_format_file_size() {
        assert_eq!(format_file_size(0), "0 B");
        assert_eq!(format_file_size(512), "512 B");
        assert_eq!(format_file_size(1024), "1.0 KB");
        assert_eq!(format_file_size(1536), "1.5 KB");
        assert_eq!(format_file_size(1024 * 1024), "1.0 MB");
        assert_eq!(format_file_size(1024 * 1024 * 1024), "1.0 GB");
    }

    #[tokio::test]
    async fn test_download_config_validation() {
        let config = DownloadConfig::default();
        assert!(config.timeout > Duration::from_secs(0));
        assert!(config.connect_timeout > Duration::from_secs(0));
        assert!(config.concurrent_connections > 0);
        assert!(config.min_chunk_size > 0);
        assert!(config.max_retries > 0);
        assert!(config.retry_delay_ms > 0);
    }
}
