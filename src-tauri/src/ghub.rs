use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GHUBApp {
    #[serde(default)]
    pub application_id: String,
    #[serde(default)]
    pub category_colors: Vec<CategoryColor>,
    #[serde(default)]
    pub commands: Vec<Command>,
    #[serde(default)]
    pub detection: Vec<Detection>,
    pub name: String,
    #[serde(alias = "poster_title_position", default)]
    pub poster_title_position: String,
    #[serde(alias = "poster_url", default)]
    pub poster_url: String,
    #[serde(default)]
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
    Steam {
        steam: SteamApp,
    },
    WinRegistry {
        #[serde(rename = "winRegistry")]
        win_registry: WinRegistry,
    },
    EpicGames {
        #[serde(rename = "epicGames")]
        epic_games: EpicGames,
    },
    OsxBundle {
        #[serde(rename = "osxBundle")]
        osx_bundle: OsxBundle,
    },
    Uplay {
        uplay: Uplay,
    },
    GogGalaxy {
        #[serde(rename = "gogGalaxy")]
        gog_galaxy: GogGalaxy,
    },
    HumbleApp {
        #[serde(rename = "humbleApp")]
        humble_app: HumbleApp,
    },
    RiotGames {
        #[serde(rename = "riotGames")]
        riot_games: RiotGames,
    },
    Glob {
        glob: String,
    },
    // Catch-all for unknown detection types
    Unknown(serde_json::Value),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SteamApp {
    #[serde(rename = "appId")]
    pub app_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WinRegistry {
    pub executable: String,
    #[serde(rename = "registryKey")]
    pub registry_key: String,
    #[serde(rename = "registryPath")]
    pub registry_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpicGames {
    #[serde(rename = "appName")]
    pub app_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsxBundle {
    #[serde(rename = "bundleId")]
    pub bundle_id: String,
    #[serde(rename = "bundlePath")]
    pub bundle_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Uplay {
    #[serde(rename = "appId")]
    pub app_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GogGalaxy {
    #[serde(rename = "productId")]
    pub product_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumbleApp {
    #[serde(rename = "gameName")]
    pub game_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiotGames {
    #[serde(rename = "appName")]
    pub app_name: String,
}
