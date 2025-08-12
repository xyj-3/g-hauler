use std::sync::Mutex;
use tauri::{AppHandle, Manager, State};
use crate::applications::models::{ApplicationsData, GHUBApp};

pub struct AppState {
    pub applications: Mutex<Vec<GHUBApp>>,
    pub settings_db_data: Mutex<Option<ApplicationsData>>,
}

pub fn store_applications_in_manager(
    app_handle: &AppHandle,
    applications: &[GHUBApp],
) -> Result<(), String> {
    let state: State<AppState> = app_handle.state();
    let mut apps = state
        .applications
        .lock()
        .map_err(|e| format!("Failed to acquire lock on applications: {}", e))?;
    *apps = applications.to_vec();
    Ok(())
}

pub fn get_stored_applications(app_handle: &AppHandle) -> Result<Vec<GHUBApp>, String> {
    let state: State<AppState> = app_handle.state();
    let apps = state
        .applications
        .lock()
        .map_err(|e| format!("Failed to acquire lock on applications: {}", e))?;
    Ok(apps.clone())
}
