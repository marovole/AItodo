CREATE TABLE IF NOT EXISTS todos (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    url TEXT,
    status TEXT DEFAULT 'pending' CHECK(status IN ('pending','researching','review','done','archived')),
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS research_results (
    id TEXT PRIMARY KEY,
    todo_id TEXT NOT NULL REFERENCES todos(id) ON DELETE CASCADE,
    source TEXT DEFAULT 'chatgpt',
    content TEXT,
    raw_html TEXT,
    started_at DATETIME,
    completed_at DATETIME,
    duration_seconds INTEGER
);
