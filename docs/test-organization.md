# Tidepool 测试组织规范

本文档详细说明 Tidepool 项目的测试组织结构，严格遵循 [Rust Book Ch11.3 测试组织](https://doc.rust-lang.org/book/ch11-03-test-organization.html) 最佳实践。

## 📋 核心原则

### 测试组织哲学
Tidepool 项目采用 **分层测试架构**，确保：
- **单元测试** - 测试内部实现和私有接口
- **集成测试** - 测试模块间协作和公共 API
- **示例程序** - 演示功能使用和验证可用性

### 测试隔离性
- 每个测试独立运行，不依赖其他测试
- 使用临时目录确保测试环境隔离
- 网络依赖测试使用 `#[ignore]` 标记
- 平台特定测试使用 `#[cfg(target_os)]` 标记

## 🏗️ 测试目录结构

### 完整测试架构

```
tidepool/
├── 📦 核心库单元测试
│   ├── crates/tidepool-version-manager/src/
│   │   ├── lib.rs                    # 包含 #[cfg(test)] 模块
│   │   ├── go.rs                     # 包含 #[cfg(test)] 模块  
│   │   └── downloader.rs             # 包含 #[cfg(test)] 模块
│   └── cli/gvm/src/
│       ├── lib.rs                    # 包含 #[cfg(test)] 模块
│       ├── cli.rs                    # 包含 #[cfg(test)] 模块
│       ├── commands.rs               # 包含 #[cfg(test)] 模块
│       ├── config.rs                 # 包含 #[cfg(test)] 模块
│       └── ui.rs                     # 包含 #[cfg(test)] 模块
│
├── 🧪 集成测试
│   ├── crates/tidepool-version-manager/tests/    # 版本管理器集成测试
│   │   ├── go_manager_tests.rs                   # Go 管理器核心功能
│   │   ├── hash_verification_tests.rs            # 哈希验证功能
│   │   ├── info_command_tests.rs                 # 版本信息查询
│   │   ├── temp_file_download_tests.rs           # 临时文件下载
│   │   ├── force_install_tests.rs                # 强制安装功能
│   │   ├── junction_tests.rs                     # Windows Junction 功能
│   │   ├── structured_interface_tests.rs         # 结构化接口测试
│   │   └── uninstall_current_version_tests.rs    # 卸载当前版本
│   ├── cli/gvm/tests/                             # CLI 集成测试
│   │   ├── environment_tests.rs                  # 环境变量配置测试
│   │   └── environment_integration_tests.rs      # 环境配置集成测试
│   └── tests/                                     # 系统级集成测试
│       ├── integration_test.rs                   # 版本管理器集成测试
│       ├── environment_setup_test.rs             # 环境设置测试
│       └── README.md                             # 测试说明文档
│
├── 🎯 示例程序
│   ├── crates/tidepool-version-manager/examples/ # 版本管理器示例
│   │   ├── downloader_test.rs                    # 下载器功能演示
│   │   ├── hash_verification_demo.rs             # 哈希验证演示
│   │   ├── junction_demo.rs                      # Junction 功能演示
│   │   ├── shields_evaluation.rs                 # Shields 徽章评估
│   │   ├── temp_file_demo.rs                     # 临时文件机制演示
│   │   └── uninstall_protection_demo.rs          # 卸载保护演示
│   └── cli/gvm/examples/                         # CLI 示例
│       └── env_demo.rs                           # 环境变量配置演示
└── 
```

## 🔬 单元测试规范

### 位置和组织
- **位置**: 源文件内部的 `#[cfg(test)]` 模块
- **范围**: 测试单个函数、方法或模块的功能
- **访问**: 可以测试私有函数和内部实现

### 单元测试示例

```rust
// src/lib.rs 或 src/module.rs 内部
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_internal_function() {
        // 准备测试数据
        let input = "test_input";
        
        // 执行操作
        let result = internal_function(input);
        
        // 断言结果
        assert_eq!(result, "expected_output");
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_windows_specific_feature() {
        // Windows 特定功能测试
        let result = windows_only_function();
        assert!(result.is_ok());
    }

    #[test]
    #[should_panic(expected = "Invalid input")]
    fn test_error_handling() {
        // 错误处理测试
        invalid_operation();
    }
}
```

### 单元测试最佳实践

#### 测试命名
- 函数名: `test_功能描述`
- 模块名: `tests` 或具体功能模块名
- 清晰描述测试目的

#### 测试结构
```rust
#[test]
fn test_feature_name() {
    // 1. 准备 (Arrange)
    let input = setup_test_data();
    
    // 2. 执行 (Act)
    let result = function_under_test(input);
    
    // 3. 断言 (Assert)
    assert_eq!(result, expected_output);
}
```

#### 平台特定测试
```rust
#[test]
#[cfg(target_os = "windows")]
fn test_windows_junction() {
    // Windows Junction 测试
}

#[test]
#[cfg(unix)]
fn test_unix_symlink() {
    // Unix 符号链接测试
}
```

## 🧩 集成测试规范

### 位置和组织
- **位置**: 各包的 `tests/` 目录下
- **文件**: 每个 `.rs` 文件都是独立的 crate
- **访问**: 只能测试公开的 API

### 集成测试示例

```rust
// tests/go_manager_tests.rs
use tidepool_version_manager::GoManager;
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn test_go_manager_integration() {
    // 设置测试环境
    let temp_dir = TempDir::new().expect("创建临时目录失败");
    let manager = GoManager::new();
    
    // 执行集成测试
    let result = manager.some_operation(&temp_dir.path());
    
    // 验证结果
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_async_download() {
    let manager = GoManager::new();
    let temp_dir = TempDir::new().expect("创建临时目录失败");
    
    // 异步操作测试
    let result = manager.download_version("1.21.3", &temp_dir.path()).await;
    assert!(result.is_ok());
}

#[test]
#[ignore = "需要网络连接，在CI或无网络环境中跳过"]
fn test_network_dependent_feature() {
    // 网络依赖测试，默认跳过
    let manager = GoManager::new();
    // ... 网络操作测试
}
```

### 集成测试文件组织

#### 版本管理器集成测试
```
crates/tidepool-version-manager/tests/
├── go_manager_tests.rs           # Go 管理器核心功能
├── hash_verification_tests.rs    # 哈希验证功能  
├── info_command_tests.rs         # 版本信息查询
├── temp_file_download_tests.rs   # 临时文件下载
├── force_install_tests.rs        # 强制安装功能
├── junction_tests.rs             # Windows Junction 功能
├── structured_interface_tests.rs # 结构化接口测试
└── uninstall_current_version_tests.rs  # 卸载当前版本
```

#### CLI 集成测试
```
cli/gvm/tests/
├── environment_tests.rs          # 环境变量配置测试
└── environment_integration_tests.rs  # 环境配置集成测试
```

#### 系统级集成测试
```
tests/                            # 根目录系统级测试
├── integration_test.rs           # 版本管理器集成测试
├── environment_setup_test.rs     # 环境设置测试
└── README.md                     # 测试说明文档
```

### 集成测试最佳实践

#### 环境隔离
```rust
use tempfile::TempDir;

#[test]
fn test_isolated_operation() {
    // 每个测试使用独立的临时目录
    let temp_dir = TempDir::new().expect("创建临时目录失败");
    let test_path = temp_dir.path();
    
    // 测试操作，自动清理
    // temp_dir 在作用域结束时自动删除
}
```

#### 异步测试
```rust
#[tokio::test]
async fn test_async_function() {
    let result = async_operation().await;
    assert!(result.is_ok());
}
```

#### 网络测试标记
```rust
#[test]
#[ignore = "需要网络连接"]
fn test_download_feature() {
    // 网络依赖的测试
}
```

## 🎯 示例程序规范

### 位置和组织
- **位置**: 各包的 `examples/` 目录下
- **用途**: 演示功能使用方法，作为文档补充

### 示例程序结构

```rust
// examples/feature_demo.rs
use tidepool_version_manager::GoManager;

fn main() {
    println!("🎯 功能演示开始");
    
    // 演示功能使用
    let manager = GoManager::new();
    
    // 详细的步骤说明
    println!("📋 步骤1: 初始化管理器");
    // ... 演示代码
    
    println!("📋 步骤2: 执行操作");
    // ... 演示代码
    
    println!("✅ 演示完成");
}
```

### 示例程序分类

#### 版本管理器示例
```
crates/tidepool-version-manager/examples/
├── downloader_test.rs            # 下载器功能演示
├── hash_verification_demo.rs     # 哈希验证演示
├── junction_demo.rs              # Junction 功能演示
├── shields_evaluation.rs         # Shields 徽章评估
├── temp_file_demo.rs             # 临时文件机制演示
└── uninstall_protection_demo.rs  # 卸载保护演示
```

#### CLI 示例
```
cli/gvm/examples/
└── env_demo.rs                   # 环境变量配置演示
```

### 示例程序最佳实践

#### 清晰的演示流程
```rust
fn main() {
    println!("🎯 {} 功能演示", "特定功能名称");
    
    // 显示演示目的
    println!("📝 本演示将展示:");
    println!("   1. 功能A的使用方法");
    println!("   2. 功能B的配置选项"); 
    println!("   3. 错误处理机制");
    
    // 分步骤演示
    demo_step_1();
    demo_step_2();
    demo_step_3();
    
    println!("✅ 演示完成");
}
```

#### 错误处理演示
```rust
fn demo_error_handling() {
    println!("📋 步骤3: 错误处理演示");
    
    match risky_operation() {
        Ok(result) => println!("✅ 操作成功: {:?}", result),
        Err(e) => println!("❌ 操作失败: {}", e),
    }
}
```

## 🚀 测试运行和管理

### 基本测试命令

```bash
# 运行所有测试（跳过网络测试）
cargo test --workspace

# 运行包含网络测试的完整测试
cargo test --workspace -- --ignored

# 运行特定包的测试
cargo test -p tidepool-version-manager
cargo test -p gvm

# 运行特定测试文件
cargo test --test go_manager_tests
cargo test --test environment_tests
```

### 示例程序运行

```bash
# 运行版本管理器示例
cargo run --example temp_file_demo --package tidepool-version-manager
cargo run --example hash_verification_demo --package tidepool-version-manager

# 运行 CLI 示例
cargo run --example env_demo --package gvm
```

### 高级测试选项

```bash
# 详细测试输出
cargo test --workspace -- --nocapture

# 并行测试控制
cargo test --workspace -- --test-threads=1

# 运行特定测试函数
cargo test test_go_manager_install

# 运行匹配模式的测试
cargo test hash_verification
```

### 测试覆盖率

```bash
# 安装 tarpaulin（代码覆盖率工具）
cargo install cargo-tarpaulin

# 生成覆盖率报告
cargo tarpaulin --workspace --out html

# 查看覆盖率统计
cargo tarpaulin --workspace --out stdout
```

## 📊 测试质量要求

### 强制性要求

#### 零容忍标准
- ✅ **零编译错误**
- ✅ **零编译警告**  
- ✅ **零 Clippy 警告**
- ✅ **所有测试通过**
- ✅ **代码已格式化**

#### 覆盖率目标
- 单元测试覆盖率 > 80%
- 集成测试覆盖所有公共 API
- 示例程序验证所有主要功能

#### 测试质量指标
- 测试之间保持隔离性
- 复杂测试有充分的注释说明
- 跨平台代码在不同系统下都能运行
- 网络依赖测试正确标记和跳过

### 质量检查流程

```bash
# 完整质量检查流程（强制执行）
cargo fmt; cargo check --workspace; cargo clippy --workspace -- -D warnings; cargo test --workspace
```

### 测试命名规范

#### 文件命名
- **单元测试**: 在源文件内部，模块名为 `tests` 或 `test`
- **集成测试**: `功能名_tests.rs`（如 `go_manager_tests.rs`）
- **示例程序**: `功能名_demo.rs`（如 `temp_file_demo.rs`）

#### 函数命名
- **测试函数**: `test_功能描述`（如 `test_install_go_version`）
- **测试模块**: `tests` 或具体功能模块名
- **辅助函数**: `setup_*` 或 `create_*`

#### 文档注释
```rust
/// 测试 Go 版本安装功能
/// 
/// 验证能够正确下载和安装指定的 Go 版本，
/// 包括文件完整性验证和目录结构创建。
#[test]
fn test_install_go_version() {
    // 测试实现
}
```

## ❌ 禁止的测试组织方式

### 错误的测试位置

#### ❌ 在 src/ 目录创建独立测试文件
```
src/
├── lib.rs
├── go_manager.rs
└── go_manager_test.rs  // ❌ 错误位置
```

**正确做法**: 在 `go_manager.rs` 内部添加 `#[cfg(test)]` 模块

#### ❌ 错误的导入方式
```rust
// 错误：在集成测试中使用 crate:: 导入
use crate::internal_module;  // ❌ 集成测试无法访问内部模块

// 正确：使用包名导入公开API
use tidepool_version_manager::GoManager;  // ✅ 正确方式
```

#### ❌ 混合测试类型
```rust
// 错误：在集成测试中测试私有函数
#[test]
fn test_private_function() {
    // ❌ 集成测试不应该测试私有实现
    internal_function();
}
```

#### ❌ 测试间相互依赖
```rust
// 错误：测试之间相互依赖
static mut SHARED_STATE: i32 = 0;

#[test]
fn test_a() {
    unsafe { SHARED_STATE = 1; }  // ❌ 修改共享状态
}

#[test] 
fn test_b() {
    unsafe { assert_eq!(SHARED_STATE, 1); }  // ❌ 依赖其他测试状态
}
```

## 🔮 测试组织进化

### 当前测试统计

```
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

### 测试扩展计划

#### 增强覆盖率
- 增加边界条件测试
- 增加错误路径测试
- 增加性能测试标记

#### 跨平台测试
- macOS 特定功能测试
- Linux 发行版兼容性测试
- ARM 架构特定测试

#### 自动化改进
- CI/CD 集成优化
- 测试报告自动生成
- 性能回归检测

## 📚 相关文档

- [项目主文档](../README.md) - 项目概览和快速开始
- [CLI 工具文档](cli-tools.md) - 命令行工具使用说明
- [版本管理器核心库文档](tidepool-version-manager.md) - 核心库使用说明
- [环境配置功能文档](environment-setup-feature.md) - 环境变量配置说明
- [项目重组说明](project-reorganization.md) - 项目结构演变历史
- [Rust Book Ch11.3](https://doc.rust-lang.org/book/ch11-03-test-organization.html) - Rust 官方测试组织指南
