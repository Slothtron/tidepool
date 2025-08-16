//! Command line interface definition
use crate::{commands, config::Config};
use clap::{Parser, Subcommand};

/// Tidepool GVM - A high-performance Go Version Manager
#[derive(Parser, Debug)]
#[command(author, version, about = "A high-performance Go version management tool")]
pub struct Cli {
    /// Verbose mode
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Quiet mode (only output errors)
    #[arg(short, long, global = true)]
    pub quiet: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Install a specific Go version
    Install {
        /// The Go version to install (e.g., 1.21.3)
        version: String,
        /// Force re-installation
        #[arg(short, long)]
        force: bool,
    },
    /// Switch to a specific Go version
    Use {
        /// The Go version to use (e.g., 1.21.3)
        version: String,
        /// Set as global version
        #[arg(short, long)]
        global: bool,
    },
    /// Uninstall a specific Go version
    Uninstall {
        /// The Go version to uninstall (e.g., 1.21.3)
        version: String,
    },
    /// List Go versions
    List {
        /// List all available remote versions
        #[arg(short, long)]
        all: bool,
    },
    /// Show the current Go version status
    Status,
    /// Show detailed information about a Go version
    Info {
        /// The Go version to show information for (e.g., 1.21.3)
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
            Commands::Use { version, global } => commands::switch(version, &config, *global, false),
            Commands::Uninstall { version } => commands::uninstall(version, &config),
            Commands::List { all } => commands::list(&config, *all),
            Commands::Status => commands::status(&config),
            Commands::Info { version } => commands::info(version, &config),
        }
    }
}
