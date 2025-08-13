use crate::VersionList;
use console::{style, Term};
use std::io::Write;

/// Enhanced Icons for cross-platform compatibility with better visual design
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
            "✓"
        } else {
            "✅"
        }
    }

    /// Get error icon with fallback
    #[must_use]
    pub fn error() -> &'static str {
        if Self::should_use_ascii() {
            "✗"
        } else {
            "❌"
        }
    }

    /// Get warning icon with fallback
    #[must_use]
    pub fn warning() -> &'static str {
        if Self::should_use_ascii() {
            "⚠"
        } else {
            "⚠️"
        }
    }

    /// Get info icon with fallback
    #[must_use]
    pub fn info() -> &'static str {
        if Self::should_use_ascii() {
            "ℹ"
        } else {
            "ℹ️"
        }
    }

    /// Get hint icon with fallback
    #[must_use]
    pub fn hint() -> &'static str {
        if Self::should_use_ascii() {
            "💡"
        } else {
            "💡"
        }
    }

    /// Get package icon with fallback
    #[must_use]
    pub fn package() -> &'static str {
        if Self::should_use_ascii() {
            "📦"
        } else {
            "📦"
        }
    }

    /// Get download icon
    #[must_use]
    pub fn download() -> &'static str {
        if Self::should_use_ascii() {
            "⬇"
        } else {
            "⬇️"
        }
    }

    /// Get install icon
    #[must_use]
    pub fn install() -> &'static str {
        if Self::should_use_ascii() {
            "⚙"
        } else {
            "⚙️"
        }
    }

    /// Get current/active icon
    #[must_use]
    pub fn current() -> &'static str {
        if Self::should_use_ascii() {
            "★"
        } else {
            "⭐"
        }
    }

    /// Get arrow right icon with fallback
    #[must_use]
    pub fn arrow_right() -> &'static str {
        if Self::should_use_ascii() {
            "→"
        } else {
            "➡️"
        }
    }

    /// Get rocket icon for actions
    #[must_use]
    pub fn rocket() -> &'static str {
        if Self::should_use_ascii() {
            "🚀"
        } else {
            "🚀"
        }
    }

    /// Get trash icon for deletion
    #[must_use]
    pub fn trash() -> &'static str {
        if Self::should_use_ascii() {
            "🗑"
        } else {
            "🗑️"
        }
    }

    /// Get check mark
    #[must_use]
    pub fn check() -> &'static str {
        "✓"
    }

    /// Get cross mark
    #[must_use]
    pub fn cross() -> &'static str {
        "✗"
    }
}

/// Enhanced UI utility module for beautiful terminal output
pub struct UI {
    term: Term,
}

impl Default for UI {
    fn default() -> Self {
        Self::new()
    }
}

impl UI {
    #[must_use]
    pub fn new() -> Self {
        Self {
            term: Term::stdout(),
        }
    }

    /// Print a banner with centered text
    pub fn banner(&self, text: &str) {
        let width = self.term.size().1 as usize;
        let text_len = text.len();
        let padding = if width > text_len { (width - text_len) / 2 } else { 0 };

        println!();
        println!("{}", "═".repeat(width.min(80)));
        println!("{}{}", " ".repeat(padding), style(text).cyan().bold());
        println!("{}", "═".repeat(width.min(80)));
        println!();
    }

    /// Print a section header with decorative styling
    pub fn header(&self, text: &str) {
        println!();
        println!("{} {}", style("▶").blue().bold(), style(text).cyan().bold());
        println!("{}", style("─".repeat(text.len() + 2)).dim());
    }

    /// Print a success message with enhanced styling
    pub fn success(&self, message: &str) {
        println!("{} {}", style(Icons::success()).green().bold(), style(message).green());
    }

    /// Print an error message with enhanced styling
    pub fn error(&self, message: &str) {
        println!("{} {}", style(Icons::error()).red().bold(), style(message).red().bold());
    }

    /// Print a warning message with enhanced styling
    pub fn warning(&self, message: &str) {
        println!("{} {}", style(Icons::warning()).yellow().bold(), style(message).yellow());
    }

    /// Print an info message with enhanced styling
    pub fn info(&self, message: &str) {
        println!("{} {}", style(Icons::info()).blue(), style(message).white());
    }

    /// Print a hint/tip message with enhanced styling
    pub fn hint(&self, message: &str) {
        println!("{} {} {}", style(Icons::hint()).yellow(), style("Tip:").cyan().bold(), style(message).white());
    }

    /// Print an action suggestion
    pub fn suggest(&self, message: &str) {
        println!("{} {} {}", style(Icons::arrow_right()).cyan(), style("Next:").cyan().bold(), style(message).cyan());
    }

    /// Print a progress message
    pub fn progress(&self, message: &str) {
        print!("{} {}...", style(Icons::install()).blue(), style(message).blue());
        let _ = std::io::stdout().flush();
    }

    /// Complete a progress message
    pub fn progress_done(&self, message: &str) {
        println!(" {}", style(message).green());
    }

    /// Print key-value pair with enhanced styling
    pub fn kv_pair(&self, key: &str, value: &str) {
        println!("  {} {}",
            style(format!("{}:", key)).dim().bold(),
            style(value).white()
        );
    }

    /// Print key-value pair with colored value
    pub fn kv_pair_colored(&self, key: &str, value: &str, color: &str) {
        let styled_value = match color {
            "green" => style(value).green(),
            "red" => style(value).red(),
            "yellow" => style(value).yellow(),
            "blue" => style(value).blue(),
            "cyan" => style(value).cyan(),
            "dimmed" => style(value).dim(),
            "bold" => style(value).bold(),
            _ => style(value),
        };
        println!("  {} {}",
            style(format!("{}:", key)).dim().bold(),
            styled_value
        );
    }

    /// Print a list item with enhanced styling
    pub fn list_item(&self, icon: &str, text: &str, is_highlight: bool) {
        if is_highlight {
            println!("  {} {}", style(icon).cyan().bold(), style(text).cyan().bold());
        } else {
            println!("  {} {}", style(icon).dim(), style(text).white());
        }
    }

    /// Print a separator line
    pub fn separator(&self) {
        println!("{}", style("─".repeat(50)).dim());
    }

    /// Print an empty line
    pub fn newline(&self) {
        println!();
    }

    /// Print multiple empty lines
    pub fn space(&self, lines: usize) {
        for _ in 0..lines {
            println!();
        }
    }

    /// Display version list with enhanced styling
    pub fn display_version_list(&self, list: &VersionList, title: &str) {
        self.header(title);

        if list.versions.is_empty() {
            self.warning("No versions found");
            return;
        }

        for version in &list.versions {
            self.list_item(Icons::package(), &version.version, false);
        }

        self.newline();
        self.info(&format!("Total: {} versions", list.total_count));
    }

    /// Display version list with current version highlighted
    pub fn display_version_list_with_current(
        &self,
        list: &VersionList,
        title: &str,
        current_version: Option<&str>,
    ) {
        self.header(title);

        if list.versions.is_empty() {
            self.warning("No versions found");
            self.suggest("Install a version with: gvm install <version>");
            return;
        }

        for version in &list.versions {
            if let Some(current) = current_version {
                if version.version == current {
                    println!("  {} {} {}",
                        style(Icons::current()).yellow().bold(),
                        style(&version.version).green().bold(),
                        style("(active)").dim()
                    );
                } else {
                    self.list_item(Icons::package(), &version.version, false);
                }
            } else {
                self.list_item(Icons::package(), &version.version, false);
            }
        }

        self.separator();
        self.kv_pair_colored("Total versions", &list.total_count.to_string(), "cyan");

        if let Some(current) = current_version {
            self.kv_pair_colored("Active version", current, "green");
        } else {
            self.warning("No version currently active");
            self.suggest("Activate a version with: gvm use <version>");
        }
    }

    /// Display installation progress and result with enhanced styling
    pub fn display_install_result(&self, version_info: &crate::VersionInfo) {
        self.success(&format!("Go {} installed successfully!", version_info.version));
        self.kv_pair_colored("Installation path", &version_info.path.display().to_string(), "dimmed");
        self.separator();
        self.suggest(&format!("Activate this version: gvm use {}", version_info.version));
    }

    /// Display installation start message
    pub fn display_install_start(&self, version: &str) {
        self.banner(&format!("Installing Go {}", version));
        self.info("This may take a few minutes depending on your internet connection...");
        self.newline();
    }

    /// Display uninstall start message
    pub fn display_uninstall_start(&self, version: &str) {
        self.header(&format!("{} Uninstalling Go {}", Icons::trash(), version));
    }

    /// Display uninstall success
    pub fn display_uninstall_success(&self, version: &str) {
        self.success(&format!("Go {} has been uninstalled", version));
        self.suggest("Install another version with: gvm install <version>");
    }

    /// Display switch/use success
    pub fn display_switch_success(&self, version: &str) {
        self.success(&format!("Switched to Go {}", version));
        self.info("Environment updated - restart your terminal or run 'source ~/.bashrc'");
        self.suggest("Verify installation: go version");
    }

    /// Display detailed information about a Go version
    pub fn display_version_info(&self, info: &crate::GoVersionInfo) {
        self.header(&format!("{} Go {} Details", Icons::package(), info.version));

        // Basic information section
        self.kv_pair("Version", &info.version);
        self.kv_pair("Platform", &format!("{}-{}", info.os, info.arch));
        self.kv_pair("Archive", &info.filename);

        // File size
        if let Some(size) = info.size {
            #[allow(clippy::cast_precision_loss)]
            let size_mb = size as f64 / 1024.0 / 1024.0;
            self.kv_pair("Size", &format!("{size_mb:.1} MB"));
        }

        self.separator();

        // Status section
        println!("  {}", style("Status:").dim().bold());

        if info.is_installed {
            println!("    {} {}", style(Icons::check()).green(), style("Installed").green());
            if let Some(ref path) = info.install_path {
                println!("    {} {}", style("📁").dim(), style(path.display().to_string()).dim());
            }
        } else {
            println!("    {} {}", style(Icons::cross()).red(), style("Not installed").red());
        }

        if info.is_cached {
            println!("    {} {}", style(Icons::check()).green(), style("Cached").green());
        } else {
            println!("    {} {}", style(Icons::cross()).yellow(), style("Not cached").yellow());
        }

        self.separator();

        // Security information
        if let Some(ref sha256) = info.sha256 {
            self.kv_pair_colored("SHA256", sha256, "dimmed");
        } else {
            self.warning("SHA256 verification not available for this version");
        }

        self.newline();

        // Action suggestions
        if info.is_installed {
            self.suggest(&format!("Use this version: gvm use {}", info.version));
            self.hint(&format!("Remove this version: gvm uninstall {}", info.version));
        } else {
            self.suggest(&format!("Install this version: gvm install {}", info.version));
        }
    }

    /// Display status information with enhanced layout
    pub fn display_status(&self, status: &crate::RuntimeStatus, base_dir: &std::path::Path, config: &crate::config::Config) {
        self.banner("Go Version Manager Status");

        // Current version section
        if let Some(ref current_version) = status.current_version {
            self.kv_pair_colored("Active Version", current_version, "green");
        } else {
            self.warning("No Go version is currently active");
        }

        if let Some(ref install_path) = status.install_path {
            self.kv_pair_colored("Go Root", &install_path.display().to_string(), "cyan");
        }

        self.separator();

        // Environment section
        self.header("Environment Variables");
        for (key, value) in &status.environment_vars {
            self.kv_pair(key, value);
        }

        self.separator();

        // Configuration section
        self.header("Configuration");
        self.kv_pair_colored("Base Directory", &base_dir.display().to_string(), "dimmed");
        self.kv_pair_colored("Versions Directory", &config.versions().display().to_string(), "dimmed");
        self.kv_pair_colored("Cache Directory", &config.cache().display().to_string(), "dimmed");

        // Link information
        if let Some(ref link_info) = status.link_info {
            self.separator();
            self.header("Symlink Information");
            self.info(link_info);
        }

        self.newline();
        self.suggest("List available versions: gvm list");
        self.hint("Install a new version: gvm install <version>");
    }

    /// Display cache info
    pub fn display_cache_info(&self, message: &str) {
        println!("{} {}", style(Icons::download()).blue(), style(message).blue());
    }

    /// Display error with suggestions
    pub fn display_error_with_suggestion(&self, error: &str, suggestion: &str) {
        self.error(error);
        self.suggest(suggestion);
    }

    /// Display download progress
    pub fn display_download_progress(&self, current: u64, total: u64) {
        let percentage = if total > 0 { (current * 100) / total } else { 0 };
        let bar_length = 40u64;
        let filled = (percentage * bar_length / 100) as usize;
        let remaining = (bar_length - (filled as u64)) as usize;
        let bar = format!("{}{}",
            "█".repeat(filled),
            "░".repeat(remaining)
        );

        print!("\r{} [{}] {}%",
            style("Downloading").blue(),
            style(bar).cyan(),
            style(percentage).bold()
        );
        let _ = std::io::stdout().flush();
    }

    /// Complete download progress
    pub fn complete_download_progress(&self) {
        println!();
        self.success("Download completed");
    }
}

/// Enhanced message templates for consistent output
pub struct Messages;

impl Messages {
    // Installation messages
    #[must_use]
    pub fn installing_go(version: &str) -> String {
        format!("Installing Go {}", version)
    }

    #[must_use]
    pub fn uninstalling_go(version: &str) -> String {
        format!("Uninstalling Go {}", version)
    }

    #[must_use]
    pub fn go_uninstalled_successfully(version: &str) -> String {
        format!("Go {} has been uninstalled successfully", version)
    }

    #[must_use]
    pub fn go_not_installed(version: &str) -> String {
        format!("Go {} is not installed", version)
    }

    #[must_use]
    pub fn uninstall_failed(version: &str, error: &str) -> String {
        format!("Failed to uninstall Go {}: {}", version, error)
    }

    #[must_use]
    pub fn cannot_uninstall_current_version(version: &str) -> String {
        format!("Cannot uninstall Go {} - it is currently active", version)
    }

    #[must_use]
    pub fn clear_current_symlink_hint() -> String {
        "Switch to another version first or manually remove the symlink".to_string()
    }

    // List messages
    #[must_use]
    pub fn installed_go_versions() -> String {
        "Installed Go Versions".to_string()
    }

    #[must_use]
    pub fn available_go_versions() -> String {
        "Available Go Versions".to_string()
    }

    #[must_use]
    pub fn no_go_versions_found() -> String {
        "No Go versions found".to_string()
    }

    #[must_use]
    pub fn installation_directory_not_found(path: &str) -> String {
        format!("Installation directory not found: {}", path)
    }

    #[must_use]
    pub fn install_version_hint() -> String {
        "gvm install <version>".to_string()
    }

    #[must_use]
    pub fn use_version_hint() -> String {
        "gvm use <version>".to_string()
    }

    #[must_use]
    pub fn error_listing_versions(error: &str) -> String {
        format!("Failed to list versions: {}", error)
    }

    #[must_use]
    pub fn error_getting_available_versions(error: &str) -> String {
        format!("Failed to get available versions: {}", error)
    }

    #[must_use]
    pub fn visit_go_website() -> String {
        "Visit https://go.dev/dl/ for complete version list".to_string()
    }

    #[must_use]
    pub fn install_with_hint() -> String {
        "gvm install <version>".to_string()
    }

    #[must_use]
    pub fn switched_to_go_successfully(version: &str) -> String {
        format!("Successfully switched to Go {}", version)
    }

    #[must_use]
    pub fn switch_failed(error: &str) -> String {
        format!("Failed to switch version: {}", error)
    }

    #[must_use]
    pub fn error_getting_status(error: &str) -> String {
        format!("Failed to get status: {}", error)
    }

    #[must_use]
    pub fn removing_existing_installation(version: &str) -> String {
        format!("Removing existing Go {} installation", version)
    }

    #[must_use]
    pub fn found_cached_download(version: &str) -> String {
        format!("Found cached download for Go {}", version)
    }

    #[must_use]
    pub fn installation_failed(error: &str) -> String {
        format!("Installation failed: {}", error)
    }

    #[must_use]
    pub fn extraction_progress() -> String {
        "Extracting archive".to_string()
    }

    #[must_use]
    pub fn download_progress() -> String {
        "Downloading Go archive".to_string()
    }

    #[must_use]
    pub fn verification_progress() -> String {
        "Verifying download integrity".to_string()
    }
}
