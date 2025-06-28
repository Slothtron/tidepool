# Tidepool

> 📖 **Language**: [English](README.md) | [中文](README.zh-CN.md)

![License](https://img.shields.io/badge/License-MIT-yellow.svg)
![Rust](https://img.shields.io/badge/Rust-1.70%2B-blue.svg)
![Build Status](https://github.com/Slothtron/tidepool/workflows/CI/badge.svg)
![Release](https://github.com/Slothtron/tidepool/workflows/Release/badge.svg)

A high-performance Go version management toolkit written in Rust, providing seamless Go version installation, switching, and management across multiple platforms.

## 🚀 Quick Start

### Install via Cargo

```bash
cargo install tidepool-gvm
```

### From Release Binaries

Download from [GitHub Releases](https://github.com/Slothtron/tidepool/releases):

```bash
# Linux/macOS
curl -L https://github.com/Slothtron/tidepool/releases/latest/download/gvm-<target>.tar.gz | tar xz
sudo mv gvm /usr/local/bin/

# Windows: Download and extract gvm-x86_64-pc-windows-msvc.zip
# Add gvm.exe to PATH
```

### Basic Usage

```bash
# Install Go version
gvm install 1.21.3

# Switch Go version
gvm use 1.21.3

# List installed versions
gvm list

# Show help
gvm --help
```

## � Project Structure

```
tidepool/
├── crates/
│   └── tidepool-version-manager/   # Core Go version management library
└── cli/
    └── tidepool-gvm/              # CLI tool (binary: gvm)
```

| Component | Description |
|-----------|-------------|
| `tidepool-version-manager` | Core library for Go version management |
| `tidepool-gvm` | Command-line interface (installs as `gvm` command) |

## ✨ Features

- **Multi-Platform Support**: Windows, macOS, and Linux
- **Fast Downloads**: Asynchronous concurrent downloads with progress display
- **Version Management**: Install, switch, and uninstall Go versions
- **Safety**: SHA256 verification and protection against accidental deletion
- **Environment Management**: Automatic GOROOT, GOPATH, and PATH configuration

## 🔧 Development

### Building from Source

```bash
git clone https://github.com/Slothtron/tidepool.git
cd tidepool

# Build CLI tool
cargo build --release --package tidepool-gvm

# Run tests
cargo test
```

### System Requirements

- Rust 1.70+
- Network connection for downloading Go versions
- Supported platforms: Windows 10+, macOS 10.15+, Linux (x86_64, ARM64)

## 📄 License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

