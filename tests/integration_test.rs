// 集成测试：测试版本管理器和 CLI 的协作
// 这里测试的是整个系统的端到端功能

use std::path::PathBuf;
use tempfile::TempDir;
use tidepool::{GoManager, ListInstalledRequest, VersionManager};

#[test]
fn test_version_manager_integration() {
    // 创建临时目录进行测试
    let temp_dir = TempDir::new().expect("创建临时目录失败");
    let base_dir = temp_dir.path().to_path_buf();

    let manager = GoManager::new();
    let request = ListInstalledRequest { base_dir };

    // 测试基本功能是否正常工作
    let result = manager.list_installed(request);
    assert!(result.is_ok(), "版本管理器基本功能应该正常工作");

    let version_list = result.unwrap();
    assert_eq!(version_list.versions.len(), 0, "新目录应该没有已安装的版本");
}

#[test]
fn test_cross_platform_paths() {
    // 测试跨平台路径处理
    let test_paths =
        vec!["/home/user/.gvm/versions/1.21.0", "C:\\gvm\\versions\\1.21.0", "/opt/go/1.19.3"];

    for path_str in test_paths {
        let path = PathBuf::from(path_str);
        let bin_path = path.join("bin");

        // 验证路径操作是否正确
        assert!(bin_path.to_string_lossy().ends_with("bin"));
        assert!(path.to_string_lossy().contains("go") || path.to_string_lossy().contains("gvm"));
    }
}

#[cfg(test)]
mod integration_helpers {
    use super::*;

    #[allow(dead_code)]
    pub fn setup_test_environment() -> TempDir {
        TempDir::new().expect("创建测试环境失败")
    }

    #[allow(dead_code)]
    pub fn cleanup_test_environment(_temp_dir: TempDir) {
        // TempDir 在 drop 时会自动清理
    }
}
