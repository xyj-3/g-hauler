use crate::applications::models::Detection;
use crate::core::state::AppState;
use crate::detection_patches::applier::{
    apply_all_patches, apply_patch_to_game, apply_patches_for_game, get_missing_detections,
    reapply_saved_patches,
};
use crate::detection_patches::loader::{get_patches_for_game, load_detection_patches};
use crate::detection_patches::models::AppliedPatchesData;
use crate::detection_patches::persistence::load_applied_patches;
use crate::websocket::client::WebSocketClient;
use std::sync::Arc;
use tauri::{AppHandle, Manager, State};

/// Get available patches (missing detections) for a specific game
#[tauri::command]
pub async fn get_available_patches_for_game(
    app_handle: AppHandle,
    application_id: String,
) -> Result<Vec<Detection>, String> {
    // Load bundled patches
    let patches_data = load_detection_patches(&app_handle)?;

    // Get patches for this game
    let game_patches = get_patches_for_game(&patches_data, &application_id)
        .ok_or_else(|| format!("No patches available for game '{}'", application_id))?;

    // Get the current app from state
    let state: State<AppState> = app_handle.state();
    let apps = state
        .applications
        .lock()
        .map_err(|e| format!("Failed to acquire lock on applications: {}", e))?;

    let app = apps
        .iter()
        .find(|a| a.application_id == application_id)
        .ok_or_else(|| format!("Application with ID '{}' not found", application_id))?
        .clone();

    drop(apps); // Release lock

    // Find missing detections
    let missing_detections = get_missing_detections(&app, &game_patches.detections);

    Ok(missing_detections)
}

/// Apply a single detection patch to a game
#[tauri::command]
pub async fn patch_apply_single(
    app_handle: AppHandle,
    ws_client: State<'_, Arc<WebSocketClient>>,
    application_id: String,
    detection: Detection,
) -> Result<(), String> {
    apply_patch_to_game(&app_handle, &ws_client, &application_id, detection).await
}

/// Apply all missing detection patches for a specific game
#[tauri::command]
pub async fn patch_apply_for_game(
    app_handle: AppHandle,
    ws_client: State<'_, Arc<WebSocketClient>>,
    application_id: String,
) -> Result<Vec<Detection>, String> {
    let patches_data = load_detection_patches(&app_handle)?;
    apply_patches_for_game(&app_handle, &ws_client, &patches_data, &application_id).await
}

/// Apply all missing detection patches for all games
#[tauri::command]
pub async fn patch_apply_all(
    app_handle: AppHandle,
    ws_client: State<'_, Arc<WebSocketClient>>,
) -> Result<AppliedPatchesData, String> {
    apply_all_patches(&app_handle, &ws_client).await
}

/// Get the current applied patches status
#[tauri::command]
pub async fn patch_get_applied(app_handle: AppHandle) -> Result<AppliedPatchesData, String> {
    load_applied_patches(&app_handle)
}

/// Reapply previously saved patches (typically called on startup)
#[tauri::command]
pub async fn patch_reapply_saved(
    app_handle: AppHandle,
    ws_client: State<'_, Arc<WebSocketClient>>,
) -> Result<(), String> {
    reapply_saved_patches(&app_handle, &ws_client).await
}
