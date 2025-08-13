//! Interactive UI module for tidepool-gvm
//!
//! This module provides interactive user prompts, confirmations, and selection interfaces
//! using the dialoguer library. It handles user input for version selection, dangerous
//! operation confirmations, and conflict resolution workflows.

use anyhow::{anyhow, Result};
use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Confirm, FuzzySelect, Input, MultiSelect, Select};

/// Actions that can be taken when a version conflict occurs
#[derive(Debug, Clone, PartialEq)]
pub enum ConflictAction {
    /// Skip the operation
    Skip,
    /// Overwrite existing version
    Overwrite,
    /// Create backup before overwriting
    Backup,
    /// Cancel the entire operation
    Cancel,
}

impl ConflictAction {
    /// Get human-readable description
    pub fn description(&self) -> &'static str {
        match self {
            ConflictAction::Skip => "Skip installation (keep existing)",
            ConflictAction::Overwrite => "Overwrite existing version",
            ConflictAction::Backup => "Backup existing and install new",
            ConflictAction::Cancel => "Cancel operation",
        }
    }
}

/// Interactive UI component for user prompts and selections
pub struct InteractiveUI {
    theme: ColorfulTheme,
    term: Term,
}

impl InteractiveUI {
    /// Create a new interactive UI instance
    pub fn new() -> Self {
        Self { theme: ColorfulTheme::default(), term: Term::stdout() }
    }

    /// Confirm a dangerous operation with detailed warning
    pub fn confirm_dangerous_operation(&self, action: &str, target: &str) -> Result<bool> {
        println!(
            "\n{} {}",
            style("⚠️").yellow().bold(),
            style("Dangerous Operation").yellow().bold()
        );
        println!("{}", style("─".repeat(40)).dim());
        println!("Action: {}", style(action).red().bold());
        println!("Target: {}", style(target).yellow());
        println!();

        let confirmed = Confirm::with_theme(&self.theme)
            .with_prompt(format!("Are you sure you want to {} {}?", action, target))
            .default(false)
            .show_default(true)
            .wait_for_newline(true)
            .interact_on(&self.term)?;

        Ok(confirmed)
    }

    /// Handle version conflict resolution
    pub fn handle_version_conflict(&self, version: &str) -> Result<ConflictAction> {
        println!("\n{} {}", style("⚠️").yellow().bold(), style("Version Conflict").yellow().bold());
        println!("{}", style("─".repeat(35)).dim());
        println!("Go version {} already exists", style(version).cyan().bold());
        println!();

        let actions = [
            ConflictAction::Skip,
            ConflictAction::Overwrite,
            ConflictAction::Backup,
            ConflictAction::Cancel,
        ];

        let action_descriptions: Vec<String> =
            actions.iter().map(|a| a.description().to_string()).collect();

        let selection = Select::with_theme(&self.theme)
            .with_prompt("How would you like to proceed?")
            .default(0)
            .items(&action_descriptions)
            .interact_on(&self.term)?;

        Ok(actions[selection].clone())
    }

    /// Select multiple versions from a list
    pub fn select_multiple_versions(&self, versions: &[String]) -> Result<Vec<String>> {
        if versions.is_empty() {
            return Err(anyhow!("No versions available for selection"));
        }

        println!("\n{}", style("📦 Select Multiple Versions").cyan().bold());
        println!("{}", style("─".repeat(40)).dim());
        println!(
            "Use {} to select/deselect, {} to confirm",
            style("Space").green(),
            style("Enter").green()
        );
        println!();

        let selections = MultiSelect::with_theme(&self.theme)
            .with_prompt("Choose versions")
            .items(versions)
            .interact_on(&self.term)?;

        let selected_versions: Vec<String> =
            selections.into_iter().map(|i| versions[i].clone()).collect();

        if selected_versions.is_empty() {
            return Err(anyhow!("No versions selected"));
        }

        Ok(selected_versions)
    }

    /// Simple yes/no confirmation
    pub fn confirm_operation(&self, message: &str) -> Result<bool> {
        let confirmed = Confirm::with_theme(&self.theme)
            .with_prompt(message)
            .default(true)
            .show_default(true)
            .interact_on(&self.term)?;

        Ok(confirmed)
    }

    /// Get text input from user
    pub fn get_input(&self, prompt: &str, default: Option<&str>) -> Result<String> {
        let mut input = Input::with_theme(&self.theme).with_prompt(prompt);

        if let Some(default_value) = default {
            input = input.default(default_value.to_string());
        }

        let result = input.interact_text_on(&self.term)?;
        Ok(result)
    }

    /// Select from a list with search functionality
    pub fn select_with_search(&self, prompt: &str, items: &[String]) -> Result<String> {
        if items.is_empty() {
            return Err(anyhow!("No items available for selection"));
        }

        let selection = FuzzySelect::with_theme(&self.theme)
            .with_prompt(prompt)
            .default(0)
            .items(items)
            .interact_on_opt(&self.term)?;

        match selection {
            Some(index) => Ok(items[index].clone()),
            None => Err(anyhow!("No item selected")),
        }
    }

    /// Display a selection menu with custom formatting
    pub fn custom_select<T>(
        &self,
        prompt: &str,
        items: &[T],
        formatter: fn(&T) -> String,
    ) -> Result<usize>
    where
        T: Clone,
    {
        if items.is_empty() {
            return Err(anyhow!("No items available for selection"));
        }

        let formatted_items: Vec<String> = items.iter().map(formatter).collect();

        let selection = Select::with_theme(&self.theme)
            .with_prompt(prompt)
            .default(0)
            .items(&formatted_items)
            .interact_on(&self.term)?;

        Ok(selection)
    }

    /// Ask for network retry on failure
    pub fn ask_retry(&self, error: &str, attempt: u32, max_attempts: u32) -> Result<bool> {
        println!("\n{} {}", style("❌").red(), style("Operation Failed").red().bold());
        println!("{}", style("─".repeat(30)).dim());
        println!("Error: {}", style(error).red());
        println!("Attempt: {} of {}", attempt, max_attempts);
        println!();

        if attempt >= max_attempts {
            println!("{}", style("Maximum retry attempts reached").yellow());
            return Ok(false);
        }

        let retry = Confirm::with_theme(&self.theme)
            .with_prompt("Would you like to retry?")
            .default(true)
            .show_default(true)
            .interact_on(&self.term)?;

        Ok(retry)
    }

    /// Display information message
    pub fn info(&self, message: &str) {
        println!("{} {}", style("ℹ️").blue(), message);
    }

    /// Display success message
    pub fn success(&self, message: &str) {
        println!("{} {}", style("✅").green(), message);
    }

    /// Display warning message
    pub fn warning(&self, message: &str) {
        println!("{} {}", style("⚠️").yellow(), message);
    }

    /// Display error message
    pub fn error(&self, message: &str) {
        println!("{} {}", style("❌").red(), message);
    }
}

impl Default for InteractiveUI {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility functions for interactive operations
pub mod utils {
    use super::*;

    /// Format version for display with additional metadata
    pub fn format_version_with_info(version: &str, size: Option<u64>, installed: bool) -> String {
        let mut formatted = version.to_string();

        if installed {
            formatted.push_str(&format!(" {}", style("(installed)").green()));
        }

        if let Some(size_bytes) = size {
            let size_mb = size_bytes as f64 / 1024.0 / 1024.0;
            formatted.push_str(&format!(" - {:.1}MB", size_mb));
        }

        formatted
    }

    /// Validate version string format
    pub fn validate_version_format(version: &str) -> bool {
        // Basic Go version validation (e.g., 1.21.3, 1.20, etc.)
        let parts: Vec<&str> = version.split('.').collect();
        if parts.is_empty() || parts.len() > 3 {
            return false;
        }

        parts.iter().all(|part| part.parse::<u32>().is_ok())
    }

    /// Create themed divider
    pub fn themed_divider(title: &str, width: usize) -> String {
        let title_len = title.len();
        if title_len >= width {
            return title.to_string();
        }

        let padding = (width - title_len) / 2;
        let left_dash = "─".repeat(padding);
        let right_dash = "─".repeat(width - padding - title_len);

        format!(
            "{} {} {}",
            style(&left_dash).dim(),
            style(title).cyan().bold(),
            style(&right_dash).dim()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conflict_action_description() {
        assert_eq!(ConflictAction::Skip.description(), "Skip installation (keep existing)");
        assert_eq!(ConflictAction::Overwrite.description(), "Overwrite existing version");
        assert_eq!(ConflictAction::Backup.description(), "Backup existing and install new");
        assert_eq!(ConflictAction::Cancel.description(), "Cancel operation");
    }

    #[test]
    fn test_version_validation() {
        use utils::validate_version_format;

        assert!(validate_version_format("1.21.3"));
        assert!(validate_version_format("1.20"));
        assert!(validate_version_format("2"));
        assert!(!validate_version_format(""));
        assert!(!validate_version_format("1.21.3.4"));
        assert!(!validate_version_format("1.a.3"));
    }

    #[test]
    fn test_format_version_with_info() {
        use utils::format_version_with_info;

        let formatted = format_version_with_info("1.21.3", Some(1048576), true);
        assert!(formatted.contains("1.21.3"));
        assert!(formatted.contains("(installed)"));
        assert!(formatted.contains("1.0MB"));
    }
}
