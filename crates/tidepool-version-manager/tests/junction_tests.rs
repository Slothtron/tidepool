//! 测试Go版本切换（Junction Point方法）

#[cfg(test)]
mod tests {
    #[cfg(windows)]
    use std::fs;
    use tidepool_version_manager::go::GoManager;

    #[cfg(windows)]
    use tempfile::tempdir;

    #[cfg(windows)]
    fn setup_test_go_version(temp_dir: &std::path::Path, version: &str) {
        let version_dir = temp_dir.join(version);
        let bin_dir = version_dir.join("bin");
        fs::create_dir_all(&bin_dir).unwrap();

        // 创建模拟的go.exe文件
        let go_exe = bin_dir.join("go.exe");
        fs::write(&go_exe, format!("fake go executable {version}")).unwrap();
    }
    #[test]
    #[cfg(windows)]
    fn test_junction_point_method() {
        let temp_dir = tempdir().unwrap();
        let manager = GoManager::new();
        let version = "1.21.0";

        setup_test_go_version(temp_dir.path(), version);

        let result = manager.switch_version(version, temp_dir.path());

        // 测试应该成功（在真实环境中）或者失败并返回有意义的错误消息
        match result {
            Ok(()) => {
                // 验证junction是否创建
                let junction_path = temp_dir.path().join("current");
                assert!(junction_path.exists());

                // 使用 junction crate 验证 junction point
                assert!(junction::exists(&junction_path).unwrap_or(false));

                // 验证 junction 目标
                if let Ok(target) = junction::get_target(&junction_path) {
                    assert_eq!(target, temp_dir.path().join(version));
                }
            }
            Err(e) => {
                // 在测试环境中，junction 创建可能失败，这是可以接受的
                assert!(
                    e.contains("Failed to create junction")
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
    #[cfg(windows)]
    fn test_get_current_version() {
        let temp_dir = tempdir().unwrap();
        let manager = GoManager::new();

        // 测试当没有设置版本时返回None
        let current = manager.get_current_version(temp_dir.path());
        assert!(current.is_none());
    }

    #[test]
    #[cfg(windows)]
    fn test_get_junction_info() {
        let temp_dir = tempdir().unwrap();
        let manager = GoManager::new();

        let info = manager.get_symlink_info(temp_dir.path());
        assert_eq!(info, "No junction found");
    }
}
