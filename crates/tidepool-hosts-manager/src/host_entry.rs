//! Host 条目解析和管理模块
//!
//! 提供 `HostEntry` 结构体用于表示 hosts 文件中的单条记录，
//! 支持 IPv4/IPv6 地址映射和注释解析。

use std::fmt;
use std::net::IpAddr;
use std::str::FromStr;

/// 表示 hosts 文件中的错误类型
#[derive(Debug, Clone)]
pub enum HostsParseError {
    /// 无效的 IP 地址格式
    InvalidIpAddress(String),
    /// 无效的主机名格式
    InvalidHostname(String),
    /// 行格式错误
    InvalidLineFormat(String),
    /// 空行或无效行
    EmptyLine,
}

impl fmt::Display for HostsParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HostsParseError::InvalidIpAddress(ip) => {
                write!(f, "无效的 IP 地址格式: {}", ip)
            }
            HostsParseError::InvalidHostname(hostname) => {
                write!(f, "无效的主机名格式: {}", hostname)
            }
            HostsParseError::InvalidLineFormat(line) => {
                write!(f, "无效的行格式: {}", line)
            }
            HostsParseError::EmptyLine => {
                write!(f, "空行或仅包含注释的行")
            }
        }
    }
}

impl std::error::Error for HostsParseError {}

/// 表示 hosts 文件中的单条记录
#[derive(Debug, Clone, PartialEq)]
pub struct HostEntry {
    /// IP 地址
    pub ip: IpAddr,
    /// 主机名列表
    pub hostnames: Vec<String>,
    /// 行尾注释（不包含 # 符号）
    pub comment: Option<String>,
    /// 是否被注释掉（整行以 # 开头）
    pub is_commented: bool,
}

impl HostEntry {
    /// 创建新的 HostEntry
    ///
    /// # 参数
    /// - `ip` - IP 地址字符串
    /// - `hostnames` - 主机名列表
    ///
    /// # 示例
    /// ```
    /// use tidepool_hosts_manager::HostEntry;
    ///
    /// let entry = HostEntry::new("127.0.0.1", &["localhost", "local"]).unwrap();
    /// assert_eq!(entry.hostnames.len(), 2);
    /// ```
    pub fn new(ip: &str, hostnames: &[&str]) -> Result<Self, HostsParseError> {
        let ip_addr =
            IpAddr::from_str(ip).map_err(|_| HostsParseError::InvalidIpAddress(ip.to_string()))?;

        let hostname_list: Result<Vec<String>, HostsParseError> =
            hostnames.iter().map(|&h| Self::validate_hostname(h)).collect();

        Ok(HostEntry { ip: ip_addr, hostnames: hostname_list?, comment: None, is_commented: false })
    }

    /// 带注释创建 HostEntry
    pub fn with_comment(
        ip: &str,
        hostnames: &[&str],
        comment: &str,
    ) -> Result<Self, HostsParseError> {
        let mut entry = Self::new(ip, hostnames)?;
        entry.comment = Some(comment.to_string());
        Ok(entry)
    }

    /// 创建被注释的 HostEntry
    pub fn commented(ip: &str, hostnames: &[&str]) -> Result<Self, HostsParseError> {
        let mut entry = Self::new(ip, hostnames)?;
        entry.is_commented = true;
        Ok(entry)
    }

    /// 验证主机名格式
    fn validate_hostname(hostname: &str) -> Result<String, HostsParseError> {
        if hostname.is_empty() {
            return Err(HostsParseError::InvalidHostname("主机名不能为空".to_string()));
        }

        // 基本的主机名验证：不能包含空格和特殊字符
        if hostname.contains(char::is_whitespace) {
            return Err(HostsParseError::InvalidHostname(format!(
                "主机名不能包含空格: {}",
                hostname
            )));
        }

        Ok(hostname.to_string())
    }

    /// 是否为 IPv4 地址
    pub fn is_ipv4(&self) -> bool {
        matches!(self.ip, IpAddr::V4(_))
    }

    /// 是否为 IPv6 地址
    pub fn is_ipv6(&self) -> bool {
        matches!(self.ip, IpAddr::V6(_))
    }

    /// 是否为本地回环地址
    pub fn is_loopback(&self) -> bool {
        match self.ip {
            IpAddr::V4(ipv4) => ipv4.is_loopback(),
            IpAddr::V6(ipv6) => ipv6.is_loopback(),
        }
    }

    /// 检查是否包含指定的主机名
    pub fn contains_hostname(&self, hostname: &str) -> bool {
        self.hostnames.iter().any(|h| h == hostname)
    }

    /// 添加主机名
    pub fn add_hostname(&mut self, hostname: &str) -> Result<(), HostsParseError> {
        let validated = Self::validate_hostname(hostname)?;
        if !self.contains_hostname(&validated) {
            self.hostnames.push(validated);
        }
        Ok(())
    }

    /// 移除主机名
    pub fn remove_hostname(&mut self, hostname: &str) -> bool {
        let original_len = self.hostnames.len();
        self.hostnames.retain(|h| h != hostname);
        self.hostnames.len() != original_len
    }
}

impl FromStr for HostEntry {
    type Err = HostsParseError;

    /// 从字符串解析 HostEntry
    ///
    /// 支持的格式：
    /// - `127.0.0.1 localhost` - 基本格式
    /// - `127.0.0.1 localhost local # 注释` - 带注释
    /// - `# 127.0.0.1 localhost` - 被注释的行
    /// - `::1 localhost` - IPv6 格式
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let line = line.trim();

        // 空行处理
        if line.is_empty()
            || line.starts_with('#') && line.trim_start_matches('#').trim().is_empty()
        {
            return Err(HostsParseError::EmptyLine);
        }

        let mut is_commented = false;
        let working_line = if line.starts_with('#') {
            is_commented = true;
            line.trim_start_matches('#').trim()
        } else {
            line
        };

        // 分割注释部分
        let (content, comment) = if let Some(comment_pos) = working_line.find('#') {
            let content = working_line[..comment_pos].trim();
            let comment = working_line[comment_pos + 1..].trim();
            (content, if comment.is_empty() { None } else { Some(comment.to_string()) })
        } else {
            (working_line, None)
        };

        // 分割 IP 和主机名
        let parts: Vec<&str> = content.split_whitespace().collect();
        if parts.len() < 2 {
            return Err(HostsParseError::InvalidLineFormat(line.to_string()));
        }

        let ip_str = parts[0];
        let hostnames: Vec<&str> = parts[1..].to_vec();

        let ip = IpAddr::from_str(ip_str)
            .map_err(|_| HostsParseError::InvalidIpAddress(ip_str.to_string()))?;

        let validated_hostnames: Result<Vec<String>, HostsParseError> =
            hostnames.iter().map(|&h| Self::validate_hostname(h)).collect();

        Ok(HostEntry { ip, hostnames: validated_hostnames?, comment, is_commented })
    }
}

impl fmt::Display for HostEntry {
    /// 将 HostEntry 格式化为 hosts 文件格式的字符串
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prefix = if self.is_commented { "# " } else { "" };
        let hostnames = self.hostnames.join(" ");
        let comment = match &self.comment {
            Some(c) => format!(" # {}", c),
            None => String::new(),
        };

        write!(f, "{}{} {}{}", prefix, self.ip, hostnames, comment)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, Ipv6Addr};

    #[test]
    fn test_new_ipv4_entry() {
        let entry = HostEntry::new("127.0.0.1", &["localhost"]).unwrap();
        assert_eq!(entry.ip, IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
        assert_eq!(entry.hostnames, vec!["localhost"]);
        assert!(!entry.is_commented);
        assert!(entry.comment.is_none());
    }

    #[test]
    fn test_new_ipv6_entry() {
        let entry = HostEntry::new("::1", &["localhost"]).unwrap();
        assert_eq!(entry.ip, IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)));
        assert!(entry.is_ipv6());
        assert!(entry.is_loopback());
    }

    #[test]
    fn test_with_comment() {
        let entry = HostEntry::with_comment("127.0.0.1", &["localhost"], "本地回环").unwrap();
        assert_eq!(entry.comment, Some("本地回环".to_string()));
    }

    #[test]
    fn test_parse_basic_line() {
        let entry: HostEntry = "127.0.0.1 localhost".parse().unwrap();
        assert_eq!(entry.ip, IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
        assert_eq!(entry.hostnames, vec!["localhost"]);
    }

    #[test]
    fn test_parse_line_with_comment() {
        let entry: HostEntry = "127.0.0.1 localhost local # 本地主机".parse().unwrap();
        assert_eq!(entry.hostnames, vec!["localhost", "local"]);
        assert_eq!(entry.comment, Some("本地主机".to_string()));
    }

    #[test]
    fn test_parse_commented_line() {
        let entry: HostEntry = "# 192.168.1.1 router".parse().unwrap();
        assert!(entry.is_commented);
        assert_eq!(entry.ip, IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)));
        assert_eq!(entry.hostnames, vec!["router"]);
    }

    #[test]
    fn test_parse_empty_line() {
        let result: Result<HostEntry, _> = "".parse();
        assert!(matches!(result, Err(HostsParseError::EmptyLine)));
    }

    #[test]
    fn test_parse_comment_only_line() {
        let result: Result<HostEntry, _> = "# 这是一个注释".parse();
        assert!(matches!(result, Err(HostsParseError::InvalidLineFormat(_))));
    }

    #[test]
    fn test_invalid_ip() {
        let result = HostEntry::new("999.999.999.999", &["invalid"]);
        assert!(matches!(result, Err(HostsParseError::InvalidIpAddress(_))));
    }

    #[test]
    fn test_invalid_hostname() {
        let result = HostEntry::new("127.0.0.1", &["host with spaces"]);
        assert!(matches!(result, Err(HostsParseError::InvalidHostname(_))));
    }

    #[test]
    fn test_hostname_operations() {
        let mut entry = HostEntry::new("127.0.0.1", &["localhost"]).unwrap();

        assert!(entry.contains_hostname("localhost"));
        assert!(!entry.contains_hostname("other"));

        entry.add_hostname("local").unwrap();
        assert_eq!(entry.hostnames.len(), 2);
        assert!(entry.contains_hostname("local"));

        // 重复添加不应该增加数量
        entry.add_hostname("local").unwrap();
        assert_eq!(entry.hostnames.len(), 2);

        assert!(entry.remove_hostname("local"));
        assert_eq!(entry.hostnames.len(), 1);
        assert!(!entry.contains_hostname("local"));
    }

    #[test]
    fn test_display_format() {
        let entry = HostEntry::new("127.0.0.1", &["localhost", "local"]).unwrap();
        assert_eq!(entry.to_string(), "127.0.0.1 localhost local");

        let commented = HostEntry::commented("192.168.1.1", &["router"]).unwrap();
        assert_eq!(commented.to_string(), "# 192.168.1.1 router");

        let with_comment = HostEntry::with_comment("127.0.0.1", &["localhost"], "测试").unwrap();
        assert_eq!(with_comment.to_string(), "127.0.0.1 localhost # 测试");
    }

    #[test]
    fn test_ipv6_parsing() {
        let entry: HostEntry = "2001:db8::1 example.com".parse().unwrap();
        assert!(entry.is_ipv6());
        assert!(!entry.is_loopback());
        assert_eq!(entry.hostnames, vec!["example.com"]);
    }
}
