# Tidepool CLI 工具文档

本文档介绍 Tidepool 项目中的 CLI 工具组件。

## 🛠️ CLI 工具概览

Tidepool 项目提供了一套高性能的 CLI 工具，用于管理各种开发环境版本。

### 当前可用工具

#### GVM - Go 版本管理器
- **位置**: `cli/gvm/`
- **可执行文件**: `gvm` (Windows 上为 `gvm.exe`)
- **功能**: 简单快速的 Go 版本管理，支持安装、切换、卸载等操作

## 🎯 GVM - Go 版本管理器

### 核心特性

- 🚀 **极速响应** - 最小开销，最大性能
- 🎯 **简单命令** - 受 scoop/brew 包管理器启发的直观 CLI
- 🎨 **美观输出** - 彩色格式化终端输出
- 📦 **便捷安装** - 自动下载安装 Go 版本
- 🔄 **快速切换** - 瞬间切换 Go 版本
- 🧹 **清理管理** - 轻松安装、卸载和管理版本
- 📊 **状态跟踪** - 检查安装状态和配置
- 🔍 **版本发现** - 列出可用和已安装版本

### 安装和构建

```bash
# 从源码构建
cargo build --release -p gvm

# 二进制文件位于
# Windows: target/release/gvm.exe
# Unix: target/release/gvm
```

### 添加到系统 PATH

**Windows:**
```powershell
# 复制到系统目录
copy target\release\gvm.exe C:\Windows\System32\

# 或创建符号链接
New-Item -ItemType SymbolicLink -Path "C:\Windows\System32\gvm.exe" -Target "$(pwd)\target\release\gvm.exe"
```

**macOS/Linux:**
```bash
# 复制到系统目录
sudo cp target/release/gvm /usr/local/bin/

# 或创建符号链接
sudo ln -s "$(pwd)/target/release/gvm" /usr/local/bin/gvm

# 确保有执行权限
chmod +x /usr/local/bin/gvm
```

### 命令参考

#### 基本操作

```bash
# 安装 Go 版本
gvm install 1.21.3
gvm install 1.21.3 --force  # 强制重新安装

# 使用 Go 版本
gvm use 1.21.3              # 当前会话使用
gvm use 1.21.3 --global     # 全局设置（持久化）

# 列出版本
gvm list                     # 已安装版本
gvm list --available         # 可下载版本
gvm list --all               # 全部版本

# 查看信息
gvm info 1.21.3              # 版本详细信息
gvm status                   # 当前状态
gvm config                   # 配置信息
```

#### 维护命令

```bash
# 版本管理
gvm uninstall 1.20.5         # 卸载版本

# 缓存管理
gvm update                   # 更新可用版本缓存
gvm cleanup                  # 清理缓存文件
gvm cleanup --all            # 清理所有（包括已安装版本）
```

### 配置说明

#### 环境变量配置

GVM 在版本切换后会自动显示环境变量配置说明，支持以下 Shell：

**Windows:**
- PowerShell - 临时和永久配置
- 命令提示符(CMD) - 临时配置
- 系统环境变量 - 图形界面配置说明

**Linux/macOS:**
- Bash - `~/.bashrc` 或 `~/.bash_profile`
- Zsh - `~/.zshrc`
- Fish - `~/.config/fish/config.fish`
- NuShell - `~/.config/nushell/config.nu`
- 其他 Shell - 通用 Bash 语法

#### 配置示例

**Linux/macOS (Bash):**
```bash
# 临时配置
export GOROOT="/home/user/.gvm/versions/1.21.3"
export PATH="/home/user/.gvm/versions/1.21.3/bin:$PATH"

# 永久配置（添加到 ~/.bashrc）
echo 'export GOROOT="/home/user/.gvm/versions/1.21.3"' >> ~/.bashrc
echo 'export PATH="/home/user/.gvm/versions/1.21.3/bin:$PATH"' >> ~/.bashrc
```

**Windows (PowerShell):**
```powershell
# 临时配置
$env:GOROOT = "C:\gvm\versions\1.21.3"
$env:PATH = "C:\gvm\versions\1.21.3\bin;$env:PATH"

# 永久配置（添加到 $PROFILE）
Add-Content $PROFILE '$env:GOROOT = "C:\gvm\versions\1.21.3"'
Add-Content $PROFILE '$env:PATH = "C:\gvm\versions\1.21.3\bin;$env:PATH"'
```

### 功能特色

#### 自动检测功能
- 根据 `$SHELL` 环境变量自动检测用户当前使用的 shell
- 提供临时配置和永久配置两种选择
- 针对不同操作系统显示相应的配置方法
- 为不同 shell 显示正确的语法格式
- 自动计算 GOROOT 和 PATH 的正确值

#### 跨平台支持
- **Windows**: ZIP 格式下载，使用 Junction 链接
- **Unix系统**: tar.gz 格式下载，使用符号链接
- **多架构**: 支持 amd64 和 arm64 架构

## 🔧 开发指南

### 项目结构

```
cli/
├── README.md           # 已删除，内容移至此文档
└── gvm/                # Go 版本管理器
    ├── Cargo.toml      # 项目配置
    ├── src/            # 源代码
    │   ├── main.rs     # 主入口点
    │   ├── cli.rs      # CLI 定义
    │   ├── commands.rs # 命令实现
    │   ├── config.rs   # 配置管理
    │   └── ui.rs       # UI 工具
    ├── examples/       # 使用示例
    │   └── env_demo.rs # 环境配置演示
    └── tests/          # 集成测试
        ├── environment_tests.rs
        └── environment_integration_tests.rs
```

### 构建和测试

```bash
# 构建所有 CLI 工具
cargo build --release

# 测试特定工具
cargo test -p gvm

# 运行示例
cargo run --example env_demo --package gvm

# 完整质量检查
cargo fmt; cargo check --workspace; cargo clippy --workspace -- -D warnings; cargo test --workspace
```

### 添加新的 CLI 工具

向 CLI 目录添加新工具时：

1. 在 `cli/` 下创建新目录
2. 添加 `Cargo.toml` 配置二进制目标
3. 将其添加到工作空间根目录的 `Cargo.toml` 中
4. 遵循现有的项目结构和编码约定
5. 添加适当的文档和示例
6. 更新此文档包含新工具信息

### 发布

构建好的二进制文件位于 `target/release/` 目录：

- **Windows**: `gvm.exe` - Go 版本管理器
- **Unix**: `gvm` - Go 版本管理器

这些可以分发给最终用户或添加到 PATH 以供系统范围使用。

## 📚 相关文档

- [项目主文档](../README.md) - 项目概览和快速开始
- [版本管理器核心库文档](tidepool-version-manager.md) - 核心库使用说明
- [测试组织文档](test-organization.md) - 测试开发规范
- [环境配置功能文档](environment-setup-feature.md) - 环境变量配置说明
- [项目重组说明](project-reorganization.md) - 项目结构演变历史
