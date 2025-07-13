# Tidepool

> 📖 **Language**: [English](README.md) | [中文](README.zh-CN.md)

![License](https://img.shields.io/badge/License-MIT-yellow.svg)
![Rust](https://img.shields.io/badge/Rust-1.70%2B-blue.svg)
![Build Status](https://github.com/Slothtron/tidepool/workflows/CI/badge.svg)
![Release](https://github.com/Slothtron/tidepool/workflows/Release/badge.svg)

A high-performance Go version management toolkit written in Rust, providing seamless Go version installation, switching, and management across multiple platforms.

## 🚀 Quick Start

```bash
# Install the CLI tool
cargo install tidepool-gvm

# Basic usage
gvm install 1.21.3    # Install Go version
gvm list              # List installed versions
gvm status            # Show current version
gvm --help            # Show all commands
```

For detailed installation options and complete usage guide, see [CLI Documentation](cli/tidepool-gvm/README.md).

## 📁 Project Structure

```
tidepool/
├── crates/
│   └── tidepool-version-manager/   # Core Go version management library
└── cli/
    └── tidepool-gvm/              # CLI tool (binary: gvm)
```

### Components

| Component | Description | Documentation |
|-----------|-------------|---------------|
| **[tidepool-version-manager](crates/tidepool-version-manager/)** | Core library providing Go version management functionality | [📖 Library Documentation](crates/tidepool-version-manager/README.md) |
| **[tidepool-gvm](cli/tidepool-gvm/)** | Command-line interface tool (installs as `gvm` command) | [📖 CLI Documentation](cli/tidepool-gvm/README.md) |

## ✨ Key Features

- **🌐 Multi-Platform Support**: Windows, macOS, and Linux
- **⚡ High Performance**: Asynchronous concurrent downloads with progress display  
- **🔧 Complete Management**: Install, switch, and uninstall Go versions
- **🛡️ Safety First**: SHA256 verification and protection against accidental deletion
- **⚙️ Smart Environment**: Automatic GOROOT, GOPATH, and PATH configuration

## 🔧 Development

### Quick Development Setup

```bash
git clone https://github.com/Slothtron/tidepool.git
cd tidepool

# Build all components
cargo build --release

# Run tests  
cargo test

# Build specific component
cargo build --release --package tidepool-gvm
```

### System Requirements

- **Rust**: 1.70+ 
- **Network**: Internet connection for downloading Go versions
- **Platforms**: Windows 10+, macOS 10.15+, Linux (x86_64, ARM64)

For detailed development setup and contribution guidelines, see individual component documentation.

## 📄 License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

