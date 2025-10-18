use crate::game_detection::models::*;
use std::path::PathBuf;

#[cfg(target_os = "windows")]
use winreg::enums::*;
#[cfg(target_os = "windows")]
use winreg::RegKey;

#[derive(Default)]
pub struct UplayDetector;

impl UplayDetector {
    pub fn new() -> Self {
        Self
    }

    /// Detect Ubisoft Connect (formerly Uplay) games
    pub async fn scan_games(&self) -> Result<Vec<DetectedGame>, String> {
        #[cfg(target_os = "windows")]
        {
            self.detect_uplay_from_registry().await
        }

        #[cfg(not(target_os = "windows"))]
        {
            Ok(Vec::new())
        }
    }

    #[cfg(target_os = "windows")]
    async fn detect_uplay_from_registry(&self) -> Result<Vec<DetectedGame>, String> {
        let mut games = Vec::new();

        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let installs_key = hklm
            .open_subkey(r"SOFTWARE\WOW6432Node\Ubisoft\Launcher\Installs")
            .map_err(|_| "Uplay registry key not found".to_string())?;

        // Collect all app IDs first to avoid holding the iterator across await
        let app_ids: Vec<String> = installs_key.enum_keys().filter_map(Result::ok).collect();

        for app_id in app_ids {
            if let Ok(game_key) = installs_key.open_subkey(&app_id) {
                if let Some(game) = self.parse_uplay_registry_entry(&game_key, &app_id) {
                    games.push(game);
                }
            }
        }

        Ok(games)
    }

    #[cfg(target_os = "windows")]
    fn parse_uplay_registry_entry(&self, key: &RegKey, app_id: &str) -> Option<DetectedGame> {
        let install_dir: PathBuf = key.get_value::<String, _>("InstallDir").ok()?.into();

        let game_name = key
            .get_value::<String, _>("DisplayName")
            .ok()
            .or_else(|| key.get_value::<String, _>("GameName").ok())
            .unwrap_or_else(|| format!("Uplay Game {}", app_id));

        let platform = GamePlatform::Uplay {
            app_id: app_id.to_string(),
        };

        Some(DetectedGame::new(
            app_id.to_string(),
            game_name,
            None,
            Some(install_dir),
            platform,
        ))
    }
}
