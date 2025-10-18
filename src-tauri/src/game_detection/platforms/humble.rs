use crate::game_detection::models::*;
use std::path::PathBuf;

#[derive(Default)]
pub struct HumbleDetector;

impl HumbleDetector {
    pub fn new() -> Self {
        Self
    }

    /// Detect Humble App games
    pub async fn scan_games(&self) -> Result<Vec<DetectedGame>, String> {
        let humble_path = self.find_humble_app_path()?;

        if !humble_path.exists() {
            return Ok(Vec::new());
        }

        let games_dir = humble_path.join("games");

        if games_dir.exists() {
            self.scan_humble_games_dir(&games_dir).await
        } else {
            Ok(Vec::new())
        }
    }

    #[cfg(target_os = "windows")]
    fn find_humble_app_path(&self) -> Result<PathBuf, String> {
        let local_app_data =
            std::env::var("LOCALAPPDATA").map_err(|_| "LOCALAPPDATA not set".to_string())?;
        Ok(PathBuf::from(local_app_data).join("Humble App"))
    }

    #[cfg(target_os = "macos")]
    fn find_humble_app_path(&self) -> Result<PathBuf, String> {
        let home = std::env::var("HOME").map_err(|_| "HOME not set")?;
        Ok(PathBuf::from(home).join("Library/Application Support/Humble App"))
    }

    #[cfg(target_os = "linux")]
    fn find_humble_app_path(&self) -> Result<PathBuf, String> {
        let home = std::env::var("HOME").map_err(|_| "HOME not set")?;
        Ok(PathBuf::from(home).join(".config/Humble App"))
    }

    async fn scan_humble_games_dir(&self, games_dir: &PathBuf) -> Result<Vec<DetectedGame>, String> {
        let mut games = Vec::new();

        let mut entries = tokio::fs::read_dir(games_dir)
            .await
            .map_err(|e| format!("Failed to read games directory: {}", e))?;

        while let Ok(Some(entry)) = entries.next_entry().await {
            let path = entry.path();

            if path.is_dir() {
                if let Ok(game) = self.parse_humble_game_dir(&path).await {
                    games.push(game);
                }
            }
        }

        Ok(games)
    }

    async fn parse_humble_game_dir(&self, game_dir: &PathBuf) -> Result<DetectedGame, String> {
        let metadata_path = game_dir.join("metadata.json");

        if !metadata_path.exists() {
            return Err("No metadata found".to_string());
        }

        let content = tokio::fs::read_to_string(&metadata_path)
            .await
            .map_err(|e| format!("Failed to read metadata: {}", e))?;

        let metadata_json: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse metadata: {}", e))?;

        let game_name = metadata_json["name"]
            .as_str()
            .or_else(|| metadata_json["gameName"].as_str())
            .unwrap_or("Unknown Humble Game")
            .to_string();

        let game_id = game_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        let install_path = metadata_json["installPath"]
            .as_str()
            .or_else(|| metadata_json["install_path"].as_str())
            .map(PathBuf::from);

        let executable = metadata_json["executable"]
            .as_str()
            .map(PathBuf::from);

        let platform = GamePlatform::HumbleApp {
            game_id: game_id.clone(),
        };

        Ok(DetectedGame::new(
            game_id,
            game_name,
            executable,
            install_path,
            platform,
        ))
    }
}
