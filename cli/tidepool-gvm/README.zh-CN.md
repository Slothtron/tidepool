# tidepool-gvm

> 📖 **Language**: [English](README.md) | [中文](README.zh-CN.md)

[![Crates.io](https://img.shields.io/crates/v/tidepool-gvm.svg)](https://crates.io/crates/tidepool-gvm)
[![License](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

Go 版本管理的命令行接口，提供直观友好的方式在跨平台环境中安装、切换和管理 Go 版本。

## ✨ 特性

- **🔄 版本管理** - 安装、切换和卸载 Go 版本
- **🚀 快速操作** - 异步下载，带进度显示
- **🛡️ 安全保护** - 防止意外删除活动版本
- **🌍 跨平台** - 支持 Windows、macOS 和 Linux
- **🎨 现代界面** - 彩色终端输出和进度指示器
- **⚙️ 环境管理** - 自动配置 GOROOT、GOPATH 和 PATH

## 📦 安装

### 通过 Cargo

```bash
cargo install tidepool-gvm
```

### 从源码构建

```bash
git clone https://github.com/Slothtron/tidepool.git
cd tidepool
cargo build --release --package tidepool-gvm
```

二进制文件将位于 `target/release/gvm`（Windows 上为 `gvm.exe`）。

## 🚀 快速开始

```bash
# 安装并切换到指定 Go 版本
gvm install 1.21.3

# 列出已安装版本
gvm list

# 显示当前状态
gvm status

# 列出可用版本
gvm list --available

# 显示帮助
gvm --help
```

## 📚 命令

| 命令 | 描述 |
|------|------|
| `gvm install <版本>` | 安装并切换到指定 Go 版本 |
| `gvm list` | 列出已安装版本 |
| `gvm list --available` | 列出可下载的版本 |
| `gvm status` | 显示当前 Go 版本和环境信息 |
| `gvm info <版本>` | 显示指定版本的详细信息 |
| `gvm uninstall <版本>` | 卸载指定版本 |

### 选项

- `--force, -f` - 强制重新安装现有版本
- `--help, -h` - 显示帮助信息
- `--version, -V` - 显示版本信息

## 🏗️ 架构

基于 [`tidepool-version-manager`](../../crates/tidepool-version-manager/) 核心库构建。

```
cli/tidepool-gvm/
├── src/
│   ├── main.rs         # 主入口点
│   ├── cli.rs          # 命令行解析
│   ├── commands.rs     # 命令实现
│   ├── config.rs       # 配置管理
│   └── ui.rs          # 用户界面辅助
└── tests/             # 集成测试
```

## 🧪 开发

```bash
# 开发构建
cargo build --package tidepool-gvm

# 运行测试
cargo test --package tidepool-gvm

# 使用调试日志运行
RUST_LOG=debug cargo run --package tidepool-gvm -- install 1.21.3
```

## 🤝 贡献指南

欢迎贡献！请查看 [贡献指南](../../CONTRIBUTING.md) 了解详情。

## � 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](../../LICENSE) 文件了解详情。

---

**由 [Tidepool 项目](https://github.com/Slothtron/tidepool) 维护** 🌊
