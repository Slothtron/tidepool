# 🚀 发布前检查清单

在创建新版本发布之前，请确保完成以下所有检查项。

## 📋 必要检查项

### 1. 代码质量 ✅

- [ ] **格式化检查**: `cargo fmt --all -- --check`
- [ ] **编译检查**: `cargo check --workspace`
- [ ] **静态分析**: `cargo clippy --workspace -- -D warnings`
- [ ] **测试通过**: `cargo test --workspace`
- [ ] **文档测试**: `cargo test --workspace --doc`
- [ ] **安全审计**: `cargo audit`

### 2. 功能验证 ✅

- [ ] **CLI 帮助**: `cargo run --package gvm -- --help`
- [ ] **版本信息**: `cargo run --package gvm -- --version`
- [ ] **基本功能**: 测试核心命令是否正常工作
- [ ] **示例程序**: 运行 examples 确保演示正常

### 3. 跨平台构建 ✅

- [ ] **Linux x86_64**: `cargo build --release --target x86_64-unknown-linux-gnu --package gvm`
- [ ] **Windows**: `cargo build --release --target x86_64-pc-windows-msvc --package gvm`
- [ ] **macOS**: `cargo build --release --target x86_64-apple-darwin --package gvm`

### 4. 文档更新 ✅

- [ ] **版本号更新**: 检查 `Cargo.toml` 中的版本号
- [ ] **CHANGELOG**: 更新变更日志（如果存在）
- [ ] **README**: 确保安装说明准确
- [ ] **文档同步**: API 文档和实际代码保持一致

### 5. Git 状态 ✅

- [ ] **工作目录清理**: `git status` 显示无未提交更改
- [ ] **在主分支**: 当前在 `main` 分支
- [ ] **远程同步**: 本地分支与远程保持同步

## 🔧 发布执行步骤

### 自动发布（推荐）

```bash
# 1. 使用发布脚本（预览模式）
./scripts/release.nu 1.0.0 --dry-run

# 2. 确认无误后执行发布
./scripts/release.nu 1.0.0
```

### 手动发布

```bash
# 1. 运行完整检查
cargo fmt; cargo check --workspace; cargo clippy --workspace -- -D warnings; cargo test --workspace

# 2. 更新版本号
# 编辑 Cargo.toml: version = "1.0.0"

# 3. 提交版本更新
git add .
git commit -m "chore: bump version to 1.0.0"

# 4. 创建标签
git tag -a v1.0.0 -m "Release 1.0.0"

# 5. 推送到远程
git push origin main
git push origin v1.0.0
```

## 🤖 GitHub Actions 验证

发布后检查以下 GitHub Actions 工作流：

- [ ] **CI 工作流**: 代码质量检查通过
- [ ] **Release 工作流**: 多平台构建成功
- [ ] **Assets 上传**: 所有平台的二进制文件都已上传
- [ ] **Checksums**: SHA256SUMS 文件已生成
- [ ] **Release Notes**: 发布说明已自动生成

## 📦 发布后验证

### 1. GitHub Releases

- [ ] **Release 页面**: 确认 release 已创建
- [ ] **下载链接**: 测试各平台下载链接
- [ ] **文件大小**: 验证二进制文件大小合理
- [ ] **校验和**: 验证 SHA256SUMS 文件

### 2. 功能测试

下载并测试发布的二进制文件：

```bash
# Linux
curl -L https://github.com/Slothtron/tidepool/releases/latest/download/gvm-x86_64-unknown-linux-gnu.tar.gz | tar xz
./gvm --version
./gvm --help

# 验证校验和
sha256sum gvm > actual.sum
curl -L https://github.com/Slothtron/tidepool/releases/latest/download/SHA256SUMS | grep x86_64-unknown-linux-gnu > expected.sum
diff actual.sum expected.sum
```

### 3. Crates.io（如果适用）

- [ ] **包发布**: 确认包已发布到 crates.io
- [ ] **文档生成**: docs.rs 文档已生成
- [ ] **下载测试**: `cargo install gvm` 成功

## ⚠️ 回滚步骤

如果发布出现问题，执行以下回滚操作：

### 删除错误的 Release

```bash
# 1. 删除远程标签
git push --delete origin v1.0.0

# 2. 删除本地标签
git tag -d v1.0.0

# 3. 在 GitHub 页面手动删除 Release
```

### 从 crates.io 撤回（慎用）

```bash
# 只有在包有严重问题时才使用
cargo yank --vers 1.0.0 gvm
```

## 🔗 相关资源

- [GitHub Releases](https://github.com/Slothtron/tidepool/releases)
- [GitHub Actions](https://github.com/Slothtron/tidepool/actions)
- [Crates.io - gvm](https://crates.io/crates/gvm)
- [发布脚本](../scripts/release.nu)
- [多平台分发文档](./github-actions-distribution.md)

## 📞 获取帮助

如果在发布过程中遇到问题：

1. 检查 [GitHub Actions 日志](https://github.com/Slothtron/tidepool/actions)
2. 查看 [故障排除文档](./github-actions-distribution.md#故障排除)
3. 在 GitHub 仓库中创建 Issue

---

**记住**: 发布是不可逆的操作，请仔细检查每一项！🚨
