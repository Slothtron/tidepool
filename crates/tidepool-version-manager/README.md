# Tidepool Version Manager

> 📖 **Language**: [English](README.md) | [中文](README.zh-CN.md)

[![Crates.io](https://img.shields.io/crates/v/tidepool-version-manager.svg)](https://crates.io/crates/tidepool-version-manager)
[![Documentation](https://docs.rs/tidepool-version-manager/badge.svg)](https://docs.rs/tidepool-version-manager)
[![License](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-blue.svg)]()

Core library of the Tidepool project, providing powerful Go version management capabilities with high-performance async operations and cross-platform support.

## ✨ Key Features

- **🚀 Async Operations** - High-performance concurrent downloading and installation
- **🔄 Version Management** - Install, switch, uninstall, and list Go versions
- **🌐 Cross-Platform** - Windows, macOS, and Linux support
- **⚡ Smart Downloads** - Chunked downloads with resume capability and progress reporting
- **🛡️ Security** - SHA256 verification and safe uninstallation protection
- **🔗 Symlink Management** - Cross-platform symbolic links without administrator privileges

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
tidepool-version-manager = "0.1.5"
```

## 🚀 Quick Start

```rust
use tidepool_version_manager::{go::GoManager, VersionManager, InstallRequest, SwitchRequest};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let go_manager = GoManager::new();

    // Install Go version
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

## 🏗️ Architecture

```
tidepool-version-manager/
├── src/
│   ├── lib.rs              # Public interfaces and type definitions
│   ├── go.rs               # Go version management implementation
│   ├── downloader.rs       # Universal downloader module
│   └── symlink.rs          # Cross-platform symlink management
└── tests/                  # Integration tests
```

## 📚 API Reference

Complete documentation: [docs.rs/tidepool-version-manager](https://docs.rs/tidepool-version-manager)

### Core Types

- [`VersionManager`](https://docs.rs/tidepool-version-manager/latest/tidepool_version_manager/trait.VersionManager.html) - Main trait for version management operations
- [`GoManager`](https://docs.rs/tidepool-version-manager/latest/tidepool_version_manager/go/struct.GoManager.html) - Go language version manager implementation
- [`Downloader`](https://docs.rs/tidepool-version-manager/latest/tidepool_version_manager/downloader/struct.Downloader.html) - High-performance file downloader

## 🧪 Development

```bash
# Run tests
cargo test

# Check code quality
cargo fmt --check
cargo clippy --all-targets --all-features
```

## 🤝 Contributing

Contributions are welcome! Please see the [main project contributing guide](../../CONTRIBUTING.md) for details.

## 📄 License

Licensed under the MIT License. See [LICENSE](../../LICENSE) for details.

---

**Part of the [Tidepool Project](https://github.com/Slothtron/tidepool)** 🌊