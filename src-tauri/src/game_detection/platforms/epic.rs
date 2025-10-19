use crate::game_detection::models::*;
use crate::game_detection::utils::normalize_path_separators;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct EpicManifest {
    app_name: String,
    display_name: String,
    install_location: String,
    #[serde(default)]
    launch_executable: String,
    #[serde(default)]
    catalog_item_id: String,
    #[serde(default)]
    main_game_catalog_item_id: String,
    #[serde(default)]
    app_categories: Vec<String>,
}

#[derive(Default)]
pub struct EpicDetector;

impl EpicDetector {
    pub fn new() -> Self {
        Self
    }

    /// Detect Epic Games Store games
    pub async fn scan_games(&self) -> Result<Vec<DetectedGame>, String> {
        let mut games = Vec::new();

        let manifest_dir = self.find_epic_manifests_path()?;

        if !manifest_dir.exists() {
            return Ok(games);
        }

        // Read all .item manifest files
        let mut entries = tokio::fs::read_dir(&manifest_dir)
            .await
            .map_err(|e| format!("Failed to read manifests directory: {}", e))?;

        while let Ok(Some(entry)) = entries.next_entry().await {
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("item") {
                if let Ok(game) = self.parse_epic_manifest(&path).await {
                    games.push(game);
                }
            }
        }

        Ok(games)
    }

    #[cfg(target_os = "windows")]
    fn find_epic_manifests_path(&self) -> Result<PathBuf, String> {
        let program_data = std::env::var("ProgramData")
            .map_err(|_| "ProgramData environment variable not set")?;
        Ok(PathBuf::from(program_data).join("Epic/EpicGamesLauncher/Data/Manifests"))
    }

    #[cfg(target_os = "macos")]
    fn find_epic_manifests_path(&self) -> Result<PathBuf, String> {
        let home = std::env::var("HOME").map_err(|_| "HOME not set")?;
        Ok(PathBuf::from(home)
            .join("Library/Application Support/Epic/EpicGamesLauncher/Data/Manifests"))
    }

    #[cfg(target_os = "linux")]
    fn find_epic_manifests_path(&self) -> Result<PathBuf, String> {
        let home = std::env::var("HOME").map_err(|_| "HOME not set")?;
        Ok(PathBuf::from(home).join(".config/Epic/EpicGamesLauncher/Data/Manifests"))
    }

    /// Check if a manifest represents DLC rather than a base game
    fn is_dlc(manifest: &EpicManifest) -> bool {
        // DLC items have a MainGameCatalogItemId that differs from their own CatalogItemId
        // Base games have MainGameCatalogItemId == CatalogItemId (self-referencing)
        if !manifest.main_game_catalog_item_id.is_empty()
            && !manifest.catalog_item_id.is_empty()
            && manifest.main_game_catalog_item_id != manifest.catalog_item_id
        {
            return true;
        }

        // Additionally check AppCategories for "addons" or "dlc"
        manifest.app_categories.iter().any(|cat| {
            let cat_lower = cat.to_lowercase();
            cat_lower.contains("addon") || cat_lower.contains("dlc")
        })
    }

    async fn parse_epic_manifest(&self, manifest_path: &PathBuf) -> Result<DetectedGame, String> {
        let content = tokio::fs::read_to_string(manifest_path)
            .await
            .map_err(|e| format!("Failed to read manifest: {}", e))?;

        let manifest: EpicManifest = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse manifest: {}", e))?;

        // Skip DLC items
        if Self::is_dlc(&manifest) {
            return Err("Skipping DLC item".to_string());
        }

        let install_path = Some(normalize_path_separators(&manifest.install_location));
        let executable_path = if !manifest.launch_executable.is_empty() {
            Some(normalize_path_separators(
                PathBuf::from(&manifest.install_location).join(&manifest.launch_executable),
            ))
        } else {
            None
        };

        let platform = GamePlatform::EpicGames {
            app_name: manifest.app_name.clone(),
        };

        let game = DetectedGame::new(
            manifest.app_name.clone(),
            manifest.display_name,
            executable_path,
            install_path,
            platform,
        );

        Ok(game)
    }
}
