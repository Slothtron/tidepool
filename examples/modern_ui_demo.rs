//! Simplified UI System Usage Example
//!
//! Demonstrates how to use the new simplified UI and progress management system.

use std::time::Duration;
use tidepool_gvm::{BasicProgress, InstallSteps, SimpleUI};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    println!("=== Simplified UI System Demo ===\n");

    // 1. Basic UI functions demo
    demo_basic_ui().await;

    // 2. Progress management demo
    demo_progress_management().await;

    // 3. Installation steps demo
    demo_installation_steps().await;
}

/// Demonstrates basic UI functions
async fn demo_basic_ui() {
    println!("1. Basic UI Functions Demo");
    println!("--------------------------");
    // Create a simplified UI
    let ui = SimpleUI::new();

    // Various message types
    ui.success("Go 1.21.3 installed successfully");
    ui.error("Network connection failed");
    ui.info("Checking for available versions");
    ui.key_value("Current Version", "1.21.3");
    ui.key_value("Installation Path", "/usr/local/go");

    println!("\nSimplified UI mode demo finished");
}

/// Demonstrates progress management functions
async fn demo_progress_management() {
    println!("\n\n2. Progress Management Demo");
    println!("---------------------------");

    // Create a basic progress bar
    let progress = BasicProgress::new("Downloading Go 1.21.4".to_string());

    // Simulate download progress
    for i in 0..=100 {
        let percent = i as f64 / 100.0;
        let info = format!("{}MB / {}MB", i * 2, 200);
        progress.show(percent, Some(&info));
        sleep(Duration::from_millis(20)).await;
    }

    println!("\nDownload complete!");

    // Create an extraction progress bar
    let extract_progress = BasicProgress::new("Extracting files".to_string());

    for i in 0..=100 {
        let percent = i as f64 / 100.0;
        let info = format!("File {}/1000", i * 10);
        extract_progress.show(percent, Some(&info));
        sleep(Duration::from_millis(15)).await;
    }

    println!("\nExtraction complete!");
}

/// Demonstrates installation steps
async fn demo_installation_steps() {
    println!("\n\n3. Installation Steps Demo");
    println!("--------------------------");

    let install_steps = InstallSteps::new();
    let ui = SimpleUI::new();

    // Start installation
    install_steps.start("1.21.4");

    // Simulate installation steps
    let steps = [
        "Validating version format",
        "Checking network connection",
        "Downloading Go archive",
        "Verifying file integrity",
        "Extracting installation files",
        "Configuring environment variables",
        "Creating symbolic link",
        "Cleaning up temporary files",
        "Verifying installation",
    ];

    for (i, step) in steps.iter().enumerate() {
        install_steps.info(&format!("Executing: {step}"));

        // Simulate step execution time
        sleep(Duration::from_millis(300)).await;

        ui.success(&format!("Completed: {step}"));

        // Display overall progress
        let progress = ((i + 1) as f64 / steps.len() as f64) * 100.0;
        println!("Overall Progress: {progress:.1}%");
        println!();
    }

    install_steps.complete("1.21.4");
    ui.info("Use 'gvm use 1.21.4' to switch to the new version");
}
