use std::fmt::format;

use chrono::NaiveDate;
use rusqlite::params;
use serde::{Deserialize, Serialize};

use crate::db::Database;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Task {
    pub id: String,
    pub day_date: NaiveDate,
    pub title: String,
    pub completed: bool,
    pub completed_at: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl Task {
    pub fn from(id: String, day_date: NaiveDate, title: String) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Task {
            id,
            day_date,
            title,
            completed: false,
            completed_at: None,
            notes: None,
            created_at: now.clone(),
            updated_at: now,
        }
    }
}

impl Database {
    pub fn create_task(&self, task: Task) -> Result<Task, String> {
        let conn = self.connection.lock().unwrap();
        let date_string = task.day_date.format("%Y-%m-%d").to_string();

        conn.execute(
            "
            INSERT INTO tasks (
                id,
                day_date,
                title,
                completed,
                completed_at,
                notes,
                created_at,
                updated_at
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
            ",
            params![
                task.id,
                date_string,
                task.title,
                task.completed,
                task.completed_at,
                task.notes,
                task.created_at,
                task.updated_at,
            ],
        )
        .map_err(|e| format!("Failed to create task: {}", e))?;

        Ok(task)
    }
    pub fn update_task(&self, task: Task) -> Result<Task, String> {
        let conn = self.connection.lock().unwrap();
        let date_string = task.day_date.format("%Y-%m-%d").to_string();

        conn.execute(
            "
            UPDATE tasks
            SET
                day_date = ?2,
                title = ?3,
                completed = ?4,
                completed_at = ?5,
                notes = ?6,
                updated_at = ?7
            WHERE id = ?1
            ",
            params![
                task.id,
                date_string,
                task.title,
                task.completed,
                task.completed_at,
                task.notes,
                task.updated_at,
            ],
        )
        .map_err(|e| format!("Failed to update task: {}", e))?;

        Ok(task)
    }

    pub fn delete_task(&self, task_id: String) -> Result<(), String> {
        let conn = self.connection.lock().unwrap();

        conn.execute(
            "
            DELETE FROM tasks
            WHERE id = ?1
            ",
            params![task_id],
        )
        .map_err(|e| format!("Failed to delete task: {}", e))?;

        Ok(())
    }

    pub fn get_tasks_for_day(&self, date: NaiveDate) -> Result<Vec<Task>, String> {
        let conn = self.connection.lock().unwrap();
        let date_string = date.format("%Y-%m-%d").to_string();
        let mut stmt = conn
            .prepare(
                "
                SELECT
                    id,
                    day_date,
                    title,
                    completed,
                    completed_at,
                    notes,
                    created_at,
                    updated_at
                FROM tasks
                WHERE day_date = ?1
                ",
            )
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let task_iter = stmt
            .query_map([date_string], |row| {
                let date_string: String = row.get(1)?;
                let day_date =
                    NaiveDate::parse_from_str(&date_string, "%Y-%m-%d").map_err(|e| {
                        rusqlite::Error::FromSqlConversionFailure(
                            1,
                            rusqlite::types::Type::Text,
                            Box::new(e),
                        )
                    })?;
                Ok(Task {
                    id: row.get(0)?,
                    day_date,
                    title: row.get(2)?,
                    completed: row.get(3)?,
                    completed_at: row.get(4)?,
                    notes: row.get(5)?,
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
                })
            })
            .map_err(|e| format!("Failed to query tasks for day {}: {}", date, e))?;

        let mut tasks = Vec::new();
        for task in task_iter {
            tasks.push(task.map_err(|e| format!("Failed to map task row: {}", e))?);
        }

        Ok(tasks)
    }

    pub fn toggle_task(&self, id: String) -> Result<Task, String> {
        let conn = self.connection.lock().unwrap();

        let rows_affected = conn
            .execute(
                "
        UPDATE tasks
        SET
            completed = NOT completed,
            completed_at = CASE
                WHEN completed = 0 THEN datetime('now')
                ELSE NULL
            END,
            updated_at = datetime('now')
        WHERE id = ?1
        ",
                params![id],
            )
            .map_err(|e| format!("failed to toggle task: {}", e))?;

        if rows_affected == 0 {
            return Err("failed to update row: id not found".to_string());
        }

        let task = conn
            .query_row(
                "
            SELECT
                id,
                day_date,
                title,
                completed,
                completed_at,
                notes,
                created_at,
                updated_at
            FROM tasks
            WHERE id = ?1
            ",
                params![id],
                |row| {
                    let day_date: String = row.get(1)?;

                    Ok(Task {
                        id: row.get(0)?,
                        day_date: NaiveDate::parse_from_str(&day_date, "%Y-%m-%d").map_err(
                            |e| {
                                rusqlite::Error::FromSqlConversionFailure(
                                    1,
                                    rusqlite::types::Type::Text,
                                    Box::new(e),
                                )
                            },
                        )?,
                        title: row.get(2)?,
                        completed: row.get(3)?,
                        completed_at: row.get(4)?,
                        notes: row.get(5)?,
                        created_at: row.get(6)?,
                        updated_at: row.get(7)?,
                    })
                },
            )
            .map_err(|e| format!("failed to get task: {}", e))?;

        Ok(task)
    }
}
