# 使用 Podman 进行跨平台构建

本项目支持使用 Podman 替代 Docker 进行跨平台编译。

## 🐚 环境要求

- **Rust**: 1.70.0 或更高版本
- **Cross**: `cargo install cross`
- **Podman**: 已安装并运行

## 🚀 快速开始

### 1. 设置 Podman 作为容器引擎

```nushell
# 在 NuShell 中设置环境变量
$env.CROSS_CONTAINER_ENGINE = 'podman'

# 或在其他 Shell 中
export CROSS_CONTAINER_ENGINE=podman
```

### 2. 构建指定目标

```nushell
# 构建 Linux x86_64 (glibc)
cross build --target x86_64-unknown-linux-gnu --release -p gvm

# 构建 Linux x86_64 (musl, 静态链接)
cross build --target x86_64-unknown-linux-musl --release -p gvm

# 构建 Linux ARM64
cross build --target aarch64-unknown-linux-gnu --release -p gvm
```

### 3. 使用构建脚本

```nushell
# 构建所有支持的目标
nu scripts/build_cross.nu all

# 测试 musl 二进制文件
nu scripts/build_cross.nu test

# 清理构建目录
nu scripts/build_cross.nu clean
```

## 📦 支持的目标平台

| 目标 | 描述 | 输出路径 | 状态 |
|------|------|----------|------|
| `x86_64-unknown-linux-gnu` | Linux x86_64 (glibc) | `target/x86_64-unknown-linux-gnu/release/gvm` | ✅ 支持 |
| `x86_64-unknown-linux-musl` | Linux x86_64 (musl, 静态) | `target/x86_64-unknown-linux-musl/release/gvm` | ✅ 支持 |
| `aarch64-unknown-linux-gnu` | Linux ARM64 (glibc) | `target/aarch64-unknown-linux-gnu/release/gvm` | ✅ 支持 |
| `armv7-unknown-linux-gnueabihf` | Linux ARM7 (硬浮点) | `target/armv7-unknown-linux-gnueabihf/release/gvm` | ✅ 支持 |
| `x86_64-apple-darwin` | macOS x86_64 | `target/x86_64-apple-darwin/release/gvm` | ⚠️ 需要特殊工具链 |
| `aarch64-apple-darwin` | macOS ARM64 (Apple Silicon) | `target/aarch64-apple-darwin/release/gvm` | ⚠️ 需要特殊工具链 |

### 🍎 macOS 交叉编译说明

在 Windows 上使用 Podman/Docker 交叉编译 macOS 目标较为复杂，需要：
- macOS SDK
- 特殊的链接器和工具链
- 可能涉及法律许可问题

**推荐方案**：
- 在 macOS 机器上直接编译
- 使用 GitHub Actions 等 CI/CD 服务
- 使用云端 macOS 构建环境

## 🔧 依赖配置

项目已配置使用 `rustls` 替代 `openssl`，避免交叉编译时的 OpenSSL 依赖问题：

```toml
[workspace.dependencies.reqwest]
version = "0.11"
default-features = false
features = [
    "stream",
    "blocking", 
    "json",
    "rustls-tls",  # 使用 rustls 替代 native-tls
]
```

## 🧪 验证构建结果

### 检查二进制文件

```nushell
# 查看所有构建的二进制文件
ls target/*/release/gvm | select name size

# 在 Alpine 容器中测试 musl 版本
podman run --rm -v (pwd):/workspace alpine:latest /workspace/target/x86_64-unknown-linux-musl/release/gvm --help
```

### 验证架构

```nushell
# 使用 file 命令检查架构 (需要在 Linux 容器中运行)
podman run --rm -v (pwd):/workspace ubuntu:latest file /workspace/target/x86_64-unknown-linux-gnu/release/gvm
```

## 🚨 故障排除

### 1. Podman 虚拟机未运行

```nushell
# 检查 Podman 虚拟机状态
podman machine ls

# 启动虚拟机 (如果需要)
podman machine start
```

### 2. 权限问题

确保 Podman 有权限访问项目目录。

### 3. 容器镜像拉取问题

Cross 会自动拉取所需的构建镜像，确保网络连接正常。

## 💡 优势

- **无需 Docker Desktop**: 在 Windows 上使用 Podman 作为轻量级替代方案
- **静态链接**: musl 目标生成静态链接的二进制文件，无需运行时依赖
- **多架构支持**: 同时支持 x86_64 和 ARM64 架构
- **一致性**: 使用容器确保构建环境的一致性

---

**提示**: 首次构建会下载相应的容器镜像，后续构建会更快。
