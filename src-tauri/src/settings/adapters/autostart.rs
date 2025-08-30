use super::SystemSetting;
use serde_json::Value;
use tauri::AppHandle;
use tauri_plugin_autostart::ManagerExt;

pub struct AutostartAdapter;

impl SystemSetting for AutostartAdapter {
    fn read(&self, app: &AppHandle) -> Result<Value, String> {
        Ok(Value::Bool(app.autolaunch().is_enabled().unwrap_or(false)))
    }

    fn apply(&self, app: &AppHandle, value: &Value) -> Result<(), String> {
        let should_enable = value.as_bool().ok_or("autostart expects boolean")?;
        if should_enable {
            app.autolaunch().enable().map_err(|e| e.to_string())
        } else {
            app.autolaunch().disable().map_err(|e| e.to_string())
        }
    }
}
