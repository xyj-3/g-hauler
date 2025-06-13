use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GHUBApp {
    pub application_id: String,
    pub category_colors: Vec<CategoryColor>,
    pub commands: Vec<Command>,
    pub detection: Vec<Detection>,
    pub name: String,
    pub poster_title_position: String,
    pub poster_url: String,
    pub version: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryColor {
    pub hex: String,
    pub tag: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub category: String,
    pub keystroke: Vec<String>,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Detection {
    Steam { steam: SteamApp },
    WinRegistry { win_registry: WinRegistry },
    EpicGames { epic_games: EpicGames },
    OsxBundle { osx_bundle: OsxBundle },
    Uplay { uplay: Uplay },
    GogGalaxy { gog_galaxy: GogGalaxy },
    HumbleApp { humble_app: HumbleApp },
    RiotGames { riot_games: RiotGames },
    Glob { glob: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SteamApp {
    pub app_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WinRegistry {
    pub executable: String,
    pub registry_key: String,
    pub registry_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpicGames {
    pub app_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsxBundle {
    pub bundle_id: String,
    pub bundle_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Uplay {
    pub app_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GogGalaxy {
    pub product_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumbleApp {
    pub game_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiotGames {
    pub app_name: String,
}
