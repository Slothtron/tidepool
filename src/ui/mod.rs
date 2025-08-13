//! UI module for tidepool-gvm
//!
//! This module provides a comprehensive user interface system with:
//! - Rich progress indicators and spinners
//! - Interactive prompts and confirmations
//! - Beautiful display formatting
//! - Consistent theming and styling

pub mod display;
pub mod interactive;
pub mod progress;
pub mod theme;

// Re-export main UI components for easy access
pub use display::{Icons, Messages, UI};
pub use interactive::{ConflictAction, InteractiveUI};
pub use progress::{ProgressManager, ProgressReporter};
pub use theme::{GvmTheme, ThemeManager};

// Type aliases for convenience
pub type UiResult<T> = anyhow::Result<T>;

/// Main UI facade that combines all UI components
pub struct GvmUI {
    pub display: UI,
    pub progress: ProgressManager,
    pub interactive: InteractiveUI,
    pub theme: ThemeManager,
}

impl GvmUI {
    /// Create a new GVM UI instance with default configuration
    pub fn new() -> Self {
        Self {
            display: UI::new(),
            progress: ProgressManager::new(),
            interactive: InteractiveUI::new(),
            theme: ThemeManager::new(),
        }
    }

    /// Create a new GVM UI instance with custom theme
    pub fn with_theme(theme: GvmTheme) -> Self {
        Self {
            display: UI::new(),
            progress: ProgressManager::new(),
            interactive: InteractiveUI::new(),
            theme: ThemeManager::with_theme(theme),
        }
    }
}

impl Default for GvmUI {
    fn default() -> Self {
        Self::new()
    }
}
