# 代码质量检查体系

本文档介绍 Tidepool 项目的跨平台代码质量检查体系，支持 Windows、Linux 和 macOS 环境。

## 🛠️ 工具概览

### 核心脚本
- `scripts/code_quality.nu` - 跨平台代码质量检查主脚本

### 支持的检查项目
1. **代码格式检查** (`rustfmt`)
2. **静态代码分析** (`clippy`)
3. **单元测试** (`cargo test`)
4. **依赖安全检查** (`cargo-audit`)
5. **文档生成** (`cargo doc`)
6. **构建验证** (`cargo build`)
7. **Cargo.toml 格式验证**
8. **代码覆盖率** (`cargo-tarpaulin`, 仅 Linux/macOS)

## 🚀 快速开始

### 1. 安装必要工具

```bash
# 安装所有代码质量检查工具
nu scripts/code_quality.nu tools
```

这将自动安装：
- `rustfmt` (代码格式化)
- `clippy` (静态分析)
- `cargo-audit` (安全检查)
- `cargo-tarpaulin` (覆盖率，仅 Unix 系统)

### 2. 快速检查

```bash
# 运行基础质量检查（格式、静态分析、测试）
nu scripts/code_quality.nu quick
```

### 3. 完整检查

```bash
# 运行所有质量检查项目
nu scripts/code_quality.nu full
```

## 📋 命令详解

### 基本命令

| 命令 | 描述 | 适用场景 |
|------|------|----------|
| `quick` | 快速检查（格式、静态分析、测试） | 提交前检查 |
| `full` | 完整检查（包括安全、文档等） | 发布前验证 |
| `fix` | 自动修复可修复的问题 | 代码维护 |
| `format` | 格式化代码 | 代码整理 |
| `tools` | 安装必要工具 | 环境设置 |
| `metrics` | 显示代码指标统计 | 项目分析 |

### 专项检查

| 命令 | 描述 | 工具 |
|------|------|------|
| `check-format` | 检查代码格式 | `rustfmt --check` |
| `clippy` | 静态代码分析 | `cargo clippy` |
| `test` | 运行单元测试 | `cargo test` |
| `security` | 安全漏洞检查 | `cargo audit` |
| `coverage` | 代码覆盖率 | `cargo tarpaulin` |
| `docs` | 生成文档 | `cargo doc` |

## 🔧 平台特性

### Windows 支持
- ✅ 代码格式化
- ✅ 静态分析
- ✅ 单元测试
- ✅ 安全检查
- ✅ 文档生成
- ❌ 代码覆盖率 (tarpaulin 不支持)

### Linux/macOS 支持
- ✅ 所有检查项目
- ✅ 完整的代码覆盖率支持

### 跨平台兼容性
- 自动检测操作系统类型
- 根据平台调整检查项目
- 统一的命令接口

## 📊 质量指标

运行 `nu scripts/code_quality.nu metrics` 可查看：

- 📄 Rust 文件数量
- 📏 总代码行数
- 📦 依赖数量
- 🧪 测试文件数量
- 🔬 测试函数数量
- 📦 二进制文件大小

## 🔄 工作流程建议

### 开发阶段
```bash
# 1. 修复代码格式和基础问题
nu scripts/code_quality.nu fix

# 2. 快速验证
nu scripts/code_quality.nu quick
```

### 提交前
```bash
# 完整质量检查
nu scripts/code_quality.nu full
```

### 发布前
```bash
# 完整检查 + 指标统计
nu scripts/code_quality.nu full
nu scripts/code_quality.nu metrics
```

## 🚨 错误处理

### 常见问题与解决方案

#### 1. 工具未安装
```
❌ cargo-audit 未安装
💡 运行: nu scripts/code_quality.nu tools
```

#### 2. 代码格式问题
```bash
# 自动修复格式问题
nu scripts/code_quality.nu format
```

#### 3. Clippy 警告
```bash
# 查看详细警告信息
cargo clippy --all-targets --all-features

# 自动修复部分问题
nu scripts/code_quality.nu fix
```

#### 4. 测试失败
```bash
# 查看测试详情
cargo test --all -- --nocapture
```

## 📈 集成建议

### VS Code 集成
可以将常用命令添加到 VS Code tasks.json：

```json
{
    "label": "code-quality-quick",
    "type": "shell",
    "command": "nu",
    "args": ["scripts/code_quality.nu", "quick"],
    "group": "test"
}
```

### CI/CD 集成
在 GitHub Actions 或其他 CI 系统中：

```yaml
- name: Code Quality Check
  run: |
    nu scripts/code_quality.nu tools
    nu scripts/code_quality.nu full
```

### Git Hooks
可以在 pre-commit hook 中添加：

```bash
#!/bin/sh
nu scripts/code_quality.nu quick
```

## 🔍 最佳实践

1. **提交前必做**：运行快速检查
2. **定期执行**：完整检查
3. **问题修复**：使用自动修复功能
4. **指标跟踪**：定期查看代码指标
5. **工具更新**：保持工具最新版本

## 🎯 配置自定义

### 调整检查项目
可以编辑 `scripts/code_quality.nu` 文件：
- 添加新的检查项目
- 调整现有检查参数
- 自定义平台特定行为

### 扩展功能
脚本支持轻松扩展：
- 添加新的代码质量工具
- 自定义报告格式
- 集成更多静态分析工具

## ⚡ 性能优化

- 使用 `--quiet` 参数减少输出
- 并行运行独立检查项目
- 缓存构建结果
- 增量检查支持

## 📚 参考资源

- [Rust 代码风格指南](https://doc.rust-lang.org/1.0.0/style/)
- [Clippy Lint 列表](https://rust-lang.github.io/rust-clippy/master/)
- [cargo-audit 文档](https://github.com/RustSec/rustsec/tree/main/cargo-audit)
- [tarpaulin 使用指南](https://github.com/xd009642/tarpaulin)

---

通过这套完整的代码质量检查体系，Tidepool 项目能够确保代码质量，提高开发效率，并保持跨平台兼容性。
