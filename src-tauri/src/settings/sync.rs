use serde_json::Value;
use tauri::AppHandle;

use crate::core::{state as app_state, store};
use crate::settings::{adapters, registry, validation};

/// Initializes and synchronizes SettingState for application
pub fn init(app: &AppHandle) -> Result<(), String> {

    ensure_defaults(app)?;
    sync_system_settings(app)?;
    app_state::refresh_settings_state(app)?;

    Ok(())
}

/// Ensure each registry key exists in store with a valid value; if missing, write default.
pub fn ensure_defaults(app: &AppHandle) -> Result<(), String> {
    for s in registry::all() {
        if store::get_store_key(app, &s.key).is_none() {
            store::set_store_key(app, &s.key, s.default_value.clone())?;
        }
    }
    Ok(())
}

/// Bring system to desired user values for system-managed keys.
pub fn sync_system_settings(app: &AppHandle) -> Result<(), String> {
    let reg = adapters::registry();
    for s in registry::all().iter().filter(|s| s.system_managed) {
        let desired = store::get_store_key(app, &s.key).unwrap_or_else(|| s.default_value.clone());
        if let Some(ad) = reg.get(s.key.as_str()) {
            let current = ad.read(app).unwrap_or(Value::Null);
            if current != desired {
                ad.apply(app, &desired)?;
            }
        }
    }
    Ok(())
}
