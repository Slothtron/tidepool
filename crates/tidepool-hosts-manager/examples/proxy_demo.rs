//! 网络代理服务演示
//!
//! 演示 SimpleHostsManager 和 HostsDownloader 的功能

use tempfile::TempDir;
use tidepool_hosts_manager::{HostEntry, HostsManager, SimpleHostsManager};

#[cfg(feature = "proxy")]
use tidepool_hosts_manager::HostsDownloader;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    env_logger::init();

    println!("=== 网络代理服务演示 ===\n");

    // 演示 SimpleHostsManager
    demo_simple_hosts_manager().await?;

    #[cfg(feature = "proxy")]
    {
        // 演示 HostsDownloader
        demo_hosts_downloader().await?;
    }

    println!("\n=== 演示完成 ===");
    Ok(())
}

/// 演示简单 hosts 管理器
async fn demo_simple_hosts_manager() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- SimpleHostsManager 演示 ---");

    let manager = SimpleHostsManager::new();

    // 创建测试 hosts 条目
    let test_entries = vec![
        HostEntry::new("127.0.0.1", &["test.local", "app.local"])?,
        HostEntry::new("192.168.1.100", &["dev.example.com"])?,
        HostEntry::new("::1", &["ipv6.local"])?,
    ];

    // 加载 hosts 条目
    #[cfg(feature = "proxy")]
    manager.load_hosts(test_entries.clone()).await;
    #[cfg(not(feature = "proxy"))]
    {
        let mut manager = manager;
        manager.load_hosts(test_entries.clone());
    }

    // 解析域名
    #[cfg(feature = "proxy")]
    {
        if let Some(result) = manager.resolve_hostname("test.local").await {
            println!("解析 'test.local': {:?}", result.addresses);
        }

        if let Some(result) = manager.resolve_hostname("dev.example.com").await {
            println!("解析 'dev.example.com': {:?}", result.addresses);
        }

        // 尝试解析不存在的域名
        if let Some(result) = manager.resolve_hostname("notfound.com").await {
            println!("解析 'notfound.com': {:?}", result.addresses);
        } else {
            println!("'notfound.com' 未在 hosts 中找到");
        }
    }

    #[cfg(not(feature = "proxy"))]
    {
        if let Some(result) = manager.resolve_hostname("test.local") {
            println!("解析 'test.local': {:?}", result.addresses);
        }

        if let Some(result) = manager.resolve_hostname("dev.example.com") {
            println!("解析 'dev.example.com': {:?}", result.addresses);
        }

        // 尝试解析不存在的域名
        if let Some(result) = manager.resolve_hostname("notfound.com") {
            println!("解析 'notfound.com': {:?}", result.addresses);
        } else {
            println!("'notfound.com' 未在 hosts 中找到");
        }
    }

    println!();
    Ok(())
}

/// 演示 hosts 文件下载
#[cfg(feature = "proxy")]
async fn demo_hosts_downloader() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- HostsDownloader 演示 ---");

    let downloader = HostsDownloader::new();

    // 注意：这里使用模拟的 hosts 内容，实际使用时需要真实的 URL
    println!("这是一个模拟演示。在实际使用中，你可以:");
    println!("1. 使用 downloader.download_hosts(url) 下载 hosts 文件");
    println!("2. 使用 downloader.download_and_save(url, path) 下载并保存");

    // 创建临时目录用于演示
    let temp_dir = TempDir::new()?;
    let hosts_path = temp_dir.path().join("hosts");

    // 创建一些示例 hosts 条目
    let entries = vec![
        HostEntry::new("0.0.0.0", &["ads.example.com"])?,
        HostEntry::new("0.0.0.0", &["tracker.example.com"])?,
        HostEntry::new("127.0.0.1", &["localhost"])?,
    ];

    // 保存到文件
    let manager = HostsManager::new(&hosts_path);
    manager.write_hosts(&entries)?;

    println!("创建了包含 {} 条记录的示例 hosts 文件", entries.len());
    println!("文件位置: {}", hosts_path.display());

    // 读取并验证
    let read_entries = manager.read_hosts()?;
    println!("成功读取 {} 条记录", read_entries.len());

    println!();
    Ok(())
}

/// 模拟网络下载错误处理
#[allow(dead_code)]
async fn demo_error_handling() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- 错误处理演示 ---");

    #[cfg(feature = "proxy")]
    {
        let downloader = HostsDownloader::new();

        // 演示无效 URL 处理
        match downloader.download_hosts("invalid-url").await {
            Ok(_) => println!("下载成功（不应该到达这里）"),
            Err(e) => println!("预期的错误: {}", e),
        }

        // 演示不支持的协议
        match downloader.download_hosts("ftp://example.com/hosts").await {
            Ok(_) => println!("下载成功（不应该到达这里）"),
            Err(e) => println!("预期的错误: {}", e),
        }
    }

    println!();
    Ok(())
}
