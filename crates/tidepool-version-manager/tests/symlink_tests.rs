//! 测试Go版本切换（跨平台符号链接方法）

#[cfg(test)]
mod tests {
    use tidepool_version_manager::go::GoManager;
    use tidepool_version_manager::symlink::{get_symlink_target, is_symlink};

    #[cfg(target_os = "windows")]
    fn setup_test_go_version(temp_dir: &std::path::Path, version: &str) {
        let version_dir = temp_dir.join(version);
        let bin_dir = version_dir.join("bin");
        std::fs::create_dir_all(&bin_dir).unwrap();

        // 创建模拟的go.exe文件
        let go_exe = bin_dir.join("go.exe");
        std::fs::write(&go_exe, format!("fake go executable {version}")).unwrap();
    }
    #[test]
    #[cfg(target_os = "windows")]
    fn test_symlink_creation_method() {
        let temp_dir = tempfile::tempdir().unwrap();
        let manager = GoManager::new();
        let version = "1.21.0";

        setup_test_go_version(temp_dir.path(), version);

        let result = manager.switch_version(version, temp_dir.path());

        // 测试应该成功（在真实环境中）或者失败并返回有意义的错误消息
        match result {
            Ok(()) => {
                // 验证符号链接是否创建
                let symlink_path = temp_dir.path().join("current");
                assert!(symlink_path.exists());

                // 使用 symlink 模块验证符号链接
                assert!(is_symlink(&symlink_path));

                // 验证符号链接目标
                if let Some(target) = get_symlink_target(&symlink_path) {
                    assert_eq!(target, temp_dir.path().join(version));
                }
            }
            Err(e) => {
                // 在测试环境中，符号链接创建可能失败，这是可以接受的
                assert!(
                    e.contains("Failed to create symlink")
                        || e.contains("Access is denied")
                        || e.contains("permission")
                );
            }
        }
    }

    #[test]
    fn test_manager_creation() {
        let manager = GoManager::new();
        // 测试默认创建成功
        assert_eq!(std::mem::size_of_val(&manager), 0); // GoManager是空结构体
    }
    #[test]
    #[cfg(target_os = "windows")]
    fn test_get_current_version() {
        let temp_dir = tempfile::tempdir().unwrap();
        let manager = GoManager::new();

        // 测试当没有设置版本时返回None
        let current = manager.get_current_version(temp_dir.path());
        assert!(current.is_none());
    }
    #[test]
    #[cfg(target_os = "windows")]
    fn test_get_symlink_info() {
        let temp_dir = tempfile::tempdir().unwrap();
        let manager = GoManager::new();

        let info = manager.get_symlink_info(temp_dir.path());
        assert_eq!(info, "No symlink found");
    }
}
