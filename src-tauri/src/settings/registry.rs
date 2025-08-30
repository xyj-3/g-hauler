use once_cell::sync::Lazy;
use serde_json::json;

use super::models::*;

pub static SETTINGS_REGISTRY: Lazy<Vec<Setting>> = Lazy::new(|| {
    vec![
        Setting {
            key: "autostart".into(),
            label: "Launch on Startup".into(),
            description: Some("Start the app automatically when you sign in".into()),
            category: SettingCategory::General,
            default_value: json!(false),
            setting_type: SettingType::Toggle,
            requires_restart: false,
            system_managed: true,
        },
        // Add more settings here (Text, Number, Select, Path, …)
    ]
});

pub fn all() -> &'static [Setting] {
    &SETTINGS_REGISTRY
}

pub fn find(key: &str) -> Option<&'static Setting> {
    SETTINGS_REGISTRY.iter().find(|s| s.key == key)
}
