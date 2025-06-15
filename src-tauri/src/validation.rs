use crate::constants::STORE_KEY_DATA_PATH;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

#[derive(serde::Serialize)]
pub struct PathValidationResult {
    pub data_path_exists: bool,
    pub applications_json_exists: bool,
    pub current_json_exists: bool,
    pub version_json_exists: bool,
    pub build_id: Option<String>,
    pub images_dir_exists: bool,
}

#[tauri::command]
pub async fn validate_paths(app_handle: AppHandle) -> PathValidationResult {
    let store = match app_handle.store(crate::constants::STORE_FILENAME) {
        Ok(store) => store,
        Err(_) => {
            return PathValidationResult {
                data_path_exists: false,
                applications_json_exists: false,
                current_json_exists: false,
                version_json_exists: false,
                build_id: None,
                images_dir_exists: false,
            };
        }
    };
    let data_path = store
        .get(STORE_KEY_DATA_PATH)
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap();

    let data_path_exists = fs::metadata(&data_path).is_ok();

    let current_json = PathBuf::from(&data_path).join("current.json");
    let version_json = PathBuf::from(&data_path).join("version.json");

    let current_json_exists = fs::metadata(&current_json).is_ok();
    let version_json_exists = fs::metadata(&version_json).is_ok();

    let build_id = if current_json_exists {
        fs::read_to_string(&current_json)
            .ok()
            .and_then(|content| serde_json::from_str::<Value>(&content).ok())
            .and_then(|json| {
                json.get("buildId")
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
            })
    } else {
        None
    };

    let applications_json_exists = if let Some(ref build_id) = build_id {
        let applications_json = PathBuf::from(&data_path)
            .join("depots")
            .join(build_id)
            .join("core/LGHUB/data/applications.json");
        fs::metadata(&applications_json).is_ok()
    } else {
        false
    };

    let images_dir_exists = if let Some(ref build_id) = build_id {
        let images_dir = PathBuf::from(&data_path)
            .join("depots")
            .join(build_id)
            .join("core_apps/images");
        fs::metadata(&images_dir)
            .map(|m| m.is_dir())
            .unwrap_or(false)
    } else {
        false
    };

    PathValidationResult {
        data_path_exists,
        applications_json_exists,
        current_json_exists,
        version_json_exists,
        build_id,
        images_dir_exists,
    }
}
