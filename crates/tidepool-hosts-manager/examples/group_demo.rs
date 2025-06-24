//! 分组管理功能演示
//!
//! 演示 GroupManager 的分组管理功能

use tempfile::TempDir;
use tidepool_hosts_manager::{GroupManager, HostEntry};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    env_logger::init();

    println!("🎯 分组管理功能演示");

    // 创建临时目录作为配置目录
    let temp_dir = TempDir::new()?;
    println!("   配置目录: {}", temp_dir.path().display());

    // 1. 创建分组管理器
    demo_group_manager_creation(&temp_dir).await?;

    // 2. 分组基本操作
    demo_group_operations(&temp_dir).await?;

    // 3. 分组切换和内容管理
    demo_group_switching(&temp_dir).await?;

    // 4. 高级分组操作
    demo_advanced_operations(&temp_dir).await?;

    println!("✅ 演示完成");
    Ok(())
}

async fn demo_group_manager_creation(temp_dir: &TempDir) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📋 步骤1: 分组管理器创建演示");

    let manager = GroupManager::new(temp_dir.path())?;
    println!("   分组管理器创建成功");

    // 检查初始状态
    let groups = manager.list_groups();
    println!("   初始分组数量: {}", groups.len());

    let active_group = manager.get_active_group();
    match active_group {
        Some(group) => println!("   当前活动分组: {}", group.name),
        None => println!("   当前无活动分组"),
    }

    Ok(())
}

async fn demo_group_operations(temp_dir: &TempDir) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📋 步骤2: 分组基本操作演示");

    let mut manager = GroupManager::new(temp_dir.path())?;

    // 添加分组
    let dev_path = manager.add_group("dev", Some("开发环境"))?;
    println!("   添加开发分组: {}", dev_path.display());

    let prod_path = manager.add_group("prod", Some("生产环境"))?;
    println!("   添加生产分组: {}", prod_path.display());

    let test_path = manager.add_group("test", Some("测试环境"))?;
    println!("   添加测试分组: {}", test_path.display());

    // 列出所有分组
    let groups = manager.list_groups();
    println!("   当前分组列表:");
    for group in groups {
        println!(
            "     - {} ({}): {}",
            group.name,
            group.description.as_deref().unwrap_or("无描述"),
            group.hosts_path.display()
        );
        println!("       创建时间: {}", group.created_at);
        println!("       启用状态: {}", group.enabled);
    }

    // 设置分组属性
    manager.set_group_proxy_port("dev", Some(8080))?;
    println!("   设置开发分组代理端口: 8080");

    manager.set_group_enabled("test", false)?;
    println!("   禁用测试分组");

    // 获取特定分组信息
    if let Some(dev_group) = manager.get_group("dev") {
        println!("   开发分组详情:");
        println!("     - 代理端口: {:?}", dev_group.proxy_port);
        println!("     - 启用状态: {}", dev_group.enabled);
    }

    Ok(())
}

async fn demo_group_switching(temp_dir: &TempDir) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📋 步骤3: 分组切换和内容管理演示");

    let mut manager = GroupManager::new(temp_dir.path())?;

    // 确保分组存在
    manager.add_group("web", Some("Web 开发环境"))?;
    manager.add_group("api", Some("API 开发环境"))?;

    // 为分组添加内容
    setup_group_content(&mut manager, "web").await?;
    setup_group_content(&mut manager, "api").await?;

    // 切换分组
    manager.switch_group("web")?;
    println!("   切换到 web 分组");

    if let Some(active) = manager.get_active_group() {
        println!("     当前活动分组: {}", active.name);
    }

    // 获取分组统计信息
    let web_stats = manager.get_group_stats("web")?;
    println!("   Web 分组统计:");
    println!("{}", web_stats);

    // 切换到另一个分组
    manager.switch_group("api")?;
    println!("   切换到 api 分组");

    let api_stats = manager.get_group_stats("api")?;
    println!("   API 分组统计:");
    println!("{}", api_stats);

    Ok(())
}

async fn demo_advanced_operations(temp_dir: &TempDir) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📋 步骤4: 高级分组操作演示");

    let mut manager = GroupManager::new(temp_dir.path())?;

    // 确保有源分组
    setup_group_with_entries(
        &mut manager,
        "source1",
        &[("127.0.0.1", &["localhost", "local"]), ("192.168.1.1", &["router"])],
    )
    .await?;

    setup_group_with_entries(
        &mut manager,
        "source2",
        &[("10.0.0.1", &["server"]), ("8.8.8.8", &["dns"])],
    )
    .await?;

    // 复制分组
    manager.copy_group("source1", "backup1", Some("source1 的备份"))?;
    println!("   复制 source1 到 backup1");

    let backup_stats = manager.get_group_stats("backup1")?;
    println!("   备份分组统计: {} 条记录", backup_stats.total_entries);

    // 创建目标分组用于合并
    setup_group_with_entries(&mut manager, "merged", &[("127.0.0.1", &["localhost"])]).await?;

    // 合并分组
    manager.merge_groups("merged", &["source1", "source2"])?;
    println!("   合并 source1 和 source2 到 merged 分组");

    let merged_stats = manager.get_group_stats("merged")?;
    println!("   合并后统计: {} 条记录", merged_stats.total_entries);

    // 演示分组管理器功能
    demo_group_managers(&mut manager).await?;

    // 清理演示
    demo_cleanup(&mut manager).await?;

    Ok(())
}

async fn setup_group_content(
    manager: &mut GroupManager,
    group_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let group_manager = manager.get_group_manager(group_name)?;

    let entries = match group_name {
        "web" => vec![
            HostEntry::new("127.0.0.1", &["localhost", "web.local"])?,
            HostEntry::with_comment("192.168.1.100", &["dev.example.com"], "开发服务器")?,
            HostEntry::new("192.168.1.101", &["staging.example.com"])?,
        ],
        "api" => vec![
            HostEntry::new("127.0.0.1", &["localhost", "api.local"])?,
            HostEntry::with_comment("192.168.1.200", &["api.dev.com"], "API 开发")?,
            HostEntry::new("192.168.1.201", &["api.staging.com"])?,
        ],
        _ => vec![],
    };

    group_manager.write_hosts(&entries)?;
    println!("   为 {} 分组添加了 {} 条记录", group_name, entries.len());

    Ok(())
}

async fn setup_group_with_entries(
    manager: &mut GroupManager,
    group_name: &str,
    entries_data: &[(&str, &[&str])],
) -> Result<(), Box<dyn std::error::Error>> {
    manager.add_group(group_name, None)?;

    let group_manager = manager.get_group_manager(group_name)?;
    let entries: Result<Vec<_>, _> =
        entries_data.iter().map(|(ip, hostnames)| HostEntry::new(ip, hostnames)).collect();

    group_manager.write_hosts(&entries?)?;
    println!("   为 {} 分组添加了 {} 条记录", group_name, entries_data.len());

    Ok(())
}

async fn demo_group_managers(manager: &mut GroupManager) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n   分组管理器功能演示:");

    // 获取各个分组的管理器
    let groups = ["source1", "source2", "merged"];

    for group_name in &groups {
        if let Ok(group_manager) = manager.get_group_manager(group_name) {
            let entries = group_manager.read_hosts()?;
            println!("     {} 分组内容:", group_name);

            for entry in entries.iter().take(3) {
                // 只显示前3条
                println!("       {}", entry);
            }

            if entries.len() > 3 {
                println!("       ... 还有 {} 条记录", entries.len() - 3);
            }
        }
    }

    Ok(())
}

async fn demo_cleanup(manager: &mut GroupManager) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n   清理演示:");

    // 移除一些测试分组
    let groups_to_remove = ["source1", "source2", "backup1"];

    for group_name in &groups_to_remove {
        if manager.get_group(group_name).is_some() {
            manager.remove_group(group_name)?;
            println!("     移除分组: {}", group_name);
        }
    }

    let remaining_groups = manager.list_groups();
    println!("   清理后剩余分组数: {}", remaining_groups.len());

    Ok(())
}
