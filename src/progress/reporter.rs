//! Progress Reporter
//!
//! Provides formatted reporting of installation progress for user display.

use super::types::{InstallProgress, TaskProgress, TaskId, ProgressState};
use super::ProgressUtils;
use std::collections::HashMap;
use std::time::Duration;

/// Formats and reports progress information
pub struct ProgressReporter;

impl ProgressReporter {
    /// Format overall installation progress as a summary string
    pub fn format_install_summary(progress: &InstallProgress) -> String {
        let percentage = (progress.current_progress * 100.0) as u8;
        let elapsed = ProgressUtils::format_duration(progress.elapsed());
        
        let eta_str = if let Some(eta) = progress.estimated_remaining {
            format!(" (ETA: {})", ProgressUtils::format_duration(eta))
        } else {
            String::new()
        };
        
        format!(
            "{}% complete - {} - Elapsed: {}{}",
            percentage,
            progress.current_step.description(),
            elapsed,
            eta_str
        )
    }
    
    /// Format task progress for display
    pub fn format_task_progress(task: &TaskProgress) -> String {
        let percentage = (task.progress * 100.0) as u8;
        let state_indicator = match task.state {
            ProgressState::NotStarted => "⏸",
            ProgressState::InProgress => "🔄",
            ProgressState::Completed => "✅",
            ProgressState::Failed { .. } => "❌",
            ProgressState::Cancelled => "⏹",
        };
        
        let subtask_info = if let Some(ref subtask) = task.current_subtask {
            format!(" - {}", subtask)
        } else {
            String::new()
        };
        
        format!(
            "{} {} ({}%){}", 
            state_indicator, 
            task.name, 
            percentage,
            subtask_info
        )
    }
    
    /// Generate a detailed progress report
    pub fn generate_detailed_report(
        install_progress: &InstallProgress,
        task_progress: &HashMap<TaskId, TaskProgress>,
    ) -> String {
        let mut report = String::new();
        
        // Overall progress
        report.push_str(&format!("=== Installation Progress ===\n"));
        report.push_str(&Self::format_install_summary(install_progress));
        report.push_str("\n\n");
        
        // Task breakdown
        if !task_progress.is_empty() {
            report.push_str("=== Task Progress ===\n");
            for (task_id, task) in task_progress {
                report.push_str(&format!("{}: {}\n", task_id, Self::format_task_progress(task)));
                
                if let ProgressState::Failed { ref error } = task.state {
                    report.push_str(&format!("  Error: {}\n", error));
                }
            }
        }
        
        report
    }
    
    /// Format download progress specifically
    pub fn format_download_progress(
        downloaded: u64,
        total: Option<u64>,
        speed: f64,
        eta: Option<Duration>,
    ) -> String {
        let downloaded_str = ProgressUtils::format_bytes(downloaded);
        let speed_str = ProgressUtils::format_bytes_per_sec(speed);
        
        if let Some(total_bytes) = total {
            let total_str = ProgressUtils::format_bytes(total_bytes);
            let percentage = (downloaded as f64 / total_bytes as f64) * 100.0;
            
            let eta_str = if let Some(eta_duration) = eta {
                format!(" (ETA: {})", ProgressUtils::format_duration(eta_duration))
            } else {
                String::new()
            };
            
            format!(
                "{}/{} ({:.1}%) at {}{}",
                downloaded_str, total_str, percentage, speed_str, eta_str
            )
        } else {
            format!("{} at {}", downloaded_str, speed_str)
        }
    }
    
    /// Format extraction progress
    pub fn format_extraction_progress(
        current_files: u64,
        total_files: Option<u64>,
        current_file: Option<&str>,
    ) -> String {
        let current_file_info = if let Some(file) = current_file {
            format!(" - {}", file)
        } else {
            String::new()
        };
        
        if let Some(total) = total_files {
            let percentage = (current_files as f64 / total as f64) * 100.0;
            format!(
                "Extracting {}/{} files ({:.1}%){}",
                current_files, total, percentage, current_file_info
            )
        } else {
            format!("Extracting {} files{}", current_files, current_file_info)
        }
    }
    
    /// Create a progress bar string representation
    pub fn create_progress_bar(progress: f32, width: usize) -> String {
        let filled_width = ((progress.clamp(0.0, 1.0) * width as f32) as usize).min(width);
        let empty_width = width - filled_width;
        
        format!(
            "[{}{}]",
            "=".repeat(filled_width),
            " ".repeat(empty_width)
        )
    }
    
    /// Format time duration in a human-friendly way
    pub fn format_elapsed_time(elapsed: Duration) -> String {
        ProgressUtils::format_duration(elapsed)
    }
}
