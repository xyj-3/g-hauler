use serde_json::Value;
use tauri::AppHandle;

use crate::core::store;
use super::{adapters, models::*, registry};

pub fn build_state(app: &AppHandle) -> Vec<SettingItemState> {
    let reg = adapters::registry();

    registry::all()
        .iter()
        .map(|s| {
            let user = store::get_store_key(app, &s.key).unwrap_or_else(|| s.default_value.clone());

            if s.system_managed {
                if let Some(ad) = reg.get(s.key.as_str()) {
                    let eff = ad.read(app).unwrap_or(Value::Null);
                    return SettingItemState {
                        key: s.key.clone(),
                        user_value: user.clone(),
                        effective_value: eff.clone(),
                        in_sync: user == eff,
                        capable: ad.capable(app),
                        error: None,
                    };
                }
            }

            SettingItemState {
                key: s.key.clone(),
                user_value: user.clone(),
                effective_value: user,
                in_sync: true,
                capable: true,
                error: None,
            }
        })
        .collect()
}
