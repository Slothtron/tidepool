//! info 命令测试
//!
//! 测试 gvm info 命令的功能，包括：
//! - 版本信息获取
//! - 平台信息显示
//! - 文件名和下载URL
//! - SHA256校验和获取
//! - 安装和缓存状态

use std::path::PathBuf;
use tidepool_version_manager::go::GoManager;

#[tokio::test]
async fn test_get_version_info_basic() {
    let manager = GoManager::new();
    let install_dir = PathBuf::from("/tmp/test_go_versions");
    let cache_dir = PathBuf::from("/tmp/test_go_cache");

    // 测试获取版本信息
    let result = manager.get_version_info("1.21.5", &install_dir, &cache_dir).await;

    assert!(result.is_ok(), "应该能够获取版本信息");

    let info = result.unwrap();
    assert_eq!(info.version, "1.21.5");
    assert_eq!(
        info.os,
        if cfg!(target_os = "windows") {
            "windows"
        } else if cfg!(target_os = "macos") {
            "darwin"
        } else {
            "linux"
        }
    );
    assert_eq!(info.arch, if cfg!(target_arch = "x86_64") { "amd64" } else { "386" });
    assert!(!info.filename.is_empty());
    assert!(info.download_url.starts_with("https://go.dev/dl/"));
    assert!(!info.is_installed); // 因为使用测试目录，应该没有安装
    assert!(!info.is_cached); // 因为使用测试目录，应该没有缓存
}

#[tokio::test]
async fn test_get_version_info_filename_format() {
    let manager = GoManager::new();
    let install_dir = PathBuf::from("/tmp/test_go_versions");
    let cache_dir = PathBuf::from("/tmp/test_go_cache");

    let result = manager.get_version_info("1.21.5", &install_dir, &cache_dir).await;

    assert!(result.is_ok());

    let info = result.unwrap();

    // 验证文件名格式
    let expected_prefix = "go1.21.5.";
    assert!(
        info.filename.starts_with(expected_prefix),
        "文件名应以 {} 开头，实际: {}",
        expected_prefix,
        info.filename
    );

    // 验证扩展名
    if cfg!(target_os = "windows") {
        assert!(
            std::path::Path::new(&info.filename)
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("zip")),
            "Windows 应使用 .zip 扩展名"
        );
    } else {
        assert!(info.filename.ends_with(".tar.gz"), "Unix 应使用 .tar.gz 扩展名");
    }
}

#[tokio::test]
async fn test_get_version_info_invalid_version() {
    let manager = GoManager::new();
    let install_dir = PathBuf::from("/tmp/test_go_versions");
    let cache_dir = PathBuf::from("/tmp/test_go_cache");

    // 测试无效版本 - 不应该失败，但SHA256可能为None
    let result = manager.get_version_info("1.99.99", &install_dir, &cache_dir).await;

    assert!(result.is_ok(), "即使版本不存在也应该返回基本信息");

    let info = result.unwrap();
    assert_eq!(info.version, "1.99.99");
    // SHA256 可能为 None（无效版本）
    // 这是预期的行为
}

#[cfg(not(target_os = "windows"))]
#[tokio::test]
#[ignore = "需要网络连接，在CI或无网络环境中跳过"]
async fn test_get_version_info_integration() {
    // 集成测试：验证实际能从go.dev获取到真实的版本信息
    // 注意：此测试需要网络连接，可以通过 `cargo test -- --ignored` 单独运行
    let manager = GoManager::new();
    let install_dir = PathBuf::from("/tmp/test_go_versions");
    let cache_dir = PathBuf::from("/tmp/test_go_cache");

    // 使用一个已知存在的Go版本
    let result = manager.get_version_info("1.21.5", &install_dir, &cache_dir).await;

    assert!(result.is_ok());

    let info = result.unwrap();

    // 验证能获取到SHA256
    assert!(info.sha256.is_some(), "应该能够获取到真实版本的SHA256");

    if let Some(sha256) = &info.sha256 {
        assert_eq!(sha256.len(), 64, "SHA256应该是64位十六进制字符串");
        assert!(sha256.chars().all(|c| c.is_ascii_hexdigit()), "SHA256应该只包含十六进制字符");
    }
}
