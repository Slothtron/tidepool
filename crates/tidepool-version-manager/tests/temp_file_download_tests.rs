/// 测试临时文件下载功能
/// 验证下载器使用临时文件机制，下载完成后重命名到目标文件
use std::path::PathBuf;
use tempfile::TempDir;
use tidepool_version_manager::downloader::{DownloadConfig, Downloader};

#[tokio::test]
async fn test_temp_file_download_mechanism() {
    println!("🧪 测试临时文件下载机制");

    // 创建临时目录用于测试
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path();

    // 配置下载器（使用较小的分片大小以触发单线程下载）
    let config = DownloadConfig {
        concurrent_connections: 1,
        enable_chunked_download: false, // 强制单线程下载以便测试
        timeout_seconds: 30,
        max_retries: 1,
        ..Default::default()
    };

    let downloader = Downloader::with_config(config);

    // 测试文件路径
    let target_file = base_path.join("test_download.txt");
    let expected_temp_file = base_path.join("test_download.txt.tmp");

    println!("📁 目标文件: {}", target_file.display());
    println!("📁 预期临时文件: {}", expected_temp_file.display());

    // 使用一个小的测试文件 URL（httpbin.org 提供测试服务）
    let test_url = "https://httpbin.org/bytes/1024"; // 下载 1KB 测试数据

    println!("🌐 测试 URL: {}", test_url);

    // 确认文件开始时不存在
    assert!(!target_file.exists(), "目标文件不应该预先存在");
    assert!(!expected_temp_file.exists(), "临时文件不应该预先存在");

    // 执行下载
    println!("⬇️  开始下载...");
    let download_result = downloader.download(test_url, &target_file, None).await;

    match download_result {
        Ok(()) => {
            println!("✅ 下载成功完成");

            // 验证最终状态
            assert!(target_file.exists(), "下载完成后目标文件应该存在");
            assert!(!expected_temp_file.exists(), "下载完成后临时文件应该被删除");

            // 验证文件大小
            let file_size = std::fs::metadata(&target_file).unwrap().len();
            assert_eq!(file_size, 1024, "文件大小应该为 1024 字节");

            println!("✅ 文件大小验证通过: {} 字节", file_size);
            println!("✅ 所有验证通过，临时文件机制工作正常");
        }
        Err(e) => {
            println!("❌ 下载失败: {}", e);

            // 即使下载失败，也要验证清理逻辑
            assert!(!target_file.exists(), "下载失败时目标文件不应该存在");
            assert!(!expected_temp_file.exists(), "下载失败时临时文件应该被清理");

            println!("✅ 失败清理验证通过，临时文件已正确清理");

            // 对于网络问题，我们不将其视为测试失败
            if e.to_string().contains("网络错误") {
                println!("ℹ️  网络错误，跳过此测试");
                return;
            }

            panic!("下载测试失败: {}", e);
        }
    }
}

#[tokio::test]
async fn test_chunked_temp_file_download() {
    println!("🧪 测试分片临时文件下载机制");

    // 创建临时目录用于测试
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path();

    // 配置下载器（启用分片下载）
    let config = DownloadConfig {
        concurrent_connections: 2,
        enable_chunked_download: true,
        min_chunk_size: 512, // 较小的分片大小
        timeout_seconds: 30,
        max_retries: 1,
        ..Default::default()
    };

    let downloader = Downloader::with_config(config);

    // 测试文件路径
    let target_file = base_path.join("chunked_test.bin");
    let expected_temp_file = base_path.join("chunked_test.bin.tmp");

    println!("📁 目标文件: {}", target_file.display());
    println!("📁 预期临时文件: {}", expected_temp_file.display());

    // 使用较大的测试文件以触发分片下载
    let test_url = "https://httpbin.org/bytes/4096"; // 下载 4KB 测试数据

    println!("🌐 测试 URL: {}", test_url);

    // 确认文件开始时不存在
    assert!(!target_file.exists(), "目标文件不应该预先存在");
    assert!(!expected_temp_file.exists(), "临时文件不应该预先存在");

    // 执行下载
    println!("⬇️  开始分片下载...");
    let download_result = downloader.download(test_url, &target_file, None).await;

    match download_result {
        Ok(()) => {
            println!("✅ 分片下载成功完成");

            // 验证最终状态
            assert!(target_file.exists(), "下载完成后目标文件应该存在");
            assert!(!expected_temp_file.exists(), "下载完成后临时文件应该被删除");

            // 验证文件大小
            let file_size = std::fs::metadata(&target_file).unwrap().len();
            assert_eq!(file_size, 4096, "文件大小应该为 4096 字节");

            println!("✅ 文件大小验证通过: {} 字节", file_size);
            println!("✅ 所有验证通过，分片临时文件机制工作正常");
        }
        Err(e) => {
            println!("❌ 分片下载失败: {}", e);

            // 验证清理逻辑
            assert!(!target_file.exists(), "下载失败时目标文件不应该存在");
            assert!(!expected_temp_file.exists(), "下载失败时临时文件应该被清理");

            println!("✅ 失败清理验证通过，临时文件已正确清理");

            // 对于网络问题，我们不将其视为测试失败
            if e.to_string().contains("网络错误") || e.to_string().contains("服务器不支持范围请求")
            {
                println!("ℹ️  网络或服务器限制，跳过此测试");
                return;
            }

            panic!("分片下载测试失败: {}", e);
        }
    }
}

#[test]
fn test_temp_file_path_generation() {
    println!("🧪 测试临时文件路径生成逻辑");

    // 测试各种文件扩展名的临时文件路径生成
    let test_cases = vec![
        ("test.txt", "test.txt.tmp"),
        ("archive.tar.gz", "archive.tar.gz.tmp"),
        ("package.zip", "package.zip.tmp"),
        ("no_extension", "no_extension.tmp"),
        ("multiple.dots.file.ext", "multiple.dots.file.ext.tmp"),
    ];

    for (input, expected) in test_cases {
        let input_path = PathBuf::from(input);
        let temp_path = input_path.with_extension(match input_path.extension() {
            Some(ext) => format!("{}.tmp", ext.to_string_lossy()),
            None => "tmp".to_string(),
        });

        let temp_name = temp_path.file_name().unwrap().to_string_lossy();
        assert_eq!(temp_name, expected, "临时文件名生成不正确");

        println!("✅ {} -> {}", input, temp_name);
    }

    println!("✅ 临时文件路径生成逻辑验证通过");
}
