use crate::detection_patches::models::{DetectionPatchesData, GameDetectionPatches};
use tauri::{AppHandle, Manager};

/// Load the bundled detection patches from the resources directory
pub fn load_detection_patches(app_handle: &AppHandle) -> Result<DetectionPatchesData, String> {
    // Get the resource path
    let resource_path = app_handle
        .path()
        .resource_dir()
        .map_err(|e| format!("Failed to get resource directory: {}", e))?;

    let patches_path = resource_path.join("detection_patches.json");

    // Read the file
    let file_content = std::fs::read_to_string(&patches_path)
        .map_err(|e| format!("Failed to read detection_patches.json from {:?}: {}", patches_path, e))?;

    // Parse JSON
    let patches_data: DetectionPatchesData = serde_json::from_str(&file_content)
        .map_err(|e| format!("Failed to parse detection_patches.json: {}", e))?;

    Ok(patches_data)
}

/// Get patches for a specific game by application ID
pub fn get_patches_for_game(
    patches_data: &DetectionPatchesData,
    application_id: &str,
) -> Option<GameDetectionPatches> {
    patches_data
        .patches
        .iter()
        .find(|p| p.application_id == application_id)
        .cloned()
}
