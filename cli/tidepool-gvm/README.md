# tidepool-gvm

> 📖 **Language**: [English](README.md) | [中文](README.zh-CN.md)

A command-line interface for Go version management, providing an intuitive and user-friendly way to install, switch, and manage Go versions across platforms.

## Overview

`tidepool-gvm` is the CLI component of the Tidepool project that provides the `gvm` command. It's built on top of the `tidepool-version-manager` core library and offers a modern, package manager-style interface for Go version management.

## Installation

### Via Cargo

```bash
cargo install tidepool-gvm
```

This installs the binary as `gvm` command.

### From Source

```bash
git clone https://github.com/Slothtron/tidepool.git
cd tidepool
cargo build --release --package tidepool-gvm
```

The binary will be available at `target/release/gvm` (or `gvm.exe` on Windows).

## Usage

### Basic Commands

```bash
# Install a Go version
gvm install 1.21.3

# Switch to a Go version
gvm use 1.21.3

# List installed versions
gvm list

# List available versions for download
gvm list --available

# Show current status
gvm status

# Show version information
gvm info 1.21.3

# Uninstall a version
gvm uninstall 1.20.5

# Update available versions cache
gvm update

# Show configuration
gvm config

# Show help
gvm --help
```

### Advanced Options

```bash
# Force installation (overwrite existing)
gvm install 1.21.3 --force

# Global setting (persistent across terminals)
gvm use 1.21.3 --global

# Custom installation directory
gvm install 1.21.3 --install-dir /opt/go-versions

# Verbose output
gvm install 1.21.3 --verbose
```

## Features

- **🔄 Version Management**: Install, switch, and uninstall Go versions
- **🚀 Fast Operations**: Asynchronous downloads with progress display
- **🛡️ Safety**: Protection against accidental deletion of active versions
- **🌍 Cross-Platform**: Works on Windows, macOS, and Linux
- **🎨 Modern UI**: Colorful terminal output and progress indicators
- **⚙️ Environment Management**: Automatic GOROOT, GOPATH, and PATH configuration

## Configuration

GVM stores configuration in platform-specific directories:

- **Windows**: `%APPDATA%\gvm\config.toml`
- **macOS/Linux**: `~/.config/gvm/config.toml`

### Example Configuration

```toml
[gvm]
install_dir = "/usr/local/go-versions"
download_dir = "/tmp/gvm-downloads"
mirror = "official"
cleanup_downloads = true
concurrent_connections = 4
```

## Environment Variables

After switching to a Go version, GVM automatically configures:

```bash
GOROOT="/usr/local/go-versions/1.21.3"
GOPATH="$HOME/go"  # if not already set
PATH="$GOROOT/bin:$GOPATH/bin:$PATH"
```

## Development

This CLI tool is built using:

- **[clap](https://crates.io/crates/clap)**: Command-line argument parsing
- **[tokio](https://crates.io/crates/tokio)**: Async runtime
- **[indicatif](https://crates.io/crates/indicatif)**: Progress bars
- **[console](https://crates.io/crates/console)**: Terminal styling
- **[tidepool-version-manager](../../../crates/tidepool-version-manager/)**: Core functionality

### Project Structure

```
cli/tidepool-gvm/
├── Cargo.toml          # Package configuration
├── src/
│   ├── main.rs         # Main entry point
│   ├── lib.rs          # Library interface
│   ├── cli.rs          # Command-line parsing
│   ├── commands.rs     # Command implementations
│   ├── config.rs       # Configuration management
│   └── ui.rs          # User interface helpers
├── examples/           # Usage examples
└── tests/             # Integration tests
```

### Building

```bash
# Development build
cargo build --package tidepool-gvm

# Release build
cargo build --release --package tidepool-gvm

# Run tests
cargo test --package tidepool-gvm

# Run with debug logging
RUST_LOG=debug cargo run --package tidepool-gvm -- install 1.21.3
```

## Architecture

The CLI follows a clean architecture pattern:

1. **CLI Layer** (`cli.rs`): Parses command-line arguments using clap
2. **Command Layer** (`commands.rs`): Implements business logic for each command
3. **UI Layer** (`ui.rs`): Handles user interface and terminal output
4. **Config Layer** (`config.rs`): Manages application configuration
5. **Core Layer**: Uses `tidepool-version-manager` for actual version management

## Error Handling

The CLI provides user-friendly error messages and suggestions:

```bash
$ gvm use 1.21.3
❌ Go version 1.21.3 is not installed

💡 Suggestions:
   1. Install it first: gvm install 1.21.3
   2. List available versions: gvm list --available
```

## License

This project is licensed under the MIT License. See [LICENSE](../../LICENSE) for details.

## Contributing

Contributions are welcome! Please see the main project's [Contributing Guide](../../CONTRIBUTING.md) for details.
