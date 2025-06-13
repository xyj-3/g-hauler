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
    WinRegistry { winRegistry: WinRegistry },
    EpicGames { epicGames: EpicGames },
    OsxBundle { osxBundle: OsxBundle },
    Uplay { uplay: Uplay },
    GogGalaxy { gogGalaxy: GogGalaxy },
    HumbleApp { humbleApp: HumbleApp },
    RiotGames { riotGames: RiotGames },
    Glob { glob: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SteamApp {
    pub appId: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WinRegistry {
    pub executable: String,
    pub registryKey: String,
    pub registryPath: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpicGames {
    pub appName: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsxBundle {
    pub bundleId: String,
    pub bundlePath: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Uplay {
    pub appId: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GogGalaxy {
    pub productId: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumbleApp {
    pub gameName: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiotGames {
    pub appName: String,
}
