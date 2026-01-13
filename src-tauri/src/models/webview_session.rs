use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebViewSession {
    pub id: String,
    pub service: String,
    pub user_data_dir: String,
    pub cookies_file: Option<String>,
    pub last_active: DateTime<Utc>,
    pub is_logged_in: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWebViewSessionRequest {
    pub service: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginStatus {
    pub service: String,
    pub is_logged_in: bool,
    pub last_checked: DateTime<Utc>,
    pub requires_action: Option<String>,
}