# Tidepool Version Manager

> 📖 **Language**: [English](README.md) | [中文](README.zh-CN.md)

[![Crates.io](https://img.shields.io/crates/v/tidepool-version-manager.svg)](https://crates.io/crates/tidepool-version-manager)
[![Documentation](https://docs.rs/tidepool-version-manager/badge.svg)](https://docs.rs/tidepool-version-manager)
[![License](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-blue.svg)]()

Tidepool 项目的核心库，提供强大的 Go 版本管理功能，具有高性能异步操作和跨平台支持。

## ✨ 核心特性

- **🚀 异步操作** - 高性能并发下载和安装
- **🔄 版本管理** - 安装、切换、卸载和列表 Go 版本
- **🌐 跨平台** - 支持 Windows、macOS 和 Linux
- **⚡ 智能下载** - 分片下载支持断点续传和进度报告
- **🛡️ 安全保护** - SHA256 验证和安全卸载保护
- **🔗 符号链接管理** - 跨平台符号链接无需管理员权限

## 📦 安装

添加到您的 `Cargo.toml`:

```toml
[dependencies]
tidepool-version-manager = "0.1.5"
```

## 🚀 快速开始

```rust
use tidepool_version_manager::{go::GoManager, VersionManager, InstallRequest, SwitchRequest};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let go_manager = GoManager::new();

    // 安装 Go 版本
    let install_request = InstallRequest {
        version: "1.21.0".to_string(),
        install_dir: PathBuf::from("/usr/local/go-versions"),
        download_dir: PathBuf::from("/tmp/go-downloads"),
        force: false,
    };

    let version_info = go_manager.install(install_request).await?;
    println!("✅ 已安装 Go {}", version_info.version);

    // 切换到该版本
    let switch_request = SwitchRequest {
        version: "1.21.0".to_string(),
        base_dir: PathBuf::from("/usr/local/go-versions"),
        global: true,
        force: false,
    };

    go_manager.switch_to(switch_request)?;
    println!("🔄 已切换到 Go 1.21.0");

    Ok(())
}
```

## 🏗️ 架构

```
tidepool-version-manager/
├── src/
│   ├── lib.rs              # 公共接口和类型定义
│   ├── go.rs               # Go 版本管理实现
│   ├── downloader.rs       # 通用下载器模块
│   └── symlink.rs          # 跨平台符号链接管理
└── tests/                  # 集成测试
```

## 📚 API 文档

完整文档: [docs.rs/tidepool-version-manager](https://docs.rs/tidepool-version-manager)

### 主要类型

- [`VersionManager`](https://docs.rs/tidepool-version-manager/latest/tidepool_version_manager/trait.VersionManager.html) - 版本管理器特征
- [`GoManager`](https://docs.rs/tidepool-version-manager/latest/tidepool_version_manager/go/struct.GoManager.html) - Go 版本管理器
- [`Downloader`](https://docs.rs/tidepool-version-manager/latest/tidepool_version_manager/downloader/struct.Downloader.html) - 高性能下载器

## 🧪 开发

```bash
# 运行测试
cargo test

# 检查代码质量
cargo fmt --check
cargo clippy --all-targets --all-features
```

## 🤝 贡献指南

我们欢迎各种形式的贡献！请查看 [贡献指南](../../CONTRIBUTING.md) 了解详情。

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](../../LICENSE) 文件了解详情。

---

**由 [Tidepool 项目](https://github.com/Slothtron/tidepool) 维护** 🌊