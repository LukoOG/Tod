use chrono::{Datelike, NaiveDate};
use serde::Serialize;

use crate::{
    db::Database,
    models::{Activity, Day, Task},
};

#[derive(Debug, Serialize)]
pub struct DayView {
    pub day: Day,
    pub activities: Vec<Activity>,
    pub tasks: Vec<Task>,
}

pub fn generate_day(db: &Database, date: NaiveDate) -> Result<DayView, String> {
    let day;
    {
        let mut conn = db.connection.lock().unwrap();

        let tx = conn.transaction().map_err(|e| e.to_string())?;
        let date_string = date.format("%Y-%m-%d").to_string();

        day = Database::get_or_create_day(&tx, &date_string)?;
        let weekday = date.weekday().number_from_monday() as u8;

        let routines = db.get_routines_for_day(&tx, weekday)?;

        for routine in routines {
            let activity = Activity::from_routine(&routine, &day);
            Database::insert_activity_if_missing(&tx, &activity)?;
        }
        tx.commit().map_err(|e| e.to_string())?;
    }

    let activities = db.get_activities_for_day(date)?;
    let tasks = db.get_tasks_for_day(date)?;

    Ok(DayView {
        day,
        activities,
        tasks,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::db::migrations::run_migrations;
    use crate::models::{activity::Activity, day::Day, routine::Routine, task::Task};

    use chrono::NaiveDate;
    use rusqlite::Connection;
    use std::sync::Mutex;

    fn setup_test_db() -> Database {
        let mut conn = Connection::open_in_memory().expect("Failed to create in-memory database");

        run_migrations(&mut conn).expect("Failed to run migrations");

        Database {
            connection: Mutex::new(conn),
        }
    }

    fn sample_routine(id: &str, title: &str, days_of_week: Vec<u8>) -> Routine {
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

    fn insert_task(db: &Database, task: &Task) {
        let conn = db.connection.lock().unwrap();
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
            rusqlite::params![
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
        .expect("Failed to insert test task");
    }

    fn sample_task(date: &str, id: &str, title: &str) -> Task {
        let day_date = NaiveDate::parse_from_str(date, "%Y-%m-%d").expect("Failed to parse date");
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

    #[test]
    fn generate_day_creates_day_if_missing() {
        let db = setup_test_db();

        let date = NaiveDate::from_ymd_opt(2026, 9, 14).unwrap();

        let result = generate_day(&db, date).expect("Failed to generate day");

        assert_eq!(result.day.date, date);
    }

    #[test]
    fn generate_day_returns_existing_day() {
        let db = setup_test_db();

        let date = NaiveDate::from_ymd_opt(2026, 9, 14).unwrap();

        // First generation creates the day.
        generate_day(&db, date).expect("Failed to generate day");

        // Update the day directly so we can verify
        // generate_day doesn't replace it.
        {
            let conn = db.connection.lock().unwrap();

            conn.execute(
                "
                UPDATE days
                SET mood = ?1,
                    reflection = ?2
                WHERE date = ?3
                ",
                rusqlite::params![5, "Had a great day", "2026-09-14"],
            )
            .expect("Failed to update test day");
        }

        let result = generate_day(&db, date).expect("Failed to generate day");

        assert_eq!(result.day.mood, Some(5));
        assert_eq!(result.day.reflection, Some("Had a great day".to_string()));
    }

    #[test]
    fn generate_day_creates_activity_for_matching_routine() {
        let db = setup_test_db();

        // 2026-09-14 is Monday = 1
        let date = NaiveDate::from_ymd_opt(2026, 9, 14).unwrap();

        let routine = sample_routine("routine_1", "Morning Workout", vec![1]);

        db.create_routine(routine)
            .expect("Failed to create routine");

        let result = generate_day(&db, date).expect("Failed to generate day");

        assert_eq!(result.activities.len(), 1);

        let activity = &result.activities[0];

        assert_eq!(activity.day_date, date);
        assert_eq!(activity.title, "Morning Workout");
        assert_eq!(activity.start_time, Some("07:00".to_string()));
        assert_eq!(activity.duration_minutes, Some(45));
        assert!(!activity.completed);
        assert_eq!(activity.source_routine_id, Some("routine_1".to_string()));
    }

    #[test]
    fn generate_day_does_not_create_activity_for_non_matching_routine() {
        let db = setup_test_db();

        // Monday
        let date = NaiveDate::from_ymd_opt(2026, 9, 14).unwrap();

        // Tuesday only
        let routine = sample_routine("routine_1", "Evening Yoga", vec![2]);

        db.create_routine(routine)
            .expect("Failed to create routine");

        let result = generate_day(&db, date).expect("Failed to generate day");

        assert!(result.activities.is_empty());
    }

    #[test]
    fn generate_day_only_creates_activities_for_active_routines() {
        let db = setup_test_db();

        // Monday
        let date = NaiveDate::from_ymd_opt(2026, 9, 14).unwrap();

        let mut active_routine = sample_routine("routine_active", "Active Routine", vec![1]);

        let mut inactive_routine = sample_routine("routine_inactive", "Inactive Routine", vec![1]);

        inactive_routine.active = false;

        db.create_routine(active_routine)
            .expect("Failed to create active routine");

        db.create_routine(inactive_routine)
            .expect("Failed to create inactive routine");

        let result = generate_day(&db, date).expect("Failed to generate day");

        assert_eq!(result.activities.len(), 1);
        assert_eq!(
            result.activities[0].source_routine_id,
            Some("routine_active".to_string())
        );
    }

    #[test]
    fn generate_day_creates_activities_for_multiple_matching_routines() {
        let db = setup_test_db();

        // Monday
        let date = NaiveDate::from_ymd_opt(2026, 9, 14).unwrap();

        let routine_1 = sample_routine("routine_1", "Morning Workout", vec![1]);

        let routine_2 = sample_routine("routine_2", "Morning Reading", vec![1]);

        let routine_3 = sample_routine("routine_3", "Evening Walk", vec![1]);

        db.create_routine(routine_1)
            .expect("Failed to create routine");

        db.create_routine(routine_2)
            .expect("Failed to create routine");

        db.create_routine(routine_3)
            .expect("Failed to create routine");

        let result = generate_day(&db, date).expect("Failed to generate day");

        assert_eq!(result.activities.len(), 3);

        let routine_ids: Vec<Option<String>> = result
            .activities
            .iter()
            .map(|activity| activity.source_routine_id.clone())
            .collect();

        assert!(routine_ids.contains(&Some("routine_1".to_string())));
        assert!(routine_ids.contains(&Some("routine_2".to_string())));
        assert!(routine_ids.contains(&Some("routine_3".to_string())));
    }

    #[test]
    fn generate_day_does_not_duplicate_activities() {
        let db = setup_test_db();

        // Monday
        let date = NaiveDate::from_ymd_opt(2026, 9, 14).unwrap();

        let routine = sample_routine("routine_1", "Morning Workout", vec![1]);

        db.create_routine(routine)
            .expect("Failed to create routine");

        // First call creates activity.
        let first = generate_day(&db, date).expect("Failed to generate first day");

        assert_eq!(first.activities.len(), 1);

        let first_activity_id = first.activities[0].id.clone();

        // Second call should NOT create another activity.
        let second = generate_day(&db, date).expect("Failed to generate second day");

        assert_eq!(second.activities.len(), 1);

        assert_eq!(second.activities[0].id, first_activity_id);
    }

    #[test]
    fn generate_day_preserves_existing_activity_state() {
        let db = setup_test_db();

        // Monday
        let date = NaiveDate::from_ymd_opt(2026, 9, 14).unwrap();

        let routine = sample_routine("routine_1", "Morning Workout", vec![1]);

        db.create_routine(routine)
            .expect("Failed to create routine");

        // Generate activity.
        let first = generate_day(&db, date).expect("Failed to generate day");

        assert_eq!(first.activities.len(), 1);

        let activity_id = first.activities[0].id.clone();

        // Simulate the user completing the activity.
        {
            let conn = db.connection.lock().unwrap();

            conn.execute(
                "
                UPDATE activities
                SET completed = ?1,
                    completed_at = ?2,
                    notes = ?3
                WHERE id = ?4
                ",
                rusqlite::params![true, "2026-09-14T08:00:00Z", "Great workout", activity_id,],
            )
            .expect("Failed to update activity");
        }

        // Generate the day again.
        let result = generate_day(&db, date).expect("Failed to generate day");

        assert_eq!(result.activities.len(), 1);

        let activity = &result.activities[0];

        assert!(activity.completed);
        assert_eq!(
            activity.completed_at,
            Some("2026-09-14T08:00:00Z".to_string())
        );
        assert_eq!(activity.notes, Some("Great workout".to_string()));
    }

    #[test]
    fn generate_day_returns_tasks_for_date() {
        let db = setup_test_db();

        let date = "2026-09-14";

        let task = sample_task(date, "task_1", "Write journal entry");

        insert_task(&db, &task);

        let parsed_date = NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap();

        let result = generate_day(&db, parsed_date).expect("Failed to generate day");

        assert_eq!(result.tasks.len(), 1);
        assert_eq!(result.tasks[0], task);
    }

    #[test]
    fn generate_day_returns_only_tasks_for_requested_date() {
        let db = setup_test_db();

        let task_today = sample_task("2026-09-14", "task_today", "Today's task");

        let task_tomorrow = sample_task("2026-09-15", "task_tomorrow", "Tomorrow's task");

        insert_task(&db, &task_today);
        insert_task(&db, &task_tomorrow);

        let date = NaiveDate::from_ymd_opt(2026, 9, 14).unwrap();

        let result = generate_day(&db, date).expect("Failed to generate day");

        assert_eq!(result.tasks.len(), 1);
        assert_eq!(result.tasks[0], task_today);
    }

    #[test]
    fn generate_day_returns_multiple_tasks_for_date() {
        let db = setup_test_db();

        let task_1 = sample_task("2026-09-14", "task_1", "Write journal entry");

        let task_2 = sample_task("2026-09-14", "task_2", "Read for 30 minutes");

        let task_3 = sample_task("2026-09-14", "task_3", "Plan tomorrow");

        insert_task(&db, &task_1);
        insert_task(&db, &task_2);
        insert_task(&db, &task_3);

        let date = NaiveDate::from_ymd_opt(2026, 9, 14).unwrap();

        let result = generate_day(&db, date).expect("Failed to generate day");

        assert_eq!(result.tasks.len(), 3);

        assert!(result.tasks.contains(&task_1));
        assert!(result.tasks.contains(&task_2));
        assert!(result.tasks.contains(&task_3));
    }

    #[test]
    fn generate_day_returns_empty_tasks_when_none_exist() {
        let db = setup_test_db();

        let date = NaiveDate::from_ymd_opt(2026, 9, 14).unwrap();

        let result = generate_day(&db, date).expect("Failed to generate day");

        assert!(result.tasks.is_empty());
    }

    #[test]
    fn generate_day_returns_activities_and_tasks_together() {
        let db = setup_test_db();

        // Monday
        let date = NaiveDate::from_ymd_opt(2026, 9, 14).unwrap();

        let routine = sample_routine("routine_1", "Morning Workout", vec![1]);

        db.create_routine(routine)
            .expect("Failed to create routine");

        let task = sample_task("2026-09-14", "task_1", "Write journal entry");

        insert_task(&db, &task);

        let result = generate_day(&db, date).expect("Failed to generate day");

        assert_eq!(result.day.date, date);

        assert_eq!(result.activities.len(), 1);
        assert_eq!(result.activities[0].title, "Morning Workout");

        assert_eq!(result.tasks.len(), 1);
        assert_eq!(result.tasks[0], task);
    }
}
