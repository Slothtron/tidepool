#!/usr/bin/env nu

# Tidepool 项目测试运行脚本
# 用于运行项目中的所有测试和示例

def main [command?: string] {
    match $command {
        "all" => run_all_tests,
        "unit" => run_unit_tests,
        "integration" => run_integration_tests,
        "clean" => clean_test_artifacts,
        _ => show_help
    }
}

# 运行所有测试
def run_all_tests [] {
    print "🧪 运行所有测试..."
    cargo test --workspace
    print "✅ 所有测试完成"
}

# 运行单元测试
def run_unit_tests [] {
    print "🔧 运行单元测试..."

    print "  📦 版本管理器单元测试..."
    cargo test --package tidepool-version-manager

    print "  🖥️  CLI 单元测试..."
    cargo test --package gvm

    print "✅ 单元测试完成"
}

# 运行集成测试
def run_integration_tests [] {
    print "🔗 运行集成测试..."

    # 运行根目录的集成测试（测试包之间的协作）
    print "  🌐 运行系统集成测试..."
    cargo test --test "*"

    # 运行各包的集成测试
    print "  📦 运行版本管理器集成测试..."
    cargo test --package tidepool-version-manager --tests

    print "  🖥️  运行 CLI 集成测试..."
    cargo test --package gvm --tests

    print "✅ 集成测试完成"
}

# 清理测试产生的文件
def clean_test_artifacts [] {
    print "🧹 清理测试文件..."
    cargo clean
    print "✅ 清理完成"
}

# 显示帮助信息
def show_help [] {
    print "🔬 Tidepool 测试运行器"
    print ""
    print "用法:"
    print "  nu run_tests.nu [命令]"
    print ""
    print "命令:"
    print "  all         - 运行所有测试"
    print "  unit        - 只运行单元测试"
    print "  integration - 只运行集成测试"
    print "  clean       - 清理测试文件"
    print ""
    print "示例:"
    print "  nu run_tests.nu all       # 运行所有测试"
    print "  nu run_tests.nu unit      # 只运行单元测试"
}
