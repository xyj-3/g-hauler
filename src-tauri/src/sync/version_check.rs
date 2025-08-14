use crate::applications::paths::get_version_json_path;
use std::fs;
use tauri::AppHandle;

pub fn get_ghub_version(app_handle: &AppHandle) -> Option<String> {
    let version_json_path = get_version_json_path(app_handle)?;
    let content = fs::read_to_string(version_json_path).ok()?;
    let json: serde_json::Value = serde_json::from_str(&content).ok()?;
    json.get("version")?.as_str().map(|s| s.to_string())
}