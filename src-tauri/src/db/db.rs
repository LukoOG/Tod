use crate::models::Routine;
use rusqlite::{params, Connection};
use std::sync::Mutex;
pub struct Database {
    pub connection: Mutex<Connection>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::migrations::run_migrations;
    use crate::models::routine::Routine;
    use rusqlite::Connection;
    use std::sync::Mutex;

    fn setup_test_db() -> Database {
        let mut conn = Connection::open_in_memory().expect("Failed to create in-memory database");

        run_migrations(&mut conn).expect("Failed to run database migrations");

        Database {
            connection: Mutex::new(conn),
        }
    }

    fn sample_routine() -> Routine {
        Routine {
            id: "routine_123".to_string(),
            title: "Morning Workout".to_string(),
            start_time: Some("07:00".to_string()),
            duration_minutes: Some(45),
            days_of_week: "Mon,Wed,Fri".to_string(),
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
            days_of_week: "Tue,Thu".to_string(),
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
            days_of_week: "Sat,Sun".to_string(),
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
            days_of_week: "Tue,Thu,Sat".to_string(),
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
            days_of_week: "Mon,Fri".to_string(),
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
            days_of_week: "Mon,Fri".to_string(),
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
            days_of_week: "Tue,Thu".to_string(),
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
