mod cli;
mod commands;
mod config;
mod ui;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging environment, enable detailed logs only in DEBUG mode
    if std::env::var("RUST_LOG").is_err() {
        if cfg!(debug_assertions) {
            unsafe {
                std::env::set_var("RUST_LOG", "debug");
            }
        } else {
            unsafe {
                std::env::set_var("RUST_LOG", "warn");
            }
        }
    }
    env_logger::init();

    let cli = Cli::parse();

    // Initialize configuration from environment variables and defaults
    let config = Config::new()?;

    // Ensure configuration directories exist
    config.ensure_directories()?;

    match cli.command {
        cli::Commands::Install { version, force } => {
            commands::install(&version, &config, force).await?;
        }
        cli::Commands::Uninstall { version } => {
            commands::uninstall(&version, &config)?;
        }
        cli::Commands::List { available } => {
            commands::list(available, &config).await?;
        }
        cli::Commands::Status => {
            commands::status(&config)?;
        }
        cli::Commands::Info { version } => {
            commands::info(&version, &config).await?;
        }
    }

    Ok(())
}
