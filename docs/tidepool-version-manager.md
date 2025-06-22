# Tidepool Version Manager 核心库

Go 版本管理核心库，为 Tidepool 工具包提供统一的版本管理接口。

## 🎯 概览

Tidepool Version Manager 提供统一的接口来管理不同编程语言的运行时版本。设计为可扩展的架构，当前支持：

- ✅ **Go 版本管理** - 完整实现，支持 Windows、macOS、Linux
- 🔄 **Python 版本管理** - 计划中
- 🔄 **Node.js 版本管理** - 计划中

## 🚀 核心特性

### 自动下载和安装
- 从官方网站下载运行时二进制文件
- 支持多个下载源（官方源、中国镜像等）
- 异步并发下载，最大化网络利用率
- SHA256 哈希校验，确保下载文件完整性
- 智能进度显示，实时下载状态

### 版本切换和管理
- 快速在已安装的运行时版本间切换
- 跨平台版本切换（Windows Junction、Unix 符号链接）
- 智能版本清理和安全卸载保护
- 防止误删当前使用版本的保护机制

### 环境管理
- 自动设置环境变量（GOROOT、GOPATH、PATH）
- 支持临时配置和永久配置
- 多 Shell 支持（Bash、Zsh、Fish、PowerShell、NuShell 等）
- 跨平台环境配置

### 跨平台支持
- 设计支持 Windows、macOS 和 Linux
- 多架构支持（amd64、arm64）
- 平台特定的归档格式处理（ZIP、tar.gz）
- 符合链接和符号链接统一接口

## 📦 使用方法

### 基本使用示例

```rust
use tidepool_version_manager::{VersionManager, go::GoManager};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化 Go 版本管理器
    let go_manager = GoManager::new();
    let install_dir = PathBuf::from("/path/to/go_versions");
    
    // 安装指定 Go 版本（异步下载，带进度显示）
    let version_info = VersionManager::install_version(
        &go_manager, 
        "1.21.3", 
        &install_dir, 
        false  // force_install
    ).await?;
    
    println!("安装完成: {} 位于 {}", 
        version_info.version, 
        version_info.install_path.display()
    );
    
    // 切换到已安装版本
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

### Go Manager 详细功能

#### 版本安装

```rust
use tidepool_version_manager::go::GoManager;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = GoManager::new();
    let install_dir = PathBuf::from("C:/dev/go_versions");
    
    // 标准安装
    let result = manager.install_version("1.21.3", &install_dir, false).await?;
    
    // 强制安装（覆盖现有版本）
    let result = manager.install_version("1.21.3", &install_dir, true).await?;
    
    println!("安装路径: {}", result.install_path.display());
    println!("下载文件: {}", result.filename);
    
    Ok(())
}
```

#### 版本切换

```rust
use tidepool_version_manager::go::GoManager;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = GoManager::new();
    let install_dir = PathBuf::from("C:/dev/go_versions");
    
    // 临时切换（当前会话）
    manager.use_version("1.21.3", &install_dir, false)?;
    
    // 全局切换（持久化配置）
    manager.use_version("1.21.3", &install_dir, true)?;
    
    println!("已切换到 Go 1.21.3");
    
    Ok(())
}
```

#### 版本查询

```rust
use tidepool_version_manager::go::GoManager;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = GoManager::new();
    let install_dir = PathBuf::from("C:/dev/go_versions");
    
    // 列出已安装版本
    let installed = manager.list_installed_versions(&install_dir)?;
    println!("已安装版本: {:?}", installed);
    
    // 获取可用版本列表
    let available = manager.list_available_versions().await?;
    println!("可用版本: {:?}", available);
    
    // 获取特定版本信息
    let info = manager.get_version_info("1.21.3").await?;
    println!("版本详情: {:?}", info);
    
    Ok(())
}
```

#### 版本卸载

```rust
use tidepool_version_manager::go::GoManager;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = GoManager::new();
    let install_dir = PathBuf::from("C:/dev/go_versions");
    
    // 安全卸载（保护当前使用版本）
    match manager.uninstall_version("1.20.5", &install_dir) {
        Ok(_) => println!("成功卸载 Go 1.20.5"),
        Err(e) => println!("卸载失败: {}", e),
    }
    
    Ok(())
}
```

## 🏗️ 架构设计

### 核心接口

```rust
/// 版本管理器通用接口
pub trait VersionManager {
    type Error;
    type VersionInfo;
    
    // 异步操作
    async fn install_version(
        &self, 
        version: &str, 
        install_dir: &Path, 
        force: bool
    ) -> Result<Self::VersionInfo, Self::Error>;
    
    async fn list_available_versions(&self) -> Result<Vec<String>, Self::Error>;
    async fn get_version_info(&self, version: &str) -> Result<Self::VersionInfo, Self::Error>;
    
    // 同步操作
    fn use_version(
        &self, 
        version: &str, 
        install_dir: &Path, 
        global: bool
    ) -> Result<(), Self::Error>;
    
    fn list_installed_versions(&self, install_dir: &Path) -> Result<Vec<String>, Self::Error>;
    fn uninstall_version(&self, version: &str, install_dir: &Path) -> Result<(), Self::Error>;
}
```

### 模块组织

```
src/
├── lib.rs              # 库入口，导出公共接口
├── go.rs               # Go 版本管理器实现
└── downloader.rs       # 下载器模块
```

#### 下载器模块 (`downloader.rs`)
- 异步并发下载支持
- 支持进度显示和断点续传
- SHA256 哈希完整性验证
- 多源下载支持
- 临时文件安全处理

#### Go 管理器 (`go.rs`)
- Go 版本的安装、卸载、切换和查询
- 从 https://go.dev/dl/ 获取版本信息
- 跨平台归档解压（ZIP、tar.gz）
- 符合链接和符号链接创建
- 环境变量配置和管理

## 🧪 测试和示例

### 测试结构

项目采用 Rust 官方推荐的测试组织结构：

#### 单元测试
位于源文件内部的 `#[cfg(test)]` 模块，测试私有函数和内部逻辑。

#### 集成测试 (`tests/`)
- `go_manager_tests.rs` - Go 管理器核心功能测试
- `hash_verification_tests.rs` - 哈希验证功能测试
- `info_command_tests.rs` - 版本信息查询测试
- `temp_file_download_tests.rs` - 临时文件下载测试
- `force_install_tests.rs` - 强制安装功能测试
- `junction_tests.rs` - Windows Junction 功能测试
- `structured_interface_tests.rs` - 结构化接口测试
- `uninstall_current_version_tests.rs` - 卸载当前版本测试

#### 示例程序 (`examples/`)
- `downloader_test.rs` - 下载器功能演示
- `hash_verification_demo.rs` - 哈希验证演示
- `junction_demo.rs` - Junction 功能演示
- `shields_evaluation.rs` - Shields 徽章评估
- `temp_file_demo.rs` - 临时文件机制演示
- `uninstall_protection_demo.rs` - 卸载保护演示

### 运行测试

```bash
# 运行所有测试（跳过网络测试）
cargo test -p tidepool-version-manager

# 运行包含网络测试的完整测试
cargo test -p tidepool-version-manager -- --ignored

# 运行特定测试文件
cargo test --test go_manager_tests

# 运行示例程序
cargo run --example temp_file_demo --package tidepool-version-manager
cargo run --example hash_verification_demo --package tidepool-version-manager
```

## 🔧 技术细节

### 下载和安装流程

1. **版本信息获取**: 从 https://go.dev/dl/ 获取可用版本列表
2. **文件下载**: 异步下载对应平台的归档文件
3. **完整性验证**: SHA256 哈希校验确保文件完整性
4. **归档解压**: 平台特定的解压处理（ZIP/tar.gz）
5. **版本安装**: 将解压内容安装到指定目录
6. **环境配置**: 自动配置环境变量和路径

### 版本切换机制

#### Windows 平台
- 使用 **Junction** (符合链接) 实现版本切换
- 创建指向目标版本目录的符合链接
- 支持快速切换，无需移动文件

#### Unix 平台 (Linux/macOS)
- 使用 **符号链接** 实现版本切换  
- 创建指向目标版本目录的符号链接
- 支持快速切换，无需移动文件

### 环境变量管理

#### 支持的环境变量
- `GOROOT` - Go 安装根目录
- `GOPATH` - Go 工作目录
- `PATH` - 可执行文件路径

#### 配置模式
- **临时配置** - 仅在当前会话生效
- **永久配置** - 持久化到配置文件

#### 多 Shell 支持
- Bash - `~/.bashrc`, `~/.bash_profile`
- Zsh - `~/.zshrc`
- Fish - `~/.config/fish/config.fish`
- PowerShell - `$PROFILE`
- NuShell - `~/.config/nushell/config.nu`

### 安全特性

#### 下载安全
- SHA256 哈希完整性检查
- 临时文件安全处理和清理
- 网络超时和重试机制

#### 卸载保护
- 防止误删当前使用版本
- 安全的版本卸载流程
- 智能依赖检查

#### 权限控制
- 安全的文件和目录操作
- 平台特定的权限处理
- 最小权限原则

### 性能优化

#### 网络优化
- 异步并发下载
- 分块下载支持
- 断点续传能力
- 智能源选择

#### 内存效率
- 流式处理大文件
- 避免内存溢出
- 高效的缓存机制

#### 存储优化
- 智能缓存管理
- 重复数据避免
- 磁盘空间优化

## 🔮 未来规划

### 即将支持的语言
- **Python** - 支持 CPython、PyPy 等多种实现
- **Node.js** - 支持官方 Node.js 和 LTS 版本
- **Rust** - 支持 stable、beta、nightly 工具链
- **Java** - 支持 OpenJDK、Oracle JDK 等

### 计划功能
- 配置文件支持（TOML/YAML）
- 自定义下载源配置
- 版本别名和标签支持
- 自动版本清理策略
- 版本依赖关系管理
- 批量操作支持

### 架构改进
- 插件化架构设计
- 更好的错误处理和恢复
- 增强的进度显示和用户体验
- 更全面的测试覆盖

## 📚 相关文档

- [项目主文档](../README.md) - 项目概览和快速开始
- [CLI 工具文档](cli-tools.md) - 命令行工具使用说明
- [测试组织文档](test-organization.md) - 测试开发规范
- [环境配置功能文档](environment-setup-feature.md) - 环境变量配置说明
- [项目重组说明](project-reorganization.md) - 项目结构演变历史
