use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub url: Option<String>,
    pub status: TodoStatus,
    pub priority: TodoPriority,
    pub preferred_service: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TodoStatus {
    Pending,
    Researching,
    Review,
    Done,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TodoPriority {
    Low,
    Medium,
    High,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTodoRequest {
    pub title: String,
    pub description: Option<String>,
    pub url: Option<String>,
    pub priority: Option<TodoPriority>,
    pub preferred_service: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTodoRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub status: Option<TodoStatus>,
    pub priority: Option<TodoPriority>,
    pub preferred_service: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoFilter {
    pub status: Option<TodoStatus>,
    pub search: Option<String>,
    pub priority: Option<TodoPriority>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}