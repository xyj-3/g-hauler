mod websocket;
mod settings;
mod core;
mod applications;

use std::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(tauri_plugin_autostart::MacosLauncher::LaunchAgent, None))
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(core::state::AppState {
            applications: Mutex::new(Vec::new()),
            settings_db_data: Mutex::new(None),
        })
        .manage(websocket::init_websocket_state())
        .setup(|app| {
            let handle = app.handle();

            // Create backup of SQLite database on startup
            if let Err(e) = crate::applications::settings_db::backup_sqlite_on_startup() {
                eprintln!("Failed to backup SQLite database: {}", e);
            }

            // Load SQLite data into app state
            if let Err(e) = crate::applications::settings_db::load_and_store_sqlite_data(&handle) {
                eprintln!("Failed to load SQLite data: {}", e);
            }

            if let Err(e) = tauri::async_runtime::block_on(crate::core::store::initialize_store(&handle))
            {
                eprintln!("Failed to initialize store: {}", e);
            }
            if let Err(e) = crate::applications::applications_json::initialize_applications_on_startup(&handle) {
                eprintln!("Failed to initialize applications: {}", e);
            }

            if let Err(e) = crate::settings::autostart::init_auto_start(&handle) {
                eprintln!("Failed to sync autostart setting: {}", e);
            }

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
            crate::applications::settings_db::load_applications_from_sqlite,
            crate::applications::settings_db::save_applications_to_sqlite,
            crate::settings::autostart::enable_auto_start,
            crate::settings::autostart::disable_auto_start,
            crate::settings::autostart::is_auto_start_enabled,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
