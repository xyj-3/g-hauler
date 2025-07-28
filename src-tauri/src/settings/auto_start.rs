use tauri_plugin_autostart::ManagerExt;

#[tauri::command]
pub fn enable_auto_start(app: tauri::AppHandle) -> Result<(), String> {
    {
        let autostart_manager = app.autolaunch();
        autostart_manager.enable().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub fn disable_auto_start(app: tauri::AppHandle) -> Result<(), String> {
    {
        let autostart_manager = app.autolaunch();
        autostart_manager.disable().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub fn is_auto_start_enabled(app: tauri::AppHandle) -> bool {
    {
        let autostart_manager = app.autolaunch();
        return autostart_manager.is_enabled().unwrap_or(false);
    }
}
