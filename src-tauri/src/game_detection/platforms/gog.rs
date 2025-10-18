use crate::game_detection::models::*;
use std::path::PathBuf;

#[cfg(target_os = "windows")]
use winreg::enums::*;
#[cfg(target_os = "windows")]
use winreg::RegKey;

#[derive(Default)]
pub struct GogDetector;

impl GogDetector {
    pub fn new() -> Self {
        Self
    }

    /// Detect GOG Galaxy games
    pub async fn scan_games(&self) -> Result<Vec<DetectedGame>, String> {
        #[cfg(target_os = "windows")]
        {
            self.detect_gog_from_registry().await
        }

        #[cfg(not(target_os = "windows"))]
        {
            Ok(Vec::new())
        }
    }

    #[cfg(target_os = "windows")]
    async fn detect_gog_from_registry(&self) -> Result<Vec<DetectedGame>, String> {
        let mut games = Vec::new();

        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let gog_key = hklm
            .open_subkey(r"SOFTWARE\WOW6432Node\GOG.com\Games")
            .map_err(|_| "GOG Galaxy registry key not found".to_string())?;

        // Collect all product IDs first to avoid holding the iterator across await
        let product_ids: Vec<String> = gog_key.enum_keys().filter_map(Result::ok).collect();

        for product_id in product_ids {
            if let Ok(game_key) = gog_key.open_subkey(&product_id) {
                if let Some(game) = self.parse_gog_registry_entry(&game_key, &product_id) {
                    games.push(game);
                }
            }
        }

        Ok(games)
    }

    #[cfg(target_os = "windows")]
    fn parse_gog_registry_entry(&self, key: &RegKey, product_id: &str) -> Option<DetectedGame> {
        let game_name: String = key.get_value("gameName").ok()?;
        let path: PathBuf = key.get_value::<String, _>("path").ok()?.into();
        let exe: Option<String> = key.get_value("exe").ok();

        let executable_path = exe.as_ref().map(|e| path.join(e));

        let platform = GamePlatform::GogGalaxy {
            product_id: product_id.to_string(),
        };

        let mut game = DetectedGame::new(
            product_id.to_string(),
            game_name,
            executable_path,
            Some(path),
            platform,
        );

        if let Ok(version) = key.get_value::<String, _>("version") {
            game.platform_data
                .insert("version".to_string(), version);
        }

        Some(game)
    }
}
