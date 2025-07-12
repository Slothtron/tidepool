//! 诊断 junction 问题的简单测试

#[cfg(test)]
mod junction_diagnostic_tests {
    use std::fs;
    use tempfile::TempDir;

    #[test]
    #[cfg(windows)]
    fn test_junction_basic_operations() {
        let temp_dir = TempDir::new().unwrap();

        // 创建源目录
        let source = temp_dir.path().join("source");
        fs::create_dir_all(&source).unwrap();
        fs::write(source.join("test.txt"), b"test content").unwrap();

        // 创建 junction
        let junction_path = temp_dir.path().join("link");

        println!("🔍 测试基本 junction 操作");
        println!("源目录: {}", source.display());
        println!("Junction 路径: {}", junction_path.display());

        // 第一次创建
        match junction::create(&junction_path, &source) {
            Ok(()) => {
                println!("✅ 第一次创建 junction 成功");

                // 验证 junction 存在
                println!(
                    "Junction 存在检查: {}",
                    junction::exists(&junction_path).unwrap_or(false)
                );

                // 检查目标
                if let Ok(target) = junction::get_target(&junction_path) {
                    println!("Junction 目标: {}", target.display());
                }

                // 尝试删除并重新创建（模拟版本切换）
                println!("🔄 尝试删除并重新创建...");

                if let Ok(true) = junction::exists(&junction_path) {
                    match junction::delete(&junction_path) {
                        Ok(()) => {
                            println!("✅ 删除成功");
                        }
                        Err(e) => {
                            println!("❌ 删除失败: {e}");
                        }
                    }
                }

                // 检查是否确实被删除
                println!("删除后路径是否存在: {}", junction_path.exists());

                // 重新创建
                match junction::create(&junction_path, &source) {
                    Ok(()) => {
                        println!("✅ 重新创建成功");
                    }
                    Err(e) => {
                        println!("❌ 重新创建失败: {e}");

                        // 如果失败，尝试强制清理
                        if junction_path.exists() {
                            println!("🧹 尝试强制清理...");
                            if junction_path.is_dir() {
                                if let Err(e2) = fs::remove_dir_all(&junction_path) {
                                    println!("强制清理目录失败: {e2}");
                                } else {
                                    println!("强制清理目录成功");
                                }
                            }

                            // 再次尝试创建
                            match junction::create(&junction_path, &source) {
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
