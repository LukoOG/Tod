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

    pub fn get_routines_for_day(&self, tx: &rusqlite::Transaction, weekday: u8) -> Result<Vec<Routine>, String> {
        let mut stmt = tx
            .prepare("SELECT * FROM routines WHERE ?1 IN (days_of_week)")
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

#[cfg(test)]
mod tests {
    use crate::models::routine::Routine;
    use crate::test_utils::setup_test_db;

    fn sample_routine() -> Routine {
        Routine {
            id: "routine_123".to_string(),
            title: "Morning Workout".to_string(),
            start_time: Some("07:00".to_string()),
            duration_minutes: Some(45),
            days_of_week: vec![1, 3, 5], // Monday, Wednesday, Friday
            active: true,
            created_at: "2026-09-13T12:00:00Z".to_string(),
            updated_at: "2026-09-13T12:00:00Z".to_string(),
        }
    }

    fn another_routine() -> Routine {
        Routine {
            id: "routine_456".to_string(),
            title: "Evening Yoga".to_string(),
            start_time: Some("18:30".to_string()),
            duration_minutes: Some(60),
            days_of_week: vec![2, 4],
            active: true,
            created_at: "2026-09-13T13:00:00Z".to_string(),
            updated_at: "2026-09-13T13:00:00Z".to_string(),
        }
    }

    #[test]
    fn list_routines_returns_empty_vec_for_empty_database() {
        let db = setup_test_db();

        let routines = db.list_routines().expect("Failed to list routines");

        assert!(routines.is_empty());
    }

    #[test]
    fn create_routine_returns_input_routine() {
        let db = setup_test_db();
        let routine = sample_routine();

        let created = db
            .create_routine(routine.clone())
            .expect("Failed to create routine");

        assert_eq!(created, routine);
    }

    #[test]
    fn create_routine_persists_routine_to_database() {
        let db = setup_test_db();
        let routine = sample_routine();

        db.create_routine(routine.clone())
            .expect("Failed to create routine");

        let routines = db.list_routines().expect("Failed to list routines");

        assert_eq!(routines.len(), 1);
        assert_eq!(routines[0], routine);
    }

    #[test]
    fn create_routine_persists_nullable_fields() {
        let db = setup_test_db();

        let routine = Routine {
            id: "routine_nullable".to_string(),
            title: "Flexible Routine".to_string(),
            start_time: None,
            duration_minutes: None,
            days_of_week: vec![6, 7], // Saturday, Sunday
            active: false,
            created_at: "2026-09-13T12:00:00Z".to_string(),
            updated_at: "2026-09-13T12:00:00Z".to_string(),
        };

        db.create_routine(routine.clone())
            .expect("Failed to create routine");

        let routines = db.list_routines().expect("Failed to list routines");

        assert_eq!(routines.len(), 1);
        assert_eq!(routines[0], routine);
    }

    #[test]
    fn list_routines_returns_multiple_routines() {
        let db = setup_test_db();

        let first = sample_routine();
        let second = another_routine();

        db.create_routine(first.clone())
            .expect("Failed to create first routine");

        db.create_routine(second.clone())
            .expect("Failed to create second routine");

        let routines = db.list_routines().expect("Failed to list routines");

        assert_eq!(routines.len(), 2);

        assert!(routines.contains(&first));
        assert!(routines.contains(&second));
    }

    #[test]
    fn update_routine_updates_all_mutable_fields() {
        let db = setup_test_db();

        let original = sample_routine();

        db.create_routine(original)
            .expect("Failed to create routine");

        let updated = Routine {
            id: "routine_123".to_string(),
            title: "Evening Yoga".to_string(),
            start_time: Some("19:30".to_string()),
            duration_minutes: Some(90),
            days_of_week: vec![2, 4, 6], // Tuesday, Thursday, Saturday
            active: false,
            created_at: "2026-09-13T12:00:00Z".to_string(),
            updated_at: "2026-09-13T15:00:00Z".to_string(),
        };

        let result = db
            .update_routine(updated.clone())
            .expect("Failed to update routine");

        assert_eq!(result, updated);

        let routines = db.list_routines().expect("Failed to list routines");

        assert_eq!(routines.len(), 1);
        assert_eq!(routines[0], updated);
    }

    #[test]
    fn update_routine_can_set_nullable_fields_to_none() {
        let db = setup_test_db();

        let original = sample_routine();

        db.create_routine(original)
            .expect("Failed to create routine");

        let updated = Routine {
            id: "routine_123".to_string(),
            title: "No Fixed Time".to_string(),
            start_time: None,
            duration_minutes: None,
            days_of_week: vec![1, 5],
            active: true,
            created_at: "2026-09-13T12:00:00Z".to_string(),
            updated_at: "2026-09-13T16:00:00Z".to_string(),
        };

        db.update_routine(updated.clone())
            .expect("Failed to update routine");

        let routines = db.list_routines().expect("Failed to list routines");

        assert_eq!(routines[0], updated);
    }

    #[test]
    fn update_routine_does_not_modify_other_routines() {
        let db = setup_test_db();

        let first = sample_routine();
        let second = another_routine();

        db.create_routine(first.clone())
            .expect("Failed to create first routine");

        db.create_routine(second.clone())
            .expect("Failed to create second routine");

        let updated_first = Routine {
            id: first.id.clone(),
            title: "Updated Workout".to_string(),
            start_time: Some("08:00".to_string()),
            duration_minutes: Some(30),
            days_of_week: vec![1, 5], // Monday, Friday
            active: false,
            created_at: first.created_at.clone(),
            updated_at: "2026-09-13T16:00:00Z".to_string(),
        };

        db.update_routine(updated_first.clone())
            .expect("Failed to update routine");

        let routines = db.list_routines().expect("Failed to list routines");

        assert_eq!(routines.len(), 2);

        assert!(routines.contains(&updated_first));
        assert!(routines.contains(&second));
        assert!(!routines.contains(&first));
    }

    #[test]
    fn delete_routine_removes_existing_routine() {
        let db = setup_test_db();

        let routine = sample_routine();

        db.create_routine(routine)
            .expect("Failed to create routine");

        db.delete_routine("routine_123".to_string())
            .expect("Failed to delete routine");

        let routines = db.list_routines().expect("Failed to list routines");

        assert!(routines.is_empty());
    }

    #[test]
    fn delete_routine_does_not_remove_other_routines() {
        let db = setup_test_db();

        let first = sample_routine();
        let second = another_routine();

        db.create_routine(first)
            .expect("Failed to create first routine");

        db.create_routine(second.clone())
            .expect("Failed to create second routine");

        db.delete_routine("routine_123".to_string())
            .expect("Failed to delete routine");

        let routines = db.list_routines().expect("Failed to list routines");

        assert_eq!(routines.len(), 1);
        assert_eq!(routines[0], second);
    }

    #[test]
    fn delete_nonexistent_routine_returns_ok() {
        let db = setup_test_db();

        let result = db.delete_routine("does_not_exist".to_string());

        assert!(result.is_err());

        let routines = db.list_routines().expect("Failed to list routines");

        assert!(routines.is_empty());
    }

    #[test]
    fn update_nonexistent_routine_returns_ok() {
        let db = setup_test_db();

        let routine = sample_routine();

        let result = db.update_routine(routine);

        assert!(result.is_err());

        let routines = db.list_routines().expect("Failed to list routines");

        assert!(routines.is_empty());
    }

    #[test]
    fn create_routine_rejects_duplicate_id() {
        let db = setup_test_db();

        let routine = sample_routine();

        db.create_routine(routine.clone())
            .expect("First insert should succeed");

        let result = db.create_routine(routine);

        assert!(result.is_err());

        let routines = db.list_routines().expect("Failed to list routines");

        assert_eq!(routines.len(), 1);
    }

    #[test]
    fn routines_are_isolated_by_id() {
        let db = setup_test_db();

        let first = sample_routine();
        let second = another_routine();

        db.create_routine(first.clone())
            .expect("Failed to create first routine");

        db.create_routine(second.clone())
            .expect("Failed to create second routine");

        db.delete_routine(first.id.clone())
            .expect("Failed to delete first routine");

        let routines = db.list_routines().expect("Failed to list routines");

        assert_eq!(routines.len(), 1);
        assert_eq!(routines[0].id, second.id);
        assert_eq!(routines[0], second);
    }

    #[test]
    fn full_routine_lifecycle() {
        let db = setup_test_db();

        // CREATE
        let original = sample_routine();

        let created = db
            .create_routine(original.clone())
            .expect("Failed to create routine");

        assert_eq!(created, original);

        // READ
        let routines = db.list_routines().expect("Failed to list routines");

        assert_eq!(routines, vec![original.clone()]);

        // UPDATE
        let updated = Routine {
            id: original.id.clone(),
            title: "Evening Yoga".to_string(),
            start_time: Some("18:00".to_string()),
            duration_minutes: Some(60),
            days_of_week: vec![2, 4],
            active: false,
            created_at: original.created_at.clone(),
            updated_at: "2026-09-13T18:00:00Z".to_string(),
        };

        db.update_routine(updated.clone())
            .expect("Failed to update routine");

        let routines = db.list_routines().expect("Failed to list routines");

        assert_eq!(routines, vec![updated.clone()]);

        // DELETE
        db.delete_routine(updated.id)
            .expect("Failed to delete routine");

        let routines = db.list_routines().expect("Failed to list routines");

        assert!(routines.is_empty());
    }
}
