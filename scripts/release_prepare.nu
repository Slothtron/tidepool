#!/usr/bin/env nu

# GVM 发布准备脚本
# 执行完整的代码质量检查和发布准备

def main [version?: string] {
    print "🚀 准备发布 Tidepool Version Manager (gvm)..."

    if ($version | is-empty) {
        let current_version = (open Cargo.toml | get workspace.package.version)
        print $"📦 当前版本: ($current_version)"
    } else {
        print $"📦 目标版本: ($version)"
        # 这里可以添加版本更新逻辑
    }

    print ""
    print "🔍 执行发布前检查..."

    # 1. 代码格式化
    print "🎨 格式化代码..."
    try {
        cargo fmt --check
        print "✅ 代码格式正确"
    } catch {
        print "⚠️ 代码需要格式化，正在自动格式化..."
        cargo fmt
        print "✅ 代码已格式化"
    }

    # 2. 编译检查
    print "🔧 检查编译..."
    try {
        cargo check --workspace
        print "✅ 编译检查通过"
    } catch {
        print "❌ 编译检查失败"
        exit 1
    }

    # 3. Clippy 检查
    print "🔍 运行 Clippy 检查..."
    try {
        cargo clippy --workspace -- -D warnings
        print "✅ Clippy 检查通过"
    } catch {
        print "❌ Clippy 检查失败，请修复警告"
        exit 1
    }    # 4. 运行测试
    print "🧪 运行测试..."
    try {
        cargo test --workspace
        print "✅ 所有测试通过"
    } catch {
        print "❌ 测试失败"
        exit 1
    }

    # 5. 构建发布版本
    print "📦 构建发布版本..."
    try {
        cargo build --release --package tidepool-gvm
        print "✅ 发布版本构建成功"
    } catch {
        print "❌ 发布版本构建失败"
        exit 1
    }    # 6. 验证二进制文件
    print "🔍 验证二进制文件..."
    let binary_path = if (sys host | get name) == "Windows" {
        "target/release/gvm.exe"
    } else {
        "target/release/gvm"
    }

    if ($binary_path | path exists) {
        let version_output = (do { ^$binary_path --version } | complete)
        if $version_output.exit_code == 0 {
            print $"✅ 二进制文件验证成功: ($version_output.stdout | str trim)"
        } else {
            print "❌ 二进制文件验证失败"
            exit 1
        }
    } else {
        print $"❌ 找不到二进制文件: ($binary_path)"
        exit 1
    }

    print ""
    print "🎉 发布前检查全部通过！"
    print ""
    print "📋 下一步操作："
    print "1. 提交所有更改："
    print "   git add ."
    print "   git commit -m \"chore: prepare for v0.1.2 release\""
    print "2. 创建并推送标签："
    print "   git tag -a v0.1.2 -m \"feat: release v0.1.2 with GitHub Actions automation\""
    print "   git push origin main"
    print "   git push origin v0.1.2"
    print "3. GitHub Actions 将自动构建和发布"
    print ""
    print "💡 发布后验证："
    print "- 检查 GitHub Releases 页面"
    print "- 验证所有平台的二进制文件"
    print "- 测试 cargo install --git 安装方式"
}
