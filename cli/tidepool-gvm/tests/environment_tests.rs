use std::path::PathBuf;

#[cfg(test)]
mod environment_display_tests {
    use super::*;
    use tidepool_gvm::ui::UI;

    #[test]
    fn test_environment_setup_display() {
        let ui = UI::new();
        let install_path = PathBuf::from("/test/go/1.21.0");
        let version = "1.21.0";

        // 这个测试主要验证函数能正常调用而不会崩溃
        // 在实际使用中会显示正确的环境变量配置信息
        ui.show_environment_setup(&install_path, version);

        // 如果能执行到这里，说明函数调用成功
    }

    #[test]
    fn test_environment_setup_with_different_paths() {
        let ui = UI::new();
        let test_cases = vec![
            ("/home/user/.gvm/versions/1.21.0", "1.21.0"),
            ("C:\\gvm\\versions\\1.20.5", "1.20.5"),
            ("/opt/go/1.19.3", "1.19.3"),
        ];

        for (path, version) in test_cases {
            let install_path = PathBuf::from(path);
            ui.show_environment_setup(&install_path, version);
        }

        // 如果能执行到这里，说明所有测试用例都成功
    }
}
