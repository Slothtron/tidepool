//! Hosts 文件管理核心模块
//!
//! 提供 `HostsManager` 用于读写和管理 hosts 文件，
//! 支持备份、恢复和原子性写入操作。

use crate::host_entry::{HostEntry, HostsParseError};
use log::{debug, info, warn};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// HostsManager 错误类型
#[derive(Debug)]
pub enum HostsManagerError {
    /// IO 错误
    Io(io::Error),
    /// 权限不足
    PermissionDenied(String),
    /// 文件不存在
    FileNotFound(PathBuf),
    /// 解析错误
    ParseError(HostsParseError),
    /// 备份操作失败
    BackupFailed(String),
}

impl std::fmt::Display for HostsManagerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HostsManagerError::Io(err) => write!(f, "IO 错误: {}", err),
            HostsManagerError::PermissionDenied(path) => {
                write!(f, "权限不足，无法访问文件: {}", path)
            }
            HostsManagerError::FileNotFound(path) => {
                write!(f, "文件未找到: {}", path.display())
            }
            HostsManagerError::ParseError(err) => write!(f, "解析错误: {}", err),
            HostsManagerError::BackupFailed(msg) => write!(f, "备份失败: {}", msg),
        }
    }
}

impl std::error::Error for HostsManagerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            HostsManagerError::Io(err) => Some(err),
            HostsManagerError::ParseError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for HostsManagerError {
    fn from(err: io::Error) -> Self {
        match err.kind() {
            io::ErrorKind::NotFound => HostsManagerError::FileNotFound(PathBuf::from("unknown")),
            io::ErrorKind::PermissionDenied => {
                HostsManagerError::PermissionDenied("unknown".to_string())
            }
            _ => HostsManagerError::Io(err),
        }
    }
}

impl From<HostsParseError> for HostsManagerError {
    fn from(err: HostsParseError) -> Self {
        HostsManagerError::ParseError(err)
    }
}

/// Hosts 文件管理器
#[derive(Debug, Clone)]
pub struct HostsManager {
    /// hosts 文件路径
    hosts_path: PathBuf,
    /// 是否启用自动备份
    auto_backup: bool,
    /// 备份文件扩展名
    backup_extension: String,
}

impl HostsManager {
    /// 创建新的 HostsManager
    ///
    /// # 参数
    /// - `hosts_path` - hosts 文件路径
    ///
    /// # 示例
    /// ```
    /// use tidepool_hosts_manager::HostsManager;
    ///
    /// let manager = HostsManager::new("./my_hosts");
    /// ```
    pub fn new<P: AsRef<Path>>(hosts_path: P) -> Self {
        HostsManager {
            hosts_path: hosts_path.as_ref().to_path_buf(),
            auto_backup: true,
            backup_extension: ".backup".to_string(),
        }
    }

    /// 设置是否启用自动备份
    pub fn with_auto_backup(mut self, enable: bool) -> Self {
        self.auto_backup = enable;
        self
    }

    /// 设置备份文件扩展名
    pub fn with_backup_extension(mut self, extension: &str) -> Self {
        self.backup_extension = extension.to_string();
        self
    }

    /// 获取 hosts 文件路径
    pub fn hosts_path(&self) -> &Path {
        &self.hosts_path
    }

    /// 检查是否启用自动备份
    pub fn is_auto_backup_enabled(&self) -> bool {
        self.auto_backup
    }

    /// 获取备份文件路径
    pub fn backup_path(&self) -> PathBuf {
        let mut backup_path = self.hosts_path.clone();
        let mut extension = backup_path
            .extension()
            .map(|ext| ext.to_string_lossy().to_string())
            .unwrap_or_default();
        extension.push_str(&self.backup_extension);
        backup_path.set_extension(extension);
        backup_path
    }

    /// 检查是否有写入权限
    pub fn can_write(&self) -> bool {
        // 尝试检查文件权限
        if let Ok(metadata) = fs::metadata(&self.hosts_path) {
            !metadata.permissions().readonly()
        } else {
            // 如果文件不存在，检查父目录权限
            if let Some(parent) = self.hosts_path.parent() {
                parent.exists()
                    && fs::metadata(parent).map(|m| !m.permissions().readonly()).unwrap_or(false)
            } else {
                false
            }
        }
    }

    /// 检查管理员权限（Windows）或 root 权限（Unix）
    #[cfg(target_os = "windows")]
    pub fn requires_admin(&self) -> bool {
        // Windows 系统的 hosts 文件通常需要管理员权限
        self.hosts_path.starts_with(r"C:\Windows\System32")
    }

    #[cfg(not(target_os = "windows"))]
    pub fn requires_admin(&self) -> bool {
        // Unix 系统的 /etc/hosts 通常需要 root 权限
        self.hosts_path.starts_with("/etc")
    }

    /// 读取 hosts 文件内容
    ///
    /// # 返回
    /// 返回解析成功的 HostEntry 列表，跳过无法解析的行
    ///
    /// # 错误
    /// 当文件不存在或无法读取时返回错误
    pub fn read_hosts(&self) -> Result<Vec<HostEntry>, HostsManagerError> {
        debug!("读取 hosts 文件: {}", self.hosts_path.display());

        if !self.hosts_path.exists() {
            return Err(HostsManagerError::FileNotFound(self.hosts_path.clone()));
        }

        let content = fs::read_to_string(&self.hosts_path).map_err(|e| {
            if e.kind() == io::ErrorKind::PermissionDenied {
                HostsManagerError::PermissionDenied(self.hosts_path.display().to_string())
            } else {
                HostsManagerError::Io(e)
            }
        })?;

        let mut entries = Vec::new();
        let mut parse_errors = 0;

        for (line_num, line) in content.lines().enumerate() {
            match line.parse::<HostEntry>() {
                Ok(entry) => entries.push(entry),
                Err(HostsParseError::EmptyLine) => {
                    // 跳过空行，这是正常情况
                    continue;
                }
                Err(e) => {
                    warn!("第 {} 行解析失败: {} - {}", line_num + 1, e, line);
                    parse_errors += 1;
                }
            }
        }

        if parse_errors > 0 {
            warn!("共有 {} 行解析失败，已跳过", parse_errors);
        }

        info!("成功读取 {} 条 hosts 记录", entries.len());
        Ok(entries)
    }

    /// 写入 hosts 文件
    ///
    /// # 参数
    /// - `entries` - 要写入的 HostEntry 列表
    ///
    /// # 功能
    /// - 如果启用自动备份，会先备份原文件
    /// - 使用原子性写入，先写入临时文件再重命名
    /// - 保留原文件的权限设置
    pub fn write_hosts(&self, entries: &[HostEntry]) -> Result<(), HostsManagerError> {
        debug!("写入 hosts 文件: {} ({} 条记录)", self.hosts_path.display(), entries.len());

        // 检查权限
        if self.requires_admin() && !self.can_write() {
            return Err(HostsManagerError::PermissionDenied(format!(
                "需要管理员权限才能修改 {}",
                self.hosts_path.display()
            )));
        }

        // 自动备份
        if self.auto_backup && self.hosts_path.exists() {
            self.create_backup()?;
        }

        // 准备内容
        let mut content = String::new();
        for entry in entries {
            content.push_str(&entry.to_string());
            content.push('\n');
        }

        // 原子性写入：先写入临时文件
        let temp_path = self.hosts_path.with_extension("tmp");

        // 写入临时文件
        fs::write(&temp_path, content).map_err(|e| {
            if e.kind() == io::ErrorKind::PermissionDenied {
                HostsManagerError::PermissionDenied(self.hosts_path.display().to_string())
            } else {
                HostsManagerError::Io(e)
            }
        })?;

        // 原子性重命名
        fs::rename(&temp_path, &self.hosts_path).map_err(HostsManagerError::Io)?;

        info!("成功写入 {} 条 hosts 记录", entries.len());
        Ok(())
    }

    /// 创建备份文件
    pub fn create_backup(&self) -> Result<PathBuf, HostsManagerError> {
        if !self.hosts_path.exists() {
            return Err(HostsManagerError::FileNotFound(self.hosts_path.clone()));
        }

        let backup_path = self.backup_path();
        debug!("创建备份文件: {}", backup_path.display());

        fs::copy(&self.hosts_path, &backup_path)
            .map_err(|e| HostsManagerError::BackupFailed(e.to_string()))?;

        info!("备份文件已创建: {}", backup_path.display());
        Ok(backup_path)
    }

    /// 从备份文件恢复
    pub fn restore_from_backup(&self) -> Result<(), HostsManagerError> {
        let backup_path = self.backup_path();

        if !backup_path.exists() {
            return Err(HostsManagerError::FileNotFound(backup_path));
        }

        debug!("从备份恢复: {}", backup_path.display());

        fs::copy(&backup_path, &self.hosts_path).map_err(HostsManagerError::Io)?;

        info!("已从备份恢复 hosts 文件");
        Ok(())
    }

    /// 添加 hosts 条目
    ///
    /// 如果条目已存在（相同 IP 和主机名），则不会重复添加
    pub fn add_entry(&self, new_entry: HostEntry) -> Result<(), HostsManagerError> {
        let mut entries = self.read_hosts()?;

        // 检查是否已存在相同条目
        let exists = entries.iter().any(|entry| {
            entry.ip == new_entry.ip
                && entry.hostnames.iter().any(|h| new_entry.hostnames.contains(h))
        });

        if !exists {
            entries.push(new_entry);
            self.write_hosts(&entries)?;
            debug!("添加新的 hosts 条目");
        } else {
            debug!("hosts 条目已存在，跳过添加");
        }

        Ok(())
    }

    /// 移除包含指定主机名的条目
    pub fn remove_hostname(&self, hostname: &str) -> Result<usize, HostsManagerError> {
        let mut entries = self.read_hosts()?;
        let original_count = entries.len();

        // 移除包含指定主机名的条目
        entries.retain(|entry| !entry.contains_hostname(hostname));

        let removed_count = original_count - entries.len();
        if removed_count > 0 {
            self.write_hosts(&entries)?;
            info!("移除了 {} 条包含主机名 '{}' 的记录", removed_count, hostname);
        }

        Ok(removed_count)
    }

    /// 移除指定 IP 地址的所有条目
    pub fn remove_ip(&self, ip: &std::net::IpAddr) -> Result<usize, HostsManagerError> {
        let mut entries = self.read_hosts()?;
        let original_count = entries.len();

        entries.retain(|entry| entry.ip != *ip);

        let removed_count = original_count - entries.len();
        if removed_count > 0 {
            self.write_hosts(&entries)?;
            info!("移除了 {} 条 IP 地址为 '{}' 的记录", removed_count, ip);
        }

        Ok(removed_count)
    }

    /// 查找包含指定主机名的条目
    pub fn find_by_hostname(&self, hostname: &str) -> Result<Vec<HostEntry>, HostsManagerError> {
        let entries = self.read_hosts()?;
        let found: Vec<HostEntry> =
            entries.into_iter().filter(|entry| entry.contains_hostname(hostname)).collect();

        debug!("找到 {} 条包含主机名 '{}' 的记录", found.len(), hostname);
        Ok(found)
    }

    /// 查找指定 IP 地址的条目
    pub fn find_by_ip(&self, ip: &std::net::IpAddr) -> Result<Vec<HostEntry>, HostsManagerError> {
        let entries = self.read_hosts()?;
        let found: Vec<HostEntry> = entries.into_iter().filter(|entry| entry.ip == *ip).collect();

        debug!("找到 {} 条 IP 地址为 '{}' 的记录", found.len(), ip);
        Ok(found)
    }

    /// 获取统计信息
    pub fn get_stats(&self) -> Result<HostsStats, HostsManagerError> {
        let entries = self.read_hosts()?;

        let total_entries = entries.len();
        let ipv4_count = entries.iter().filter(|e| e.is_ipv4()).count();
        let ipv6_count = entries.iter().filter(|e| e.is_ipv6()).count();
        let commented_count = entries.iter().filter(|e| e.is_commented).count();
        let with_comments = entries.iter().filter(|e| e.comment.is_some()).count();

        // 收集所有唯一主机名
        let mut unique_hostnames = std::collections::HashSet::new();
        for entry in &entries {
            for hostname in &entry.hostnames {
                unique_hostnames.insert(hostname);
            }
        }

        Ok(HostsStats {
            total_entries,
            ipv4_count,
            ipv6_count,
            commented_count,
            with_comments,
            unique_hostnames: unique_hostnames.len(),
        })
    }
}

/// Hosts 文件统计信息
#[derive(Debug, Clone)]
pub struct HostsStats {
    /// 总条目数
    pub total_entries: usize,
    /// IPv4 条目数
    pub ipv4_count: usize,
    /// IPv6 条目数
    pub ipv6_count: usize,
    /// 被注释的条目数
    pub commented_count: usize,
    /// 有注释的条目数
    pub with_comments: usize,
    /// 唯一主机名数量
    pub unique_hostnames: usize,
}

impl std::fmt::Display for HostsStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Hosts 统计信息:\n\
             - 总条目数: {}\n\
             - IPv4 条目: {}\n\
             - IPv6 条目: {}\n\
             - 被注释条目: {}\n\
             - 有注释条目: {}\n\
             - 唯一主机名: {}",
            self.total_entries,
            self.ipv4_count,
            self.ipv6_count,
            self.commented_count,
            self.with_comments,
            self.unique_hostnames
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::net::{IpAddr, Ipv4Addr};
    use tempfile::{NamedTempFile, TempDir};

    fn create_test_hosts_file() -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "# 测试 hosts 文件").unwrap();
        writeln!(file, "127.0.0.1 localhost").unwrap();
        writeln!(file, "127.0.0.1 local.test # 测试域名").unwrap();
        writeln!(file, "# 192.168.1.1 router").unwrap();
        writeln!(file, "::1 localhost").unwrap();
        writeln!(file, "").unwrap(); // 空行
        file
    }

    #[test]
    fn test_manager_creation() {
        let manager = HostsManager::new("/tmp/hosts");
        assert_eq!(manager.hosts_path(), Path::new("/tmp/hosts"));
        assert!(manager.is_auto_backup_enabled());
    }

    #[test]
    fn test_backup_path() {
        let manager = HostsManager::new("./test_hosts");
        let backup_path = manager.backup_path();
        assert!(backup_path.to_string_lossy().contains(".backup"));
    }

    #[test]
    fn test_read_hosts() {
        let temp_file = create_test_hosts_file();
        let manager = HostsManager::new(temp_file.path());

        let entries = manager.read_hosts().unwrap();
        assert_eq!(entries.len(), 4); // 应该有4条有效记录

        // 检查第一个条目
        let first_entry = &entries[0];
        assert_eq!(first_entry.ip, IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
        assert_eq!(first_entry.hostnames, vec!["localhost"]);
        assert!(!first_entry.is_commented);

        // 检查带注释的条目
        let second_entry = &entries[1];
        assert_eq!(second_entry.comment, Some("测试域名".to_string()));

        // 检查被注释的条目
        let commented_entry = &entries[2];
        assert!(commented_entry.is_commented);
    }

    #[test]
    fn test_write_hosts() {
        let temp_dir = TempDir::new().unwrap();
        let hosts_path = temp_dir.path().join("hosts");
        let manager = HostsManager::new(&hosts_path);

        let entries = vec![
            HostEntry::new("127.0.0.1", &["localhost"]).unwrap(),
            HostEntry::with_comment("192.168.1.1", &["router"], "网关").unwrap(),
        ];

        manager.write_hosts(&entries).unwrap();
        assert!(hosts_path.exists());

        // 读取并验证
        let read_entries = manager.read_hosts().unwrap();
        assert_eq!(read_entries.len(), 2);
        assert_eq!(read_entries[0].hostnames, vec!["localhost"]);
        assert_eq!(read_entries[1].comment, Some("网关".to_string()));
    }

    #[test]
    fn test_add_entry() {
        let temp_file = create_test_hosts_file();
        let manager = HostsManager::new(temp_file.path());

        let new_entry = HostEntry::new("10.0.0.1", &["test.local"]).unwrap();
        manager.add_entry(new_entry).unwrap();

        let entries = manager.read_hosts().unwrap();
        assert!(entries.iter().any(|e| e.contains_hostname("test.local")));
    }

    #[test]
    fn test_remove_hostname() {
        let temp_file = create_test_hosts_file();
        let manager = HostsManager::new(temp_file.path());

        let removed = manager.remove_hostname("localhost").unwrap();
        assert!(removed > 0);

        let entries = manager.read_hosts().unwrap();
        assert!(!entries.iter().any(|e| e.contains_hostname("localhost")));
    }

    #[test]
    fn test_find_by_hostname() {
        let temp_file = create_test_hosts_file();
        let manager = HostsManager::new(temp_file.path());

        let found = manager.find_by_hostname("localhost").unwrap();
        assert!(found.len() >= 1);
    }

    #[test]
    fn test_stats() {
        let temp_file = create_test_hosts_file();
        let manager = HostsManager::new(temp_file.path());

        let stats = manager.get_stats().unwrap();
        assert_eq!(stats.total_entries, 4);
        assert_eq!(stats.ipv4_count, 3);
        assert_eq!(stats.ipv6_count, 1);
        assert_eq!(stats.commented_count, 1);
        assert_eq!(stats.with_comments, 1);
    }

    #[test]
    fn test_backup_and_restore() {
        let temp_dir = TempDir::new().unwrap();
        let hosts_path = temp_dir.path().join("hosts");

        // 创建初始文件
        let manager = HostsManager::new(&hosts_path);
        let initial_entries = vec![HostEntry::new("127.0.0.1", &["localhost"]).unwrap()];
        manager.write_hosts(&initial_entries).unwrap();

        // 创建备份
        let backup_path = manager.create_backup().unwrap();
        assert!(backup_path.exists());

        // 修改文件
        let modified_entries = vec![HostEntry::new("192.168.1.1", &["router"]).unwrap()];
        manager.write_hosts(&modified_entries).unwrap();

        // 从备份恢复
        manager.restore_from_backup().unwrap();
        let restored_entries = manager.read_hosts().unwrap();
        assert_eq!(restored_entries.len(), 1);
        assert!(restored_entries[0].contains_hostname("localhost"));
    }
}
