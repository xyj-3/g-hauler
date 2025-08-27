use crate::core::constants::{STORE_KEY_DATA_PATH, STORE_FILENAME};
use crate::applications::paths::{
    get_applications_json_path, get_current_json_path, get_images_dir_path, get_version_json_path,
};
use serde_json::Value;
use std::fs;
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
    let store = match app_handle.store(STORE_FILENAME) {
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

    let data_path = match store
        .get(STORE_KEY_DATA_PATH)
        .and_then(|v| v.as_str().map(|s| s.to_string()))
    {
        Some(path) => path,
        None => {
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

    let data_path_exists = fs::metadata(&data_path).is_ok();

    let current_json_path = get_current_json_path(&app_handle);
    let version_json_path = get_version_json_path(&app_handle);

    let current_json_exists = current_json_path
        .as_ref()
        .map(|p| fs::metadata(p).is_ok())
        .unwrap_or(false);

    let version_json_exists = version_json_path
        .as_ref()
        .map(|p| fs::metadata(p).is_ok())
        .unwrap_or(false);

    let build_id = if current_json_exists {
        current_json_path
            .and_then(|path| fs::read_to_string(&path).ok())
            .and_then(|content| serde_json::from_str::<Value>(&content).ok())
            .and_then(|json| {
                json.get("buildId")
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
            })
    } else {
        None
    };

    let applications_json_exists = if let Some(ref build_id) = build_id {
        if let Some(applications_json_path) = get_applications_json_path(&app_handle, build_id) {
            fs::metadata(&applications_json_path).is_ok()
        } else {
            false
        }
    } else {
        false
    };

    let images_dir_exists = if let Some(ref build_id) = build_id {
        if let Some(images_dir_path) = get_images_dir_path(&app_handle, build_id) {
            fs::metadata(&images_dir_path)
                .map(|m| m.is_dir())
                .unwrap_or(false)
        } else {
            false
        }
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
