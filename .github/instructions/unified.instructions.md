---
applyTo: "**"
description: Tidepool 项目 Copilot 统一指令规范 - 包含开发规范、测试组织和代码质量要求
---

# Tidepool 项目 Copilot 统一指令规范

## 🎯 核心行为要求

### 语言和交互
- **回复语言**: 始终使用**简体中文**进行回复、解释和交流
- **语调**: 保持专业、有帮助和鼓励的语调
- **代码注释**: 复杂逻辑必须用中文注释解释

### Shell 环境和命令操作
- **默认 Shell**: 开发环境使用 **NuShell**
- **命令语法**: 所有 shell 命令必须符合 **NuShell 语法**
- **多命令操作**: 使用 `;` 分隔多条命令
- **常用命令示例**:
  ```bash
  # 单条命令
  ls                    # 列出文件
  cd <path>            # 切换目录
  rm <file>            # 删除文件
  
  # 多条命令操作（使用 ; 分隔）
  cargo fmt; cargo check; cargo test           # 格式化、检查、测试
  cd target; ls; cd ..                        # 切换目录、列出、返回
  git add .; git status; git commit -m "..."   # Git 操作链
  ```

## 🛠️ 代码质量强制流程

### 每次代码修改必须执行（不可跳过）：
```bash
# 完整质量检查流程（可用分号链式执行）
cargo fmt; cargo check --workspace; cargo clippy --workspace -- -D warnings; cargo test --workspace
```

### 质量标准（零容忍）
- ✅ **零编译错误**
- ✅ **零编译警告**  
- ✅ **零 Clippy 警告**
- ✅ **所有测试通过**
- ✅ **代码已格式化**

### Rust 编码规范
- **依赖管理**: 优先使用 `std` 库，最小化外部依赖
- **错误处理**: 使用 `Result<T, E>`，提供有用的错误信息
- **测试**: 为所有新功能编写测试，使用 `#[cfg(target_os)]` 标记平台特定测试
- **文档**: 公共 API 必须有 rustdoc 注释
- **库级别**: 禁止直接输出（`println!`），只使用日志系统（`log` crate）

## 🧪 测试组织规范

### 核心原则
本项目严格遵循 [Rust Book Ch11.3 测试组织](https://doc.rust-lang.org/book/ch11-03-test-organization.html) 最佳实践。

### 测试目录结构

#### 单元测试（Unit Tests）
- **位置**: 在对应的源文件内部，使用 `#[cfg(test)]` 模块
- **范围**: 测试单个函数、方法或模块的功能
- **访问**: 可以测试私有函数和内部实现

```rust
// src/lib.rs 或 src/module.rs 内部
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_internal_function() {
        // 测试私有函数
    }
}
```

#### 集成测试（Integration Tests）
- **位置**: 各包的 `tests/` 目录下
- **文件**: 每个 `.rs` 文件都是独立的 crate
- **访问**: 只能测试公开的 API

**CLI 集成测试**:
```
cli/gvm/tests/
├── environment_tests.rs          # 环境配置相关测试
├── environment_integration_tests.rs  # 环境集成测试
└── [其他集成测试文件]
```

**版本管理器集成测试**:
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

**系统级集成测试**:
```
tests/                            # 根目录系统级测试
├── integration_test.rs           # 版本管理器集成测试
├── environment_setup_test.rs     # 环境设置测试
└── README.md                     # 测试说明文档
```

#### 示例程序（Examples）
- **位置**: 各包的 `examples/` 目录下
- **用途**: 演示功能使用方法，作为文档补充

**CLI 示例**:
```
cli/gvm/examples/
└── env_demo.rs                   # 环境变量配置演示
```

**版本管理器示例**:
```
crates/tidepool-version-manager/examples/
├── downloader_test.rs            # 下载器功能演示
├── hash_verification_demo.rs     # 哈希验证演示
├── junction_demo.rs              # Junction 功能演示
├── shields_evaluation.rs         # Shields 徽章评估
├── temp_file_demo.rs             # 临时文件机制演示
└── uninstall_protection_demo.rs  # 卸载保护演示
```

### 测试编写规范

#### 单元测试规范
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_name() {
        // 准备测试数据
        let input = "test_input";
        
        // 执行操作
        let result = function_under_test(input);
        
        // 断言结果
        assert_eq!(result, expected_output);
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_windows_specific_feature() {
        // Windows 特定功能测试
    }

    #[test]
    #[should_panic(expected = "error message")]
    fn test_error_handling() {
        // 错误处理测试
    }
}
```

#### 集成测试规范
```rust
// tests/new_feature_tests.rs
use tidepool_version_manager::GoManager;
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn test_new_feature_integration() {
    // 设置测试环境
    let temp_dir = TempDir::new().expect("创建临时目录失败");
    let manager = GoManager::new();
    
    // 执行集成测试
    let result = manager.some_operation(&temp_dir.path());
    
    // 验证结果
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_async_operation() {
    // 异步操作测试
    let result = async_function().await;
    assert!(result.is_ok());
}

#[test]
#[ignore = "需要网络连接，在CI或无网络环境中跳过"]
fn test_network_dependent_feature() {
    // 网络依赖测试，默认跳过
}
```

#### 示例程序规范
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
    
    println!("✅ 演示完成");
}
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

### 测试质量要求

#### 最佳实践
1. **测试隔离** - 每个测试使用独立的临时目录，使用 `TempDir` 确保自动清理
2. **错误处理测试** - 测试正常路径和错误路径，使用 `#[should_panic]` 测试预期错误
3. **平台特定测试** - 使用 `#[cfg(target_os = "...")]` 标记平台特定测试
4. **网络和I/O测试** - 网络依赖测试使用 `#[ignore]` 标记
5. **性能测试** - 大文件操作测试标记性能要求，避免在CI中运行耗时测试

#### 测试运行命令
```bash
# 运行所有测试（跳过网络测试）
cargo test --workspace

# 运行包含网络测试的完整测试
cargo test --workspace -- --ignored

# 运行特定包的测试
cargo test -p tidepool-version-manager
cargo test -p gvm

# 运行示例程序
cargo run --example temp_file_demo --package tidepool-version-manager
cargo run --example env_demo --package gvm
```

#### 质量指标要求
- ✅ 所有测试通过（除忽略的网络测试）
- ✅ 代码覆盖率 > 80%
- ✅ 测试之间保持隔离性
- ✅ 复杂测试有充分的注释说明
- ✅ 跨平台代码在不同系统下都能运行

### 禁止的测试组织方式

#### ❌ 错误的测试位置
```rust
// 错误：在 src/ 目录创建独立的测试文件
src/
├── lib.rs
├── go_manager.rs
└── go_manager_test.rs  // ❌ 错误位置
```

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

## 📝 提交规范

### 提交信息格式（强制）
```
<type>[scope]: <description>

[可选的详细说明，用中文]
```

### 提交类型
- `feat`: 新功能 | `fix`: 错误修复 | `refactor`: 代码重构
- `test`: 测试相关 | `docs`: 文档更新 | `style`: 代码格式化
- `build`: 构建/依赖变更 | `chore`: 维护任务

### 作用域
- `version-manager`: tidepool-version-manager crate
- `cli`: gvm CLI 工具 | `deps`: 依赖管理 | `config`: 配置相关

### 语言规则
- **标题**: 必须使用**英文**
- **说明**: 可以使用**中文**进行详细解释

## 🔧 严格执行规则

### 文件操作
- **编辑前必读**: 使用编辑工具前必须先用 `read_file` 读取内容
- **分批编辑**: 大型更改分解为多个小编辑操作
- **验证编辑**: 每次编辑后检查编译错误
- **绝对路径**: 所有文件路径使用绝对路径

### 错误处理策略
- **立即修复**: 发现编译错误或警告时立即修复
- **三次限制**: 同一文件修复尝试不超过3次
- **失败停止**: 3次尝试失败后询问用户下一步操作

### 工具使用
- **并行调用**: 可并行使用多个工具（除了 `semantic_search`）
- **链式命令**: 使用 `;` 连接多个相关命令提高效率
- **避免重复**: 不要连续多次调用 `run_in_terminal`

## ❌ 禁止行为

### 代码输出
- ❌ **禁止打印代码块**: 不要打印文件更改，使用编辑工具
- ❌ **禁止打印命令**: 不要打印终端命令，使用 `run_in_terminal`
- ❌ **禁止假设**: 不要假设情况，先收集上下文

### 测试组织约束
- ❌ **禁止在 src/ 目录创建独立测试文件**: 单元测试必须在源文件内部的 `#[cfg(test)]` 模块
- ❌ **禁止集成测试使用私有API**: 集成测试只能使用包名导入公开API（如 `use tidepool_version_manager::GoManager`）
- ❌ **禁止混合测试类型**: 集成测试不应该测试私有函数或内部实现
- ❌ **禁止测试间相互依赖**: 所有测试必须保持隔离性，不能共享状态
- ❌ **禁止跳过必要测试**: 所有新功能必须有对应的单元测试或集成测试

### 文档管理约束
- ✅ **统一文档位置**: 所有项目文档必须统一保存在 `docs/` 目录下
- ✅ **文档分类管理**: 按功能模块分类组织文档，避免重复
- ❌ **禁止分散 README**: 不允许在各子目录创建独立的 README.md 文件
- ❌ **禁止重复文档**: 各模块的说明统一整理到相应的 docs 文件中
- ✅ **文档引用管理**: 主 README.md 可以引用 docs 目录下的具体文档

#### 文档目录结构
```
docs/
├── cli-tools.md                    # CLI 工具使用文档
├── tidepool-version-manager.md     # 版本管理器核心库文档
├── test-organization.md            # 测试组织规范文档
├── environment-setup-feature.md    # 环境配置功能文档
├── project-reorganization.md       # 项目重组历史文档
└── [其他功能文档]
```

#### 文档维护规则
- **创建新文档**: 新功能文档直接在 docs 目录创建
- **整理现有文档**: 分散的 README 内容合并到对应 docs 文件
- **删除重复文档**: 清理各子目录中的冗余 README 文件
- **更新引用链接**: 确保文档间的引用链接正确

### 文档创建限制
- ❌ **禁止自动创建总结文档**: 不要创建功能总结、改进报告等文档文件
- ❌ **禁止创建 IMPROVEMENTS.md**: 不要创建改进说明文档
- ❌ **禁止创建功能说明文档**: 除非用户明确要求，否则不创建额外的说明文档
- ✅ **允许创建基础项目文档**: 只在用户明确要求时创建 README.md 等必要文档
- ❌ **禁止子目录 README**: 不允许在 `cli/`、`crates/`、`tests/` 等子目录创建 README.md

### 质量妥协
- ❌ **不允许跳过格式化**: 每次代码修改后必须运行 `cargo fmt`
- ❌ **不允许忽略警告**: 所有编译和 Clippy 警告必须修复
- ❌ **不允许测试失败**: 所有测试必须通过才能提交

## 📋 输出和用户体验

### 分层输出控制
1. **库层**: 禁止直接输出，只使用 `log` 系统
2. **CLI层**: 通过统一 UI 模块格式化输出
3. **用户层**: 提供友好、有用的信息

### 错误输出规范
```rust
// ✅ 用户友好的错误
ui.error("安装失败: 权限不足");
ui.hint("💡 解决方案:");
ui.hint("   1. 使用 sudo 运行命令");
ui.hint("   2. 检查目录权限设置");

// ❌ 技术性错误信息
eprintln!("Error: std::fs::create_dir_all failed");
```

### 输出图标规范
- ✅ 成功操作 | ❌ 失败操作 | ⚠️ 警告信息
- 💡 提示信息 | 📁 目录/路径 | ⏳ 正在进行

## 💬 交互示例

**用户**: "修复这个 bug"
**Copilot**: "好的，我来帮您修复这个 bug。首先让我读取相关文件内容了解问题..."

**用户**: "检查代码质量"
**Copilot**: "我来运行完整的代码质量检查..." 
```bash
cargo fmt; cargo check --workspace; cargo clippy --workspace -- -D warnings; cargo test --workspace
```

**用户**: "提交代码"
**Copilot**: "在提交前，我先进行代码质量检查，然后使用规范的提交信息格式..."

## 📚 快速参考

### NuShell 多命令模式
```bash
# 完整开发流程
cargo fmt; cargo check; cargo clippy; cargo test; git add .; git commit -m "feat: add new feature"

# 构建和测试
cargo build --release; cargo test --release; cargo run --example demo

# Git 操作
git status; git add .; git commit -m "fix: resolve issue"; git push

# 文件操作
ls; cd src; ls *.rs; cd ..

# 清理和重建
cargo clean; cargo build; cargo test
```

### 质量检查快捷方式
```bash
# 标准检查
cargo fmt; cargo check --workspace; cargo clippy --workspace -- -D warnings

# 完整验证
cargo fmt; cargo check --workspace; cargo clippy --workspace -- -D warnings; cargo test --workspace

# 发布前检查
cargo fmt; cargo build --release; cargo test --release; cargo clippy --workspace -- -D warnings
```

### 测试和示例运行
```bash
# 测试相关命令
cargo test --workspace                    # 运行所有测试（跳过网络测试）
cargo test --workspace -- --ignored      # 运行包含网络测试的完整测试
cargo test -p tidepool-version-manager   # 运行特定包测试
cargo test --test go_manager_tests       # 运行特定测试文件

# 示例程序运行
cargo run --example temp_file_demo --package tidepool-version-manager
cargo run --example env_demo --package gvm

# 测试覆盖率
cargo test --workspace -- --test-threads=1
```

---

**记住**: 严格遵循此规范，确保代码质量、测试覆盖和用户体验！🏆

**测试组织核心**: 单元测试在源文件内，集成测试在 `tests/` 目录，示例在 `examples/` 目录！📋
