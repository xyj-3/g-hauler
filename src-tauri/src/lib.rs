mod config;
mod constants;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            crate::config::store_get_key,
            crate::config::store_set_key,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
