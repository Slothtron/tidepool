# Tidepool Version Manager

> 📖 **Language**: [English](README.md) | [中文](README.zh-CN.md)

[![Crates.io](https://img.shields.io/crates/v/tidepool-version-manager.svg)](https://crates.io/crates/tidepool-version-manager)
[![Documentation](https://docs.rs/tidepool-version-manager/badge.svg)](https://docs.rs/tidepool-version-manager)
[![License](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-blue.svg)]()

`tidepool-version-manager` is the core library of the Tidepool project, providing powerful runtime version management capabilities. It supports installation, switching, uninstallation, and management of multiple programming language runtimes. Currently focused on Go version management, it will expand to support Python, Node.js, and more runtimes in the future.

## 🌟 Features

### 🚀 Core Functionality
- **Async Version Installation** - High-performance concurrent downloading and installation
- **Smart Version Switching** - Cross-platform version switching mechanism
- **Complete Lifecycle Management** - Install, switch, uninstall, and list operations
- **Status Querying** - Real-time runtime status checking
- **Multi-Platform Support** - Full compatibility with Windows, macOS, and Linux

### 🔧 Downloader Features
- **Chunked Concurrent Downloads** - Multi-connection download acceleration
- **Resume Downloads** - Automatic recovery from network interruptions
- **Progress Reporting** - Real-time download progress feedback
- **File Integrity Verification** - SHA256 checksums for security
- **Smart Retry Mechanism** - Automatic handling of network exceptions

### 🛡️ Security Features
- **File Hash Verification** - Automatic verification of downloaded file integrity
- **Permission Security** - Cross-platform symlinks without administrator privileges
- **Safe Uninstallation** - Protection mechanisms to prevent accidental system file deletion

## 📦 Installation

Add the following to your `Cargo.toml`:

[dependencies]
tidepool-version-manager = "0.1.3"
```
```toml
[dependencies]
tidepool-version-manager = "0.1.4"
```

## 🚀 Quick Start

### Basic Usage

```rust
use tidepool_version_manager::{
    go::GoManager,
    VersionManager,
    InstallRequest,
    SwitchRequest
};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create Go version manager
    let go_manager = GoManager::new();

    // Install Go 1.21.0
    let install_request = InstallRequest {
        version: "1.21.0".to_string(),
        install_dir: PathBuf::from("/usr/local/go-versions"),
        download_dir: PathBuf::from("/tmp/go-downloads"),
        force: false,
    };

    let version_info = go_manager.install(install_request).await?;
    println!("✅ Installed Go {}", version_info.version);

    // Switch to that version
    let switch_request = SwitchRequest {
        version: "1.21.0".to_string(),
        base_dir: PathBuf::from("/usr/local/go-versions"),
        global: true,
        force: false,
    };

    go_manager.switch_to(switch_request)?;
    println!("🔄 Switched to Go 1.21.0");

    Ok(())
}
```

### Query Available Versions

```rust
use tidepool_version_manager::{go::GoManager, VersionManager};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let go_manager = GoManager::new();

    // Get available versions list
    let available_versions = go_manager.list_available().await?;
    println!("📋 Available Go versions ({} total):", available_versions.total_count);

    for version in available_versions.versions.iter().take(10) {
        println!("   - {}", version);
    }

    Ok(())
}
```

### Status Query

```rust
use tidepool_version_manager::{go::GoManager, VersionManager, StatusRequest};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let go_manager = GoManager::new();

    let status_request = StatusRequest {
        base_dir: Some(PathBuf::from("/usr/local/go-versions")),
    };

    let status = go_manager.status(status_request)?;

    if let Some(version) = status.current_version {
        println!("🎯 Current Go version: {}", version);

        if let Some(path) = status.install_path {
            println!("📁 Installation path: {}", path.display());
        }

        println!("🌍 Environment variables:");
        for (key, value) in status.environment_vars {
            println!("   {}={}", key, value);
        }
    } else {
        println!("❌ No installed Go version detected");
    }

    Ok(())
}
```

## 🔧 Advanced Configuration

### Custom Downloader Configuration

```rust
use tidepool_version_manager::downloader::{Downloader, DownloadConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create custom download configuration
    let config = DownloadConfig {
        concurrent_connections: 8,        // Number of concurrent connections
        timeout_seconds: 300,             // Timeout in seconds
        enable_chunked_download: true,    // Enable chunked downloads
        max_retries: 3,                   // Maximum retry attempts
        min_chunk_size: 10 * 1024 * 1024, // Minimum chunk size (10MB)
        ..Default::default()
    };

    let downloader = Downloader::with_config(config);

    // Use custom downloader for downloads
    let url = "https://go.dev/dl/go1.21.0.linux-amd64.tar.gz";
    let output_path = "/tmp/go1.21.0.linux-amd64.tar.gz";

    downloader.download(url, output_path, None).await?;
    println!("✅ Download completed: {}", output_path);

    Ok(())
}
```

### Progress Callbacks

```rust
use tidepool_version_manager::downloader::{Downloader, ProgressReporter};

struct MyProgressReporter;

impl ProgressReporter for MyProgressReporter {
    fn report_progress(&self, downloaded: u64, total: Option<u64>) {
        if let Some(total) = total {
            let percentage = (downloaded as f64 / total as f64) * 100.0;
            println!("📊 Download progress: {:.1}% ({}/{})", percentage, downloaded, total);
        } else {
            println!("📊 Downloaded: {} bytes", downloaded);
        }
    }

    fn report_error(&self, error: &str) {
        eprintln!("❌ Download error: {}", error);
    }

    fn report_completion(&self) {
        println!("✅ Download completed!");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let downloader = Downloader::new();
    let progress_reporter = MyProgressReporter;

    let url = "https://go.dev/dl/go1.21.0.linux-amd64.tar.gz";
    let output_path = "/tmp/go1.21.0.linux-amd64.tar.gz";

    downloader.download(url, output_path, Some(&progress_reporter)).await?;

    Ok(())
}
```

## 🏗️ Architecture Design

### Core Components

```
tidepool-version-manager/
├── src/
│   ├── lib.rs              # Public interfaces and type definitions
│   ├── go.rs               # Go version management implementation
│   └── downloader.rs       # Universal downloader module
├── examples/               # Usage examples
└── tests/                  # Integration tests
```

### Feature Overview

- **`VersionManager` Trait** - Unified version management interface, extensible for other runtimes
- **`GoManager`** - Concrete implementation for Go language version management
- **`Downloader`** - High-performance async downloader with resume and progress reporting
- **Cross-platform Support** - Unified abstraction for cross-platform symlinks

## 🧪 Running Examples

The project includes several example programs demonstrating different use cases:

```bash
# Downloader functionality demo
cargo run --example downloader_test

# Hash verification demo
cargo run --example hash_verification_demo



# Temporary file handling demo
cargo run --example temp_file_demo

# Uninstall protection demo
cargo run --example uninstall_protection_demo
```

## 🧪 Testing

Run the complete test suite:

```bash
# Run all tests
cargo test

# Run specific tests
cargo test go_manager_tests

# Run integration tests
cargo test --test integration_tests
```

## 🔄 Version Compatibility

| Version | Rust Version | Features |
|---------|-------------|----------|
| 0.1.3   | 1.70+       | Go version management, high-performance downloader |
| 0.1.2   | 1.70+       | Basic version management functionality |
| 0.1.1   | 1.70+       | Initial release |

## 🚧 Future Plans

- [ ] **Python Version Management** - Support for Python/pyenv compatibility
- [ ] **Node.js Version Management** - Support for Node.js/nvm compatibility
- [ ] **Configuration File Support** - Project-level version configuration
- [ ] **Plugin System** - Custom version management extensions
- [ ] **Mirror Source Support** - Accelerated downloads via domestic mirrors

## 📋 API Documentation

Complete API documentation is available at [docs.rs](https://docs.rs/tidepool-version-manager).

### Main Types

- [`VersionManager`](https://docs.rs/tidepool-version-manager/latest/tidepool_version_manager/trait.VersionManager.html) - Version manager trait
- [`GoManager`](https://docs.rs/tidepool-version-manager/latest/tidepool_version_manager/go/struct.GoManager.html) - Go version manager
- [`Downloader`](https://docs.rs/tidepool-version-manager/latest/tidepool_version_manager/downloader/struct.Downloader.html) - High-performance downloader
- [`InstallRequest`](https://docs.rs/tidepool-version-manager/latest/tidepool_version_manager/struct.InstallRequest.html) - Installation request structure
- [`RuntimeStatus`](https://docs.rs/tidepool-version-manager/latest/tidepool_version_manager/struct.RuntimeStatus.html) - Runtime status information

## 🤝 Contributing

We welcome all forms of contributions! Please see [CONTRIBUTING.md](../../CONTRIBUTING.md) for details.

### Development Environment Setup

```bash
# Clone repository
git clone https://github.com/Slothtron/tidepool.git
cd tidepool/crates/tidepool-version-manager

# Run tests
cargo test

# Check code formatting
cargo fmt --check

# Run Clippy checks
cargo clippy --all-targets --all-features
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](../../LICENSE) file for details.

## 🔗 Related Links

- [GitHub Repository](https://github.com/Slothtron/tidepool)
- [Issue Tracker](https://github.com/Slothtron/tidepool/issues)
- [Changelog](../../CHANGELOG.md)
- [Releases](https://github.com/Slothtron/tidepool/releases)

---

**Maintained by the [Tidepool Project](https://github.com/Slothtron/tidepool)** 🌊
