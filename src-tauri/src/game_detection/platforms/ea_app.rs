use crate::game_detection::models::*;
use crate::game_detection::utils::normalize_path_separators;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EaInstallationDb {
    install_infos: Vec<EaInstallInfo>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EaInstallInfo {
    base_install_path: String,
    base_slug: String,
    software_id: String,
    #[serde(default)]
    installed_version: String,
    #[serde(default)]
    executable_check: String,
    #[serde(default)]
    content_manifest_launchers: String,
    #[serde(default)]
    dlc_sub_path: String,
    detailed_state: DetailedState,
    #[serde(default)]
    local_install_properties: LocalInstallProperties,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct LocalInstallProperties {
    #[serde(default)]
    launchers: Vec<Launcher>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Launcher {
    exe_path: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DetailedState {
    install_status: u32,
}

#[derive(Default)]
pub struct EaAppDetector;

impl EaAppDetector {
    pub fn new() -> Self {
        Self
    }

    /// Detect EA App (formerly Origin) games
    pub async fn scan_games(&self) -> Result<Vec<DetectedGame>, String> {
        let mut games = Vec::new();

        let ea_desktop_dir = self.find_ea_desktop_path()?;

        if !ea_desktop_dir.exists() {
            // Try legacy Origin path as fallback
            return self.scan_legacy_origin().await;
        }

        // Read all subdirectories in EA Desktop
        let mut entries = tokio::fs::read_dir(&ea_desktop_dir)
            .await
            .map_err(|e| format!("Failed to read EA Desktop directory: {}", e))?;

        while let Ok(Some(entry)) = entries.next_entry().await {
            let path = entry.path();

            // Each installation database is in a hashed directory
            if path.is_dir() {
                let is_json = path.join("IS.json");
                if is_json.exists() {
                    if let Ok(detected_games) = self.parse_ea_desktop_db(&is_json).await {
                        games.extend(detected_games);
                    }
                }
            }
        }

        Ok(games)
    }

    #[cfg(target_os = "windows")]
    fn find_ea_desktop_path(&self) -> Result<PathBuf, String> {
        let program_data = std::env::var("ProgramData")
            .map_err(|_| "ProgramData environment variable not set")?;
        Ok(PathBuf::from(program_data).join("EA Desktop"))
    }

    #[cfg(target_os = "macos")]
    fn find_ea_desktop_path(&self) -> Result<PathBuf, String> {
        let home = std::env::var("HOME").map_err(|_| "HOME not set")?;
        Ok(PathBuf::from(home).join("Library/Application Support/EA Desktop"))
    }

    /// Check if an install info entry represents a DLC rather than a base game
    fn is_dlc(&self, install_info: &EaInstallInfo) -> bool {
        // DLCs typically have:
        // 1. Empty or "[]" executable check
        // 2. No launchers defined
        // 3. Non-empty dlcSubPath (often just "\\")

        let has_no_executable = install_info.executable_check.is_empty()
            || install_info.executable_check == "[]";

        let has_no_launchers = install_info.local_install_properties.launchers.is_empty()
            && install_info.content_manifest_launchers.is_empty();

        let has_dlc_subpath = !install_info.dlc_sub_path.is_empty();

        // If it has no executable AND no launchers, it's likely a DLC
        // Or if it has a DLC subpath and no launchers
        (has_no_executable && has_no_launchers) || (has_dlc_subpath && has_no_launchers)
    }

    /// Parse EA Desktop installation database (IS.json)
    async fn parse_ea_desktop_db(&self, json_path: &PathBuf) -> Result<Vec<DetectedGame>, String> {
        let content = tokio::fs::read_to_string(json_path)
            .await
            .map_err(|e| format!("Failed to read IS.json: {}", e))?;

        let db: EaInstallationDb = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse IS.json: {}", e))?;

        let mut games = Vec::new();

        for install_info in db.install_infos {
            // Only include installed games (installStatus == 5 means installed)
            if install_info.detailed_state.install_status != 5 {
                continue;
            }

            // Skip if no install path
            if install_info.base_install_path.is_empty() {
                continue;
            }

            // Skip DLC entries
            if self.is_dlc(&install_info) {
                continue;
            }

            let install_path = Some(normalize_path_separators(&install_info.base_install_path));

            // Try to extract executable path from executableCheck
            let executable_path = self.extract_executable_path(&install_info.executable_check, &install_path);

            // Generate a friendly display name from the slug
            let display_name = self.slug_to_display_name(&install_info.base_slug);

            let platform = GamePlatform::EaApp {
                game_id: install_info.software_id.clone(),
            };

            let game = DetectedGame::new(
                install_info.software_id,
                display_name,
                executable_path,
                install_path,
                platform,
            );

            games.push(game);
        }

        Ok(games)
    }

    /// Extract executable path from registry-based executableCheck string
    /// Format: "[HKEY_LOCAL_MACHINE\\SOFTWARE\\...\\Install Dir]executable.exe"
    fn extract_executable_path(&self, exec_check: &str, install_path: &Option<PathBuf>) -> Option<PathBuf> {
        if exec_check.is_empty() || exec_check == "[]" {
            return None;
        }

        // Extract the executable name from the registry path
        if let Some(start) = exec_check.rfind(']') {
            let exe_name = &exec_check[start + 1..];

            // Combine with install path
            if let Some(ref base_path) = install_path {
                let exe_path = base_path.join(exe_name);
                if exe_path.exists() {
                    return Some(exe_path);
                }
            }
        }

        None
    }

    /// Convert slug to display name (e.g., "battlefield-4" -> "Battlefield 4")
    fn slug_to_display_name(&self, slug: &str) -> String {
        slug.split('-')
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Legacy Origin scanner (for old .mfst format)
    async fn scan_legacy_origin(&self) -> Result<Vec<DetectedGame>, String> {
        let manifest_dir = self.find_legacy_origin_path()?;

        if !manifest_dir.exists() {
            return Ok(Vec::new());
        }

        let mut games = Vec::new();

        // Read all subdirectories in LocalContent
        let mut entries = tokio::fs::read_dir(&manifest_dir)
            .await
            .map_err(|e| format!("Failed to read manifests directory: {}", e))?;

        while let Ok(Some(entry)) = entries.next_entry().await {
            let path = entry.path();

            // Each game has its own directory
            if path.is_dir() {
                if let Ok(game) = self.scan_legacy_game_directory(&path).await {
                    games.push(game);
                }
            }
        }

        Ok(games)
    }

    #[cfg(target_os = "windows")]
    fn find_legacy_origin_path(&self) -> Result<PathBuf, String> {
        let program_data = std::env::var("ProgramData")
            .map_err(|_| "ProgramData environment variable not set")?;
        Ok(PathBuf::from(program_data).join("Origin/LocalContent"))
    }

    #[cfg(target_os = "macos")]
    fn find_legacy_origin_path(&self) -> Result<PathBuf, String> {
        let home = std::env::var("HOME").map_err(|_| "HOME not set")?;
        Ok(PathBuf::from(home).join("Library/Application Support/Origin/LocalContent"))
    }

    /// Scan a legacy Origin game directory for .mfst files
    async fn scan_legacy_game_directory(&self, game_dir: &PathBuf) -> Result<DetectedGame, String> {
        // Find the .mfst file in the directory
        let mut entries = tokio::fs::read_dir(game_dir)
            .await
            .map_err(|e| format!("Failed to read game directory: {}", e))?;

        let mut mfst_files = Vec::new();

        while let Ok(Some(entry)) = entries.next_entry().await {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("mfst") {
                mfst_files.push(path);
            }
        }

        // If multiple .mfst files exist, we need to find the main game manifest
        let mut detected_game: Option<DetectedGame> = None;

        for mfst_file in mfst_files {
            if let Ok(game) = self.parse_legacy_manifest(&mfst_file).await {
                if detected_game.is_none() {
                    detected_game = Some(game);
                }
            }
        }

        detected_game.ok_or_else(|| "No valid manifest found in directory".to_string())
    }

    async fn parse_legacy_manifest(&self, manifest_path: &PathBuf) -> Result<DetectedGame, String> {
        let content = tokio::fs::read_to_string(manifest_path)
            .await
            .map_err(|e| format!("Failed to read manifest: {}", e))?;

        // Parse the manifest content (URL query parameter format)
        let content = content.trim().replace('\n', "").replace('\r', "");

        let mut game_id: Option<String> = None;
        let mut install_path: Option<String> = None;
        let mut display_name: Option<String> = None;

        // Remove leading '?' if present and split by '&'
        let params_str = content.trim_start_matches('?');

        // Parse parameters
        for param in params_str.split('&') {
            if let Some((key, value)) = param.split_once('=') {
                let key_lower = key.trim().to_lowercase();
                match key_lower.as_str() {
                    "id" => {
                        // URL decode the ID (e.g., OFB-EAST%3a52209 -> OFB-EAST:52209)
                        let decoded = urlencoding::decode(value.trim())
                            .map_err(|_| "Failed to decode game ID")?
                            .to_string();
                        game_id = Some(decoded);
                    }
                    "dipinstallpath" => {
                        // URL decode the install path
                        let decoded = urlencoding::decode(value.trim())
                            .map_err(|_| "Failed to decode install path")?
                            .to_string();
                        install_path = Some(decoded);
                    }
                    _ => {}
                }
            }
        }

        let game_id = game_id.ok_or("Missing game ID in manifest")?;

        // Try to derive a display name from the install path or game directory
        if display_name.is_none() {
            if let Some(ref path) = install_path {
                // Extract the last component of the path as the game name
                if let Some(name) = PathBuf::from(path).file_name().and_then(|n| n.to_str()) {
                    display_name = Some(name.to_string());
                }
            }
        }

        // Fallback: use the parent directory name
        if display_name.is_none() {
            if let Some(parent) = manifest_path.parent() {
                if let Some(name) = parent.file_name().and_then(|n| n.to_str()) {
                    display_name = Some(name.to_string());
                }
            }
        }

        let display_name = display_name.unwrap_or_else(|| game_id.clone());

        let install_path_buf = install_path.map(|p| normalize_path_separators(&p));

        // Try to find the executable
        let executable_path = if let Some(ref path) = install_path_buf {
            self.find_game_executable(path).await
        } else {
            None
        };

        let platform = GamePlatform::EaApp {
            game_id: game_id.clone(),
        };

        let game = DetectedGame::new(
            game_id,
            display_name,
            executable_path,
            install_path_buf,
            platform,
        );

        Ok(game)
    }

    async fn find_game_executable(&self, game_path: &PathBuf) -> Option<PathBuf> {
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

        // For macOS, look for .app bundles
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

        // Return the first executable found
        candidates.into_iter().next()
    }
}
