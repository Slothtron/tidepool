//! 分组管理器集成测试
//!
//! 测试 GroupManager 的分组管理功能

use std::fs;
use tempfile::TempDir;
use tidepool_hosts_manager::{GroupError, GroupManager, HostEntry};

#[test]
fn test_group_manager_creation() {
    let temp_dir = TempDir::new().unwrap();
    let manager = GroupManager::new(temp_dir.path()).unwrap();

    // 验证配置文件和目录结构已创建
    assert!(temp_dir.path().join("groups.toml").exists());
    assert!(temp_dir.path().join("groups").exists());

    // 初始状态应该没有分组
    assert_eq!(manager.list_groups().len(), 0);
    assert!(manager.get_active_group().is_none());
}

#[test]
fn test_add_and_remove_groups() {
    let temp_dir = TempDir::new().unwrap();
    let mut manager = GroupManager::new(temp_dir.path()).unwrap();

    // 添加分组
    let dev_path = manager.add_group("dev", Some("开发环境")).unwrap();
    assert!(dev_path.exists());
    assert!(dev_path.to_string_lossy().contains("dev.hosts"));

    let prod_path = manager.add_group("prod", Some("生产环境")).unwrap();
    assert!(prod_path.exists());

    let test_path = manager.add_group("test", None).unwrap();
    assert!(test_path.exists());

    // 验证分组列表
    let groups = manager.list_groups();
    assert_eq!(groups.len(), 3);

    let group_names: Vec<&str> = groups.iter().map(|g| g.name.as_str()).collect();
    assert!(group_names.contains(&"dev"));
    assert!(group_names.contains(&"prod"));
    assert!(group_names.contains(&"test"));

    // 验证分组属性
    let dev_group = manager.get_group("dev").unwrap();
    assert_eq!(dev_group.description, Some("开发环境".to_string()));
    assert!(dev_group.enabled);

    let test_group = manager.get_group("test").unwrap();
    assert_eq!(test_group.description, None);

    // 移除分组
    manager.remove_group("test").unwrap();
    assert_eq!(manager.list_groups().len(), 2);
    assert!(manager.get_group("test").is_none());
    assert!(!test_path.exists());
}

#[test]
fn test_duplicate_group_names() {
    let temp_dir = TempDir::new().unwrap();
    let mut manager = GroupManager::new(temp_dir.path()).unwrap();

    // 添加第一个分组
    manager.add_group("duplicate", None).unwrap();

    // 尝试添加重复名称的分组应该失败
    let result = manager.add_group("duplicate", None);
    assert!(matches!(result, Err(GroupError::GroupAlreadyExists(_))));
}

#[test]
fn test_group_switching() {
    let temp_dir = TempDir::new().unwrap();
    let mut manager = GroupManager::new(temp_dir.path()).unwrap();

    // 创建测试分组
    let web_path = manager.add_group("web", Some("Web 开发")).unwrap();
    let api_path = manager.add_group("api", Some("API 开发")).unwrap();

    // 为分组添加不同的内容
    let web_entries = vec![
        HostEntry::new("127.0.0.1", &["web.local"]).unwrap(),
        HostEntry::new("192.168.1.100", &["web.dev"]).unwrap(),
    ];

    let api_entries = vec![
        HostEntry::new("127.0.0.1", &["api.local"]).unwrap(),
        HostEntry::new("192.168.1.200", &["api.dev"]).unwrap(),
    ];

    // 写入分组内容
    fs::write(&web_path, format_entries(&web_entries)).unwrap();
    fs::write(&api_path, format_entries(&api_entries)).unwrap();

    // 切换到 web 分组
    manager.switch_group("web").unwrap();
    assert_eq!(manager.get_active_group().unwrap().name, "web");

    // 切换到 api 分组
    manager.switch_group("api").unwrap();
    assert_eq!(manager.get_active_group().unwrap().name, "api");

    // 尝试切换到不存在的分组应该失败
    let result = manager.switch_group("nonexistent");
    assert!(matches!(result, Err(GroupError::GroupNotFound(_))));
}

#[test]
fn test_group_enable_disable() {
    let temp_dir = TempDir::new().unwrap();
    let mut manager = GroupManager::new(temp_dir.path()).unwrap();

    manager.add_group("test", None).unwrap();

    // 默认应该启用
    assert!(manager.get_group("test").unwrap().enabled);

    // 禁用分组
    manager.set_group_enabled("test", false).unwrap();
    assert!(!manager.get_group("test").unwrap().enabled);

    // 尝试切换到禁用的分组应该失败
    let result = manager.switch_group("test");
    assert!(matches!(result, Err(GroupError::ConfigError(_))));

    // 重新启用
    manager.set_group_enabled("test", true).unwrap();
    assert!(manager.get_group("test").unwrap().enabled);

    // 现在应该可以切换
    manager.switch_group("test").unwrap();
    assert_eq!(manager.get_active_group().unwrap().name, "test");
}

#[test]
fn test_group_proxy_port() {
    let temp_dir = TempDir::new().unwrap();
    let mut manager = GroupManager::new(temp_dir.path()).unwrap();

    manager.add_group("proxy_test", None).unwrap();

    // 默认没有代理端口
    assert_eq!(manager.get_group("proxy_test").unwrap().proxy_port, None);

    // 设置代理端口
    manager.set_group_proxy_port("proxy_test", Some(8080)).unwrap();
    assert_eq!(manager.get_group("proxy_test").unwrap().proxy_port, Some(8080));

    // 清除代理端口
    manager.set_group_proxy_port("proxy_test", None).unwrap();
    assert_eq!(manager.get_group("proxy_test").unwrap().proxy_port, None);

    // 对不存在的分组设置端口应该失败
    let result = manager.set_group_proxy_port("nonexistent", Some(9090));
    assert!(matches!(result, Err(GroupError::GroupNotFound(_))));
}

#[test]
fn test_copy_group() {
    let temp_dir = TempDir::new().unwrap();
    let mut manager = GroupManager::new(temp_dir.path()).unwrap();

    // 创建源分组
    let source_path = manager.add_group("source", Some("源分组")).unwrap();
    let entries = vec![
        HostEntry::new("127.0.0.1", &["localhost"]).unwrap(),
        HostEntry::new("192.168.1.1", &["router"]).unwrap(),
    ];
    fs::write(&source_path, format_entries(&entries)).unwrap();

    // 复制分组
    manager.copy_group("source", "target", Some("复制的分组")).unwrap();

    // 验证目标分组存在
    assert!(manager.get_group("target").is_some());
    let target_group = manager.get_group("target").unwrap();
    assert_eq!(target_group.description, Some("复制的分组".to_string()));

    // 验证内容相同
    let source_manager = manager.get_group_manager("source").unwrap();
    let target_manager = manager.get_group_manager("target").unwrap();

    let source_entries = source_manager.read_hosts().unwrap();
    let target_entries = target_manager.read_hosts().unwrap();

    assert_eq!(source_entries.len(), target_entries.len());
    assert_eq!(source_entries.len(), 2);

    // 尝试复制到已存在的分组应该失败
    let result = manager.copy_group("source", "target", None);
    assert!(matches!(result, Err(GroupError::GroupAlreadyExists(_))));

    // 从不存在的源分组复制应该失败
    let result = manager.copy_group("nonexistent", "new_target", None);
    assert!(matches!(result, Err(GroupError::GroupNotFound(_))));
}

#[test]
fn test_merge_groups() {
    let temp_dir = TempDir::new().unwrap();
    let mut manager = GroupManager::new(temp_dir.path()).unwrap();

    // 创建多个源分组
    let group1_path = manager.add_group("group1", None).unwrap();
    let group2_path = manager.add_group("group2", None).unwrap();
    let target_path = manager.add_group("target", None).unwrap();

    // 添加不同内容
    let group1_entries = vec![HostEntry::new("127.0.0.1", &["localhost"]).unwrap()];
    let group2_entries = vec![HostEntry::new("192.168.1.1", &["router"]).unwrap()];
    let target_entries = vec![HostEntry::new("10.0.0.1", &["server"]).unwrap()];

    fs::write(&group1_path, format_entries(&group1_entries)).unwrap();
    fs::write(&group2_path, format_entries(&group2_entries)).unwrap();
    fs::write(&target_path, format_entries(&target_entries)).unwrap();

    // 合并分组
    manager.merge_groups("target", &["group1", "group2"]).unwrap();

    // 验证合并结果
    let target_manager = manager.get_group_manager("target").unwrap();
    let merged_entries = target_manager.read_hosts().unwrap();

    assert_eq!(merged_entries.len(), 3); // 原有1个 + 合并2个

    // 验证包含所有主机名
    let hostnames: Vec<String> = merged_entries.iter().flat_map(|e| e.hostnames.clone()).collect();
    assert!(hostnames.contains(&"localhost".to_string()));
    assert!(hostnames.contains(&"router".to_string()));
    assert!(hostnames.contains(&"server".to_string()));
}

#[test]
fn test_group_statistics() {
    let temp_dir = TempDir::new().unwrap();
    let mut manager = GroupManager::new(temp_dir.path()).unwrap();

    // 创建分组并添加内容
    let group_path = manager.add_group("stats_test", None).unwrap();
    let entries = vec![
        HostEntry::new("127.0.0.1", &["localhost"]).unwrap(),
        HostEntry::with_comment("192.168.1.1", &["router"], "网关").unwrap(),
        HostEntry::commented("10.0.0.1", &["disabled"]).unwrap(),
        HostEntry::new("::1", &["localhost6"]).unwrap(),
    ];
    fs::write(&group_path, format_entries(&entries)).unwrap();

    // 获取统计信息
    let stats = manager.get_group_stats("stats_test").unwrap();

    assert_eq!(stats.total_entries, 4);
    assert_eq!(stats.ipv4_count, 3);
    assert_eq!(stats.ipv6_count, 1);
    assert_eq!(stats.commented_count, 1);
    assert_eq!(stats.with_comments, 1);
}

#[test]
fn test_config_persistence() {
    let temp_dir = TempDir::new().unwrap();

    // 创建第一个管理器实例
    {
        let mut manager = GroupManager::new(temp_dir.path()).unwrap();
        manager.add_group("persistent1", Some("持久化测试1")).unwrap();
        manager.add_group("persistent2", Some("持久化测试2")).unwrap();
        manager.set_group_proxy_port("persistent1", Some(8080)).unwrap();
        manager.switch_group("persistent1").unwrap();
    }

    // 创建新的管理器实例，验证配置持久化
    {
        let manager = GroupManager::new(temp_dir.path()).unwrap();

        // 验证分组存在
        assert!(manager.get_group("persistent1").is_some());
        assert!(manager.get_group("persistent2").is_some());

        // 验证分组属性
        let group1 = manager.get_group("persistent1").unwrap();
        assert_eq!(group1.description, Some("持久化测试1".to_string()));
        assert_eq!(group1.proxy_port, Some(8080));

        // 验证活动分组
        assert_eq!(manager.get_active_group().unwrap().name, "persistent1");
    }
}

#[test]
fn test_get_active_hosts() {
    let temp_dir = TempDir::new().unwrap();
    let mut manager = GroupManager::new(temp_dir.path()).unwrap();

    // 没有活动分组时应该返回错误
    assert!(manager.get_active_hosts().is_err());

    // 创建分组并激活
    let hosts_path = manager.add_group("test_hosts", None).unwrap();

    // 写入测试内容
    std::fs::write(&hosts_path, "127.0.0.1 localhost\n192.168.1.1 router").unwrap();

    manager.switch_group("test_hosts").unwrap();

    // 获取活动分组的 hosts 条目
    let hosts = manager.get_active_hosts().unwrap();
    assert_eq!(hosts.len(), 2);
}

#[test]
fn test_group_managers() {
    let temp_dir = TempDir::new().unwrap();
    let mut manager = GroupManager::new(temp_dir.path()).unwrap();

    // 创建分组
    manager.add_group("manager_test", None).unwrap();

    // 获取分组管理器
    let group_manager = manager.get_group_manager("manager_test").unwrap();

    // 使用分组管理器操作
    let entries = vec![HostEntry::new("127.0.0.1", &["test.local"]).unwrap()];
    group_manager.write_hosts(&entries).unwrap();

    // 验证写入成功
    let read_entries = group_manager.read_hosts().unwrap();
    assert_eq!(read_entries.len(), 1);
    assert!(read_entries[0].contains_hostname("test.local"));

    // 获取不存在分组的管理器应该失败
    let result = manager.get_group_manager("nonexistent");
    assert!(matches!(result, Err(GroupError::GroupNotFound(_))));
}

/// 辅助函数：将 HostEntry 列表格式化为字符串
fn format_entries(entries: &[HostEntry]) -> String {
    entries.iter().map(|e| e.to_string()).collect::<Vec<_>>().join("\n")
}
