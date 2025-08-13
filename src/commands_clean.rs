use crate::config::Config;
use crate::progress::install_with_fallback;
use crate::ui::{Messages, ProgressManager, UI};
use crate::{
    GoManager, InstallRequest, ListInstalledRequest, StatusRequest, SwitchRequest, UninstallRequest,
};
use anyhow::Result;

/// Install a Go version using enhanced installation system.
///
/// # Errors
/// Returns an error if the installation fails, network issues occur, or file system operations fail.
pub async fn install(version: &str, config: &Config, force: bool) -> Result<()> {
    install_with_fallback(version, config, force).await
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
                if entry.path().is_dir() && !name.starts_with('.') {
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
pub async fn uninstall(version: &str, config: &Config) -> Result<()> {
    let ui = UI::new();
    let manager = GoManager::new();
    let base_dir = config.versions();

    let uninstall_request = UninstallRequest {
        version: version.to_string(),
        base_dir: base_dir.clone(),
    };

    ui.spinner(&format!("Uninstalling Go {version}"));

    match manager.uninstall_with_request(&uninstall_request).await {
        Ok(_) => {
            ui.success(&format!("Successfully uninstalled Go {version}"));
        }
        Err(e) => {
            ui.display_error_with_suggestion(
                &format!("Failed to uninstall Go {}: {}", version, e),
                "Check if the version is currently in use or if you have proper permissions",
            );
        }
    }

    Ok(())
}

/// Switch to a specific Go version.
///
/// # Errors
/// Returns an error if the switch operation fails.
pub async fn switch(version: &str, config: &Config, global: bool, force: bool) -> Result<()> {
    let ui = UI::new();
    let manager = GoManager::new();
    let base_dir = config.versions();

    let switch_request = SwitchRequest {
        version: version.to_string(),
        base_dir: base_dir.clone(),
        global,
        force,
    };

    ui.spinner(&format!("Switching to Go {version}"));

    match manager.switch_with_request(&switch_request).await {
        Ok(_) => {
            ui.success(&format!(
                "Switched to Go {version} {}",
                if global { "globally" } else { "locally" }
            ));
        }
        Err(e) => {
            ui.display_error_with_suggestion(
                &format!("Failed to switch to Go {}: {}", version, e),
                "Make sure the version is installed and you have proper permissions",
            );
        }
    }

    Ok(())
}

/// Show the current Go version status.
///
/// # Errors
/// Returns an error if the status operation fails.
pub async fn status(config: &Config) -> Result<()> {
    let ui = UI::new();
    let manager = GoManager::new();
    let base_dir = config.versions();

    let status_request = StatusRequest { base_dir };

    match manager.status_with_request(&status_request).await {
        Ok(status) => {
            ui.display_status(&status);
        }
        Err(e) => {
            ui.display_error_with_suggestion(
                &format!("Failed to get status: {}", e),
                "Check if Go is properly installed",
            );
        }
    }

    Ok(())
}

/// List Go versions, either installed or available online.
///
/// # Errors
/// Returns an error if the listing operation fails.
pub async fn list(config: &Config, all: bool) -> Result<()> {
    if all {
        list_available_versions().await
    } else {
        list_installed_versions(config).await
    }
}

/// List installed Go versions.
async fn list_installed_versions(config: &Config) -> Result<()> {
    let ui = UI::new();
    let manager = GoManager::new();
    let base_dir = config.versions();

    let list_request = ListInstalledRequest { base_dir: base_dir.clone() };

    match manager.list_installed(list_request).await {
        Ok(list) => {
            if list.versions.is_empty() {
                ui.warning(&Messages::no_go_versions_found());
                ui.info(&Messages::installation_directory_not_found(
                    &base_dir.display().to_string(),
                ));
            } else {
                ui.display_version_list(&list);
            }
        }
        Err(e) => {
            ui.display_error_with_suggestion(
                &format!("Failed to list installed versions: {}", e),
                "Check if the installation directory exists and is accessible",
            );
        }
    }

    Ok(())
}

/// List available Go versions from remote.
async fn list_available_versions() -> Result<()> {
    let ui = UI::new();
    let progress = ProgressManager::new();
    
    progress.start("Fetching available Go versions");

    match crate::go::list_available_versions().await {
        Ok(versions) => {
            progress.finish();
            ui.display_available_versions(&versions);
        }
        Err(e) => {
            progress.fail();
            ui.display_error_with_suggestion(
                &format!("Failed to fetch available versions: {}", e),
                "Check your internet connection",
            );
        }
    }

    Ok(())
}

/// Show detailed information about a Go version.
///
/// # Errors
/// Returns an error if the info operation fails.
pub async fn info(version: &str, config: &Config) -> Result<()> {
    let ui = UI::new();
    let manager = GoManager::new();
    let install_dir = config.versions();
    let cache_dir = config.cache();

    match manager.get_version_info(version, install_dir, cache_dir).await {
        Ok(info) => {
            ui.display_version_info(&info);
        }
        Err(e) => {
            ui.display_error_with_suggestion(
                &format!("Failed to get information for Go {}: {}", version, e),
                "Make sure the version exists and is accessible",
            );
        }
    }

    Ok(())
}
