# tidepool-hosts-manager

[![Build Status](https://github.com/Slothtron/tidepool/workflows/Rust/badge.svg)](https://github.com/Slothtron/tidepool)
[![License: MIT](https://img.shields.io/github/license/Slothtron/tidepool)](https://github.com/Slothtron/tidepool)
[![Crates.io](https://img.shields.io/crates/v/tidepool-hosts-manger)](https://crates.io/crates/tidepool-hosts-manger)

Hosts 文件管理工具库，提供以下核心功能：
- Hosts 映射解析与管理
- 多文件分组管理
- 网络代理下载服务

## 📦 功能特性

### 1. Hosts 映射管理
- 支持标准 hosts 文件格式（IPv4/IPv6、域名、注释）
- 提供 `HostEntry` 结构体操作条目
- 支持读写系统 hosts 文件, 提示要求管理员权限

### 2. Hosts 分组管理
- 支持多个 hosts 文件的逻辑分组
- 可动态添加/切换分组
- 配置本地持久化存储（`groups.toml`）

### 3. 网络代理服务
- 启用中间网络代理服务
- 支持不同分组的hosts启用不同的端口
- 用户请求到达代理服务后, 查找是否有自定义的hosts映射, 如果没有则使用系统DNS

## 🛠️ 安装

在 `Cargo.toml` 中添加依赖：
```toml
[dependencies]
tidepool-hosts-manger = "0.1.0"
```

## 🚀 使用示例

### 读写 Hosts 文件
```rust
use tidepool_hosts_manger::{HostsManager, HostEntry};

let manager = HostsManager::new("/etc/hosts");
let entries = manager.read_hosts().unwrap();

// 添加新条目
let mut new_entries = entries.clone();
new_entries.push(HostEntry::new("10.0.0.1", &["test.local"]));
manager.write_hosts(&new_entries).unwrap();
```

### 分组管理
```rust
use tidepool_hosts_manger::group::GroupManager;
use std::fs;

let mut manager = GroupManager::new("./config").unwrap();
manager.add_group("dev", "dev_hosts.txt");
manager.switch_group("dev").unwrap();
```

### 代理下载
```rust
use tidepool_hosts_manger::proxy::ProxyService;
use tempfile::NamedTempFile;

let proxy = ProxyService::new("https://example.com/hosts.txt");
let temp_file = NamedTempFile::new().unwrap();
proxy.download_hosts(temp_file.path().to_str().unwrap()).await.unwrap();
```

## 🧪 测试与构建

```bash
# 构建项目
cargo build

# 运行所有测试
cargo test -p tidepool-hosts-manger

# 单独测试代理服务
cargo test -p tidepool-hosts-manger --test proxy_service_integration_test
```

## 📄 许可证

本项目采用 MIT 许可证。详见 [LICENSE](LICENSE) 文件。

## 🤝 贡献指南

欢迎提交 Issue 和 Pull Request。请遵循 [Contribution Guidelines](CONTRIBUTING.md)。

## 📁 目录结构
```
src/
├── host_entry.rs     # HostEntry 结构体定义
├── group.rs          # 分组管理实现
├── proxy.rs          # 网络代理服务
└── lib.rs            # 模块入口
```
