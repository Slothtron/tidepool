# Tidepool

> 📖 **Language**: [English](README.md) | [中文](README.zh-CN.md)

![License](https://img.shields.io/badge/License-MIT-yellow.svg)
![Rust](https://img.shields.io/badge/Rust-1.70%2B-blue.svg)
![Build Status](https://github.com/Slothtron/tidepool/workflows/CI/badge.svg)
![Release](https://github.com/Slothtron/tidepool/workflows/Release/badge.svg)

A simple, high-performance Go version management tool written in Rust. Designed with simplicity in mind, providing fast and reliable Go version installation, switching, and management across multiple platforms.

## 🚀 Quick Start

```bash
# Install from source
git clone https://github.com/Slothtron/tidepool.git
cd tidepool
cargo install --path .

# Basic usage
gvm install 1.21.3         # Install Go version
gvm use 1.21.3             # Switch to Go version
gvm list                   # List installed versions
gvm list --all             # List all available versions
gvm status                 # Show current version
gvm uninstall 1.21.3      # Uninstall Go version
gvm info 1.21.3           # Show version details
gvm --help                 # Show all commands
```

### ✨ Simple and Clean Output

```bash
# Clean status display
$ gvm status
[OK] 当前版本: Go 1.23.10
  安装路径: C:\Users\User\.gvm\versions\1.23.10
[INFO] Go 环境已配置
[TIP] 使用 'go version' 验证安装

# Simple list display
$ gvm list
> 已安装的 Go 版本
  - 1.21.3
  * 1.23.10 (当前版本)
[INFO] 总计: 2 个版本
[TIP] 使用 gvm use <版本> 切换版本
```



## 📖 Command Reference

| Command               | Description                                         | Example Usage                 |
| --------------------- | --------------------------------------------------- | ----------------------------- |
| `gvm install <ver>`   | Install a specific Go version                       | `gvm install 1.22.1 --force`  |
| `gvm use <ver>`       | Switch to an installed Go version                   | `gvm use 1.22.1 --global`     |
| `gvm uninstall <ver>` | Uninstall a specific Go version                     | `gvm uninstall 1.21.3`        |
| `gvm list`            | List installed Go versions                          | `gvm list --all`              |
| `gvm status`          | Show current Go version and environment status      | `gvm status --verbose`        |
| `gvm info <ver>`      | Display detailed information about a specific version | `gvm info 1.22.1`             |
| `gvm --help`          | Show help for all commands                          | `gvm --help`                  |
| `gvm --version`       | Show GVM version                                    | `gvm --version`               |

### Global Options

| Option        | Description                    | Usage                         |
| ------------- | ------------------------------ | ----------------------------- |
| `-v, --verbose` | Enable verbose output        | `gvm status --verbose`        |
| `-q, --quiet`   | Enable quiet mode (errors only) | `gvm install 1.21.3 --quiet` |

## 📁 Project Structure

```
tidepool-gvm/
├── src/                         # Source code directory
│   ├── main.rs                  # CLI entry point
│   ├── lib.rs                   # Library entry point
│   ├── cli.rs                   # CLI command parsing and dispatch
│   ├── commands.rs              # Command implementations
│   ├── config.rs                # Configuration management
│   ├── go.rs                    # Go version management core
│   ├── downloader.rs            # File download functionality
│   ├── symlink.rs               # Symbolic link handling
│   ├── platform.rs              # Platform detection and adaptation
│   ├── error.rs                 # Unified error handling
│   ├── ui_flat.rs               # Simplified UI system
│   └── progress_flat.rs         # Simplified progress system
├── examples/                    # Usage examples
│   └── modern_ui_demo.rs        # UI demonstration
├── README.md                    # English documentation
├── README.zh-CN.md              # Chinese documentation
├── Cargo.toml                   # Rust package configuration
├── Cargo.lock                   # Locked dependency versions
└── rustfmt.toml                 # Rust formatting configuration
```

## ✨ Key Features

- **🌐 Multi-Platform Support**: Windows, macOS, and Linux
- **⚡ High Performance**: Fast downloads with optimized async operations
- **🔧 Complete Management**: Install, switch, and uninstall Go versions
- **🛡️ Safety First**: SHA256 verification and protection against accidental deletion
- **⚙️ Smart Environment**: Automatic GOROOT, GOPATH, and PATH configuration
- **📦 Simple Architecture**: Clean, maintainable codebase with minimal dependencies
- **🎯 User Friendly**: Simple CLI with consistent commands and clear output
- **🚀 Cross-Platform**: Stable ASCII output, no Unicode dependencies

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
