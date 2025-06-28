#!/usr/bin/env nu

# 🚀 Tidepool 发布脚本
# 用于创建新版本标签并触发 GitHub Actions 发布流程

def main [
    version: string,          # 版本号，例如 "1.0.0"
    --dry-run                 # 仅显示将要执行的命令，不实际执行
] {
    let tag = $"v($version)"

    print $"🎯 准备发布版本: ($tag)"

    # 检查是否在 git 仓库中
    if not (ls -la | where name == ".git" | is-not-empty) {
        print "❌ 错误: 当前目录不是 git 仓库"
        return
    }

    # 检查工作目录是否干净
    let git_status = (git status --porcelain | lines | where $it != "")
    if ($git_status | is-not-empty) {
        print "❌ 错误: 工作目录不干净，请先提交所有更改"
        print "未提交的文件:"
        $git_status | each { |file| print $"  ($file)" }
        return
    }

    # 检查是否在主分支
    let current_branch = (git branch --show-current)
    if $current_branch != "main" {
        print $"⚠️  警告: 当前不在 main 分支 (当前: ($current_branch))"
        let confirm = (input "是否继续? (y/N): ")
        if $confirm != "y" and $confirm != "Y" {
            print "❌ 取消发布"
            return
        }
    }

    # 更新版本号
    print "📝 更新版本号..."
    if not $dry_run {        # 更新 Cargo.toml 中的版本号
        let cargo_toml = (open Cargo.toml)
        let updated_cargo = ($cargo_toml | upsert workspace.package.version $version)
        $updated_cargo | save --force Cargo.toml

        # 运行 cargo check 来更新 Cargo.lock
        cargo check --workspace
    } else {
        print $"  将更新版本号到: ($version)"
    }

    # 运行完整的质量检查
    print "🔍 运行代码质量检查..."
    let quality_commands = [
        "cargo fmt --all",
        "cargo check --workspace",
        "cargo clippy --workspace -- -D warnings",
        "cargo test --workspace"
    ]

    for command in $quality_commands {
        print $"  执行: ($command)"
        if not $dry_run {
            let result = (do -i { ^$command })
            if $env.LAST_EXIT_CODE != 0 {
                print $"❌ 命令失败: ($command)"
                return
            }
        }
    }

    # 构建发布版本进行最终验证
    print "🔨 构建发布版本进行验证..."
    if not $dry_run {
        cargo build --release --package tidepool-gvm
        if $env.LAST_EXIT_CODE != 0 {
            print "❌ 发布版本构建失败"
            return
        }
    } else {
        print "  将执行: cargo build --release --package tidepool-gvm"
    }

    # 提交版本更新
    print "📤 提交版本更新..."
    if not $dry_run {
        git add Cargo.toml Cargo.lock
        git commit -m $"chore: bump version to ($version)"
        if $env.LAST_EXIT_CODE != 0 {
            print "❌ 提交失败"
            return
        }
    } else {
        print $"  将执行: git commit -m \"chore: bump version to ($version)\""
    }

    # 创建标签
    print $"🏷️  创建标签: ($tag)"
    if not $dry_run {
        git tag -a $tag -m $"Release ($version)"
        if $env.LAST_EXIT_CODE != 0 {
            print "❌ 创建标签失败"
            return
        }
    } else {
        print $"  将执行: git tag -a ($tag) -m \"Release ($version)\""
    }

    # 推送到远程仓库
    print "🚀 推送到远程仓库..."
    if not $dry_run {
        git push origin main
        git push origin $tag
        if $env.LAST_EXIT_CODE != 0 {
            print "❌ 推送失败"
            return
        }
    } else {
        print "  将执行: git push origin main"
        print $"  将执行: git push origin ($tag)"
    }

    if $dry_run {
        print ""
        print "🎯 预览模式完成 - 以上是将要执行的操作"
        print "要实际执行发布，请运行: ./scripts/release.nu <version>"
    } else {
        print ""
        print "✅ 发布流程完成!"
        print ""
        print $"🎉 版本 ($tag) 已成功发布"
        print ""
        print "📋 后续步骤:"
        print "1. 等待 GitHub Actions 完成构建"
        print "2. 检查 GitHub Releases 页面"
        print "3. 验证二进制文件下载"
        print ""
        print $"🔗 GitHub Actions: https://github.com/Slothtron/tidepool/actions"
        print $"🔗 Releases: https://github.com/Slothtron/tidepool/releases/tag/($tag)"
    }
}
