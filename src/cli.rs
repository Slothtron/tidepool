use crate::{commands, config::Config};
use clap::{Parser, Subcommand};

/// Tidepool GVM - 高性能 Go 版本管理工具
#[derive(Parser)]
#[command(name = "gvm")]
#[command(about = "高性能 Go 版本管理工具")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 安装指定版本的 Go
    Install {
        /// Go 版本号 (例如: 1.21.3)
        version: String,
        /// 强制重新安装
        #[arg(short, long)]
        force: bool,
    },
    /// 切换到指定版本的 Go
    Switch {
        /// Go 版本号 (例如: 1.21.3)
        version: String,
    },
    /// 卸载指定版本的 Go
    Uninstall {
        /// Go 版本号 (例如: 1.21.3)
        version: String,
    },
    /// 列出已安装的 Go 版本
    List,
    /// 显示当前 Go 版本状态
    Status,
    /// 显示指定版本的详细信息
    Info {
        /// Go 版本号 (例如: 1.21.3)
        version: String,
    },
}

impl Cli {
    pub async fn run(&self) -> anyhow::Result<()> {
        let config = Config::new()?;

        match &self.command {
            Commands::Install { version, force } => {
                commands::install(version, &config, *force).await
            }
            Commands::Switch { version } => {
                commands::switch_to_existing_version(
                    &crate::go::GoManager::new(),
                    &crate::ui::UI::new(),
                    crate::SwitchRequest {
                        version: version.clone(),
                        base_dir: config.versions().clone(),
                        global: false,
                        force: false,
                    },
                )
                .await
            }
            Commands::Uninstall { version } => commands::uninstall(version, &config).await,
            Commands::List => commands::list(false, &config).await,
            Commands::Status => commands::status(&config).await,
            Commands::Info { version } => commands::info(version, &config).await,
        }
    }
}
