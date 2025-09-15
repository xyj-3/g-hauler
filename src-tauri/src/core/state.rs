use std::sync::Mutex;
use tauri::{AppHandle, Manager, State};

use crate::applications::models::GHUBApp;
use crate::settings::models::{SettingsState};
use crate::settings::state as settings_state;

pub struct AppState {
    pub applications: Mutex<Vec<GHUBApp>>,
    pub settings_state: Mutex<SettingsState>,
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


pub fn refresh_settings_state(app_handle: &AppHandle) -> Result<(), String> {
    let built = settings_state::build_state(app_handle);
    let state: State<AppState> = app_handle.state();
    let mut ss = state
        .settings_state
        .lock()
        .map_err(|e| format!("Failed to lock settings_state: {}", e))?;
    ss.items = built;
    Ok(())
}

pub fn get_settings_state(app_handle: &AppHandle) -> Result<SettingsState, String> {
    let state: State<AppState> = app_handle.state();
    let ss = state
        .settings_state
        .lock()
        .map_err(|e| format!("Failed to lock settings_state: {}", e))?;
    Ok(SettingsState { items: ss.items.clone() })
}
