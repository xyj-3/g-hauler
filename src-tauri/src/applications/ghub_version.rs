use crate::applications::paths::get_version_json_path;
use crate::core::constants::STORE_KEY_LAST_GHUB_VERSION;
use crate::core::store::{get_store_key, set_store_key};
use std::fs;
use tauri::AppHandle;

/// Get the current G HUB version from version.json
pub fn get_ghub_version(app_handle: &AppHandle) -> Option<String> {
    let version_json_path = get_version_json_path(app_handle)?;
    let content = fs::read_to_string(version_json_path).ok()?;
    let json: serde_json::Value = serde_json::from_str(&content).ok()?;
    json.get("version")?.as_str().map(|s| s.to_string())
}

/// Check if G HUB version has changed since last run
pub fn has_version_changed(app_handle: &AppHandle) -> Result<bool, String> {
    // Get current G HUB version from version.json
    let current_version = get_ghub_version(app_handle);

    // Get last stored version
    let stored_version = get_store_key(app_handle, STORE_KEY_LAST_GHUB_VERSION)
        .and_then(|v| v.as_str().map(String::from));

    match (current_version, stored_version) {
        (Some(current), Some(stored)) => {
            // Both versions exist - compare them
            Ok(current != stored)
        }
        (Some(_), None) => {
            // Current version exists but no stored version - treat as changed
            Ok(true)
        }
        (None, _) => {
            // Can't determine current version - don't reapply patches
            Err("Could not determine G HUB version".to_string())
        }
    }
}

/// Update the stored G HUB version to the current version
pub fn update_stored_version(app_handle: &AppHandle) -> Result<(), String> {
    let current_version = get_ghub_version(app_handle)
        .ok_or("Could not determine G HUB version")?;

    set_store_key(
        app_handle,
        STORE_KEY_LAST_GHUB_VERSION,
        serde_json::json!(current_version),
    )?;

    println!("Updated stored G HUB version to: {}", current_version);
    Ok(())
}
