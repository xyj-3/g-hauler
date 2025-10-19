use serde_json::Value;
use tauri::AppHandle;

pub trait SystemSetting {
    fn read(&self, app: &AppHandle) -> Result<Value, String>;
    fn apply(&self, app: &AppHandle, value: &Value) -> Result<(), String>;
    fn capable(&self, _app: &AppHandle) -> bool { true }
}

pub mod autostart;
pub mod tray;

use std::collections::HashMap;
use autostart::AutostartAdapter;
use tray::TrayAdapter;
use crate::core::constants::{STORE_KEY_AUTOSTART, STORE_KEY_MINIMIZE_TO_TRAY};

pub fn registry() -> HashMap<&'static str, Box<dyn SystemSetting + Send + Sync>> {
    let mut map: HashMap<&'static str, Box<dyn SystemSetting + Send + Sync>> = HashMap::new();
    map.insert(STORE_KEY_AUTOSTART, Box::new(AutostartAdapter));
    map.insert(STORE_KEY_MINIMIZE_TO_TRAY, Box::new(TrayAdapter));
    map
}
