use crate::applications::models::Detection;
use serde::{Deserialize, Serialize};

/// The bundled detection patches file that ships with the app
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionPatchesData {
    pub version: u32,
    pub patches: Vec<GameDetectionPatches>,
}

/// Detection patches available for a specific game
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameDetectionPatches {
    pub application_id: String,
    pub game_name: String,
    pub detections: Vec<Detection>,
}

/// User's applied patches tracking (saved to applied_patches.json)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppliedPatchesData {
    pub apply_all_defaults: bool,
    pub applied_at: Option<String>,
    pub per_game_overrides: Vec<GamePatchOverride>,
}

impl Default for AppliedPatchesData {
    fn default() -> Self {
        Self {
            apply_all_defaults: false,
            applied_at: None,
            per_game_overrides: Vec::new(),
        }
    }
}

/// Individual game patch override applied by the user
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GamePatchOverride {
    pub application_id: String,
    pub game_name: String,
    pub applied_detections: Vec<Detection>,
    pub applied_at: String,
}
