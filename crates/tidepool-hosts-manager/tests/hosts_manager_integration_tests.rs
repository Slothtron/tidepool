//! Hosts 管理器集成测试
//!
//! 测试 HostsManager 的核心功能和错误处理

use std::io::Write;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use tempfile::{NamedTempFile, TempDir};
use tidepool_hosts_manager::{HostEntry, HostsManager, HostsManagerError};

/// 创建测试用的 hosts 文件内容
fn create_test_hosts_content() -> &'static str {
    r#"# 测试 hosts 文件
127.0.0.1 localhost
127.0.0.1 local.test # 测试域名
# 192.168.1.1 router
::1 localhost6
192.168.1.100 dev.example.com
10.0.0.1 server.local admin.local

# 空行测试

8.8.8.8 dns.google
"#
}

/// 创建测试用的 hosts 文件
fn create_test_hosts_file() -> Result<NamedTempFile, std::io::Error> {
    let mut temp_file = NamedTempFile::new()?;
    write!(temp_file, "{}", create_test_hosts_content())?;
    temp_file.flush()?;
    Ok(temp_file)
}

#[test]
fn test_hosts_manager_creation() {
    let temp_file = create_test_hosts_file().unwrap();
    let manager = HostsManager::new(temp_file.path());

    assert_eq!(manager.hosts_path(), temp_file.path());

    // 测试链式配置
    let manager_with_config =
        HostsManager::new(temp_file.path()).with_auto_backup(false).with_backup_extension(".bak");

    assert!(!manager_with_config.is_auto_backup_enabled());
    assert!(manager_with_config.backup_path().to_string_lossy().ends_with(".bak"));
}

#[test]
fn test_read_hosts_file() {
    let temp_file = create_test_hosts_file().unwrap();
    let manager = HostsManager::new(temp_file.path());

    let entries = manager.read_hosts().unwrap();

    // 验证解析的条目数（应该有7条有效记录）
    assert_eq!(entries.len(), 7);

    // 验证第一个条目
    let first_entry = &entries[0];
    assert_eq!(first_entry.ip, IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    assert_eq!(first_entry.hostnames, vec!["localhost"]);
    assert!(!first_entry.is_commented);
    assert!(first_entry.comment.is_none());

    // 验证带注释的条目
    let commented_entry = &entries[1];
    assert_eq!(commented_entry.comment, Some("测试域名".to_string()));

    // 验证被注释的条目
    let disabled_entry = &entries[2];
    assert!(disabled_entry.is_commented);
    assert_eq!(disabled_entry.ip, IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)));

    // 验证 IPv6 条目
    let ipv6_entry = &entries[3];
    assert_eq!(ipv6_entry.ip, IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)));
    assert!(ipv6_entry.is_ipv6());

    // 验证多主机名条目
    let multi_hostname_entry = &entries[5];
    assert_eq!(multi_hostname_entry.hostnames.len(), 2);
    assert!(multi_hostname_entry.contains_hostname("server.local"));
    assert!(multi_hostname_entry.contains_hostname("admin.local"));
}

#[test]
fn test_write_hosts_file() {
    let temp_dir = TempDir::new().unwrap();
    let hosts_path = temp_dir.path().join("test_hosts");
    let manager = HostsManager::new(&hosts_path);

    let entries = vec![
        HostEntry::new("127.0.0.1", &["localhost"]).unwrap(),
        HostEntry::with_comment("192.168.1.1", &["router"], "网关设备").unwrap(),
        HostEntry::commented("10.0.0.1", &["disabled.server"]).unwrap(),
    ];

    // 写入文件
    manager.write_hosts(&entries).unwrap();
    assert!(hosts_path.exists());

    // 读取并验证
    let read_entries = manager.read_hosts().unwrap();
    assert_eq!(read_entries.len(), 3);

    // 验证内容
    assert_eq!(read_entries[0].hostnames, vec!["localhost"]);
    assert_eq!(read_entries[1].comment, Some("网关设备".to_string()));
    assert!(read_entries[2].is_commented);
}

#[test]
fn test_add_entry() {
    let temp_file = create_test_hosts_file().unwrap();
    let manager = HostsManager::new(temp_file.path());

    let original_count = manager.read_hosts().unwrap().len();

    // 添加新条目
    let new_entry = HostEntry::new("203.0.113.1", &["test.example.com"]).unwrap();
    manager.add_entry(new_entry).unwrap();

    let updated_entries = manager.read_hosts().unwrap();
    assert_eq!(updated_entries.len(), original_count + 1);

    // 验证添加的条目
    let added_entry =
        updated_entries.iter().find(|e| e.contains_hostname("test.example.com")).unwrap();
    assert_eq!(added_entry.ip, IpAddr::V4(Ipv4Addr::new(203, 0, 113, 1)));

    // 重复添加相同条目应该不会增加数量
    let duplicate_entry = HostEntry::new("203.0.113.1", &["test.example.com"]).unwrap();
    manager.add_entry(duplicate_entry).unwrap();

    let final_entries = manager.read_hosts().unwrap();
    assert_eq!(final_entries.len(), original_count + 1); // 数量不应该变化
}

#[test]
fn test_remove_hostname() {
    let temp_file = create_test_hosts_file().unwrap();
    let manager = HostsManager::new(temp_file.path());

    // 移除包含 localhost 的条目
    let removed_count = manager.remove_hostname("localhost").unwrap();
    assert!(removed_count > 0);

    // 验证移除结果
    let entries = manager.read_hosts().unwrap();
    let localhost_entries: Vec<_> =
        entries.iter().filter(|e| e.contains_hostname("localhost")).collect();
    assert_eq!(localhost_entries.len(), 0);
}

#[test]
fn test_remove_ip() {
    let temp_file = create_test_hosts_file().unwrap();
    let manager = HostsManager::new(temp_file.path());

    let target_ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let removed_count = manager.remove_ip(&target_ip).unwrap();
    assert!(removed_count > 0);

    // 验证移除结果
    let entries = manager.read_hosts().unwrap();
    let ip_entries: Vec<_> = entries.iter().filter(|e| e.ip == target_ip).collect();
    assert_eq!(ip_entries.len(), 0);
}

#[test]
fn test_find_operations() {
    let temp_file = create_test_hosts_file().unwrap();
    let manager = HostsManager::new(temp_file.path());

    // 按主机名查找
    let localhost_entries = manager.find_by_hostname("localhost").unwrap();
    assert!(localhost_entries.len() > 0);
    assert!(localhost_entries.iter().all(|e| e.contains_hostname("localhost")));

    // 按 IP 查找
    let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let ip_entries = manager.find_by_ip(&ip).unwrap();
    assert!(ip_entries.len() > 0);
    assert!(ip_entries.iter().all(|e| e.ip == ip));

    // 查找不存在的主机名
    let nonexistent = manager.find_by_hostname("nonexistent.domain").unwrap();
    assert_eq!(nonexistent.len(), 0);
}

#[test]
fn test_backup_and_restore() {
    let temp_dir = TempDir::new().unwrap();
    let hosts_path = temp_dir.path().join("hosts");
    let manager = HostsManager::new(&hosts_path);

    // 创建初始文件
    let initial_entries = vec![HostEntry::new("127.0.0.1", &["localhost"]).unwrap()];
    manager.write_hosts(&initial_entries).unwrap();

    // 创建备份
    let backup_path = manager.create_backup().unwrap();
    assert!(backup_path.exists());
    assert_ne!(backup_path, hosts_path);

    // 修改文件
    let modified_entries = vec![HostEntry::new("192.168.1.1", &["router"]).unwrap()];
    manager.write_hosts(&modified_entries).unwrap();

    // 验证文件已修改
    let current_entries = manager.read_hosts().unwrap();
    assert_eq!(current_entries.len(), 1);
    assert!(current_entries[0].contains_hostname("router"));

    // 从备份恢复
    manager.restore_from_backup().unwrap();

    // 验证恢复结果
    let restored_entries = manager.read_hosts().unwrap();
    assert_eq!(restored_entries.len(), 1);
    assert!(restored_entries[0].contains_hostname("localhost"));
}

#[test]
fn test_statistics() {
    let temp_file = create_test_hosts_file().unwrap();
    let manager = HostsManager::new(temp_file.path());

    let stats = manager.get_stats().unwrap();

    // 验证统计信息
    assert_eq!(stats.total_entries, 7);
    assert_eq!(stats.ipv4_count, 6);
    assert_eq!(stats.ipv6_count, 1);
    assert_eq!(stats.commented_count, 1);
    assert_eq!(stats.with_comments, 1);
    assert!(stats.unique_hostnames > 0);

    // 测试统计信息的显示格式
    let stats_display = format!("{}", stats);
    assert!(stats_display.contains("总条目数"));
    assert!(stats_display.contains("IPv4 条目"));
    assert!(stats_display.contains("IPv6 条目"));
}

#[test]
fn test_error_handling() {
    // 测试文件不存在
    let manager = HostsManager::new("/nonexistent/path/hosts");
    let result = manager.read_hosts();
    assert!(matches!(result, Err(HostsManagerError::FileNotFound(_))));

    // 测试空条目写入
    let temp_dir = TempDir::new().unwrap();
    let hosts_path = temp_dir.path().join("empty_hosts");
    let manager = HostsManager::new(&hosts_path);

    manager.write_hosts(&[]).unwrap();
    let entries = manager.read_hosts().unwrap();
    assert_eq!(entries.len(), 0);
}

#[test]
fn test_permissions() {
    let temp_file = create_test_hosts_file().unwrap();
    let manager = HostsManager::new(temp_file.path());

    // 在测试环境中，临时文件应该是可写的
    assert!(manager.can_write());

    // 测试临时文件不需要特殊权限
    assert!(!manager.requires_admin());
}

#[test]
fn test_atomic_write() {
    let temp_dir = TempDir::new().unwrap();
    let hosts_path = temp_dir.path().join("atomic_test");
    let manager = HostsManager::new(&hosts_path);

    let entries = vec![
        HostEntry::new("127.0.0.1", &["test1"]).unwrap(),
        HostEntry::new("127.0.0.2", &["test2"]).unwrap(),
    ];

    // 写入操作应该是原子性的
    manager.write_hosts(&entries).unwrap();

    // 验证临时文件不存在
    let temp_path = hosts_path.with_extension("tmp");
    assert!(!temp_path.exists());

    // 验证实际文件存在且内容正确
    assert!(hosts_path.exists());
    let read_entries = manager.read_hosts().unwrap();
    assert_eq!(read_entries.len(), 2);
}

#[test]
fn test_auto_backup_disabled() {
    let temp_dir = TempDir::new().unwrap();
    let hosts_path = temp_dir.path().join("no_backup_hosts");
    let manager = HostsManager::new(&hosts_path).with_auto_backup(false);

    // 写入初始内容
    let entries = vec![HostEntry::new("127.0.0.1", &["localhost"]).unwrap()];
    manager.write_hosts(&entries).unwrap();

    // 再次写入（不应该创建备份）
    let new_entries = vec![HostEntry::new("192.168.1.1", &["router"]).unwrap()];
    manager.write_hosts(&new_entries).unwrap();

    // 验证没有自动创建备份文件
    let backup_path = manager.backup_path();
    assert!(!backup_path.exists());
}
