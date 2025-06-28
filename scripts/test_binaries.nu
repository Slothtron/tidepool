# test_binaries.nu - 测试所有构建的二进制文件

# 设置 Podman 作为容器引擎
$env.CROSS_CONTAINER_ENGINE = 'podman'

# 定义测试容器映射
let test_containers = {
    "x86_64-unknown-linux-gnu": "ubuntu:latest"
    "x86_64-unknown-linux-musl": "alpine:latest"
    "aarch64-unknown-linux-gnu": "arm64v8/ubuntu:latest"
    "armv7-unknown-linux-gnueabihf": "arm32v7/ubuntu:latest"
}

# 测试单个二进制文件
def test_binary [target: string] {
    let binary_path = $"target/($target)/release/gvm"
    
    if not ($binary_path | path exists) {
        print $"❌ 二进制文件不存在: ($binary_path)"
        return
    }
    
    print $"🧪 测试 ($target)..."
    
    let container = ($test_containers | get $target)
    let workspace_path = (pwd | str replace '\' '/')
    
    try {
        # 测试版本信息
        let result = (podman run --rm -v $"($workspace_path):/workspace" $container /workspace/($binary_path) --version)
        print $"  ✅ 版本信息: ($result | lines | first)"
        
        # 测试帮助信息
        let help_result = (podman run --rm -v $"($workspace_path):/workspace" $container /workspace/($binary_path) --help | lines | first)
        print $"  ✅ 帮助信息可用"
        
        print $"✅ ($target) 测试通过"
    } catch {
        print $"❌ ($target) 测试失败"
    }
}

# 测试所有二进制文件
def test_all [] {
    print "🚀 开始测试所有二进制文件..."
    
    for target in ($test_containers | columns) {
        test_binary $target
        print ""
    }
    
    print "📊 测试总结:"
    ls target/*/release/gvm | select name size | each { |row|
        let target = ($row.name | str replace 'target\\' '' | str replace '\\release\\gvm' '')
        { target: $target, size: $row.size }
    }
}

# 清理测试容器镜像
def clean_images [] {
    print "🧹 清理测试容器镜像..."
    
    for container in ($test_containers | values) {
        try {
            podman rmi $container
            print $"✅ 已清理: ($container)"
        } catch {
            print $"⚠️ 清理失败或镜像不存在: ($container)"
        }
    }
}

# 只测试 musl 版本（最兼容）
def test_musl [] {
    test_binary "x86_64-unknown-linux-musl"
}

# 检查文件信息
def check_files [] {
    print "📁 检查构建文件..."
    
    ls target/*/release/gvm | each { |row|
        let target = ($row.name | str replace 'target\\' '' | str replace '\\release\\gvm' '')
        print $"($target): ($row.size)"
    }
}

# 主函数
def main [command?: string] {
    match $command {
        "all" => { test_all }
        "musl" => { test_musl }
        "clean" => { clean_images }
        "files" => { check_files }
        _ => {
            print "使用方法:"
            print "  nu scripts/test_binaries.nu all    # 测试所有二进制文件"
            print "  nu scripts/test_binaries.nu musl   # 只测试 musl 版本"
            print "  nu scripts/test_binaries.nu files  # 检查文件信息"
            print "  nu scripts/test_binaries.nu clean  # 清理容器镜像"
        }
    }
}
