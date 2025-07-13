# Tidepool

> 📖 **Language**: [English](README.md) | [中文](README.zh-CN.md)

![License](https://img.shields.io/badge/License-MIT-yellow.svg)
![Rust](https://img.shields.io/badge/Rust-1.70%2B-blue.svg)
![Build Status](https://github.com/Slothtron/tidepool/workflows/CI/badge.svg)
![Release](https://github.com/Slothtron/tidepool/workflows/Release/badge.svg)

使用 Rust 编写的高性能 Go 版本管理工具包，支持跨平台无缝的 Go 版本安装、切换和管理。

## 🚀 快速开始

```bash
# 安装 CLI 工具
cargo install tidepool-gvm

# 基本用法
gvm install 1.21.3    # 安装 Go 版本
gvm list              # 列出已安装版本
gvm status            # 显示当前版本
gvm --help            # 显示所有命令
```

详细安装选项和完整使用指南，请参见 [CLI 文档](cli/tidepool-gvm/README.zh-CN.md)。

## 📁 项目结构

```
tidepool/
├── crates/
│   └── tidepool-version-manager/   # Go 版本管理核心库
└── cli/
    └── tidepool-gvm/              # CLI 工具 (二进制名: gvm)
```

### 组件说明

| 组件 | 描述 | 文档 |
|------|------|------|
| **[tidepool-version-manager](crates/tidepool-version-manager/)** | 提供 Go 版本管理功能的核心库 | [📖 库文档](crates/tidepool-version-manager/README.zh-CN.md) |
| **[tidepool-gvm](cli/tidepool-gvm/)** | 命令行接口工具 (安装为 `gvm` 命令) | [📖 CLI 文档](cli/tidepool-gvm/README.zh-CN.md) |

## ✨ 核心特性

- **🌐 多平台支持**: Windows、macOS 和 Linux
- **⚡ 高性能**: 异步并发下载，带进度显示
- **🔧 完整管理**: 安装、切换和卸载 Go 版本  
- **🛡️ 安全优先**: SHA256 验证和防止意外删除保护
- **⚙️ 智能环境**: 自动 GOROOT、GOPATH 和 PATH 配置

## 🔧 开发

### 快速开发环境设置

```bash
git clone https://github.com/Slothtron/tidepool.git
cd tidepool

# 构建所有组件
cargo build --release

# 运行测试
cargo test

# 构建特定组件
cargo build --release --package tidepool-gvm
```

### 系统要求

- **Rust**: 1.70+
- **网络**: 需要互联网连接下载 Go 版本
- **平台**: Windows 10+、macOS 10.15+、Linux (x86_64, ARM64)

详细的开发环境设置和贡献指南，请参见各组件文档。

## 📄 许可证

本项目基于 MIT 许可证。详见 [LICENSE](LICENSE) 文件。

## 🤝 贡献

欢迎贡献！请随时提交 Pull Request。


