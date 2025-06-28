# code_quality.nu - 跨平台代码质量检查和格式化

# 检查当前平台
def get_platform [] {
    if ($nu.os-info.name == "windows") {
        "windows"
    } else {
        "unix"
    }
}

# 执行代码格式化
def format_code [] {
    print "🎨 格式化代码..."
    
    try {
        cargo fmt --all
        print "✅ 代码格式化完成"
        true
    } catch {
        print "❌ 代码格式化失败"
        false
    }
}

# 检查代码格式
def check_format [] {
    print "🔍 检查代码格式..."
    
    try {
        cargo fmt --all -- --check
        print "✅ 代码格式检查通过"
        true
    } catch {
        print "❌ 代码格式检查失败 - 需要运行 cargo fmt"
        false
    }
}

# 运行 Clippy 静态检查
def run_clippy [] {
    print "🔧 运行 Clippy 静态检查..."
    
    try {
        cargo clippy --all-targets --all-features -- -D warnings
        print "✅ Clippy 检查通过"
        true
    } catch {
        print "❌ Clippy 检查失败 - 存在警告或错误"
        false
    }
}

# 运行测试
def run_tests [] {
    print "🧪 运行测试..."
    
    try {
        cargo test --all
        print "✅ 测试通过"
        true
    } catch {
        print "❌ 测试失败"
        false
    }
}

# 检查依赖安全性
def check_security [] {
    print "🔒 检查依赖安全性..."
    
    # 检查是否安装了 cargo-audit
    try {
        cargo audit --version | ignore
    } catch {
        print "⚠️ cargo-audit 未安装，跳过安全检查"
        print "💡 可运行: cargo install cargo-audit"
        return true
    }
    
    try {
        cargo audit
        print "✅ 依赖安全检查通过"
        true
    } catch {
        print "❌ 发现安全漏洞"
        false
    }
}

# 检查代码覆盖率
def check_coverage [] {
    print "📊 检查代码覆盖率..."
    
    # 检查是否安装了 tarpaulin (仅在 Unix 系统上)
    let platform = (get_platform)
    if $platform == "windows" {
        print "⚠️ Windows 系统暂不支持 tarpaulin，跳过覆盖率检查"
        return true
    }
    
    try {
        cargo tarpaulin --version | ignore
    } catch {
        print "⚠️ cargo-tarpaulin 未安装，跳过覆盖率检查"
        print "💡 可运行: cargo install cargo-tarpaulin"
        return true
    }
    
    try {
        cargo tarpaulin --out Html --output-dir coverage
        print "✅ 代码覆盖率检查完成，报告生成在 coverage/ 目录"
        true
    } catch {
        print "❌ 代码覆盖率检查失败"
        false
    }
}

# 检查文档
def check_docs [] {
    print "📚 检查文档..."
    
    try {
        cargo doc --all --no-deps
        print "✅ 文档生成成功"
        true
    } catch {
        print "❌ 文档生成失败"
        false
    }
}

# 验证构建
def verify_build [] {
    print "🔨 验证构建..."
    
    try {
        cargo build --all --release
        print "✅ Release 构建成功"
        true
    } catch {
        print "❌ Release 构建失败"
        false
    }
}

# 检查 Cargo.toml 格式
def check_cargo_toml [] {
    print "📋 检查 Cargo.toml 格式..."
    
    # 检查工作空间配置
    let workspace_files = [
        "Cargo.toml"
        "cli/gvm/Cargo.toml"
        "crates/tidepool-version-manager/Cargo.toml"
    ]
    
    let results = $workspace_files | each { |file|
        if ($file | path exists) {
            try {
                # 使用 cargo check 验证整个项目配置
                let result = (cargo check --manifest-path $file --quiet | complete)
                if ($result.exit_code == 0) {
                    print $"  ✅ ($file) 格式正确"
                    true
                } else {
                    print $"  ❌ ($file) 格式错误"
                    false
                }
            } catch {
                # 如果 cargo check 失败，可能是依赖问题，仅检查 TOML 语法
                try {
                    # 简单的语法检查：确保文件可以被读取且包含基本的 TOML 结构
                    let content = (open $file | into string)
                    if ($content | str contains "[package]" or $content | str contains "[workspace]") {
                        print $"  ✅ ($file) 基本格式正确"
                        true
                    } else {
                        print $"  ❌ ($file) 缺少基本 TOML 结构"
                        false
                    }
                } catch {
                    print $"  ❌ ($file) 文件读取失败"
                    false
                }
            }
        } else {
            print $"  ⚠️ ($file) 文件不存在"
            false
        }
    }
    
    let all_valid = ($results | all { |x| $x })
    
    if $all_valid {
        print "✅ 所有 Cargo.toml 文件格式正确"
    } else {
        print "❌ 部分 Cargo.toml 文件格式有问题"
    }
    
    $all_valid
}

# 完整的代码质量检查
def full_check [] {
    print "🚀 开始完整代码质量检查..."
    print ""
    
    let platform = (get_platform)
    print $"🖥️ 当前平台: ($platform)"
    print ""
    
    # 执行所有检查
    let checks = [
        { name: "Cargo.toml 格式", func: {|| check_cargo_toml } }
        { name: "代码格式检查", func: {|| check_format } }
        { name: "Clippy 静态检查", func: {|| run_clippy } }
        { name: "测试运行", func: {|| run_tests } }
        { name: "构建验证", func: {|| verify_build } }
        { name: "文档生成", func: {|| check_docs } }
        { name: "依赖安全检查", func: {|| check_security } }
        { name: "代码覆盖率", func: {|| check_coverage } }
    ]
    
    let results = $checks | each { |check|
        let result = (do $check.func)
        print ""
        { name: $check.name, passed: $result }
    }
    
    # 显示总结
    print "📊 检查结果总结:"
    print "=" * 50
    
    let passed_count = ($results | where passed == true | length)
    for result in $results {
        let status = if $result.passed { "✅ 通过" } else { "❌ 失败" }
        print $"($result.name): ($status)"
    }
    
    print ""
    print $"总计: ($passed_count)/($results | length) 项检查通过"
    
    if $passed_count == ($results | length) {
        print "🎉 所有代码质量检查通过！"
        true
    } else {
        print "⚠️ 部分检查未通过，请修复问题后重新检查"
        false
    }
}

# 快速检查（仅基本项目）
def quick_check [] {
    print "⚡ 快速代码质量检查..."
    print ""
    
    mut all_passed = true
    
    # 基本检查项
    if not (check_format) { $all_passed = false }
    print ""
    if not (run_clippy) { $all_passed = false }
    print ""
    if not (run_tests) { $all_passed = false }
    
    print ""
    if $all_passed {
        print "✅ 快速检查通过！"
    } else {
        print "❌ 快速检查失败"
    }
    
    $all_passed
}

# 修复代码格式和常见问题
def fix_issues [] {
    print "🔧 修复代码格式和常见问题..."
    print ""
    
    # 格式化代码
    format_code
    print ""
    
    # 尝试自动修复 Clippy 建议
    print "🔧 尝试自动修复 Clippy 建议..."
    try {
        cargo clippy --all-targets --all-features --fix --allow-dirty --allow-staged
        print "✅ Clippy 自动修复完成"
    } catch {
        print "⚠️ Clippy 自动修复失败，可能需要手动修复"
    }
    
    print ""
    print "🔍 重新运行快速检查..."
    quick_check
}

# 安装开发工具
def install_tools [] {
    print "🛠️ 安装代码质量检查工具..."
    
    let tools = [
        { name: "rustfmt", command: "rustup component add rustfmt" }
        { name: "clippy", command: "rustup component add clippy" }
        { name: "cargo-audit", command: "cargo install cargo-audit" }
    ]
    
    let platform = (get_platform)
    let tools_with_tarpaulin = if $platform == "unix" {
        $tools | append { name: "cargo-tarpaulin", command: "cargo install cargo-tarpaulin" }
    } else {
        $tools
    }
    
    for tool in $tools_with_tarpaulin {
        print $"📥 安装 ($tool.name)..."
        try {
            nu -c $tool.command
            print $"  ✅ ($tool.name) 安装成功"
        } catch {
            print $"  ❌ ($tool.name) 安装失败"
        }
    }
}

# 显示代码质量指标
def show_metrics [] {
    print "📈 代码质量指标:"
    print "=" * 40
    
    # 代码行数统计
    try {
        let rust_files = (ls **/*.rs | length)
        let total_lines = (ls **/*.rs | each { |file| open $file.name | lines | length } | math sum)
        print $"📄 Rust 文件数量: ($rust_files)"
        print $"📏 总代码行数: ($total_lines)"
    } catch {
        print "⚠️ 无法统计代码行数"
    }
    
    print ""
    
    # 依赖数量
    try {
        let deps = (open Cargo.toml | get workspace.dependencies | columns | length)
        print $"📦 工作空间依赖数量: ($deps)"
    } catch {
        print "⚠️ 无法读取依赖信息"
    }
    
    print ""
    
    # 测试文件统计
    try {
        let test_files = (ls **/tests/*.rs | length)
        let test_functions = (ls **/tests/*.rs | each { |file| 
            open $file.name | lines | where ($it | str contains "#[test]") | length 
        } | math sum)
        print $"🧪 测试文件数量: ($test_files)"
        print $"🔬 测试函数数量: ($test_functions)"
    } catch {
        print "⚠️ 无法统计测试信息"
    }
}

# 主函数
def main [command?: string] {
    match $command {
        "full" => { full_check }
        "quick" => { quick_check }
        "format" => { format_code }
        "check-format" => { check_format }
        "clippy" => { run_clippy }
        "test" => { run_tests }
        "fix" => { fix_issues }
        "tools" => { install_tools }
        "metrics" => { show_metrics }
        "security" => { check_security }
        "coverage" => { check_coverage }
        "docs" => { check_docs }
        _ => {
            print "🔍 代码质量检查工具"
            print "=" * 40
            print "使用方法:"
            print "  nu scripts/code_quality.nu full         # 完整代码质量检查"
            print "  nu scripts/code_quality.nu quick        # 快速检查（格式、Clippy、测试）"
            print "  nu scripts/code_quality.nu fix          # 自动修复格式和常见问题"
            print "  nu scripts/code_quality.nu format       # 格式化代码"
            print "  nu scripts/code_quality.nu check-format # 检查代码格式"
            print "  nu scripts/code_quality.nu clippy       # 运行 Clippy"
            print "  nu scripts/code_quality.nu test         # 运行测试"
            print "  nu scripts/code_quality.nu security     # 安全检查"
            print "  nu scripts/code_quality.nu coverage     # 代码覆盖率（Unix 系统）"
            print "  nu scripts/code_quality.nu docs         # 生成文档"
            print "  nu scripts/code_quality.nu tools        # 安装开发工具"
            print "  nu scripts/code_quality.nu metrics      # 显示代码质量指标"
            print ""
            print "💡 推荐工作流程:"
            print "  1. nu scripts/code_quality.nu tools     # 首次安装工具"
            print "  2. nu scripts/code_quality.nu fix       # 修复常见问题"
            print "  3. nu scripts/code_quality.nu quick     # 快速检查"
            print "  4. nu scripts/code_quality.nu full      # 完整检查"
        }
    }
}
