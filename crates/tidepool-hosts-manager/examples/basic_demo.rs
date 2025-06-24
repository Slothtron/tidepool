//! Hosts 管理基础功能演示
//!
//! 演示 HostEntry 和 HostsManager 的基本使用方法

use std::io::Write;
use std::net::IpAddr;
use tempfile::NamedTempFile;
use tidepool_hosts_manager::{HostEntry, HostsManager};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    env_logger::init();

    println!("🎯 Hosts 管理基础功能演示");

    // 1. 创建 HostEntry 示例
    demo_host_entry()?;

    // 2. 文件读写演示
    demo_file_operations()?;

    // 3. 条目管理演示
    demo_entry_management()?;

    // 4. 统计信息演示
    demo_statistics()?;

    println!("✅ 演示完成");
    Ok(())
}

fn demo_host_entry() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📋 步骤1: HostEntry 创建和操作演示");

    // 创建基本条目
    let mut entry = HostEntry::new("127.0.0.1", &["localhost", "local.test"])?;
    println!("   基本条目: {}", entry);

    // 创建带注释的条目
    let commented_entry = HostEntry::with_comment("192.168.1.1", &["router"], "网关设备")?;
    println!("   带注释条目: {}", commented_entry);

    // 创建被注释的条目
    let disabled_entry = HostEntry::commented("10.0.0.1", &["server"])?;
    println!("   被注释条目: {}", disabled_entry);

    // IPv6 条目
    let ipv6_entry = HostEntry::new("::1", &["localhost6"])?;
    println!("   IPv6 条目: {}", ipv6_entry);

    // 操作主机名
    entry.add_hostname("test.local")?;
    println!("   添加主机名后: {}", entry);

    let removed = entry.remove_hostname("local.test");
    println!("   移除主机名 ({}): {}", removed, entry);

    // 检查条目属性
    println!("   条目信息:");
    println!("     - 是否为 IPv4: {}", entry.is_ipv4());
    println!("     - 是否为回环地址: {}", entry.is_loopback());
    println!("     - 包含 'localhost': {}", entry.contains_hostname("localhost"));

    Ok(())
}

fn demo_file_operations() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📋 步骤2: 文件读写操作演示");

    // 创建临时文件作为测试
    let mut temp_file = NamedTempFile::new()?;
    writeln!(temp_file, "# 测试 hosts 文件")?;
    writeln!(temp_file, "127.0.0.1 localhost")?;
    writeln!(temp_file, "127.0.0.1 local.test # 测试域名")?;
    writeln!(temp_file, "# 192.168.1.1 router")?;
    writeln!(temp_file, "::1 localhost6")?;
    writeln!(temp_file, "")?; // 空行
    temp_file.flush()?;

    // 创建 HostsManager
    let manager = HostsManager::new(temp_file.path()).with_auto_backup(true);

    println!("   创建管理器: {}", manager.hosts_path().display());

    // 读取 hosts 文件
    let entries = manager.read_hosts()?;
    println!("   读取到 {} 条记录:", entries.len());

    for (i, entry) in entries.iter().enumerate() {
        println!("     {}. {}", i + 1, entry);
    }

    // 添加新条目
    let new_entry = HostEntry::new("10.0.0.1", &["test.example.com"])?;
    manager.add_entry(new_entry)?;
    println!("   添加新条目完成");

    // 重新读取验证
    let updated_entries = manager.read_hosts()?;
    println!("   更新后共 {} 条记录", updated_entries.len());

    Ok(())
}

fn demo_entry_management() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📋 步骤3: 条目管理操作演示");

    // 创建测试文件
    let temp_file = create_test_hosts_file()?;
    let manager = HostsManager::new(temp_file.path());

    // 查找操作
    let localhost_entries = manager.find_by_hostname("localhost")?;
    println!("   包含 'localhost' 的记录数: {}", localhost_entries.len());

    let ip_addr: IpAddr = "127.0.0.1".parse()?;
    let ip_entries = manager.find_by_ip(&ip_addr)?;
    println!("   IP 为 '127.0.0.1' 的记录数: {}", ip_entries.len());

    // 移除操作
    let removed_count = manager.remove_hostname("test.local")?;
    println!("   移除包含 'test.local' 的记录数: {}", removed_count);

    // 备份和恢复演示
    if temp_file.path().exists() {
        let backup_path = manager.create_backup()?;
        println!("   创建备份文件: {}", backup_path.display());

        // 添加一些条目后恢复
        manager.add_entry(HostEntry::new("8.8.8.8", &["dns.google"])?)?;
        println!("   添加测试条目");

        manager.restore_from_backup()?;
        println!("   从备份恢复");

        let final_entries = manager.read_hosts()?;
        println!("   恢复后记录数: {}", final_entries.len());
    }

    Ok(())
}

fn demo_statistics() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📋 步骤4: 统计信息演示");

    let temp_file = create_test_hosts_file()?;
    let manager = HostsManager::new(temp_file.path());

    let stats = manager.get_stats()?;
    println!("   文件统计信息:");
    println!("{}", stats);

    // 权限检查
    println!("\n   权限检查:");
    println!("     - 可写入: {}", manager.can_write());
    println!("     - 需要管理员权限: {}", manager.requires_admin());

    Ok(())
}

fn create_test_hosts_file() -> Result<NamedTempFile, std::io::Error> {
    let mut temp_file = NamedTempFile::new()?;
    writeln!(temp_file, "# 测试 hosts 文件")?;
    writeln!(temp_file, "127.0.0.1 localhost")?;
    writeln!(temp_file, "127.0.0.1 test.local # 测试域名")?;
    writeln!(temp_file, "# 192.168.1.1 router")?;
    writeln!(temp_file, "::1 localhost6")?;
    writeln!(temp_file, "8.8.8.8 dns.google")?;
    writeln!(temp_file, "1.1.1.1 one.one.one.one # Cloudflare DNS")?;
    writeln!(temp_file, "")?; // 空行
    temp_file.flush()?;
    Ok(temp_file)
}
