//! Cross-platform symbolic link unified interface module
//!
//! This module provides a cross-platform interface for symbolic link operations:
//! - Windows: Uses junction points for directory links (internal implementation).
//! - Unix: Uses symbolic links.
//!
//! The unified API allows higher-level code to be platform-agnostic.
//! All operations are attempted only once; on failure, a detailed error is returned directly
//! without retries, ensuring transparent error propagation to the user.

#[cfg(windows)]
use junction;

/// Creates a directory symbolic link cross-platform (junction on Windows, symlink on Unix)
///
/// # Arguments
/// * `from` - The target path the link points to
/// * `to` - The path of the link to be created
///
/// # Errors
/// Returns an error if the link cannot be created.
#[cfg(unix)]
pub fn create_symlink(from: &std::path::Path, to: &std::path::Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(from, to)
}

/// Creates a directory symbolic link cross-platform (junction on Windows, symlink on Unix)
///
/// # Arguments
/// * `from` - The target path the link points to
/// * `to` - The path of the link to be created
///
/// # Errors
/// Returns an error if the link cannot be created.
#[cfg(windows)]
pub fn create_symlink(from: &std::path::Path, to: &std::path::Path) -> std::io::Result<()> {
    junction::create(from, to)
}

/// Deletes a directory symbolic link cross-platform
///
/// # Arguments
/// * `path` - The path of the link to be deleted
///
/// # Errors
/// Returns an error if the link cannot be deleted.
#[cfg(windows)]
pub fn remove_symlink(path: &std::path::Path) -> std::io::Result<()> {
    // Windows: junctions are deleted as directories
    std::fs::remove_dir(path)
}

/// Deletes a directory symbolic link cross-platform
///
/// # Arguments
/// * `path` - The path of the link to be deleted
///
/// # Errors
/// Returns an error if the link cannot be deleted.
#[cfg(unix)]
pub fn remove_symlink(path: &std::path::Path) -> std::io::Result<()> {
    // Unix: symlinks are deleted as files
    std::fs::remove_file(path)
}

/// Reads the target of a symbolic link (cross-platform)
///
/// # Arguments
/// * `path` - The symbolic link path
///
/// # Returns
/// The target path the link points to
///
/// # Errors
/// Returns an error if the path is not a symbolic link or cannot be read.
#[cfg(windows)]
pub fn read_link(path: &std::path::Path) -> std::io::Result<std::path::PathBuf> {
    // Windows: use the junction crate
    junction::get_target(path)
}

/// Reads the target of a symbolic link (cross-platform)
///
/// # Arguments
/// * `path` - The symbolic link path
///
/// # Returns
/// The target path the link points to
///
/// # Errors
/// Returns an error if the path is not a symbolic link or cannot be read.
#[cfg(unix)]
pub fn read_link(path: &std::path::Path) -> std::io::Result<std::path::PathBuf> {
    // Unix: use standard read_link
    std::fs::read_link(path)
}

/// Checks if a path is a symbolic link/junction
///
/// # Arguments
/// * `path` - The path to check
#[cfg(windows)]
pub fn is_symlink(path: &std::path::Path) -> bool {
    junction::exists(path).unwrap_or(false)
}

#[cfg(unix)]
pub fn is_symlink(path: &std::path::Path) -> bool {
    path.symlink_metadata().map(|m| m.file_type().is_symlink()).unwrap_or(false)
}
