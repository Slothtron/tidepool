use crate::config::Config;
use crate::progress::install_with_fallback;
use crate::ui::{Messages, UI};
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
pub async fn uninstall(version: &str, config: &Config) -> Result<()> {
    let ui = UI::new();
    let manager = GoManager::new();
    let base_dir = config.versions();

    println!("{}", Messages::uninstalling_go(version));

    let uninstall_request =
        UninstallRequest { version: version.to_string(), base_dir: base_dir.clone() };

    match manager.uninstall(uninstall_request).await {
        Ok(()) => {
            ui.success(&Messages::go_uninstalled_successfully(version));
        }
        Err(e) => {
            if e.to_string().contains("not installed") {
                ui.warning(&Messages::go_not_installed(version));
            } else if e.to_string().contains("currently active") {
                ui.error(&Messages::cannot_uninstall_current_version(version));
                ui.info(&Messages::clear_current_symlink_hint());
            } else {
                ui.error(&Messages::uninstall_failed(version, &e.to_string()));
            }
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

    match manager.switch_to(switch_request).await {
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

    let status_request = StatusRequest { base_dir: Some(base_dir.clone()) };

    match manager.status(status_request).await {
        Ok(status) => {
            ui.header("📊 Go Version Manager Status");

            // Current version
            if let Some(current_version) = status.current_version {
                ui.success(&format!("Current version: {}", current_version));
            } else {
                ui.warning("No version currently active");
            }

            // Installation path
            if let Some(install_path) = status.install_path {
                ui.kv_pair_colored("Install Path", &install_path.display().to_string(), "dimmed");
            }

            // Environment variables
            ui.newline();
            ui.header("Environment Variables");
            for (key, value) in status.environment_vars {
                ui.kv_pair_colored(&key, &value, "cyan");
            }

            // Link information
            if let Some(link_info) = status.link_info {
                ui.newline();
                ui.kv_pair_colored("Symlink Info", &link_info, "yellow");
            }
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
                ui.hint(&Messages::install_version_hint());
            } else {
                ui.display_version_list(&list, &Messages::installed_go_versions());
            }
        }
        Err(e) => {
            ui.error(&Messages::error_listing_versions(&e.to_string()));
        }
    }

    Ok(())
}

/// List available Go versions from remote.
async fn list_available_versions() -> Result<()> {
    let ui = UI::new();
    let manager = GoManager::new();

    match manager.list_available().await {
        Ok(list) => {
            if list.versions.is_empty() {
                ui.warning(&Messages::no_go_versions_found());
            } else {
                ui.display_version_list(&list, &Messages::available_go_versions());
            }
            ui.info(&Messages::visit_go_website());
            ui.hint(&Messages::install_with_hint());
        }
        Err(e) => {
            ui.error(&Messages::error_getting_available_versions(&e.to_string()));
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

/// Switch to an existing version.
pub async fn switch_to_existing_version(
    manager: &GoManager,
    ui: &UI,
    switch_request: SwitchRequest,
) -> Result<()> {
    let version = switch_request.version.clone();
    match manager.switch_to(switch_request).await {
        Ok(()) => {
            ui.success(&Messages::switched_to_go_successfully(&version));
        }
        Err(e) => {
            ui.error(&Messages::switch_failed(&e.to_string()));
        }
    }

    Ok(())
}
