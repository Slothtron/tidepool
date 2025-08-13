use crate::config::Config;
use crate::ui::{Messages, UI};
use crate::{GoManager, InstallRequest, SwitchRequest};
use anyhow::Result;
use std::path::Path;
use std::{fs};

/// Install with fallback to standard installation method
///
/// This is a simplified implementation until Phase 3 is fully integrated
pub async fn install_with_fallback(version: &str, config: &Config, force: bool) -> Result<()> {
    let ui = UI::new();
    let manager = GoManager::new();
    let install_dir = config.versions();
    let cache_dir = config.cache();

    println!("{}", Messages::installing_go(version));

    // Check if the version directory already exists
    let version_dir = install_dir.join(version);
    if version_dir.exists() && !force {
        ui.info(&format!("Go {version} is already installed, switching to it"));

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
        println!("{}", Messages::removing_existing_installation(version));
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
        ).await;
    }

    // 确保目录存在
    fs::create_dir_all(install_dir)
        .map_err(|e| anyhow::anyhow!("Failed to create directory: {}", e))?;
    fs::create_dir_all(cache_dir)
        .map_err(|e| anyhow::anyhow!("Failed to create cache directory: {}", e))?;

    // 下载并安装
    download_and_install(
        version, 
        install_dir, 
        cache_dir, 
        &manager, 
        &ui,
        force
    )
    .await
}

/// Switch to an existing version.
async fn switch_to_existing_version(
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

/// Install from cached file.
async fn install_from_cache(
    version: &str,
    cached_file: &Path,
    version_dir: &Path,
    manager: &GoManager,
    ui: &UI,
) -> Result<()> {
    ui.progress("Extracting archive from cache");
    
    // Use the standard installer
    let request = InstallRequest {
        version: version.to_string(),
        install_dir: version_dir.parent().unwrap().to_path_buf(),
        download_dir: cached_file.parent().unwrap().to_path_buf(),
        force: false,
    };

    match manager.install(request).await {
        Ok(_) => {
            ui.progress_done("Successfully installed from cache");
            ui.success(&format!("Successfully installed Go {version} from cache"));
            Ok(())
        }
        Err(e) => {
            ui.error(&Messages::installation_failed(&e.to_string()));
            Err(e)
        }
    }
}

/// Download and install a Go version.
async fn download_and_install(
    version: &str,
    install_dir: &Path,
    cache_dir: &Path,
    manager: &GoManager,
    ui: &UI,
    force: bool,
) -> Result<()> {
    ui.progress("Downloading Go archive");
    
    let request = InstallRequest {
        version: version.to_string(),
        install_dir: install_dir.to_path_buf(),
        download_dir: cache_dir.to_path_buf(),
        force,
    };

    match manager.install(request).await {
        Ok(_) => {
            ui.progress_done("Download and installation completed");
            ui.success(&format!("Successfully installed Go {version}"));
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
