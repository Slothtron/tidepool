# Tidepool

> 📖 **Language**: [English](README.md) | [中文](README.zh-CN.md)

![License](https://img.shields.io/badge/License-MIT-yellow.svg)
![Rust](https://img.shields.io/badge/Rust-1.70%2B-blue.svg)
![Build Status](https://github.com/Slothtron/tidepool/workflows/CI/badge.svg)
![Release](https://github.com/Slothtron/tidepool/workflows/Release/badge.svg)

A high-performance Go version management toolkit written in Rust, providing seamless Go version installation, switching, and management across multiple platforms.

## 🚀 Quick Start

```bash
# Install from source
git clone https://github.com/Slothtron/tidepool.git
cd tidepool
cargo install --path .

# Basic usage
gvm install 1.21.3    # Install Go version
gvm list              # List installed versions
gvm status            # Show current version
gvm --help            # Show all commands
```

### ✨ Beautiful Command Output

```bash
# Enhanced list display
$ gvm list
▶ Installed Go Versions
───────────────────────
  📦 1.21.3
  ⭐ 1.22.1 (active)
──────────────────────────────────────────────────
  Total versions: 2
  Active version: 1.22.1
```



## 📖 Command Reference

| Command               | Description                                         | Example Usage                 |
| --------------------- | --------------------------------------------------- | ----------------------------- |
| `gvm install <ver>`   | Install a specific Go version                       | `gvm install 1.22.1`          |
| `gvm use <ver>`       | Switch to an installed Go version                   | `gvm use 1.22.1`              |
| `gvm uninstall <ver>` | Uninstall a specific Go version                     | `gvm uninstall 1.21.3`        |
| `gvm list`            | List all installed Go versions                      | `gvm list`                    |
| `gvm status`          | Show current Go version and environment status      | `gvm status`                  |
| `gvm info <ver>`      | Display detailed information about a specific version | `gvm info 1.22.1`             |
| `gvm --help`          | Show help for all commands                          | `gvm --help`                  |
| `gvm --version`       | Show GVM version                                    | `gvm --version`               |

## 📁 Project Structure

```
tidepool/
├── src/                         # Source code directory
│   ├── main.rs                  # CLI entry point
│   ├── lib.rs                   # Library entry point
│   ├── cli.rs                   # CLI command parsing
│   ├── commands.rs              # Command implementations
│   ├── config.rs                # Configuration management
│   ├── ui.rs                    # User interface
│   ├── go.rs                    # Go version management core
│   ├── downloader.rs            # Downloader module
│   └── symlink.rs               # Symbolic link handling
├── README.md                    # English documentation
├── README.zh-CN.md              # Chinese documentation
├── Cargo.toml                   # Rust package configuration
├── Cargo.lock                   # Locked dependency versions
├── .github/                     # GitHub workflows
└── rustfmt.toml                 # Rust formatting configuration
```

## ✨ Key Features

- **🌐 Multi-Platform Support**: Windows, macOS, and Linux
- **⚡ High Performance**: Asynchronous concurrent downloads with progress display
- **🔧 Complete Management**: Install, switch, and uninstall Go versions
- **🛡️ Safety First**: SHA256 verification and protection against accidental deletion
- **⚙️ Smart Environment**: Automatic GOROOT, GOPATH, and PATH configuration
- **📦 Simple Architecture**: Single crate design for easy maintenance
- **🛠️ Developer Friendly**: Includes detailed architecture documentation and optimized build configurations

## 🔧 Development

### Quick Development Setup

```bash
git clone https://github.com/Slothtron/tidepool.git
cd tidepool

# Build the project
cargo build --release

# Run tests
cargo test

# Run with debug logging
RUST_LOG=debug cargo run -- install 1.21.3
```

### System Requirements

- **Rust**: 1.70+
- **Network**: Internet connection for downloading Go versions
- **Platforms**: Windows 10+, macOS 10.15+, Linux (x86_64, ARM64)

### Build for Different Platforms

```bash
# Build for current platform
cargo build --release

# Cross-compile (requires target toolchain)
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-pc-windows-msvc
```

## 📄 License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
