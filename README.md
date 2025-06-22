# Tidepool - Go Version Manager

![License](https://img.shields.io/badge/License-MIT-yellow.svg)
![Rust](https://img.shields.io/badge/Rust-1.70%2B-blue.svg)
![Build Status](https://github.com/Slothtron/tidepool/workflows/CI/badge.svg)
![Release](https://github.com/Slothtron/tidepool/workflows/Release/badge.svg)
![Downloads](https://img.shields.io/github/downloads/Slothtron/tidepool/total)

Tidepool 是一个高性能的 Go 版本管理工具包，支持多平台，提供无缝的 Go 版本安装、切换和管理体验。采用 Rust 编写，确保卓越的性能和安全性。

## 🚀 快速安装

### 二进制文件下载 (推荐)

从 [GitHub Releases](https://github.com/Slothtron/tidepool/releases) 下载适合您平台的预编译二进制文件：

```bash
# Linux x86_64
curl -L https://github.com/Slothtron/tidepool/releases/latest/download/gvm-x86_64-unknown-linux-gnu.tar.gz | tar xz
sudo mv gvm /usr/local/bin/

# macOS (Intel)
curl -L https://github.com/Slothtron/tidepool/releases/latest/download/gvm-x86_64-apple-darwin.tar.gz | tar xz
sudo mv gvm /usr/local/bin/

# macOS (Apple Silicon)
curl -L https://github.com/Slothtron/tidepool/releases/latest/download/gvm-aarch64-apple-darwin.tar.gz | tar xz
sudo mv gvm /usr/local/bin/

# Windows (PowerShell)
# 下载并解压 gvm-x86_64-pc-windows-msvc.zip
# 将 gvm.exe 添加到 PATH
```

### 从源码编译

```bash
# 克隆仓库
git clone https://github.com/Slothtron/tidepool.git
cd tidepool

# 构建发布版本
cargo build --release --package gvm

# 安装到系统 (Unix)
sudo cp target/release/gvm /usr/local/bin/
```

### 通过 Cargo 安装

```bash
cargo install gvm
```

## 核心特性

* 🔄 **多平台 Go 版本管理**
  - 支持 Windows、macOS 和 Linux 系统
  - 自动下载和安装 Go 版本（支持官方和中国镜像源）
  - 快速版本切换和版本信息查询
  - 智能版本清理和安全卸载保护
  - 自动环境变量配置 (GOROOT, GOPATH, PATH)

* 🎯 **现代化 CLI 界面**
  - 直观的命令设计，灵感来自 scoop/brew
  - 彩色终端输出，增强可读性
  - 完善的帮助系统和详细错误提示
  - 支持强制安装和全局配置选项

* 🚀 **高性能设计**
  - 异步并发下载，最大化网络利用率
  - SHA256 哈希校验，确保下载文件完整性
  - 智能进度显示，实时下载状态
  - 最小开销设计，快速执行
  - 高效的归档解压和文件管理

* 🛡️ **安全可靠**
  - 防止误删当前使用版本的保护机制
  - 下载文件完整性验证
  - 临时文件安全清理
  - 符合链接（Windows）和符号链接（Unix）支持

## 项目架构

```
tidepool/
├── crates/
│   └── tidepool-version-manager/   # Go 版本管理核心库
└── cli/
    └── gvm/                        # Go 版本管理 CLI 工具
```

### 核心组件

| 组件 | 描述 | 状态 |
| ---- | ---- | ---- |
| `tidepool-version-manager` | Go 版本管理核心库，提供版本安装、切换、下载等功能 | ✅ 已实现 |
| `gvm` | Go 版本管理 CLI 工具，提供用户友好的命令行界面 | ✅ 已实现 |

### 功能模块

* **下载器模块** - 异步并发下载，支持进度显示和断点续传
* **版本管理器** - Go 版本的安装、卸载、切换和查询
* **哈希验证** - SHA256 完整性校验，确保下载安全
* **环境配置** - 自动管理 GOROOT、GOPATH 和 PATH 环境变量
* **跨平台支持** - Windows 符合链接和 Unix 符号链接

## 快速开始

### 系统要求

* Rust 1.70+ （开发环境）
* 支持的操作系统：
  - Windows 10+ (amd64)
  - macOS 10.15+ (amd64, arm64)
  - Linux (amd64, arm64)
* 网络连接（用于下载 Go 版本）

### 安装

#### 从源码构建

```bash
# 克隆仓库
git clone https://github.com/Slothtron/tidepool.git
cd tidepool

# 构建 GVM (Go Version Manager)
cargo build --release -p gvm

# 二进制文件位于
# Windows: target/release/gvm.exe
# Unix: target/release/gvm
```

#### 安装到系统

**Windows:**
```powershell
# 复制到系统目录
copy target\release\gvm.exe C:\Windows\System32\

# 或添加到 PATH
$env:PATH += ";$(pwd)\target\release"
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

## 使用示例

### GVM - Go 版本管理器

```bash
# 安装 Go 版本
gvm install 1.21.3

# 强制重新安装（覆盖现有版本）
gvm install 1.21.3 --force

# 列出已安装版本
gvm list

# 列出可用版本（从官方获取）
gvm list --available

# 使用指定版本（临时切换）
gvm use 1.21.3

# 全局设置（持久化配置）
gvm use 1.21.3 --global

# 检查当前状态和配置
gvm status

# 显示特定版本详细信息
gvm info 1.21.3

# 卸载 Go 版本
gvm uninstall 1.20.5

# 更新可用版本缓存
gvm update

# 显示配置信息和路径
gvm config

# 显示帮助信息
gvm --help
gvm <command> --help
```

### Rust 库使用

```rust
use tidepool_version_manager::{VersionManager, go::GoManager};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化 Go 版本管理器
    let go_manager = GoManager::new();

    // 安装指定 Go 版本 (异步下载，带进度显示)
    let install_dir = PathBuf::from("/path/to/go_versions");
    let version_info = VersionManager::install_version(
        &go_manager,
        "1.21.3",
        &install_dir,
        false  // force_install
    ).await?;

    println!("安装完成: {}", version_info.filename);

    // 切换到指定 Go 版本
    VersionManager::use_version(&go_manager, "1.21.3", &install_dir, false)?;

    // 列出已安装版本
    let installed_versions = VersionManager::list_installed_versions(&go_manager, &install_dir)?;
    for version in installed_versions {
        println!("已安装: {}", version);
    }

    // 获取版本详细信息
    let info = VersionManager::get_version_info(&go_manager, "1.21.3").await?;
    println!("版本信息: {} - {}", info.version, info.filename);

    Ok(())
}
```

## 核心功能

### Go 版本管理

* **智能下载** - 从官方 Go 发布页面自动获取版本列表和下载链接
* **多源支持** - 支持官方源和中国镜像源，自动选择最佳下载源
* **完整性验证** - SHA256 哈希校验确保下载文件完整性
* **版本安装** - 自动解压和安装到指定目录
* **版本切换** - 在已安装版本间快速切换
* **环境管理** - 自动配置 GOROOT、GOPATH 和 PATH 环境变量
* **安全卸载** - 防止误删当前使用版本，提供卸载保护

### CLI 设计特色

* **包管理器风格** - 类似 brew/scoop 的直观命令（install, uninstall, use, list）
* **智能提示** - 详细的帮助信息和操作指导
* **错误处理** - 友好的错误消息和问题解决建议
* **彩色输出** - 增强视觉体验的终端输出
* **进度显示** - 实时下载进度和状态指示器

### 跨平台特性

* **Windows** - 使用符合链接（Junction）实现版本切换
* **Unix系统** - 使用符号链接实现版本切换
* **多架构** - 支持 amd64 和 arm64 架构
* **归档格式** - Windows 使用 ZIP，Unix 使用 tar.gz

## 功能开发状态

| 功能模块 | 状态 | 说明 |
| ---- | ---- | ---- |
| Go 版本安装 | ✅ 完成 | 支持自动下载和安装，带完整性验证 |
| Go 版本切换 | ✅ 完成 | 跨平台版本切换，自动环境配置 |
| 环境变量管理 | ✅ 完成 | GOROOT, GOPATH, PATH 自动管理 |
| CLI 用户界面 | ✅ 完成 | 完整命令集，彩色输出，进度显示 |
| 版本列表管理 | ✅ 完成 | 已安装和可用版本查询 |
| 安全卸载保护 | ✅ 完成 | 防止误删当前使用版本 |
| 哈希完整性验证 | ✅ 完成 | SHA256 下载文件校验 |
| 多平台支持 | ✅ 完成 | Windows, macOS, Linux 全支持 |
| 强制安装选项 | ✅ 完成 | 支持覆盖现有版本安装 |
| 配置信息查询 | ✅ 完成 | 显示当前配置和路径信息 |

### 测试覆盖

Tidepool 项目采用 **Rust 官方推荐的测试组织结构**，严格遵循 [Rust Book Ch11.3](https://doc.rust-lang.org/book/ch11-03-test-organization.html) 最佳实践：

#### 测试架构
- **单元测试** - 位于源文件内部 `#[cfg(test)]` 模块，测试私有函数和内部逻辑
- **集成测试** - 位于各包的 `tests/` 目录，只测试公开 API
- **示例程序** - 位于各包的 `examples/` 目录，演示功能使用方法

#### 测试统计
```bash
# 测试运行结果
✅ 单元测试: 6 个测试通过
   - CLI 配置测试: 3 个
   - CLI 主程序测试: 3 个

✅ 集成测试: 38 个测试通过 (1 个网络测试被跳过)
   - CLI 集成测试: 5 个
   - 根目录集成测试: 5 个
   - 版本管理器集成测试: 28 个

✅ 示例程序: 7 个示例可正常运行
   - CLI 示例: 1 个 (环境配置演示)
   - 版本管理器示例: 6 个 (各功能演示)
```

#### 测试目录结构
```
tidepool/
├── cli/gvm/tests/                          # CLI 集成测试
│   ├── environment_tests.rs
│   └── environment_integration_tests.rs
├── crates/tidepool-version-manager/tests/  # 版本管理器集成测试
│   ├── go_manager_tests.rs
│   ├── hash_verification_tests.rs
│   ├── info_command_tests.rs
│   ├── temp_file_download_tests.rs
│   └── [其他测试文件...]
├── tests/                                  # 系统级集成测试
│   ├── integration_test.rs
│   └── environment_setup_test.rs
└── examples/                               # 系统级示例
    └── README.md
```

#### 测试运行命令
```bash
# 运行所有测试 (跳过网络测试)
cargo test --workspace

# 运行包含网络测试的完整测试
cargo test --workspace -- --ignored

# 运行示例程序
cargo run --example env_demo --package gvm
cargo run --example temp_file_demo --package tidepool-version-manager
```

## 平台支持

### 完全支持
- **Windows amd64** - 完整功能，自动下载，符合链接支持
- **Windows arm64** - 完整功能（Go 1.17+）
- **macOS amd64** - 完整功能，符号链接支持
- **macOS arm64 (Apple Silicon)** - 完整功能（Go 1.16+）
- **Linux amd64** - 完整功能，符号链接支持
- **Linux arm64** - 完整功能，适配 ARM 架构

### 下载源支持
- **官方源**: https://go.dev/dl/ - 全球默认源
- **中国镜像**: 计划支持国内加速下载

### 归档格式支持
- **Windows**: ZIP 格式 (.zip)
- **Unix系统**: Gzip 压缩 tar 格式 (.tar.gz)

## 技术细节

### 下载和安装
- **下载源**: 官方 Go 二进制文件来自 https://go.dev/dl/
- **归档格式**: Windows 使用 ZIP，Unix 使用 tar.gz
- **异步操作**: 基于 Tokio 的非阻塞下载和安装
- **并发下载**: 支持分块并发下载，提升下载速度
- **完整性验证**: SHA256 哈希校验确保文件完整性

### 环境管理
- **环境变量**: 自动管理 GOROOT, GOPATH 和 PATH
- **版本切换**: Windows 使用符合链接，Unix 使用符号链接
- **配置持久化**: 支持全局和临时配置模式
- **路径管理**: 智能处理安装路径和工作目录

### 性能优化
- **内存效率**: 流式处理大文件，避免内存溢出
- **网络优化**: 支持断点续传和并发下载
- **缓存机制**: 版本信息缓存，减少网络请求
- **最小依赖**: 优先使用标准库，减少外部依赖

### 安全特性
- **下载验证**: SHA256 哈希完整性检查
- **权限控制**: 安全的文件和目录操作
- **卸载保护**: 防止误删当前使用版本
- **临时文件**: 安全的临时文件处理和清理

## 贡献指南

欢迎为 Tidepool 项目贡献代码！请查看我们的[贡献指南](CONTRIBUTING.md)了解详情。

### 开发工作流

1. Fork 仓库
2. 创建功能分支: `git checkout -b feature/amazing-feature`
3. 进行更改并添加测试（**必须遵循测试组织规范**）
4. 运行质量检查: `cargo fmt; cargo check --workspace; cargo clippy --workspace -- -D warnings; cargo test --workspace`
5. 构建发布版本: `cargo build --release`
6. 提交更改: `git commit -m "feat: add amazing feature"`
7. 推送分支: `git push origin feature/amazing-feature`
8. 提交 Pull Request

### 测试开发规范

**重要**: 本项目严格遵循 [统一开发规范](.github/instructions/unified.instructions.md) 中的测试组织要求，请在添加新测试前仔细阅读：

#### 测试位置要求
- **单元测试**: 必须在源文件内部使用 `#[cfg(test)]` 模块
- **集成测试**: 必须放在对应包的 `tests/` 目录下
- **示例程序**: 必须放在对应包的 `examples/` 目录下

#### 命名规范
- 集成测试文件: `功能名_tests.rs` (如 `go_manager_tests.rs`)
- 示例程序文件: `功能名_demo.rs` (如 `temp_file_demo.rs`)
- 测试函数: `test_功能描述` (如 `test_install_go_version`)

#### 测试质量要求
- ✅ 所有新功能必须有对应测试
- ✅ 集成测试只能使用公开 API
- ✅ 网络依赖测试使用 `#[ignore]` 标记
- ✅ 平台特定测试使用 `#[cfg(target_os)]` 标记
- ✅ 测试之间必须保持隔离性

#### 禁止行为
- ❌ 在 `src/` 目录创建独立的测试文件
- ❌ 在集成测试中使用 `crate::` 导入内部模块
- ❌ 测试之间相互依赖或共享状态
- ❌ 跳过必要的测试编写

### 代码规范

* **Rust 编码风格**: 遵循官方 Rust 编码规范
* **错误处理**: 使用 `Result<T, E>` 类型，提供有意义的错误信息
* **测试覆盖**: 为新功能编写单元测试和集成测试
* **文档**: 为公共 API 编写 rustdoc 文档
* **平台兼容**: 使用 `#[cfg(target_os)]` 处理平台特定代码

### 质量保证

* **零警告政策**: 所有代码必须通过 Clippy 检查
* **格式化**: 使用 `cargo fmt` 保持代码格式一致
* **测试要求**: 所有新功能必须有对应测试
* **文档要求**: 公共接口必须有完整文档

### 项目结构

```
tidepool/
├── Cargo.toml                      # 工作空间配置
├── Cargo.lock                      # 依赖锁定文件
├── README.md                       # 项目文档
├── rustfmt.toml                    # 代码格式化配置
├── .gitignore                      # Git 忽略文件配置
│
├── .github/                        # GitHub 配置和约束
│   └── instructions/               # 项目开发约束
│       ├── unified.instructions.md # 统一开发规范
│       └── test.organization.md    # 测试组织规范
│
├── crates/                         # 核心库
│   └── tidepool-version-manager/   # Go 版本管理库
│       ├── Cargo.toml              # 库配置
│       ├── src/                    # 库源码
│       │   ├── lib.rs              # 库入口
│       │   ├── go.rs               # Go 管理器实现
│       │   └── downloader.rs       # 下载器模块
│       ├── examples/               # 库使用示例
│       │   ├── temp_file_demo.rs   # 临时文件机制演示
│       │   ├── hash_verification_demo.rs  # 哈希验证演示
│       │   └── [其他示例...]
│       └── tests/                  # 库集成测试
│           ├── go_manager_tests.rs # Go 管理器测试
│           ├── hash_verification_tests.rs  # 哈希验证测试
│           └── [其他测试...]
│
├── cli/                            # 命令行工具
│   └── gvm/                        # Go 版本管理器 CLI
│       ├── Cargo.toml              # CLI 配置
│       ├── src/                    # CLI 源码
│       │   ├── main.rs             # 主入口
│       │   ├── lib.rs              # 库接口 (支持集成测试)
│       │   ├── cli.rs              # 命令行解析
│       │   ├── commands.rs         # 命令实现
│       │   ├── config.rs           # 配置管理
│       │   └── ui.rs               # 用户界面
│       ├── examples/               # CLI 使用示例
│       │   └── env_demo.rs         # 环境配置演示
│       └── tests/                  # CLI 集成测试
│           ├── environment_tests.rs            # 环境测试
│           └── environment_integration_tests.rs  # 环境集成测试
│
├── tests/                          # 系统级集成测试
│   ├── integration_test.rs         # 版本管理器集成测试
│   ├── environment_setup_test.rs   # 环境设置测试
│   └── README.md                   # 测试说明文档
│
├── examples/                       # 系统级示例
│   └── README.md                   # 示例说明文档
│
└── target/                         # 构建输出
    ├── debug/                      # 开发构建
    │   └── gvm                     # 调试版本
    └── release/                    # 发布构建
        └── gvm                     # 生产版本
```

## 许可证

本项目基于 MIT 许可证开源 - 查看 [LICENSE](LICENSE) 文件了解详情。
