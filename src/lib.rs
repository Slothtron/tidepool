// Tidepool 项目根库
// 主要用于支持集成测试，重新导出核心功能

pub use tidepool_version_manager as version_manager;

// 重新导出常用类型，方便集成测试使用
pub use tidepool_version_manager::{
    go::GoManager, InstallRequest, ListInstalledRequest, RuntimeStatus, StatusRequest,
    SwitchRequest, UninstallRequest, VersionInfo, VersionList, VersionManager,
};
