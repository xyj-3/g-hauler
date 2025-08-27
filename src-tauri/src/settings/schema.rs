use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashSet;

#[derive(Deserialize, Serialize, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SettingType {
    Switch,
    Text { placeholder: Option<String>, max_len: Option<u32> },
    Select { options: Vec<String> },
    Slider { min: i64, max: i64, step: i64 },
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct SettingSchema {
    pub key: String,
    pub label: String,
    #[serde(rename = "type")]
    pub kind: SettingType,
    pub default: Value,
    pub description: Option<String>,
    pub system_managed: bool,
}

const RAW_SCHEMA_JSON: &str = include_str!("./settings_schema.json");

pub fn load_schema() -> Result<Vec<SettingSchema>, String> {
    let mut items: Vec<SettingSchema> =
        serde_json::from_str(RAW_SCHEMA_JSON).map_err(|e| format!("schema parse error: {e}"))?;
    validate(&items)?;
    items.sort_by(|a, b| a.key.cmp(&b.key));
    Ok(items)
}

fn validate(items: &[SettingSchema]) -> Result<(), String> {
    let mut seen = HashSet::new();
    for s in items {
        if !seen.insert(&s.key) {
            return Err(format!("duplicate setting key: {}", s.key));
        }
    }

    for s in items {
        match &s.kind {
            SettingType::Switch => {
                if !s.default.is_boolean() {
                    return Err(bad_default(&s.key, "boolean", "switch"));
                }
                // policy: only switches may be system-managed
                // (relax this if you add other system-backed kinds)
            }

            SettingType::Text { max_len, .. } => {
                let Some(def) = s.default.as_str() else {
                    return Err(bad_default(&s.key, "string", "text"));
                };
                if let Some(limit) = max_len {
                    if def.chars().count() as u32 > *limit {
                        return Err(format!("'{}' default length exceeds max_len={}", s.key, limit));
                    }
                }
                if s.system_managed {
                    return Err(sys_only_switch(&s.key));
                }
            }

            SettingType::Select { options } => {
                let Some(def) = s.default.as_str() else {
                    return Err(bad_default(&s.key, "string", "select"));
                };
                if options.is_empty() {
                    return Err(format!("'{}' select has empty options", s.key));
                }
                if !options.iter().any(|o| o == def) {
                    return Err(format!(
                        "'{}' default '{}' is not one of the allowed options",
                        s.key, def
                    ));
                }
                if s.system_managed {
                    return Err(sys_only_switch(&s.key));
                }
            }

            SettingType::Slider { min, max, step } => {
                if min > max {
                    return Err(format!("'{}' slider has min({}) > max({})", s.key, min, max));
                }
                if *step <= 0 {
                    return Err(format!("'{}' slider step must be > 0", s.key));
                }
                let Some(def) = as_i64(&s.default) else {
                    return Err(bad_default(&s.key, "integer", "slider"));
                };
                if def < *min || def > *max {
                    return Err(format!(
                        "'{}' default {} out of range [{}, {}]",
                        s.key, def, min, max
                    ));
                }
                if s.system_managed {
                    return Err(sys_only_switch(&s.key));
                }
            }
        }
    }
    Ok(())
}

fn as_i64(v: &Value) -> Option<i64> {
    v.as_i64().or_else(|| v.as_u64().and_then(|u| (u <= i64::MAX as u64).then_some(u as i64)))
}
fn bad_default(key: &str, want: &str, kind: &str) -> String {
    format!("'{}' default must be {} for type '{}'", key, want, kind)
}
fn sys_only_switch(key: &str) -> String {
    format!("'{}': system_managed is only supported for 'switch'", key)
}
