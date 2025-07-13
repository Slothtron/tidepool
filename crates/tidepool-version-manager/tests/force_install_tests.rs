// 测试强制安装功能的参数处理
use tempfile::TempDir;
use tidepool_version_manager::InstallRequest;

#[test]
fn test_force_parameter_handling() {
    // 测试 InstallRequest 结构体正确处理 force 参数
    let temp_install = TempDir::new().unwrap();
    let temp_cache = TempDir::new().unwrap();

    let request = InstallRequest {
        version: "1.21.3".to_string(),
        install_dir: temp_install.path().to_path_buf(),
        download_dir: temp_cache.path().to_path_buf(),
        force: true,
    };

    assert_eq!(request.version, "1.21.3");
    assert!(request.force);

    let request_no_force = InstallRequest {
        version: "1.21.3".to_string(),
        install_dir: temp_install.path().to_path_buf(),
        download_dir: temp_cache.path().to_path_buf(),
        force: false,
    };

    assert!(!request_no_force.force);
}
