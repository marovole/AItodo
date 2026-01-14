use rusqlite::{Connection, Result, params};
use std::path::Path;

pub mod todo_service;
pub mod research_service;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)?;
        let db = Database { conn };
        db.init()?;
        Ok(db)
    }

    fn init(&self) -> Result<()> {
        self.conn.execute_batch(
            r#"
            -- Core Todos table
            CREATE TABLE IF NOT EXISTS todos (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                description TEXT,
                url TEXT,
                status TEXT NOT NULL DEFAULT 'pending',
                priority INTEGER NOT NULL DEFAULT 0,
                preferred_service TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                completed_at DATETIME
            );

            -- Research results storage
            CREATE TABLE IF NOT EXISTS research_results (
                id TEXT PRIMARY KEY,
                todo_id TEXT NOT NULL,
                service TEXT NOT NULL,
                prompt TEXT,
                content TEXT NOT NULL,
                raw_html TEXT,
                metadata TEXT,
                started_at DATETIME,
                completed_at DATETIME,
                duration_seconds INTEGER,
                status TEXT DEFAULT 'completed',
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (todo_id) REFERENCES todos(id) ON DELETE CASCADE
            );

            -- Application settings
            CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                type TEXT DEFAULT 'string',
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );

            -- WebView session persistence
            CREATE TABLE IF NOT EXISTS webview_sessions (
                id TEXT PRIMARY KEY,
                service TEXT NOT NULL,
                user_data_dir TEXT NOT NULL,
                cookies_file TEXT,
                last_active DATETIME,
                is_logged_in BOOLEAN DEFAULT FALSE,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );

            -- Research progress tracking
            CREATE TABLE IF NOT EXISTS research_progress (
                todo_id TEXT NOT NULL,
                service TEXT NOT NULL,
                stage TEXT NOT NULL,
                progress_percentage INTEGER DEFAULT 0,
                current_step TEXT,
                estimated_remaining INTEGER,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                PRIMARY KEY (todo_id, service),
                FOREIGN KEY (todo_id) REFERENCES todos(id) ON DELETE CASCADE
            );

            -- Create indexes for better performance
            CREATE INDEX IF NOT EXISTS idx_todos_status ON todos(status);
            CREATE INDEX IF NOT EXISTS idx_todos_created_at ON todos(created_at);
            CREATE INDEX IF NOT EXISTS idx_research_results_todo_id ON research_results(todo_id);
            CREATE INDEX IF NOT EXISTS idx_research_results_service ON research_results(service);
            CREATE INDEX IF NOT EXISTS idx_research_progress_todo_service ON research_progress(todo_id, service);
            "#,
        )?;
        
        self.init_default_settings()?;
        
        Ok(())
    }

    fn init_default_settings(&self) -> Result<()> {
        let settings = [
            ("theme", "system", "string"),
            ("stay_on_top", "false", "boolean"),
            ("notification_enabled", "true", "boolean"),
            ("default_service", "chatgpt", "string"),
            ("research_queue_enabled", "true", "boolean"),
            ("auto_save_enabled", "true", "boolean"),
            ("compact_mode", "false", "boolean"),
        ];

        for (key, value, setting_type) in settings.iter() {
            self.conn.execute(
                "INSERT OR IGNORE INTO settings (key, value, type) VALUES (?1, ?2, ?3)",
                params![key, value, setting_type],
            )?;
        }

        Ok(())
    }

    pub fn connection(&self) -> &Connection {
        &self.conn
    }
}