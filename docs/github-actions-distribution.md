# 🚀 GitHub Actions 多平台分发指南

本文档详细介绍如何使用 GitHub Actions 实现 Tidepool 项目的多平台自动分发。

## 📋 目录

- [概述](#概述)
- [支持的平台](#支持的平台)
- [工作流程详解](#工作流程详解)
- [配置文件说明](#配置文件说明)
- [发布流程](#发布流程)
- [自动化特性](#自动化特性)
- [故障排除](#故障排除)

## 🎯 概述

我们的 GitHub Actions 配置实现了以下功能：

- ✅ **多平台构建**: 支持 6 个主要平台和架构
- ✅ **自动发布**: 基于 Git 标签自动创建 GitHub Releases
- ✅ **代码质量保证**: 每次构建前进行完整的质量检查
- ✅ **安全验证**: 为所有二进制文件生成 SHA256 校验和
- ✅ **依赖管理**: 自动更新依赖项并创建 PR
- ✅ **文档生成**: 自动生成发布说明

## 🖥️ 支持的平台

### 正式支持的平台

| 平台 | 架构 | 文件名 | 说明 |
|------|------|--------|------|
| Linux | x86_64 | `gvm-x86_64-unknown-linux-gnu.tar.gz` | 主流 Linux 发行版 |
| Linux | ARM64 | `gvm-aarch64-unknown-linux-gnu.tar.gz` | ARM64 Linux 系统 |
| macOS | Intel | `gvm-x86_64-apple-darwin.tar.gz` | Intel Mac |
| macOS | Apple Silicon | `gvm-aarch64-apple-darwin.tar.gz` | M1/M2/M3 Mac |
| Windows | x86_64 | `gvm-x86_64-pc-windows-msvc.zip` | Windows 10/11 |
| Windows | ARM64 | `gvm-aarch64-pc-windows-msvc.zip` | ARM64 Windows |

### 平台特性

- **Linux**: 使用 GLIBC，兼容主流发行版
- **macOS**: 支持 macOS 10.12+ (Sierra)
- **Windows**: 使用 MSVC 运行时，支持 Windows 10+

## 🔄 工作流程详解

### 1. CI 工作流 (`ci.yml`)

**触发条件**:
- 推送到 `main` 或 `develop` 分支
- Pull Request 到 `main` 或 `develop` 分支

**执行任务**:
```yaml
jobs:
  test:           # 多平台测试矩阵
  security-audit: # 安全漏洞检查
  coverage:       # 代码覆盖率报告
  check-docs:     # 文档检查
  build-matrix:   # 构建矩阵验证
```

**质量标准**:
- ✅ 代码格式化检查 (`cargo fmt`)
- ✅ Clippy 静态分析 (`cargo clippy`)
- ✅ 编译检查 (`cargo check`)
- ✅ 单元和集成测试 (`cargo test`)
- ✅ 文档测试 (`cargo test --doc`)
- ✅ 安全审计 (`cargo audit`)

### 2. 发布工作流 (`release.yml`)

**触发条件**:
- 推送以 `v` 开头的标签 (如 `v1.0.0`)
- 手动触发 (`workflow_dispatch`)

**构建矩阵**:
```yaml
strategy:
  matrix:
    include:
      - target: x86_64-unknown-linux-gnu
        os: ubuntu-latest
        cross: false
      - target: aarch64-unknown-linux-gnu
        os: ubuntu-latest
        cross: true
      # ... 其他平台
```

**构建步骤**:
1. **环境准备**: 安装 Rust 工具链和目标平台
2. **依赖缓存**: 缓存 Cargo 注册表和构建目录
3. **跨平台构建**: 使用 `cross` 工具进行跨平台编译
4. **二进制优化**: 剥离符号表减少文件大小
5. **打包压缩**: 创建平台特定的压缩包
6. **校验和生成**: 计算 SHA256 校验和
7. **发布创建**: 上传到 GitHub Releases

### 3. 依赖更新工作流 (`update-dependencies.yml`)

**调度**: 每周一早上 8 点自动运行

**功能**:
- 自动更新 `Cargo.lock` 中的依赖版本
- 运行完整测试套件验证更新
- 创建 Pull Request 进行人工审核

## ⚙️ 配置文件说明

### `Cargo.toml` 优化配置

```toml
# 发布版本优化
[profile.release]
lto = true              # 链接时优化
codegen-units = 1       # 单个代码生成单元
panic = "abort"         # 减少二进制大小
opt-level = "z"         # 大小优化
strip = true            # 剥离符号

# 分发专用配置
[profile.dist]
inherits = "release"
lto = "fat"             # 最激进的LTO
overflow-checks = false # 禁用溢出检查
```

### `Cross.toml` 跨平台配置

```toml
[build.env]
passthrough = [
    "GITHUB_TOKEN",
    "CARGO_REGISTRY_TOKEN",
]

[target.aarch64-unknown-linux-gnu]
image = "ghcr.io/cross-rs/aarch64-unknown-linux-gnu:edge"
```

## 🚀 发布流程

### 自动发布 (推荐)

1. **使用发布脚本**:
   ```bash
   # 预览模式 - 查看将要执行的操作
   ./scripts/release.nu 1.0.0 --dry-run

   # 执行发布
   ./scripts/release.nu 1.0.0
   ```

2. **脚本执行流程**:
   - 检查工作目录状态
   - 更新版本号
   - 运行质量检查
   - 提交版本更新
   - 创建并推送标签
   - 触发 GitHub Actions

### 手动发布

1. **更新版本号**:
   ```bash
   # 编辑 Cargo.toml 中的版本号
   # [workspace.package]
   # version = "1.0.0"
   ```

2. **运行质量检查**:
   ```bash
   cargo fmt; cargo check --workspace; cargo clippy --workspace -- -D warnings; cargo test --workspace
   ```

3. **提交并创建标签**:
   ```bash
   git add .
   git commit -m "chore: bump version to 1.0.0"
   git tag -a v1.0.0 -m "Release 1.0.0"
   git push origin main
   git push origin v1.0.0
   ```

## 🤖 自动化特性

### 1. 智能发布说明生成

- 自动检测 `CHANGELOG.md` 并提取相关版本信息
- 生成包含下载说明和安装指导的发布说明
- 包含安全验证和平台兼容性信息

### 2. 预发布版本处理

- 自动检测包含 `alpha`、`beta`、`rc` 的标签
- 标记为预发布版本
- 不会触发 crates.io 发布

### 3. Crates.io 自动发布

- 仅在正式版本发布时触发
- 按依赖顺序发布包：
  1. `tidepool-version-manager`
  2. `gvm` (CLI)

### 4. 安全和质量保证

- 每次构建前运行完整测试套件
- 自动安全审计检查
- 代码覆盖率报告
- 文档测试验证

## 🔧 故障排除

### 常见问题

#### 1. 构建失败

**现象**: GitHub Actions 构建失败
**排查步骤**:
```bash
# 本地验证构建
cargo build --release --target x86_64-unknown-linux-gnu --package gvm

# 检查跨平台编译
cargo install cross
cross build --release --target aarch64-unknown-linux-gnu --package gvm
```

#### 2. 测试失败

**现象**: CI 测试阶段失败
**排查步骤**:
```bash
# 运行完整测试套件
cargo test --workspace

# 检查特定测试
cargo test --package tidepool-version-manager
cargo test --package gvm

# 运行被忽略的测试
cargo test --workspace -- --ignored
```

#### 3. 依赖问题

**现象**: 依赖解析或安全审计失败
**排查步骤**:
```bash
# 检查依赖树
cargo tree

# 运行安全审计
cargo audit

# 更新依赖
cargo update
```

#### 4. 发布权限问题

**现象**: 无法发布到 GitHub Releases 或 crates.io
**解决方案**:
- 检查 `GITHUB_TOKEN` 权限
- 确认 `CARGO_REGISTRY_TOKEN` 配置
- 验证仓库权限设置

### 调试技巧

#### 1. 本地测试 GitHub Actions

使用 [act](https://github.com/nektos/act) 本地运行 Actions:
```bash
# 安装 act
curl https://raw.githubusercontent.com/nektos/act/master/install.sh | sudo bash

# 运行 CI 工作流
act -W .github/workflows/ci.yml

# 运行发布工作流
act -W .github/workflows/release.yml --eventpath event.json
```

#### 2. 查看详细日志

在 GitHub Actions 页面:
1. 点击失败的工作流
2. 展开失败的步骤
3. 查看完整日志输出

#### 3. 重现构建环境

```bash
# 使用相同的 Docker 镜像
docker run --rm -it ghcr.io/cross-rs/aarch64-unknown-linux-gnu:edge

# 在容器中构建
cargo build --release --target aarch64-unknown-linux-gnu
```

## 📚 相关资源

### 官方文档
- [GitHub Actions 文档](https://docs.github.com/en/actions)
- [Rust 跨平台编译](https://rust-lang.github.io/rustup/cross-compilation.html)
- [Cross 工具使用](https://github.com/cross-rs/cross)

### 最佳实践
- [Rust 发布最佳实践](https://doc.rust-lang.org/cargo/guide/publishing.html)
- [GitHub Releases 指南](https://docs.github.com/en/repositories/releasing-projects-on-github)
- [Cargo 配置优化](https://doc.rust-lang.org/cargo/reference/profiles.html)

## 🔗 快速链接

- [GitHub Actions 页面](https://github.com/Slothtron/tidepool/actions)
- [GitHub Releases](https://github.com/Slothtron/tidepool/releases)
- [Crates.io 页面](https://crates.io/crates/gvm)
- [项目仓库](https://github.com/Slothtron/tidepool)

---

**💡 提示**: 如果您遇到任何问题，请先查看 [GitHub Actions 页面](https://github.com/Slothtron/tidepool/actions) 的构建日志，或者在项目仓库中创建 Issue。
