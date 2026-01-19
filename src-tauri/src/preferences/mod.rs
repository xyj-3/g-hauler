use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};
use tauri_plugin_store::StoreExt;

use crate::core::constants::{PREFERENCES_STORE_FILENAME, STORE_KEY_USER_PREFERENCES};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPreferences {
    theme: String,
    sidebar_collapsed: bool,
    grid_view_density: String,
    notifications_enabled: bool,
}

#[tauri::command]
pub async fn get_user_preferences(app: AppHandle) -> Result<Option<UserPreferences>, String> {
    let store = app
        .store(PREFERENCES_STORE_FILENAME)
        .map_err(|e| format!("Failed to access store: {}", e))?;

    match store.get(STORE_KEY_USER_PREFERENCES) {
        Some(value) => {
            let prefs: UserPreferences = serde_json::from_value(value.clone())
                .map_err(|e| format!("Failed to deserialize preferences: {}", e))?;
            Ok(Some(prefs))
        }
        None => Ok(None),
    }
}

#[tauri::command]
pub async fn set_user_preferences(
    app: AppHandle,
    preferences: UserPreferences,
) -> Result<(), String> {
    let store = app
        .store(PREFERENCES_STORE_FILENAME)
        .map_err(|e| format!("Failed to access store: {}", e))?;

    let value = serde_json::to_value(&preferences)
        .map_err(|e| format!("Failed to serialize preferences: {}", e))?;

    store.set(STORE_KEY_USER_PREFERENCES, value);
    store
        .save()
        .map_err(|e| format!("Failed to save store: {}", e))?;

    Ok(())
}
