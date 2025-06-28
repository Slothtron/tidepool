use tidepool_version_manager::downloader::{DownloadConfig, Downloader};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 测试改进的下载器功能");

    // 创建自定义配置的下载器
    let config = DownloadConfig {
        concurrent_connections: 4,
        timeout_seconds: 180,
        enable_chunked_download: true,
        max_retries: 2,
        ..Default::default()
    };

    println!("✨ 下载器配置:");
    println!("   - 并发连接数: {}", config.concurrent_connections);
    println!("   - 超时时间: {} 秒", config.timeout_seconds);
    println!("   - 分片下载: {}", if config.enable_chunked_download { "启用" } else { "禁用" });
    println!("   - 重试次数: {}", config.max_retries);
    println!("   - 最小分片: {} MB", config.min_chunk_size / 1024 / 1024);

    let downloader = Downloader::with_config(config);

    // 测试获取文件大小
    let url = "https://go.dev/dl/go1.21.0.linux-amd64.tar.gz";
    println!("📏 获取文件大小: {url}");    #[allow(clippy::cast_precision_loss)]
    match downloader.get_file_size(url).await {
        Ok(size) => {
            println!("✅ 文件大小: {} 字节 ({:.2} MB)", size, size as f64 / 1024.0 / 1024.0);
            println!("ℹ️  实际下载测试已跳过 (避免重复下载大文件)");
        }
        Err(e) => {
            println!("❌ 获取文件大小失败: {e}");
        }
    }

    println!("🎉 下载器功能测试完成！");

    Ok(())
}
