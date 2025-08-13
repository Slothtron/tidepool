//! Installation Task System
//!
//! Provides a comprehensive task-based approach to managing complex installations
//! with detailed planning, execution tracking, and rollback capabilities.

use crate::progress::types::TaskId;
use std::path::PathBuf;
use std::time::Duration;
use anyhow::Result;

/// Complete installation plan with tasks, dependencies, and rollback strategy
#[derive(Debug, Clone)]
pub struct DetailedInstallPlan {
    pub version: String,
    pub tasks: Vec<InstallTask>,
    pub dependencies: Vec<TaskDependency>,
    pub rollback_plan: Vec<RollbackStep>,
    pub estimated_total_time: Duration,
}

impl DetailedInstallPlan {
    /// Get tasks in execution order based on dependencies
    pub fn execution_order(&self) -> Result<Vec<TaskId>> {
        let mut ordered = Vec::new();
        let mut remaining: Vec<_> = self.tasks.iter().map(|t| &t.id).collect();
        let mut added = std::collections::HashSet::new();
        
        while !remaining.is_empty() {
            let mut progress_made = false;
            
            remaining.retain(|&task_id| {
                let task = self.tasks.iter().find(|t| &t.id == task_id).unwrap();
                
                // Check if all prerequisites are satisfied
                let can_execute = task.prerequisites.iter()
                    .all(|prereq| added.contains(prereq));
                
                if can_execute {
                    ordered.push(task_id.clone());
                    added.insert(task_id.clone());
                    progress_made = true;
                    false // Remove from remaining
                } else {
                    true // Keep in remaining
                }
            });
            
            if !progress_made {
                return Err(anyhow::anyhow!("Circular dependency detected in tasks"));
            }
        }
        
        Ok(ordered)
    }
    
    /// Get total estimated duration
    pub fn total_estimated_duration(&self) -> Duration {
        self.tasks.iter()
            .map(|task| task.estimated_duration)
            .sum()
    }
}

/// Individual installation task with subtasks and validation
#[derive(Debug, Clone)]
pub struct InstallTask {
    pub id: TaskId,
    pub name: String,
    pub description: String,
    pub estimated_duration: Duration,
    pub prerequisites: Vec<TaskId>,
    pub subtasks: Vec<SubTask>,
    pub validation_criteria: Vec<ValidationCheck>,
    pub retry_policy: RetryPolicy,
    /// Progress weight for the task (0.0 to 1.0)
    pub progress_weight: f32,
    /// Pre-execution validations
    pub pre_validations: Vec<ValidationCheck>,
    /// Post-execution validations  
    pub post_validations: Vec<ValidationCheck>,
    /// Main action to execute
    pub action: TaskAction,
    /// Rollback steps for this task
    pub rollback_steps: Vec<RollbackStep>,
}

impl InstallTask {
    pub fn new(id: &str, name: &str, description: &str) -> Self {
        Self {
            id: TaskId::new(id),
            name: name.to_string(),
            description: description.to_string(),
            estimated_duration: Duration::from_secs(30),
            prerequisites: Vec::new(),
            subtasks: Vec::new(),
            validation_criteria: Vec::new(),
            retry_policy: RetryPolicy::default(),
            progress_weight: 1.0,
            pre_validations: Vec::new(),
            post_validations: Vec::new(),
            action: TaskAction::Verification {
                check: ValidationCheck::PathExists {
                    path: std::path::PathBuf::from("/"),
                },
            },
            rollback_steps: Vec::new(),
        }
    }
    
    /// Add a prerequisite task
    pub fn depends_on(mut self, task_id: TaskId) -> Self {
        self.prerequisites.push(task_id);
        self
    }
    
    /// Add a subtask
    pub fn with_subtask(mut self, subtask: SubTask) -> Self {
        self.subtasks.push(subtask);
        self
    }
    
    /// Add validation criteria
    pub fn with_validation(mut self, validation: ValidationCheck) -> Self {
        self.validation_criteria.push(validation);
        self
    }
    
    /// Set estimated duration
    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.estimated_duration = duration;
        self
    }
    
    /// Set progress weight
    pub fn with_progress_weight(mut self, weight: f32) -> Self {
        self.progress_weight = weight;
        self
    }
    
    /// Set main action
    pub fn with_action(mut self, action: TaskAction) -> Self {
        self.action = action;
        self
    }
    
    /// Add pre-validation
    pub fn with_pre_validation(mut self, validation: ValidationCheck) -> Self {
        self.pre_validations.push(validation);
        self
    }
    
    /// Add post-validation
    pub fn with_post_validation(mut self, validation: ValidationCheck) -> Self {
        self.post_validations.push(validation);
        self
    }
    
    /// Add rollback step
    pub fn with_rollback_step(mut self, step: RollbackStep) -> Self {
        self.rollback_steps.push(step);
        self
    }
    
    /// Get action description for display
    pub fn action_description(&self) -> String {
        match &self.action {
            TaskAction::FileDownload { url, .. } => format!("Download from {}", url),
            TaskAction::DirectoryCreate { path, .. } => format!("Create directory {}", path.display()),
            TaskAction::ArchiveExtract { source, .. } => format!("Extract {}", source.display()),
            TaskAction::FileMove { source, destination, .. } => format!("Move {} to {}", source.display(), destination.display()),
            TaskAction::SymlinkCreate { target, link, .. } => format!("Link {} to {}", link.display(), target.display()),
            TaskAction::PermissionSet { path, .. } => format!("Set permissions on {}", path.display()),
            TaskAction::Verification { .. } => "Run verification".to_string(),
            TaskAction::Command { program, .. } => format!("Run command {}", program),
            TaskAction::Cleanup { .. } => "Cleanup files".to_string(),
        }
    }
}

/// Individual subtask within a larger task
#[derive(Debug, Clone)]
pub struct SubTask {
    pub name: String,
    pub action: TaskAction,
    pub progress_weight: f32, // Weight in overall task progress (0.0 to 1.0)
    pub can_retry: bool,
    pub max_retries: u8,
    pub timeout: Option<Duration>,
}

impl SubTask {
    pub fn new(name: &str, action: TaskAction) -> Self {
        Self {
            name: name.to_string(),
            action,
            progress_weight: 1.0,
            can_retry: true,
            max_retries: 3,
            timeout: None,
        }
    }
    
    pub fn with_weight(mut self, weight: f32) -> Self {
        self.progress_weight = weight;
        self
    }
    
    pub fn no_retry(mut self) -> Self {
        self.can_retry = false;
        self.max_retries = 0;
        self
    }
    
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }
}

/// Specific actions that can be performed during installation
#[derive(Debug, Clone)]
pub enum TaskAction {
    /// Download a file from a URL
    FileDownload {
        url: String,
        destination: PathBuf,
        checksum: Option<String>,
        expected_size: Option<u64>,
    },
    /// Create a directory with optional permissions
    DirectoryCreate {
        path: PathBuf,
        permissions: Option<u32>,
        create_parents: bool,
    },
    /// Extract an archive to a destination
    ArchiveExtract {
        source: PathBuf,
        destination: PathBuf,
        format: ArchiveFormat,
        strip_components: Option<u32>,
    },
    /// Move a file or directory
    FileMove {
        source: PathBuf,
        destination: PathBuf,
        create_parents: bool,
    },
    /// Create a symbolic link
    SymlinkCreate {
        target: PathBuf,
        link: PathBuf,
    },
    /// Set file or directory permissions
    PermissionSet {
        path: PathBuf,
        permissions: u32,
        recursive: bool,
    },
    /// Run a verification check
    Verification {
        check: ValidationCheck,
    },
    /// Execute a custom command
    Command {
        program: String,
        args: Vec<String>,
        working_dir: Option<PathBuf>,
        environment: Option<std::collections::HashMap<String, String>>,
    },
    /// Clean up temporary files
    Cleanup {
        paths: Vec<PathBuf>,
        ignore_errors: bool,
    },
}

/// Archive format types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArchiveFormat {
    Zip,
    TarGz,
    TarXz,
    Tar,
}

impl ArchiveFormat {
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "zip" => Some(Self::Zip),
            "tar.gz" | "tgz" => Some(Self::TarGz),
            "tar.xz" | "txz" => Some(Self::TarXz),
            "tar" => Some(Self::Tar),
            _ => None,
        }
    }
}

/// Validation checks to ensure installation correctness
#[derive(Debug, Clone)]
pub enum ValidationCheck {
    /// Check if a path exists
    PathExists {
        path: PathBuf,
    },
    /// Check if a path is writable
    PathWritable {
        path: PathBuf,
    },
    /// Check file size is within expected range
    FileSize {
        path: PathBuf,
        min_size: Option<u64>,
        max_size: Option<u64>,
    },
    /// Verify file integrity with checksum
    FileIntegrity {
        file: PathBuf,
        expected_checksum: String,
        algorithm: ChecksumAlgorithm,
    },
    /// Check if an executable works
    ExecutableWorks {
        path: PathBuf,
        args: Vec<String>,
        expected_exit_code: Option<i32>,
    },
    /// Verify Go version matches expected
    GoVersionMatch {
        go_binary: PathBuf,
        expected_version: String,
    },
    /// Check installation completeness
    InstallationComplete {
        install_dir: PathBuf,
        expected_files: Vec<PathBuf>,
    },
    /// Verify disk space availability
    DiskSpace {
        path: PathBuf,
        required_bytes: u64,
    },
    /// Validate version format
    VersionFormat {
        version: String,
    },
    /// Check for existing installation
    ExistingInstallation {
        version: String,
        install_dir: PathBuf,
        force: bool,
    },
}

/// Checksum algorithms
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChecksumAlgorithm {
    Sha256,
    Sha1,
    Md5,
}

/// Task dependency relationship
#[derive(Debug, Clone)]
pub struct TaskDependency {
    pub task_id: TaskId,
    pub depends_on: TaskId,
    pub dependency_type: DependencyType,
}

/// Type of dependency between tasks
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DependencyType {
    /// Task must complete before dependent task can start
    Sequential,
    /// Task output is required for dependent task
    DataDependency,
    /// Tasks share resources and should not run concurrently
    ResourceConflict,
}

/// Rollback operations for cleaning up failed installations
#[derive(Debug, Clone)]
pub enum RollbackStep {
    /// Remove a file
    RemoveFile {
        path: PathBuf,
    },
    /// Remove a directory and its contents
    RemoveDirectory {
        path: PathBuf,
    },
    /// Restore a file from backup
    RestoreFile {
        backup_path: PathBuf,
        original_path: PathBuf,
    },
    /// Remove a symbolic link
    RemoveSymlink {
        path: PathBuf,
    },
    /// Run a custom cleanup command
    CustomCleanup {
        program: String,
        args: Vec<String>,
    },
}

/// Retry policy for handling transient failures
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    pub max_attempts: u8,
    pub base_delay: Duration,
    pub max_delay: Duration,
    pub backoff_multiplier: f64,
    pub retryable_errors: Vec<ErrorType>,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            base_delay: Duration::from_millis(1000),
            max_delay: Duration::from_secs(30),
            backoff_multiplier: 2.0,
            retryable_errors: vec![
                ErrorType::NetworkError,
                ErrorType::TemporaryFileSystemError,
                ErrorType::ResourceTemporarilyUnavailable,
            ],
        }
    }
}

/// Types of errors that can be retried
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorType {
    NetworkError,
    FileSystemError,
    TemporaryFileSystemError,
    ResourceTemporarilyUnavailable,
    ChecksumMismatch,
    ExtractionError,
    PermissionDenied,
    InvalidInput,
}

/// Platform information for installation planning
#[derive(Debug, Clone)]
pub struct PlatformInfo {
    pub os: String,
    pub arch: String,
    pub extension: String,
}

impl PlatformInfo {
    pub fn detect() -> Self {
        let (os, arch) = if cfg!(target_os = "windows") {
            ("windows", if cfg!(target_arch = "x86_64") { "amd64" } else { "386" })
        } else if cfg!(target_os = "macos") {
            ("darwin", if cfg!(target_arch = "x86_64") { "amd64" } else { "arm64" })
        } else {
            ("linux", if cfg!(target_arch = "x86_64") { "amd64" } else { "386" })
        };
        
        let extension = if cfg!(target_os = "windows") { "zip" } else { "tar.gz" };
        
        Self {
            os: os.to_string(),
            arch: arch.to_string(),
            extension: extension.to_string(),
        }
    }
    
    pub fn archive_filename(&self, version: &str) -> String {
        format!("go{}.{}-{}.{}", version, self.os, self.arch, self.extension)
    }
}
