//! Enhanced Progress Management System
//!
//! This module provides comprehensive progress tracking for complex installation operations.
//! It supports multi-step processes with detailed progress reporting, estimated time remaining,
//! and rich user feedback.

pub mod types;
pub mod manager;
pub mod tracker;
pub mod reporter;
pub mod simple_install;

// Phase 3 modules - enabled
pub mod tasks;
pub mod planner;
pub mod executor;
pub mod validator;
pub mod enhanced_install;

// Re-export main types
pub use types::{
    InstallStep, InstallProgress, DownloadProgress, TaskProgress,
    ProgressWeight, EstimatedTime, ProgressState, TaskId
};
pub use manager::{EnhancedProgressManager, TaskProgressHandle};
pub use tracker::TaskProgressTracker;
pub use reporter::ProgressReporter;
pub use simple_install::install_with_fallback;

// Phase 3 exports - enabled
pub use tasks::{
    DetailedInstallPlan, InstallTask, SubTask, TaskAction, ValidationCheck,
    ArchiveFormat, PlatformInfo, RetryPolicy, ErrorType
};
pub use planner::InstallPlanner;
pub use executor::TaskExecutor;
pub use validator::{ValidationEngine, ValidationReport, RuntimeValidationContext};
pub use enhanced_install::EnhancedInstallationCoordinator;

use std::time::Duration;

/// Progress tracking utilities
pub struct ProgressUtils;

impl ProgressUtils {
    /// Calculate estimated time remaining based on current progress and elapsed time
    pub fn estimate_remaining_time(
        current_progress: f32,
        elapsed: Duration,
    ) -> Option<Duration> {
        if current_progress <= 0.0 || current_progress >= 1.0 {
            return None;
        }
        
        let total_estimated = elapsed.as_secs_f64() / current_progress as f64;
        let remaining = total_estimated - elapsed.as_secs_f64();
        
        if remaining > 0.0 {
            Some(Duration::from_secs_f64(remaining))
        } else {
            None
        }
    }
    
    /// Format bytes per second as human readable string
    pub fn format_bytes_per_sec(bytes_per_sec: f64) -> String {
        const UNITS: &[&str] = &["B/s", "KB/s", "MB/s", "GB/s"];
        let mut size = bytes_per_sec;
        let mut unit_index = 0;
        
        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }
        
        if unit_index == 0 {
            format!("{:.0} {}", size, UNITS[unit_index])
        } else {
            format!("{:.1} {}", size, UNITS[unit_index])
        }
    }
    
    /// Format bytes as human readable string
    pub fn format_bytes(bytes: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        let mut size = bytes as f64;
        let mut unit_index = 0;
        
        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }
        
        if unit_index == 0 {
            format!("{:.0} {}", size, UNITS[unit_index])
        } else {
            format!("{:.1} {}", size, UNITS[unit_index])
        }
    }
    
    /// Format duration as human readable string
    pub fn format_duration(duration: Duration) -> String {
        let total_secs = duration.as_secs();
        
        if total_secs < 60 {
            format!("{}s", total_secs)
        } else if total_secs < 3600 {
            let minutes = total_secs / 60;
            let seconds = total_secs % 60;
            format!("{}m{}s", minutes, seconds)
        } else {
            let hours = total_secs / 3600;
            let minutes = (total_secs % 3600) / 60;
            format!("{}h{}m", hours, minutes)
        }
    }
}

/// Common result type for progress operations
pub type ProgressResult<T> = anyhow::Result<T>;
