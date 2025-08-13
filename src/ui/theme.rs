//! Theme module for tidepool-gvm
//!
//! This module provides consistent styling and theming across all UI components.
//! It defines color schemes, progress bar styles, and other visual elements
//! to ensure a cohesive user experience.

use console::{Color, Style};
use indicatif::ProgressStyle;

/// Color scheme definition for the GVM theme
#[derive(Debug, Clone)]
pub struct ColorScheme {
    pub primary: Color,
    pub secondary: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub info: Color,
    pub muted: Color,
    pub accent: Color,
}

impl ColorScheme {
    /// Default color scheme with cyan primary
    pub fn default() -> Self {
        Self {
            primary: Color::Cyan,
            secondary: Color::Blue,
            success: Color::Green,
            warning: Color::Yellow,
            error: Color::Red,
            info: Color::Blue,
            muted: Color::Color256(8), // Dark gray
            accent: Color::Magenta,
        }
    }

    /// Dark theme optimized for dark terminals
    pub fn dark() -> Self {
        Self {
            primary: Color::Color256(39),  // Bright cyan
            secondary: Color::Color256(33), // Bright blue
            success: Color::Color256(46),   // Bright green
            warning: Color::Color256(226),  // Bright yellow
            error: Color::Color256(196),    // Bright red
            info: Color::Color256(75),      // Light blue
            muted: Color::Color256(102),    // Medium gray
            accent: Color::Color256(207),   // Bright magenta
        }
    }

    /// Light theme optimized for light terminals
    pub fn light() -> Self {
        Self {
            primary: Color::Color256(31),   // Dark cyan
            secondary: Color::Color256(25), // Dark blue
            success: Color::Color256(28),   // Dark green
            warning: Color::Color256(130),  // Dark orange
            error: Color::Color256(124),    // Dark red
            info: Color::Color256(25),      // Dark blue
            muted: Color::Color256(243),    // Light gray
            accent: Color::Color256(125),   // Dark magenta
        }
    }

    /// Monochrome theme for terminals without color support
    pub fn monochrome() -> Self {
        Self {
            primary: Color::White,
            secondary: Color::White,
            success: Color::White,
            warning: Color::White,
            error: Color::White,
            info: Color::White,
            muted: Color::Color256(8),
            accent: Color::White,
        }
    }
}

/// Main theme configuration for GVM
#[derive(Debug, Clone)]
pub struct GvmTheme {
    pub colors: ColorScheme,
    pub icons: IconSet,
    pub styles: StyleSet,
}

impl GvmTheme {
    /// Create default theme
    pub fn default() -> Self {
        Self {
            colors: ColorScheme::default(),
            icons: IconSet::unicode(),
            styles: StyleSet::default(),
        }
    }

    /// Create dark theme
    pub fn dark() -> Self {
        Self {
            colors: ColorScheme::dark(),
            icons: IconSet::unicode(),
            styles: StyleSet::default(),
        }
    }

    /// Create light theme
    pub fn light() -> Self {
        Self {
            colors: ColorScheme::light(),
            icons: IconSet::unicode(),
            styles: StyleSet::default(),
        }
    }

    /// Create minimal theme for limited terminals
    pub fn minimal() -> Self {
        Self {
            colors: ColorScheme::monochrome(),
            icons: IconSet::ascii(),
            styles: StyleSet::minimal(),
        }
    }

    /// Auto-detect appropriate theme based on environment
    pub fn auto() -> Self {
        // Check environment variables and terminal capabilities
        if std::env::var("NO_COLOR").is_ok() || std::env::var("TERM").unwrap_or_default() == "dumb" {
            Self::minimal()
        } else if Self::is_dark_terminal() {
            Self::dark()
        } else {
            Self::default()
        }
    }

    /// Detect if terminal uses dark background
    fn is_dark_terminal() -> bool {
        // Simple heuristic based on common terminal names
        let term = std::env::var("TERM").unwrap_or_default().to_lowercase();
        let term_program = std::env::var("TERM_PROGRAM").unwrap_or_default().to_lowercase();

        term.contains("dark")
            || term_program.contains("iterm")
            || term_program.contains("vscode")
            || std::env::var("COLORFGBG").unwrap_or_default().ends_with(";0")
    }
}

/// Icon set for different terminal capabilities
#[derive(Debug, Clone)]
pub struct IconSet {
    pub success: &'static str,
    pub error: &'static str,
    pub warning: &'static str,
    pub info: &'static str,
    pub package: &'static str,
    pub download: &'static str,
    pub install: &'static str,
    pub current: &'static str,
    pub arrow_right: &'static str,
    pub spinner: &'static [&'static str],
}

impl IconSet {
    /// Unicode icons for modern terminals
    pub fn unicode() -> Self {
        Self {
            success: "✅",
            error: "❌",
            warning: "⚠️",
            info: "ℹ️",
            package: "📦",
            download: "⬇️",
            install: "⚙️",
            current: "⭐",
            arrow_right: "➡️",
            spinner: &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
        }
    }

    /// ASCII icons for limited terminals
    pub fn ascii() -> Self {
        Self {
            success: "[✓]",
            error: "[✗]",
            warning: "[!]",
            info: "[i]",
            package: "[*]",
            download: "[↓]",
            install: "[+]",
            current: "[>]",
            arrow_right: "->",
            spinner: &["|", "/", "-", "\\"],
        }
    }

    /// Minimal icons using only basic characters
    pub fn minimal() -> Self {
        Self {
            success: "[OK]",
            error: "[ERR]",
            warning: "[WARN]",
            info: "[INFO]",
            package: "[PKG]",
            download: "[DL]",
            install: "[INST]",
            current: "[CUR]",
            arrow_right: "=>",
            spinner: &[".", "..", "...", "...."],
        }
    }
}

/// Style definitions for consistent formatting
#[derive(Debug, Clone)]
pub struct StyleSet {
    pub header: Style,
    pub subheader: Style,
    pub success: Style,
    pub error: Style,
    pub warning: Style,
    pub info: Style,
    pub muted: Style,
    pub highlight: Style,
    pub code: Style,
}

impl StyleSet {
    /// Default style set
    pub fn default() -> Self {
        Self {
            header: Style::new().bold().cyan(),
            subheader: Style::new().bold().blue(),
            success: Style::new().green(),
            error: Style::new().red().bold(),
            warning: Style::new().yellow(),
            info: Style::new().blue(),
            muted: Style::new().dim(),
            highlight: Style::new().bold().cyan(),
            code: Style::new().on_black().white(),
        }
    }

    /// Minimal style set without colors
    pub fn minimal() -> Self {
        Self {
            header: Style::new().bold(),
            subheader: Style::new().bold(),
            success: Style::new(),
            error: Style::new().bold(),
            warning: Style::new(),
            info: Style::new(),
            muted: Style::new().dim(),
            highlight: Style::new().bold(),
            code: Style::new().bold(),
        }
    }
}

/// Progress bar style templates
pub struct ProgressStyles;

impl ProgressStyles {
    /// Download progress bar style
    pub fn download() -> ProgressStyle {
        ProgressStyle::default_bar()
            .template("{msg} {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
            .unwrap()
            .progress_chars("█▉▊▋▌▍▎▏ ")
    }

    /// Installation progress bar style
    pub fn install() -> ProgressStyle {
        ProgressStyle::default_bar()
            .template("{msg} {spinner:.green} [{elapsed_precise}] [{bar:30.cyan/blue}] {pos}/{len}")
            .unwrap()
            .progress_chars("█▉▊▋▌▍▎▏ ")
    }

    /// Spinner style for unknown duration tasks
    pub fn spinner() -> ProgressStyle {
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
    }

    /// Multi-step progress style
    pub fn multi_step() -> ProgressStyle {
        ProgressStyle::default_bar()
            .template("{msg} {spinner:.green} [{elapsed_precise}] [{bar:25.cyan/blue}] {pos}/{len} - {wide_msg}")
            .unwrap()
            .progress_chars("█▉▊▋▌▍▎▏ ")
    }

    /// Minimal progress bar for limited terminals
    pub fn minimal() -> ProgressStyle {
        ProgressStyle::default_bar()
            .template("[{bar:20}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("=> ")
    }
}

/// Theme manager for handling theme switching and persistence
pub struct ThemeManager {
    current_theme: GvmTheme,
}

impl ThemeManager {
    /// Create new theme manager with default theme
    pub fn new() -> Self {
        Self {
            current_theme: GvmTheme::auto(),
        }
    }

    /// Create theme manager with specific theme
    pub fn with_theme(theme: GvmTheme) -> Self {
        Self {
            current_theme: theme,
        }
    }

    /// Get current theme
    pub fn current(&self) -> &GvmTheme {
        &self.current_theme
    }

    /// Switch to a different theme
    pub fn switch_theme(&mut self, theme: GvmTheme) {
        self.current_theme = theme;
    }

    /// Apply theme colors to a style
    pub fn apply_color(&self, base_style: Style, color_type: ColorType) -> Style {
        match color_type {
            ColorType::Primary => base_style.fg(self.current_theme.colors.primary),
            ColorType::Secondary => base_style.fg(self.current_theme.colors.secondary),
            ColorType::Success => base_style.fg(self.current_theme.colors.success),
            ColorType::Warning => base_style.fg(self.current_theme.colors.warning),
            ColorType::Error => base_style.fg(self.current_theme.colors.error),
            ColorType::Info => base_style.fg(self.current_theme.colors.info),
            ColorType::Muted => base_style.fg(self.current_theme.colors.muted),
            ColorType::Accent => base_style.fg(self.current_theme.colors.accent),
        }
    }

    /// Get themed progress style
    pub fn progress_style(&self, style_type: ProgressStyleType) -> ProgressStyle {
        match style_type {
            ProgressStyleType::Download => ProgressStyles::download(),
            ProgressStyleType::Install => ProgressStyles::install(),
            ProgressStyleType::Spinner => ProgressStyles::spinner(),
            ProgressStyleType::MultiStep => ProgressStyles::multi_step(),
            ProgressStyleType::Minimal => ProgressStyles::minimal(),
        }
    }
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Color type enumeration for theme application
#[derive(Debug, Clone, Copy)]
pub enum ColorType {
    Primary,
    Secondary,
    Success,
    Warning,
    Error,
    Info,
    Muted,
    Accent,
}

/// Progress style type enumeration
#[derive(Debug, Clone, Copy)]
pub enum ProgressStyleType {
    Download,
    Install,
    Spinner,
    MultiStep,
    Minimal,
}

/// Utility functions for theme detection and management
pub mod utils {
    use super::*;

    /// Detect if terminal supports colors
    pub fn supports_color() -> bool {
        if std::env::var("NO_COLOR").is_ok() {
            return false;
        }

        if let Ok(term) = std::env::var("TERM") {
            !term.is_empty() && term != "dumb"
        } else {
            false
        }
    }

    /// Detect if terminal supports unicode
    pub fn supports_unicode() -> bool {
        if let Ok(lang) = std::env::var("LANG") {
            lang.to_uppercase().contains("UTF")
        } else if let Ok(lc_all) = std::env::var("LC_ALL") {
            lc_all.to_uppercase().contains("UTF")
        } else {
            // Default to unicode support on Windows and modern systems
            cfg!(windows) || supports_color()
        }
    }

    /// Get appropriate theme based on terminal capabilities
    pub fn detect_theme() -> GvmTheme {
        if !supports_color() {
            GvmTheme::minimal()
        } else if GvmTheme::is_dark_terminal() {
            GvmTheme::dark()
        } else {
            GvmTheme::default()
        }
    }

    /// Format text with theme-aware styling
    pub fn themed_text(text: &str, color_type: ColorType, theme: &GvmTheme) -> String {
        let color = match color_type {
            ColorType::Primary => theme.colors.primary,
            ColorType::Secondary => theme.colors.secondary,
            ColorType::Success => theme.colors.success,
            ColorType::Warning => theme.colors.warning,
            ColorType::Error => theme.colors.error,
            ColorType::Info => theme.colors.info,
            ColorType::Muted => theme.colors.muted,
            ColorType::Accent => theme.colors.accent,
        };

        Style::new().fg(color).apply_to(text).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_scheme_creation() {
        let default_scheme = ColorScheme::default();
        assert_eq!(default_scheme.primary, Color::Cyan);
        assert_eq!(default_scheme.success, Color::Green);
        assert_eq!(default_scheme.error, Color::Red);
    }

    #[test]
    fn test_theme_manager() {
        let mut manager = ThemeManager::new();
        assert!(matches!(manager.current().colors.primary, Color::Cyan | Color::Color256(_)));

        manager.switch_theme(GvmTheme::dark());
        // Theme should be switched to dark
    }

    #[test]
    fn test_icon_sets() {
        let unicode = IconSet::unicode();
        let ascii = IconSet::ascii();
        let minimal = IconSet::minimal();

        assert_eq!(unicode.success, "✅");
        assert_eq!(ascii.success, "[✓]");
        assert_eq!(minimal.success, "[OK]");
    }

    #[test]
    fn test_utils() {
        // These tests depend on environment, so we just test they don't panic
        let _ = utils::supports_color();
        let _ = utils::supports_unicode();
        let _ = utils::detect_theme();
    }
}
