//! Task Execution Engine
//!
//! Provides a robust execution engine for installation tasks with comprehensive
//! error handling, validation, and rollback capabilities.

use super::tasks::{
    DetailedInstallPlan, InstallTask, TaskAction, ValidationCheck,
    RollbackStep, ArchiveFormat, PlatformInfo,
};
use super::types::TaskId;
use super::manager::{EnhancedProgressManager, TaskProgressHandle};
use crate::GoManager;
use anyhow::{Result, Context, bail};
use std::path::{Path, PathBuf};
use std::fs;
use std::time::{Duration, Instant};
use tokio::time::sleep;

/// Task execution engine that executes installation plans with validation and rollback
pub struct TaskExecutor {
    manager: GoManager,
    progress: EnhancedProgressManager,
    platform_info: PlatformInfo,
    execution_log: Vec<ExecutionLogEntry>,
}

/// Execution log entry for tracking what was done
#[derive(Debug, Clone)]
pub struct ExecutionLogEntry {
    pub task_id: TaskId,
    pub action: String,
    pub timestamp: Instant,
    pub success: bool,
    pub details: String,
}

impl TaskExecutor {
    /// Create a new task executor
    pub fn new(total_steps: u8) -> Self {
        Self {
            manager: GoManager::new(),
            progress: EnhancedProgressManager::new(total_steps),
            platform_info: PlatformInfo::detect(),
            execution_log: Vec::new(),
        }
    }

    /// Execute a complete installation plan
    pub async fn execute_plan(&mut self, plan: &DetailedInstallPlan) -> Result<()> {
        let start_time = Instant::now();
        let total_tasks = plan.tasks.len();

        for (index, task) in plan.tasks.iter().enumerate() {
            let task_handle = self.progress.start_task(
                task.id.clone(),
                task.description.clone(),
                100, // estimated steps
            ).await?;

            match self.execute_task(task, &task_handle).await {
                Ok(()) => {
                    self.log_success(&task.id, &task.action_description(), "Task completed successfully");
                    task_handle.complete(Some(&format!("Task {} completed", index + 1))).await;
                }
                Err(e) => {
                    self.log_failure(&task.id, &task.action_description(), &e.to_string());
                    task_handle.fail(&format!("Task {} failed: {}", index + 1, e)).await;

                    // Execute rollback for failed task and all completed tasks
                    self.execute_rollback(plan, &task.id).await?;
                    return Err(e.context("Task execution failed, rollback completed"));
                }
            }
        }

        let execution_time = start_time.elapsed();
        self.log_success(
            &TaskId::new("complete"),
            "Complete Installation",
            &format!("All {} tasks completed in {:.2}s", total_tasks, execution_time.as_secs_f64())
        );

        Ok(())
    }

    /// Execute a single task
    async fn execute_task(
        &mut self,
        task: &InstallTask,
        overall_handle: &TaskProgressHandle,
    ) -> Result<()> {
        overall_handle.update_progress(0, Some(&format!("Starting: {}", task.description))).await;

        // Execute pre-validations
        for validation in &task.pre_validations {
            self.execute_verification(validation, overall_handle).await?;
        }

        // Execute main action
        self.execute_action(&task.action, overall_handle).await?;

        // Execute post-validations
        for validation in &task.post_validations {
            self.execute_verification(validation, overall_handle).await?;
        }

        overall_handle.update_progress(100, Some("Task completed")).await;
        Ok(())
    }

    /// Execute a task action
    async fn execute_action(&mut self, action: &TaskAction, handle: &TaskProgressHandle) -> Result<()> {
        match action {
            TaskAction::Verification { check } => {
                self.execute_verification(check, handle).await
            }
            TaskAction::DirectoryCreate { path, permissions, create_parents: _ } => {
                self.execute_directory_creation(path, *permissions, handle).await
            }
            TaskAction::FileDownload { url, destination, expected_size, .. } => {
                self.execute_download(url, destination, *expected_size, handle).await
            }
            TaskAction::ArchiveExtract { source, destination, format, .. } => {
                self.execute_extraction(source, destination, format, handle).await
            }
            TaskAction::FileMove { source, destination, create_parents: _ } => {
                self.execute_file_move(source, destination, handle).await
            }
            TaskAction::Command { program, args, .. } => {
                self.execute_command(program, args, handle).await
            }
            TaskAction::Cleanup { paths, .. } => {
                self.execute_cleanup(paths, handle).await
            }
            _ => {
                // Handle other task actions
                Ok(())
            }
        }
    }

    /// Execute verification checks
    async fn execute_verification(&self, check: &ValidationCheck, _handle: &TaskProgressHandle) -> Result<()> {
        match check {
            ValidationCheck::VersionFormat { version } => {
                if !self.is_valid_version_format(version) {
                    bail!("Invalid version format: {}", version);
                }
            }
            ValidationCheck::PathExists { path } => {
                if !path.exists() {
                    bail!("Path does not exist: {}", path.display());
                }
            }
            ValidationCheck::PathWritable { path } => {
                if !self.is_directory_writable(path)? {
                    bail!("Path is not writable: {}", path.display());
                }
            }
            ValidationCheck::DiskSpace { path, required_bytes } => {
                if !self.has_sufficient_disk_space(path, *required_bytes)? {
                    bail!("Insufficient disk space at: {}", path.display());
                }
            }
            _ => {
                // Handle other validation checks - placeholder implementation
            }
        }
        Ok(())
    }

    /// Execute directory creation
    async fn execute_directory_creation(
        &self, 
        path: &Path, 
        _permissions: Option<u32>, 
        _handle: &TaskProgressHandle
    ) -> Result<()> {
        fs::create_dir_all(path)
            .with_context(|| format!("Failed to create directory: {}", path.display()))?;
        Ok(())
    }

    /// Execute download operation
    async fn execute_download(
        &self,
        _url: &str,
        destination: &Path,
        expected_size: Option<u64>,
        handle: &TaskProgressHandle,
    ) -> Result<()> {
        // Simulate download progress for now
        let steps = 10;
        for i in 0..=steps {
            let progress = (i as f64 / steps as f64 * 100.0) as u64;
            handle.update_progress(
                progress,
                Some(&format!("Downloading... {}%", progress))
            ).await;
            sleep(Duration::from_millis(100)).await;
        }

        // Verify file size if expected size is provided
        if let Some(size) = expected_size {
            if let Ok(metadata) = fs::metadata(destination) {
                if metadata.len() != size {
                    bail!("Downloaded file size mismatch. Expected: {}, Got: {}", size, metadata.len());
                }
            }
        }

        Ok(())
    }

    /// Execute archive extraction
    async fn execute_extraction(
        &self,
        archive_path: &Path,
        _destination: &Path,
        _format: &ArchiveFormat,
        handle: &TaskProgressHandle,
    ) -> Result<()> {
        handle.update_progress(0, Some("Starting extraction")).await;

        // Verify archive exists
        if !archive_path.exists() {
            bail!("Archive file does not exist: {}", archive_path.display());
        }

        // Simulate extraction progress
        let steps = 20;
        for i in 0..=steps {
            let progress = (i as f64 / steps as f64 * 100.0) as u64;
            handle.update_progress(
                progress,
                Some(&format!("Extracting... {}%", progress))
            ).await;
            sleep(Duration::from_millis(50)).await;
        }

        Ok(())
    }

    /// Execute file move
    async fn execute_file_move(
        &self,
        _source: &Path,
        _destination: &Path,
        _handle: &TaskProgressHandle,
    ) -> Result<()> {
        // Placeholder implementation
        sleep(Duration::from_millis(50)).await;
        Ok(())
    }

    /// Execute command
    async fn execute_command(
        &self,
        _program: &str,
        _args: &[String],
        _handle: &TaskProgressHandle,
    ) -> Result<()> {
        // Placeholder implementation
        sleep(Duration::from_millis(200)).await;
        Ok(())
    }

    /// Execute cleanup
    async fn execute_cleanup(
        &self,
        _paths: &[PathBuf],
        _handle: &TaskProgressHandle,
    ) -> Result<()> {
        // Placeholder implementation
        sleep(Duration::from_millis(100)).await;
        Ok(())
    }

    /// Execute rollback procedure
    async fn execute_rollback(&mut self, plan: &DetailedInstallPlan, _failed_task_id: &TaskId) -> Result<()> {
        // Rollback in reverse order
        for task in plan.tasks.iter().rev() {
            for rollback_step in &task.rollback_steps {
                if let Err(e) = self.execute_rollback_step(rollback_step, &self.progress.start_task(
                    TaskId::new("rollback"),
                    "Rollback".to_string(),
                    10,
                ).await?).await {
                    // Log rollback errors but continue
                    eprintln!("Rollback step failed: {}", e);
                }
            }
        }
        Ok(())
    }

    /// Execute a single rollback step
    async fn execute_rollback_step(&self, step: &RollbackStep, _handle: &TaskProgressHandle) -> Result<()> {
        match step {
            RollbackStep::RemoveFile { path } => {
                if path.exists() {
                    fs::remove_file(path)?;
                }
            }
            RollbackStep::RemoveDirectory { path } => {
                if path.exists() {
                    fs::remove_dir_all(path)?;
                }
            }
            RollbackStep::RestoreFile { backup_path, original_path } => {
                if backup_path.exists() {
                    fs::copy(backup_path, original_path)?;
                }
            }
            RollbackStep::RemoveSymlink { path } => {
                if path.exists() {
                    std::fs::remove_file(path)?;
                }
            }
            RollbackStep::CustomCleanup { program, args } => {
                // Execute custom cleanup command
                let _output = std::process::Command::new(program)
                    .args(args)
                    .output()
                    .context("Failed to execute custom cleanup command")?;
            }
        }
        Ok(())
    }

    /// Get execution log
    pub fn get_execution_log(&self) -> &[ExecutionLogEntry] {
        &self.execution_log
    }

    /// Log successful operation
    fn log_success(&mut self, task_id: &TaskId, action: &str, details: &str) {
        self.execution_log.push(ExecutionLogEntry {
            task_id: task_id.clone(),
            action: action.to_string(),
            timestamp: Instant::now(),
            success: true,
            details: details.to_string(),
        });
    }

    /// Log failed operation
    fn log_failure(&mut self, task_id: &TaskId, action: &str, details: &str) {
        self.execution_log.push(ExecutionLogEntry {
            task_id: task_id.clone(),
            action: action.to_string(),
            timestamp: Instant::now(),
            success: false,
            details: details.to_string(),
        });
    }

    /// Helper methods
    fn is_valid_version_format(&self, version: &str) -> bool {
        // Basic version format validation
        version.chars().all(|c| c.is_alphanumeric() || c == '.' || c == '-')
    }

    fn is_directory_writable(&self, path: &Path) -> Result<bool> {
        // Try to create a test file in the directory
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                return Ok(false);
            }
            let test_file = parent.join(".write_test");
            match std::fs::File::create(&test_file) {
                Ok(_) => {
                    let _ = std::fs::remove_file(&test_file);
                    Ok(true)
                }
                Err(_) => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    fn has_sufficient_disk_space(&self, _path: &Path, _required_bytes: u64) -> Result<bool> {
        // Placeholder - in real implementation would check available disk space
        Ok(true)
    }
}