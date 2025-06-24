//! Hosts 分组管理模块
//!
//! 提供多个 hosts 文件的逻辑分组管理功能，
//! 支持分组配置的持久化存储和动态切换。

use crate::host_entry::HostEntry;
use crate::hosts_manager::{HostsManager, HostsManagerError};
use log::{debug, info, warn};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// 分组管理错误类型
#[derive(Debug)]
pub enum GroupError {
    /// IO 错误
    Io(std::io::Error),
    /// 序列化/反序列化错误
    Serialization(String),
    /// 分组不存在
    GroupNotFound(String),
    /// 分组已存在
    GroupAlreadyExists(String),
    /// 配置文件错误
    ConfigError(String),
    /// Hosts 管理器错误
    HostsManager(HostsManagerError),
}

impl std::fmt::Display for GroupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GroupError::Io(err) => write!(f, "IO 错误: {}", err),
            GroupError::Serialization(msg) => write!(f, "序列化错误: {}", msg),
            GroupError::GroupNotFound(name) => write!(f, "分组不存在: {}", name),
            GroupError::GroupAlreadyExists(name) => write!(f, "分组已存在: {}", name),
            GroupError::ConfigError(msg) => write!(f, "配置错误: {}", msg),
            GroupError::HostsManager(err) => write!(f, "Hosts 管理错误: {}", err),
        }
    }
}

impl std::error::Error for GroupError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            GroupError::Io(err) => Some(err),
            GroupError::HostsManager(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for GroupError {
    fn from(err: std::io::Error) -> Self {
        GroupError::Io(err)
    }
}

impl From<HostsManagerError> for GroupError {
    fn from(err: HostsManagerError) -> Self {
        GroupError::HostsManager(err)
    }
}

impl From<toml::de::Error> for GroupError {
    fn from(err: toml::de::Error) -> Self {
        GroupError::Serialization(format!("TOML 反序列化失败: {}", err))
    }
}

impl From<toml::ser::Error> for GroupError {
    fn from(err: toml::ser::Error) -> Self {
        GroupError::Serialization(format!("TOML 序列化失败: {}", err))
    }
}

/// 分组配置信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupConfig {
    /// 分组名称
    pub name: String,
    /// hosts 文件路径
    pub hosts_path: PathBuf,
    /// 分组描述
    pub description: Option<String>,
    /// 创建时间
    pub created_at: String,
    /// 是否启用
    pub enabled: bool,
    /// 代理端口（用于网络代理功能）
    pub proxy_port: Option<u16>,
}

impl GroupConfig {
    /// 创建新的分组配置
    pub fn new(name: &str, hosts_path: PathBuf) -> Self {
        GroupConfig {
            name: name.to_string(),
            hosts_path,
            description: None,
            created_at: chrono::Utc::now().to_rfc3339(),
            enabled: true,
            proxy_port: None,
        }
    }

    /// 设置描述
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    /// 设置代理端口
    pub fn with_proxy_port(mut self, port: u16) -> Self {
        self.proxy_port = Some(port);
        self
    }
}

/// 分组管理器配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupManagerConfig {
    /// 当前活动分组
    pub active_group: Option<String>,
    /// 所有分组配置
    pub groups: HashMap<String, GroupConfig>,
    /// 配置版本
    pub version: String,
}

impl Default for GroupManagerConfig {
    fn default() -> Self {
        GroupManagerConfig {
            active_group: None,
            groups: HashMap::new(),
            version: "1.0".to_string(),
        }
    }
}

/// 分组管理器
#[derive(Debug)]
pub struct GroupManager {
    /// 配置文件路径
    config_path: PathBuf,
    /// 当前配置
    config: GroupManagerConfig,
    /// 分组文件存储目录
    groups_dir: PathBuf,
}

impl GroupManager {
    /// 创建新的分组管理器
    ///
    /// # 参数
    /// - `config_dir` - 配置文件目录
    ///
    /// # 示例
    /// ```
    /// use tidepool_hosts_manager::GroupManager;
    /// use tempfile::TempDir;
    ///
    /// let temp_dir = TempDir::new().unwrap();
    /// let manager = GroupManager::new(temp_dir.path()).unwrap();
    /// ```
    pub fn new<P: AsRef<Path>>(config_dir: P) -> Result<Self, GroupError> {
        let config_dir = config_dir.as_ref();

        // 确保配置目录存在
        if !config_dir.exists() {
            fs::create_dir_all(config_dir)?;
            debug!("创建配置目录: {}", config_dir.display());
        }

        let config_path = config_dir.join("groups.toml");
        let groups_dir = config_dir.join("groups");

        // 确保分组目录存在
        if !groups_dir.exists() {
            fs::create_dir_all(&groups_dir)?;
            debug!("创建分组目录: {}", groups_dir.display());
        }

        let config = if config_path.exists() {
            Self::load_config(&config_path)?
        } else {
            debug!("配置文件不存在，使用默认配置");
            GroupManagerConfig::default()
        };

        let manager = GroupManager { config_path, config, groups_dir };

        // 初始保存配置
        manager.save_config()?;

        info!("分组管理器初始化完成，配置目录: {}", config_dir.display());
        Ok(manager)
    }

    /// 从配置文件加载配置
    fn load_config(config_path: &Path) -> Result<GroupManagerConfig, GroupError> {
        let content = fs::read_to_string(config_path)?;
        let config: GroupManagerConfig = toml::from_str(&content)?;
        debug!("从配置文件加载配置: {}", config_path.display());
        Ok(config)
    }

    /// 保存配置到文件
    fn save_config(&self) -> Result<(), GroupError> {
        let content = toml::to_string_pretty(&self.config)?;
        fs::write(&self.config_path, content)?;
        debug!("保存配置到文件: {}", self.config_path.display());
        Ok(())
    }

    /// 获取分组文件路径
    fn get_group_file_path(&self, group_name: &str) -> PathBuf {
        self.groups_dir.join(format!("{}.hosts", group_name))
    }

    /// 添加新分组
    ///
    /// # 参数
    /// - `name` - 分组名称
    /// - `description` - 分组描述
    ///
    /// # 返回
    /// 返回分组的 hosts 文件路径
    pub fn add_group(
        &mut self,
        name: &str,
        description: Option<&str>,
    ) -> Result<PathBuf, GroupError> {
        if self.config.groups.contains_key(name) {
            return Err(GroupError::GroupAlreadyExists(name.to_string()));
        }

        let hosts_path = self.get_group_file_path(name);
        let mut group_config = GroupConfig::new(name, hosts_path.clone());

        if let Some(desc) = description {
            group_config = group_config.with_description(desc);
        }

        // 创建空的 hosts 文件
        if !hosts_path.exists() {
            fs::write(&hosts_path, "# Hosts 文件 - 分组: {}\n")?;
            debug!("创建分组 hosts 文件: {}", hosts_path.display());
        }

        self.config.groups.insert(name.to_string(), group_config);
        self.save_config()?;

        info!("添加新分组: {} -> {}", name, hosts_path.display());
        Ok(hosts_path)
    }

    /// 移除分组
    pub fn remove_group(&mut self, name: &str) -> Result<(), GroupError> {
        if !self.config.groups.contains_key(name) {
            return Err(GroupError::GroupNotFound(name.to_string()));
        }

        // 如果是当前活动分组，需要先切换
        if self.config.active_group.as_ref() == Some(&name.to_string()) {
            self.config.active_group = None;
        }

        // 移除分组配置
        let group_config = self.config.groups.remove(name).unwrap();

        // 删除分组 hosts 文件
        if group_config.hosts_path.exists() {
            fs::remove_file(&group_config.hosts_path)?;
            debug!("删除分组 hosts 文件: {}", group_config.hosts_path.display());
        }

        self.save_config()?;
        info!("移除分组: {}", name);
        Ok(())
    }

    /// 列出所有分组
    pub fn list_groups(&self) -> Vec<&GroupConfig> {
        self.config.groups.values().collect()
    }

    /// 获取指定分组配置
    pub fn get_group(&self, name: &str) -> Option<&GroupConfig> {
        self.config.groups.get(name)
    }

    /// 获取当前活动分组
    pub fn get_active_group(&self) -> Option<&GroupConfig> {
        self.config.active_group.as_ref().and_then(|name| self.config.groups.get(name))
    }

    /// 切换到指定分组
    ///
    /// 仅设置活动分组，不会修改系统文件
    pub fn switch_group(&mut self, name: &str) -> Result<(), GroupError> {
        let group_config = self
            .config
            .groups
            .get(name)
            .ok_or_else(|| GroupError::GroupNotFound(name.to_string()))?;

        if !group_config.enabled {
            warn!("分组 '{}' 已被禁用", name);
            return Err(GroupError::ConfigError(format!("分组 '{}' 已被禁用", name)));
        }

        // 只更新活动分组配置
        self.config.active_group = Some(name.to_string());
        self.save_config()?;

        info!("切换到分组: {}", name);
        Ok(())
    }

    /// 获取活动分组的 hosts 条目
    ///
    /// 用于服务内映射，不涉及系统文件
    pub fn get_active_hosts(&self) -> Result<Vec<HostEntry>, GroupError> {
        let active_group = self
            .get_active_group()
            .ok_or_else(|| GroupError::ConfigError("未设置活动分组".to_string()))?;

        let group_manager = HostsManager::new(&active_group.hosts_path);
        let entries = group_manager.read_hosts().map_err(GroupError::HostsManager)?;

        Ok(entries)
    }

    /// 更新分组配置
    pub fn update_group(
        &mut self,
        name: &str,
        updater: impl FnOnce(&mut GroupConfig),
    ) -> Result<(), GroupError> {
        let group_config = self
            .config
            .groups
            .get_mut(name)
            .ok_or_else(|| GroupError::GroupNotFound(name.to_string()))?;

        updater(group_config);
        self.save_config()?;

        debug!("更新分组配置: {}", name);
        Ok(())
    }

    /// 启用/禁用分组
    pub fn set_group_enabled(&mut self, name: &str, enabled: bool) -> Result<(), GroupError> {
        self.update_group(name, |config| {
            config.enabled = enabled;
        })?;

        let status = if enabled { "启用" } else { "禁用" };
        info!("{} 分组: {}", status, name);
        Ok(())
    }

    /// 设置分组代理端口
    pub fn set_group_proxy_port(
        &mut self,
        name: &str,
        port: Option<u16>,
    ) -> Result<(), GroupError> {
        self.update_group(name, |config| {
            config.proxy_port = port;
        })?;

        match port {
            Some(p) => info!("设置分组 '{}' 代理端口: {}", name, p),
            None => info!("清除分组 '{}' 代理端口", name),
        }
        Ok(())
    }

    /// 获取分组的 HostsManager
    pub fn get_group_manager(&self, name: &str) -> Result<HostsManager, GroupError> {
        let group_config = self
            .config
            .groups
            .get(name)
            .ok_or_else(|| GroupError::GroupNotFound(name.to_string()))?;

        Ok(HostsManager::new(&group_config.hosts_path))
    }

    /// 复制分组
    pub fn copy_group(
        &mut self,
        source: &str,
        target: &str,
        description: Option<&str>,
    ) -> Result<(), GroupError> {
        if !self.config.groups.contains_key(source) {
            return Err(GroupError::GroupNotFound(source.to_string()));
        }

        if self.config.groups.contains_key(target) {
            return Err(GroupError::GroupAlreadyExists(target.to_string()));
        }

        // 读取源分组内容
        let source_manager = self.get_group_manager(source)?;
        let entries = source_manager.read_hosts()?;

        // 创建目标分组
        let target_path = self.add_group(target, description)?;
        let target_manager = HostsManager::new(&target_path);
        target_manager.write_hosts(&entries)?;

        info!("复制分组 '{}' 到 '{}'", source, target);
        Ok(())
    }

    /// 合并分组
    pub fn merge_groups(&mut self, target: &str, sources: &[&str]) -> Result<(), GroupError> {
        if !self.config.groups.contains_key(target) {
            return Err(GroupError::GroupNotFound(target.to_string()));
        }

        let target_manager = self.get_group_manager(target)?;
        let mut all_entries = target_manager.read_hosts()?;

        // 收集所有源分组的条目
        for &source in sources {
            if !self.config.groups.contains_key(source) {
                warn!("源分组不存在，跳过: {}", source);
                continue;
            }

            let source_manager = self.get_group_manager(source)?;
            let source_entries = source_manager.read_hosts()?;

            // 简单合并，避免重复（基于 IP 和第一个主机名）
            for entry in source_entries {
                let exists = all_entries.iter().any(|existing| {
                    existing.ip == entry.ip && existing.hostnames.first() == entry.hostnames.first()
                });

                if !exists {
                    all_entries.push(entry);
                }
            }
        }

        target_manager.write_hosts(&all_entries)?;
        info!("合并 {} 个分组到 '{}'", sources.len(), target);
        Ok(())
    }

    /// 获取分组统计信息
    pub fn get_group_stats(
        &self,
        name: &str,
    ) -> Result<crate::hosts_manager::HostsStats, GroupError> {
        let manager = self.get_group_manager(name)?;
        Ok(manager.get_stats()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_hosts_content() -> &'static str {
        "127.0.0.1 localhost\n192.168.1.1 router # 网关\n# ::1 localhost\n"
    }

    #[test]
    fn test_group_manager_creation() {
        let temp_dir = TempDir::new().unwrap();
        let _manager = GroupManager::new(temp_dir.path()).unwrap();

        assert!(temp_dir.path().join("groups.toml").exists());
        assert!(temp_dir.path().join("groups").exists());
    }

    #[test]
    fn test_add_and_remove_group() {
        let temp_dir = TempDir::new().unwrap();
        let mut manager = GroupManager::new(temp_dir.path()).unwrap();

        // 添加分组
        let hosts_path = manager.add_group("dev", Some("开发环境")).unwrap();
        assert!(hosts_path.exists());
        assert!(manager.get_group("dev").is_some());

        // 移除分组
        manager.remove_group("dev").unwrap();
        assert!(manager.get_group("dev").is_none());
        assert!(!hosts_path.exists());
    }

    #[test]
    fn test_list_groups() {
        let temp_dir = TempDir::new().unwrap();
        let mut manager = GroupManager::new(temp_dir.path()).unwrap();

        manager.add_group("dev", Some("开发环境")).unwrap();
        manager.add_group("prod", Some("生产环境")).unwrap();

        let groups = manager.list_groups();
        assert_eq!(groups.len(), 2);

        let group_names: Vec<&str> = groups.iter().map(|g| g.name.as_str()).collect();
        assert!(group_names.contains(&"dev"));
        assert!(group_names.contains(&"prod"));
    }

    #[test]
    fn test_switch_group() {
        let temp_dir = TempDir::new().unwrap();
        let mut manager = GroupManager::new(temp_dir.path()).unwrap();

        // 创建测试分组和内容
        let hosts_path = manager.add_group("test", None).unwrap();
        fs::write(&hosts_path, create_test_hosts_content()).unwrap();

        // 切换分组
        manager.switch_group("test").unwrap();
        assert_eq!(manager.get_active_group().unwrap().name, "test");
    }

    #[test]
    fn test_group_enable_disable() {
        let temp_dir = TempDir::new().unwrap();
        let mut manager = GroupManager::new(temp_dir.path()).unwrap();

        manager.add_group("test", None).unwrap();

        // 禁用分组
        manager.set_group_enabled("test", false).unwrap();
        assert!(!manager.get_group("test").unwrap().enabled);

        // 尝试切换到禁用的分组应该失败
        let result = manager.switch_group("test");
        assert!(matches!(result, Err(GroupError::ConfigError(_))));
    }

    #[test]
    fn test_copy_group() {
        let temp_dir = TempDir::new().unwrap();
        let mut manager = GroupManager::new(temp_dir.path()).unwrap();

        // 创建源分组
        let source_path = manager.add_group("source", None).unwrap();
        fs::write(&source_path, create_test_hosts_content()).unwrap();

        // 复制分组
        manager.copy_group("source", "target", Some("复制的分组")).unwrap();

        assert!(manager.get_group("target").is_some());

        // 验证内容相同
        let source_manager = manager.get_group_manager("source").unwrap();
        let target_manager = manager.get_group_manager("target").unwrap();

        let source_entries = source_manager.read_hosts().unwrap();
        let target_entries = target_manager.read_hosts().unwrap();

        assert_eq!(source_entries.len(), target_entries.len());
    }

    #[test]
    fn test_merge_groups() {
        let temp_dir = TempDir::new().unwrap();
        let mut manager = GroupManager::new(temp_dir.path()).unwrap();

        // 创建多个分组
        let group1_path = manager.add_group("group1", None).unwrap();
        let group2_path = manager.add_group("group2", None).unwrap();
        let target_path = manager.add_group("target", None).unwrap();

        fs::write(&group1_path, "127.0.0.1 localhost\n").unwrap();
        fs::write(&group2_path, "192.168.1.1 router\n").unwrap();
        fs::write(&target_path, "10.0.0.1 server\n").unwrap();

        // 合并分组
        manager.merge_groups("target", &["group1", "group2"]).unwrap();

        let target_manager = manager.get_group_manager("target").unwrap();
        let merged_entries = target_manager.read_hosts().unwrap();

        assert_eq!(merged_entries.len(), 3); // 原有1个 + 合并2个
    }

    #[test]
    fn test_group_proxy_port() {
        let temp_dir = TempDir::new().unwrap();
        let mut manager = GroupManager::new(temp_dir.path()).unwrap();

        manager.add_group("test", None).unwrap();

        // 设置代理端口
        manager.set_group_proxy_port("test", Some(8080)).unwrap();
        assert_eq!(manager.get_group("test").unwrap().proxy_port, Some(8080));

        // 清除代理端口
        manager.set_group_proxy_port("test", None).unwrap();
        assert_eq!(manager.get_group("test").unwrap().proxy_port, None);
    }

    #[test]
    fn test_config_persistence() {
        let temp_dir = TempDir::new().unwrap();

        // 创建管理器并添加分组
        {
            let mut manager = GroupManager::new(temp_dir.path()).unwrap();

            manager.add_group("persistent", Some("持久化测试")).unwrap();
            manager.switch_group("persistent").unwrap();
        }

        // 重新加载管理器，验证配置持久化
        {
            let manager = GroupManager::new(temp_dir.path()).unwrap();
            assert!(manager.get_group("persistent").is_some());
            assert_eq!(manager.get_active_group().unwrap().name, "persistent");
        }
    }
}
