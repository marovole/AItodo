use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchResult {
    pub id: String,
    pub todo_id: String,
    pub service: String,
    pub prompt: Option<String>,
    pub content: String,
    pub raw_html: Option<String>,
    pub metadata: Option<ResearchMetadata>,
    pub started_at: DateTime<Utc>,
    pub completed_at: DateTime<Utc>,
    pub duration_seconds: i32,
    pub status: ResearchStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchMetadata {
    pub sources: Vec<String>,
    pub confidence: Option<f64>,
    pub keywords: Vec<String>,
    pub additional_info: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResearchStatus {
    Completed,
    Failed,
    Timeout,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateResearchRequest {
    pub todo_id: String,
    pub service: String,
    pub prompt: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResearchProgress {
    pub todo_id: String,
    pub service: String,
    pub stage: ResearchStage,
    pub progress_percentage: i32,
    pub current_step: Option<String>,
    pub estimated_remaining: Option<i32>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResearchStage {
    Searching,
    Analyzing,
    Synthesizing,
    Completed,
}