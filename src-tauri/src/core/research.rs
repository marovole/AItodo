use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchResult {
    pub id: String,
    pub todo_id: String,
    pub source: String,
    pub content: Option<String>,
    pub raw_html: Option<String>,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub duration_seconds: Option<i64>,
}
