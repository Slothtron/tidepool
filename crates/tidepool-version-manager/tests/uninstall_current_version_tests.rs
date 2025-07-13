/// 测试卸载当前活跃版本时的保护机制
///
/// 验证当用户尝试卸载当前正在使用的 Go 版本时，
/// 系统会阻止操作并提供友好的错误信息。
use std::fs;
use tempfile::TempDir;
use tidepool_version_manager::symlink::symlink_dir;
use tidepool_version_manager::{go::GoManager, UninstallRequest, VersionManager};

/// 测试卸载当前活跃版本时的错误处理
#[test]
fn test_cannot_uninstall_current_version() {
    let temp_dir = TempDir::new().expect("无法创建临时目录");
    let base_dir = temp_dir.path().to_path_buf();
    let manager = GoManager::new();

    // 设置测试版本
    let version = "1.21.0";
    let version_dir = base_dir.join(version);
    let bin_dir = version_dir.join("bin");

    // 创建模拟的 Go 安装目录
    fs::create_dir_all(&bin_dir).expect("无法创建 bin 目录");

    #[cfg(target_os = "windows")]
    let go_binary = bin_dir.join("go.exe");
    #[cfg(not(target_os = "windows"))]
    let go_binary = bin_dir.join("go");

    fs::write(&go_binary, "fake go binary").expect("无法创建 go 二进制文件"); // 创建符号链接指向当前版本
    let current_link = base_dir.join("current"); // 使用统一的 symlink 模块创建符号链接
    if symlink_dir(&version_dir, &current_link).is_err() {
        // 如果符号链接创建失败（可能权限不足），跳过这个测试
        println!("跳过测试：无法创建符号链接，可能权限不足");
        return;
    }

    // 验证当前版本检测正常工作
    let current_version = manager.get_current_version(&base_dir);
    assert_eq!(current_version, Some(version.to_string()), "应该检测到当前版本");

    // 尝试卸载当前版本，应该失败
    let uninstall_request =
        UninstallRequest { version: version.to_string(), base_dir: base_dir.clone() };

    let result = manager.uninstall(uninstall_request);
    assert!(result.is_err(), "卸载当前版本应该失败");

    let error_msg = result.unwrap_err();
    assert!(
        error_msg.contains("currently active"),
        "错误信息应该指出版本正在被使用，实际错误: {error_msg}"
    );

    // 验证版本目录仍然存在（没有被删除）
    assert!(version_dir.exists(), "版本目录应该仍然存在");
    assert!(go_binary.exists(), "Go 二进制文件应该仍然存在");
}

/// 测试卸载非当前版本时的正常行为
#[test]
fn test_can_uninstall_non_current_version() {
    let temp_dir = TempDir::new().expect("无法创建临时目录");
    let base_dir = temp_dir.path().to_path_buf();
    let manager = GoManager::new();

    // 设置两个测试版本
    let current_version = "1.21.0";
    let other_version = "1.20.5";

    let current_version_dir = base_dir.join(current_version);
    let other_version_dir = base_dir.join(other_version);

    let current_bin_dir = current_version_dir.join("bin");
    let other_bin_dir = other_version_dir.join("bin");

    // 创建两个模拟的 Go 安装目录
    fs::create_dir_all(&current_bin_dir).expect("无法创建当前版本 bin 目录");
    fs::create_dir_all(&other_bin_dir).expect("无法创建其他版本 bin 目录");

    #[cfg(target_os = "windows")]
    {
        let current_go_binary = current_bin_dir.join("go.exe");
        let other_go_binary = other_bin_dir.join("go.exe");
        fs::write(&current_go_binary, "fake go binary").expect("无法创建当前版本 go 二进制文件");
        fs::write(&other_go_binary, "fake go binary").expect("无法创建其他版本 go 二进制文件");
    }

    #[cfg(not(target_os = "windows"))]
    {
        let current_go_binary = current_bin_dir.join("go");
        let other_go_binary = other_bin_dir.join("go");
        fs::write(&current_go_binary, "fake go binary").expect("无法创建当前版本 go 二进制文件");
        fs::write(&other_go_binary, "fake go binary").expect("无法创建其他版本 go 二进制文件");
    } // 创建符号链接指向当前版本
    let current_link = base_dir.join("current");

    // 使用统一的 symlink 模块创建符号链接
    if symlink_dir(&current_version_dir, &current_link).is_err() {
        println!("跳过测试：无法创建符号链接，可能权限不足");
        return;
    }

    // 验证当前版本检测正常工作
    let detected_current = manager.get_current_version(&base_dir);
    assert_eq!(detected_current, Some(current_version.to_string()));

    // 尝试卸载非当前版本，应该成功
    let uninstall_request =
        UninstallRequest { version: other_version.to_string(), base_dir: base_dir.clone() };

    let result = manager.uninstall(uninstall_request);
    assert!(result.is_ok(), "卸载非当前版本应该成功，错误: {result:?}");

    // 验证其他版本目录被删除，当前版本目录仍然存在
    assert!(!other_version_dir.exists(), "其他版本目录应该被删除");
    assert!(current_version_dir.exists(), "当前版本目录应该仍然存在");
    assert!(current_link.exists(), "当前链接应该仍然存在");
}

/// 测试没有当前版本时的卸载行为
#[test]
fn test_uninstall_when_no_current_version() {
    let temp_dir = TempDir::new().expect("无法创建临时目录");
    let base_dir = temp_dir.path().to_path_buf();
    let manager = GoManager::new();

    // 设置测试版本
    let version = "1.21.0";
    let version_dir = base_dir.join(version);
    let bin_dir = version_dir.join("bin");

    // 创建模拟的 Go 安装目录
    fs::create_dir_all(&bin_dir).expect("无法创建 bin 目录");

    #[cfg(target_os = "windows")]
    let go_binary = bin_dir.join("go.exe");
    #[cfg(not(target_os = "windows"))]
    let go_binary = bin_dir.join("go");

    fs::write(&go_binary, "fake go binary").expect("无法创建 go 二进制文件");

    // 验证没有当前版本
    let current_version = manager.get_current_version(&base_dir);
    assert_eq!(current_version, None, "应该没有检测到当前版本");

    // 尝试卸载版本，应该成功（因为没有当前版本）
    let uninstall_request =
        UninstallRequest { version: version.to_string(), base_dir: base_dir.clone() };

    let result = manager.uninstall(uninstall_request);
    assert!(result.is_ok(), "当没有当前版本时，卸载应该成功，错误: {result:?}");

    // 验证版本目录被删除
    assert!(!version_dir.exists(), "版本目录应该被删除");
}

/// 测试卸载不存在的版本
#[test]
fn test_uninstall_non_existent_version() {
    let temp_dir = TempDir::new().expect("无法创建临时目录");
    let base_dir = temp_dir.path().to_path_buf();
    let manager = GoManager::new();

    let version = "1.99.99"; // 不存在的版本

    // 尝试卸载不存在的版本，应该失败
    let uninstall_request =
        UninstallRequest { version: version.to_string(), base_dir: base_dir.clone() };

    let result = manager.uninstall(uninstall_request);
    assert!(result.is_err(), "卸载不存在的版本应该失败");

    let error_msg = result.unwrap_err();
    assert!(
        error_msg.contains("not installed"),
        "错误信息应该指出版本未安装，实际错误: {error_msg}"
    );
}
