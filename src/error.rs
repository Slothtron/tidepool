//! Unified error handling module
//!
//! Provides a unified error type and handling for the project.

/// Unified Result type for the project
pub type Result<T, E = anyhow::Error> = std::result::Result<T, E>;

/// Common error creation macro
#[macro_export]
macro_rules! err {
    ($($arg:tt)*) => { anyhow::anyhow!($($arg)*) };
}

/// Error handling utility functions
pub struct ErrorUtils;

impl ErrorUtils {
    /// Converts an IO error to a user-friendly message
    pub fn io_error_to_message(err: &std::io::Error) -> String {
        match err.kind() {
            std::io::ErrorKind::NotFound => "File or directory not found".to_string(),
            std::io::ErrorKind::PermissionDenied => "Permission denied".to_string(),
            std::io::ErrorKind::AlreadyExists => "File or directory already exists".to_string(),
            std::io::ErrorKind::InvalidInput => "Invalid input".to_string(),
            _ => format!("IO Error: {err}"),
        }
    }

    /// Converts a network error to a user-friendly message
    pub fn network_error_to_message(err: &reqwest::Error) -> String {
        if err.is_timeout() {
            "Network request timed out".to_string()
        } else if err.is_connect() {
            "Could not connect to the server".to_string()
        } else if err.is_status() {
            format!(
                "Server returned an error: {}",
                err.status().map_or("Unknown".to_string(), |s| s.to_string())
            )
        } else {
            format!("Network error: {err}")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_io_error_messages() {
        let err = io::Error::new(io::ErrorKind::NotFound, "not found");
        assert_eq!(ErrorUtils::io_error_to_message(&err), "File or directory not found");

        let err = io::Error::new(io::ErrorKind::PermissionDenied, "permission denied");
        assert_eq!(ErrorUtils::io_error_to_message(&err), "Permission denied");
    }
}
