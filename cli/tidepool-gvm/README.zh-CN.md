# tidepool-gvm

> 📖 **Language**: [English](README.md) | [中文](README.zh-CN.md)

Go 版本管理的命令行接口，提供直观友好的方式在跨平台环境中安装、切换和管理 Go 版本。

## 概述

`tidepool-gvm` 是 Tidepool 项目的 CLI 组件，提供 `gvm` 命令。它基于 `tidepool-version-manager` 核心库构建，为 Go 版本管理提供现代化的包管理器风格界面。

## 安装

### 通过 Cargo

```bash
cargo install tidepool-gvm
```

这会将二进制文件安装为 `gvm` 命令。

### 从源码构建

```bash
git clone https://github.com/Slothtron/tidepool.git
cd tidepool
cargo build --release --package tidepool-gvm
```

二进制文件将位于 `target/release/gvm`（Windows 上为 `gvm.exe`）。

## 使用方法

### 基本命令

```bash
# 安装 Go 版本
gvm install 1.21.3

# 切换到指定 Go 版本
gvm use 1.21.3

# 列出已安装版本
gvm list

# 列出可下载的版本
gvm list --available

# 显示当前状态
gvm status

# 显示版本信息
gvm info 1.21.3

# 卸载版本
gvm uninstall 1.20.5

# 更新可用版本缓存
gvm update

# 显示配置
gvm config

# 显示帮助
gvm --help
```

### 高级选项

```bash
# 强制安装（覆盖现有版本）
gvm install 1.21.3 --force

# 全局设置（在终端间持久化）
gvm use 1.21.3 --global

# 自定义安装目录
gvm install 1.21.3 --install-dir /opt/go-versions

# 详细输出
gvm install 1.21.3 --verbose
```

## 特性

- **🔄 版本管理**: 安装、切换和卸载 Go 版本
- **🚀 快速操作**: 异步下载，带进度显示
- **🛡️ 安全保护**: 防止意外删除活动版本
- **🌍 跨平台**: 支持 Windows、macOS 和 Linux
- **🎨 现代界面**: 彩色终端输出和进度指示器
- **⚙️ 环境管理**: 自动配置 GOROOT、GOPATH 和 PATH

## 配置

GVM 将配置存储在平台特定目录中：

- **Windows**: `%APPDATA%\gvm\config.toml`
- **macOS/Linux**: `~/.config/gvm/config.toml`

### 配置示例

```toml
[gvm]
install_dir = "/usr/local/go-versions"
download_dir = "/tmp/gvm-downloads"
mirror = "official"
cleanup_downloads = true
concurrent_connections = 4
```

## 环境变量

切换到 Go 版本后，GVM 会自动配置：

```bash
GOROOT="/usr/local/go-versions/1.21.3"
GOPATH="$HOME/go"  # 如果尚未设置
PATH="$GOROOT/bin:$GOPATH/bin:$PATH"
```

## 开发

此 CLI 工具使用以下技术构建：

- **[clap](https://crates.io/crates/clap)**: 命令行参数解析
- **[tokio](https://crates.io/crates/tokio)**: 异步运行时
- **[indicatif](https://crates.io/crates/indicatif)**: 进度条
- **[console](https://crates.io/crates/console)**: 终端样式
- **[tidepool-version-manager](../../../crates/tidepool-version-manager/)**: 核心功能

### 项目结构

```
cli/tidepool-gvm/
├── Cargo.toml          # 包配置
├── src/
│   ├── main.rs         # 主入口点
│   ├── lib.rs          # 库接口
│   ├── cli.rs          # 命令行解析
│   ├── commands.rs     # 命令实现
│   ├── config.rs       # 配置管理
│   └── ui.rs          # 用户界面辅助
├── examples/           # 使用示例
└── tests/             # 集成测试
```

### 构建

```bash
# 开发构建
cargo build --package tidepool-gvm

# 发布构建
cargo build --release --package tidepool-gvm

# 运行测试
cargo test --package tidepool-gvm

# 使用调试日志运行
RUST_LOG=debug cargo run --package tidepool-gvm -- install 1.21.3
```

## 架构

CLI 遵循清洁架构模式：

1. **CLI 层** (`cli.rs`): 使用 clap 解析命令行参数
2. **命令层** (`commands.rs`): 实现每个命令的业务逻辑
3. **UI 层** (`ui.rs`): 处理用户界面和终端输出
4. **配置层** (`config.rs`): 管理应用程序配置
5. **核心层**: 使用 `tidepool-version-manager` 进行实际版本管理

## 错误处理

CLI 提供用户友好的错误消息和建议：

```bash
$ gvm use 1.21.3
❌ Go 版本 1.21.3 未安装

💡 建议:
   1. 先安装它: gvm install 1.21.3
   2. 列出可用版本: gvm list --available
```

## 许可证

本项目基于 MIT 许可证。详见 [LICENSE](../../LICENSE) 文件。

## 贡献

欢迎贡献！请查看主项目的[贡献指南](../../CONTRIBUTING.md)了解详情。
