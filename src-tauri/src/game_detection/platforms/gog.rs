use crate::game_detection::models::*;
use crate::game_detection::utils::normalize_path_separators;

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

        // Check for installed games registry key
        // This key only exists when GOG games are installed
        let gog_key = match hklm.open_subkey(r"SOFTWARE\WOW6432Node\GOG.com\Games") {
            Ok(key) => key,
            Err(_) => {
                // GOG Galaxy not installed or no games are installed
                return Ok(Vec::new());
            }
        };

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
        let path_str: String = key.get_value("path").ok()?;
        let path = normalize_path_separators(&path_str);
        let exe: Option<String> = key.get_value("exe").ok();

        let executable_path = exe.as_ref().map(|e| normalize_path_separators(path.join(e)));

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
