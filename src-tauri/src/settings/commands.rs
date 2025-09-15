use serde_json::Value;
use tauri::AppHandle;

use crate::core::{state as app_state, store};
use crate::settings::{adapters, registry, validation};
use crate::settings::models::Setting;

#[tauri::command]
pub async fn settings_get_registry() -> Result<serde_json::Value, String> {
    Ok(serde_json::to_value(registry::all()).unwrap())
}

#[tauri::command]
pub async fn settings_get_state(app: AppHandle) -> Result<serde_json::Value, String> {
    // Serve the cached state kept in AppState
    let ss = app_state::get_settings_state(&app)?;
    Ok(serde_json::to_value(ss.items).unwrap())
}

#[tauri::command]
pub async fn settings_set_and_apply(app: AppHandle, key: String, value: Value) -> Result<serde_json::Value, String> {
    let setting: &Setting = registry::find(&key).ok_or_else(|| format!("unknown key '{}'", key))?;

    validation::validate_runtime_value(setting, &value)?;

    let prev = store::get_store_key(&app, &key);

    store::set_store_key(&app, &key, value.clone())?;

    if setting.system_managed {
        if let Some(adapter) = adapters::registry().get(key.as_str()) {
            if let Err(e) = adapter.apply(&app, &value) {
                if let Some(p) = prev {
                    let _ = store::set_store_key(&app, &key, p);
                }
                return Err(format!("failed to apply system setting '{}': {}", key, e));
            }
        }
    }

    app_state::refresh_settings_state(&app)?;
    let ss = app_state::get_settings_state(&app)?;
    Ok(serde_json::to_value(ss.items).unwrap())
}
