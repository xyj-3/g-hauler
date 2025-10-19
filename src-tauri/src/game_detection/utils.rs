use std::path::{Path, PathBuf};

/// Normalizes path separators based on the target platform.
///
/// On Windows, converts forward slashes to backslashes and removes duplicate backslashes.
/// On Unix-like systems (macOS, Linux), converts backslashes to forward slashes.
///
/// # Arguments
/// * `path` - The path string to normalize
///
/// # Returns
/// A PathBuf with normalized separators for the current platform
pub fn normalize_path_separators<P: AsRef<Path>>(path: P) -> PathBuf {
    let path_str = path.as_ref().to_string_lossy();

    #[cfg(target_os = "windows")]
    {
        // On Windows, convert forward slashes to backslashes and remove duplicate backslashes
        let normalized = path_str
            .replace('/', "\\")
            .split('\\')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("\\");
        PathBuf::from(normalized)
    }

    #[cfg(not(target_os = "windows"))]
    {
        // On Unix-like systems, convert backslashes to forward slashes
        let normalized = path_str.replace('\\', "/");
        PathBuf::from(normalized)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(target_os = "windows")]
    fn test_normalize_windows_paths() {
        // Test mixed slashes
        assert_eq!(
            normalize_path_separators("C:/Program Files/Game"),
            PathBuf::from("C:\\Program Files\\Game")
        );

        // Test double backslashes
        assert_eq!(
            normalize_path_separators("C:\\\\Program Files\\\\Game"),
            PathBuf::from("C:\\Program Files\\Game")
        );

        // Test already normalized
        assert_eq!(
            normalize_path_separators("C:\\Program Files\\Game"),
            PathBuf::from("C:\\Program Files\\Game")
        );

        // Test mixed double and single
        assert_eq!(
            normalize_path_separators("C:\\\\Program Files/Game\\Folder"),
            PathBuf::from("C:\\Program Files\\Game\\Folder")
        );
    }

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn test_normalize_unix_paths() {
        // Test backslashes to forward slashes
        assert_eq!(
            normalize_path_separators("/Users\\username\\Games"),
            PathBuf::from("/Users/username/Games")
        );

        // Test already normalized
        assert_eq!(
            normalize_path_separators("/Users/username/Games"),
            PathBuf::from("/Users/username/Games")
        );
    }
}
