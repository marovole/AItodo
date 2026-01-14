use crate::models::todo::*;
use crate::db::{Database, todo_service, research_service};
use tauri::{command, AppHandle, State};

pub struct AppState {
    db: Option<Database>,
}

impl AppState {
    pub fn new() -> Self {
        Self { db: None }
    }

    pub fn init_db(&mut self, db: Database) {
        self.db = Some(db);
    }
}

#[command]
pub async fn create_todo(
    app: AppHandle,
    state: State<'_, AppState>,
    request: CreateTodoRequest,
) -> Result<Todo, String> {
    match &state.db {
        Some(db) => {
            let service = todo_service::TodoService::new(db.clone());
            service.create_todo(request)
                .map_err(|e| e.to_string())
        }
        None => Err("Database not initialized".to_string())
    }
}

#[command]
pub async fn update_todo(
    app: AppHandle,
    state: State<'_, AppState>,
    id: String,
    request: UpdateTodoRequest,
) -> Result<Option<Todo>, String> {
    match &state.db {
        Some(db) => {
            let service = todo_service::TodoService::new(db);
            service.update_todo(&id, request)
                .map_err(|e| e.to_string())
        }
        None => Err("Database not initialized".to_string())
    }
}

#[command]
pub async fn delete_todo(
    app: AppHandle,
    state: State<'_, AppState>,
    id: String,
) -> Result<bool, String> {
    match &state.db {
        Some(db) => {
            let service = todo_service::TodoService::new(db);
            service.delete_todo(&id)
                .map_err(|e| e.to_string())
        }
        None => Err("Database not initialized".to_string())
    }
}

#[command]
pub async fn get_todo(
    app: AppHandle,
    state: State<'_, AppState>,
    id: String,
) -> Result<Option<Todo>, String> {
    match &state.db {
        Some(db) => {
            let service = todo_service::TodoService::new(db);
            service.get_todo(&id)
                .map_err(|e| e.to_string())
        }
        None => Err("Database not initialized".to_string())
    }
}

#[command]
pub async fn list_todos(
    app: AppHandle,
    state: State<'_, AppState>,
    filter: Option<TodoFilter>,
) -> Result<Vec<Todo>, String> {
    match &state.db {
        Some(db) => {
            let service = todo_service::TodoService::new(db);
            let filter = filter.unwrap_or(TodoFilter {
                status: None,
                search: None,
                priority: None,
                limit: None,
                offset: None,
            });
            
            service.list_todos(filter)
                .map_err(|e: rusqlite::Error| e.to_string())
        }
        None => Err("Database not initialized".to_string())
    }
}

#[command]
pub async fn start_research(
    app: AppHandle,
    state: State<'_, AppState>,
    request: CreateResearchRequest,
) -> Result<ResearchProgress, String> {
    match &state.db {
        Some(db) => {
            let service = research_service::ResearchService::new(db);
            service.create_research_request(request)
                .map_err(|e: rusqlite::Error| e.to_string())
        }
        None => Err("Database not initialized".to_string())
    }
}

#[command]
pub async fn get_research_results(
    app: AppHandle,
    state: State<'_, AppState>,
    todo_id: String,
) -> Result<Vec<ResearchResult>, String> {
    match &state.db {
        Some(db) => {
            let service = research_service::ResearchService::new(db);
            service.get_research_results(&todo_id)
                .map_err(|e: rusqlite::Error| e.to_string())
        }
        None => Err("Database not initialized".to_string())
    }
}

#[command]
pub async fn get_research_progress(
    app: AppHandle,
    state: State<'_, AppState>,
    todo_id: String,
    service: String,
) -> Result<Option<ResearchProgress>, String> {
    match &state.db {
        Some(db) => {
            let svc = research_service::ResearchService::new(db);
            svc.get_research_progress(&todo_id, &service)
                .map_err(|e: rusqlite::Error| e.to_string())
        }
        None => Err("Database not initialized".to_string())
    }
}