# build_cross.nu - 使用 Cross 和 Podman 进行跨平台构建

# 设置 Podman 作为容器引擎
$env.CROSS_CONTAINER_ENGINE = 'podman'

# 支持的目标平台
let targets = [
    "x86_64-unknown-linux-gnu"
    "x86_64-unknown-linux-musl" 
    "aarch64-unknown-linux-gnu"
    "armv7-unknown-linux-gnueabihf"
    # 注意: macOS 目标需要特殊的交叉编译工具链，在 Windows 上暂不支持
    # "x86_64-apple-darwin"
    # "aarch64-apple-darwin"
]

# 构建函数
def build_target [target: string] {
    print $"🔨 构建目标: ($target)"
    
    try {
        cross build --target $target --release -p gvm
        print $"✅ ($target) 构建成功"
    } catch {
        print $"❌ ($target) 构建失败"
    }
}

# 构建所有目标
def build_all [] {
    print "🚀 开始跨平台构建..."
    
    for target in $targets {
        build_target $target
    }
    
    print "📦 构建完成！查看结果:"
    ls target/*/release/gvm | select name size
}

# 清理构建目录
def clean [] {
    print "🧹 清理构建目录..."
    cargo clean
    print "✅ 清理完成"
}

# 验证二进制文件 (使用 Alpine 容器测试 musl 版本)
def test_musl [] {
    print "🧪 测试 musl 二进制文件..."
    
    try {
        let workspace_path = (pwd | str replace '\' '/' | str replace 'D:' '/mnt/d')
        podman run --rm -v $"(pwd):/workspace" alpine:latest /workspace/target/x86_64-unknown-linux-musl/release/gvm --version
        print "✅ musl 二进制文件测试通过"
    } catch {
        print "❌ musl 二进制文件测试失败"
    }
}

# 主函数
def main [command?: string] {
    match $command {
        "all" => { build_all }
        "clean" => { clean }
        "test" => { test_musl }
        _ => {
            print "使用方法:"
            print "  nu scripts/build_cross.nu all    # 构建所有目标"
            print "  nu scripts/build_cross.nu clean  # 清理构建目录"
            print "  nu scripts/build_cross.nu test   # 测试 musl 二进制文件"
        }
    }
}
