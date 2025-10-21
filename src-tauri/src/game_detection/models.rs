use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Represents different gaming platforms/launchers with platform-specific identifiers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum GamePlatform {
    #[serde(rename_all = "camelCase")]
    Steam { app_id: String },
    #[serde(rename_all = "camelCase")]
    EpicGames { app_name: String },
    #[serde(rename_all = "camelCase")]
    WinRegistry { registry_key: String },
    #[serde(rename_all = "camelCase")]
    Uplay { app_id: String },
    #[serde(rename_all = "camelCase")]
    GogGalaxy { product_id: String },
    #[serde(rename_all = "camelCase")]
    RiotGames { app_name: String },
    #[serde(rename_all = "camelCase")]
    OsxBundle { bundle_id: String },
    #[serde(rename_all = "camelCase")]
    EaApp { game_id: String },
}

impl GamePlatform {
    // Platform name constants
    pub const STEAM_NAME: &'static str = "Steam";
    pub const EPIC_GAMES_NAME: &'static str = "Epic Games";
    pub const WIN_REGISTRY_NAME: &'static str = "Windows Registry";
    pub const UPLAY_NAME: &'static str = "Ubisoft Connect";
    pub const GOG_GALAXY_NAME: &'static str = "GOG Galaxy";
    pub const RIOT_GAMES_NAME: &'static str = "Riot Games";
    pub const OSX_BUNDLE_NAME: &'static str = "macOS App";
    pub const EA_APP_NAME: &'static str = "EA App";

    /// Get the platform name as a string from an instance
    pub fn name(&self) -> &str {
        match self {
            GamePlatform::Steam { .. } => Self::STEAM_NAME,
            GamePlatform::EpicGames { .. } => Self::EPIC_GAMES_NAME,
            GamePlatform::WinRegistry { .. } => Self::WIN_REGISTRY_NAME,
            GamePlatform::Uplay { .. } => Self::UPLAY_NAME,
            GamePlatform::GogGalaxy { .. } => Self::GOG_GALAXY_NAME,
            GamePlatform::RiotGames { .. } => Self::RIOT_GAMES_NAME,
            GamePlatform::OsxBundle { .. } => Self::OSX_BUNDLE_NAME,
            GamePlatform::EaApp { .. } => Self::EA_APP_NAME,
        }
    }

    /// Get the platform-specific identifier from an instance
    pub fn identifier(&self) -> &str {
        match self {
            GamePlatform::Steam { app_id } => app_id,
            GamePlatform::EpicGames { app_name } => app_name,
            GamePlatform::WinRegistry { registry_key } => registry_key,
            GamePlatform::Uplay { app_id } => app_id,
            GamePlatform::GogGalaxy { product_id } => product_id,
            GamePlatform::RiotGames { app_name } => app_name,
            GamePlatform::OsxBundle { bundle_id } => bundle_id,
            GamePlatform::EaApp { game_id } => game_id,
        }
    }
}

/// Represents a locally detected/installed game
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DetectedGame {
    /// Unique identifier for the game (e.g., Steam App ID, Epic app name, etc.)
    pub id: String,
    /// Display name of the game
    pub name: String,
    /// Path to the game's executable
    pub executable_path: Option<PathBuf>,
    /// Path to the game's installation directory
    pub install_path: Option<PathBuf>,
    /// Platform/launcher where the game was detected
    pub platform: GamePlatform,
    /// Additional platform-specific data
    #[serde(default)]
    pub platform_data: HashMap<String, String>,
}

impl DetectedGame {
    /// Create a new DetectedGame with the required fields
    pub fn new(
        id: String,
        name: String,
        executable_path: Option<PathBuf>,
        install_path: Option<PathBuf>,
        platform: GamePlatform,
    ) -> Self {
        Self {
            id,
            name,
            executable_path,
            install_path,
            platform,
            platform_data: HashMap::new(),
        }
    }

    /// Add platform-specific data (builder pattern)
    pub fn with_platform_data(mut self, key: String, value: String) -> Self {
        self.platform_data.insert(key, value);
        self
    }
}

/// Container for all detected games from a scan
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameScanResult {
    /// List of all detected games
    pub games: Vec<DetectedGame>,
    /// Games grouped by platform name
    pub games_by_platform: HashMap<String, Vec<DetectedGame>>,
    /// Number of games found
    pub total_count: usize,
    /// Scan duration in milliseconds
    pub scan_duration_ms: u64,
    /// Platforms that were successfully scanned
    pub scanned_platforms: Vec<String>,
    /// Platforms that failed during scanning
    pub failed_platforms: Vec<String>,
    /// Any errors encountered during scanning
    pub errors: Vec<String>,
}

impl GameScanResult {
    pub fn new() -> Self {
        Self {
            games: Vec::new(),
            games_by_platform: HashMap::new(),
            total_count: 0,
            scan_duration_ms: 0,
            scanned_platforms: Vec::new(),
            failed_platforms: Vec::new(),
            errors: Vec::new(),
        }
    }

    /// Add games from a platform scan
    pub fn add_games(&mut self, platform_name: String, mut games: Vec<DetectedGame>) {
        self.total_count += games.len();
        self.games_by_platform.insert(platform_name.clone(), games.clone());
        self.games.append(&mut games);
        self.scanned_platforms.push(platform_name);
    }

    /// Mark a platform as failed
    pub fn mark_failed(&mut self, platform_name: String, error: String) {
        self.failed_platforms.push(platform_name);
        self.errors.push(error);
    }

    /// Finalize the result with scan duration
    pub fn finalize(mut self, duration_ms: u64) -> Self {
        self.scan_duration_ms = duration_ms;

        // // Deduplicate games by ID, keeping the first occurrence
        // let mut seen_ids = std::collections::HashSet::new();
        // self.games.retain(|game| seen_ids.insert(game.id.clone()));

        // // Update total count to reflect deduplicated games
        // self.total_count = self.games.len();

        self
    }
}

impl Default for GameScanResult {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration for game scanning
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanOptions {
    /// Whether to scan Steam games
    pub scan_steam: bool,
    /// Whether to scan Epic Games
    pub scan_epic_games: bool,
    /// Whether to scan Uplay/Ubisoft Connect
    pub scan_uplay: bool,
    /// Whether to scan GOG Galaxy
    pub scan_gog_galaxy: bool,
    /// Whether to scan Riot Games
    pub scan_riot_games: bool,
    /// Whether to scan Windows Registry for games
    pub scan_win_registry: bool,
    /// Whether to scan macOS app bundles
    pub scan_osx_bundle: bool,
    /// Whether to scan EA App (formerly Origin)
    pub scan_ea_app: bool,
}

impl ScanOptions {
    /// Create scan options with all platforms enabled
    pub fn all() -> Self {
        Self {
            scan_steam: true,
            scan_epic_games: true,
            scan_uplay: true,
            scan_gog_galaxy: true,
            scan_riot_games: true,
            scan_win_registry: true,
            scan_osx_bundle: true,
            scan_ea_app: true,
        }
    }

    /// Quick scan preset (common platforms)
    pub fn quick() -> Self {
        Self {
            scan_steam: true,
            scan_epic_games: true,
            scan_uplay: false,
            scan_gog_galaxy: true,
            scan_riot_games: true,
            scan_win_registry: false,
            scan_osx_bundle: cfg!(target_os = "macos"),
            scan_ea_app: true,
        }
    }
}

impl Default for ScanOptions {
    fn default() -> Self {
        Self::all()
    }
}
