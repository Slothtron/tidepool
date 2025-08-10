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

## 📁 Project Structure

```
tidepool/
├── src/
│   ├── main.rs                   # CLI entry point
│   ├── lib.rs                    # Library entry point
│   ├── cli.rs                    # CLI command parsing
│   ├── commands.rs               # Command implementations
│   ├── config.rs                 # Configuration management
│   ├── ui.rs                     # User interface
│   ├── go.rs                     # Go version management core
│   ├── downloader.rs             # Downloader
│   └── symlink.rs                # Symbolic link handling
├── README.md                     # Documentation
├── README.zh-CN.md              # Chinese documentation
├── Cargo.toml                    # Package configuration
└── .github/                      # GitHub workflows
```

## ✨ Key Features

- **🌐 Multi-Platform Support**: Windows, macOS, and Linux
- **⚡ High Performance**: Asynchronous concurrent downloads with progress display
- **🔧 Complete Management**: Install, switch, and uninstall Go versions
- **🛡️ Safety First**: SHA256 verification and protection against accidental deletion
- **⚙️ Smart Environment**: Automatic GOROOT, GOPATH, and PATH configuration
- **📦 Simple Architecture**: Single crate design for easy maintenance

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
