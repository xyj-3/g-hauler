use crate::ghub::GHUBApp;
use crate::util::{get_applications_json_path, get_build_id};
use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::Mutex;
use tauri::{AppHandle, Manager, State};

pub struct AppState {
    pub applications: Mutex<Vec<GHUBApp>>,
    pub settings_db_data: Mutex<Option<ApplicationsData>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationsData {
    pub applications: Vec<GHUBApp>,
}

fn store_applications_in_manager(
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

pub fn load_and_store_applications(
    app_handle: &AppHandle,
    build_id: &str,
) -> Result<Vec<GHUBApp>, String> {
    let json_path = get_applications_json_path(app_handle, build_id)
        .ok_or("Failed to get applications.json path")?;

    if !json_path.exists() {
        return Err(format!(
            "Applications file not found at: {}",
            json_path.display()
        ));
    }

    let file_content = fs::read_to_string(&json_path)
        .map_err(|e| format!("Failed to read applications.json: {}", e))?;

    // Parse the JSON as a raw value first
    let json_value: serde_json::Value =
        serde_json::from_str(&file_content).map_err(|e| format!("Failed to parse JSON: {}", e))?;

    // Extract the applications array
    let applications_array = json_value
        .get("applications")
        .and_then(|v| v.as_array())
        .ok_or("No 'applications' array found in JSON")?;

    // Parse each application individually, skipping malformed ones
    let mut valid_applications = Vec::new();
    let mut skipped_count = 0;

    for (index, app_value) in applications_array.iter().enumerate() {
        match serde_json::from_value::<GHUBApp>(app_value.clone()) {
            Ok(app) => {
                // Only include applications that have essential fields
                if !app.application_id.is_empty() && !app.name.is_empty() {
                    valid_applications.push(app);
                } else {
                    skipped_count += 1;
                    eprintln!("Skipped application {} (missing essential fields)", index);
                }
            }
            Err(e) => {
                skipped_count += 1;
                eprintln!("Failed to parse application {}: {} - skipping", index, e);
            }
        }
    }
    if skipped_count > 0 {
        eprintln!(
            "Loaded {} valid applications, skipped {} malformed entries",
            valid_applications.len(),
            skipped_count
        );
    } else {
        println!(
            "Successfully loaded {} applications",
            valid_applications.len()
        );
    }

    store_applications_in_manager(app_handle, &valid_applications)?;

    Ok(valid_applications)
}

pub fn get_stored_applications(app_handle: &AppHandle) -> Result<Vec<GHUBApp>, String> {
    let state: State<AppState> = app_handle.state();
    let apps = state
        .applications
        .lock()
        .map_err(|e| format!("Failed to acquire lock on applications: {}", e))?;
    Ok(apps.clone())
}

pub fn initialize_applications_on_startup(app_handle: &AppHandle) -> Result<(), String> {
    // Try to get build_id and load applications
    if let Some(build_id) = get_build_id(app_handle) {
        match load_and_store_applications(app_handle, &build_id) {
            Ok(apps) => {
                println!("Successfully loaded {} applications on startup", apps.len());
                Ok(())
            }
            Err(e) => {
                eprintln!("Warning: Failed to load applications on startup: {}", e);
                // Don't return error - app should still start even if applications can't be loaded
                Ok(())
            }
        }
    } else {
        eprintln!("Warning: Could not determine build_id on startup - applications not loaded");
        // Don't return error - app should still start
        Ok(())
    }
}

#[tauri::command]
pub async fn get_applications(app_handle: AppHandle) -> Result<Vec<GHUBApp>, String> {
    get_stored_applications(&app_handle)
}

#[tauri::command]
pub async fn update_application(
    app_handle: AppHandle,
    updated_app: GHUBApp,
) -> Result<(), String> {
    let state: State<AppState> = app_handle.state();
    let mut apps = state
        .applications
        .lock()
        .map_err(|e| format!("Failed to acquire lock on applications: {}", e))?;

    // Find the application by application_id and update it
    let app_index = apps
        .iter()
        .position(|app| app.application_id == updated_app.application_id)
        .ok_or_else(|| {
            format!(
                "Application with ID '{}' not found",
                updated_app.application_id
            )
        })?;

    apps[app_index] = updated_app;
    
    println!("Successfully updated application with ID: {}", apps[app_index].application_id);
    Ok(())
}

#[tauri::command]
pub async fn get_application_by_id(
    app_handle: AppHandle,
    application_id: String,
) -> Result<Option<GHUBApp>, String> {
    let state: State<AppState> = app_handle.state();
    let apps = state
        .applications
        .lock()
        .map_err(|e| format!("Failed to acquire lock on applications: {}", e))?;

    let app = apps
        .iter()
        .find(|app| app.application_id == application_id)
        .cloned();

    Ok(app)
}

#[tauri::command]
pub async fn save_applications_to_disk(app_handle: AppHandle) -> Result<(), String> {
    let build_id = get_build_id(&app_handle).ok_or("Failed to get build_id")?;
    let json_path = get_applications_json_path(&app_handle, &build_id)
        .ok_or("Failed to get applications.json path")?;

    let state: State<AppState> = app_handle.state();
    let apps = state
        .applications
        .lock()
        .map_err(|e| format!("Failed to acquire lock on applications: {}", e))?;

    let applications_data = ApplicationsData {
        applications: apps.clone(),
    };

    let json_content = serde_json::to_string_pretty(&applications_data)
        .map_err(|e| format!("Failed to serialize applications: {}", e))?;

    // Create parent directory if it doesn't exist
    if let Some(parent) = json_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    fs::write(&json_path, json_content)
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                format!("Permission denied writing to {}: {}. Please run as administrator or check folder permissions.", json_path.display(), e)
            } else {
                format!("Failed to write applications.json: {}", e)
            }
        })?;

    println!("Successfully saved {} applications to disk", apps.len());
    Ok(())
}
