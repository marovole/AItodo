use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Setting {
    pub key: String,
    pub value: String,
    pub setting_type: SettingType,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SettingType {
    String,
    Number,
    Boolean,
    Json,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSettingRequest {
    pub value: String,
    pub setting_type: Option<SettingType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub theme: String,
    pub stay_on_top: bool,
    pub notification_enabled: bool,
    pub default_service: String,
    pub research_queue_enabled: bool,
    pub auto_save_enabled: bool,
    pub compact_mode: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: "system".to_string(),
            stay_on_top: false,
            notification_enabled: true,
            default_service: "chatgpt".to_string(),
            research_queue_enabled: true,
            auto_save_enabled: true,
            compact_mode: false,
        }
    }
}