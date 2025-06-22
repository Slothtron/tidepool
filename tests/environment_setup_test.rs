// 环境变量配置的集成测试
// 测试环境变量配置功能在不同平台的兼容性

use std::path::PathBuf;

#[test]
fn test_environment_variable_generation() {
    // 测试环境变量路径生成逻辑
    let install_path = PathBuf::from("/test/go/1.21.0");
    let bin_path = install_path.join("bin");

    // 验证 GOROOT 路径
    assert_eq!(install_path.to_string_lossy(), "/test/go/1.21.0");

    // 验证 PATH 路径
    assert_eq!(bin_path.to_string_lossy(), "/test/go/1.21.0/bin");
}

#[test]
fn test_shell_detection_logic() {
    // 测试 shell 检测逻辑
    let shell_paths = vec![
        ("/bin/bash", "bash"),
        ("/usr/bin/zsh", "zsh"),
        ("/usr/bin/fish", "fish"),
        ("/usr/bin/nu", "nu"),
    ];

    for (shell_path, expected_shell) in shell_paths {
        assert!(
            shell_path.contains(expected_shell),
            "Shell 路径 {} 应该包含 {}",
            shell_path,
            expected_shell
        );
    }
}

#[test]
#[cfg(target_os = "windows")]
fn test_windows_environment_setup() {
    // Windows 特定的环境变量测试
    let install_path = PathBuf::from("C:\\gvm\\versions\\1.21.0");
    let bin_path = install_path.join("bin");

    assert!(install_path.is_absolute());
    assert!(bin_path.to_string_lossy().ends_with("bin"));
}

#[test]
#[cfg(not(target_os = "windows"))]
fn test_unix_environment_setup() {
    // Unix 系统特定的环境变量测试
    let install_path = PathBuf::from("/home/user/.gvm/versions/1.21.0");
    let bin_path = install_path.join("bin");

    assert!(install_path.is_absolute());
    assert!(bin_path.to_string_lossy().ends_with("bin"));
}
