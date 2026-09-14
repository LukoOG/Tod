#[cfg(test)]
use std::sync::Mutex;
#[cfg(test)]
use crate::db::{Database, migrations::run_migrations};

#[cfg(test)]
pub fn setup_test_db() -> Database {
    use rusqlite::Connection;

    let mut conn = Connection::open_in_memory().unwrap();

    // Enable foreign key enforcement to match production behaviour.
    conn.execute_batch("PRAGMA foreign_keys = ON;").unwrap();

    run_migrations(&mut conn).unwrap();

    Database {
        connection: Mutex::new(conn),
    }
}

#[cfg(test)]
pub mod fixtures {
    use chrono::NaiveDate;

    use crate::models::{Routine, Task};

    pub fn sample_routine() -> Routine {
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

    pub fn another_routine() -> Routine {
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

    pub fn named_routine(id: &str, title: &str, days_of_week: Vec<u8>) -> Routine {
        Routine {
            id: id.to_string(),
            title: title.to_string(),
            start_time: Some("07:00".to_string()),
            duration_minutes: Some(45),
            days_of_week,
            active: true,
            created_at: "2026-09-13T12:00:00Z".to_string(),
            updated_at: "2026-09-13T12:00:00Z".to_string(),
        }
    }

    pub fn sample_task(date: &str, id: &str, title: &str) -> Task {
        let day_date = NaiveDate::parse_from_str(date, "%Y-%m-%d")
            .expect("Failed to parse date in sample_task");
        Task {
            id: id.to_string(),
            day_date,
            title: title.to_string(),
            completed: false,
            completed_at: None,
            notes: None,
            created_at: "2026-09-13T12:00:00Z".to_string(),
            updated_at: "2026-09-13T12:00:00Z".to_string(),
        }
    }
}
