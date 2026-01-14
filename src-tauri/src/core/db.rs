use std::path::PathBuf;
use std::sync::Mutex;

use once_cell::sync::OnceCell;
use rusqlite::{Connection, Result as SqliteResult};
use tauri::{AppHandle, Manager};

static DB_INSTANCE: OnceCell<Mutex<Connection>> = OnceCell::new();

/// Get the database file path
pub fn get_db_path(app: &AppHandle) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let data_dir = app.path().app_data_dir()?;

    std::fs::create_dir_all(&data_dir)?;
    Ok(data_dir.join("aitodo.db"))
}

/// Initialize the database connection and create tables
pub fn init_db(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let db_path = get_db_path(app)?;
    let conn = Connection::open(&db_path)?;

    // Enable foreign keys
    conn.execute("PRAGMA foreign_keys = ON", [])?;

    // Create tables using migration file
    let migration_sql = include_str!("../migrations/001_init.sql");
    conn.execute_batch(migration_sql)?;
    ensure_research_results_raw_html_column(&conn)?;

    // Store the connection
    DB_INSTANCE
        .set(Mutex::new(conn))
        .map_err(|_| "Database already initialized")?;

    Ok(())
}

fn ensure_research_results_raw_html_column(conn: &Connection) -> SqliteResult<()> {
    let mut stmt = conn.prepare("PRAGMA table_info(research_results)")?;
    let columns = stmt.query_map([], |row| row.get::<_, String>(1))?;

    let mut has_raw_html = false;
    for column in columns {
        if column.as_deref() == Ok("raw_html") {
            has_raw_html = true;
            break;
        }
    }

    if !has_raw_html {
        conn.execute("ALTER TABLE research_results ADD COLUMN raw_html TEXT", [])?;
    }

    Ok(())
}

/// Get a reference to the database connection
pub fn get_db() -> &'static Mutex<Connection> {
    DB_INSTANCE.get().expect("Database not initialized")
}

#[allow(dead_code)]
/// Get a setting value
pub fn get_setting(key: &str) -> SqliteResult<Option<String>> {
    let conn = get_db().lock().unwrap();
    let mut stmt = conn.prepare("SELECT value FROM settings WHERE key = ?")?;
    let mut rows = stmt.query([key])?;

    if let Some(row) = rows.next()? {
        Ok(Some(row.get(0)?))
    } else {
        Ok(None)
    }
}

#[allow(dead_code)]
/// Set a setting value
pub fn set_setting(key: &str, value: &str) -> SqliteResult<()> {
    let conn = get_db().lock().unwrap();
    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value) VALUES (?, ?)",
        [key, value],
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_tables() {
        let conn = Connection::open_in_memory().unwrap();
        let migration_sql = include_str!("../migrations/001_init.sql");
        conn.execute_batch(migration_sql).unwrap();

        // Verify tables exist
        let mut stmt = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")
            .unwrap();
        let tables: Vec<String> = stmt
            .query_map([], |row| row.get(0))
            .unwrap()
            .filter_map(|r| r.ok())
            .collect();

        assert!(tables.contains(&"todos".to_string()));
        assert!(tables.contains(&"research_results".to_string()));
    }
}
