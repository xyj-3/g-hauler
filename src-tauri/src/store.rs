use serde_json::Value;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;
use crate::constants::{
    STORE_FILENAME,
    STORE_KEY_INSTALL_PATH,
    STORE_KEY_DATA_PATH,
    LGHUB_DEFAULT_PATH,
    LGHUB_DATA_PATH
};

use serde_json::json;

pub async fn initialize_store(app_handle: &tauri::AppHandle) -> Result<(), tauri_plugin_store::Error> {
    let store = app_handle.store(STORE_FILENAME)?;
    let mut changed = false;

    if store.get(STORE_KEY_INSTALL_PATH).is_none() {
        store.set(STORE_KEY_INSTALL_PATH, json!(LGHUB_DEFAULT_PATH));
        changed = true;
    }
    if store.get(STORE_KEY_DATA_PATH).is_none() {
        store.set(STORE_KEY_DATA_PATH, json!(LGHUB_DATA_PATH));
        changed = true;
    }
    if changed {
        store.save()?;
    }
    Ok(())
}

#[tauri::command]
pub async fn store_get_key(app_handle: AppHandle, key: String) -> Option<Value> {
    match app_handle.store(STORE_FILENAME) {
        Ok(store) => store.get(&key),
        Err(_) => None
    }
}

#[tauri::command]
pub async fn store_set_key(app_handle: AppHandle, key: String, value: Value) -> Result<(), String> {
    let store = app_handle.store(STORE_FILENAME)
        .map_err(|e| format!("Failed to access store: {}", e))?;
    store.set(key.clone(), value);
    store.save()
        .map_err(|e| format!("Failed to save store: {}", e))
}
