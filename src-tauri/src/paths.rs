use std::path::PathBuf;
use tauri::AppHandle;
use crate::constants::STORE_KEY_DATA_PATH;
use crate::store::get_store_key;

pub fn get_data_path(app_handle: &AppHandle) -> Option<String> {
    get_store_key(app_handle, STORE_KEY_DATA_PATH)?
        .as_str()
        .map(|s| s.to_string())
}

pub fn get_current_json_path(app_handle: &AppHandle) -> Option<PathBuf> {
    let data_path = get_data_path(app_handle)?;
    Some(PathBuf::from(data_path).join("current.json"))
}

pub fn get_version_json_path(app_handle: &AppHandle) -> Option<PathBuf> {
    let data_path = get_data_path(app_handle)?;
    Some(PathBuf::from(data_path).join("version.json"))
}

pub fn get_applications_json_path(app_handle: &AppHandle, build_id: &str) -> Option<PathBuf> {
    let data_path = get_data_path(app_handle)?;
    Some(PathBuf::from(data_path)
        .join("depots")
        .join(build_id)
        .join("core/LGHUB/data/applications.json"))
}

pub fn get_images_dir_path(app_handle: &AppHandle, build_id: &str) -> Option<PathBuf> {
    let data_path = get_data_path(app_handle)?;
    Some(PathBuf::from(data_path)
        .join("depots")
        .join(build_id)
        .join("core_apps/images"))
}