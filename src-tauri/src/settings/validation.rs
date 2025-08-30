use regex::Regex;
use serde_json::Value;
use std::collections::HashSet;

use super::models::{Setting, SettingType};

/// Validate the in-code registry once at startup:
/// - unique keys
/// - default_value type matches SettingType + constraints
/// - policy: only Toggle may be system_managed (adjust if you extend)
pub fn validate_registry(settings: &[Setting]) -> Result<(), String> {
    let mut seen = HashSet::new();
    for s in settings {
        if !seen.insert(&s.key) {
            return Err(format!("duplicate setting key: {}", s.key));
        }
    }

    // per-kind default checks
    for s in settings {
        match &s.setting_type {
            SettingType::Toggle => {
                if !s.default_value.is_boolean() {
                    return Err(fmt_err(&s.key, "boolean", "toggle"));
                }
            }

            SettingType::Text { validation, .. } => {
                let Some(def) = s.default_value.as_str() else {
                    return Err(fmt_err(&s.key, "string", "text"));
                };
                if let Some(v) = validation {
                    if let Some(min) = v.min_length {
                        if def.chars().count() < min {
                            return Err(format!("'{}' default below min_length {}", s.key, min));
                        }
                    }
                    if let Some(max) = v.max_length {
                        if def.chars().count() > max {
                            return Err(format!("'{}' default above max_length {}", s.key, max));
                        }
                    }
                    if let Some(pat) = &v.pattern {
                        let re = Regex::new(pat).map_err(|e| format!("bad regex for '{}': {}", s.key, e))?;
                        if !re.is_match(def) {
                            return Err(format!("'{}' default does not match pattern", s.key));
                        }
                    }
                }
                if s.system_managed {
                    return Err(sys_only_toggle(&s.key));
                }
            }

            SettingType::Number { min, max, .. } => {
                let Some(def) = as_f64(&s.default_value) else {
                    return Err(fmt_err(&s.key, "number", "number"));
                };
                if let Some(lo) = min {
                    if def < *lo {
                        return Err(format!("'{}' default below min {}", s.key, lo));
                    }
                }
                if let Some(hi) = max {
                    if def > *hi {
                        return Err(format!("'{}' default above max {}", s.key, hi));
                    }
                }
                if s.system_managed {
                    return Err(sys_only_toggle(&s.key));
                }
            }

            SettingType::Select { options } => {
                let Some(def) = s.default_value.as_str() else {
                    return Err(fmt_err(&s.key, "string", "select"));
                };
                if options.is_empty() {
                    return Err(format!("'{}' select has empty options", s.key));
                }
                if !options.iter().any(|o| o.value == def) {
                    return Err(format!(
                        "'{}' default '{}' is not one of the allowed option values",
                        s.key, def
                    ));
                }
                if s.system_managed {
                    return Err(sys_only_toggle(&s.key));
                }
            }

            SettingType::Path { .. } => {
                // Keep permissive: FE pickers enforce UX; later you can add OS checks.
                if !s.default_value.is_string() {
                    return Err(fmt_err(&s.key, "string path", "path"));
                }
                if s.system_managed {
                    return Err(sys_only_toggle(&s.key));
                }
            }

            SettingType::Color => {
                if !s.default_value.is_string() {
                    return Err(fmt_err(&s.key, "string", "color"));
                }
                if s.system_managed {
                    return Err(sys_only_toggle(&s.key));
                }
            }

            SettingType::Keybind => {
                if !s.default_value.is_string() {
                    return Err(fmt_err(&s.key, "string", "keybind"));
                }
                if s.system_managed {
                    return Err(sys_only_toggle(&s.key));
                }
            }
        }
    }

    Ok(())
}

/// Validate a runtime write *for a single setting* (simple, contained here).
pub fn validate_runtime_value(s: &Setting, v: &Value) -> Result<(), String> {
    use SettingType::*;

    match &s.setting_type {
        Toggle => {
            if !v.is_boolean() { return Err(fmt_err(&s.key, "boolean", "toggle")); }
        }
        Text { validation, .. } => {
            let Some(txt) = v.as_str() else { return Err(fmt_err(&s.key, "string", "text")); };
            if let Some(val) = validation {
                if let Some(min) = val.min_length { if txt.chars().count() < min { return Err(format!("'{}' min_length {}", s.key, min)); } }
                if let Some(max) = val.max_length { if txt.chars().count() > max { return Err(format!("'{}' max_length {}", s.key, max)); } }
                if let Some(pat) = &val.pattern {
                    let re = Regex::new(pat).map_err(|e| format!("bad pattern for '{}': {}", s.key, e))?;
                    if !re.is_match(txt) { return Err(format!("'{}' does not match pattern", s.key)); }
                }
            }
        }
        Number { min, max, .. } => {
            let num = v.as_f64().ok_or_else(|| fmt_err(&s.key, "number", "number"))?;
            if let Some(lo) = min { if num < *lo { return Err(format!("'{}' below min {}", s.key, lo)); } }
            if let Some(hi) = max { if num > *hi { return Err(format!("'{}' above max {}", s.key, hi)); } }
        }
        Select { options } => {
            let sel = v.as_str().ok_or_else(|| fmt_err(&s.key, "string", "select"))?;
            if !options.iter().any(|o| o.value == sel) {
                return Err(format!("'{}' invalid option '{}'", s.key, sel));
            }
        }
        Path { .. } => {
            // Keep loose: FE path pickers should ensure validity; you can add OS checks later
            if !v.is_string() { return Err(fmt_err(&s.key, "string path", "path")); }
        }
        Color => {
            if !v.is_string() { return Err(fmt_err(&s.key, "string", "color")); }
        }
        Keybind => {
            if !v.is_string() { return Err(fmt_err(&s.key, "string", "keybind")); }
        }
    }
    Ok(())
}

/* helpers */
fn as_f64(v: &Value) -> Option<f64> {
    v.as_f64()
        .or_else(|| v.as_i64().map(|i| i as f64))
        .or_else(|| v.as_u64().map(|u| u as f64))
}

fn fmt_err(key: &str, want: &str, kind: &str) -> String {
    format!("'{}' default must be {} for '{}'", key, want, kind)
}

fn sys_only_toggle(key: &str) -> String {
    format!("'{}': system_managed is only supported for 'toggle'", key)
}
