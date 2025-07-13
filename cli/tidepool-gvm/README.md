# tidepool-gvm

> 📖 **Language**: [English](README.md) | [中文](README.zh-CN.md)

[![Crates.io](https://img.shields.io/crates/v/tidepool-gvm.svg)](https://crates.io/crates/tidepool-gvm)
[![License](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

A command-line interface for Go version management, providing an intuitive and user-friendly way to install, switch, and manage Go versions across platforms.

## ✨ Features

- **🔄 Version Management** - Install, switch, and uninstall Go versions
- **🚀 Fast Operations** - Asynchronous downloads with progress display
- **🛡️ Safety** - Protection against accidental deletion of active versions
- **🌍 Cross-Platform** - Works on Windows, macOS, and Linux
- **🎨 Modern UI** - Colorful terminal output and progress indicators
- **⚙️ Environment Management** - Automatic GOROOT, GOPATH, and PATH configuration

## 📦 Installation

### Via Cargo

```bash
cargo install tidepool-gvm
```

### From Source

```bash
git clone https://github.com/Slothtron/tidepool.git
cd tidepool
cargo build --release --package tidepool-gvm
```

The binary will be available at `target/release/gvm` (or `gvm.exe` on Windows).

## 🚀 Quick Start

```bash
# Install and switch to a Go version
gvm install 1.21.3

# List installed versions
gvm list

# Show current status
gvm status

# List available versions
gvm list --available

# Show help
gvm --help
```

## 📚 Commands

| Command | Description |
|---------|-------------|
| `gvm install <VERSION>` | Install and switch to a Go version |
| `gvm list` | List installed versions |
| `gvm list --available` | List available versions for download |
| `gvm status` | Show current Go version and environment |
| `gvm info <VERSION>` | Show detailed information about a version |
| `gvm uninstall <VERSION>` | Uninstall a Go version |

### Options

- `--force, -f` - Force reinstall existing version
- `--help, -h` - Show help information
- `--version, -V` - Show version information

## 🏗️ Architecture

Built on top of [`tidepool-version-manager`](../../crates/tidepool-version-manager/) core library.

```
cli/tidepool-gvm/
├── src/
│   ├── main.rs         # Main entry point
│   ├── cli.rs          # Command-line parsing
│   ├── commands.rs     # Command implementations
│   ├── config.rs       # Configuration management
│   └── ui.rs          # User interface helpers
└── tests/             # Integration tests
```

## 🧪 Development

```bash
# Development build
cargo build --package tidepool-gvm

# Run tests
cargo test --package tidepool-gvm

# Run with debug logging
RUST_LOG=debug cargo run --package tidepool-gvm -- install 1.21.3
```

## 🤝 Contributing

Contributions are welcome! Please see the [main project contributing guide](../../CONTRIBUTING.md) for details.

## 📄 License

Licensed under the MIT License. See [LICENSE](../../LICENSE) for details.

---

**Part of the [Tidepool Project](https://github.com/Slothtron/tidepool)** 🌊
