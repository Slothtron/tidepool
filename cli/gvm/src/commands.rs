use crate::config::Config;
use crate::ui::{Messages, UI};
use anyhow::{Context, Result};
use std::{env, fs, path::Path};
use tidepool_version_manager::{
    go::GoManager, InstallRequest, ListInstalledRequest, StatusRequest, SwitchRequest,
    UninstallRequest, VersionManager,
};

pub async fn install(version: &str, config: &Config, force: bool) -> Result<()> {
    let ui = UI::new();
    let manager = GoManager::new();
    let install_dir = config.versions();
    let cache_dir = config.cache();

    println!("{}", Messages::installing_go(version));

    // Check if the version directory already exists
    let version_dir = install_dir.join(version);
    if version_dir.exists() && !force {
        ui.info(&format!("Go {} is already installed, switching to it", version));

        // 直接切换到已存在的版本
        let switch_request = SwitchRequest {
            version: version.to_string(),
            base_dir: install_dir.clone(),
            global: false,
            force: false,
        };

        return switch_to_existing_version(&manager, &ui, switch_request);
    }
    if force && version_dir.exists() {
        println!("{}", Messages::removing_existing_installation(version));
        fs::remove_dir_all(&version_dir).ok();
    }

    ui.kv_pair_colored("Install Directory", &install_dir.display().to_string(), "dimmed");
    ui.kv_pair_colored("Cache Directory", &cache_dir.display().to_string(), "dimmed");

    // 检查缓存文件是否存在 - 支持跨平台文件名
    let (os, arch) = if cfg!(target_os = "windows") {
        ("windows", if cfg!(target_arch = "x86_64") { "amd64" } else { "386" })
    } else if cfg!(target_os = "macos") {
        ("darwin", if cfg!(target_arch = "x86_64") { "amd64" } else { "arm64" })
    } else {
        ("linux", if cfg!(target_arch = "x86_64") { "amd64" } else { "386" })
    };
    let extension = if cfg!(target_os = "windows") { "zip" } else { "tar.gz" };
    let archive_name = format!("go{}.{}-{}.{}", version, os, arch, extension);
    let cached_file = cache_dir.join(&archive_name);

    // 如果强制安装，删除现有缓存文件
    if force && cached_file.exists() {
        ui.info(&format!("Force mode: removing cached file for Go {}", version));
        fs::remove_file(&cached_file).ok();
    }

    if cached_file.exists() && !force {
        ui.info(&Messages::found_cached_download(version));
        // 从缓存解压安装
        return install_from_cache(version, &cached_file, &version_dir, &manager, &ui).await;
    } // 缓存和版本目录都不存在，需要下载
    ui.info(&format!("Go {} not found in cache, downloading...", version));

    // 确保目录存在
    fs::create_dir_all(install_dir)
        .with_context(|| format!("Failed to create directory: {}", install_dir.display()))?;
    fs::create_dir_all(cache_dir)
        .with_context(|| format!("Failed to create cache directory: {}", cache_dir.display()))?; // 下载并安装
    download_and_install(version, install_dir, cache_dir, &manager, &ui, force).await
}

pub fn uninstall(version: &str, config: &Config) -> Result<()> {
    let ui = UI::new();
    let manager = GoManager::new();
    let base_dir = config.versions();

    println!("{}", Messages::uninstalling_go(version));

    let uninstall_request =
        UninstallRequest { version: version.to_string(), base_dir: base_dir.clone() };

    match manager.uninstall(uninstall_request) {
        Ok(()) => {
            ui.success(&Messages::go_uninstalled_successfully(version));
        }
        Err(e) => {
            if e.contains("not installed") {
                ui.warning(&Messages::go_not_installed(version));
            } else if e.contains("currently active") {
                // 处理当前版本卸载错误，提供友好的提示
                ui.warning(&Messages::cannot_uninstall_current_version(version));
                ui.info(&Messages::clear_current_symlink_hint());
                return Err(anyhow::anyhow!("Cannot uninstall currently active version"));
            } else {
                ui.error(&Messages::uninstall_failed(version, &e.to_string()));
                return Err(anyhow::anyhow!("Uninstall failed: {}", e));
            }
        }
    }

    Ok(())
}

pub async fn list(show_available: bool, config: &Config) -> Result<()> {
    if show_available {
        list_available_versions().await?;
    } else {
        list_installed_versions(config)?;
    }
    Ok(())
}

fn list_installed_versions(config: &Config) -> Result<()> {
    let ui = UI::new();
    let manager = GoManager::new();
    let base_dir = config.versions();

    let list_request = ListInstalledRequest { base_dir: base_dir.clone() };

    match manager.list_installed(list_request) {
        Ok(version_list) => {
            if version_list.versions.is_empty() {
                if !base_dir.exists() {
                    ui.warning(&Messages::installation_directory_not_found(
                        &base_dir.display().to_string(),
                    ));
                } else {
                    ui.warning(&Messages::no_go_versions_found());
                }
                ui.hint(&Messages::install_version_hint());
            } else {
                // 获取当前使用的版本
                let current_version = manager.get_current_version(base_dir);

                ui.display_version_list_with_current(
                    &version_list,
                    &Messages::installed_go_versions(),
                    current_version.as_deref(),
                );
                ui.hint(&Messages::use_version_hint());
            }
        }
        Err(e) => {
            ui.error(&Messages::error_listing_versions(&e.to_string()));
        }
    }

    Ok(())
}

async fn list_available_versions() -> Result<()> {
    let ui = UI::new();
    let manager = GoManager::new();

    match manager.list_available().await {
        Ok(version_list) => {
            ui.display_version_list(&version_list, &Messages::available_go_versions());
        }
        Err(e) => {
            ui.warning(&Messages::error_getting_available_versions(&e.to_string()));
        }
    }
    ui.newline();
    ui.hint(&Messages::visit_go_website());
    ui.hint(&Messages::install_with_hint());
    Ok(())
}

pub fn status(config: &Config) -> Result<()> {
    let ui = UI::new();
    let manager = GoManager::new();
    let base_dir = config.versions();

    let status_request = StatusRequest { base_dir: Some(base_dir.clone()) };

    match manager.status(status_request) {
        Ok(runtime_status) => {
            // 显示当前版本信息（精简版）
            if let Some(ref version) = runtime_status.current_version {
                ui.kv_pair_colored("Current Version", version, "green");

                if let Some(ref path) = runtime_status.install_path {
                    ui.kv_pair_colored("Install Path", &path.display().to_string(), "dimmed");
                }
            } else {
                ui.kv_pair_colored("Current Version", "None", "yellow");
                ui.hint("Use 'gvm install <version>' to install a Go version");
                return Ok(());
            }

            // 检查Go命令是否可用
            if let Ok(output) = std::process::Command::new("go").arg("version").output() {
                if output.status.success() {
                    let version_output = String::from_utf8_lossy(&output.stdout);
                    ui.kv_pair("Go Command", version_output.trim());
                } else {
                    ui.kv_pair_colored("Go Command", "Failed to execute", "red");
                }
            } else {
                ui.kv_pair_colored("Go Command", "Not available", "red");
                ui.hint("💡 Restart your terminal to apply environment changes");
            }
        }
        Err(e) => {
            ui.error(&Messages::error_getting_status(&e.to_string()));

            // 简化的备用检查
            match env::var("GOROOT") {
                Ok(goroot) => ui.kv_pair("GOROOT", &goroot),
                Err(_) => ui.kv_pair_colored("GOROOT", "Not set", "yellow"),
            }
        }
    }

    Ok(())
}

/// Display detailed information about a specified Go version
pub async fn info(version: &str, config: &Config) -> Result<()> {
    let ui = UI::new();
    let manager = GoManager::new();
    let install_dir = config.versions();
    let cache_dir = config.cache();

    ui.header(&format!("Go {} Information", version));

    match manager.get_version_info(version, install_dir, cache_dir).await {
        Ok(info) => {
            ui.display_version_info(&info);
            Ok(())
        }
        Err(e) => {
            ui.error(&format!("Failed to get version information: {}", e));
            Err(anyhow::anyhow!("Failed to get version information: {}", e))
        }
    }
}

fn switch_to_existing_version(
    manager: &GoManager,
    ui: &UI,
    switch_request: SwitchRequest,
) -> Result<()> {
    let version = switch_request.version.clone();
    let base_dir = switch_request.base_dir.clone();

    match manager.switch_to(switch_request) {
        Ok(()) => {
            ui.success(&Messages::switched_to_go_successfully(&version));

            // 显示软链接/Junction信息
            let symlink_info = manager.get_symlink_info(&base_dir);
            if !symlink_info.is_empty() {
                ui.info(&symlink_info);
            }

            // 显示环境变量配置说明
            let install_path = base_dir.join(&version);
            ui.show_environment_setup(&install_path, &version);
        }
        Err(e) => {
            ui.error(&Messages::switch_failed(&e.to_string()));

            // 提供更详细的错误信息和解决方案
            if e.contains("administrator privileges") || e.contains("symlink") {
                ui.newline();
                ui.hint("💡 解决方案:");
                ui.hint("   1. 以管理员身份运行: 右键点击终端，选择'以管理员身份运行'");
                ui.hint("   2. 启用开发者模式: 设置 > 更新和安全 > 开发者选项 > 开发者模式");
            }

            return Err(anyhow::anyhow!("Go version switch failed: {}", e));
        }
    }
    Ok(())
}

/// Helper function to install from cached archive
async fn install_from_cache(
    version: &str,
    cached_file: &Path,
    version_dir: &Path,
    manager: &GoManager,
    ui: &UI,
) -> Result<()> {
    ui.info(&format!("Extracting Go {} from cache...", version));

    // 确保版本目录存在
    fs::create_dir_all(version_dir).with_context(|| {
        format!("Failed to create version directory: {}", version_dir.display())
    })?;

    // 解压缓存的文件
    manager
        .extract_archive(cached_file, version_dir)
        .map_err(|e| anyhow::anyhow!("Failed to extract archive: {}", e))?;

    ui.success(&format!("Go {} extracted successfully from cache", version));

    // 切换到新安装的版本
    let switch_request = SwitchRequest {
        version: version.to_string(),
        base_dir: version_dir.parent().unwrap().to_path_buf(),
        global: false,
        force: false,
    };

    switch_to_existing_version(manager, ui, switch_request)
}

/// Helper function to download and install
async fn download_and_install(
    version: &str,
    install_dir: &Path,
    cache_dir: &Path,
    manager: &GoManager,
    ui: &UI,
    force: bool,
) -> Result<()> {
    // 创建安装请求使用原来的逻辑
    let install_request = InstallRequest {
        version: version.to_string(),
        install_dir: install_dir.to_path_buf(),
        download_dir: cache_dir.to_path_buf(),
        force,
    };

    match manager.install(install_request).await {
        Ok(version_info) => {
            ui.display_install_result(&version_info);
            // 切换到新安装的版本
            let switch_request = SwitchRequest {
                version: version.to_string(),
                base_dir: install_dir.to_path_buf(),
                global: false,
                force: false,
            };

            switch_to_existing_version(manager, ui, switch_request)
        }
        Err(e) => {
            ui.error(&Messages::installation_failed(&e.to_string()));
            Err(anyhow::anyhow!("Go installation failed: {}", e))
        }
    }
}
