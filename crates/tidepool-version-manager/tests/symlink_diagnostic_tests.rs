//! 诊断符号链接问题的简单测试

#[cfg(test)]
mod symlink_diagnostic_tests {
    use std::fs;
    use tempfile::TempDir;
    use tidepool_version_manager::symlink::{
        get_symlink_target, is_symlink, remove_symlink_dir, symlink_dir,
    };
    #[test]
    #[cfg(target_os = "windows")]
    fn test_symlink_basic_operations() {
        let temp_dir = TempDir::new().unwrap();

        // 创建源目录
        let source = temp_dir.path().join("source");
        fs::create_dir_all(&source).unwrap();
        fs::write(source.join("test.txt"), b"test content").unwrap();

        // 创建符号链接
        let symlink_path = temp_dir.path().join("link");

        println!("🔍 测试基本符号链接操作");
        println!("源目录: {}", source.display());
        println!("符号链接路径: {}", symlink_path.display());

        // 第一次创建
        match symlink_dir(&source, &symlink_path) {
            Ok(()) => {
                println!("✅ 第一次创建符号链接成功");

                // 验证 symlink 存在
                println!("Symlink 存在检查: {}", is_symlink(&symlink_path));

                // 检查目标
                if let Some(target) = get_symlink_target(&symlink_path) {
                    println!("Symlink 目标: {}", target.display());
                }

                // 尝试删除并重新创建（模拟版本切换）
                println!("🔄 尝试删除并重新创建...");

                if is_symlink(&symlink_path) {
                    match remove_symlink_dir(&symlink_path) {
                        Ok(()) => {
                            println!("✅ 删除成功");
                        }
                        Err(e) => {
                            println!("❌ 删除失败: {e}");
                        }
                    }
                }

                // 检查是否确实被删除
                println!("删除后路径是否存在: {}", symlink_path.exists());

                // 重新创建
                match symlink_dir(&source, &symlink_path) {
                    Ok(()) => {
                        println!("✅ 重新创建成功");
                    }
                    Err(e) => {
                        println!("❌ 重新创建失败: {e}");

                        // 如果失败，尝试强制清理
                        if symlink_path.exists() {
                            println!("🧹 尝试强制清理...");
                            if symlink_path.is_dir() {
                                if let Err(e2) = fs::remove_dir_all(&symlink_path) {
                                    println!("强制清理目录失败: {e2}");
                                } else {
                                    println!("强制清理目录成功");
                                }
                            }

                            // 再次尝试创建
                            match symlink_dir(&source, &symlink_path) {
                                Ok(()) => {
                                    println!("✅ 强制清理后创建成功");
                                }
                                Err(e3) => {
                                    println!("❌ 强制清理后仍失败: {e3}");
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                if e.to_string().contains("permission")
                    || e.to_string().contains("Access is denied")
                {
                    println!("⚠️ 跳过：权限不足 - {e}");
                } else {
                    println!("❌ 创建失败: {e}");
                }
            }
        }
    }
}
