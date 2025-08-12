use tauri::{AppHandle};
use tauri_plugin_autostart::ManagerExt;
use crate::shared::store;
use serde_json::{json, Value};

use crate::shared::constants::{STORE_KEY_AUTOSTART};

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

pub fn sync_auto_start_with_store(app: &AppHandle) -> Result<(), String> {
    let desired = match store::get_store_key(app, STORE_KEY_AUTOSTART) {
        Some(Value::Bool(val)) => val,
        _ => return Ok(()),
    };

    let current = app.autolaunch().is_enabled().unwrap_or(false);
    if desired == current {
        return Ok(());
    }

    let result = if desired {
        app.autolaunch().enable()
    } else {
        app.autolaunch().disable()
    };

    result.map_err(|e| e.to_string())?;
    println!("Synced autostart: store = {}, system = {} → updated", desired, current);
    Ok(())
}


pub fn init_auto_start(app: &AppHandle) -> Result<(), String> {
    if !matches!(store::get_store_key(app, STORE_KEY_AUTOSTART), Some(Value::Bool(_))) {
        store::set_store_key(app, STORE_KEY_AUTOSTART, json!(false))?;
        println!("Initialized autostart key in store to false.");
    }

    sync_auto_start_with_store(app)
}
