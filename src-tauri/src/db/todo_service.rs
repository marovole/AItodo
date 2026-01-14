use crate::models::todo::*;
use crate::db::Database;
use rusqlite::{params, Result, OptionalExtension};
use uuid::Uuid;
use chrono::Utc;

pub struct TodoService {
    db: Database,
}

impl TodoService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub fn create_todo(&self, request: CreateTodoRequest) -> Result<Todo> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let priority = request.priority.unwrap_or(TodoPriority::Medium);

        let todo = Todo {
            id: id.clone(),
            title: request.title,
            description: request.description,
            url: request.url,
            status: TodoStatus::Pending,
            priority,
            preferred_service: request.preferred_service,
            created_at: now,
            updated_at: now,
            completed_at: None,
        };

        self.db.connection().execute(
            r#"
            INSERT INTO todos (id, title, description, url, status, priority, preferred_service, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
            "#,
            params![
                todo.id,
                todo.title,
                todo.description,
                todo.url,
                status_to_string(&todo.status),
                priority_to_i32(&todo.priority),
                todo.preferred_service,
                todo.created_at,
                todo.updated_at
            ],
        )?;

        Ok(todo)
    }

    pub fn update_todo(&self, id: &str, request: UpdateTodoRequest) -> Result<Option<Todo>> {
        if let Some(ref title) = request.title {
            self.db.connection().execute(
                "UPDATE todos SET title = ?, updated_at = ? WHERE id = ?",
                rusqlite::params![title, Utc::now(), id]
            )?;
        }
        
        if let Some(ref description) = request.description {
            self.db.connection().execute(
                "UPDATE todos SET description = ?, updated_at = ? WHERE id = ?",
                rusqlite::params![description, Utc::now(), id]
            )?;
        }
        
        if let Some(ref url) = request.url {
            self.db.connection().execute(
                "UPDATE todos SET url = ?, updated_at = ? WHERE id = ?",
                rusqlite::params![url, Utc::now(), id]
            )?;
        }
        
        if let Some(ref status) = request.status {
            self.db.connection().execute(
                "UPDATE todos SET status = ?, updated_at = ? WHERE id = ?",
                rusqlite::params![status_to_string(status), Utc::now(), id]
            )?;
        }
        
        if let Some(ref priority) = request.priority {
            self.db.connection().execute(
                "UPDATE todos SET priority = ?, updated_at = ? WHERE id = ?",
                rusqlite::params![priority_to_i32(priority), Utc::now(), id]
            )?;
        }
        
        if let Some(ref preferred_service) = request.preferred_service {
            self.db.connection().execute(
                "UPDATE todos SET preferred_service = ?, updated_at = ? WHERE id = ?",
                rusqlite::params![preferred_service, Utc::now(), id]
            )?;
        }

        self.get_todo(id)
    }

    pub fn delete_todo(&self, id: &str) -> Result<bool> {
        let rows_affected = self.db.connection().execute(
            "DELETE FROM todos WHERE id = ?",
            params![id],
        )?;
        Ok(rows_affected > 0)
    }

    pub fn get_todo(&self, id: &str) -> Result<Option<Todo>> {
        let mut stmt = self.db.connection().prepare(
            r#"
            SELECT id, title, description, url, status, priority, preferred_service, 
                   created_at, updated_at, completed_at
            FROM todos WHERE id = ?
            "#,
        )?;

        let todo = stmt.query_row(params![id], |row| {
            Ok(Todo {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                url: row.get(3)?,
                status: string_to_status(row.get(4)?),
                priority: i32_to_priority(row.get(5)?),
                preferred_service: row.get(6)?,
                created_at: row.get(7)?,
                updated_at: row.get(8)?,
                completed_at: row.get(9)?,
            })
        }).optional()?;

        Ok(todo)
    }

    pub fn list_todos(&self, filter: TodoFilter) -> Result<Vec<Todo>> {
        let mut conditions = Vec::new();
        let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(ref status) = filter.status {
            conditions.push("status = ?");
            params.push(Box::new(status_to_string(status)));
        }

        if let Some(ref priority) = filter.priority {
            conditions.push("priority = ?");
            params.push(Box::new(priority_to_i32(priority)));
        }

        if let Some(ref search) = filter.search {
            conditions.push("(title LIKE ? OR description LIKE ? OR url LIKE ?)");
            let search_pattern = format!("%{}%", search);
            params.push(Box::new(search_pattern.clone()));
            params.push(Box::new(search_pattern.clone()));
            params.push(Box::new(search_pattern));
        }

        let mut query = String::from("SELECT id, title, description, url, status, priority, preferred_service, created_at, updated_at, completed_at FROM todos");
        
        if !conditions.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&conditions.join(" AND "));
        }

        query.push_str(" ORDER BY priority DESC, created_at DESC");

        if let Some(limit) = filter.limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }
        if let Some(offset) = filter.offset {
            query.push_str(&format!(" OFFSET {}", offset));
        }

        let mut stmt = self.db.connection().prepare(&query)?;
        let todos = stmt.query_map(rusqlite::params_from_iter(params), |row| {
            Ok(Todo {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                url: row.get(3)?,
                status: string_to_status(row.get(4)?),
                priority: i32_to_priority(row.get(5)?),
                preferred_service: row.get(6)?,
                created_at: row.get(7)?,
                updated_at: row.get(8)?,
                completed_at: row.get(9)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;

        Ok(todos)
    }
}

fn status_to_string(status: &TodoStatus) -> String {
    match status {
        TodoStatus::Pending => "pending".to_string(),
        TodoStatus::Researching => "researching".to_string(),
        TodoStatus::Review => "review".to_string(),
        TodoStatus::Done => "done".to_string(),
        TodoStatus::Archived => "archived".to_string(),
    }
}

fn string_to_status(s: String) -> TodoStatus {
    match s.as_str() {
        "pending" => TodoStatus::Pending,
        "researching" => TodoStatus::Researching,
        "review" => TodoStatus::Review,
        "done" => TodoStatus::Done,
        "archived" => TodoStatus::Archived,
        _ => TodoStatus::Pending,
    }
}

fn priority_to_i32(priority: &TodoPriority) -> i32 {
    match priority {
        TodoPriority::Low => 0,
        TodoPriority::Medium => 1,
        TodoPriority::High => 2,
    }
}

fn i32_to_priority(i: i32) -> TodoPriority {
    match i {
        0 => TodoPriority::Low,
        1 => TodoPriority::Medium,
        2 => TodoPriority::High,
        _ => TodoPriority::Medium,
    }
}