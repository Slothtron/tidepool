tidepool-gvm\PROJECT_STRUCTURE.md
```

```
# Project Architecture

This document provides an overview of the architecture of the Tidepool project, a high-performance Go version management toolkit written in Rust.

---

## Project Structure

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

---

## Core Modules

### CLI Module
- **Files**: `src/main.rs`, `src/cli.rs`
- **Responsibilities**:
  - Provide a command-line interface for users.
  - Parse user commands and arguments.
  - Delegate tasks to the appropriate command implementation.

### Command Implementation Module
- **File**: `src/commands.rs`
- **Responsibilities**:
  - Implement the logic for commands such as `install`, `list`, and `status`.
  - Handle Go version installation, switching, and management.

### Configuration Management Module
- **File**: `src/config.rs`
- **Responsibilities**:
  - Manage user configuration files and environment variables.
  - Automatically configure GOROOT, GOPATH, and PATH.

### Go Version Management Core
- **File**: `src/go.rs`
- **Responsibilities**:
  - Core logic for managing Go versions.
  - Download and install specific Go versions.
  - Verify file integrity using SHA256 checksums.

### Downloader Module
- **File**: `src/downloader.rs`
- **Responsibilities**:
  - Handle file downloads with asynchronous concurrency.
  - Display download progress to the user.

### Symbolic Link Handling Module
- **File**: `src/symlink.rs`
- **Responsibilities**:
  - Create and manage symbolic links across platforms.

### User Interface Module
- **File**: `src/ui.rs`
- **Responsibilities**:
  - Provide user-friendly output and error messages.
  - Display command execution results.

---

## Dependencies

The project uses several high-quality Rust libraries to achieve its functionality:
- **Command-line Parsing**: `clap`
- **Asynchronous Runtime**: `tokio`
- **HTTP Requests**: `reqwest`
- **JSON Handling**: `serde_json`
- **Error Handling**: `anyhow`, `thiserror`
- **File Operations**: `tempfile`, `dirs`
- **Logging**: `log`, `env_logger`
- **Download Progress**: `indicatif`

---

## Cross-Platform Support

The project supports multiple platforms with conditional compilation:
- **Windows**: Uses the `junction` library for symbolic link handling.
- **Unix**: Uses `flate2` and `tar` libraries for handling compressed files.

---

## Performance Optimizations

- **Asynchronous Concurrency**: Utilizes `tokio` and `reqwest` for high-performance asynchronous downloads.
- **Compilation Optimizations**: Configured in `Cargo.toml` with options like `lto`, `codegen-units`, and `opt-level` to reduce binary size and improve runtime performance.

---

## Development and Testing

### Quick Setup
1. Clone the repository:
   ```bash
   git clone https://github.com/Slothtron/tidepool.git
   cd tidepool
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run tests:
   ```bash
   cargo test
   ```

4. Enable debug logging:
   ```bash
   RUST_LOG=debug cargo run -- install 1.21.3
   ```

---

## Key Features

- **Multi-Platform Support**: Compatible with Windows, macOS, and Linux.
- **High Performance**: Asynchronous concurrent downloads with progress display.
- **Complete Management**: Install, switch, and uninstall Go versions.
- **Safety**: SHA256 verification and protection against accidental deletion.
- **Smart Environment Configuration**: Automatic setup of GOROOT, GOPATH, and PATH.

---

This document serves as a reference for understanding the architecture and design of the Tidepool project. Contributions and improvements are welcome!