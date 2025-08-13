//! Enhanced Installation Command
//!
//! Integrates the task executor, validation engine, and enhanced progress management
//! to provide a comprehensive installation experience with detailed feedback,
//! validation, and rollback capabilities.

use crate::config::Config;
use crate::progress::{
    InstallPlanner, TaskExecutor, ValidationEngine,
};
use crate::ui::{Messages, UI};
use crate::{GoManager, InstallRequest, SwitchRequest};
use anyhow::{Result, Context};
use std::fs;

/// Enhanced installation coordinator that orchestrates the complete installation process
pub struct EnhancedInstallationCoordinator {
    ui: UI,
    go_manager: GoManager,
    validator: ValidationEngine,
    planner: InstallPlanner,
}

impl EnhancedInstallationCoordinator {
    /// Create a new enhanced installation coordinator
    pub fn new() -> Self {
        Self {
            ui: UI::new(),
            go_manager: GoManager::new(),
            validator: ValidationEngine::new(),
            planner: InstallPlanner::new(),
        }
    }

    /// Execute enhanced installation with comprehensive validation and progress tracking
    pub async fn install_enhanced(
        &mut self,
        version: &str,
        config: &Config,
        force: bool,
    ) -> Result<()> {
        let install_dir = config.versions();
        let cache_dir = config.cache();

        self.ui.display_install_start(version);

        // Phase 1: Pre-installation validation
        self.ui.info("🔍 Running pre-installation validation...");
        let validation_report = self
            .validator
            .pre_installation_validation(version, &install_dir, &cache_dir, force)
            .await?;

        if !validation_report.overall_success {
            self.ui.display_error_with_suggestion(
                "Pre-installation validation failed",
                "Check the validation report for details",
            );
            return Err(anyhow::anyhow!("Pre-installation validation failed"));
        }

        self.ui.info(&format!(
            "✅ Pre-installation validation passed ({} checks)",
            validation_report.successful_checks
        ));

        // Phase 2: Create detailed installation plan
        self.ui.info("📋 Creating detailed installation plan...");
        let plan = self
            .planner
            .create_detailed_plan(version, &install_dir, &cache_dir, force)?;

        self.ui.info(&format!(
            "📋 Installation plan created with {} tasks (estimated: {:.1}s)",
            plan.tasks.len(),
            plan.estimated_total_time.as_secs_f64()
        ));

        // Check for existing installation (if not force)
        let version_dir = install_dir.join(version);
        if version_dir.exists() && !force {
            self.ui.info(&format!("Go {version} is already installed"));
            self.ui.suggest("Switching to existing installation");

            let switch_request = SwitchRequest {
                version: version.to_string(),
                base_dir: install_dir.clone(),
                global: false,
                force: false,
            };

            return self.switch_to_existing_version(switch_request).await;
        }

        // Phase 3: Execute installation plan with enhanced progress tracking
        self.ui.info("🚀 Starting enhanced installation...");
        let mut executor = TaskExecutor::new(plan.tasks.len() as u8);

        match executor.execute_plan(&plan).await {
            Ok(()) => {
                self.ui.info("✅ Installation plan executed successfully");

                // Phase 4: Post-installation validation
                self.ui.info("🔍 Running post-installation validation...");
                let post_validation = self
                    .validator
                    .post_installation_validation(version, &version_dir)
                    .await?;

                if post_validation.overall_success {
                    self.ui.info(&format!(
                        "✅ Post-installation validation passed ({} checks)",
                        post_validation.successful_checks
                    ));
                    self.ui.info(&format!("🎉 Go {} installed successfully!", version));
                } else {
                    self.ui.warning("⚠️ Post-installation validation found issues");
                    // Installation succeeded but validation failed - still usable
                }

                // Display execution summary
                self.display_execution_summary(&executor);
                Ok(())
            }
            Err(e) => {
                self.ui.display_error_with_suggestion(
                    &format!("Installation failed: {}", e),
                    "Check the execution log for details",
                );

                // Display execution log for debugging
                self.display_execution_log(&executor);
                Err(e)
            }
        }
    }

    /// Switch to existing installation
    async fn switch_to_existing_version(&self, switch_request: SwitchRequest) -> Result<()> {
        let version = switch_request.version.clone();
        match self.go_manager.switch_to(switch_request).await {
            Ok(()) => {
                self.ui.info(&format!(
                    "Switched to Go {}",
                    version
                ));
                Ok(())
            }
            Err(e) => {
                self.ui.display_error_with_suggestion(
                    &format!("Failed to switch to existing version: {}", e),
                    "Try installing with --force to reinstall",
                );
                Err(e)
            }
        }
    }

    /// Display execution summary
    fn display_execution_summary(&self, executor: &TaskExecutor) {
        let log = executor.get_execution_log();
        let successful = log.iter().filter(|entry| entry.success).count();
        let failed = log.len() - successful;

        self.ui.separator();
        self.ui.info("📊 Execution Summary:");
        self.ui.kv_pair_colored("Total Operations", &log.len().to_string(), "cyan");
        self.ui.kv_pair_colored("Successful", &successful.to_string(), "green");

        if failed > 0 {
            self.ui.kv_pair_colored("Failed", &failed.to_string(), "red");
        }

        self.ui.separator();
    }

    /// Display execution log for debugging
    fn display_execution_log(&self, executor: &TaskExecutor) {
        let log = executor.get_execution_log();

        self.ui.separator();
        self.ui.info("📜 Execution Log:");

        for entry in log.iter().take(10) {
            // Show last 10 entries
            let status = if entry.success { "✅" } else { "❌" };
            self.ui.info(&format!(
                "{} [{}] {}: {}",
                status,
                entry.task_id.0,
                entry.action,
                entry.details
            ));
        }

        if log.len() > 10 {
            self.ui.info(&format!("... and {} more entries", log.len() - 10));
        }

        self.ui.separator();
    }
}

/// Fallback installation function for compatibility
pub async fn install_with_fallback(
    version: &str,
    config: &Config,
    force: bool,
) -> Result<()> {
    // Try enhanced installation first
    let mut coordinator = EnhancedInstallationCoordinator::new();
    match coordinator.install_enhanced(version, config, force).await {
        Ok(()) => Ok(()),
        Err(e) => {
            // If enhanced installation fails, fall back to original implementation
            coordinator.ui.warning("Enhanced installation failed, falling back to basic installation");
            coordinator.ui.warning(&format!("Error: {}", e));

            // Use the original install function as fallback
            install_basic_fallback(version, config, force).await
        }
    }
}

/// Basic fallback installation (simplified version of original)
async fn install_basic_fallback(version: &str, config: &Config, force: bool) -> Result<()> {
    let ui = UI::new();
    let manager = GoManager::new();
    let install_dir = config.versions();
    let cache_dir = config.cache();
    let version_dir = install_dir.join(version);

    ui.display_install_start(version);
    ui.info("Using basic installation mode");

    // Basic validation
    if version_dir.exists() && !force {
        ui.info(&format!("Go {version} is already installed"));
        return Ok(());
    }

    if force && version_dir.exists() {
        ui.warning(&Messages::removing_existing_installation(version));
        fs::remove_dir_all(&version_dir).ok();
    }

    // Ensure directories exist
    fs::create_dir_all(&install_dir)
        .with_context(|| format!("Failed to create directory: {}", install_dir.display()))?;
    fs::create_dir_all(&cache_dir)
        .with_context(|| format!("Failed to create cache directory: {}", cache_dir.display()))?;

    // Basic installation request
    let install_request = InstallRequest {
        version: version.to_string(),
        install_dir: install_dir.clone(),
        download_dir: cache_dir.clone(),
        force,
    };

    match manager.install(install_request).await {
        Ok(version_info) => {
            ui.info(&format!("✅ Go {} installed successfully!", version));
            ui.info(&format!("Installation path: {}", version_info.path.display()));
            Ok(())
        }
        Err(e) => {
            ui.display_error_with_suggestion(
                &Messages::installation_failed(&e.to_string()),
                "Check your internet connection or try with --force flag",
            );
            Err(e)
        }
    }
}
