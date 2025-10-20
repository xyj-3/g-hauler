use crate::game_detection::models::*;

#[cfg(target_os = "macos")]
use plist::Value;
#[cfg(target_os = "macos")]
use std::path::PathBuf;

#[derive(Default)]
pub struct OsxDetector;

impl OsxDetector {
    pub fn new() -> Self {
        Self
    }

    /// Detect macOS application bundles
    pub async fn scan_games(&self) -> Result<Vec<DetectedGame>, String> {
        #[cfg(target_os = "macos")]
        {
            let mut games = Vec::new();

            let search_paths = vec![
                PathBuf::from("/Applications"),
                PathBuf::from(std::env::var("HOME").unwrap_or_default()).join("Applications"),
            ];

            for search_path in search_paths {
                if search_path.exists() {
                    if let Ok(detected) = self.scan_applications_directory(&search_path).await {
                        games.extend(detected);
                    }
                }
            }

            Ok(games)
        }

        #[cfg(not(target_os = "macos"))]
        {
            Ok(Vec::new())
        }
    }

    #[cfg(target_os = "macos")]
    async fn scan_applications_directory(
        &self,
        apps_dir: &PathBuf,
    ) -> Result<Vec<DetectedGame>, String> {
        let mut games = Vec::new();

        let mut entries = tokio::fs::read_dir(apps_dir)
            .await
            .map_err(|e| format!("Failed to read applications directory: {}", e))?;

        while let Ok(Some(entry)) = entries.next_entry().await {
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("app") {
                if let Ok(game) = self.parse_app_bundle(&path).await {
                    if self.is_likely_game(&game) {
                        games.push(game);
                    }
                }
            }
        }

        Ok(games)
    }

    #[cfg(target_os = "macos")]
    async fn parse_app_bundle(&self, bundle_path: &PathBuf) -> Result<DetectedGame, String> {
        let info_plist_path = bundle_path.join("Contents/Info.plist");

        if !info_plist_path.exists() {
            return Err("Info.plist not found".to_string());
        }

        let plist = Value::from_file(&info_plist_path)
            .map_err(|e| format!("Failed to read Info.plist: {}", e))?;

        let bundle_id = plist
            .as_dictionary()
            .and_then(|d| d.get("CFBundleIdentifier"))
            .and_then(|v| v.as_string())
            .ok_or("Missing CFBundleIdentifier")?
            .to_string();

        let bundle_name = plist
            .as_dictionary()
            .and_then(|d| d.get("CFBundleDisplayName").or_else(|| d.get("CFBundleName")))
            .and_then(|v| v.as_string())
            .ok_or("Missing bundle name")?
            .to_string();

        let executable = plist
            .as_dictionary()
            .and_then(|d| d.get("CFBundleExecutable"))
            .and_then(|v| v.as_string())
            .map(|exe_name| bundle_path.join("Contents/MacOS").join(exe_name));

        let version = plist
            .as_dictionary()
            .and_then(|d| d.get("CFBundleShortVersionString"))
            .and_then(|v| v.as_string())
            .map(|s| s.to_string());

        let platform = GamePlatform::OsxBundle {
            bundle_id: bundle_id.clone(),
        };

        let mut game = DetectedGame::new(
            bundle_id.clone(),
            bundle_name,
            executable,
            Some(bundle_path.clone()),
            platform,
        );

        if let Some(ver) = version {
            game.platform_data
                .insert("version".to_string(), ver);
        }

        Ok(game)
    }

    #[cfg(target_os = "macos")]
    fn is_likely_game(&self, game: &DetectedGame) -> bool {
        // Expanded list of game-related indicators
        let game_indicators = [
            "steam", "epic", "blizzard", "ubisoft", "ea", "game", "unity", "unreal",
            "riot", "gog", "itch", "origin", "bethesda", "rockstar",
        ];

        let name_lower = game.name.to_lowercase();
        let id_lower = game.id.to_lowercase();

        // Check for game indicators in name or ID
        if game_indicators
            .iter()
            .any(|indicator| name_lower.contains(indicator) || id_lower.contains(indicator))
        {
            return true;
        }

        // Exclude common system and utility apps
        let system_apps = [
            "com.apple.",
            "com.microsoft.",
            "com.adobe.",
            "com.google.",
            "utility",
            "settings",
            "system",
            "preferences",
        ];

        if system_apps
            .iter()
            .any(|prefix| id_lower.starts_with(prefix) || name_lower.contains(prefix))
        {
            return false;
        }

        // Include apps that are likely games (generic approach)
        // Apps in common game directories or with game-like characteristics
        true
    }
}
