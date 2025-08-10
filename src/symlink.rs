//! 跨平台符号链接统一接口模块
//!
//! 这个模块提供了跨平台的符号链接操作接口：
//! - Windows: 使用 junction points 进行目录链接（内部实现）
//! - Unix: 使用符号链接
//!
//! 统一的 API 使得上层代码无需关心平台差异。
//!
//! 所有操作都只尝试一次，失败时直接返回详细错误信息，
//! 不进行重试，确保错误信息透明传递给用户。

use log::debug;
use std::path::{Path, PathBuf};

/// 跨平台创建目录符号链接（Windows 使用 junction，Unix 使用 symlink）
///
/// # 参数
/// * `from` - 链接指向的目标路径
/// * `to` - 要创建的链接路径
///
/// # 错误
/// 当无法创建链接时返回错误
pub fn symlink_dir<P: AsRef<Path>, U: AsRef<Path>>(src: P, dst: U) -> std::io::Result<()> {
    let src = src.as_ref();
    let dst = dst.as_ref();

    debug!("Creating symlink: {} -> {}", dst.display(), src.display());

    #[cfg(not(target_os = "windows"))]
    {
        std::os::unix::fs::symlink(src, dst)?;
    }

    #[cfg(target_os = "windows")]
    {
        junction::create(src, dst)?;
    }

    debug!("Successfully created symlink: {} -> {}", dst.display(), src.display());
    Ok(())
}

/// 跨平台删除目录符号链接
///
/// # 参数
/// * `path` - 要删除的链接路径
///
/// # 错误
/// 当无法删除链接时返回错误
pub fn remove_symlink_dir<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    let path = path.as_ref();
    debug!("Removing symlink: {}", path.display());

    #[cfg(target_os = "windows")]
    {
        // Windows: junction 被当作目录删除
        std::fs::remove_dir(path)?;
    }

    #[cfg(not(target_os = "windows"))]
    {
        // Unix: symlink 被当作文件删除
        std::fs::remove_file(path)?;
    }

    debug!("Successfully removed symlink: {}", path.display());
    Ok(())
}

/// 读取符号链接目标（跨平台）
///
/// # 参数
/// * `path` - 符号链接路径
///
/// # 返回
/// 链接指向的目标路径
///
/// # 错误
/// 当路径不是符号链接或无法读取时返回错误
pub fn read_symlink<P: AsRef<Path>>(path: P) -> std::io::Result<PathBuf> {
    let path = path.as_ref();
    #[cfg(target_os = "windows")]
    {
        // Windows: 使用 junction crate
        junction::get_target(path)
    }

    #[cfg(not(target_os = "windows"))]
    {
        // Unix: 使用标准 read_link
        std::fs::read_link(path)
    }
}

/// 检查路径是否为符号链接/junction
///
/// # 参数
/// * `path` - 要检查的路径
///
/// # 返回
/// 如果是符号链接/junction 返回 true，否则返回 false
pub fn is_symlink<P: AsRef<Path>>(path: P) -> bool {
    let path = path.as_ref();
    #[cfg(target_os = "windows")]
    {
        // Windows: 仅检查 junction
        junction::exists(path).unwrap_or(false)
    }

    #[cfg(not(target_os = "windows"))]
    {
        // Unix: 仅检查标准 symlink
        path.is_symlink()
    }
}

/// 获取符号链接目标路径（如果是符号链接）
///
/// # 参数
/// * `path` - 要检查的路径
///
/// # 返回
/// 如果是符号链接，返回目标路径；否则返回 None
pub fn get_symlink_target<P: AsRef<Path>>(path: P) -> Option<PathBuf> {
    let path = path.as_ref();

    if !path.exists() {
        return None;
    }
    #[cfg(target_os = "windows")]
    {
        // Windows: 仅使用 junction crate
        if junction::exists(path).unwrap_or(false) {
            junction::get_target(path).ok()
        } else {
            None
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        // Unix: 仅使用标准 symlink
        if path.is_symlink() {
            std::fs::read_link(path).ok()
        } else {
            None
        }
    }
}
