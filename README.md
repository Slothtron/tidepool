# Tidepool - Developer Environment & Network Management Toolkit

[https://img.shields.io/badge/License-MIT-yellow.svg](https://opensource.org/licenses/MIT)

[https://img.shields.io/badge/Rust-1.72%2B-blue.svg](https://www.rust-lang.org/)

Tidepool is a Rust-powered toolkit for comprehensive developer environment management and network control. It provides core functionality for environment variables, runtime version switching, traffic monitoring, and network virtualization.

## Core Features

* 🛠️ **Environment Management**

  Centralized control of environment variables across platforms
* 🔄 **Runtime Version Switching**

  Seamless switching between language versions (Go, Python, Node.js, etc.)
* 🌐 **Network Proxy & Mirroring**

  Custom HTTP/HTTPS proxy with request interception capabilities
* 📡 **DNS Mapping**

  Advanced domain-to-IP/Domain redirection rules
* 📊 **Traffic Monitoring**

  Real-time network traffic capture and analysis
* 🔍 **Extensible Architecture**

  Modular design for custom integrations and plugins

## Architecture Overview

```
graph TD
    A[Tidepool Core] --> B[Environment Management]
    A --> C[Proxy Service]
    A --> D[Traffic Capture]
    A --> E[DNS Mirroring]
    F[CLI Tool] --> A
    G[TAURI Client] --> A
    H[API Service] --> A
```

### Modular Components

| Component            | Description                               | Status      |
| -------------------- | ----------------------------------------- | ----------- |
| `tidepool-core`    | Core functionality library                | Implemented |
| `tidepool-cli`     | Command-line interface for developers     | Planned     |
| `tidepool-app`     | Desktop application (TAURI-based)         | Planned     |
| `tidepool-api`     | REST API for remote management            | Planned     |
| `tidepool-plugins` | Extension system for custom functionality | Planned     |

## Getting Started

### Prerequisites

* Rust 1.72+
* Cargo package manager
* Platform-specific dependencies (openssl, libpcap, etc.)

### Installation

Add Tidepool Core to your project:

```
[dependencies]
tidepool-core = { git = "https://github.com/your-username/tidepool.git", package = "tidepool-core" }
```

## Core Concepts

### Environment Management

* Unified interface for environment variables
* Language-specific version switching
* Cross-platform consistency (Windows, macOS, Linux)

### Network Services

* Customizable HTTP/S proxy
* Domain-based traffic routing
* SSL/TLS interception capabilities
* System hosts file manipulation

### DNS Mirroring

* Real-time domain resolution override
* Rule-based traffic redirection:
  * Domain → IP address
  * Domain → Domain
* Rule persistence across sessions

## Development Status

| Module           | Windows | macOS | Linux | Notes                     |
| ---------------- | ------- | ----- | ----- | ------------------------- |
| Environment Vars | ✅      | ✅    | ✅    | Language-specific support |
| Hosts Management | ✅      | ✅    | ✅    | Atomic updates            |
| HTTP Proxy       | ✅      | ✅    | ✅    | MITM support planned      |
| HTTPS Proxy      | 🔶      | 🔶    | 🔶    | Certificate management    |
| Traffic Analysis | 🔶      | 🔶    | 🔶    | PCAP integration          |

✅ Stable  🔶 In Development

## Usage Examples

```
use tidepool_core::{EnvironmentManager, DnsMirror};

// Initialize environment manager
let mut env_mgr = EnvironmentManager::new();

// Set Go version
env_mgr.set_version("go", "1.21.3")?;

// Configure DNS mirroring
let mut dns_mirror = DnsMirror::new();
dns_mirror.add_rule("api.example.com", "127.0.0.1");
dns_mirror.apply()?;
```

## Contributing

Tidepool welcomes contributions! Please see our [Contribution Guidelines](https://tencent.yuanbao/CONTRIBUTING.md) for details.

### Development Workflow

```
sequenceDiagram
    Contributor->>Fork: Create personal fork
    Contributor->>Local: cargo build --release
    Contributor->>Tests: cargo test
    Contributor->>PR: Submit pull request
    Maintainer->>CI: Automated testing
    Maintainer->>Review: Code review
    Maintainer->>Merge: Approval & merge
```

## Documentation

* [API Reference](https://tencent.yuanbao/docs/API.md)
* [Configuration Guide](https://tencent.yuanbao/docs/CONFIGURATION.md)
* [Architecture Overview](https://tencent.yuanbao/docs/ARCHITECTURE.md)

## Roadmap

### Phase 1: Core Functionality

* [X] Environment variables management
* [X] Basic HTTP proxy
* [X] DNS mirroring engine
* [ ] Cross-platform TLS support
* [ ] Traffic visualization foundation

### Phase 2: Extension Ecosystem

* [ ] Plugin architecture specification
* [ ] WASM plugin runtime
* [ ] Plugin registry protocol
* [ ] Authentication system

## License

Tidepool is distributed under the MIT License. See [LICENSE](https://tencent.yuanbao/LICENSE) for details.
