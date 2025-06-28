// Library interface for the GVM CLI
// This allows integration tests to access internal modules

pub mod cli;
pub mod commands;
pub mod config;
pub mod ui;

// Re-export commonly used types for convenience
pub use cli::Cli;
pub use config::Config;
pub use ui::UI;
