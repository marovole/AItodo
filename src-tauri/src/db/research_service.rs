use crate::models::research_result::*;
use crate::db::Database;
use rusqlite::{params, Result, OptionalExtension};
use chrono::Utc;

pub struct ResearchService {
    db: Database,
}

impl ResearchService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub fn create_research_request(&self, request: CreateResearchRequest) -> Result<ResearchProgress> {
        let now = Utc::now();

        let progress = ResearchProgress {
            todo_id: request.todo_id.clone(),
            service: request.service.clone(),
            stage: ResearchStage::Searching,
            progress_percentage: 0,
            current_step: Some("Initializing research".to_string()),
            estimated_remaining: Some(300),
            updated_at: now,
        };

        self.db.connection().execute(
            r#"
            INSERT OR REPLACE INTO research_progress 
            (todo_id, service, stage, progress_percentage, current_step, estimated_remaining, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            "#,
            rusqlite::params![
                progress.todo_id,
                progress.service,
                stage_to_string(&progress.stage),
                progress.progress_percentage,
                progress.current_step,
                progress.estimated_remaining,
                progress.updated_at
            ],
        )?;

        Ok(progress)
    }

    pub fn update_research_progress(&self, progress: &ResearchProgress) -> Result<()> {
        self.db.connection().execute(
            r#"
            UPDATE research_progress 
            SET stage = ?, progress_percentage = ?, current_step = ?, 
                estimated_remaining = ?, updated_at = ?
            WHERE todo_id = ? AND service = ?
            "#,
            rusqlite::params![
                stage_to_string(&progress.stage),
                progress.progress_percentage,
                progress.current_step,
                progress.estimated_remaining,
                progress.updated_at,
                progress.todo_id,
                progress.service
            ],
        )?;

        Ok(())
    }

    pub fn complete_research(&self, todo_id: &str, result: ResearchResult) -> Result<ResearchResult> {
        let now = Utc::now();
        let completed_result = ResearchResult {
            completed_at: now,
            status: ResearchStatus::Completed,
            ..result
        };

        self.db.connection().execute(
            r#"
            INSERT INTO research_results 
            (id, todo_id, service, prompt, content, raw_html, metadata, 
             started_at, completed_at, duration_seconds, status, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)
            "#,
            rusqlite::params![
                completed_result.id,
                completed_result.todo_id,
                completed_result.service,
                completed_result.prompt,
                completed_result.content,
                completed_result.raw_html,
                completed_result.metadata.as_ref().map(|m| serde_json::to_string(m).ok()).flatten(),
                completed_result.started_at,
                completed_result.completed_at,
                completed_result.duration_seconds,
                status_to_string(&completed_result.status),
                completed_result.created_at
            ],
        )?;

        self.db.connection().execute(
            "DELETE FROM research_progress WHERE todo_id = ? AND service = ?",
            rusqlite::params![todo_id, completed_result.service]
        )?;

        self.db.connection().execute(
            "DELETE FROM research_progress WHERE todo_id = ? AND service = ?",
            params![todo_id, completed_result.service]
        )?;

        Ok(completed_result)
    }

    pub fn get_research_results(&self, todo_id: &str) -> Result<Vec<ResearchResult>> {
        let mut stmt = self.db.connection().prepare(
            r#"
            SELECT id, todo_id, service, prompt, content, raw_html, metadata,
                   started_at, completed_at, duration_seconds, status, created_at
            FROM research_results 
            WHERE todo_id = ? 
            ORDER BY completed_at DESC
            "#,
        )?;

        let results = stmt.query_map(params![todo_id], |row| {
            let metadata_str: Option<String> = row.get(6)?;
            let metadata = metadata_str
                .and_then(|s| serde_json::from_str(&s).ok());

            Ok(ResearchResult {
                id: row.get(0)?,
                todo_id: row.get(1)?,
                service: row.get(2)?,
                prompt: row.get(3)?,
                content: row.get(4)?,
                raw_html: row.get(5)?,
                metadata,
                started_at: row.get(7)?,
                completed_at: row.get(8)?,
                duration_seconds: row.get(9)?,
                status: string_to_research_status(row.get(10)?),
                created_at: row.get(11)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;

        Ok(results)
    }

    pub fn get_research_progress(&self, todo_id: &str, service: &str) -> Result<Option<ResearchProgress>> {
        let mut stmt = self.db.connection().prepare(
            r#"
            SELECT todo_id, service, stage, progress_percentage, current_step, 
                   estimated_remaining, updated_at
            FROM research_progress 
            WHERE todo_id = ? AND service = ?
            "#,
        )?;

        let progress = stmt.query_row(params![todo_id, service], |row| {
            Ok(ResearchProgress {
                todo_id: row.get(0)?,
                service: row.get(1)?,
                stage: string_to_research_stage(row.get(2)?),
                progress_percentage: row.get(3)?,
                current_step: row.get(4)?,
                estimated_remaining: row.get(5)?,
                updated_at: row.get(6)?,
            })
        }).optional()?;

        Ok(progress)
    }
}

fn stage_to_string(stage: &ResearchStage) -> String {
    match stage {
        ResearchStage::Searching => "searching".to_string(),
        ResearchStage::Analyzing => "analyzing".to_string(),
        ResearchStage::Synthesizing => "synthesizing".to_string(),
        ResearchStage::Completed => "completed".to_string(),
    }
}

fn string_to_research_stage(s: String) -> ResearchStage {
    match s.as_str() {
        "searching" => ResearchStage::Searching,
        "analyzing" => ResearchStage::Analyzing,
        "synthesizing" => ResearchStage::Synthesizing,
        "completed" => ResearchStage::Completed,
        _ => ResearchStage::Searching,
    }
}

fn status_to_string(status: &ResearchStatus) -> String {
    match status {
        ResearchStatus::Completed => "completed".to_string(),
        ResearchStatus::Failed => "failed".to_string(),
        ResearchStatus::Timeout => "timeout".to_string(),
    }
}

fn string_to_research_status(s: String) -> ResearchStatus {
    match s.as_str() {
        "completed" => ResearchStatus::Completed,
        "failed" => ResearchStatus::Failed,
        "timeout" => ResearchStatus::Timeout,
        _ => ResearchStatus::Failed,
    }
}