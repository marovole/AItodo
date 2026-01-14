use chrono::{DateTime, Utc};
use rusqlite::{params, Result as SqliteResult};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::db::get_db;
pub use crate::core::research::ResearchResult;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TodoStatus {
    Pending,
    Researching,
    Review,
    Done,
    Archived,
}

impl TodoStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TodoStatus::Pending => "pending",
            TodoStatus::Researching => "researching",
            TodoStatus::Review => "review",
            TodoStatus::Done => "done",
            TodoStatus::Archived => "archived",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "pending" => TodoStatus::Pending,
            "researching" => TodoStatus::Researching,
            "review" => TodoStatus::Review,
            "done" => TodoStatus::Done,
            "archived" => TodoStatus::Archived,
            _ => TodoStatus::Pending,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub url: Option<String>,
    pub status: TodoStatus,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTodoInput {
    pub title: String,
    pub description: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTodoInput {
    pub title: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub status: Option<TodoStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoWithResearch {
    pub todo: Todo,
    pub research: Option<ResearchResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusCounts {
    pub pending: i64,
    pub researching: i64,
    pub review: i64,
    pub done: i64,
    pub archived: i64,
    pub total: i64,
}

pub fn create_todo(input: CreateTodoInput) -> SqliteResult<Todo> {
    let conn = get_db().lock().unwrap();
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO todos (id, title, description, url, status, created_at, updated_at) 
         VALUES (?1, ?2, ?3, ?4, 'pending', ?5, ?5)",
        params![id, input.title, input.description, input.url, now],
    )?;

    Ok(Todo {
        id,
        title: input.title,
        description: input.description,
        url: input.url,
        status: TodoStatus::Pending,
        created_at: now.clone(),
        updated_at: now,
    })
}

pub fn get_todos(status_filter: Option<&str>) -> SqliteResult<Vec<Todo>> {
    let conn = get_db().lock().unwrap();

    let sql = match status_filter {
        Some(_) => {
            "SELECT id, title, description, url, status, created_at, updated_at 
                    FROM todos WHERE status = ?1 ORDER BY created_at DESC"
        }
        None => {
            "SELECT id, title, description, url, status, created_at, updated_at 
                 FROM todos WHERE status != 'archived' ORDER BY created_at DESC"
        }
    };

    let mut stmt = conn.prepare(sql)?;

    let rows = if let Some(status) = status_filter {
        stmt.query_map([status], row_to_todo)?
    } else {
        stmt.query_map([], row_to_todo)?
    };

    rows.collect()
}

pub fn get_todo_by_id(id: &str) -> SqliteResult<Option<Todo>> {
    let conn = get_db().lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, title, description, url, status, created_at, updated_at 
         FROM todos WHERE id = ?1",
    )?;

    let mut rows = stmt.query([id])?;
    if let Some(row) = rows.next()? {
        Ok(Some(row_to_todo(row)?))
    } else {
        Ok(None)
    }
}

pub fn get_todo_with_research(id: &str) -> SqliteResult<Option<TodoWithResearch>> {
    let todo = get_todo_by_id(id)?;

    if let Some(todo) = todo {
        let research = get_research_by_todo_id(&todo.id)?;
        Ok(Some(TodoWithResearch { todo, research }))
    } else {
        Ok(None)
    }
}

pub fn update_todo(id: &str, input: UpdateTodoInput) -> SqliteResult<Option<Todo>> {
    let conn = get_db().lock().unwrap();
    let now = Utc::now().to_rfc3339();

    let current = {
        let mut stmt = conn.prepare(
            "SELECT id, title, description, url, status, created_at, updated_at FROM todos WHERE id = ?1",
        )?;
        let mut rows = stmt.query([id])?;
        match rows.next()? {
            Some(row) => row_to_todo(row)?,
            None => return Ok(None),
        }
    };

    let new_title = input.title.unwrap_or(current.title);
    let new_description = input.description.or(current.description);
    let new_url = input.url.or(current.url);
    let new_status = input.status.unwrap_or(current.status);

    conn.execute(
        "UPDATE todos SET title = ?1, description = ?2, url = ?3, status = ?4, updated_at = ?5 
         WHERE id = ?6",
        params![
            new_title,
            new_description,
            new_url,
            new_status.as_str(),
            now,
            id
        ],
    )?;

    Ok(Some(Todo {
        id: id.to_string(),
        title: new_title,
        description: new_description,
        url: new_url,
        status: new_status,
        created_at: current.created_at,
        updated_at: now,
    }))
}

pub fn delete_todo(id: &str) -> SqliteResult<bool> {
    let conn = get_db().lock().unwrap();
    let affected = conn.execute("DELETE FROM todos WHERE id = ?1", [id])?;
    Ok(affected > 0)
}

pub fn get_status_counts() -> SqliteResult<StatusCounts> {
    let conn = get_db().lock().unwrap();
    let mut stmt = conn.prepare("SELECT status, COUNT(*) as count FROM todos GROUP BY status")?;

    let mut counts = StatusCounts {
        pending: 0,
        researching: 0,
        review: 0,
        done: 0,
        archived: 0,
        total: 0,
    };

    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
    })?;

    for row in rows {
        let (status, count) = row?;
        match status.as_str() {
            "pending" => counts.pending = count,
            "researching" => counts.researching = count,
            "review" => counts.review = count,
            "done" => counts.done = count,
            "archived" => counts.archived = count,
            _ => {}
        }
        counts.total += count;
    }

    Ok(counts)
}

pub fn save_research_result(
    todo_id: &str,
    source: &str,
    content: &str,
    raw_html: Option<&str>,
    started_at: &str,
) -> SqliteResult<ResearchResult> {
    let mut conn = get_db().lock().unwrap();
    let id = Uuid::new_v4().to_string();
    let completed_at = Utc::now().to_rfc3339();

    let started: DateTime<Utc> = started_at.parse().unwrap_or_else(|_| Utc::now());
    let completed: DateTime<Utc> = completed_at.parse().unwrap_or_else(|_| Utc::now());
    let duration = (completed - started).num_seconds();

    let tx = conn.transaction()?;

    tx.execute(
        "INSERT INTO research_results (id, todo_id, source, content, raw_html, started_at, completed_at, duration_seconds)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![id, todo_id, source, content, raw_html, started_at, completed_at, duration],
    )?;

    tx.execute(
        "UPDATE todos SET status = 'review', updated_at = ?1 WHERE id = ?2",
        params![completed_at, todo_id],
    )?;

    tx.commit()?;

    Ok(ResearchResult {
        id,
        todo_id: todo_id.to_string(),
        source: source.to_string(),
        content: Some(content.to_string()),
        raw_html: raw_html.map(|s| s.to_string()),
        started_at: Some(started_at.to_string()),
        completed_at: Some(completed_at),
        duration_seconds: Some(duration),
    })
}

pub fn get_research_by_todo_id(todo_id: &str) -> SqliteResult<Option<ResearchResult>> {
    let conn = get_db().lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, todo_id, source, content, raw_html, started_at, completed_at, duration_seconds
         FROM research_results WHERE todo_id = ?1 ORDER BY completed_at DESC LIMIT 1",
    )?;

    let mut rows = stmt.query([todo_id])?;
    if let Some(row) = rows.next()? {
        Ok(Some(ResearchResult {
            id: row.get(0)?,
            todo_id: row.get(1)?,
            source: row.get(2)?,
            content: row.get(3)?,
            raw_html: row.get(4)?,
            started_at: row.get(5)?,
            completed_at: row.get(6)?,
            duration_seconds: row.get(7)?,
        }))
    } else {
        Ok(None)
    }
}

fn row_to_todo(row: &rusqlite::Row) -> SqliteResult<Todo> {
    Ok(Todo {
        id: row.get(0)?,
        title: row.get(1)?,
        description: row.get(2)?,
        url: row.get(3)?,
        status: TodoStatus::from_str(&row.get::<_, String>(4)?),
        created_at: row.get(5)?,
        updated_at: row.get(6)?,
    })
}
