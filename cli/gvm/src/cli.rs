use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "gvm")]
#[command(about = "Go Version Manager - Simple and fast Go version switching")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(author = "Tidepool Team")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Install and switch to a Go version (checks version dir, then cache, downloads if needed)
    Install {
        /// Go version to install and use (e.g., 1.21.3, latest)
        version: String,
        /// Force reinstall if version already exists
        #[arg(short, long)]
        force: bool,
    },

    /// Uninstall a Go version
    Uninstall {
        /// Go version to uninstall
        version: String,
    },

    /// List installed Go versions
    List {
        /// Show available versions (not installed)
        #[arg(short, long)]
        available: bool,
    },
    /// Show current Go version and environment
    Status,
    /// Show detailed information about a Go version
    Info {
        /// Go version to get information about (e.g., 1.21.3)
        version: String,
    },
}
