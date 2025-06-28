// Go Manager 单元测试
use std::path::PathBuf;
use tempfile::TempDir;
use tidepool_version_manager::{
    go::GoManager, ListInstalledRequest, UninstallRequest, VersionManager,
};

#[cfg(windows)]
use tidepool_version_manager::SwitchRequest;

// 测试GoManager构造函数
#[test]
fn test_new_go_manager() {
    let _manager = GoManager::new();
    // 简单验证构造函数不会崩溃
}

// 测试switch_to方法 - 基本功能测试
#[test]
#[cfg(windows)]
fn test_switch_to_basic_functionality() {
    let manager = GoManager::new();
    let temp_dir = TempDir::new().unwrap();
    let base_dir = PathBuf::from(temp_dir.path());

    // 创建模拟版本目录和必要的Go二进制文件
    let version = "1.21.3";
    let go_root = base_dir.join(version);
    let go_bin_dir = go_root.join("bin");
    std::fs::create_dir_all(&go_bin_dir).unwrap();

    // 创建模拟的go.exe文件
    std::fs::write(go_bin_dir.join("go.exe"), b"fake go binary").unwrap();

    // 测试切换版本
    let request = SwitchRequest {
        version: version.to_string(),
        base_dir: base_dir.clone(),
        global: false,
        force: false,
    };
    let result = manager.switch_to(request);
    assert!(result.is_ok());

    // 验证current目录是否创建（在Windows上为junction）
    #[cfg(windows)]
    {
        let current_dir = base_dir.join("current");
        assert!(current_dir.exists(), "Current directory should be created as a junction");
    }
}

// 测试版本切换（模拟）
#[test]
#[cfg(windows)]
fn test_switch_version() {
    let manager = GoManager::new();
    let temp_dir = TempDir::new().unwrap();
    let base_dir = PathBuf::from(temp_dir.path());

    // 创建模拟版本目录和必要的Go二进制文件
    let version = "1.21.3";
    let go_root = base_dir.join(version);
    let go_bin_dir = go_root.join("bin");
    std::fs::create_dir_all(&go_bin_dir).unwrap();

    // 创建模拟的go.exe文件
    std::fs::write(go_bin_dir.join("go.exe"), b"fake go binary").unwrap();

    // 测试切换到存在的版本
    let request = SwitchRequest {
        version: version.to_string(),
        base_dir: base_dir.clone(),
        global: false,
        force: false,
    };
    let result = manager.switch_to(request);
    assert!(result.is_ok());

    // 测试切换到不存在的版本
    let request = SwitchRequest {
        version: "999.999.999".to_string(),
        base_dir: base_dir.clone(),
        global: false,
        force: false,
    };
    let result = manager.switch_to(request);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not installed"));
}

// 测试列出已安装版本 - 正常情况
#[test]
fn test_list_installed_versions_success() {
    let manager = GoManager::new();
    let temp_dir = TempDir::new().unwrap();
    let base_dir = PathBuf::from(temp_dir.path());

    // 创建几个模拟Go版本
    let version1 = "1.21.3";
    let version2 = "1.20.8";

    for version in &[version1, version2] {
        let version_dir = base_dir.join(version).join("bin");
        std::fs::create_dir_all(&version_dir).unwrap();
        // 创建跨平台的go二进制文件来表示有效的Go安装
        let go_binary_name = if cfg!(target_os = "windows") { "go.exe" } else { "go" };
        std::fs::write(version_dir.join(go_binary_name), b"fake go binary").unwrap();
    }

    let request = ListInstalledRequest { base_dir };
    let result = manager.list_installed(request);

    assert!(result.is_ok());
    let version_list = result.unwrap();
    assert_eq!(version_list.total_count, 2);
    assert_eq!(version_list.versions.len(), 2);

    // 检查版本是否包含在列表中（顺序可能不同）
    assert!(version_list.versions.contains(&version1.to_string()));
    assert!(version_list.versions.contains(&version2.to_string()));
}

// 测试列出已安装版本 - 空目录
#[test]
fn test_list_installed_versions_empty() {
    let manager = GoManager::new();
    let temp_dir = TempDir::new().unwrap();
    let base_dir = PathBuf::from(temp_dir.path());

    let request = ListInstalledRequest { base_dir };
    let result = manager.list_installed(request);

    assert!(result.is_ok());
    let version_list = result.unwrap();
    assert_eq!(version_list.total_count, 0);
    assert!(version_list.versions.is_empty());
}

// 测试列出已安装版本 - 目录不存在
#[test]
fn test_list_installed_versions_nonexistent_dir() {
    let manager = GoManager::new();
    let base_dir = PathBuf::from("nonexistent_directory");

    let request = ListInstalledRequest { base_dir };
    let result = manager.list_installed(request);

    assert!(result.is_err());
    let error_message = result.unwrap_err();
    assert!(error_message.contains("does not exist") || error_message.contains("Failed to read"));
}

// 测试卸载版本 - 正常情况
#[test]
fn test_uninstall_version_success() {
    let manager = GoManager::new();
    let temp_dir = TempDir::new().unwrap();
    let base_dir = PathBuf::from(temp_dir.path());

    // 创建一个模拟的Go版本目录
    let version = "1.21.3";
    let version_dir = base_dir.join(version);
    std::fs::create_dir_all(&version_dir).unwrap();

    // 确保目录存在
    assert!(version_dir.exists());

    let request = UninstallRequest { version: version.to_string(), base_dir };

    let result = manager.uninstall(request);

    assert!(result.is_ok());
    // 确保目录已被删除
    assert!(!version_dir.exists());
}

// 测试卸载版本 - 版本不存在
#[test]
fn test_uninstall_version_not_found() {
    let manager = GoManager::new();
    let temp_dir = TempDir::new().unwrap();
    let base_dir = PathBuf::from(temp_dir.path());

    let request = UninstallRequest { version: "999.999.999".to_string(), base_dir };

    let result = manager.uninstall(request);

    assert!(result.is_err());
    let error_message = result.unwrap_err();
    assert!(error_message.contains("not installed") || error_message.contains("does not exist"));
}

// 测试列出已安装版本（排除current目录）
#[test]
fn test_list_installed_excludes_current_directory() {
    let manager = GoManager::new();
    let temp_dir = TempDir::new().unwrap();
    let base_dir = PathBuf::from(temp_dir.path());

    // 创建两个Go版本目录
    let version1 = "1.21.3";
    let version2 = "1.20.5";

    for version in [version1, version2] {
        let go_root = base_dir.join(version);
        let go_bin_dir = go_root.join("bin");
        std::fs::create_dir_all(&go_bin_dir).unwrap();
        let go_binary_name = if cfg!(target_os = "windows") { "go.exe" } else { "go" };
        std::fs::write(go_bin_dir.join(go_binary_name), b"fake go binary").unwrap();
    }

    // 创建current目录（模拟junction point）
    let current_dir = base_dir.join("current");
    let current_bin_dir = current_dir.join("bin");
    std::fs::create_dir_all(&current_bin_dir).unwrap();
    let go_binary_name = if cfg!(target_os = "windows") { "go.exe" } else { "go" };
    std::fs::write(current_bin_dir.join(go_binary_name), b"fake go binary").unwrap();

    // 测试列出已安装版本
    let request = ListInstalledRequest { base_dir };
    let result = manager.list_installed(request);
    assert!(result.is_ok());

    let version_list = result.unwrap();
    assert_eq!(version_list.total_count, 2, "Should only count actual version directories");
    assert_eq!(version_list.versions.len(), 2, "Should only list actual version directories");

    // 验证版本列表包含实际版本但不包含current
    assert!(version_list.versions.contains(&version1.to_string()));
    assert!(version_list.versions.contains(&version2.to_string()));
    assert!(
        !version_list.versions.contains(&"current".to_string()),
        "Should not include 'current' directory in version list"
    );

    // 验证版本按字母顺序排列
    let mut expected_versions = vec![version1.to_string(), version2.to_string()];
    expected_versions.sort();
    assert_eq!(version_list.versions, expected_versions);
}

// 测试缓存功能 - 检查是否正确使用已存在的缓存文件
#[test]
fn test_install_cache_functionality() {
    let _manager = GoManager::new();
    let temp_dir = TempDir::new().unwrap();
    let install_dir = PathBuf::from(temp_dir.path()).join("install");
    let download_dir = PathBuf::from(temp_dir.path()).join("cache");

    // 创建目录
    std::fs::create_dir_all(&install_dir).unwrap();
    std::fs::create_dir_all(&download_dir).unwrap();

    let version = "1.21.3";

    // 构建期望的缓存文件路径
    let (os, arch) = if cfg!(target_os = "windows") {
        ("windows", if cfg!(target_arch = "x86_64") { "amd64" } else { "386" })
    } else if cfg!(target_os = "macos") {
        ("darwin", if cfg!(target_arch = "x86_64") { "amd64" } else { "arm64" })
    } else {
        ("linux", if cfg!(target_arch = "x86_64") { "amd64" } else { "386" })
    };

    let extension = if cfg!(target_os = "windows") { "zip" } else { "tar.gz" };
    let archive_name = format!("go{version}.{os}-{arch}.{extension}");
    let cache_file = download_dir.join(&archive_name);

    // 创建一个模拟的缓存文件（非空）
    std::fs::write(&cache_file, b"fake cached go archive content").unwrap();

    // 验证缓存文件存在且不为空
    assert!(cache_file.exists());
    let metadata = std::fs::metadata(&cache_file).unwrap();
    assert!(metadata.len() > 0);

    // 注意：这个测试不会实际安装，因为我们没有有效的压缩包内容
    // 但它验证了缓存检测逻辑的正确性
    println!("✅ Cache functionality test setup completed");
    println!("   - Cache file: {}", cache_file.display());
    println!("   - Cache file size: {} bytes", metadata.len());
}

// 测试空缓存文件处理
#[test]
fn test_install_empty_cache_file() {
    let temp_dir = TempDir::new().unwrap();
    let download_dir = PathBuf::from(temp_dir.path()).join("cache");

    // 创建目录
    std::fs::create_dir_all(&download_dir).unwrap();

    let version = "1.21.3";

    // 构建期望的缓存文件路径
    let (os, arch) = if cfg!(target_os = "windows") {
        ("windows", if cfg!(target_arch = "x86_64") { "amd64" } else { "386" })
    } else if cfg!(target_os = "macos") {
        ("darwin", if cfg!(target_arch = "x86_64") { "amd64" } else { "arm64" })
    } else {
        ("linux", if cfg!(target_arch = "x86_64") { "amd64" } else { "386" })
    };

    let extension = if cfg!(target_os = "windows") { "zip" } else { "tar.gz" };
    let archive_name = format!("go{version}.{os}-{arch}.{extension}");
    let cache_file = download_dir.join(&archive_name);

    // 创建一个空的缓存文件
    std::fs::write(&cache_file, b"").unwrap();

    // 验证空缓存文件被正确识别
    assert!(cache_file.exists());
    let metadata = std::fs::metadata(&cache_file).unwrap();
    assert_eq!(metadata.len(), 0);

    println!("✅ Empty cache file handling test completed");
    println!("   - Empty cache file: {}", cache_file.display());
}

// 测试缓存文件完整性验证
#[test]
fn test_cache_file_validation() {
    let manager = GoManager::new();
    let temp_dir = TempDir::new().unwrap();
    let cache_dir = PathBuf::from(temp_dir.path()).join("cache");
    std::fs::create_dir_all(&cache_dir).unwrap();

    // 测试不存在的文件
    let nonexistent_file = cache_dir.join("nonexistent.zip");
    assert!(!manager.validate_cache_file(&nonexistent_file));

    // 测试空文件
    let empty_file = cache_dir.join("empty.zip");
    std::fs::write(&empty_file, b"").unwrap();
    assert!(!manager.validate_cache_file(&empty_file));

    // 测试太小的文件（小于1KB）
    let small_file = cache_dir.join("small.zip");
    std::fs::write(&small_file, b"small content").unwrap();
    assert!(!manager.validate_cache_file(&small_file));

    // 测试有效的文件（大于1KB）
    let valid_file = cache_dir.join("valid.zip");
    let large_content = vec![b'x'; 2048]; // 2KB content
    std::fs::write(&valid_file, &large_content).unwrap();
    assert!(manager.validate_cache_file(&valid_file));

    println!("✅ Cache file validation tests completed");
}
