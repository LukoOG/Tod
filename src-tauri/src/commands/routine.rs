use rusqlite::params;

use crate::models::routine::Routine;
use crate::db::Database;

pub fn create_routine(routine: Routine, state: tauri::State<'_, Database>) -> Result<Routine, String> {
    let conn = state.connection.lock().unwrap();
    let mut result = conn.execute(
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

pub fn list_routines(state: tauri::State<'_, Database>) -> Result<Vec<Routine>, String >{
    let conn = state.connection.lock().unwrap();
    let mut stmt = conn.prepare("SELECT * FROM routines").map_err(|e| format!("Failed to prepare statement: {}", e))?;
    let routine_iter = stmt.query_map([], |row| {
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
    }).map_err(|e| format!("Failed to query routines: {}", e))?;

    let mut routines = Vec::new();
    for routine in routine_iter {
        routines.push(routine.map_err(|e| format!("Failed to map routine: {}", e))?);
    }
    Ok(routines)
}

pub fn update_routine(routine: Routine, state: tauri::State<'_, Database>) -> Result<Routine, String> {
    let conn = state.connection.lock().unwrap();
    let result = conn.execute(
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
    );
    match result {
        Ok(_) => Ok(routine),
        Err(e) => Err(format!("Failed to update routine: {}", e)),
    }
}

pub fn delete_routine(routine_id: String, state: tauri::State<'_, Database>) -> Result<(), String> {
    let conn = state.connection.lock().unwrap();
    let result = conn.execute(
        "
            DELETE FROM routines
            WHERE id = ?1
        ",
        params![routine_id],
    );
    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to delete routine: {}", e)),
    }
}