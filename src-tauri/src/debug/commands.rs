use tauri::{AppHandle, WebviewWindow};
use crate::core::store::get_store_key;
use crate::core::constants::STORE_KEY_DEVELOPER_MODE;

#[tauri::command]
pub async fn is_developer_mode(app_handle: AppHandle) -> bool {
    get_store_key(&app_handle, STORE_KEY_DEVELOPER_MODE)
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
}

#[tauri::command]
pub async fn open_devtools(app_handle: AppHandle, window: WebviewWindow) -> Result<(), String> {
    // Check if developer mode is enabled
    let is_dev_mode = get_store_key(&app_handle, STORE_KEY_DEVELOPER_MODE)
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if !is_dev_mode {
        return Err("Developer mode is not enabled".to_string());
    }

    #[cfg(debug_assertions)]
    {
        window.open_devtools();
        Ok(())
    }

    #[cfg(not(debug_assertions))]
    {
        Err("DevTools are only available in debug builds".to_string())
    }
}

#[tauri::command]
pub async fn close_devtools(app_handle: AppHandle, window: WebviewWindow) -> Result<(), String> {
    // Check if developer mode is enabled
    let is_dev_mode = get_store_key(&app_handle, STORE_KEY_DEVELOPER_MODE)
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if !is_dev_mode {
        return Err("Developer mode is not enabled".to_string());
    }

    #[cfg(debug_assertions)]
    {
        window.close_devtools();
        Ok(())
    }

    #[cfg(not(debug_assertions))]
    {
        Err("DevTools are only available in debug builds".to_string())
    }
}

#[tauri::command]
pub async fn is_devtools_open(window: WebviewWindow) -> bool {
    #[cfg(debug_assertions)]
    {
        window.is_devtools_open()
    }

    #[cfg(not(debug_assertions))]
    {
        false
    }
}
