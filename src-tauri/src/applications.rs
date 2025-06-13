use std::fs;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, State, Manager};
use crate::paths::get_applications_json_path;
use crate::ghub::GHUBApp;

pub struct AppState {
    pub applications: Mutex<Vec<GHUBApp>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationsData {
    pub applications: Vec<GHUBApp>,
}

pub fn load_and_store_applications(app_handle: &AppHandle, build_id: &str) -> Result<Vec<GHUBApp>, String> {
    let json_path = get_applications_json_path(app_handle, build_id)
        .ok_or("Failed to get applications.json path")?;
    
    if !json_path.exists() {
        return Err(format!("Applications file not found at: {}", json_path.display()));
    }
    
    let file_content = fs::read_to_string(&json_path)
        .map_err(|e| format!("Failed to read applications.json: {}", e))?;
    
    let applications_data: ApplicationsData = serde_json::from_str(&file_content)
        .map_err(|e| format!("Failed to parse applications.json: {}", e))?;
    
    store_applications_in_manager(app_handle, &applications_data.applications)?;
    
    Ok(applications_data.applications)
}

fn store_applications_in_manager(app_handle: &AppHandle, applications: &[GHUBApp]) -> Result<(), String> {
    let state: State<AppState> = app_handle.state();
    let mut apps = state.applications.lock().unwrap();
    *apps = applications.to_vec();
    Ok(())
}

pub fn get_stored_applications(app_handle: &AppHandle) -> Result<Vec<GHUBApp>, String> {
    let state: State<AppState> = app_handle.state();
    let apps = state.applications.lock().unwrap();
    Ok(apps.clone())
}

#[tauri::command]
pub async fn load_applications_from_json(app_handle: AppHandle, build_id: String) -> Result<Vec<GHUBApp>, String> {
    load_and_store_applications(&app_handle, &build_id)
}

#[tauri::command]
pub async fn get_applications(app_handle: AppHandle) -> Result<Vec<GHUBApp>, String> {
    get_stored_applications(&app_handle)
}