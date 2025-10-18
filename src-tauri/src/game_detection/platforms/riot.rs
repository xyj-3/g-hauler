use crate::game_detection::models::*;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct RiotClientInstalls {
    #[serde(default)]
    associated_client: Vec<AssociatedClient>,
}

#[derive(Debug, Deserialize)]
struct AssociatedClient {
    #[serde(default)]
    name: String,
    #[serde(default)]
    id: String,
    #[serde(default)]
    install_location: String,
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

    #[cfg(target_os = "linux")]
    fn find_riot_client_path(&self) -> Result<PathBuf, String> {
        Err("Riot Games detection not implemented for Linux".to_string())
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

        for client in installs.associated_client {
            if client.name.is_empty() {
                continue;
            }

            let install_path = if !client.install_location.is_empty() {
                Some(PathBuf::from(&client.install_location))
            } else {
                None
            };

            let executable = if let Some(ref path) = install_path {
                self.find_riot_game_executable(path, &client.name).await
            } else {
                None
            };

            let platform = GamePlatform::RiotGames {
                app_name: client.name.clone(),
            };

            let mut game = DetectedGame::new(
                client.id.clone(),
                client.name.clone(),
                executable,
                install_path,
                platform,
            );

            if !client.id.is_empty() {
                game.platform_data
                    .insert("app_id".to_string(), client.id);
            }

            games.push(game);
        }

        Ok(games)
    }

    async fn find_riot_game_executable(
        &self,
        install_path: &PathBuf,
        game_name: &str,
    ) -> Option<PathBuf> {
        let possible_exes = match game_name.to_lowercase().as_str() {
            name if name.contains("league") => vec!["LeagueClient.exe", "League of Legends.exe"],
            name if name.contains("valorant") => vec!["VALORANT.exe", "Valorant.exe"],
            name if name.contains("legends of runeterra") => vec!["LoR.exe"],
            name if name.contains("teamfight tactics") => vec!["TeamfightTactics.exe"],
            _ => vec![],
        };

        for exe_name in possible_exes {
            let exe_path = install_path.join(exe_name);
            if exe_path.exists() {
                return Some(exe_path);
            }

            for subdir in &["Game", "live"] {
                let exe_path = install_path.join(subdir).join(exe_name);
                if exe_path.exists() {
                    return Some(exe_path);
                }
            }
        }

        None
    }
}
