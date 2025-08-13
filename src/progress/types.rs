//! Progress Types
//!
//! Core types for enhanced progress tracking system

use std::time::{Duration, Instant};
use std::fmt;
use serde::{Deserialize, Serialize};

/// Unique identifier for installation tasks
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TaskId(pub String);

impl TaskId {
    pub fn new(id: &str) -> Self {
        Self(id.to_string())
    }
}

impl fmt::Display for TaskId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for TaskId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Installation step types with detailed information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InstallStep {
    /// Initial validation phase
    Validating,
    /// Checking local cache for existing files
    CheckingCache,
    /// Creating necessary directories
    CreatingDirectories,
    /// Downloading files from remote sources
    Downloading { 
        url: String, 
        size: Option<u64>,
        filename: String,
    },
    /// Extracting archive files
    Extracting { 
        archive: String,
        destination: String,
    },
    /// Installing components
    Installing { 
        component: String 
    },
    /// Configuring the installation
    Configuring,
    /// Verifying installation integrity
    Verifying,
    /// Finalizing installation
    Finalizing,
    /// Custom step with name and description
    Custom {
        name: String,
        description: String,
    },
}

impl InstallStep {
    /// Get a human-readable description of the step
    pub fn description(&self) -> String {
        match self {
            InstallStep::Validating => "Validating environment and requirements".to_string(),
            InstallStep::CheckingCache => "Checking local cache for existing files".to_string(),
            InstallStep::CreatingDirectories => "Creating installation directories".to_string(),
            InstallStep::Downloading { filename, .. } => {
                format!("Downloading {}", filename)
            },
            InstallStep::Extracting { archive, .. } => {
                format!("Extracting {}", archive)
            },
            InstallStep::Installing { component } => {
                format!("Installing {}", component)
            },
            InstallStep::Configuring => "Configuring installation".to_string(),
            InstallStep::Verifying => "Verifying installation integrity".to_string(),
            InstallStep::Finalizing => "Finalizing installation".to_string(),
            InstallStep::Custom { description, .. } => description.clone(),
        }
    }
    
    /// Get the step name
    pub fn name(&self) -> &str {
        match self {
            InstallStep::Validating => "Validating",
            InstallStep::CheckingCache => "Checking Cache",
            InstallStep::CreatingDirectories => "Creating Directories",
            InstallStep::Downloading { .. } => "Downloading",
            InstallStep::Extracting { .. } => "Extracting",
            InstallStep::Installing { .. } => "Installing",
            InstallStep::Configuring => "Configuring",
            InstallStep::Verifying => "Verifying",
            InstallStep::Finalizing => "Finalizing",
            InstallStep::Custom { name, .. } => name,
        }
    }
}

/// Overall installation progress information
#[derive(Debug, Clone)]
pub struct InstallProgress {
    /// Current step being executed
    pub current_step: InstallStep,
    /// Total number of steps
    pub total_steps: u8,
    /// Number of completed steps
    pub completed_steps: u8,
    /// Overall progress percentage (0.0 to 1.0)
    pub current_progress: f32,
    /// Estimated time remaining
    pub estimated_remaining: Option<Duration>,
    /// Download-specific progress information
    pub download_progress: Option<DownloadProgress>,
    /// When the installation started
    pub start_time: Instant,
    /// Additional step-specific details
    pub step_details: Option<String>,
}

impl InstallProgress {
    /// Create a new installation progress tracker
    pub fn new(total_steps: u8) -> Self {
        Self {
            current_step: InstallStep::Validating,
            total_steps,
            completed_steps: 0,
            current_progress: 0.0,
            estimated_remaining: None,
            download_progress: None,
            start_time: Instant::now(),
            step_details: None,
        }
    }
    
    /// Update the current step
    pub fn update_step(&mut self, step: InstallStep, details: Option<String>) {
        self.current_step = step;
        self.step_details = details;
    }
    
    /// Mark a step as completed
    pub fn complete_step(&mut self) {
        self.completed_steps += 1;
        self.current_progress = self.completed_steps as f32 / self.total_steps as f32;
        
        // Update estimated time remaining
        let elapsed = self.start_time.elapsed();
        self.estimated_remaining = super::ProgressUtils::estimate_remaining_time(
            self.current_progress,
            elapsed,
        );
    }
    
    /// Get elapsed time since start
    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }
    
    /// Check if installation is complete
    pub fn is_complete(&self) -> bool {
        self.completed_steps >= self.total_steps
    }
}

/// Download progress information
#[derive(Debug, Clone)]
pub struct DownloadProgress {
    /// Bytes downloaded so far
    pub bytes_downloaded: u64,
    /// Total bytes to download (if known)
    pub total_bytes: Option<u64>,
    /// Current download speed in bytes per second
    pub download_speed: f64,
    /// Estimated time to completion
    pub eta: Option<Duration>,
    /// Download start time
    pub start_time: Instant,
}

impl DownloadProgress {
    /// Create a new download progress tracker
    pub fn new(total_bytes: Option<u64>) -> Self {
        Self {
            bytes_downloaded: 0,
            total_bytes,
            download_speed: 0.0,
            eta: None,
            start_time: Instant::now(),
        }
    }
    
    /// Update download progress
    pub fn update(&mut self, bytes_downloaded: u64) {
        self.bytes_downloaded = bytes_downloaded;
        
        let elapsed = self.start_time.elapsed();
        if elapsed.as_secs_f64() > 0.0 {
            self.download_speed = bytes_downloaded as f64 / elapsed.as_secs_f64();
        }
        
        // Calculate ETA if total size is known
        if let Some(total) = self.total_bytes {
            if bytes_downloaded > 0 && bytes_downloaded < total {
                let remaining_bytes = total - bytes_downloaded;
                if self.download_speed > 0.0 {
                    let eta_secs = remaining_bytes as f64 / self.download_speed;
                    self.eta = Some(Duration::from_secs_f64(eta_secs));
                }
            }
        }
    }
    
    /// Get download progress as percentage (0.0 to 1.0)
    pub fn percentage(&self) -> Option<f32> {
        self.total_bytes.map(|total| {
            if total > 0 {
                self.bytes_downloaded as f32 / total as f32
            } else {
                0.0
            }
        })
    }
}

/// Task-specific progress information
#[derive(Debug, Clone)]
pub struct TaskProgress {
    pub task_id: TaskId,
    pub name: String,
    pub current_subtask: Option<String>,
    pub progress: f32, // 0.0 to 1.0
    pub state: ProgressState,
    pub start_time: Instant,
    pub estimated_duration: Option<Duration>,
}

impl TaskProgress {
    pub fn new(task_id: TaskId, name: String) -> Self {
        Self {
            task_id,
            name,
            current_subtask: None,
            progress: 0.0,
            state: ProgressState::NotStarted,
            start_time: Instant::now(),
            estimated_duration: None,
        }
    }
}

/// Progress state for individual tasks
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProgressState {
    NotStarted,
    InProgress,
    Completed,
    Failed { error: String },
    Cancelled,
}

/// Weight assigned to different progress components
#[derive(Debug, Clone)]
pub struct ProgressWeight {
    pub download: f32,
    pub extraction: f32,
    pub installation: f32,
    pub verification: f32,
}

impl Default for ProgressWeight {
    fn default() -> Self {
        Self {
            download: 0.4,     // 40% of total progress
            extraction: 0.3,   // 30% of total progress
            installation: 0.2, // 20% of total progress
            verification: 0.1, // 10% of total progress
        }
    }
}

/// Estimated time information
#[derive(Debug, Clone)]
pub struct EstimatedTime {
    pub total: Option<Duration>,
    pub remaining: Option<Duration>,
    pub per_step: Option<Duration>,
}

impl EstimatedTime {
    pub fn new() -> Self {
        Self {
            total: None,
            remaining: None,
            per_step: None,
        }
    }
}
