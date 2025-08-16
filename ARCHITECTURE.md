# Tidepool GVM 架构文档

## 🎯 设计理念

Tidepool GVM 采用**简洁优先**的设计理念，避免过度工程化，专注于提供可靠、高效的 Go 版本管理功能。

### 核心原则

1. **简单胜过复杂** - 直接解决问题，避免不必要的抽象
2. **实用胜过完美** - 功能优先，保持代码可维护性
3. **一致性** - 统一的错误处理、命令接口和代码风格
4. **跨平台兼容** - 稳定的 ASCII 输出，避免平台特定功能

## 🏗️ 架构概览

### 模块结构

```
src/
├── main.rs              # 程序入口点
├── lib.rs               # 库入口和模块导出
├── cli.rs               # CLI 命令解析和分发
├── commands.rs          # 命令实现逻辑
├── config.rs            # 配置管理
├── go.rs                # Go 版本管理核心
├── downloader.rs        # 文件下载功能
├── symlink.rs           # 符号链接处理
├── platform.rs          # 平台检测和适配
├── error.rs             # 统一错误处理
├── ui_flat.rs           # 简化的 UI 系统
└── progress_flat.rs     # 简化的进度系统
```

### 数据流

```
用户命令 → CLI解析 → 命令分发 → 业务逻辑 → 平台操作 → 用户反馈
    ↓         ↓         ↓         ↓         ↓         ↓
  main.rs   cli.rs  commands.rs  go.rs   platform.rs ui_flat.rs
```

## 🔧 核心模块详解

### 1. CLI 层 (`cli.rs`)

**职责**: 命令行参数解析和命令分发

```rust
pub struct Cli {
    #[arg(short, long, global = true)]
    pub verbose: bool,
    #[arg(short, long, global = true)]
    pub quiet: bool,
    #[command(subcommand)]
    pub command: Commands,
}
```

**设计特点**:
- 使用 `clap` 进行声明式命令定义
- 支持全局选项（verbose, quiet）
- 统一的命令处理模式

### 2. 命令实现层 (`commands.rs`)

**职责**: 具体命令的业务逻辑实现

```rust
// 统一的函数签名模式
pub async fn install(version: &str, config: &Config, force: bool) -> Result<()>
pub fn switch(version: &str, config: &Config, global: bool, force: bool) -> Result<()>
pub fn list(config: &Config, show_all: bool) -> Result<()>
```

**设计特点**:
- 简单直接的函数签名
- 统一的错误处理 (`anyhow::Result`)
- 清晰的参数传递

### 3. 核心逻辑层 (`go.rs`)

**职责**: Go 版本管理的核心业务逻辑

```rust
pub struct GoManager {
    config: Config,
}

impl GoManager {
    // 异步操作：涉及网络下载
    pub async fn install(&self, version: &str, force: bool) -> Result<()>
    
    // 同步操作：本地文件系统操作
    pub fn switch_to(&self, request: SwitchRequest) -> Result<()>
    pub fn uninstall(&self, version: &str) -> Result<()>
    pub fn list_installed(&self) -> Result<Vec<GoVersionInfo>>
}
```

**设计特点**:
- 集中的版本管理逻辑
- 合理的异步/同步划分
- 简化的数据结构

### 4. 平台抽象层 (`platform.rs`)

**职责**: 跨平台兼容性处理

```rust
pub struct PlatformInfo {
    pub os: String,
    pub arch: String,
    pub exe_extension: String,
}

impl PlatformInfo {
    pub fn detect() -> Self
    pub fn archive_filename(&self, version: &str) -> String
    pub fn go_executable_name(&self) -> String
}
```

**设计特点**:
- 统一的平台检测逻辑
- 集中处理平台差异
- 简洁的 API 设计

### 5. UI 系统 (`ui_flat.rs`)

**职责**: 用户界面输出

```rust
pub struct SimpleUI {
    pub use_colors: bool,
}

impl SimpleUI {
    pub fn success(&self, message: &str)
    pub fn error(&self, message: &str)
    pub fn info(&self, message: &str)
    pub fn key_value(&self, key: &str, value: &str)
}
```

**设计特点**:
- 纯 ASCII 输出，跨平台兼容
- 基础颜色支持，自动检测终端能力
- 一致的消息格式

### 6. 进度系统 (`progress_flat.rs`)

**职责**: 进度显示和状态跟踪

```rust
pub struct BasicProgress {
    pub label: String,
    pub current: f64,
    pub total: f64,
}
```

**设计特点**:
- scoop 风格的简洁进度条
- 基础的进度跟踪
- 清晰的状态显示

## 📊 依赖管理

### 核心依赖

```toml
[dependencies]
anyhow = "1.0"          # 统一错误处理
clap = "4.0"            # CLI 参数解析
tokio = "1.0"           # 异步运行时
reqwest = "0.11"        # HTTP 客户端
serde = "1.0"           # 序列化支持
chrono = "0.4"          # 时间处理
sha2 = "0.10"           # 哈希验证
zip = "0.6"             # 压缩文件处理
```

### 依赖选择原则

1. **最小化原则** - 只引入必要的依赖
2. **稳定性优先** - 选择成熟稳定的 crate
3. **功能专一** - 避免功能重叠的依赖
4. **维护活跃** - 选择维护活跃的项目

## 🔄 错误处理策略

### 统一错误类型

```rust
// 使用 anyhow::Result 作为统一错误类型
pub type Result<T> = anyhow::Result<T>;

// 错误上下文添加
.with_context(|| format!("Failed to install Go {}", version))?
```

### 错误处理层次

1. **系统级错误** - I/O 错误、网络错误等
2. **业务级错误** - 版本不存在、安装失败等
3. **用户级错误** - 参数错误、配置错误等

## 🚀 性能优化

### 异步策略

```rust
// 仅在必要的网络操作中使用 async
pub async fn install() -> Result<()>  // 网络下载

// 本地操作使用同步函数
pub fn switch_to() -> Result<()>      // 文件系统操作
```

### 内存优化

- 使用借用而非克隆：`&str` vs `String`
- 避免不必要的数据复制
- 合理使用 `Arc` 和 `Rc` 进行共享

### 编译优化

```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
```

## 🧪 测试策略

### 单元测试

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_platform_detection() {
        let platform = PlatformInfo::detect();
        assert!(!platform.os.is_empty());
        assert!(!platform.arch.is_empty());
    }
}
```

### 集成测试

- 命令行接口测试
- 端到端功能测试
- 跨平台兼容性测试

## 📈 质量保证

### 代码质量工具

```bash
cargo check         # 编译检查
cargo test          # 运行测试
cargo clippy        # 代码规范检查
cargo fmt           # 代码格式化
```

### 持续集成

- 多平台构建测试
- 依赖安全扫描
- 性能回归测试

## 🔮 扩展性设计

### 模块化设计

每个模块都有清晰的职责边界，便于：
- 独立测试
- 功能扩展
- 代码维护

### 配置驱动

```rust
pub struct Config {
    pub versions_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub default_version: Option<String>,
}
```

### 插件接口（未来）

```rust
pub trait Installer {
    fn install(&self, version: &str) -> Result<()>;
    fn uninstall(&self, version: &str) -> Result<()>;
}
```

## 💡 设计决策

### 为什么选择简单架构？

1. **维护成本低** - 减少抽象层次，降低认知负担
2. **调试容易** - 直接的调用链，问题定位快速
3. **性能更好** - 减少间接调用，提升运行效率
4. **扩展灵活** - 简单结构更容易修改和扩展

### 为什么移除复杂功能？

1. **用户体验** - 简单直接的命令更符合用户期望
2. **跨平台性** - 避免复杂 UI 库的平台兼容问题
3. **依赖管理** - 减少外部依赖，降低供应链风险
4. **构建速度** - 更少的依赖意味着更快的编译

## 🎯 未来规划

### 短期目标

- [ ] 配置文件支持
- [ ] Shell 自动补全
- [ ] 更多平台支持

### 长期目标

- [ ] 插件系统
- [ ] GUI 界面（可选）
- [ ] 云配置同步

### 架构演进原则

1. **保持简洁** - 新功能不应增加架构复杂度
2. **向后兼容** - 保持 API 稳定性
3. **渐进增强** - 功能逐步添加，避免大幅重构

---

这个架构设计体现了"简单即美"的哲学，通过合理的模块划分和清晰的职责边界，实现了高效、可维护的 Go 版本管理工具。
