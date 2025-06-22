/// 测试 Go 版本安装的 SHA256 校验功能
///
/// 验证下载文件的完整性校验机制，确保只有官方认证的文件才能被安装。
use std::fs;
use tempfile::TempDir;
use tidepool_version_manager::go::GoManager;

/// 测试模拟的校验功能（由于网络依赖，使用模拟数据）
#[tokio::test]
async fn test_hash_verification_functionality() {
    let temp_dir = TempDir::new().expect("无法创建临时目录");
    let base_dir = temp_dir.path().to_path_buf();
    let manager = GoManager::new();

    // 创建模拟的文件用于测试哈希计算
    let test_file = base_dir.join("test_file.txt");
    let test_content = "Hello, World!";
    fs::write(&test_file, test_content).expect("无法创建测试文件");

    // 测试哈希计算功能
    let hash_result = manager.calculate_file_hash(&test_file).await;
    assert!(hash_result.is_ok(), "哈希计算应该成功");

    let hash_value = hash_result.unwrap();
    // "Hello, World!" 的 SHA256 哈希值
    let expected_hash = "dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f";
    assert_eq!(hash_value, expected_hash, "哈希值应该匹配");
}

/// 测试哈希计算函数
#[tokio::test]
async fn test_calculate_file_hash() {
    let temp_dir = TempDir::new().expect("无法创建临时目录");
    let base_dir = temp_dir.path().to_path_buf();
    let manager = GoManager::new();

    // 测试不同大小的文件
    let large_content = "X".repeat(10000); // 10KB 文件
    let test_cases = vec![
        ("small.txt", "Hello"),
        ("medium.txt", "This is a medium-sized test file for SHA256 calculation"),
        ("large.txt", &large_content),
    ];

    for (filename, content) in test_cases {
        let test_file = base_dir.join(filename);
        fs::write(&test_file, content).expect("无法创建测试文件");

        let result = manager.calculate_file_hash(&test_file).await;
        assert!(result.is_ok(), "计算 {} 的哈希值应该成功", filename);

        let hash = result.unwrap();
        assert_eq!(hash.len(), 64, "SHA256 哈希值应该是 64 个字符");
        assert!(hash.chars().all(|c| c.is_ascii_hexdigit()), "哈希值应该只包含十六进制字符");
    }
}

/// 测试空文件的哈希计算
#[tokio::test]
async fn test_empty_file_hash() {
    let temp_dir = TempDir::new().expect("无法创建临时目录");
    let base_dir = temp_dir.path().to_path_buf();
    let manager = GoManager::new();

    // 创建空文件
    let empty_file = base_dir.join("empty.txt");
    fs::write(&empty_file, "").expect("无法创建空文件");

    let result = manager.calculate_file_hash(&empty_file).await;
    assert!(result.is_ok(), "计算空文件的哈希值应该成功");

    let hash = result.unwrap();
    // 空文件的 SHA256 哈希值
    let expected_empty_hash = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
    assert_eq!(hash, expected_empty_hash, "空文件的哈希值应该匹配");
}

/// 测试不存在文件的哈希计算
#[tokio::test]
async fn test_nonexistent_file_hash() {
    let temp_dir = TempDir::new().expect("无法创建临时目录");
    let base_dir = temp_dir.path().to_path_buf();
    let manager = GoManager::new();

    let nonexistent_file = base_dir.join("nonexistent.txt");

    let result = manager.calculate_file_hash(&nonexistent_file).await;
    assert!(result.is_err(), "计算不存在文件的哈希值应该失败");

    let error = result.unwrap_err();
    assert!(error.contains("Failed to open file"), "错误信息应该指出无法打开文件");
}

/// 测试大文件的哈希计算性能
#[tokio::test]
async fn test_large_file_hash_performance() {
    let temp_dir = TempDir::new().expect("无法创建临时目录");
    let base_dir = temp_dir.path().to_path_buf();
    let manager = GoManager::new();

    // 创建 1MB 的测试文件
    let large_file = base_dir.join("large.bin");
    let large_content = vec![0u8; 1024 * 1024]; // 1MB
    fs::write(&large_file, &large_content).expect("无法创建大文件");

    let start = std::time::Instant::now();
    let result = manager.calculate_file_hash(&large_file).await;
    let duration = start.elapsed();

    assert!(result.is_ok(), "计算大文件的哈希值应该成功");
    println!("计算 1MB 文件哈希耗时: {:?}", duration);

    // 验证哈希值格式正确
    let hash = result.unwrap();
    assert_eq!(hash.len(), 64, "SHA256 哈希值应该是 64 个字符");
    assert!(hash.chars().all(|c| c.is_ascii_hexdigit()), "哈希值应该只包含十六进制字符");
}

/// 测试相同内容文件的哈希一致性
#[tokio::test]
async fn test_hash_consistency() {
    let temp_dir = TempDir::new().expect("无法创建临时目录");
    let base_dir = temp_dir.path().to_path_buf();
    let manager = GoManager::new();

    let content = "This is a test for hash consistency";

    // 创建两个相同内容的文件
    let file1 = base_dir.join("file1.txt");
    let file2 = base_dir.join("file2.txt");
    fs::write(&file1, content).expect("无法创建文件1");
    fs::write(&file2, content).expect("无法创建文件2");

    // 计算两个文件的哈希值
    let hash1 = manager.calculate_file_hash(&file1).await.expect("计算文件1哈希失败");
    let hash2 = manager.calculate_file_hash(&file2).await.expect("计算文件2哈希失败");

    assert_eq!(hash1, hash2, "相同内容的文件应该有相同的哈希值");

    // 修改其中一个文件
    fs::write(&file2, format!("{} modified", content)).expect("无法修改文件2");
    let hash2_modified =
        manager.calculate_file_hash(&file2).await.expect("计算修改后文件2哈希失败");

    assert_ne!(hash1, hash2_modified, "不同内容的文件应该有不同的哈希值");
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    /// 集成测试：模拟完整的校验流程
    #[tokio::test]
    async fn test_file_verification_workflow() {
        let temp_dir = TempDir::new().expect("无法创建临时目录");
        let base_dir = temp_dir.path().to_path_buf();
        let manager = GoManager::new();

        // 创建模拟的 Go 安装包文件
        let test_file = base_dir.join("go1.21.0.linux-amd64.tar.gz");
        let test_content = "This is a mock Go installation package";
        fs::write(&test_file, test_content).expect("无法创建测试文件");

        // 计算文件哈希
        let file_hash = manager.calculate_file_hash(&test_file).await;
        assert!(file_hash.is_ok(), "文件哈希计算应该成功");

        println!("模拟 Go 安装包哈希值: {}", file_hash.unwrap());

        // 注意：实际的官方校验和获取需要网络连接，在单元测试中我们只测试哈希计算
        // 完整的集成测试可以在具有网络连接的环境中进行
    }
}
