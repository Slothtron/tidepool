# Tidepool GVM 使用指南

## 🚀 快速开始

### 安装

```bash
# 从源码安装
git clone https://github.com/Slothtron/tidepool-gvm.git
cd tidepool-gvm
cargo install --path .
```

### 基本使用

```bash
# 安装 Go 版本
gvm install 1.21.3

# 切换版本
gvm use 1.21.3

# 查看状态
gvm status

# 列出版本
gvm list
```

## 📖 详细命令说明

### 安装命令 (`install`)

安装指定版本的 Go。

```bash
gvm install <版本号> [选项]
```

**选项**:
- `-f, --force`: 强制重新安装（覆盖现有版本）
- `-v, --verbose`: 显示详细安装过程
- `-q, --quiet`: 静默安装（仅显示错误）

**示例**:
```bash
gvm install 1.21.3              # 安装 Go 1.21.3
gvm install 1.22.1 --force      # 强制重新安装
gvm install 1.20.5 --verbose    # 详细模式安装
```

### 切换命令 (`use`)

切换到已安装的 Go 版本。

```bash
gvm use <版本号> [选项]
```

**选项**:
- `-g, --global`: 全局设置（影响所有项目）
- `-v, --verbose`: 显示详细切换过程
- `-q, --quiet`: 静默切换

**示例**:
```bash
gvm use 1.21.3                  # 切换到 Go 1.21.3
gvm use 1.22.1 --global         # 全局切换到 Go 1.22.1
gvm use 1.20.5 --verbose        # 详细模式切换
```

### 列表命令 (`list`)

列出 Go 版本信息。

```bash
gvm list [选项]
```

**选项**:
- `-a, --all`: 显示所有可用版本（包括远程）
- `-v, --verbose`: 显示详细版本信息
- `-q, --quiet`: 简洁输出

**示例**:
```bash
gvm list                        # 列出已安装版本
gvm list --all                  # 列出所有可用版本
gvm list --verbose              # 详细版本信息
```

**输出示例**:
```
> 已安装的 Go 版本
  - 1.21.3
  * 1.23.10 (当前版本)
[INFO] 总计: 2 个版本
[TIP] 使用 gvm use <版本> 切换版本
```

### 状态命令 (`status`)

显示当前 Go 版本和环境状态。

```bash
gvm status [选项]
```

**选项**:
- `-v, --verbose`: 显示详细环境信息
- `-q, --quiet`: 简洁状态输出

**示例**:
```bash
gvm status                      # 显示当前状态
gvm status --verbose            # 详细环境信息
```

**输出示例**:
```
[OK] 当前版本: Go 1.23.10
  安装路径: C:\Users\User\.gvm\versions\1.23.10
[INFO] Go 环境已配置
[TIP] 使用 'go version' 验证安装
```

### 卸载命令 (`uninstall`)

卸载指定版本的 Go。

```bash
gvm uninstall <版本号> [选项]
```

**选项**:
- `-v, --verbose`: 显示详细卸载过程
- `-q, --quiet`: 静默卸载

**示例**:
```bash
gvm uninstall 1.21.3            # 卸载 Go 1.21.3
gvm uninstall 1.20.5 --verbose  # 详细模式卸载
```

### 信息命令 (`info`)

显示指定版本的详细信息。

```bash
gvm info <版本号> [选项]
```

**选项**:
- `-v, --verbose`: 显示更多详细信息
- `-q, --quiet`: 简洁信息输出

**示例**:
```bash
gvm info 1.21.3                 # 显示版本信息
gvm info 1.22.1 --verbose       # 详细版本信息
```

## 🎯 使用场景

### 开发环境管理

```bash
# 为不同项目使用不同 Go 版本
cd project-old
gvm use 1.19.5

cd project-new  
gvm use 1.21.3

# 检查当前项目使用的版本
gvm status
```

### 版本测试

```bash
# 安装多个版本进行兼容性测试
gvm install 1.20.5
gvm install 1.21.3
gvm install 1.22.1

# 切换版本测试
gvm use 1.20.5
go test ./...

gvm use 1.21.3
go test ./...
```

### 全局环境设置

```bash
# 设置全局默认版本
gvm use 1.21.3 --global

# 验证全局设置
gvm status
go version
```

## ⚙️ 配置和环境

### 默认安装路径

- **Windows**: `C:\Users\<用户名>\.gvm\versions\`
- **macOS/Linux**: `~/.gvm/versions/`

### 环境变量

GVM 会自动配置以下环境变量：

- `GOROOT`: Go 安装路径
- `GOPATH`: Go 工作空间路径  
- `PATH`: 添加 Go 二进制文件路径

### 配置文件

GVM 配置文件位置：

- **Windows**: `C:\Users\<用户名>\.gvm\config.toml`
- **macOS/Linux**: `~/.gvm/config.toml`

## 🔧 故障排除

### 常见问题

#### 1. 版本切换后 `go version` 显示旧版本

**解决方案**:
```bash
# 重启终端或重新加载环境
# Windows (PowerShell)
refreshenv

# macOS/Linux (Bash/Zsh)
source ~/.bashrc  # 或 ~/.zshrc
```

#### 2. 安装失败：网络连接问题

**解决方案**:
```bash
# 使用详细模式查看错误信息
gvm install 1.21.3 --verbose

# 检查网络连接
ping golang.org
```

#### 3. 权限错误

**解决方案**:
```bash
# Windows: 以管理员身份运行
# macOS/Linux: 检查目录权限
chmod 755 ~/.gvm
```

#### 4. 版本不存在

**解决方案**:
```bash
# 查看可用版本
gvm list --all

# 使用正确的版本号格式
gvm install 1.21.3  # 正确
gvm install v1.21.3  # 错误
```

### 调试模式

```bash
# 启用详细日志
RUST_LOG=debug gvm install 1.21.3

# 查看更多信息
gvm status --verbose
gvm list --verbose
```

## 💡 最佳实践

### 1. 版本管理策略

```bash
# 为每个项目创建 .go-version 文件（未来功能）
echo "1.21.3" > .go-version

# 使用语义化版本号
gvm install 1.21.3  # 推荐
gvm install 1.21     # 不推荐
```

### 2. 定期清理

```bash
# 定期查看已安装版本
gvm list

# 卸载不需要的版本
gvm uninstall 1.19.5
```

### 3. 备份重要版本

```bash
# 在升级前备份当前工作版本
gvm status  # 记录当前版本
gvm install 1.22.1  # 安装新版本
gvm use 1.21.3      # 如需回退
```

## 🚀 高级用法

### 脚本自动化

```bash
#!/bin/bash
# 自动安装和切换脚本

# 安装多个版本
versions=("1.20.5" "1.21.3" "1.22.1")
for version in "${versions[@]}"; do
    echo "安装 Go $version..."
    gvm install "$version" --quiet
done

# 设置默认版本
gvm use 1.21.3 --global
echo "设置完成！"
```

### CI/CD 集成

```yaml
# GitHub Actions 示例
- name: Setup Go
  run: |
    gvm install 1.21.3
    gvm use 1.21.3
    go version
```

## 📚 更多资源

- [Go 官方文档](https://golang.org/doc/)
- [Go 版本发布历史](https://golang.org/doc/devel/release.html)
- [项目 GitHub 仓库](https://github.com/Slothtron/tidepool-gvm)

## 🤝 获得帮助

如果遇到问题：

1. 查看内置帮助：`gvm --help`
2. 查看命令帮助：`gvm <命令> --help`
3. 启用详细模式：`gvm <命令> --verbose`
4. 提交 Issue 到 GitHub 仓库

---

**提示**: 使用 `gvm --help` 随时查看最新的命令选项和帮助信息。
