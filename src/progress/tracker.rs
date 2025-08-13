//! Task Progress Tracker
//!
//! Tracks the progress of individual tasks and subtasks during installation.

use super::types::{TaskId, TaskProgress, ProgressState};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use anyhow::Result;

/// Tracks progress for multiple concurrent tasks
pub struct TaskProgressTracker {
    tasks: Arc<RwLock<HashMap<TaskId, TaskProgress>>>,
}

impl TaskProgressTracker {
    /// Create a new task progress tracker
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Register a new task for tracking
    pub async fn register_task(&self, task_id: TaskId, name: String) -> Result<()> {
        let task_progress = TaskProgress::new(task_id.clone(), name);
        self.tasks.write().await.insert(task_id, task_progress);
        Ok(())
    }
    
    /// Update task progress
    pub async fn update_task_progress(
        &self,
        task_id: &TaskId,
        progress: f32,
        subtask: Option<String>,
    ) -> Result<()> {
        if let Some(task) = self.tasks.write().await.get_mut(task_id) {
            task.progress = progress.clamp(0.0, 1.0);
            task.current_subtask = subtask;
            task.state = if progress >= 1.0 {
                ProgressState::Completed
            } else {
                ProgressState::InProgress
            };
        }
        Ok(())
    }
    
    /// Mark task as failed
    pub async fn fail_task(&self, task_id: &TaskId, error: String) -> Result<()> {
        if let Some(task) = self.tasks.write().await.get_mut(task_id) {
            task.state = ProgressState::Failed { error };
        }
        Ok(())
    }
    
    /// Get current progress for a task
    pub async fn get_task_progress(&self, task_id: &TaskId) -> Option<TaskProgress> {
        self.tasks.read().await.get(task_id).cloned()
    }
    
    /// Get all task progress
    pub async fn get_all_progress(&self) -> HashMap<TaskId, TaskProgress> {
        self.tasks.read().await.clone()
    }
}

impl Default for TaskProgressTracker {
    fn default() -> Self {
        Self::new()
    }
}
