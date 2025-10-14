use crate::core::constants::APPLIED_PATCHES_FILENAME;
use crate::detection_patches::models::AppliedPatchesData;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

/// Get the path to the applied_patches.json file
pub fn get_applied_patches_path(app_handle: &AppHandle) -> Result<PathBuf, String> {
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;

    // Ensure the directory exists
    if !app_data_dir.exists() {
        std::fs::create_dir_all(&app_data_dir)
            .map_err(|e| format!("Failed to create app data directory: {}", e))?;
    }

    Ok(app_data_dir.join(APPLIED_PATCHES_FILENAME))
}

/// Load the applied patches data from disk
pub fn load_applied_patches(app_handle: &AppHandle) -> Result<AppliedPatchesData, String> {
    let patches_path = get_applied_patches_path(app_handle)?;

    // If the file doesn't exist, return default (empty) data
    if !patches_path.exists() {
        return Ok(AppliedPatchesData::default());
    }

    // Read the file
    let file_content = std::fs::read_to_string(&patches_path)
        .map_err(|e| format!("Failed to read applied_patches.json: {}", e))?;

    // Parse JSON
    let applied_patches: AppliedPatchesData = serde_json::from_str(&file_content)
        .map_err(|e| format!("Failed to parse applied_patches.json: {}", e))?;

    Ok(applied_patches)
}

/// Save the applied patches data to disk
pub fn save_applied_patches(
    app_handle: &AppHandle,
    data: &AppliedPatchesData,
) -> Result<(), String> {
    let patches_path = get_applied_patches_path(app_handle)?;

    // Serialize to JSON
    let json_content = serde_json::to_string_pretty(data)
        .map_err(|e| format!("Failed to serialize applied patches: {}", e))?;

    // Write to file
    std::fs::write(&patches_path, json_content)
        .map_err(|e| format!("Failed to write applied_patches.json: {}", e))?;

    Ok(())
}
