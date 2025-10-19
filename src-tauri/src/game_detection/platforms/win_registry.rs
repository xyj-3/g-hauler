use crate::game_detection::models::*;
use std::path::PathBuf;

#[cfg(target_os = "windows")]
use winreg::enums::*;
#[cfg(target_os = "windows")]
use winreg::RegKey;

#[derive(Default)]
pub struct RegistryDetector;

impl RegistryDetector {
    pub fn new() -> Self {
        Self
    }

    /// Detect games via Windows Registry (Windows only)
    #[cfg(target_os = "windows")]
    pub async fn scan_games(&self) -> Result<Vec<DetectedGame>, String> {
        let mut games = Vec::new();

        let registry_paths = vec![
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall",
            r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall",
        ];

        for reg_path in registry_paths {
            if let Ok(detected) = self.scan_registry_path(reg_path).await {
                games.extend(detected);
            }
        }

        Ok(games)
    }

    #[cfg(not(target_os = "windows"))]
    pub async fn scan_games(&self) -> Result<Vec<DetectedGame>, String> {
        Ok(Vec::new())
    }

    #[cfg(target_os = "windows")]
    async fn scan_registry_path(&self, path: &str) -> Result<Vec<DetectedGame>, String> {
        let mut games = Vec::new();

        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let uninstall_key = hklm
            .open_subkey(path)
            .map_err(|e| format!("Failed to open registry key: {}", e))?;

        // Collect all subkey names first to avoid holding the iterator across await
        let subkey_names: Vec<String> = uninstall_key.enum_keys().filter_map(Result::ok).collect();

        for subkey_name in subkey_names {
            if let Ok(subkey) = uninstall_key.open_subkey(&subkey_name) {
                if let Some(game) = self.parse_registry_entry(&subkey, &subkey_name) {
                    games.push(game);
                }
            }
        }

        Ok(games)
    }

    #[cfg(target_os = "windows")]
    fn parse_registry_entry(&self, key: &RegKey, key_name: &str) -> Option<DetectedGame> {
        let display_name: String = key.get_value("DisplayName").ok()?;

        // Filter out non-game entries
        if self.is_likely_system_software(&display_name) {
            return None;
        }

        let install_location: Option<PathBuf> = key
            .get_value::<String, _>("InstallLocation")
            .ok()
            .map(PathBuf::from);

        let executable: Option<PathBuf> = key
            .get_value::<String, _>("DisplayIcon")
            .ok()
            .or_else(|| key.get_value::<String, _>("UninstallString").ok())
            .and_then(|s| self.extract_executable_from_path(&s))
            .map(PathBuf::from);

        let publisher: Option<String> = key.get_value("Publisher").ok();
        let version: Option<String> = key.get_value("DisplayVersion").ok();

        let platform = GamePlatform::WinRegistry {
            registry_key: key_name.to_string(),
        };

        let mut game = DetectedGame::new(
            key_name.to_string(),
            display_name,
            executable,
            install_location,
            platform,
        );

        if let Some(pub_name) = publisher {
            game.platform_data
                .insert("publisher".to_string(), pub_name);
        }

        if let Some(ver) = version {
            game.platform_data
                .insert("version".to_string(), ver);
        }

        Some(game)
    }

    #[cfg(target_os = "windows")]
    fn is_likely_system_software(&self, name: &str) -> bool {
        let system_keywords = [
            "Microsoft Visual C++",
            "Microsoft .NET",
            "Windows Update",
            "Driver Update",
            "Redistributable",
            ".NET Framework",
            "DirectX Runtime",
            "DirectX for Windows",
        ];

        system_keywords.iter().any(|keyword| name.contains(keyword))
            || (name.starts_with("Update for") && name.contains("Microsoft"))
            || (name.contains("KB") && name.contains("Windows"))
    }

    #[cfg(target_os = "windows")]
    fn extract_executable_from_path(&self, path: &str) -> Option<String> {
        let cleaned = path.trim_matches('"');

        if cleaned.to_lowercase().ends_with(".exe") {
            if let Some(exe_end) = cleaned.to_lowercase().find(".exe") {
                return Some(cleaned[..exe_end + 4].to_string());
            }
            return Some(cleaned.to_string());
        }

        None
    }
}
