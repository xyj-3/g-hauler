mod g_hub;
mod g_hauler;
mod constants;
mod models;
mod settings_db;
mod store;
mod util;
mod validation;
mod settings;

mod autostart {
    pub use crate::settings::auto_start::*;
}

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
        .manage(models::AppState {
            applications: Mutex::new(Vec::new()),
            settings_db_data: Mutex::new(None),
        })
        .setup(|app| {
            let handle = app.handle();

            // Create backup of SQLite database on startup
            if let Err(e) = crate::g_hub::settings_db::backup_sqlite_on_startup() {
                eprintln!("Failed to backup SQLite database: {}", e);
            }

            // Load SQLite data into app state
            if let Err(e) = crate::g_hub::settings_db::load_and_store_sqlite_data(&handle) {
                eprintln!("Failed to load SQLite data: {}", e);
            }

            if let Err(e) = tauri::async_runtime::block_on(crate::g_hauler::store::initialize_store(&handle))
            {
                eprintln!("Failed to initialize store: {}", e);
            }
            if let Err(e) = crate::g_hub::applications_json::initialize_applications_on_startup(&handle) {
                eprintln!("Failed to initialize applications: {}", e);
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            crate::g_hauler::store::store_get_key,
            crate::g_hauler::store::store_set_key,
            crate::g_hub::util::get_pipeline_path,
            crate::g_hauler::validation::validate_paths,
            crate::g_hub::applications_json::get_applications,
            crate::g_hub::applications_json::update_application,
            crate::g_hub::applications_json::get_application_by_id,
            crate::g_hub::applications_json::save_applications_to_disk,
            crate::g_hub::settings_db::load_applications_from_sqlite,
            crate::g_hub::settings_db::save_applications_to_sqlite,
            crate::autostart::enable_auto_start,
            crate::autostart::disable_auto_start,
            crate::autostart::is_auto_start_enabled,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
