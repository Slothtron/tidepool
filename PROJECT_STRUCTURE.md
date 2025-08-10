# Tidepool 项目结构

## 概述

Tidepool 是一个用 Rust 编写的高性能 Go 版本管理工具包。项目采用单一 crate 设计，简化了架构，提高了可维护性。

## 目录结构

```
tidepool/
├── src/                          # 源代码目录
│   ├── main.rs                   # CLI 入口点
│   ├── lib.rs                    # 库入口点
│   ├── cli.rs                    # CLI 命令解析
│   ├── commands.rs               # 命令实现
│   ├── config.rs                 # 配置管理
│   ├── ui.rs                     # 用户界面
│   ├── go.rs                     # Go 版本管理核心
│   ├── downloader.rs             # 下载器
│   └── symlink.rs                # 符号链接处理
├── README.md                     # 英文文档
├── README.zh-CN.md              # 中文文档
├── Cargo.toml                    # 包配置
├── Cargo.lock                    # 依赖锁定文件
├── rustfmt.toml                  # Rust 格式化配置
├── .gitignore                    # Git 忽略文件
└── .github/                      # GitHub 工作流
```

## 模块说明

### 核心模块

- **`main.rs`**: CLI 应用程序入口点
- **`lib.rs`**: 库模块定义和公共 API
- **`cli.rs`**: 命令行参数解析和命令分发
- **`commands.rs`**: 具体命令的实现逻辑
- **`config.rs`**: 配置管理和环境变量处理
- **`ui.rs`**: 用户界面和消息显示

### 功能模块

- **`go.rs`**: Go 版本管理的核心逻辑
  - 版本安装、切换、卸载
  - 版本信息获取和验证
  - 环境变量配置

- **`downloader.rs`**: 文件下载功能
  - 异步并发下载
  - 进度显示
  - 断点续传支持
  - SHA256 校验

- **`symlink.rs`**: 跨平台符号链接处理
  - Windows 和 Unix 系统兼容
  - 符号链接创建和删除
  - 链接状态检查

## 设计原则

### 1. 单一职责
每个模块专注于特定功能，职责明确。

### 2. 跨平台兼容
所有平台相关代码都通过条件编译处理。

### 3. 错误处理
使用 `anyhow` 和 `thiserror` 进行统一的错误处理。

### 4. 异步优先
充分利用 Rust 的异步特性，提高性能。

### 5. 用户友好
提供清晰的错误信息和进度反馈。

## 构建和运行

### 开发环境
```bash
# 克隆项目
git clone https://github.com/Slothtron/tidepool.git
cd tidepool

# 构建项目
cargo build --release

# 运行测试
cargo test

# 安装到系统
cargo install --path .
```

### 跨平台构建
```bash
# 构建当前平台
cargo build --release

# 交叉编译
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-pc-windows-msvc
```

## 配置

项目支持通过环境变量进行配置：

- `GVM_ROOT_PATH`: 根目录路径
- `GVM_VERSIONS_PATH`: 版本安装目录
- `GVM_CACHE_PATH`: 缓存目录
- `RUST_LOG`: 日志级别

## 扩展性

项目设计考虑了未来的扩展性：

1. **多语言支持**: 可以轻松添加其他语言的版本管理
2. **插件系统**: 支持自定义下载器和存储后端
3. **配置扩展**: 支持更多配置选项和自定义规则

## 维护

### 代码质量
- 使用 `rustfmt` 进行代码格式化
- 遵循 Rust 编码规范
- 完整的错误处理和文档注释

### 测试
- 单元测试覆盖核心功能
- 集成测试验证完整流程
- 跨平台兼容性测试

### 发布
- 使用 GitHub Actions 进行自动化构建和发布
- 支持多平台二进制文件发布
- 版本管理和变更日志维护
