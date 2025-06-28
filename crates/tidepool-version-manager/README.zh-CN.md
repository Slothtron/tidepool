# Tidepool Version Manager

> 📖 **Language**: [English](README.md) | [中文](README.zh-CN.md)

[![Crates.io](https://img.shields.io/crates/v/tidepool-version-manager.svg)](https://crates.io/crates/tidepool-version-manager)
[![Documentation](https://docs.rs/tidepool-version-manager/badge.svg)](https://docs.rs/tidepool-version-manager)
[![License](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-blue.svg)]()

`tidepool-version-manager` 是 Tidepool 项目的核心库，提供了强大的运行时版本管理功能。它支持多种编程语言运行时的安装、切换、卸载和管理，目前专注于 Go 版本管理，未来将扩展支持 Python、Node.js 等更多运行时。

## 🌟 特性

### 🚀 核心功能
- **异步版本安装** - 高性能的并发下载和安装
- **智能版本切换** - 跨平台的版本切换机制
- **完整生命周期管理** - 安装、切换、卸载、列表查看
- **状态查询** - 实时运行时状态检查
- **多平台支持** - Windows、macOS、Linux 全平台兼容

### 🔧 下载器特性
- **分片并发下载** - 多连接加速下载
- **断点续传** - 网络中断自动恢复
- **进度报告** - 实时下载进度反馈
- **文件完整性验证** - SHA256 校验确保安全
- **智能重试机制** - 自动处理网络异常

### 🛡️ 安全特性
- **文件哈希验证** - 自动验证下载文件的完整性
- **权限安全** - Windows 无需管理员权限的 Junction 链接
- **安全卸载** - 防护机制避免意外删除系统文件

## 📦 安装

将以下内容添加到您的 `Cargo.toml`:

```toml
[dependencies]
tidepool-version-manager = "0.1.3"
```

## 🚀 快速开始

### 基本使用

```rust
use tidepool_version_manager::{
    go::GoManager, 
    VersionManager, 
    InstallRequest, 
    SwitchRequest
};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建 Go 版本管理器
    let go_manager = GoManager::new();
    
    // 安装 Go 1.21.0
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

### 查询可用版本

```rust
use tidepool_version_manager::{go::GoManager, VersionManager};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let go_manager = GoManager::new();
    
    // 获取可用版本列表
    let available_versions = go_manager.list_available().await?;
    println!("📋 可用的 Go 版本 ({} 个):", available_versions.total_count);
    
    for version in available_versions.versions.iter().take(10) {
        println!("   - {}", version);
    }
    
    Ok(())
}
```

### 状态查询

```rust
use tidepool_version_manager::{go::GoManager, VersionManager, StatusRequest};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let go_manager = GoManager::new();
    
    let status_request = StatusRequest {
        base_dir: Some(PathBuf::from("/usr/local/go-versions")),
    };
    
    let status = go_manager.status(status_request)?;
    
    if let Some(version) = status.current_version {
        println!("🎯 当前 Go 版本: {}", version);
        
        if let Some(path) = status.install_path {
            println!("📁 安装路径: {}", path.display());
        }
        
        println!("🌍 环境变量:");
        for (key, value) in status.environment_vars {
            println!("   {}={}", key, value);
        }
    } else {
        println!("❌ 未检测到已安装的 Go 版本");
    }
    
    Ok(())
}
```

## 🔧 高级配置

### 自定义下载器配置

```rust
use tidepool_version_manager::downloader::{Downloader, DownloadConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建自定义下载配置
    let config = DownloadConfig {
        concurrent_connections: 8,        // 并发连接数
        timeout_seconds: 300,             // 超时时间（秒）
        enable_chunked_download: true,    // 启用分片下载
        max_retries: 3,                   // 最大重试次数
        min_chunk_size: 10 * 1024 * 1024, // 最小分片大小（10MB）
        ..Default::default()
    };
    
    let downloader = Downloader::with_config(config);
    
    // 使用自定义下载器进行下载
    let url = "https://go.dev/dl/go1.21.0.linux-amd64.tar.gz";
    let output_path = "/tmp/go1.21.0.linux-amd64.tar.gz";
    
    downloader.download(url, output_path, None).await?;
    println!("✅ 下载完成: {}", output_path);
    
    Ok(())
}
```

### 进度回调

```rust
use tidepool_version_manager::downloader::{Downloader, ProgressReporter};

struct MyProgressReporter;

impl ProgressReporter for MyProgressReporter {
    fn report_progress(&self, downloaded: u64, total: Option<u64>) {
        if let Some(total) = total {
            let percentage = (downloaded as f64 / total as f64) * 100.0;
            println!("📊 下载进度: {:.1}% ({}/{})", percentage, downloaded, total);
        } else {
            println!("📊 已下载: {} 字节", downloaded);
        }
    }
    
    fn report_error(&self, error: &str) {
        eprintln!("❌ 下载错误: {}", error);
    }
    
    fn report_completion(&self) {
        println!("✅ 下载完成!");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let downloader = Downloader::new();
    let progress_reporter = MyProgressReporter;
    
    let url = "https://go.dev/dl/go1.21.0.linux-amd64.tar.gz";
    let output_path = "/tmp/go1.21.0.linux-amd64.tar.gz";
    
    downloader.download(url, output_path, Some(&progress_reporter)).await?;
    
    Ok(())
}
```

## 🏗️ 架构设计

### 核心组件

```
tidepool-version-manager/
├── src/
│   ├── lib.rs              # 公共接口和类型定义
│   ├── go.rs               # Go 版本管理实现
│   └── downloader.rs       # 通用下载器模块
├── examples/               # 使用示例
└── tests/                  # 集成测试
```

### 特性说明

- **`VersionManager` 特征** - 统一的版本管理接口，支持扩展其他运行时
- **`GoManager`** - Go 语言版本管理的具体实现
- **`Downloader`** - 高性能异步下载器，支持断点续传和进度报告
- **跨平台支持** - Windows Junction 和 Unix 符号链接的统一抽象

## 🧪 运行示例

项目包含了多个示例程序，演示不同的使用场景：

```bash
# 下载器功能演示
cargo run --example downloader_test

# 哈希验证演示
cargo run --example hash_verification_demo

# Windows Junction 演示（仅 Windows）
cargo run --example junction_demo

# 临时文件处理演示
cargo run --example temp_file_demo

# 卸载保护演示
cargo run --example uninstall_protection_demo
```

## 🧪 测试

运行完整的测试套件：

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test go_manager_tests

# 运行集成测试
cargo test --test integration_tests
```

## 🔄 版本兼容性

| Version | Rust Version | Features |
|---------|-------------|----------|
| 0.1.3   | 1.70+       | Go 版本管理、高性能下载器 |
| 0.1.2   | 1.70+       | 基础版本管理功能 |
| 0.1.1   | 1.70+       | 初始版本 |

## 🚧 未来计划

- [ ] **Python 版本管理** - 支持 Python/pyenv 兼容
- [ ] **Node.js 版本管理** - 支持 Node.js/nvm 兼容  
- [ ] **配置文件支持** - 项目级别的版本配置
- [ ] **插件系统** - 自定义版本管理扩展
- [ ] **镜像源支持** - 国内镜像源加速下载

## 📋 API 文档

完整的 API 文档可在 [docs.rs](https://docs.rs/tidepool-version-manager) 查看。

### 主要类型

- [`VersionManager`](https://docs.rs/tidepool-version-manager/latest/tidepool_version_manager/trait.VersionManager.html) - 版本管理器特征
- [`GoManager`](https://docs.rs/tidepool-version-manager/latest/tidepool_version_manager/go/struct.GoManager.html) - Go 版本管理器
- [`Downloader`](https://docs.rs/tidepool-version-manager/latest/tidepool_version_manager/downloader/struct.Downloader.html) - 高性能下载器
- [`InstallRequest`](https://docs.rs/tidepool-version-manager/latest/tidepool_version_manager/struct.InstallRequest.html) - 安装请求结构
- [`RuntimeStatus`](https://docs.rs/tidepool-version-manager/latest/tidepool_version_manager/struct.RuntimeStatus.html) - 运行时状态信息

## 🤝 贡献指南

我们欢迎各种形式的贡献！请查看 [CONTRIBUTING.md](../../CONTRIBUTING.md) 了解详细信息。

### 开发环境设置

```bash
# 克隆仓库
git clone https://github.com/Slothtron/tidepool.git
cd tidepool/crates/tidepool-version-manager

# 运行测试
cargo test

# 检查代码格式
cargo fmt --check

# 运行 Clippy 检查
cargo clippy --all-targets --all-features
```

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](../../LICENSE) 文件了解详情。

## 🔗 相关链接

- [GitHub 仓库](https://github.com/Slothtron/tidepool)
- [问题跟踪](https://github.com/Slothtron/tidepool/issues)
- [发布页面](https://github.com/Slothtron/tidepool/releases)

---

**由 [Tidepool 项目](https://github.com/Slothtron/tidepool) 维护** 🌊
