use serde_json::Value;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;
use crate::constants::STORE_FILENAME;

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
