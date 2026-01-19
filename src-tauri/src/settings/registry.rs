use once_cell::sync::Lazy;
use serde_json::json;

use super::models::*;
use crate::core::constants::{STORE_KEY_AUTOSTART, STORE_KEY_MINIMIZE_TO_TRAY, STORE_KEY_IGDB_CLIENT_ID, STORE_KEY_IGDB_CLIENT_SECRET};

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
        Setting {
            key: STORE_KEY_IGDB_CLIENT_ID.into(),
            label: "IGDB Client ID".into(),
            description: Some("Client ID for IGDB API authentication".into()),
            category: SettingCategory::Advanced,
            default_value: json!(""),
            setting_type: SettingType::Text {
                placeholder: Some("Enter your IGDB Client ID".into()),
                validation: None,
            },
            requires_restart: false,
            system_managed: false,
        },
        Setting {
            key: STORE_KEY_IGDB_CLIENT_SECRET.into(),
            label: "IGDB Client Secret".into(),
            description: Some("Client Secret for IGDB API authentication".into()),
            category: SettingCategory::Advanced,
            default_value: json!(""),
            setting_type: SettingType::Text {
                placeholder: Some("Enter your IGDB Client Secret".into()),
                validation: None,
            },
            requires_restart: false,
            system_managed: false,
        },
    ]
});

pub fn all() -> &'static [Setting] {
    &SETTINGS_REGISTRY
}

pub fn find(key: &str) -> Option<&'static Setting> {
    SETTINGS_REGISTRY.iter().find(|s| s.key == key)
}
