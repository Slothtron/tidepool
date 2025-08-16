//! Simplified UI system
//!
//! Provides a clean, cross-platform compatible user interface,
//! using the colored crate for cross-platform color support.

use colored::*;
use std::io::{self, Write};

/// Simplified UI manager
pub struct SimpleUI {
    use_colors: bool,
}

impl SimpleUI {
    /// Creates a new UI instance
    pub fn new() -> Self {
        let use_colors = Self::should_use_colors();
        Self { use_colors }
    }

    /// Detects if colors should be used
    fn should_use_colors() -> bool {
        // colored crate handles this automatically, but we can add custom logic
        std::env::var("NO_COLOR").is_err() && std::env::var("TERM").unwrap_or_default() != "dumb"
    }

    /// Displays a success message
    pub fn success(&self, message: &str) {
        if self.use_colors {
            println!("{} {}", "[OK]".green(), message);
        } else {
            println!("[OK] {message}");
        }
    }

    /// Displays an error message
    pub fn error(&self, message: &str) {
        if self.use_colors {
            println!("{} {}", "[ERROR]".red(), message);
        } else {
            println!("[ERROR] {message}");
        }
    }

    /// Displays a warning message
    pub fn warning(&self, message: &str) {
        if self.use_colors {
            println!("{} {}", "[WARN]".yellow(), message);
        } else {
            println!("[WARN] {message}");
        }
    }

    /// Displays an informational message
    pub fn info(&self, message: &str) {
        if self.use_colors {
            println!("{} {}", "[INFO]".blue(), message);
        } else {
            println!("[INFO] {message}");
        }
    }

    /// Displays a hint message
    pub fn hint(&self, message: &str) {
        if self.use_colors {
            println!("{} {}", "[TIP]".cyan(), message);
        } else {
            println!("[TIP] {message}");
        }
    }

    /// Displays a title
    pub fn title(&self, text: &str) {
        println!();
        println!("{}", "=".repeat(60));
        println!("{text}");
        println!("{}", "=".repeat(60));
        println!();
    }

    /// Displays a section header
    pub fn section(&self, text: &str) {
        println!();
        if self.use_colors {
            println!("{}", format!("> {text}").cyan());
        } else {
            println!("> {text}");
        }
        println!("{}", "-".repeat(text.len() + 2));
    }

    /// Displays a list item
    pub fn list_item(&self, text: &str, is_current: bool) {
        if is_current {
            if self.use_colors {
                println!("  {} {}", format!("* {text}").green(), "(active)".dimmed());
            } else {
                println!("  * {text} (active)");
            }
        } else {
            println!("  - {text}");
        }
    }

    /// Displays a key-value pair
    pub fn key_value(&self, key: &str, value: &str) {
        if self.use_colors {
            println!("  {}: {}", key.dimmed(), value);
        } else {
            println!("  {key}: {value}");
        }
    }

    /// Displays a colored key-value pair
    pub fn key_value_colored(&self, key: &str, value: &str, color: &str) {
        if self.use_colors {
            let colored_value = match color {
                "green" => value.green().to_string(),
                "red" => value.red().to_string(),
                "yellow" => value.yellow().to_string(),
                "blue" => value.blue().to_string(),
                "cyan" => value.cyan().to_string(),
                "dim" => value.dimmed().to_string(),
                _ => value.to_string(),
            };
            println!("  {}: {}", key.dimmed(), colored_value);
        } else {
            println!("  {key}: {value}");
        }
    }

    /// Displays progress information
    pub fn progress(&self, current: usize, total: usize, description: &str) {
        if self.use_colors {
            println!(
                "[{}/{}] {}",
                current.to_string().cyan(),
                total.to_string().cyan(),
                description
            );
        } else {
            println!("[{current}/{total}] {description}");
        }
    }

    /// Displays a concise status message
    pub fn status(&self, message: &str) {
        if self.use_colors {
            println!("{}", message.dimmed());
        } else {
            println!("{message}");
        }
    }

    /// Displays a separator line
    pub fn separator(&self) {
        println!("{}", "-".repeat(50));
    }

    /// Displays a newline
    pub fn newline(&self) {
        println!();
    }

    /// Displays a suggestion
    pub fn suggest(&self, message: &str) {
        if self.use_colors {
            println!("{} Suggestion: {}", "->".cyan(), message);
        } else {
            println!("-> Suggestion: {message}");
        }
    }
}

impl Default for SimpleUI {
    fn default() -> Self {
        Self::new()
    }
}

/// A simple progress bar
pub struct SimpleProgressBar {
    label: String,
    use_colors: bool,
}

impl SimpleProgressBar {
    pub fn new(label: String) -> Self {
        let use_colors = SimpleUI::should_use_colors();
        Self { label, use_colors }
    }

    /// Updates the progress
    pub fn update(&self, percent: f64, message: Option<&str>) {
        let bar_width = 30;
        let filled = (percent * bar_width as f64) as usize;
        let empty = bar_width - filled;

        let bar = if self.use_colors {
            format!("{}{}", "=".repeat(filled).green(), " ".repeat(empty))
        } else {
            format!("{}{}", "=".repeat(filled), " ".repeat(empty))
        };

        let display_message = message.unwrap_or("");

        if self.use_colors {
            print!(
                "\r{} [{}] {} {}",
                self.label.cyan(),
                bar,
                format!("{:.1}%", percent * 100.0).bold(),
                display_message
            );
        } else {
            print!("\r{} [{}] {:.1}% {}", self.label, bar, percent * 100.0, display_message);
        }

        io::stdout().flush().ok();
    }

    /// Finishes the progress
    pub fn finish(&self, message: &str) {
        println!();
        if self.use_colors {
            println!("{} {}", "[OK]".green(), message);
        } else {
            println!("[OK] {message}");
        }
    }

    /// Fails the progress
    pub fn fail(&self, message: &str) {
        println!();
        if self.use_colors {
            println!("{} {}", "[ERROR]".red(), message);
        } else {
            println!("[ERROR] {message}");
        }
    }
}

/// Formats a file size in a human-readable format
pub fn format_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

/// Formats a duration in a human-readable format
pub fn format_duration(seconds: u64) -> String {
    if seconds < 60 {
        format!("{seconds}s")
    } else if seconds < 3600 {
        let minutes = seconds / 60;
        let secs = seconds % 60;
        format!("{minutes}m{secs}s")
    } else {
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;
        format!("{hours}h{minutes}m")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(1024), "1.0 KB");
        assert_eq!(format_size(1048576), "1.0 MB");
        assert_eq!(format_size(500), "500 B");
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(30), "30s");
        assert_eq!(format_duration(90), "1m30s");
        assert_eq!(format_duration(3661), "1h1m");
    }
}
