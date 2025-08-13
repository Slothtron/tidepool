//! Progress indication module for tidepool-gvm
//!
//! This module provides comprehensive progress tracking and display functionality using
//! the indicatif library. It supports download progress, installation steps, spinners,
//! and multi-operation coordination.

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::sync::Arc;
use std::time::Duration;

/// Progress manager that coordinates multiple progress indicators
pub struct ProgressManager {
    multi_progress: Arc<MultiProgress>,
}

impl ProgressManager {
    /// Create a new progress manager
    pub fn new() -> Self {
        Self { multi_progress: Arc::new(MultiProgress::new()) }
    }

    /// Create a download progress bar with percentage and formatted size display
    pub fn new_download_bar(&self, total_size: u64) -> ProgressBar {
        let bar = self.multi_progress.add(ProgressBar::new(total_size));
        bar.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} {msg}")
                .unwrap()
                .progress_chars("█▉▊▋▌▍▎▏ "),
        );

        let total_formatted = format_file_size(total_size);
        bar.set_message(format!("0% (0/{total_formatted})"));
        bar.enable_steady_tick(Duration::from_millis(100));
        bar
    }

    /// Create an installation progress bar showing step-by-step progress
    pub fn new_install_bar(&self, total_steps: u64) -> ProgressBar {
        let bar = self.multi_progress.add(ProgressBar::new(total_steps));
        bar.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{msg} {spinner:.green} [{elapsed_precise}] [{bar:30.cyan/blue}] {pos}/{len}",
                )
                .unwrap()
                .progress_chars("█▉▊▋▌▍▎▏ "),
        );
        bar.set_message("Installing");
        bar
    }

    /// Create a spinner for unknown duration tasks
    pub fn new_spinner(&self, message: &str) -> ProgressBar {
        let spinner = self.multi_progress.add(ProgressBar::new_spinner());
        spinner.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
        );
        spinner.set_message(message.to_string());
        spinner.enable_steady_tick(Duration::from_millis(80));
        spinner
    }

    /// Create a multi-step progress bar with predefined steps
    pub fn new_multi_step_bar(&self, steps: &[&str]) -> ProgressBar {
        let bar = self.multi_progress.add(ProgressBar::new(steps.len() as u64));
        bar.set_style(
            ProgressStyle::default_bar()
                .template("{msg} {spinner:.green} [{elapsed_precise}] [{bar:25.cyan/blue}] {pos}/{len} - {wide_msg}")
                .unwrap()
                .progress_chars("█▉▊▋▌▍▎▏ ")
        );
        bar.set_message("Processing");
        bar
    }

    /// Get reference to the underlying MultiProgress for advanced usage
    pub fn multi_progress(&self) -> &MultiProgress {
        &self.multi_progress
    }

    /// Join all progress bars (wait for completion)
    pub fn join(&self) -> std::io::Result<()> {
        self.multi_progress.clear()
    }
}

impl Default for ProgressManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Enhanced progress reporter with additional functionality
#[derive(Debug, Clone)]
pub struct ProgressReporter {
    /// Progress bar instance
    progress_bar: ProgressBar,
}

impl ProgressReporter {
    /// Create new progress reporter from progress bar
    pub fn new(progress_bar: ProgressBar) -> Self {
        Self { progress_bar }
    }

    /// Update progress with current and total values
    pub fn update(&self, current: u64, total: u64) {
        if total > 0 {
            self.progress_bar.set_length(total);
        }
        self.progress_bar.set_position(current);

        // Update the message to show formatted sizes with percentage
        if total > 0 {
            let current_formatted = format_file_size(current);
            let total_formatted = format_file_size(total);
            let percent = (current * 100) / total;
            self.progress_bar
                .set_message(format!("{}% ({}/{})", percent, current_formatted, total_formatted));
        }
    }

    /// Update progress with speed information
    pub fn update_with_speed(&self, downloaded: u64, total: u64, speed: f64) {
        self.update(downloaded, total);
        if speed > 0.0 {
            let _speed_str = format_bytes_per_sec(speed);
            // Speed is automatically calculated by indicatif when using bytes
        }
    }

    /// Set custom message
    pub fn set_message(&self, msg: &str) {
        self.progress_bar.set_message(msg.to_string());
    }

    /// Finish progress with result
    pub fn finish_with_result(&self, success: bool, message: &str) {
        if success {
            self.progress_bar.finish_with_message(format!("✅ {}", message));
        } else {
            self.progress_bar.finish_with_message(format!("❌ {}", message));
        }
    }

    /// Abandon progress (for errors)
    pub fn abandon(&self, message: &str) {
        self.progress_bar.abandon_with_message(format!("❌ {}", message));
    }

    /// Increment progress by specified amount
    pub fn inc(&self, delta: u64) {
        self.progress_bar.inc(delta);
    }

    /// Set progress to specific position
    pub fn set_position(&self, position: u64) {
        self.progress_bar.set_position(position);
    }
}

/// Helper function to format file sizes in human-readable format
fn format_file_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut value = bytes as f64;
    let mut unit_index = 0;

    while value >= 1024.0 && unit_index < UNITS.len() - 1 {
        value /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1} {}", value, UNITS[unit_index])
    }
}

/// Helper function to format bytes per second
fn format_bytes_per_sec(bytes_per_sec: f64) -> String {
    const UNITS: &[&str] = &["B/s", "KB/s", "MB/s", "GB/s"];
    let mut value = bytes_per_sec;
    let mut unit_index = 0;

    while value >= 1024.0 && unit_index < UNITS.len() - 1 {
        value /= 1024.0;
        unit_index += 1;
    }

    format!("{:.1} {}", value, UNITS[unit_index])
}

/// Installation step enumeration for multi-step progress
#[derive(Debug, Clone)]
pub enum InstallStep {
    Download,
    Verify,
    Extract,
    Configure,
    Finalize,
}

impl InstallStep {
    /// Get human-readable description
    pub fn description(&self) -> &'static str {
        match self {
            InstallStep::Download => "Downloading Go archive",
            InstallStep::Verify => "Verifying download integrity",
            InstallStep::Extract => "Extracting files",
            InstallStep::Configure => "Configuring installation",
            InstallStep::Finalize => "Finalizing setup",
        }
    }

    /// Get all steps in order
    pub fn all_steps() -> Vec<InstallStep> {
        vec![
            InstallStep::Download,
            InstallStep::Verify,
            InstallStep::Extract,
            InstallStep::Configure,
            InstallStep::Finalize,
        ]
    }
}
