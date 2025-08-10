use crate::VersionList;
use console::style;

/// Icons for cross-platform compatibility
pub struct Icons;

impl Icons {
    /// Determine if we should use ASCII icons based on environment
    fn should_use_ascii() -> bool {
        // Check for explicit user preference first
        match std::env::var("GVM_ICON_STYLE").as_deref() {
            Ok("ascii") => return true,
            Ok("unicode") => return false,
            _ => {} // Fall through to auto-detection
        }

        // Auto-detection based on platform and terminal
        std::env::var("TERM").unwrap_or_default().is_empty() ||
        std::env::var("WT_SESSION").is_ok() ||  // Windows Terminal
        std::env::consts::OS == "windows"
    }

    /// Get success icon with fallback for unsupported terminals
    #[must_use]
    pub fn success() -> &'static str {
        if Self::should_use_ascii() {
            "√"
        } else {
            "✓"
        }
    }

    /// Get error icon with fallback
    #[must_use]
    pub fn error() -> &'static str {
        if Self::should_use_ascii() {
            "×"
        } else {
            "✗"
        }
    }

    /// Get warning icon with fallback
    #[must_use]
    pub fn warning() -> &'static str {
        if Self::should_use_ascii() {
            "!"
        } else {
            "⚠"
        }
    }

    /// Get info icon with fallback
    #[must_use]
    pub fn info() -> &'static str {
        if Self::should_use_ascii() {
            "i"
        } else {
            "ℹ"
        }
    }

    /// Get hint icon with fallback
    #[must_use]
    pub fn hint() -> &'static str {
        if Self::should_use_ascii() {
            "*"
        } else {
            "💡"
        }
    }

    /// Get package icon with fallback
    #[must_use]
    pub fn package() -> &'static str {
        if Self::should_use_ascii() {
            ">"
        } else {
            "📦"
        }
    }

    /// Get arrow right icon with fallback
    #[must_use]
    pub fn arrow_right() -> &'static str {
        if Self::should_use_ascii() {
            "->"
        } else {
            "➡"
        }
    }
}

/// UI utility module for consistent terminal output formatting
pub struct UI;

impl Default for UI {
    fn default() -> Self {
        Self::new()
    }
}

impl UI {
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Print a success message
    #[allow(clippy::unused_self)]
    pub fn success(&self, message: &str) {
        println!("{} {}", style(Icons::success()).green().bold(), message);
    }

    /// Print an error message
    #[allow(clippy::unused_self)]
    pub fn error(&self, message: &str) {
        println!("{} {}", style(Icons::error()).red().bold(), message);
    }

    /// Print a warning message
    #[allow(clippy::unused_self)]
    pub fn warning(&self, message: &str) {
        println!("{} {}", style(Icons::warning()).yellow(), message);
    }

    /// Print an info message
    #[allow(clippy::unused_self)]
    pub fn info(&self, message: &str) {
        println!("{} {}", style(Icons::info()).blue(), message);
    }

    /// Print a hint/tip message
    #[allow(clippy::unused_self)]
    pub fn hint(&self, message: &str) {
        println!("{} {}", style(Icons::hint()).blue(), message);
    }

    /// Print a section header
    #[allow(clippy::unused_self)]
    pub fn header(&self, text: &str) {
        println!("{}", style(text).cyan().bold());
    }

    /// Print key-value pair
    #[allow(clippy::unused_self)]
    pub fn kv_pair(&self, key: &str, value: &str) {
        println!("  {} {}", style(format!("{key}:")).dim(), value);
    }

    /// Print key-value pair with colored value
    #[allow(clippy::unused_self)]
    pub fn kv_pair_colored(&self, key: &str, value: &str, color: &str) {
        let styled_value = match color {
            "green" => style(value).green(),
            "red" => style(value).red(),
            "yellow" => style(value).yellow(),
            "blue" => style(value).blue(),
            "cyan" => style(value).cyan(),
            "dimmed" => style(value).dim(),
            _ => style(value),
        };
        println!("  {} {}", style(format!("{key}:")).dim(), styled_value);
    }

    /// Print a list item
    #[allow(clippy::unused_self)]
    pub fn list_item(&self, icon: &str, text: &str) {
        println!("  {icon} {text}");
    }

    /// Print an empty line
    #[allow(clippy::unused_self)]
    pub fn newline(&self) {
        println!();
    }

    /// Display version list
    pub fn display_version_list(&self, list: &VersionList, title: &str) {
        self.header(title);

        if list.versions.is_empty() {
            self.warning("No versions found");
            return;
        }

        for version in &list.versions {
            self.list_item(Icons::package(), &style(version).green().to_string());
        }

        self.newline();
        self.info(&format!("Total: {} versions", list.total_count));
    }

    /// Display version list with current version marked
    pub fn display_version_list_with_current(
        &self,
        list: &VersionList,
        title: &str,
        current_version: Option<&str>,
    ) {
        self.header(title);

        if list.versions.is_empty() {
            self.warning("No versions found");
            return;
        }

        for version in &list.versions {
            if let Some(current) = current_version {
                if version.version == current {
                    // 标记当前使用的版本
                    self.list_item(
                        Icons::arrow_right(),
                        &format!("{} {}", style(version).green().bold(), style("(current)").dim()),
                    );
                } else {
                    self.list_item(Icons::package(), &style(version).green().to_string());
                }
            } else {
                self.list_item(Icons::package(), &style(version).green().to_string());
            }
        }

        self.newline();
        self.info(&format!("Total: {} versions", list.total_count));

        if let Some(current) = current_version {
            self.info(&format!("Current: {}", style(current).cyan().bold()));
        } else {
            self.warning("No version currently active");
        }
    }

    /// Display installation progress and result
    pub fn display_install_result(&self, version_info: &crate::VersionInfo) {
        self.success(&format!("Go {} installed successfully!", version_info.version));
        self.kv_pair_colored("Location", &version_info.path.display().to_string(), "dimmed");
        self.newline();
        self.hint(&format!("Use this version: gvm use {}", version_info.version));
    }

    /// Display detailed information about a Go version
    pub fn display_version_info(&self, info: &crate::GoVersionInfo) {
        // Basic information
        self.kv_pair("Version", &info.version);
        self.kv_pair("Platform", &format!("{}-{}", info.os, info.arch));
        self.kv_pair("Filename", &info.filename);
        self.kv_pair("Download URL", &info.download_url);

        // Hash information
        if let Some(ref sha256) = info.sha256 {
            self.kv_pair("SHA256", sha256);
        } else {
            self.kv_pair_colored("SHA256", "Not available (unsupported version)", "yellow");
        }

        // File size
        if let Some(size) = info.size {
            #[allow(clippy::cast_precision_loss)]
            let size_mb = size as f64 / 1024.0 / 1024.0;
            self.kv_pair("Size", &format!("{size_mb:.1} MB ({size} bytes)"));
        } else {
            self.kv_pair_colored("Size", "Unknown", "dimmed");
        }

        self.newline();

        // Status information
        if info.is_installed {
            self.success("✓ Installed");
            if let Some(ref path) = info.install_path {
                self.kv_pair_colored("Install Path", &path.display().to_string(), "dimmed");
            }
        } else {
            self.info("○ Not installed");
        }

        if info.is_cached {
            self.success("✓ Cached");
            if let Some(ref path) = info.cache_path {
                self.kv_pair_colored("Cache Path", &path.display().to_string(), "dimmed");
            }
        } else {
            self.info("○ Not cached");
        }

        self.newline();

        // Hint information
        if info.is_installed {
            self.hint(&format!("Switch to this version: gvm use {}", info.version));
        } else {
            self.hint(&format!("Install this version: gvm install {}", info.version));
        }
    }
}

/// Message templates for consistent output
pub struct Messages;

impl Messages {
    // Installation messages
    #[must_use]
    pub fn installing_go(version: &str) -> String {
        format!("{} {}", style("Installing Go").cyan().bold(), style(version).green().bold())
    }

    #[must_use]
    pub fn uninstalling_go(version: &str) -> String {
        format!("{} {}", style("Uninstalling Go").cyan().bold(), style(version).red().bold())
    }

    #[must_use]
    pub fn go_uninstalled_successfully(version: &str) -> String {
        format!("Go {} uninstalled successfully!", style(version).green())
    }

    #[must_use]
    pub fn go_not_installed(version: &str) -> String {
        format!("Go {} is not installed", style(version).red())
    }

    #[must_use]
    pub fn uninstall_failed(version: &str, error: &str) -> String {
        format!("Failed to uninstall Go {}: {}", style(version).red(), style(error).red())
    }

    #[must_use]
    pub fn cannot_uninstall_current_version(version: &str) -> String {
        format!(
            "{} Cannot uninstall Go {} as it is currently active.\n{}",
            style("Warning:").yellow().bold(),
            style(version).yellow(),
            style("Please switch to another version or clear the current symlink first.").dim()
        )
    }

    #[must_use]
    pub fn clear_current_symlink_hint() -> String {
        format!(
            "Solutions: {} or {}",
            style("gvm install <other-version>").cyan(),
            style("remove the 'current' symlink manually").cyan()
        )
    }

    // List messages
    #[must_use]
    pub fn installed_go_versions() -> String {
        style("Installed Go versions:").cyan().bold().to_string()
    }

    #[must_use]
    pub fn available_go_versions() -> String {
        style("Available Go versions (latest releases):").cyan().bold().to_string()
    }

    #[must_use]
    pub fn no_go_versions_found() -> String {
        "No Go versions found".to_string()
    }

    #[must_use]
    pub fn installation_directory_not_found(path: &str) -> String {
        format!("Installation directory not found: {}", style(path).dim())
    }

    #[must_use]
    pub fn install_version_hint() -> String {
        format!("Install a version: {}", style("gvm install <version>").cyan())
    }

    #[must_use]
    pub fn use_version_hint() -> String {
        format!("Install version: {}", style("gvm install <version>").cyan())
    }

    #[must_use]
    pub fn error_listing_versions(error: &str) -> String {
        format!("Error listing versions: {}", style(error).red())
    }

    #[must_use]
    pub fn error_getting_available_versions(error: &str) -> String {
        format!("Error getting available versions: {error}")
    }

    #[must_use]
    pub fn visit_go_website() -> String {
        format!("For complete list visit: {}", style("https://go.dev/dl/").blue())
    }

    #[must_use]
    pub fn install_with_hint() -> String {
        format!("Install with: {}", style("gvm install <version>").cyan())
    }

    #[must_use]
    pub fn switched_to_go_successfully(version: &str) -> String {
        format!("Successfully switched to Go {}!", style(version).green())
    }

    #[must_use]
    pub fn switch_failed(error: &str) -> String {
        format!("Switch failed: {}", style(error).red())
    }

    #[must_use]
    pub fn error_getting_status(error: &str) -> String {
        format!("Error getting status: {error}")
    }

    #[must_use]
    pub fn removing_existing_installation(version: &str) -> String {
        format!("{} {}", style("Removing existing installation...").yellow(), version)
    }

    #[must_use]
    pub fn found_cached_download(version: &str) -> String {
        format!("Found cached download for Go {}", style(version).green())
    }

    #[must_use]
    pub fn installation_failed(error: &str) -> String {
        format!("Installation failed: {}", style(error).red())
    }
}
