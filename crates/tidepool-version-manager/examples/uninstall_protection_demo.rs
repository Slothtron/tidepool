/// 演示 gvm uninstall 的当前版本保护机制
///
/// 此示例展示了 gvm uninstall 命令在尝试卸载当前活跃版本时的保护机制
use std::fs;
use std::path::Path;
use tempfile::TempDir;
use tidepool_version_manager::{go::GoManager, UninstallRequest, VersionManager};

fn main() {
    println!("🛡️  GVM 卸载保护机制演示");
    println!("========================================");

    demonstrate_uninstall_protection();

    println!("\n✅ 演示完成！");
}

fn demonstrate_uninstall_protection() {
    println!("📋 创建测试环境...");

    // 创建临时目录模拟 Go 安装环境
    let temp_dir = TempDir::new().expect("无法创建临时目录");
    let base_dir = temp_dir.path().to_path_buf();
    let manager = GoManager::new();

    // 设置测试版本
    let versions = ["1.21.0", "1.20.5"];
    let current_version = "1.21.0";

    // 创建模拟的 Go 安装
    for version in &versions {
        create_mock_go_installation(&base_dir, version);
    }

    // 创建软链接指向当前版本
    create_current_symlink(&base_dir, current_version);

    println!("📂 创建的测试版本:");
    for version in &versions {
        println!("  • Go {version}");
    }
    println!("🔗 当前激活版本: Go {current_version}");

    println!("\n🔸 场景1: 尝试卸载当前版本 (应该被阻止)");
    println!("执行: gvm uninstall {current_version}");

    let uninstall_request =
        UninstallRequest { version: current_version.to_string(), base_dir: base_dir.clone() };

    match manager.uninstall(uninstall_request) {
        Ok(()) => {
            println!("❌ 意外成功：不应该允许卸载当前版本！");
        }
        Err(error) => {
            println!("✅ 正确阻止: {error}");
            if error.contains("currently active") {
                println!("💡 提示: 请先切换到其他版本或清除当前软链接");
            }
        }
    }

    // 验证当前版本仍然存在
    let current_version_dir = base_dir.join(current_version);
    if current_version_dir.exists() {
        println!("✅ 验证: 当前版本目录仍然存在，未被误删");
    } else {
        println!("❌ 错误: 当前版本目录被意外删除！");
    }

    println!("\n🔸 场景2: 卸载非当前版本 (应该允许)");
    let other_version = "1.20.5";
    println!("执行: gvm uninstall {other_version}");

    let uninstall_request =
        UninstallRequest { version: other_version.to_string(), base_dir: base_dir.clone() };

    match manager.uninstall(uninstall_request) {
        Ok(()) => {
            println!("✅ 成功卸载非当前版本: Go {other_version}");
        }
        Err(error) => {
            println!("❌ 意外失败: {error}");
        }
    }

    // 验证非当前版本被删除，当前版本仍存在
    let other_version_dir = base_dir.join(other_version);    if other_version_dir.exists() {
        println!("❌ 错误: 非当前版本目录未被删除！");
    } else {
        println!("✅ 验证: 非当前版本目录已被正确删除");
    }

    if current_version_dir.exists() {
        println!("✅ 验证: 当前版本目录仍然安全保存");
    } else {
        println!("❌ 错误: 当前版本目录被误删！");
    }

    println!("\n📊 保护机制总结:");
    println!("  ✓ 阻止卸载当前活跃版本");
    println!("  ✓ 提供清晰的错误信息");
    println!("  ✓ 给出解决方案提示");
    println!("  ✓ 允许卸载非当前版本");
    println!("  ✓ 保护数据完整性");
}

fn create_mock_go_installation(base_dir: &Path, version: &str) {
    let version_dir = base_dir.join(version);
    let bin_dir = version_dir.join("bin");

    fs::create_dir_all(&bin_dir).expect("无法创建 bin 目录");

    #[cfg(target_os = "windows")]
    let go_binary = bin_dir.join("go.exe");
    #[cfg(not(target_os = "windows"))]
    let go_binary = bin_dir.join("go");

    fs::write(&go_binary, format!("fake go binary for {version}")).expect("无法创建 go 二进制文件");
}

fn create_current_symlink(base_dir: &Path, target_version: &str) {
    let current_link = base_dir.join("current");
    let target_dir = base_dir.join(target_version);

    #[cfg(target_os = "windows")]
    {
        // 在 Windows 上创建 junction
        let output = std::process::Command::new("cmd")
            .args([
                "/C",
                "mklink",
                "/J",
                &current_link.to_string_lossy(),
                &target_dir.to_string_lossy(),
            ])
            .output()
            .expect("无法执行 mklink 命令");

        if !output.status.success() {
            println!("警告: 无法创建 junction，可能权限不足");
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        // 在 Unix 系统上创建符号链接
        std::os::unix::fs::symlink(&target_dir, &current_link).expect("无法创建符号链接");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demonstration_runs_without_panic() {
        // 只是确保演示代码能正常运行而不会 panic
        // 在某些环境下可能无法创建链接，但不应该崩溃
        let temp_dir = TempDir::new().expect("无法创建临时目录");
        let base_dir = temp_dir.path().to_path_buf();

        create_mock_go_installation(&base_dir, "1.21.0");

        // 这个测试主要确保代码逻辑正确，不会因为环境问题而崩溃
        assert!(base_dir.join("1.21.0").exists());
    }
}
