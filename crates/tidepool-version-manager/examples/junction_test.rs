use std::path::Path;
use tempfile::TempDir;

#[cfg(target_os = "windows")]
fn main() {
    use tidepool_version_manager::junction_utils::{
        safe_create_junction, safe_remove_junction_or_dir,
    };

    println!("Testing junction functionality...");

    let temp_dir = TempDir::new().unwrap();
    println!("Temp dir: {}", temp_dir.path().display());

    // Create a target directory
    let target_path = temp_dir.path().join("target");
    std::fs::create_dir_all(&target_path).unwrap();

    // Create a test file in target
    std::fs::write(target_path.join("test.txt"), "Hello, world!").unwrap();

    let junction_path = temp_dir.path().join("junction");

    println!("Creating junction: {} -> {}", junction_path.display(), target_path.display());

    match safe_create_junction(&junction_path, &target_path) {
        Ok(()) => {
            println!("✅ Junction created successfully!");

            // Test if we can access files through the junction
            let test_file = junction_path.join("test.txt");
            if test_file.exists() {
                println!("✅ Can access files through junction");
                let content = std::fs::read_to_string(test_file).unwrap();
                println!("File content: {}", content);
            } else {
                println!("❌ Cannot access files through junction");
            }

            // Test removal
            println!("Removing junction...");
            match safe_remove_junction_or_dir(&junction_path) {
                Ok(()) => println!("✅ Junction removed successfully!"),
                Err(e) => println!("❌ Failed to remove junction: {}", e),
            }
        }
        Err(e) => {
            println!("❌ Failed to create junction: {}", e);

            // Check if it's a permissions issue
            if e.contains("Access is denied") || e.contains("权限") {
                println!("This appears to be a permissions issue.");
                println!("Try running as Administrator or enable Developer Mode in Windows.");
            }
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn main() {
    println!("This test is only for Windows");
}
