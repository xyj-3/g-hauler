use serde_json::Value;
use tauri::AppHandle;

pub trait SystemSetting {
    fn read(&self, app: &AppHandle) -> Result<Value, String>;
    fn apply(&self, app: &AppHandle, value: &Value) -> Result<(), String>;
    fn capable(&self, _app: &AppHandle) -> bool { true }
}

pub mod autostart;

use std::collections::HashMap;
use autostart::AutostartAdapter;

pub fn registry() -> HashMap<&'static str, Box<dyn SystemSetting + Send + Sync>> {
    let mut map: HashMap<&'static str, Box<dyn SystemSetting + Send + Sync>> = HashMap::new();
    map.insert("autostart", Box::new(AutostartAdapter));
    map
}
