use serde_json::Value;
use tauri::AppHandle;

use crate::core::store;
use super::{models::Setting, registry, state, validation};

#[tauri::command]
pub async fn settings_get_registry() -> Result<serde_json::Value, String> {
    Ok(serde_json::to_value(registry::all()).unwrap())
}

#[tauri::command]
pub async fn settings_get_state(app: AppHandle) -> Result<serde_json::Value, String> {
    let items = state::build_state(&app);
    Ok(serde_json::to_value(items).unwrap())
}

#[tauri::command]
pub async fn settings_set_and_apply(app: AppHandle, key: String, value: Value) -> Result<serde_json::Value, String> {
    let setting: &Setting = registry::find(&key).ok_or_else(|| format!("unknown key '{}'", key))?;

    validation::validate_runtime_value(setting, &value)?;

    // Save old value for rollback
    let prev = store::get_store_key(&app, &key);

    // 1) write user preference
    store::set_store_key(&app, &key, value.clone())?;

    // 2) apply system side-effect if needed
    if setting.system_managed {
        let adapters = super::adapters::registry();
        if let Some(adapter) = adapters.get(key.as_str()) {
            if let Err(e) = adapter.apply(&app, &value) {
                if let Some(p) = prev {
                    let _ = store::set_store_key(&app, &key, p);
                }
                return Err(format!("failed to apply system setting '{}': {}", key, e));
            }
        }
    }

    // 3) return fresh state
    let items = state::build_state(&app);
    Ok(serde_json::to_value(items).unwrap())
}
