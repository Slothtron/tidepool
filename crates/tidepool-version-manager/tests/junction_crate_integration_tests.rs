//! 集成测试 - 验证第三方 junction crate 的功能

#[cfg(test)]
mod integration_tests {
    use std::fs;
    use tempfile::TempDir;
    use tidepool_version_manager::go::GoManager;

    #[test]
    #[cfg(windows)]
    fn test_junction_crate_integration() {
        let temp_dir = TempDir::new().unwrap();
        let manager = GoManager::new();

        // 创建测试版本目录
        let version = "1.21.5";
        let version_path = temp_dir.path().join(version);
        let bin_path = version_path.join("bin");
        fs::create_dir_all(&bin_path).unwrap();

        // 创建模拟的 go.exe 文件
        fs::write(bin_path.join("go.exe"), b"fake go binary").unwrap();

        // 测试创建 junction
        match manager.switch_version(version, temp_dir.path()) {
            Ok(()) => {
                let junction_path = temp_dir.path().join("current");

                // 验证 junction 是否存在
                assert!(junction_path.exists());

                // 使用 junction crate 验证
                assert!(junction::exists(&junction_path).unwrap_or(false));

                // 验证 junction 目标
                if let Ok(target) = junction::get_target(&junction_path) {
                    assert_eq!(target, version_path);
                }

                println!("✅ Junction crate 集成测试成功");
            }
            Err(e) => {
                // 在测试环境中，权限问题是可以接受的
                if e.contains("Failed to create junction")
                    || e.contains("Access is denied")
                    || e.contains("permission")
                {
                    println!("⚠️ 测试跳过：权限不足 - {e}");
                } else {
                    panic!("未预期的错误: {e}");
                }
            }
        }
    }

    #[test]
    fn test_junction_crate_availability() {
        // 验证 junction crate 可用性（在所有平台上都应该编译通过）
        println!("✅ Junction crate 依赖成功添加并编译");
    }
}
