//! Cross-platform compatibility module
//!
//! Provides unified platform detection and related utility functions.

/// Platform information structure
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformInfo {
    pub os: String,
    pub arch: String,
    pub extension: String,
}

impl PlatformInfo {
    /// Automatically detects the current platform information
    pub fn detect() -> Self {
        let (os, arch) = detect_os_arch();
        let extension = detect_archive_extension();

        Self { os: os.to_string(), arch: arch.to_string(), extension: extension.to_string() }
    }

    /// Generates the Go archive filename
    pub fn archive_filename(&self, version: &str) -> String {
        format!("go{}.{}-{}.{}", version, self.os, self.arch, self.extension)
    }

    /// Gets the Go executable filename (including platform-specific extension)
    pub fn go_executable_name() -> &'static str {
        if cfg!(target_os = "windows") {
            "go.exe"
        } else {
            "go"
        }
    }
}

impl Default for PlatformInfo {
    fn default() -> Self {
        Self::detect()
    }
}

/// Detects the operating system and architecture
fn detect_os_arch() -> (&'static str, &'static str) {
    let os = match std::env::consts::OS {
        "linux" => "linux",
        "macos" => "darwin",
        "windows" => "windows",
        _ => "linux", // Default to linux
    };

    let arch = match std::env::consts::ARCH {
        "x86_64" => "amd64",
        "x86" => "386",
        "aarch64" => "arm64",
        "arm" => "armv6l",
        _ => "amd64", // Default to amd64
    };

    (os, arch)
}

/// Detects the archive file extension
fn detect_archive_extension() -> &'static str {
    if cfg!(target_os = "windows") {
        "zip"
    } else {
        "tar.gz"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_detection() {
        let platform = PlatformInfo::detect();
        assert!(!platform.os.is_empty());
        assert!(!platform.arch.is_empty());
        assert!(!platform.extension.is_empty());
    }

    #[test]
    fn test_archive_filename() {
        let platform = PlatformInfo::detect();
        let filename = platform.archive_filename("1.21.0");
        assert!(filename.starts_with("go1.21.0."));
        assert!(filename.contains(&platform.os));
        assert!(filename.contains(&platform.arch));
        assert!(filename.ends_with(&platform.extension));
    }

    #[test]
    fn test_go_executable_name() {
        let exe_name = PlatformInfo::go_executable_name();
        if cfg!(target_os = "windows") {
            assert_eq!(exe_name, "go.exe");
        } else {
            assert_eq!(exe_name, "go");
        }
    }
}
