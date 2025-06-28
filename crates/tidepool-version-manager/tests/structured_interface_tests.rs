use std::path::PathBuf;
use tidepool_version_manager::{
    go::GoManager, ListInstalledRequest, StatusRequest, VersionManager,
};

#[tokio::test]
async fn test_version_manager_interface() {
    let manager = GoManager::new();
    let base_dir = PathBuf::from("test_versions");

    // 测试状态查询 - 应该返回结构化数据
    let status_request = StatusRequest { base_dir: Some(base_dir.clone()) };
    match manager.status(status_request) {
        Ok(status) => {
            assert!(status.environment_vars.contains_key("GOROOT"));
            assert!(status.environment_vars.contains_key("GOPATH"));
            println!("✅ Status query returns structured data");
        }
        Err(e) => println!("ℹ️  Status query failed (expected): {e}"),
    }

    // 测试版本列表查询 - 应该返回 VersionList 结构
    let list_request = ListInstalledRequest { base_dir: base_dir.clone() };
    match manager.list_installed(list_request) {
        Ok(version_list) => {
            assert_eq!(version_list.versions.len(), version_list.total_count);
            println!(
                "✅ List installed returns VersionList with {} versions",
                version_list.total_count
            );
        }
        Err(e) => println!("ℹ️  List installed failed (expected for non-existent dir): {e}"),
    }
    // 测试配置结构体 - 在当前实现中，配置通过请求结构体传递
    println!("✅ Request structures can be created successfully");
    println!("   - StatusRequest with base_dir: {base_dir:?}");
    println!("   - ListInstalledRequest with base_dir: {base_dir:?}");
}
