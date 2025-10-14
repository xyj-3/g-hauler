mod websocket;
mod settings;
mod core;
mod applications;
mod detection_patches;

use std::sync::{Arc, Mutex};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(tauri_plugin_autostart::MacosLauncher::LaunchAgent, None))
        .plugin(tauri_plugin_log::Builder::new()
            .targets([
                tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Stdout),
                tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::LogDir { 
                    file_name: Some("app".to_string()) 
                }),
            ])
            .filter(|metadata| {
                // Filter out all tungstenite-related logs (including tokio_tungstenite)
                !metadata.target().starts_with("tungstenite") && 
                !metadata.target().starts_with("tokio_tungstenite")
            })
            .build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(core::state::AppState {
            applications: Mutex::new(Vec::new()),
            settings_state: Mutex::new(Default::default()),
        })
        .setup(|app| {
            let handle = app.handle().clone();
            
            // Initialize singleton WebSocket client
            let ws_client = Arc::new(websocket::WebSocketClient::new(handle.clone()));
            app.manage(ws_client);

            if let Err(e) = tauri::async_runtime::block_on(crate::core::store::initialize_store(&handle)) {
                eprintln!("Failed to initialize store: {}", e);
            }

            if let Err(e) = crate::applications::applications_json::initialize_applications_on_startup(&handle) {
                eprintln!("Failed to initialize applications: {}", e);
            }

            if let Err(e) = crate::settings::validation::validate_registry(crate::settings::registry::all()) {
                eprintln!("settings registry validation error: {e}");
            }

            if let Err(e) = crate::settings::sync::init(&handle) {
                eprintln!("Settings system sync error: {}", e);
            }

            // Check if G HUB version has changed and reapply patches if needed
            // This is done in a background task since it requires the WebSocket client
            let handle_clone = handle.clone();
            let ws_client_clone = app.state::<Arc<websocket::WebSocketClient>>().inner().clone();
            tauri::async_runtime::spawn(async move {
                // Give the app time to fully initialize before checking version
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

                // Check if G HUB version has changed
                match crate::applications::ghub_version::has_version_changed(&handle_clone) {
                    Ok(true) => {
                        println!("G HUB version has changed - reapplying detection patches");

                        if let Err(e) = crate::detection_patches::applier::reapply_saved_patches(
                            &handle_clone,
                            &ws_client_clone,
                        ).await {
                            eprintln!("Warning: Failed to reapply saved detection patches: {}", e);
                        } else {
                            // Update stored version after successful reapplication
                            if let Err(e) = crate::applications::ghub_version::update_stored_version(&handle_clone) {
                                eprintln!("Warning: Failed to update stored G HUB version: {}", e);
                            }
                        }
                    }
                    Ok(false) => {
                        println!("G HUB version unchanged - skipping patch reapplication");
                    }
                    Err(e) => {
                        eprintln!("Could not check G HUB version: {} - skipping patch reapplication", e);
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            crate::core::store::store_get_key,
            crate::core::store::store_set_key,
            crate::applications::paths::get_pipeline_path,
            crate::applications::validation::validate_paths,
            crate::applications::applications_json::get_applications,
            crate::applications::applications_json::update_application,
            crate::applications::applications_json::get_application_by_id,
            crate::applications::applications_json::save_applications_to_disk,
            crate::settings::commands::settings_get_registry,
            crate::settings::commands::settings_get_state,
            crate::settings::commands::settings_set_and_apply,
            crate::websocket::commands::ws_connect,
            crate::websocket::commands::ws_send_message,
            crate::websocket::commands::ws_disconnect,
            crate::websocket::commands::ws_is_connected,
            crate::detection_patches::commands::get_available_patches_for_game,
            crate::detection_patches::commands::patch_apply_single,
            crate::detection_patches::commands::patch_apply_for_game,
            crate::detection_patches::commands::patch_apply_all,
            crate::detection_patches::commands::patch_get_applied,
            crate::detection_patches::commands::patch_reapply_saved,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
