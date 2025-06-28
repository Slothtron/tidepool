/// 演示 Go 版本安装的 SHA256 校验功能
///
/// 本示例展示了新增的文件完整性校验机制，确保下载的 Go 安装包
/// 与官方发布的版本完全一致，防止恶意篡改或传输错误。
use std::fs;
use tempfile::TempDir;
use tidepool_version_manager::go::GoManager;

#[tokio::main]
async fn main() {
    println!("🔐 Go 安装包 SHA256 校验演示");
    println!("========================================");

    demonstrate_hash_calculation().await;
    demonstrate_verification_process().await;
    demonstrate_security_benefits().await;

    println!("\n✅ 演示完成！");
}

async fn demonstrate_hash_calculation() {
    println!("📋 SHA256 哈希计算演示");
    println!("----------------------------------------");

    let temp_dir = TempDir::new().expect("无法创建临时目录");
    let base_dir = temp_dir.path().to_path_buf();
    let manager = GoManager::new();

    // 创建不同大小的模拟文件
    let test_cases = vec![
        ("small_package.zip", "小型测试包", 50),
        ("medium_package.tar.gz", "中型测试包", 500),
        ("large_package.tar.gz", "大型测试包", 5000),
    ];

    for (filename, description, size_kb) in test_cases {
        let file_path = base_dir.join(filename);
        let content = "X".repeat(size_kb * 1024); // 生成指定大小的内容
        fs::write(&file_path, &content).expect("无法创建测试文件");

        println!("🔸 计算 {description} ({filename}) 的哈希值...");

        let start_time = std::time::Instant::now();
        match manager.calculate_file_hash(&file_path).await {
            Ok(hash) => {
                let duration = start_time.elapsed();
                println!("  📊 文件大小: {size_kb} KB");
                println!("  🔑 SHA256: {}...{}", &hash[..16], &hash[hash.len() - 16..]);
                println!("  ⏱️  计算耗时: {duration:?}");
            }
            Err(e) => {
                println!("  ❌ 计算失败: {e}");
            }
        }
        println!();
    }
}

async fn demonstrate_verification_process() {
    println!("🔍 文件完整性校验流程演示");
    println!("----------------------------------------");

    let temp_dir = TempDir::new().expect("无法创建临时目录");
    let base_dir = temp_dir.path().to_path_buf();
    let manager = GoManager::new();

    // 模拟场景1：完整文件
    println!("🔸 场景1: 完整文件校验");
    let intact_file = base_dir.join("go1.21.0.linux-amd64.tar.gz");
    let original_content = "This is the original Go 1.21.0 Linux AMD64 package content";
    fs::write(&intact_file, original_content).expect("无法创建完整文件");

    let original_hash = manager.calculate_file_hash(&intact_file).await.unwrap();
    println!(
        "  ✅ 原始文件哈希: {}...{}",
        &original_hash[..16],
        &original_hash[original_hash.len() - 16..]
    );

    // 模拟场景2：损坏文件
    println!("\n🔸 场景2: 损坏文件检测");
    let corrupted_file = base_dir.join("go1.21.0.linux-amd64.tar.gz.corrupted");
    let corrupted_content = "This content has been modified or corrupted during download";
    fs::write(&corrupted_file, corrupted_content).expect("无法创建损坏文件");

    let corrupted_hash = manager.calculate_file_hash(&corrupted_file).await.unwrap();
    println!(
        "  ❌ 损坏文件哈希: {}...{}",
        &corrupted_hash[..16],
        &corrupted_hash[corrupted_hash.len() - 16..]
    );

    if original_hash != corrupted_hash {
        println!("  🛡️  检测到文件已被修改！");
        println!("  💡 系统会自动拒绝使用损坏的文件");
    }

    // 模拟场景3：部分下载
    println!("\n🔸 场景3: 部分下载检测");
    let partial_file = base_dir.join("go1.21.0.linux-amd64.tar.gz.partial");
    let partial_content = &original_content[..original_content.len() / 2]; // 只有一半内容
    fs::write(&partial_file, partial_content).expect("无法创建部分文件");

    let partial_hash = manager.calculate_file_hash(&partial_file).await.unwrap();
    println!(
        "  ⚠️  部分文件哈希: {}...{}",
        &partial_hash[..16],
        &partial_hash[partial_hash.len() - 16..]
    );

    if original_hash != partial_hash {
        println!("  🛡️  检测到文件不完整！");
        println!("  💡 系统会自动重新下载完整文件");
    }
}

async fn demonstrate_security_benefits() {
    println!("🛡️  安全性和可靠性改进");
    println!("----------------------------------------");

    println!("🔹 安全性保障:");
    println!("  ✓ 防止恶意篡改: 确保下载的文件与官方发布版本完全一致");
    println!("  ✓ 检测传输错误: 网络传输中的数据损坏会被立即发现");
    println!("  ✓ 验证文件完整性: 部分下载或中断的文件不会被误用");
    println!("  ✓ 供应链安全: 防止中间人攻击和文件替换");

    println!("\n🔹 用户体验改进:");
    println!("  ✓ 自动校验: 无需用户手动验证，系统自动完成");
    println!("  ✓ 失败恢复: 校验失败时自动清理并重新下载");
    println!("  ✓ 缓存验证: 即使是缓存文件也会进行完整性检查");
    println!("  ✓ 透明过程: 用户可以看到校验进度和结果");

    println!("\n🔹 技术实现:");
    println!("  • 算法: SHA256 (FIPS 140-2 认证的安全哈希算法)");
    println!("  • 数据源: Go 官方发布的校验和 (https://go.dev/dl/)");
    println!("  • 校验时机: 下载完成后立即校验，解压前验证");
    println!("  • 错误处理: 校验失败时自动清理损坏文件");

    println!("\n🔹 性能影响:");
    println!("  • 哈希计算: 使用高效的流式算法，内存占用低");
    println!("  • 网络开销: 仅在首次下载版本信息时需要额外请求");
    println!("  • 存储开销: 无额外存储需求，校验和在线获取");
    println!("  • 时间开销: 通常在秒级完成，相比下载时间可忽略");

    println!("\n🔹 实际效果演示:");

    let temp_dir = TempDir::new().expect("无法创建临时目录");
    let base_dir = temp_dir.path().to_path_buf();
    let manager = GoManager::new();

    // 演示不同大小文件的校验性能
    let sizes = vec![(100, "100KB 小包"), (1000, "1MB 中包"), (10000, "10MB 大包")];

    for (size_kb, description) in sizes {
        let test_file = base_dir.join(format!("test_{size_kb}.bin"));
        let content = vec![0u8; size_kb * 1024];
        fs::write(&test_file, &content).expect("无法创建测试文件");

        let start = std::time::Instant::now();
        let _ = manager.calculate_file_hash(&test_file).await;
        let duration = start.elapsed();

        println!("  • {description} 校验耗时: {duration:?}");
    }

    println!("\n🏆 总结:");
    println!("  SHA256 校验机制为 gvm 提供了企业级的安全保障，");
    println!("  确保每个安装的 Go 版本都是官方认证的正版文件，");
    println!("  大大提升了整个开发环境的安全性和可靠性。");
}
