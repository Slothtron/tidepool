//! Basic progress display system
//!
//! Inspired by the simple progress display style of tools like Scoop.

use std::io::{self, Write};

/// Basic progress indicator
#[derive(Clone)]
pub struct BasicProgress {
    label: String,
    use_colors: bool,
}

impl BasicProgress {
    /// Creates a new progress indicator
    pub fn new(label: String) -> Self {
        let use_colors = Self::should_use_colors();
        Self { label, use_colors }
    }

    /// Detects if colors should be used
    fn should_use_colors() -> bool {
        std::env::var("NO_COLOR").is_err() && std::env::var("TERM").unwrap_or_default() != "dumb"
    }

    /// Displays progress (Scoop style)
    pub fn show(&self, percent: f64, info: Option<&str>) {
        let bar_width = 50;
        let filled = (percent * bar_width as f64) as usize;
        let empty = bar_width - filled;

        // Build progress bar with simple characters
        let bar = format!("{}{}", "=".repeat(filled), " ".repeat(empty));

        let info_text = info.unwrap_or("");

        // Both branches are the same, merged into one
        print!("\r{} [{}] {:.0}% {}", self.label, bar, percent * 100.0, info_text);

        io::stdout().flush().ok();
    }

    /// Displays download progress
    pub fn show_download(&self, downloaded: u64, total: u64) {
        let percent = if total > 0 { downloaded as f64 / total as f64 } else { 0.0 };
        let info = if total > 0 {
            format!("{}/{}", format_size(downloaded), format_size(total))
        } else {
            format_size(downloaded)
        };
        self.show(percent, Some(&info));
    }

    /// Finalizes the progress display as done
    pub fn done(&self, message: &str) {
        println!();
        if self.use_colors {
            println!("{message} ... \x1b[32mdone\x1b[0m.");
        } else {
            println!("{message} ... done.");
        }
    }

    /// Finalizes the progress display as failed
    pub fn failed(&self, message: &str) {
        println!();
        if self.use_colors {
            println!("{message} ... \x1b[31mfailed\x1b[0m.");
        } else {
            println!("{message} ... failed.");
        }
    }
}

/// Installation steps manager
pub struct InstallSteps {
    use_colors: bool,
}

impl Default for InstallSteps {
    fn default() -> Self {
        Self::new()
    }
}

impl InstallSteps {
    pub fn new() -> Self {
        let use_colors = std::env::var("NO_COLOR").is_err()
            && std::env::var("TERM").unwrap_or_default() != "dumb";
        Self { use_colors }
    }

    /// Displays the start of the installation
    pub fn start(&self, version: &str) {
        if self.use_colors {
            println!("\x1b[36mInstalling 'go' ({version}) [64bit] from 'workspace'\x1b[0m");
        } else {
            println!("Installing 'go' ({version}) [64bit] from 'workspace'");
        }
    }

    /// Displays step information
    pub fn info(&self, message: &str) {
        if self.use_colors {
            println!("\x1b[90m{message}\x1b[0m");
        } else {
            println!("{message}");
        }
    }

    /// Displays a warning message
    pub fn warn(&self, message: &str) {
        if self.use_colors {
            println!("\x1b[33mWARN\x1b[0m {message}");
        } else {
            println!("WARN {message}");
        }
    }

    /// Displays the completion message
    pub fn complete(&self, version: &str) {
        if self.use_colors {
            println!("\x1b[32m'go' ({version}) was installed successfully!\x1b[0m");
        } else {
            println!("'go' ({version}) was installed successfully!");
        }
    }
}

/// Formats a file size (simplified version)
fn format_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{bytes} {}", UNITS[unit_index])
    } else {
        format!("{size:.1}{}", UNITS[unit_index])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(1024), "1.0KB");
        assert_eq!(format_size(1048576), "1.0MB");
        assert_eq!(format_size(500), "500 B");
    }

    #[test]
    fn test_basic_progress_creation() {
        let progress = BasicProgress::new("Testing".to_string());
        assert_eq!(progress.label, "Testing");
    }
}
