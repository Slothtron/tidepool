use crate::config::Config;
use crate::ui::{Messages, ProgressManager, UI};
use crate::{
    GoManager, InstallRequest, ListInstalledRequest, StatusRequest, SwitchRequest, UninstallRequest,
};
use anyhow::{Context, Result};
use std::{fs, path::Path};

/// Install a Go version.
///
/// # Errors
/// Returns an error if the installation fails, network issues occur, or file system operations fail.
pub async fn install(version: &str, config: &Config, force: bool) -> Result<()> {
    let ui = UI::new();
    let progress_manager = ProgressManager::new();
    let manager = GoManager::new();
    let install_dir = config.versions();
    let cache_dir = config.cache();

    ui.display_install_start(version);

    // Check if the version directory already exists
    let version_dir = install_dir.join(version);
    if version_dir.exists() && !force {
        ui.info(&format!("Go {version} is already installed"));
        ui.suggest("Switching to existing installation");

        // 直接切换到已存在的版本
        let switch_request = SwitchRequest {
            version: version.to_string(),
            base_dir: install_dir.clone(),
            global: false,
            force: false,
        };

        return switch_to_existing_version(&manager, &ui, switch_request).await;
    }
    if force && version_dir.exists() {
        ui.warning(&Messages::removing_existing_installation(version));
        fs::remove_dir_all(&version_dir).ok();
    }

    ui.kv_pair_colored("Install Directory", &install_dir.display().to_string(), "dimmed");
    ui.kv_pair_colored("Cache Directory", &cache_dir.display().to_string(), "dimmed");
    ui.separator();

    // 检查缓存文件是否存在 - 支持跨平台文件名
    let (os, arch) = if cfg!(target_os = "windows") {
        ("windows", if cfg!(target_arch = "x86_64") { "amd64" } else { "386" })
    } else if cfg!(target_os = "macos") {
        ("darwin", if cfg!(target_arch = "x86_64") { "amd64" } else { "arm64" })
    } else {
        ("linux", if cfg!(target_arch = "x86_64") { "amd64" } else { "386" })
    };
    let extension = if cfg!(target_os = "windows") { "zip" } else { "tar.gz" };
    let archive_name = format!("go{version}.{os}-{arch}.{extension}");
    let cached_file = cache_dir.join(&archive_name);

    // 如果强制安装，删除现有缓存文件
    if force && cached_file.exists() {
        ui.warning(&format!("Force mode: removing cached file for Go {version}"));
        fs::remove_file(&cached_file).ok();
    }

    if cached_file.exists() && !force {
        ui.display_cache_info(&Messages::found_cached_download(version));
        // 从缓存解压安装
        return install_from_cache(
            version,
            &cached_file,
            &version_dir,
            &manager,
            &ui,
            &progress_manager,
        )
        .await;
    } // 缓存和版本目录都不存在，需要下载
    let download_spinner = progress_manager.new_spinner(&Messages::download_progress());
    download_spinner.finish_with_message("Ready to download");

    // 确保目录存在
    fs::create_dir_all(install_dir)
        .with_context(|| format!("Failed to create directory: {}", install_dir.display()))?;
    fs::create_dir_all(cache_dir)
        .with_context(|| format!("Failed to create cache directory: {}", cache_dir.display()))?; // 下载并安装
    download_and_install(version, install_dir, cache_dir, &manager, &ui, &progress_manager, force)
        .await
}

/// Uninstall a Go version.
///
/// # Errors
/// Returns an error if the uninstallation fails or file system operations fail.
pub async fn uninstall(version: &str, config: &Config) -> Result<()> {
    let ui = UI::new();
    let manager = GoManager::new();
    let base_dir = config.versions();

    ui.display_uninstall_start(version);

    let uninstall_request =
        UninstallRequest { version: version.to_string(), base_dir: base_dir.clone() };

    match manager.uninstall(uninstall_request).await {
        Ok(()) => {
            ui.display_uninstall_success(version);
        }
        Err(e) => {
            if e.to_string().contains("not installed") {
                ui.display_error_with_suggestion(
                    &Messages::go_not_installed(version),
                    "List installed versions: gvm list",
                );
            } else if e.to_string().contains("currently active") {
                ui.display_error_with_suggestion(
                    &Messages::cannot_uninstall_current_version(version),
                    &Messages::clear_current_symlink_hint(),
                );
            } else {
                ui.display_error_with_suggestion(
                    &Messages::uninstall_failed(version, &e.to_string()),
                    "Check if the version exists: gvm list",
                );
            }
        }
    }

    Ok(())
}

/// List installed or available Go versions.
///
/// # Errors
/// Returns an error if the listing operation fails.
pub async fn list(show_available: bool, config: &Config) -> Result<()> {
    if show_available {
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
                ui.suggest(&Messages::install_version_hint());
            } else {
                // Get current version
                let current_version = manager.get_current_version(base_dir);
                ui.display_version_list_with_current(
                    &list,
                    &Messages::installed_go_versions(),
                    current_version.as_deref(),
                );
            }
        }
        Err(e) => {
            ui.display_error_with_suggestion(
                &Messages::error_listing_versions(&e.to_string()),
                "Check your installation directory or install a version first",
            );
        }
    }

    Ok(())
}

/// List available Go versions.
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
            ui.suggest(&Messages::install_with_hint());
        }
        Err(e) => {
            ui.display_error_with_suggestion(
                &Messages::error_getting_available_versions(&e.to_string()),
                "Check your internet connection and try again",
            );
        }
    }

    Ok(())
}

/// Show current Go version and environment status.
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
            ui.display_status(&status, base_dir, config);
        }
        Err(e) => {
            ui.display_error_with_suggestion(
                &Messages::error_getting_status(&e.to_string()),
                "Check your GVM installation and configuration",
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
                "Verify the version number or check available versions: gvm list",
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
            ui.display_switch_success(&version);
        }
        Err(e) => {
            ui.display_error_with_suggestion(
                &Messages::switch_failed(&e.to_string()),
                "Check if the version is installed: gvm list",
            );
        }
    }

    Ok(())
}

/// Install from cached file.
async fn install_from_cache(
    version: &str,
    cached_file: &Path,
    version_dir: &Path,
    manager: &GoManager,
    ui: &UI,
    progress_manager: &ProgressManager,
) -> Result<()> {
    let extraction_spinner = progress_manager.new_spinner(&Messages::extraction_progress());

    let install_request = InstallRequest {
        version: version.to_string(),
        install_dir: version_dir.to_path_buf(),
        download_dir: cached_file.parent().unwrap().to_path_buf(),
        force: false,
    };

    match manager.install(install_request).await {
        Ok(version_info) => {
            extraction_spinner.finish_with_message("Extraction completed");
            ui.display_install_result(&version_info);
        }
        Err(e) => {
            extraction_spinner.abandon_with_message("Extraction failed");
            ui.display_error_with_suggestion(
                &Messages::installation_failed(&e.to_string()),
                "Try installing with --force flag or check your internet connection",
            );
        }
    }

    Ok(())
}

/// Download and install a Go version.
async fn download_and_install(
    version: &str,
    install_dir: &Path,
    cache_dir: &Path,
    manager: &GoManager,
    ui: &UI,
    progress_manager: &ProgressManager,
    force: bool,
) -> Result<()> {
    let install_request = InstallRequest {
        version: version.to_string(),
        install_dir: install_dir.to_path_buf(),
        download_dir: cache_dir.to_path_buf(),
        force,
    };

    let install_bar = progress_manager.new_install_bar(3);
    install_bar.set_message("Starting installation...");

    match manager.install(install_request).await {
        Ok(version_info) => {
            install_bar.finish_with_message("Installation completed");
            ui.display_install_result(&version_info);
        }
        Err(e) => {
            install_bar.abandon_with_message("Installation failed");
            ui.display_error_with_suggestion(
                &Messages::installation_failed(&e.to_string()),
                "Check your internet connection or try with --force flag",
            );
        }
    }

    Ok(())
}
