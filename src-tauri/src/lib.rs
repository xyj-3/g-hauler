mod store;
mod constants;
mod validation;
mod applications;
mod util;
mod ghub;

use std::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(applications::AppState {
            applications: Mutex::new(Vec::new()),
        })
        .setup(|app| {
            let handle = app.handle();
            if let Err(e) = tauri::async_runtime::block_on(crate::store::initialize_store(&handle)) {
                eprintln!("Failed to initialize store: {}", e);
            }
            if let Err(e) = crate::applications::initialize_applications_on_startup(&handle) {
                eprintln!("Failed to initialize applications: {}", e);
            }
            Ok(())
        }).invoke_handler(tauri::generate_handler![
            crate::store::store_get_key,
            crate::store::store_set_key,
            crate::util::get_pipeline_path,
            crate::validation::validate_paths,
            crate::applications::get_applications,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
