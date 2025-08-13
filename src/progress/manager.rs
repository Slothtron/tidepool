//! Enhanced Progress Manager
//!
//! Manages multiple concurrent progress tracking operations with rich feedback

use super::types::{InstallProgress, TaskProgress, TaskId, InstallStep, ProgressState};
use crate::ui::ProgressManager as BaseProgressManager;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use anyhow::Result;

/// Enhanced progress manager that supports multiple concurrent operations
pub struct EnhancedProgressManager {
    /// Base progress manager for compatibility
    base_manager: BaseProgressManager,
    /// Multi-progress display for concurrent operations
    multi_progress: MultiProgress,
    /// Individual task progress bars
    task_bars: Arc<RwLock<HashMap<TaskId, ProgressBar>>>,
    /// Main overall progress bar
    overall_bar: ProgressBar,
    /// Current installation progress state
    install_progress: Arc<RwLock<InstallProgress>>,
    /// Task progress tracking
    task_progress: Arc<RwLock<HashMap<TaskId, TaskProgress>>>,
    /// Start time for overall operation
    start_time: Instant,
}

impl EnhancedProgressManager {
    /// Create a new enhanced progress manager
    pub fn new(total_steps: u8) -> Self {
        let base_manager = BaseProgressManager::new();
        let multi_progress = MultiProgress::new();
        
        // Create overall progress bar
        let overall_bar = multi_progress.add(ProgressBar::new(total_steps as u64));
        overall_bar.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({percent}%) {msg}")
                .unwrap()
                .progress_chars("#>-")
        );
        overall_bar.set_message("Preparing installation...");
        
        Self {
            base_manager,
            multi_progress,
            task_bars: Arc::new(RwLock::new(HashMap::new())),
            overall_bar,
            install_progress: Arc::new(RwLock::new(InstallProgress::new(total_steps))),
            task_progress: Arc::new(RwLock::new(HashMap::new())),
            start_time: Instant::now(),
        }
    }
    
    /// Update the current installation step
    pub async fn update_step(&self, step: InstallStep, details: Option<String>) -> Result<()> {
        let mut progress = self.install_progress.write().await;
        progress.update_step(step.clone(), details.clone());
        
        // Update overall progress bar message
        let message = if let Some(ref details) = details {
            format!("{}: {}", step.name(), details)
        } else {
            step.description()
        };
        
        self.overall_bar.set_message(message);
        
        Ok(())
    }
    
    /// Start tracking a new task
    pub async fn start_task(
        &self, 
        task_id: TaskId, 
        task_name: String, 
        estimated_steps: u64
    ) -> Result<TaskProgressHandle> {
        // Create progress bar for this task
        let task_bar = self.multi_progress.add(ProgressBar::new(estimated_steps));
        task_bar.set_style(
            ProgressStyle::default_bar()
                .template("  {prefix} [{wide_bar:.green/dim}] {pos}/{len} {msg}")
                .unwrap()
                .progress_chars("=>-")
        );
        task_bar.set_prefix(format!("  {}", task_name));
        task_bar.set_message("Starting...");
        
        // Store the progress bar
        self.task_bars.write().await.insert(task_id.clone(), task_bar.clone());
        
        // Create and store task progress
        let task_progress = TaskProgress::new(task_id.clone(), task_name);
        self.task_progress.write().await.insert(task_id.clone(), task_progress);
        
        Ok(TaskProgressHandle::new(
            task_id,
            task_bar,
            self.overall_bar.clone(),
            self.task_progress.clone(),
        ))
    }
    
    /// Complete a task
    pub async fn complete_task(&self, task_id: &TaskId, success: bool) -> Result<()> {
        // Update task progress state
        if let Some(mut task) = self.task_progress.write().await.get_mut(task_id) {
            task.state = if success {
                ProgressState::Completed
            } else {
                ProgressState::Failed { error: "Task failed".to_string() }
            };
        }
        
        // Update task progress bar
        if let Some(bar) = self.task_bars.write().await.get(task_id) {
            if success {
                bar.finish_with_message("✓ Completed");
            } else {
                bar.finish_with_message("✗ Failed");
            }
        }
        
        // Update overall progress if successful
        if success {
            self.overall_bar.inc(1);
            let mut progress = self.install_progress.write().await;
            progress.complete_step();
        }
        
        Ok(())
    }
    
    /// Update download progress for a specific task
    pub async fn update_download_progress(
        &self,
        task_id: &TaskId,
        downloaded: u64,
        total: Option<u64>,
        speed: f64,
    ) -> Result<()> {
        if let Some(bar) = self.task_bars.read().await.get(task_id) {
            if let Some(total_bytes) = total {
                bar.set_length(total_bytes);
                bar.set_position(downloaded);
                
                let percent = (downloaded as f64 / total_bytes as f64) * 100.0;
                let speed_str = super::ProgressUtils::format_bytes_per_sec(speed);
                let message = format!("{:.1}% - {}/s", percent, speed_str);
                bar.set_message(message);
            } else {
                let downloaded_str = super::ProgressUtils::format_bytes(downloaded);
                let speed_str = super::ProgressUtils::format_bytes_per_sec(speed);
                let message = format!("{} - {}/s", downloaded_str, speed_str);
                bar.set_message(message);
            }
        }
        
        Ok(())
    }
    
    /// Get current installation progress
    pub async fn get_progress(&self) -> InstallProgress {
        self.install_progress.read().await.clone()
    }
    
    /// Get progress for a specific task
    pub async fn get_task_progress(&self, task_id: &TaskId) -> Option<TaskProgress> {
        self.task_progress.read().await.get(task_id).cloned()
    }
    
    /// Get total elapsed time
    pub fn total_elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }
    
    /// Finish the overall progress
    pub async fn finish(&self, success: bool) {
        if success {
            self.overall_bar.finish_with_message("✓ Installation completed successfully");
        } else {
            self.overall_bar.finish_with_message("✗ Installation failed");
        }
    }
    
    /// Get access to the base progress manager for compatibility
    pub fn base(&self) -> &BaseProgressManager {
        &self.base_manager
    }
}

/// Handle for updating progress of a specific task
pub struct TaskProgressHandle {
    task_id: TaskId,
    task_bar: ProgressBar,
    overall_bar: ProgressBar,
    task_progress: Arc<RwLock<HashMap<TaskId, TaskProgress>>>,
}

impl TaskProgressHandle {
    fn new(
        task_id: TaskId,
        task_bar: ProgressBar,
        overall_bar: ProgressBar,
        task_progress: Arc<RwLock<HashMap<TaskId, TaskProgress>>>,
    ) -> Self {
        Self {
            task_id,
            task_bar,
            overall_bar,
            task_progress,
        }
    }
    
    /// Update task progress
    pub async fn update_progress(&self, current: u64, message: Option<&str>) {
        self.task_bar.set_position(current);
        if let Some(msg) = message {
            self.task_bar.set_message(msg.to_string());
        }
        
        // Update internal progress tracking
        if let Some(mut task) = self.task_progress.write().await.get_mut(&self.task_id) {
            task.current_subtask = message.map(|s| s.to_string());
            task.state = ProgressState::InProgress;
        }
    }
    
    /// Update download progress specifically
    pub fn update_download_progress(&self, downloaded: u64, total: Option<u64>, speed: f64) {
        if let Some(total_bytes) = total {
            self.task_bar.set_length(total_bytes);
            self.task_bar.set_position(downloaded);
            
            let percent = (downloaded as f64 / total_bytes as f64) * 100.0;
            let speed_str = super::ProgressUtils::format_bytes_per_sec(speed);
            let message = format!("{:.1}% - {}/s", percent, speed_str);
            self.task_bar.set_message(message);
        } else {
            let downloaded_str = super::ProgressUtils::format_bytes(downloaded);
            let speed_str = super::ProgressUtils::format_bytes_per_sec(speed);
            let message = format!("{} - {}/s", downloaded_str, speed_str);
            self.task_bar.set_message(message);
        }
    }
    
    /// Update extraction progress
    pub fn update_extraction_progress(&self, current: u64, total: u64, current_file: &str) {
        self.task_bar.set_length(total);
        self.task_bar.set_position(current);
        
        let percent = (current as f64 / total as f64) * 100.0;
        let message = format!("{:.1}% - {}", percent, current_file);
        self.task_bar.set_message(message);
    }
    
    /// Mark task as completed
    pub async fn complete(&self, message: Option<&str>) {
        let final_message = message.unwrap_or("Completed");
        self.task_bar.finish_with_message(format!("✓ {}", final_message));
        
        // Update internal state
        if let Some(mut task) = self.task_progress.write().await.get_mut(&self.task_id) {
            task.state = ProgressState::Completed;
            task.progress = 1.0;
        }
    }
    
    /// Mark task as failed
    pub async fn fail(&self, error: &str) {
        self.task_bar.finish_with_message(format!("✗ {}", error));
        
        // Update internal state
        if let Some(mut task) = self.task_progress.write().await.get_mut(&self.task_id) {
            task.state = ProgressState::Failed { error: error.to_string() };
        }
    }
    
    /// Get the task ID
    pub fn task_id(&self) -> &TaskId {
        &self.task_id
    }
}
