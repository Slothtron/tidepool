# Tidepool

> 📖 **Language**: [English](README.md) | [中文](README.zh-CN.md)

![License](https://img.shields.io/badge/License-MIT-yellow.svg)
![Rust](https://img.shields.io/badge/Rust-1.70%2B-blue.svg)
![Build Status](https://github.com/Slothtron/tidepool/workflows/CI/badge.svg)
![Release](https://github.com/Slothtron/tidepool/workflows/Release/badge.svg)

使用 Rust 编写的高性能 Go 版本管理工具包，支持跨平台无缝的 Go 版本安装、切换和管理。

## 🚀 快速开始

```bash
# 从源码安装
git clone https://github.com/Slothtron/tidepool.git
cd tidepool
cargo install --path .

# 基本用法
gvm install 1.21.3    # 安装 Go 版本
gvm list              # 列出已安装版本
gvm status            # 显示当前版本
gvm --help            # 显示所有命令
```

## 📁 项目结构

```
tidepool/
├── src/
│   ├── main.rs                   # CLI 入口点
│   ├── lib.rs                    # 库入口点
│   ├── cli.rs                    # CLI 命令解析
│   ├── commands.rs               # 命令实现
│   ├── config.rs                 # 配置管理
│   ├── ui.rs                     # 用户界面
│   ├── go.rs                     # Go 版本管理核心
│   ├── downloader.rs             # 下载器
│   └── symlink.rs                # 符号链接处理
├── README.md                     # 文档
├── README.zh-CN.md              # 中文文档
├── Cargo.toml                    # 包配置
└── .github/                      # GitHub 工作流
```

## ✨ 核心特性

- **🌐 多平台支持**: Windows、macOS 和 Linux
- **⚡ 高性能**: 异步并发下载，带进度显示
- **🔧 完整管理**: 安装、切换和卸载 Go 版本
- **🛡️ 安全优先**: SHA256 验证和防止意外删除保护
- **⚙️ 智能环境**: 自动 GOROOT、GOPATH 和 PATH 配置
- **📦 简洁架构**: 单一 crate 设计，易于维护

## 🔧 开发

### 快速开发环境设置

```bash
git clone https://github.com/Slothtron/tidepool.git
cd tidepool

# 构建项目
cargo build --release

# 运行测试
cargo test

# 使用调试日志运行
RUST_LOG=debug cargo run -- install 1.21.3
```

### 系统要求

- **Rust**: 1.70+
- **网络**: 需要互联网连接下载 Go 版本
- **平台**: Windows 10+、macOS 10.15+、Linux (x86_64, ARM64)

### 跨平台构建

```bash
# 构建当前平台
cargo build --release

# 交叉编译（需要目标工具链）
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-pc-windows-msvc
```

## 📄 许可证

本项目基于 MIT 许可证。详见 [LICENSE](LICENSE) 文件。

## 🤝 贡献

欢迎贡献！请随时提交 Pull Request。
