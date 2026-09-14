use super::super::db::Database;
use rusqlite::params;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Routine {
    pub id: String,
    pub title: String,
    pub start_time: Option<String>,
    pub duration_minutes: Option<i64>,
    pub days_of_week: Vec<u8>,
    pub active: bool,
    pub created_at: String,
    pub updated_at: String,
}

//Routine Crud
impl Database {
    pub fn create_routine(&self, routine: Routine) -> Result<Routine, String> {
        let conn = self.connection.lock().unwrap();
        let result = conn.execute(
        "
            INSERT INTO routines (id, title, start_time, duration_minutes, days_of_week, active, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
        ",
        params![
            routine.id,
            routine.title,
            routine.start_time,
            routine.duration_minutes,
            routine.days_of_week,
            routine.active,
            routine.created_at,
            routine.updated_at
        ],
    );
        match result {
            Ok(_) => Ok(routine),
            Err(e) => Err(format!("Failed to create routine: {}", e)),
        }
    }

    pub fn list_routines(&self) -> Result<Vec<Routine>, String> {
        let conn = self.connection.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT * FROM routines")
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;
        let routine_iter = stmt
            .query_map([], |row| {
                Ok(Routine {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    start_time: row.get(2)?,
                    duration_minutes: row.get(3)?,
                    days_of_week: row.get(4)?,
                    active: row.get(5)?,
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
                })
            })
            .map_err(|e| format!("Failed to query routines: {}", e))?;

        let mut routines = Vec::new();
        for routine in routine_iter {
            routines.push(routine.map_err(|e| format!("Failed to map routine: {}", e))?);
        }
        Ok(routines)
    }

    pub fn update_routine(&self, routine: Routine) -> Result<Routine, String> {
        let conn = self.connection.lock().unwrap();
        let rows_affected = conn.execute(
        "
            UPDATE routines
            SET title = ?2, start_time = ?3, duration_minutes = ?4, days_of_week = ?5, active = ?6, updated_at = ?7
            WHERE id = ?1
        ",
        params![
            routine.id,
            routine.title,
            routine.start_time,
            routine.duration_minutes,
            routine.days_of_week,
            routine.active,
            routine.updated_at
        ],
    )
    .map_err(|e| format!("Failed to update routine: {}", e))?;
        if rows_affected == 0 {
            return Err(format!("No routine found with id: {}", routine.id));
        }
        Ok(routine)
    }

    pub fn delete_routine(&self, routine_id: String) -> Result<(), String> {
        let conn = self.connection.lock().unwrap();
        let rows_affected = conn
            .execute(
                "
            DELETE FROM routines
            WHERE id = ?1
        ",
                params![routine_id],
            )
            .map_err(|e| format!("Failed to update routine: {}", e))?;

        if rows_affected == 0 {
            return Err(format!("No routine found with id: {}", routine_id));
        }

        Ok(())
    }

    pub fn get_routines_for_day(
        &self,
        tx: &rusqlite::Transaction,
        weekday: u8,
    ) -> Result<Vec<Routine>, String> {
        let mut stmt = tx
            .prepare(
                "SELECT 
                        routines.id,
                        routines.title,
                        routines.start_time,
                        routines.duration_minutes,
                        routines.days_of_week,
                        routines.active,
                        routines.created_at,
                        routines.updated_at
                     FROM routines, json_each(days_of_week) 
                     WHERE json_each.value = ?1 AND routines.active = 1
                "
            )
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;
        let routine_iter = stmt
            .query_map(params![weekday], |row| {
                Ok(Routine {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    start_time: row.get(2)?,
                    duration_minutes: row.get(3)?,
                    days_of_week: row.get(4)?,
                    active: row.get(5)?,
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
                })
            })
            .map_err(|e| format!("Failed to query routines for day: {}", e))?;

        let mut routines = Vec::new();
        for routine in routine_iter {
            routines.push(routine.map_err(|e| format!("Failed to map routine: {}", e))?);
        }
        Ok(routines)
    }
}

