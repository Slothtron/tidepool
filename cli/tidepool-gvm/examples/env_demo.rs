#!/usr/bin/env rust-script

// 这是一个演示程序，展示不同操作系统下的环境变量配置说明
use std::path::{Path, PathBuf};

// 简化的UI结构，用于演示
struct DemoUI;

impl DemoUI {
    fn new() -> Self {
        Self
    }
    fn header(text: &str) {
        println!("\n{text}");
        println!("{}", "=".repeat(text.len()));
    }

    fn info(message: &str) {
        println!("ℹ️  {message}");
    }

    fn list_item(icon: &str, text: &str) {
        println!("{icon} {text}");
    }

    fn hint(message: &str) {
        println!("💡 {message}");
    }

    fn newline() {
        println!();
    }

    // 从主项目复制的方法
    fn show_environment_setup(&self, install_path: &Path, version: &str) {
        DemoUI::newline();
        DemoUI::header("📋 环境变量配置说明");

        let bin_path = install_path.join("bin");
        let go_root = install_path;

        if cfg!(target_os = "windows") {
            self.show_windows_env_setup(&bin_path, go_root, version);
        } else {
            self.show_unix_env_setup(&bin_path, go_root, version);
        }

        DemoUI::newline();
        DemoUI::hint(&format!("💡 切换完成！现在可以使用 Go {version} 了"));
        DemoUI::hint("   运行 'go version' 验证当前版本");
    }

    #[allow(clippy::unused_self)]
    fn show_windows_env_setup(&self, bin_path: &Path, go_root: &Path, version: &str) {
        DemoUI::info(&format!("已切换到 Go {version}，以下是环境变量配置说明："));
        DemoUI::newline();

        DemoUI::list_item("🔷", "PowerShell 临时配置（当前会话）:");
        println!("    $env:GOROOT = \"{}\"", go_root.display());
        println!("    $env:PATH = \"{};$env:PATH\"", bin_path.display());
        DemoUI::newline();

        DemoUI::list_item("🔷", "PowerShell 永久配置（添加到 $PROFILE）:");
        println!("    $env:GOROOT = \"{}\"", go_root.display());
        println!("    $env:PATH = \"{};$env:PATH\"", bin_path.display());
        DemoUI::newline();

        DemoUI::list_item("🔶", "命令提示符(CMD) 临时配置:");
        println!("    set GOROOT={}", go_root.display());
        println!("    set PATH={};%PATH%", bin_path.display());
        DemoUI::newline();

        DemoUI::list_item("⚙️", "系统环境变量配置（推荐）:");
        DemoUI::hint("   1. 右键'此电脑' → 属性 → 高级系统设置");
        DemoUI::hint("   2. 点击'环境变量'按钮");
        DemoUI::hint(&format!("   3. 新建 GOROOT = {}", go_root.display()));
        DemoUI::hint(&format!("   4. 编辑 PATH，添加 {}", bin_path.display()));
        DemoUI::hint("   5. 重启终端生效");
    }

    #[allow(clippy::unused_self)]
    fn show_unix_env_setup(&self, bin_path: &Path, go_root: &Path, version: &str) {
        DemoUI::info(&format!("已切换到 Go {version}，以下是环境变量配置说明："));
        DemoUI::newline();

        DemoUI::list_item("🟢", "当前会话临时配置:");
        println!("    export GOROOT=\"{}\"", go_root.display());
        println!("    export PATH=\"{}:$PATH\"", bin_path.display());
        DemoUI::newline();

        let shell = std::env::var("SHELL").unwrap_or_default();
        let (shell_name, config_file) = if shell.contains("zsh") {
            ("Zsh", "~/.zshrc")
        } else if shell.contains("fish") {
            ("Fish", "~/.config/fish/config.fish")
        } else if shell.contains("nu") {
            ("NuShell", "~/.config/nushell/config.nu")
        } else {
            ("Bash", "~/.bashrc 或 ~/.bash_profile")
        };

        DemoUI::list_item("🟢", &format!("{shell_name} 永久配置（添加到 {config_file}）:"));

        if shell.contains("fish") {
            println!("    set -gx GOROOT \"{}\"", go_root.display());
            println!("    set -gx PATH \"{}\" $PATH", bin_path.display());
        } else if shell.contains("nu") {
            println!("    $env.GOROOT = \"{}\"", go_root.display());
            println!("    $env.PATH = ($env.PATH | prepend \"{}\")", bin_path.display());
        } else {
            println!("    export GOROOT=\"{}\"", go_root.display());
            println!("    export PATH=\"{}:$PATH\"", bin_path.display());
        }
        DemoUI::newline();

        DemoUI::list_item("⚡", "立即应用配置:");
        if shell.contains("fish") {
            DemoUI::hint(&format!("   source {config_file}"));
        } else if shell.contains("nu") {
            DemoUI::hint("   重启 NuShell 或重新加载配置");
        } else {
            DemoUI::hint(&format!("   source {config_file}"));
        }

        if cfg!(target_os = "macos") {
            DemoUI::newline();
            DemoUI::list_item("🍎", "macOS 用户注意:");
            DemoUI::hint("   如果使用 Terminal.app，配置文件可能是 ~/.bash_profile");
            DemoUI::hint("   如果使用 iTerm2 + Zsh，配置文件是 ~/.zshrc");
        }
    }
}

fn main() {
    let ui = DemoUI::new();

    println!("🎯 GVM 环境变量配置说明演示");
    println!("当前操作系统: {}", std::env::consts::OS);
    println!("当前架构: {}", std::env::consts::ARCH);

    // 演示不同路径和版本的配置说明
    let test_cases = vec![
        (PathBuf::from("/home/user/.gvm/versions/1.21.0"), "1.21.0"),
        (PathBuf::from("/opt/go/1.20.5"), "1.20.5"),
    ];

    for (install_path, version) in test_cases {
        ui.show_environment_setup(&install_path, version);
        println!("\n{}", "─".repeat(50));
    }
}
