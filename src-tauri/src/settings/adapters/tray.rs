use super::SystemSetting;
use crate::core::constants::{STORE_FILENAME, STORE_KEY_MINIMIZE_TO_TRAY};
use serde_json::Value;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

pub struct TrayAdapter;

impl SystemSetting for TrayAdapter {
    fn read(&self, app: &AppHandle) -> Result<Value, String> {
        // Read from store since this is a preference-based setting
        let store = app.store(STORE_FILENAME)
            .map_err(|e| e.to_string())?;

        let value = match store.get(STORE_KEY_MINIMIZE_TO_TRAY) {
            Some(v) => v.clone(),
            None => Value::Bool(false),
        };

        Ok(value)
    }

    fn apply(&self, app: &AppHandle, value: &Value) -> Result<(), String> {
        // Store the preference
        let should_enable = value.as_bool().ok_or("minimize_to_tray expects boolean")?;

        let store = app.store(STORE_FILENAME)
            .map_err(|e| e.to_string())?;

        store.set(STORE_KEY_MINIMIZE_TO_TRAY, Value::Bool(should_enable));
        store.save().map_err(|e| e.to_string())
    }
}
