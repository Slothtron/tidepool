# 环境变量配置说明功能

## 功能概述

在 `gvm install` 成功安装并切换到对应版本后，CLI 工具现在会自动显示环境变量配置说明，帮助用户正确配置 Go 开发环境。

## 支持的操作系统和 Shell

### Windows
- **PowerShell** - 临时和永久配置
- **命令提示符(CMD)** - 临时配置
- **系统环境变量** - 图形界面配置说明

### Linux/macOS
- **Bash** - `~/.bashrc` 或 `~/.bash_profile`
- **Zsh** - `~/.zshrc`
- **Fish** - `~/.config/fish/config.fish`
- **NuShell** - `~/.config/nushell/config.nu`
- **其他 Shell** - 通用 Bash 语法

## 功能特性

1. **自动检测**: 根据 `$SHELL` 环境变量自动检测用户当前使用的 shell
2. **多种配置方式**: 提供临时配置和永久配置两种选择
3. **平台特定**: 针对不同操作系统显示相应的配置方法
4. **语法正确**: 为不同 shell 显示正确的语法格式
5. **路径自动计算**: 自动计算 GOROOT 和 PATH 的正确值

## 示例输出

### Linux 用户（NuShell）
```
📋 环境变量配置说明
================================

ℹ️  已切换到 Go 1.21.0，以下是环境变量配置说明：

🟢 当前会话临时配置:
    export GOROOT="/home/user/.gvm/versions/1.21.0"
    export PATH="/home/user/.gvm/versions/1.21.0/bin:$PATH"

🟢 NuShell 永久配置（添加到 ~/.config/nushell/config.nu）:
    $env.GOROOT = "/home/user/.gvm/versions/1.21.0"
    $env.PATH = ($env.PATH | prepend "/home/user/.gvm/versions/1.21.0/bin")

⚡ 立即应用配置:
💡    重启 NuShell 或重新加载配置

💡 切换完成！现在可以使用 Go 1.21.0 了
💡    运行 'go version' 验证当前版本
```

### Windows 用户（PowerShell）
```
📋 环境变量配置说明
================================

ℹ️  已切换到 Go 1.21.0，以下是环境变量配置说明：

🔷 PowerShell 临时配置（当前会话）:
    $env:GOROOT = "C:\gvm\versions\1.21.0"
    $env:PATH = "C:\gvm\versions\1.21.0\bin;$env:PATH"

🔷 PowerShell 永久配置（添加到 $PROFILE）:
    $env:GOROOT = "C:\gvm\versions\1.21.0"
    $env:PATH = "C:\gvm\versions\1.21.0\bin;$env:PATH"

🔶 命令提示符(CMD) 临时配置:
    set GOROOT=C:\gvm\versions\1.21.0
    set PATH=C:\gvm\versions\1.21.0\bin;%PATH%

⚙️ 系统环境变量配置（推荐）:
💡    1. 右键'此电脑' → 属性 → 高级系统设置
💡    2. 点击'环境变量'按钮
💡    3. 新建 GOROOT = C:\gvm\versions\1.21.0
💡    4. 编辑 PATH，添加 C:\gvm\versions\1.21.0\bin
💡    5. 重启终端生效

💡 切换完成！现在可以使用 Go 1.21.0 了
💡    运行 'go version' 验证当前版本
```

## 实现细节

- 在 `switch_to_existing_version` 函数中调用 `ui.show_environment_setup()`
- 根据编译时的目标操作系统选择相应的配置说明
- 通过环境变量 `$SHELL` 检测用户当前的 shell 类型
- 为每种 shell 提供正确的语法格式
- 包含特殊平台的提示（如 macOS 的不同终端应用）

## 测试

功能包含完整的测试用例：
- 基本功能测试
- 不同路径和版本的测试
- 跨平台兼容性验证

运行测试：
```bash
cargo test --package gvm environment_tests
```
