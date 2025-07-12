//! 提供 junction 工具函数

use log::{debug, warn};
use std::path::Path;

/// 安全删除 junction 或目录的工具函数
pub fn safe_remove_junction_or_dir(path: &Path) -> Result<(), String> {
    if !path.exists() {
        debug!("Path does not exist, nothing to remove: {}", path.display());
        return Ok(());
    }

    debug!("Attempting to remove path: {}", path.display());

    // 在Windows上尝试多种删除方法，增加更多重试逻辑
    let mut attempts = 0;
    const MAX_ATTEMPTS: usize = 5;

    while path.exists() && attempts < MAX_ATTEMPTS {
        attempts += 1;
        debug!("Removal attempt {attempts}");

        // 方法1: 尝试使用 junction crate（如果是 junction）
        #[cfg(target_os = "windows")]
        {
            if let Ok(true) = junction::exists(path) {
                debug!("Detected junction, using junction crate to remove");
                if let Ok(()) = junction::delete(path) {
                    debug!("Successfully removed junction with junction crate");
                    if !path.exists() {
                        return Ok(());
                    }
                } else {
                    warn!("Junction crate deletion failed, trying alternative methods");
                }
            }
        }

        // 方法2: Windows 特殊处理 - 使用 rmdir
        #[cfg(target_os = "windows")]
        {
            if path.exists() {
                debug!("Trying Windows rmdir command");
                let output = std::process::Command::new("cmd")
                    .args(["/C", "rmdir", "/S", "/Q", &path.to_string_lossy()])
                    .output();

                if let Ok(output) = output {
                    if output.status.success() {
                        debug!("Successfully removed with rmdir");
                        if !path.exists() {
                            return Ok(());
                        }
                    } else {
                        debug!("rmdir failed: {}", String::from_utf8_lossy(&output.stderr));
                    }
                }
            }
        }

        // 方法3: 标准文件系统操作
        if path.exists() {
            if path.is_dir() {
                debug!("Trying standard remove_dir_all");
                if let Ok(()) = std::fs::remove_dir_all(path) {
                    debug!("Successfully removed directory with remove_dir_all");
                    if !path.exists() {
                        return Ok(());
                    }
                }
            } else {
                debug!("Trying standard remove_file");
                if let Ok(()) = std::fs::remove_file(path) {
                    debug!("Successfully removed file with remove_file");
                    if !path.exists() {
                        return Ok(());
                    }
                }
            }
        }

        // 如果路径仍然存在，添加延迟后重试
        if path.exists() && attempts < MAX_ATTEMPTS {
            debug!("Path still exists, waiting before retry");
            std::thread::sleep(std::time::Duration::from_millis(100 * attempts as u64));
        }
    }

    // 最终验证
    if path.exists() {
        return Err(format!(
            "Path still exists after {MAX_ATTEMPTS} removal attempts: {}",
            path.display()
        ));
    }

    debug!("Successfully removed path: {}", path.display());
    Ok(())
}

/// 安全创建 junction 的工具函数
pub fn safe_create_junction(junction_path: &Path, target_path: &Path) -> Result<(), String> {
    // 首先确保目标路径不存在
    safe_remove_junction_or_dir(junction_path)?;

    // 在 Windows 上添加延迟，确保文件系统完全释放资源
    #[cfg(target_os = "windows")]
    {
        std::thread::sleep(std::time::Duration::from_millis(150));
    }

    // 创建 junction，带重试逻辑
    let mut attempts = 0;
    const MAX_ATTEMPTS: usize = 3;

    loop {
        attempts += 1;
        debug!(
            "Creating junction attempt {attempts}: {} -> {}",
            junction_path.display(),
            target_path.display()
        );

        match junction::create(junction_path, target_path) {
            Ok(()) => {
                debug!("Successfully created junction on attempt {attempts}");
                return Ok(());
            }
            Err(e) => {
                if attempts >= MAX_ATTEMPTS {
                    return Err(format!(
                        "Failed to create junction after {MAX_ATTEMPTS} attempts: {e}"
                    ));
                }

                debug!("Junction creation attempt {attempts} failed: {e}");

                // 如果创建失败且路径存在，尝试清理后重试
                if junction_path.exists() {
                    debug!("Target path exists after failed creation, cleaning up");
                    let _ = safe_remove_junction_or_dir(junction_path);
                }

                // 增加延迟后重试
                std::thread::sleep(std::time::Duration::from_millis(200 * attempts as u64));
            }
        }
    }
}
