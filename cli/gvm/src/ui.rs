use console::style;
use tidepool_version_manager::VersionList;

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
    /// Print a success message
    #[allow(clippy::unused_self)]
    pub fn success(&self, message: &str) {
        println!("{} {}", style(Icons::success()).green().bold(), message);
    }    /// Print an error message
    #[allow(clippy::unused_self)]
    pub fn error(&self, message: &str) {
        println!("{} {}", style(Icons::error()).red().bold(), message);
    }    /// Print a warning message
    #[allow(clippy::unused_self)]
    pub fn warning(&self, message: &str) {
        println!("{} {}", style(Icons::warning()).yellow(), message);
    }

    /// Print an info message
    /// Print an info message
    #[allow(clippy::unused_self)]
    pub fn info(&self, message: &str) {
        println!("{} {}", style(Icons::info()).blue(), message);
    }

    /// Print a hint/tip message
    /// Print a hint message
    #[allow(clippy::unused_self)]
    pub fn hint(&self, message: &str) {
        println!("{} {}", style(Icons::hint()).blue(), message);
    }
    /// Print a section header
    /// Print a header message
    #[allow(clippy::unused_self)]
    pub fn header(&self, text: &str) {
        println!("{}", style(text).cyan().bold());
    }

    /// Print key-value pair
    /// Print a key-value pair
    #[allow(clippy::unused_self)]
    pub fn kv_pair(&self, key: &str, value: &str) {
        println!("  {} {}", style(format!("{key}:")).dim(), value);
    }

    /// Print key-value pair with colored value
    /// Print a colored key-value pair
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
    /// Print a list item
    #[allow(clippy::unused_self)]
    pub fn list_item(&self, icon: &str, text: &str) {
        println!("  {icon} {text}");
    }
    /// Print an empty line
    /// Print a newline
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
                if version == current {
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
    pub fn display_install_result(&self, version_info: &tidepool_version_manager::VersionInfo) {
        self.success(&format!("Go {} installed successfully!", version_info.version));
        self.kv_pair_colored(
            "Location",
            &version_info.install_path.display().to_string(),
            "dimmed",
        );
        self.newline();
        self.hint(&format!("Use this version: gvm use {}", version_info.version));
    }

    /// Display detailed information about a Go version
    pub fn display_version_info(&self, info: &tidepool_version_manager::GoVersionInfo) {
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

    /// 显示环境变量配置说明，支持不同操作系统
    pub fn show_environment_setup(&self, install_path: &std::path::Path, version: &str) {
        self.newline();
        self.header("📋 环境变量配置说明");

        // 计算需要配置的路径
        let bin_path = install_path.join("bin");
        let go_root = install_path;

        // 检测操作系统并显示相应的配置说明
        if cfg!(target_os = "windows") {
            self.show_windows_env_setup(&bin_path, go_root, version);
        } else {
            self.show_unix_env_setup(&bin_path, go_root, version);
        }

        self.newline();
        self.hint(&format!("💡 切换完成！现在可以使用 Go {version} 了"));
        self.hint("   运行 'go version' 验证当前版本");
    }

    /// 显示 Windows 环境变量配置说明
    fn show_windows_env_setup(
        &self,
        bin_path: &std::path::Path,
        go_root: &std::path::Path,
        version: &str,
    ) {
        self.info(&format!("已切换到 Go {version}，以下是环境变量配置说明："));
        self.newline();

        // PowerShell 配置
        self.list_item("🔷", "PowerShell 临时配置（当前会话）:");
        println!("    $env:GOROOT = \"{}\"", go_root.display());
        println!("    $env:PATH = \"{};$env:PATH\"", bin_path.display());
        self.newline();

        // PowerShell 永久配置
        self.list_item("🔷", "PowerShell 永久配置（添加到 $PROFILE）:");
        println!("    $env:GOROOT = \"{}\"", go_root.display());
        println!("    $env:PATH = \"{};$env:PATH\"", bin_path.display());
        self.newline();

        // CMD 配置
        self.list_item("🔶", "命令提示符(CMD) 临时配置:");
        println!("    set GOROOT={}", go_root.display());
        println!("    set PATH={};%PATH%", bin_path.display());
        self.newline();

        // 系统环境变量配置
        self.list_item("⚙️", "系统环境变量配置（推荐）:");
        self.hint("   1. 右键'此电脑' → 属性 → 高级系统设置");
        self.hint("   2. 点击'环境变量'按钮");
        self.hint(&format!("   3. 新建 GOROOT = {}", go_root.display()));
        self.hint(&format!("   4. 编辑 PATH，添加 {}", bin_path.display()));
        self.hint("   5. 重启终端生效");
    }

    /// 显示 Unix 系（Linux/macOS）环境变量配置说明
    fn show_unix_env_setup(
        &self,
        bin_path: &std::path::Path,
        go_root: &std::path::Path,
        version: &str,
    ) {
        self.info(&format!("已切换到 Go {version}，以下是环境变量配置说明："));
        self.newline();

        // 临时配置
        self.list_item("🟢", "当前会话临时配置:");
        println!("    export GOROOT=\"{}\"", go_root.display());
        println!("    export PATH=\"{}:$PATH\"", bin_path.display());
        self.newline();

        // 检测用户的shell类型
        let shell = std::env::var("SHELL").unwrap_or_default();
        let (shell_name, config_file) = if shell.contains("zsh") {
            ("Zsh", "~/.zshrc")
        } else if shell.contains("fish") {
            ("Fish", "~/.config/fish/config.fish")
        } else if shell.contains("nu") {
            ("NuShell", "~/.config/nushell/config.nu")
        } else {
            ("Bash", "~/.bashrc 或 ~/.bash_profile")
        };

        // 永久配置
        self.list_item("🟢", &format!("{shell_name} 永久配置（添加到 {config_file}）:"));

        if shell.contains("fish") {
            // Fish shell 语法
            println!("    set -gx GOROOT \"{}\"", go_root.display());
            println!("    set -gx PATH \"{}\" $PATH", bin_path.display());
        } else if shell.contains("nu") {
            // NuShell 语法
            println!("    $env.GOROOT = \"{}\"", go_root.display());
            println!("    $env.PATH = ($env.PATH | prepend \"{}\")", bin_path.display());
        } else {
            // Bash/Zsh 语法
            println!("    export GOROOT=\"{}\"", go_root.display());
            println!("    export PATH=\"{}:$PATH\"", bin_path.display());
        }
        self.newline();

        // 快速应用配置的说明
        self.list_item("⚡", "立即应用配置:");
        if shell.contains("fish") {
            self.hint(&format!("   source {config_file}"));
        } else if shell.contains("nu") {
            self.hint("   重启 NuShell 或重新加载配置");
        } else {
            self.hint(&format!("   source {config_file}"));
        }

        // macOS 特殊说明
        if cfg!(target_os = "macos") {
            self.newline();
            self.list_item("🍎", "macOS 用户注意:");
            self.hint("   如果使用 Terminal.app，配置文件可能是 ~/.bash_profile");
            self.hint("   如果使用 iTerm2 + Zsh，配置文件是 ~/.zshrc");
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

    // Hash verification messages
    #[allow(dead_code)]
    #[must_use]
    pub fn verifying_checksum() -> String {
        format!("{} Verifying file integrity...", style("🔍").cyan())
    }

    #[allow(dead_code)]
    #[must_use]
    pub fn checksum_verification_passed() -> String {
        format!("{} File integrity verification passed", style("✓").green())
    }

    #[allow(dead_code)]
    #[must_use]
    pub fn checksum_verification_failed(expected: &str, actual: &str) -> String {
        format!(
            "{} File integrity verification failed!\n{}: {}\n{}: {}",
            style("✗").red(),
            style("Expected").yellow(),
            style(expected).dim(),
            style("Actual").yellow(),
            style(actual).dim()
        )
    }

    #[allow(dead_code)]
    #[must_use]
    pub fn corrupted_file_removed(path: &str) -> String {
        format!("{} Removed corrupted file: {}", style("🗑️").yellow(), style(path).dim())
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
