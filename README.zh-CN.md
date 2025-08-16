# Tidepool

> 📖 **Language**: [English](README.md) | [中文](README.zh-CN.md)

![License](https://img.shields.io/badge/License-MIT-yellow.svg)
![Rust](https://img.shields.io/badge/Rust-1.70%2B-blue.svg)
![Build Status](https://github.com/Slothtron/tidepool/workflows/CI/badge.svg)
![Release](https://github.com/Slothtron/tidepool/workflows/Release/badge.svg)

使用 Rust 编写的简洁、高性能 Go 版本管理工具。以简单性为设计理念，提供快速可靠的跨平台 Go 版本安装、切换和管理功能。

## 🚀 快速开始

```bash
# 从源码安装
git clone https://github.com/Slothtron/tidepool.git
cd tidepool
cargo install --path .

# 基本用法
gvm install 1.21.3         # 安装 Go 版本
gvm use 1.21.3             # 切换到指定版本
gvm list                   # 列出已安装版本
gvm list --all             # 列出所有可用版本
gvm status                 # 显示当前版本状态
gvm uninstall 1.21.3      # 卸载指定版本
gvm info 1.21.3           # 显示版本详细信息
gvm --help                 # 显示所有命令帮助
```

### ✨ 简洁清晰的输出

```bash
# 清晰的状态显示
$ gvm status
[OK] 当前版本: Go 1.23.10
  安装路径: C:\Users\User\.gvm\versions\1.23.10
[INFO] Go 环境已配置
[TIP] 使用 'go version' 验证安装

# 简洁的列表显示
$ gvm list
> 已安装的 Go 版本
  - 1.21.3
  * 1.23.10 (当前版本)
[INFO] 总计: 2 个版本
[TIP] 使用 gvm use <版本> 切换版本
```

## 📖 命令参考

| 命令                  | 描述                           | 使用示例                      |
| -------------------- | ------------------------------ | ----------------------------- |
| `gvm install <版本>` | 安装指定的 Go 版本             | `gvm install 1.22.1 --force` |
| `gvm use <版本>`     | 切换到已安装的 Go 版本         | `gvm use 1.22.1 --global`    |
| `gvm uninstall <版本>` | 卸载指定的 Go 版本           | `gvm uninstall 1.21.3`       |
| `gvm list`           | 列出已安装的 Go 版本           | `gvm list --all`             |
| `gvm status`         | 显示当前 Go 版本和环境状态     | `gvm status --verbose`       |
| `gvm info <版本>`    | 显示指定版本的详细信息         | `gvm info 1.22.1`            |
| `gvm --help`         | 显示所有命令的帮助信息         | `gvm --help`                 |
| `gvm --version`      | 显示 GVM 版本                  | `gvm --version`              |

### 全局选项

| 选项            | 描述                     | 使用示例                      |
| --------------- | ------------------------ | ----------------------------- |
| `-v, --verbose` | 启用详细输出模式         | `gvm status --verbose`       |
| `-q, --quiet`   | 启用静默模式（仅显示错误） | `gvm install 1.21.3 --quiet` |

## 📁 项目结构

```
tidepool-gvm/
├── src/                         # 源代码目录
│   ├── main.rs                  # CLI 入口点
│   ├── lib.rs                   # 库入口点
│   ├── cli.rs                   # CLI 命令解析和分发
│   ├── commands.rs              # 命令实现逻辑
│   ├── config.rs                # 配置管理
│   ├── go.rs                    # Go 版本管理核心
│   ├── downloader.rs            # 文件下载功能
│   ├── symlink.rs               # 符号链接处理
│   ├── platform.rs              # 平台检测和适配
│   ├── error.rs                 # 统一错误处理
│   ├── ui_flat.rs               # 简化的UI系统
│   └── progress_flat.rs         # 简化的进度系统
├── examples/                    # 使用示例
│   └── modern_ui_demo.rs        # UI演示
├── README.md                    # 英文文档
├── README.zh-CN.md              # 中文文档
├── Cargo.toml                   # Rust 包配置文件
├── Cargo.lock                   # 锁定依赖版本
└── rustfmt.toml                 # Rust 格式化配置
```

## ✨ 核心特性

- **🌐 多平台支持**: Windows、macOS 和 Linux
- **⚡ 高性能**: 优化的异步操作，快速下载
- **🔧 完整管理**: 安装、切换和卸载 Go 版本
- **🛡️ 安全优先**: SHA256 验证和防止意外删除保护
- **⚙️ 智能环境**: 自动 GOROOT、GOPATH 和 PATH 配置
- **📦 简洁架构**: 清晰的代码结构，最少依赖
- **🎯 用户友好**: 简洁的CLI，一致的命令和清晰的输出
- **🚀 跨平台稳定**: 稳定的ASCII输出，无Unicode依赖

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
