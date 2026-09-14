use chrono::NaiveDate;

use crate::domain::generate_day;
use crate::test_utils::{fixtures::*, setup_test_db};

// ── Day creation ──────────────────────────────────────────────────────────────

#[test]
fn generate_day_creates_day_if_missing() {
    let db = setup_test_db();

    let date = NaiveDate::from_ymd_opt(2026, 9, 14).unwrap();

    let result = generate_day(&db, date).expect("Failed to generate day");

    assert_eq!(result.day.date, date);
}

#[test]
fn generate_day_returns_existing_day_without_overwriting_it() {
    let db = setup_test_db();

    let date = NaiveDate::from_ymd_opt(2026, 9, 14).unwrap();

    // First generation creates the day.
    generate_day(&db, date).expect("Failed to generate day (first)");

    // Simulate user editing the day's metadata.
    {
        let conn = db.connection.lock().unwrap();
        conn.execute(
            "UPDATE days SET mood = ?1, reflection = ?2 WHERE date = ?3",
            rusqlite::params![5, "Had a great day", "2026-09-14"],
        )
        .expect("Failed to update test day");
    }

    // Second generation must return the existing day (not reset it).
    let result = generate_day(&db, date).expect("Failed to generate day (second)");

    assert_eq!(result.day.mood, Some(5));
    assert_eq!(
        result.day.reflection,
        Some("Had a great day".to_string())
    );
}

// ── Activity generation ───────────────────────────────────────────────────────

#[test]
fn generate_day_creates_activity_for_matching_routine() {
    let db = setup_test_db();

    // 2026-09-14 is Monday = weekday 1
    let date = NaiveDate::from_ymd_opt(2026, 9, 14).unwrap();

    let routine = named_routine("routine_1", "Morning Workout", vec![1]);
    db.create_routine(routine.clone())
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

    // Routine is Tuesday-only
    let routine = named_routine("routine_1", "Evening Yoga", vec![2]);
    db.create_routine(routine).expect("Failed to create routine");

    let result = generate_day(&db, date).expect("Failed to generate day");

    assert!(result.activities.is_empty());
}

#[test]
fn generate_day_does_not_create_activity_for_inactive_routine() {
    let db = setup_test_db();

    // Monday
    let date = NaiveDate::from_ymd_opt(2026, 9, 14).unwrap();

    let active_routine = named_routine("routine_active", "Active Routine", vec![1]);

    let mut inactive_routine = named_routine("routine_inactive", "Inactive Routine", vec![1]);
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

    let routine_1 = named_routine("routine_1", "Morning Workout", vec![1]);
    let routine_2 = named_routine("routine_2", "Morning Reading", vec![1]);
    let routine_3 = named_routine("routine_3", "Evening Walk", vec![1]);

    db.create_routine(routine_1).expect("Failed to create routine 1");
    db.create_routine(routine_2).expect("Failed to create routine 2");
    db.create_routine(routine_3).expect("Failed to create routine 3");

    let result = generate_day(&db, date).expect("Failed to generate day");

    assert_eq!(result.activities.len(), 3);

    let routine_ids: Vec<Option<String>> = result
        .activities
        .iter()
        .map(|a| a.source_routine_id.clone())
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

    let routine = named_routine("routine_1", "Morning Workout", vec![1]);
    db.create_routine(routine).expect("Failed to create routine");

    // First call creates the activity.
    let first = generate_day(&db, date).expect("Failed to generate day (first)");
    assert_eq!(first.activities.len(), 1);

    let first_activity_id = first.activities[0].id.clone();

    // Second call must NOT create a duplicate.
    let second = generate_day(&db, date).expect("Failed to generate day (second)");
    assert_eq!(second.activities.len(), 1);
    assert_eq!(second.activities[0].id, first_activity_id);
}

#[test]
fn generate_day_preserves_existing_activity_state() {
    let db = setup_test_db();

    // Monday
    let date = NaiveDate::from_ymd_opt(2026, 9, 14).unwrap();

    let routine = named_routine("routine_1", "Morning Workout", vec![1]);
    db.create_routine(routine).expect("Failed to create routine");

    // Generate activity.
    let first = generate_day(&db, date).expect("Failed to generate day (first)");
    assert_eq!(first.activities.len(), 1);

    let activity_id = first.activities[0].id.clone();

    // Simulate the user completing the activity.
    {
        let conn = db.connection.lock().unwrap();
        conn.execute(
            "UPDATE activities SET completed = ?1, completed_at = ?2, notes = ?3 WHERE id = ?4",
            rusqlite::params![true, "2026-09-14T08:00:00Z", "Great workout", activity_id],
        )
        .expect("Failed to update activity");
    }

    // Generate the day again — completion state must be preserved.
    let result = generate_day(&db, date).expect("Failed to generate day (second)");

    assert_eq!(result.activities.len(), 1);

    let activity = &result.activities[0];
    assert!(activity.completed);
    assert_eq!(
        activity.completed_at,
        Some("2026-09-14T08:00:00Z".to_string())
    );
    assert_eq!(activity.notes, Some("Great workout".to_string()));
}

// ── Task inclusion ────────────────────────────────────────────────────────────

/// Helper: insert a Day via the model layer (satisfies FK for tasks).
fn create_day(db: &crate::db::Database, date_str: &str) {
    let mut conn = db.connection.lock().unwrap();
    let tx = conn.transaction().unwrap();
    crate::db::Database::get_or_create_day(&tx, date_str)
        .expect("Failed to create day in test setup");
    tx.commit().unwrap();
}

#[test]
fn generate_day_returns_empty_tasks_when_none_exist() {
    let db = setup_test_db();

    let date = NaiveDate::from_ymd_opt(2026, 9, 14).unwrap();

    let result = generate_day(&db, date).expect("Failed to generate day");

    assert!(result.tasks.is_empty());
}

#[test]
fn generate_day_returns_existing_tasks_for_the_date() {
    let db = setup_test_db();

    let date_str = "2026-09-14";

    // Day must exist before a Task can be inserted.
    create_day(&db, date_str);

    let task = sample_task(date_str, "task_1", "Write journal entry");
    db.create_task(task.clone()).expect("Failed to create task");

    let parsed_date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap();
    let result = generate_day(&db, parsed_date).expect("Failed to generate day");

    assert_eq!(result.tasks.len(), 1);
    assert_eq!(result.tasks[0], task);
}

#[test]
fn generate_day_returns_only_tasks_for_requested_date() {
    let db = setup_test_db();

    create_day(&db, "2026-09-14");
    create_day(&db, "2026-09-15");

    let task_today = sample_task("2026-09-14", "task_today", "Today's task");
    let task_tomorrow = sample_task("2026-09-15", "task_tomorrow", "Tomorrow's task");

    db.create_task(task_today.clone())
        .expect("Failed to create today's task");
    db.create_task(task_tomorrow.clone())
        .expect("Failed to create tomorrow's task");

    let date = NaiveDate::from_ymd_opt(2026, 9, 14).unwrap();
    let result = generate_day(&db, date).expect("Failed to generate day");

    assert_eq!(result.tasks.len(), 1);
    assert_eq!(result.tasks[0], task_today);
}

#[test]
fn generate_day_returns_activities_and_tasks_together() {
    let db = setup_test_db();

    // Monday
    let date = NaiveDate::from_ymd_opt(2026, 9, 14).unwrap();
    let date_str = "2026-09-14";

    let routine = named_routine("routine_1", "Morning Workout", vec![1]);
    db.create_routine(routine).expect("Failed to create routine");

    // Day must exist before creating the task.
    create_day(&db, date_str);

    let task = sample_task(date_str, "task_1", "Write journal entry");
    db.create_task(task.clone()).expect("Failed to create task");

    let result = generate_day(&db, date).expect("Failed to generate day");

    assert_eq!(result.day.date, date);
    assert_eq!(result.activities.len(), 1);
    assert_eq!(result.activities[0].title, "Morning Workout");
    assert_eq!(result.tasks.len(), 1);
    assert_eq!(result.tasks[0], task);
}
