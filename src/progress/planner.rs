//! Installation Planning System
//!
//! Creates detailed, executable plans for Go installation with comprehensive
//! task breakdown, dependency management, and rollback strategies.

use super::tasks::{
    DetailedInstallPlan, InstallTask, SubTask, TaskAction, TaskDependency, DependencyType,
    RollbackStep, ValidationCheck, ArchiveFormat, PlatformInfo,
};
use crate::progress::types::TaskId;
use std::path::Path;
use std::time::Duration;
use anyhow::Result;

/// Installation planner that creates comprehensive installation plans
pub struct InstallPlanner {
    platform_info: PlatformInfo,
}

impl InstallPlanner {
    /// Create a new installation planner
    pub fn new() -> Self {
        Self {
            platform_info: PlatformInfo::detect(),
        }
    }
    
    /// Create a detailed installation plan for a Go version
    pub fn create_detailed_plan(
        &self,
        version: &str,
        install_dir: &Path,
        cache_dir: &Path,
        force: bool,
    ) -> Result<DetailedInstallPlan> {
        let mut tasks = Vec::new();
        
        // Task 1: Pre-installation validation
        tasks.push(self.create_precheck_task(version, install_dir, cache_dir, force)?);
        
        // Task 2: Directory setup
        tasks.push(self.create_directory_setup_task(install_dir, cache_dir, version)?);
        
        // Task 3: Download (if needed)
        let archive_name = self.platform_info.archive_filename(version);
        let cached_file = cache_dir.join(&archive_name);
        let needs_download = !cached_file.exists() || force;
        
        if needs_download {
            tasks.push(self.create_download_task(version, &cached_file, &archive_name)?);
        }
        
        // Task 4: Extraction and installation
        tasks.push(self.create_extraction_task(version, &cached_file, install_dir)?);
        
        // Task 5: Post-installation verification
        tasks.push(self.create_verification_task(version, install_dir)?);
        
        // Task 6: Finalization
        tasks.push(self.create_finalization_task(version, install_dir)?);
        
        // Build dependencies and rollback plan
        let dependencies = self.build_dependency_graph(&tasks);
        let rollback_plan = self.create_rollback_plan(&tasks, install_dir, version);
        let estimated_total_time = tasks.iter().map(|t| t.estimated_duration).sum();
        
        Ok(DetailedInstallPlan {
            version: version.to_string(),
            tasks,
            dependencies,
            rollback_plan,
            estimated_total_time,
        })
    }
    
    /// Create pre-installation validation task
    fn create_precheck_task(
        &self,
        version: &str,
        install_dir: &Path,
        cache_dir: &Path,
        force: bool,
    ) -> Result<InstallTask> {
        let task = InstallTask::new(
            "precheck",
            "Pre-installation Checks",
            "Validate environment and requirements",
        )
        .with_duration(Duration::from_secs(5))
        .with_subtask(
            SubTask::new(
                "Validate version format",
                TaskAction::Verification {
                    check: ValidationCheck::VersionFormat {
                        version: version.to_string(),
                    },
                },
            )
            .with_weight(0.2)
            .no_retry(),
        )
        .with_subtask(
            SubTask::new(
                "Check disk space",
                TaskAction::Verification {
                    check: ValidationCheck::DiskSpace {
                        path: install_dir.to_path_buf(),
                        required_bytes: self.estimate_required_space(version),
                    },
                },
            )
            .with_weight(0.3),
        )
        .with_subtask(
            SubTask::new(
                "Check existing installation",
                TaskAction::Verification {
                    check: ValidationCheck::ExistingInstallation {
                        version: version.to_string(),
                        install_dir: install_dir.to_path_buf(),
                        force,
                    },
                },
            )
            .with_weight(0.5)
            .no_retry(),
        )
        .with_validation(ValidationCheck::PathWritable {
            path: install_dir.to_path_buf(),
        })
        .with_validation(ValidationCheck::PathWritable {
            path: cache_dir.to_path_buf(),
        });
        
        Ok(task)
    }
    
    /// Create directory setup task
    fn create_directory_setup_task(
        &self,
        install_dir: &Path,
        cache_dir: &Path,
        version: &str,
    ) -> Result<InstallTask> {
        let task = InstallTask::new(
            "create_dirs",
            "Directory Setup",
            "Create necessary directories",
        )
        .with_duration(Duration::from_secs(2))
        .depends_on(TaskId::new("precheck"))
        .with_subtask(
            SubTask::new(
                "Create install directory",
                TaskAction::DirectoryCreate {
                    path: install_dir.to_path_buf(),
                    permissions: Some(0o755),
                    create_parents: true,
                },
            )
            .with_weight(0.4),
        )
        .with_subtask(
            SubTask::new(
                "Create cache directory",
                TaskAction::DirectoryCreate {
                    path: cache_dir.to_path_buf(),
                    permissions: Some(0o755),
                    create_parents: true,
                },
            )
            .with_weight(0.4),
        )
        .with_subtask(
            SubTask::new(
                "Create version-specific directory",
                TaskAction::DirectoryCreate {
                    path: install_dir.join(version),
                    permissions: Some(0o755),
                    create_parents: true,
                },
            )
            .with_weight(0.2),
        )
        .with_validation(ValidationCheck::PathExists {
            path: install_dir.to_path_buf(),
        })
        .with_validation(ValidationCheck::PathExists {
            path: cache_dir.to_path_buf(),
        })
        .with_validation(ValidationCheck::PathWritable {
            path: install_dir.join(version),
        });
        
        Ok(task)
    }
    
    /// Create download task
    fn create_download_task(
        &self,
        version: &str,
        cached_file: &Path,
        archive_name: &str,
    ) -> Result<InstallTask> {
        let download_url = format!("https://go.dev/dl/{}", archive_name);
        
        let task = InstallTask::new(
            "download",
            "Download Go Archive",
            &format!("Download {} from official repository", archive_name),
        )
        .with_duration(Duration::from_secs(120)) // Estimate 2 minutes
        .depends_on(TaskId::new("create_dirs"))
        .with_subtask(
            SubTask::new(
                "Download archive",
                TaskAction::FileDownload {
                    url: download_url,
                    destination: cached_file.to_path_buf(),
                    checksum: None, // TODO: Add checksum verification
                    expected_size: None,
                },
            )
            .with_weight(0.95)
            .with_timeout(Duration::from_secs(300)), // 5 minute timeout
        )
        .with_subtask(
            SubTask::new(
                "Verify download integrity",
                TaskAction::Verification {
                    check: ValidationCheck::FileSize {
                        path: cached_file.to_path_buf(),
                        min_size: Some(50 * 1024 * 1024), // At least 50MB
                        max_size: None,
                    },
                },
            )
            .with_weight(0.05),
        )
        .with_validation(ValidationCheck::PathExists {
            path: cached_file.to_path_buf(),
        });
        
        Ok(task)
    }
    
    /// Create extraction task
    fn create_extraction_task(
        &self,
        version: &str,
        archive_source: &Path,
        install_dir: &Path,
    ) -> Result<InstallTask> {
        let version_dir = install_dir.join(version);
        let archive_format = ArchiveFormat::from_extension(&self.platform_info.extension)
            .unwrap_or(ArchiveFormat::TarGz);
        
        let task = InstallTask::new(
            "extract",
            "Extract Archive",
            "Extract Go archive to installation directory",
        )
        .with_duration(Duration::from_secs(30))
        .depends_on(if archive_source.exists() {
            TaskId::new("create_dirs")
        } else {
            TaskId::new("download")
        })
        .with_subtask(
            SubTask::new(
                "Extract archive",
                TaskAction::ArchiveExtract {
                    source: archive_source.to_path_buf(),
                    destination: version_dir.clone(),
                    format: archive_format,
                    strip_components: Some(1), // Remove the top-level "go" directory
                },
            )
            .with_weight(0.9),
        )
        .with_subtask(
            SubTask::new(
                "Set executable permissions",
                TaskAction::PermissionSet {
                    path: version_dir.join("bin"),
                    permissions: 0o755,
                    recursive: true,
                },
            )
            .with_weight(0.1),
        )
        .with_validation(ValidationCheck::PathExists {
            path: version_dir.join("bin").join(if cfg!(target_os = "windows") { "go.exe" } else { "go" }),
        })
        .with_validation(ValidationCheck::PathExists {
            path: version_dir.join("src"),
        })
        .with_validation(ValidationCheck::PathExists {
            path: version_dir.join("pkg"),
        });
        
        Ok(task)
    }
    
    /// Create verification task
    fn create_verification_task(
        &self,
        version: &str,
        install_dir: &Path,
    ) -> Result<InstallTask> {
        let version_dir = install_dir.join(version);
        let go_binary = version_dir.join("bin").join(if cfg!(target_os = "windows") { "go.exe" } else { "go" });
        
        let task = InstallTask::new(
            "verify_install",
            "Verify Installation",
            "Verify Go installation is working correctly",
        )
        .with_duration(Duration::from_secs(10))
        .depends_on(TaskId::new("extract"))
        .with_subtask(
            SubTask::new(
                "Test Go binary",
                TaskAction::Verification {
                    check: ValidationCheck::ExecutableWorks {
                        path: go_binary.clone(),
                        args: vec!["version".to_string()],
                        expected_exit_code: Some(0),
                    },
                },
            )
            .with_weight(0.5),
        )
        .with_subtask(
            SubTask::new(
                "Verify installation completeness",
                TaskAction::Verification {
                    check: ValidationCheck::InstallationComplete {
                        install_dir: version_dir.clone(),
                        expected_files: vec![
                            "bin/go".into(),
                            "bin/gofmt".into(),
                            "src".into(),
                            "pkg".into(),
                        ],
                    },
                },
            )
            .with_weight(0.5),
        )
        .with_validation(ValidationCheck::GoVersionMatch {
            go_binary,
            expected_version: version.to_string(),
        });
        
        Ok(task)
    }
    
    /// Create finalization task
    fn create_finalization_task(
        &self,
        version: &str,
        install_dir: &Path,
    ) -> Result<InstallTask> {
        let task = InstallTask::new(
            "finalize",
            "Finalize Installation",
            "Complete installation and cleanup",
        )
        .with_duration(Duration::from_secs(5))
        .depends_on(TaskId::new("verify_install"))
        .with_subtask(
            SubTask::new(
                "Register installation",
                TaskAction::Verification {
                    check: ValidationCheck::PathExists {
                        path: install_dir.join(version),
                    },
                },
            )
            .with_weight(1.0)
            .no_retry(),
        );
        
        Ok(task)
    }
    
    /// Build task dependency graph
    fn build_dependency_graph(&self, tasks: &[InstallTask]) -> Vec<TaskDependency> {
        let mut dependencies = Vec::new();
        
        for task in tasks {
            for prereq in &task.prerequisites {
                dependencies.push(TaskDependency {
                    task_id: task.id.clone(),
                    depends_on: prereq.clone(),
                    dependency_type: DependencyType::Sequential,
                });
            }
        }
        
        dependencies
    }
    
    /// Create rollback plan
    fn create_rollback_plan(
        &self,
        tasks: &[InstallTask],
        install_dir: &Path,
        version: &str,
    ) -> Vec<RollbackStep> {
        let mut rollback_steps = Vec::new();
        
        // Rollback in reverse order
        for task in tasks.iter().rev() {
            match task.id.as_ref() {
                "create_dirs" => {
                    rollback_steps.push(RollbackStep::RemoveDirectory {
                        path: install_dir.join(version),
                    });
                }
                "download" => {
                    // Find the download destination from subtasks
                    for subtask in &task.subtasks {
                        if let TaskAction::FileDownload { destination, .. } = &subtask.action {
                            rollback_steps.push(RollbackStep::RemoveFile {
                                path: destination.clone(),
                            });
                        }
                    }
                }
                "extract" => {
                    rollback_steps.push(RollbackStep::RemoveDirectory {
                        path: install_dir.join(version),
                    });
                }
                _ => {} // Other tasks don't require special cleanup
            }
        }
        
        rollback_steps
    }
    
    /// Estimate required disk space for installation
    fn estimate_required_space(&self, version: &str) -> u64 {
        // Base estimation: most Go installations are between 100-500MB
        // We'll be conservative and estimate 500MB
        let base_size = 500 * 1024 * 1024; // 500MB
        
        // Add some version-specific adjustments
        if version.starts_with("1.2") {
            base_size + 100 * 1024 * 1024 // Newer versions are larger
        } else {
            base_size
        }
    }
}

impl Default for InstallPlanner {
    fn default() -> Self {
        Self::new()
    }
}
