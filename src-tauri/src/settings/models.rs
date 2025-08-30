use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Setting {
    pub key: String,
    pub label: String,
    pub description: Option<String>,
    pub category: SettingCategory,
    pub default_value: Value,
    pub setting_type: SettingType,
    pub requires_restart: bool,
    /// If true, this setting has a system side-effect handled by an adapter.
    pub system_managed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SettingCategory {
    General,
    Interface,
    Paths,
    Advanced,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SettingType {
    Toggle,
    Text {
        placeholder: Option<String>,
        validation: Option<TextValidation>,
    },
    Number {
        min: Option<f64>,
        max: Option<f64>,
        step: Option<f64>,
        unit: Option<String>,
    },
    Select {
        options: Vec<SelectOption>,
    },
    Path {
        directory: bool,
        extensions: Option<Vec<String>>,
    },
    Color,
    Keybind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextValidation {
    pub pattern: Option<String>, // regex
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
}

/// What the FE typically binds to (separate from the Setting descriptor).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SettingItemState {
    pub key: String,
    pub user_value: Value,
    pub effective_value: Value,
    pub in_sync: bool,
    pub capable: bool,
    pub error: Option<String>,
}

/// Keep SettingsState under AppState (as requested).
#[derive(Debug, Default)]
pub struct SettingsState {
    pub items: Vec<SettingItemState>,
}
