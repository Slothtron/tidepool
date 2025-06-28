# Tidepool

> 📖 **Language**: [English](README.md) | [中文](README.zh-CN.md)

![License](https://img.shields.io/badge/License-MIT-yellow.svg)
![Rust](https://img.shields.io/badge/Rust-1.70%2B-blue.svg)
![Build Status](https://github.com/Slothtron/tidepool/workflows/CI/badge.svg)
![Release](https://github.com/Slothtron/tidepool/workflows/Release/badge.svg)

使用 Rust 编写的高性能 Go 版本管理工具包，支持跨平台无缝的 Go 版本安装、切换和管理。

## 🚀 快速开始

### 通过 Cargo 安装

```bash
cargo install tidepool-gvm
```

### 下载预编译二进制文件

从 [GitHub Releases](https://github.com/Slothtron/tidepool/releases) 下载：

```bash
# Linux/macOS
curl -L https://github.com/Slothtron/tidepool/releases/latest/download/gvm-<target>.tar.gz | tar xz
sudo mv gvm /usr/local/bin/

# Windows: 下载并解压 gvm-x86_64-pc-windows-msvc.zip
# 将 gvm.exe 添加到 PATH
```

### 基本用法

```bash
# 安装并切换到指定 Go 版本
gvm install 1.21.3

# 强制重新安装（如果版本已存在）
gvm install 1.21.3 --force

# 列出已安装的 Go 版本
gvm list

# 显示可用版本（未安装的）
gvm list --available

# 显示当前 Go 版本和环境信息
gvm status

# 显示指定 Go 版本的详细信息
gvm info 1.21.3

# 卸载指定 Go 版本
gvm uninstall 1.21.3

# 显示帮助信息
gvm --help
```

## 核心特性

## 📁 项目结构

```
tidepool/
├── crates/
│   └── tidepool-version-manager/   # Go 版本管理核心库
└── cli/
    └── tidepool-gvm/              # CLI 工具 (二进制名: gvm)
```

| 组件 | 描述 |
|------|------|
| `tidepool-version-manager` | Go 版本管理核心库 |
| `tidepool-gvm` | 命令行接口 (安装为 `gvm` 命令) |

## ✨ 特性

- **多平台支持**: Windows、macOS 和 Linux
- **快速下载**: 异步并发下载，带进度显示
- **版本管理**: 安装、切换和卸载 Go 版本
- **安全保护**: SHA256 验证和防止意外删除保护
- **环境管理**: 自动 GOROOT、GOPATH 和 PATH 配置

## 🔧 开发

### 从源码构建

```bash
git clone https://github.com/Slothtron/tidepool.git
cd tidepool

# 构建 CLI 工具
cargo build --release --package tidepool-gvm

# 运行测试
cargo test
```

### 系统要求

- Rust 1.70+
- 网络连接用于下载 Go 版本
- 支持平台: Windows 10+、macOS 10.15+、Linux (x86_64, ARM64)

## 📄 许可证

本项目基于 MIT 许可证。详见 [LICENSE](LICENSE) 文件。

## 🤝 贡献


