use crate::game_detection::models::*;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

#[cfg(target_os = "windows")]
use winreg::enums::*;
#[cfg(target_os = "windows")]
use winreg::RegKey;

#[derive(Default)]
pub struct SteamDetector;

impl SteamDetector {
    pub fn new() -> Self {
        Self
    }

    /// Scan for Steam games using manifest files
    pub async fn scan_games(&self) -> Result<Vec<DetectedGame>, String> {
        let mut games = Vec::new();

        // Find Steam installation
        let steam_path = self.find_steam_path()?;

        // Parse library folders
        let library_folders = self.parse_library_folders(&steam_path)?;

        // Scan each library folder for installed games
        for library in library_folders {
            if let Ok(library_games) = self.scan_steam_library(&library).await {
                games.extend(library_games);
            }
        }

        Ok(games)
    }

    #[cfg(target_os = "windows")]
    fn find_steam_path(&self) -> Result<PathBuf, String> {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        if let Ok(steam_key) = hklm.open_subkey(r"SOFTWARE\WOW6432Node\Valve\Steam") {
            if let Ok(install_path) = steam_key.get_value::<String, _>("InstallPath") {
                return Ok(PathBuf::from(install_path));
            }
        }

        Err("Steam installation not found".to_string())
    }

    #[cfg(target_os = "macos")]
    fn find_steam_path(&self) -> Result<PathBuf, String> {
        let home = std::env::var("HOME").map_err(|_| "HOME not set")?;
        let steam_path = PathBuf::from(home).join("Library/Application Support/Steam");

        if steam_path.exists() {
            Ok(steam_path)
        } else {
            Err("Steam installation not found".to_string())
        }
    }

    #[cfg(target_os = "linux")]
    fn find_steam_path(&self) -> Result<PathBuf, String> {
        let home = std::env::var("HOME").map_err(|_| "HOME not set")?;
        let steam_path = PathBuf::from(home).join(".steam/steam");

        if steam_path.exists() {
            Ok(steam_path)
        } else {
            Err("Steam installation not found".to_string())
        }
    }

    fn parse_library_folders(&self, steam_path: &Path) -> Result<Vec<PathBuf>, String> {
        let mut folders_set: HashSet<PathBuf> = HashSet::new();

        // Add the main Steam path
        folders_set.insert(steam_path.to_path_buf());

        let library_vdf = steam_path.join("steamapps/libraryfolders.vdf");

        if !library_vdf.exists() {
            return Ok(folders_set.into_iter().collect());
        }

        let content = std::fs::read_to_string(&library_vdf)
            .map_err(|e| format!("Failed to read library folders: {}", e))?;

        // Parse VDF format to find additional library paths
        // Note: libraryfolders.vdf often includes the main Steam path, so we use a HashSet to deduplicate
        for line in content.lines() {
            if line.contains("\"path\"") {
                if let Some(path_str) = self.extract_vdf_value(line) {
                    let path = PathBuf::from(path_str);
                    if path.exists() {
                        folders_set.insert(path);
                    }
                }
            }
        }

        Ok(folders_set.into_iter().collect())
    }

    async fn scan_steam_library(&self, library_path: &Path) -> Result<Vec<DetectedGame>, String> {
        let mut games = Vec::new();
        let steamapps = library_path.join("steamapps");

        if !steamapps.exists() {
            return Ok(games);
        }

        // Read all .acf manifest files
        let mut entries = tokio::fs::read_dir(&steamapps)
            .await
            .map_err(|e| format!("Failed to read steamapps: {}", e))?;

        while let Ok(Some(entry)) = entries.next_entry().await {
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("acf") {
                if let Ok(game) = self.parse_app_manifest(&path).await {
                    games.push(game);
                }
            }
        }

        Ok(games)
    }

    async fn parse_app_manifest(&self, manifest_path: &Path) -> Result<DetectedGame, String> {
        let content = tokio::fs::read_to_string(manifest_path)
            .await
            .map_err(|e| format!("Failed to read manifest: {}", e))?;

        let mut app_id = None;
        let mut name = None;
        let mut install_dir = None;

        for line in content.lines() {
            if line.contains("\"appid\"") {
                app_id = self.extract_vdf_value(line);
            } else if line.contains("\"name\"") {
                name = self.extract_vdf_value(line);
            } else if line.contains("\"installdir\"") {
                install_dir = self.extract_vdf_value(line);
            }
        }

        let app_id = app_id.ok_or("Missing appid")?;
        let name = name.ok_or("Missing name")?;
        let install_dir = install_dir.ok_or("Missing install directory")?;

        // Construct the full install path
        let install_path = manifest_path
            .parent()
            .and_then(|p| Some(p.join("common").join(&install_dir)))
            .filter(|p| p.exists());

        // Try to find the executable
        let executable_path = if let Some(ref path) = install_path {
            self.find_game_executable(path).await
        } else {
            None
        };

        let platform = GamePlatform::Steam {
            app_id: app_id.clone(),
        };

        Ok(DetectedGame::new(
            app_id,
            name,
            executable_path,
            install_path,
            platform,
        ))
    }

    async fn find_game_executable(&self, game_path: &Path) -> Option<PathBuf> {
        if !game_path.exists() {
            return None;
        }

        let mut candidates = Vec::new();

        // Look for .exe files in the game directory (Windows)
        #[cfg(target_os = "windows")]
        {
            if let Ok(mut entries) = tokio::fs::read_dir(game_path).await {
                while let Ok(Some(entry)) = entries.next_entry().await {
                    let path = entry.path();
                    if path.extension().and_then(|e| e.to_str()) == Some("exe") {
                        candidates.push(path);
                    }
                }
            }

            // Check common subdirectories
            if candidates.is_empty() {
                let subdirs = ["bin", "Bin", "Binaries", "Game", "game"];
                for subdir in &subdirs {
                    let subdir_path = game_path.join(subdir);
                    if let Ok(mut entries) = tokio::fs::read_dir(&subdir_path).await {
                        while let Ok(Some(entry)) = entries.next_entry().await {
                            let path = entry.path();
                            if path.extension().and_then(|e| e.to_str()) == Some("exe") {
                                candidates.push(path);
                            }
                        }
                    }
                }
            }
        }

        // For macOS/Linux, look for executables differently
        #[cfg(not(target_os = "windows"))]
        {
            // Look for .app bundles on macOS
            #[cfg(target_os = "macos")]
            {
                if let Ok(mut entries) = tokio::fs::read_dir(game_path).await {
                    while let Ok(Some(entry)) = entries.next_entry().await {
                        let path = entry.path();
                        if path.extension().and_then(|e| e.to_str()) == Some("app") {
                            return Some(path);
                        }
                    }
                }
            }

            // Look for executable files on Linux
            #[cfg(target_os = "linux")]
            {
                if let Ok(mut entries) = tokio::fs::read_dir(game_path).await {
                    while let Ok(Some(entry)) = entries.next_entry().await {
                        let path = entry.path();
                        if let Ok(metadata) = entry.metadata().await {
                            if metadata.is_file() && metadata.permissions().mode() & 0o111 != 0 {
                                candidates.push(path);
                            }
                        }
                    }
                }
            }
        }

        // Return the most likely candidate (prefer files with game name in them)
        candidates.into_iter().next()
    }

    fn extract_vdf_value(&self, line: &str) -> Option<String> {
        // Extract value from VDF format: "key" "value"
        let parts: Vec<&str> = line.split('"').collect();
        if parts.len() >= 4 {
            Some(parts[3].to_string())
        } else {
            None
        }
    }
}
