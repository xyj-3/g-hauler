use once_cell::sync::Lazy;
use serde_json::json;

use super::models::*;
use crate::core::constants::{STORE_KEY_AUTOSTART, STORE_KEY_MINIMIZE_TO_TRAY};

pub static SETTINGS_REGISTRY: Lazy<Vec<Setting>> = Lazy::new(|| {
    vec![
        Setting {
            key: STORE_KEY_AUTOSTART.into(),
            label: "Launch on Startup".into(),
            description: Some("Start the app automatically when you sign in".into()),
            category: SettingCategory::General,
            default_value: json!(false),
            setting_type: SettingType::Toggle,
            requires_restart: false,
            system_managed: true,
        },
        Setting {
            key: STORE_KEY_MINIMIZE_TO_TRAY.into(),
            label: "Minimize to Tray".into(),
            description: Some("Hide the app to system tray when minimized instead of taskbar".into()),
            category: SettingCategory::General,
            default_value: json!(false),
            setting_type: SettingType::Toggle,
            requires_restart: false,
            system_managed: true,
        },
    ]
});

pub fn all() -> &'static [Setting] {
    &SETTINGS_REGISTRY
}

pub fn find(key: &str) -> Option<&'static Setting> {
    SETTINGS_REGISTRY.iter().find(|s| s.key == key)
}
