use crate::applications::models::{Detection, GHUBApp};
use crate::core::state::AppState;
use crate::ghub_game_patches::loader::{get_patches_for_game, load_detection_patches};
use crate::ghub_game_patches::models::{
    AppliedPatchesData, DetectionPatchesData, GamePatchOverride,
};
use crate::ghub_game_patches::persistence::{load_applied_patches, save_applied_patches};
use crate::websocket::client::{WebSocketClient, WebSocketMessage};
use std::sync::Arc;
use tauri::{AppHandle, Manager, State};

/// Get missing detection platforms for a specific game
pub fn get_missing_detections(
    app: &GHUBApp,
    available_patches: &[Detection],
) -> Vec<Detection> {
    let mut missing = Vec::new();

    for patch_detection in available_patches {
        // Check if this detection type already exists in the app
        let already_exists = app.detection.iter().any(|existing_detection| {
            detection_types_match(existing_detection, patch_detection)
        });

        if !already_exists {
            missing.push(patch_detection.clone());
        }
    }

    missing
}

/// Check if two Detection variants are of the same type (e.g., both Steam, both EpicGames, etc.)
fn detection_types_match(a: &Detection, b: &Detection) -> bool {
    match (a, b) {
        (Detection::Steam { .. }, Detection::Steam { .. }) => true,
        (Detection::WinRegistry { .. }, Detection::WinRegistry { .. }) => true,
        (Detection::EpicGames { .. }, Detection::EpicGames { .. }) => true,
        (Detection::OsxBundle { .. }, Detection::OsxBundle { .. }) => true,
        (Detection::Uplay { .. }, Detection::Uplay { .. }) => true,
        (Detection::GogGalaxy { .. }, Detection::GogGalaxy { .. }) => true,
        (Detection::HumbleApp { .. }, Detection::HumbleApp { .. }) => true,
        (Detection::RiotGames { .. }, Detection::RiotGames { .. }) => true,
        (Detection::Glob { .. }, Detection::Glob { .. }) => true,
        _ => false,
    }
}

/// Apply a single detection patch to a game via WebSocket
pub async fn apply_patch_to_game(
    app_handle: &AppHandle,
    ws_client: &Arc<WebSocketClient>,
    app_id: &str,
    detection: Detection,
) -> Result<(), String> {
    // Get the current application from state and update it
    let updated_app = {
        let state: State<AppState> = app_handle.state();
        let mut apps = state
            .applications
            .lock()
            .map_err(|e| format!("Failed to acquire lock on applications: {}", e))?;

        let app = apps
            .iter_mut()
            .find(|a| a.application_id == app_id)
            .ok_or_else(|| format!("Application with ID '{}' not found", app_id))?;

        // Add the detection to the app's detection vector
        app.detection.push(detection.clone());

        // Clone the updated app before releasing the lock
        app.clone()
    }; // Lock is released here

    // Send the updated app to G HUB via WebSocket
    let payload = serde_json::to_value(updated_app)
        .map_err(|e| format!("Failed to serialize application: {}", e))?;

    let message = WebSocketMessage {
        verb: "SET".to_string(),
        path: "/application".to_string(),
        payload,
    };

    ws_client
        .send_message(message)
        .await
        .map_err(|e| format!("Failed to send WebSocket message: {}", e))?;

    Ok(())
}

/// Apply patches for a single game
pub async fn apply_patches_for_game(
    app_handle: &AppHandle,
    ws_client: &Arc<WebSocketClient>,
    patches_data: &DetectionPatchesData,
    app_id: &str,
) -> Result<Vec<Detection>, String> {
    // Get patches for this game
    let game_patches = get_patches_for_game(patches_data, app_id)
        .ok_or_else(|| format!("No patches available for game '{}'", app_id))?;

    // Get the current app and find missing detections (within a block to ensure lock is released)
    let missing_detections = {
        let state: State<AppState> = app_handle.state();
        let apps = state
            .applications
            .lock()
            .map_err(|e| format!("Failed to acquire lock on applications: {}", e))?;

        let app = apps
            .iter()
            .find(|a| a.application_id == app_id)
            .ok_or_else(|| format!("Application with ID '{}' not found", app_id))?
            .clone();

        // Find missing detections before releasing the lock
        get_missing_detections(&app, &game_patches.detections)
    }; // Lock is released here

    if missing_detections.is_empty() {
        return Ok(Vec::new());
    }

    // Apply each missing detection
    for detection in &missing_detections {
        apply_patch_to_game(app_handle, ws_client, app_id, detection.clone()).await?;
    }

    Ok(missing_detections)
}

/// Apply patches to all games that have missing detections
pub async fn apply_all_patches(
    app_handle: &AppHandle,
    ws_client: &Arc<WebSocketClient>,
) -> Result<AppliedPatchesData, String> {
    // Load the bundled patches
    let patches_data = load_detection_patches(app_handle)?;

    // Get all applications
    let state: State<AppState> = app_handle.state();
    let apps = state
        .applications
        .lock()
        .map_err(|e| format!("Failed to acquire lock on applications: {}", e))?
        .clone();

    drop(apps); // Release the lock early

    let mut per_game_overrides = Vec::new();

    // Apply patches for each game that has available patches
    for game_patch in &patches_data.patches {
        match apply_patches_for_game(
            app_handle,
            ws_client,
            &patches_data,
            &game_patch.application_id,
        )
        .await
        {
            Ok(applied_detections) if !applied_detections.is_empty() => {
                let override_entry = GamePatchOverride {
                    application_id: game_patch.application_id.clone(),
                    game_name: game_patch.game_name.clone(),
                    applied_detections,
                    applied_at: chrono::Utc::now().to_rfc3339(),
                };
                per_game_overrides.push(override_entry);
            }
            Ok(_) => {
                // No patches applied for this game (already has all detections)
            }
            Err(e) => {
                eprintln!(
                    "Warning: Failed to apply patches for game '{}': {}",
                    game_patch.game_name, e
                );
                // Continue with other games even if one fails
            }
        }
    }

    // Create the applied patches data
    let applied_data = AppliedPatchesData {
        apply_all_defaults: true,
        applied_at: Some(chrono::Utc::now().to_rfc3339()),
        per_game_overrides,
    };

    // Save to disk
    save_applied_patches(app_handle, &applied_data)?;

    Ok(applied_data)
}

/// Reapply saved patches on startup
pub async fn reapply_saved_patches(
    app_handle: &AppHandle,
    ws_client: &Arc<WebSocketClient>,
) -> Result<(), String> {
    // Load saved patches
    let applied_data = load_applied_patches(app_handle)?;

    if !applied_data.apply_all_defaults && applied_data.per_game_overrides.is_empty() {
        // No patches to reapply
        return Ok(());
    }

    if applied_data.apply_all_defaults {
        // User had "apply all" enabled - reapply all patches
        apply_all_patches(app_handle, ws_client).await?;
    } else {
        // Reapply individual game patches
        for game_override in &applied_data.per_game_overrides {
            for detection in &game_override.applied_detections {
                if let Err(e) = apply_patch_to_game(
                    app_handle,
                    ws_client,
                    &game_override.application_id,
                    detection.clone(),
                )
                .await
                {
                    eprintln!(
                        "Warning: Failed to reapply patch for game '{}': {}",
                        game_override.game_name, e
                    );
                }
            }
        }
    }

    Ok(())
}
