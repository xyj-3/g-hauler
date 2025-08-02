use crate::constants::{LGHUB_DEFAULT_DATA_PATH, STORE_FILENAME, STORE_KEY_DATA_PATH, STORE_KEY_AUTOSTART};
use serde_json::Value;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

use serde_json::json;

pub async fn initialize_store(
    app_handle: &tauri::AppHandle,
) -> Result<(), tauri_plugin_store::Error> {
    let store = app_handle.store(STORE_FILENAME)?;
    let mut changed = false;

    // Initialize default data path if not set
    if store.get(STORE_KEY_DATA_PATH).is_none() {
        store.set(STORE_KEY_DATA_PATH, json!(LGHUB_DEFAULT_DATA_PATH));
        changed = true;
    }

    if store.get(STORE_KEY_AUTOSTART).is_none() {
        store.set(STORE_KEY_AUTOSTART, json!(false));
        changed = true;
    }

    if changed {
        store.save()?;
        println!("Store initialized with default values");
    }

    Ok(())
}

// Internal synchronous function for use within Rust code
pub fn get_store_key(app_handle: &AppHandle, key: &str) -> Option<Value> {
    match app_handle.store(STORE_FILENAME) {
        Ok(store) => store.get(key),
        Err(_) => None,
    }
}

// Internal synchronous function for use within Rust code
pub fn set_store_key(app_handle: &AppHandle, key: &str, value: Value) -> Result<(), String> {
    let store = app_handle
        .store(STORE_FILENAME)
        .map_err(|e| format!("Failed to access store: {}", e))?;
    store.set(key, value);
    store
        .save()
        .map_err(|e| format!("Failed to save store: {}", e))
}

// Tauri command for frontend calls
#[tauri::command]
pub async fn store_get_key(app_handle: AppHandle, key: String) -> Option<Value> {
    get_store_key(&app_handle, &key)
}

// Tauri command for frontend calls
#[tauri::command]
pub async fn store_set_key(app_handle: AppHandle, key: String, value: Value) -> Result<(), String> {
    set_store_key(&app_handle, &key, value)
}
