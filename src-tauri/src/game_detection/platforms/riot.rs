use crate::game_detection::models::*;
use crate::game_detection::utils::normalize_path_separators;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct RiotClientInstalls {
    #[serde(default)]
    associated_client: std::collections::HashMap<String, String>,
    #[serde(default)]
    patchlines: std::collections::HashMap<String, String>,
}

#[derive(Default)]
pub struct RiotDetector;

impl RiotDetector {
    pub fn new() -> Self {
        Self
    }

    /// Detect Riot Games client games
    pub async fn scan_games(&self) -> Result<Vec<DetectedGame>, String> {
        let riot_path = self.find_riot_client_path()?;

        if !riot_path.exists() {
            return Ok(Vec::new());
        }

        let config_path = riot_path.join("RiotClientInstalls.json");

        if config_path.exists() {
            self.parse_riot_client_installs(&config_path).await
        } else {
            Ok(Vec::new())
        }
    }

    #[cfg(target_os = "windows")]
    fn find_riot_client_path(&self) -> Result<PathBuf, String> {
        let program_data =
            std::env::var("ProgramData").map_err(|_| "ProgramData not set".to_string())?;
        Ok(PathBuf::from(program_data).join("Riot Games"))
    }

    #[cfg(target_os = "macos")]
    fn find_riot_client_path(&self) -> Result<PathBuf, String> {
        let home = std::env::var("HOME").map_err(|_| "HOME not set")?;
        Ok(PathBuf::from(home).join("Library/Application Support/Riot Games"))
    }

    async fn parse_riot_client_installs(
        &self,
        config_path: &PathBuf,
    ) -> Result<Vec<DetectedGame>, String> {
        let mut games = Vec::new();

        let content = tokio::fs::read_to_string(config_path)
            .await
            .map_err(|e| format!("Failed to read Riot config: {}", e))?;

        let installs: RiotClientInstalls = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse Riot config: {}", e))?;

        // Parse associated_client entries
        // Format: {"C:/Riot Games/VALORANT/live/": "C:/Riot Games/Riot Client/RiotClientServices.exe"}
        for (install_path_str, _riot_client_exe) in installs.associated_client {
            let install_path = PathBuf::from(&install_path_str);

            // Extract game name from path
            let game_name = self.extract_game_name_from_path(&install_path);

            if game_name.is_empty() {
                continue;
            }

            // Find the actual game executable
            let executable = self.find_riot_game_executable(&install_path, &game_name)
                .await
                .map(|p| normalize_path_separators(p));

            let platform = GamePlatform::RiotGames {
                app_name: game_name.clone(),
            };

            // Generate a game ID from the path
            let game_id = format!("riot_{}", game_name.to_lowercase().replace(" ", "_"));

            let mut game = DetectedGame::new(
                game_id.clone(),
                game_name.clone(),
                executable,
                Some(normalize_path_separators(&install_path_str)),
                platform,
            );

            game.platform_data
                .insert("app_id".to_string(), game_id);

            games.push(game);
        }

        Ok(games)
    }

    /// Extract game name from install path
    /// Example: "C:/Riot Games/VALORANT/live/" -> "VALORANT"
    fn extract_game_name_from_path(&self, path: &PathBuf) -> String {
        let path_str = path.to_string_lossy();

        // Pattern: "C:/Riot Games/VALORANT/live/" or "C:/Riot Games/League of Legends/"
        if let Some(after_riot_games) = path_str.split("Riot Games/").nth(1) {
            // Get the game directory name (first component after "Riot Games/")
            let game_component = after_riot_games
                .split('/')
                .next()
                .unwrap_or("")
                .trim();

            if !game_component.is_empty() {
                return game_component.to_string();
            }
        }

        String::new()
    }

    async fn find_riot_game_executable(
        &self,
        install_path: &PathBuf,
        game_name: &str,
    ) -> Option<PathBuf> {
        // Canonical executable names for Riot Games titles
        let (exe_names, subdirs): (Vec<&str>, Vec<&str>) = match game_name {
            "VALORANT" => (vec!["VALORANT.exe"], vec!["", "live"]),
            "League of Legends" => (vec!["LeagueClient.exe", "League of Legends.exe"], vec!["", "Game"]),
            "Legends of Runeterra" => (vec!["LoR.exe"], vec![""]),
            "Teamfight Tactics" => (vec!["TeamfightTactics.exe"], vec![""]),
            _ => (vec![], vec![""]),
        };

        // Check each possible location
        for subdir in &subdirs {
            let search_dir = if subdir.is_empty() {
                install_path.clone()
            } else {
                install_path.join(subdir)
            };

            for exe_name in &exe_names {
                let exe_path = search_dir.join(exe_name);
                if exe_path.exists() {
                    return Some(exe_path);
                }
            }
        }

        None
    }
}
