//! 演示Go版本管理器的Junction Point功能
//!
//! 使用方法: cargo run --example junction_demo

use std::env;
use std::path::PathBuf;
use tidepool_version_manager::go::GoManager;

fn main() {
    println!("🚀 Go版本管理器 - Junction Point演示");

    // 获取基础目录
    let args: Vec<String> = env::args().collect();
    let base_dir = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        // 默认使用当前目录下的go_versions
        env::current_dir().unwrap().join("go_versions")
    };

    println!("📁 基础目录: {}", base_dir.display());

    let manager = GoManager::new();

    // 显示当前状态
    println!("\n📊 当前状态:");
    if let Some(current) = manager.get_current_version(&base_dir) {
        println!("  当前版本: {current}");
    } else {
        println!("  当前版本: 未设置");
    }

    // 显示junction信息
    #[cfg(windows)]
    {
        let junction_info = manager.get_symlink_info(&base_dir);
        println!("  Junction状态: {junction_info}");
    }

    // 显示环境变量
    println!("\n🌐 环境变量:");
    if let Ok(goroot) = env::var("GOROOT") {
        println!("  GOROOT: {goroot}");
    } else {
        println!("  GOROOT: 未设置");
    }

    if let Ok(gopath) = env::var("GOPATH") {
        println!("  GOPATH: {gopath}");
    } else {
        println!("  GOPATH: 未设置");
    }

    println!("\n💡 要测试版本切换，请先安装Go版本到指定目录，然后使用:");
    println!("   cargo run --example junction_demo /path/to/go/versions");
    println!("   然后调用 manager.switch_version_windows(\"版本号\", &base_dir)");
}
