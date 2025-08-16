use crate::config::Config;

use crate::ui_flat::SimpleUI;
use crate::{
    GoManager, ListInstalledRequest, Result, StatusRequest, SwitchRequest, UninstallRequest,
};

/// Install a Go version using simplified installation system.
///
/// # Errors
/// Returns an error if the installation fails, network issues occur, or file system operations fail.
pub async fn install(version: &str, config: &Config, force: bool) -> Result<()> {
    // The actual installation logic (requires network download)
    let manager = GoManager::new();
    let install_request = crate::InstallRequest {
        version: version.to_string(),
        install_dir: config.versions().clone(),
        download_dir: config.cache().clone(),
        force,
    };

    match manager.install(install_request).await {
        Ok(version_info) => {
            let ui = SimpleUI::new();
            ui.success(&format!("Go {} installed successfully", version_info.version));
            if let Some(install_path) = &version_info.install_path {
                ui.info(&format!("Installation path: {}", install_path.display()));
            }
            ui.hint(&format!("Use 'gvm use {version}' to activate this version"));
            Ok(())
        }
        Err(e) => {
            let ui = SimpleUI::new();
            ui.error(&format!("Failed to install Go {version}: {e}"));
            Err(e)
        }
    }
}

/// List all installed Go versions.
///
/// # Errors
/// Returns an error if the directory listing fails or if I/O operations fail.
pub fn list_installed(config: &Config) -> Result<Vec<String>> {
    let base_dir = config.versions();
    let mut versions = Vec::new();

    if let Ok(entries) = std::fs::read_dir(base_dir) {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                if entry.path().is_dir() && !name.starts_with('.') && name != "current" {
                    versions.push(name.to_string());
                }
            }
        }
    }

    versions.sort();
    Ok(versions)
}

/// Uninstall a Go version.
///
/// # Errors
/// Returns an error if the uninstallation fails or file system operations fail.
pub fn uninstall(version: &str, config: &Config) -> Result<()> {
    let ui = SimpleUI::new();
    let manager = GoManager::new();
    let base_dir = config.versions();

    ui.info(&format!("Uninstalling Go {version}"));

    let uninstall_request =
        UninstallRequest { version: version.to_string(), base_dir: base_dir.clone() };

    match manager.uninstall(uninstall_request) {
        Ok(()) => {
            ui.success(&format!("Go {version} has been successfully uninstalled"));
        }
        Err(e) => {
            if e.to_string().contains("not installed") {
                ui.warning(&format!("Go {version} is not installed"));
            } else if e.to_string().contains("currently active") {
                ui.error(&format!("Cannot uninstall Go {version} - it is currently in use"));
                ui.hint("Please switch to another version or remove the symbolic link manually");
            } else {
                ui.error(&format!("Failed to uninstall Go {version}: {e}"));
            }
        }
    }

    Ok(())
}

/// Switch to a specific Go version.
///
/// # Errors
/// Returns an error if the switch operation fails.
pub fn switch(version: &str, config: &Config, global: bool, force: bool) -> Result<()> {
    let ui = SimpleUI::new();
    let manager = GoManager::new();
    let base_dir = config.versions();

    let switch_request =
        SwitchRequest { version: version.to_string(), base_dir: base_dir.clone(), global, force };

    match manager.switch_to(switch_request) {
        Ok(_) => {
            ui.success(&format!(
                "Switched to Go {} {}",
                version,
                if global { "(global)" } else { "(local)" }
            ));
        }
        Err(e) => {
            ui.error(&format!("Failed to switch to Go {version}: {e}"));
            ui.hint("Ensure the version is installed and you have the correct permissions");
        }
    }

    Ok(())
}

/// Show the current Go version status.
///
/// # Errors
/// Returns an error if the status operation fails.
pub fn status(config: &Config) -> Result<()> {
    let ui = SimpleUI::new();
    let manager = GoManager::new();
    let base_dir = config.versions();

    let status_request = StatusRequest { base_dir: Some(base_dir.clone()) };

    match manager.status(status_request) {
        Ok(status) => {
            // Simplified output, showing only the most important information
            if let Some(current_version) = status.current_version {
                ui.success(&format!("Current version: Go {current_version}"));

                // Show only GOROOT, not the full PATH
                if let Some(goroot) = status.environment_vars.get("GOROOT") {
                    ui.key_value("Installation path", goroot);
                }

                // Show simplified status information
                ui.info("Go environment is configured");
                ui.hint("Use 'go version' to verify the installation");
            } else {
                ui.warning("No active Go version found");
                ui.hint("Use 'gvm list' to see installed versions");
                ui.hint("Use 'gvm use <version>' to activate a version");
            }
        }
        Err(e) => {
            ui.error(&format!("Failed to get status: {e}"));
            ui.hint("Please check if Go is installed correctly");
        }
    }

    Ok(())
}

/// List Go versions, either installed or available online.
///
/// # Errors
/// Returns an error if the listing operation fails.
pub fn list(config: &Config, all: bool) -> Result<()> {
    if all {
        list_available_versions()
    } else {
        list_installed_versions(config)
    }
}

/// List installed Go versions.
fn list_installed_versions(config: &Config) -> Result<()> {
    let ui = SimpleUI::new();
    let manager = GoManager::new();
    let base_dir = config.versions();

    let list_request = ListInstalledRequest { base_dir: base_dir.clone() };

    match manager.list_installed(list_request) {
        Ok(list) => {
            if list.versions.is_empty() {
                ui.warning("No installed Go versions found");
                ui.hint("Use 'gvm install <version>' to install a new version");
            } else {
                // List versions directly without a title
                for version in &list.versions {
                    ui.list_item(&version.version, version.is_current);
                }
                // Show total count only if there are multiple versions
                if list.versions.len() > 1 {
                    ui.info(&format!("Total: {} versions", list.versions.len()));
                }
            }
        }
        Err(e) => {
            ui.error(&format!("Failed to list versions: {e}"));
        }
    }

    Ok(())
}

/// List available Go versions from remote.
fn list_available_versions() -> Result<()> {
    let ui = SimpleUI::new();
    let manager = GoManager::new();

    match manager.list_available() {
        Ok(list) => {
            if list.versions.is_empty() {
                ui.warning("No available Go versions found");
            } else {
                ui.section("Available Go Versions");
                for version in &list.versions {
                    ui.list_item(&version.version, version.is_current);
                }
                ui.newline();
                ui.info(&format!("Total: {} versions", list.total_count));
            }
            ui.newline();
            ui.info("Visit https://go.dev/dl/ for a full list of versions");
            ui.hint("Use 'gvm install <version>' to install");
        }
        Err(e) => {
            ui.error(&format!("Failed to fetch available versions: {e}"));
        }
    }

    Ok(())
}

/// Show detailed information about a Go version.
///
/// # Errors
/// Returns an error if the info operation fails.
pub fn info(version: &str, config: &Config) -> Result<()> {
    let ui = SimpleUI::new();
    let manager = GoManager::new();
    let install_dir = config.versions();
    let cache_dir = config.cache();

    match manager.get_version_info(version, install_dir, cache_dir) {
        Ok(info) => {
            // Simplified output, showing only key information
            ui.info(&format!("Go {} ({}-{})", info.version, info.os, info.arch));

            if info.is_installed {
                ui.success("Installed");
                ui.hint(&format!("To use: gvm use {version}"));
            } else {
                ui.warning("Not installed");
                ui.hint(&format!("To install: gvm install {version}"));
            }
        }
        Err(e) => {
            ui.error(&format!("Failed to get info for Go {version}: {e}"));
            ui.hint("Please ensure the version exists and is accessible");
        }
    }

    Ok(())
}
