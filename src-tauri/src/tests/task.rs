use chrono::NaiveDate;

use crate::test_utils::{fixtures::*, setup_test_db};

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Ensure the corresponding Day row exists before inserting a Task.
/// This satisfies the FK constraint `tasks.day_date → days.date`.
fn create_day(db: &crate::db::Database, date_str: &str) {
    let mut conn = db.connection.lock().unwrap();
    let tx = conn.transaction().unwrap();
    crate::db::Database::get_or_create_day(&tx, date_str)
        .expect("Failed to create day in test setup");
    tx.commit().unwrap();
}

// ── Task CRUD ─────────────────────────────────────────────────────────────────

#[test]
fn create_task_returns_input_task() {
    let db = setup_test_db();

    create_day(&db, "2026-09-14");

    let task = sample_task("2026-09-14", "task_1", "Write journal entry");

    let created = db.create_task(task.clone()).expect("Failed to create task");

    assert_eq!(created, task);
}

#[test]
fn create_task_persists_task_to_database() {
    let db = setup_test_db();

    create_day(&db, "2026-09-14");

    let task = sample_task("2026-09-14", "task_1", "Write journal entry");

    db.create_task(task.clone()).expect("Failed to create task");

    let tasks = db
        .get_tasks_for_day(task.day_date)
        .expect("Failed to get tasks");

    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0], task);
}

#[test]
fn create_task_fails_when_day_does_not_exist() {
    let db = setup_test_db();

    // No Day is created — FK should prevent insertion.
    let task = sample_task("2026-09-14", "task_orphan", "Orphaned task");

    let result = db.create_task(task);

    assert!(
        result.is_err(),
        "Expected an error when inserting a Task without a corresponding Day"
    );
}

#[test]
fn get_tasks_for_day_returns_empty_when_no_tasks_exist() {
    let db = setup_test_db();

    let date = NaiveDate::from_ymd_opt(2026, 9, 14).unwrap();

    let tasks = db
        .get_tasks_for_day(date)
        .expect("Failed to get tasks for day");

    assert!(tasks.is_empty());
}

#[test]
fn get_tasks_for_day_returns_all_tasks_for_that_day() {
    let db = setup_test_db();

    create_day(&db, "2026-09-14");

    let task_1 = sample_task("2026-09-14", "task_1", "Write journal entry");
    let task_2 = sample_task("2026-09-14", "task_2", "Read for 30 minutes");
    let task_3 = sample_task("2026-09-14", "task_3", "Plan tomorrow");

    db.create_task(task_1.clone()).expect("Failed to create task 1");
    db.create_task(task_2.clone()).expect("Failed to create task 2");
    db.create_task(task_3.clone()).expect("Failed to create task 3");

    let tasks = db
        .get_tasks_for_day(task_1.day_date)
        .expect("Failed to get tasks for day");

    assert_eq!(tasks.len(), 3);
    assert!(tasks.contains(&task_1));
    assert!(tasks.contains(&task_2));
    assert!(tasks.contains(&task_3));
}

#[test]
fn get_tasks_for_day_only_returns_tasks_for_requested_day() {
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
    let tasks = db
        .get_tasks_for_day(date)
        .expect("Failed to get tasks for day");

    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0], task_today);
}

#[test]
fn update_task_updates_all_mutable_fields() {
    let db = setup_test_db();

    create_day(&db, "2026-09-14");

    let original = sample_task("2026-09-14", "task_1", "Write journal entry");

    db.create_task(original.clone())
        .expect("Failed to create task");

    let updated = crate::models::Task {
        title: "Updated: Write journal entry".to_string(),
        completed: true,
        completed_at: Some("2026-09-14T22:00:00Z".to_string()),
        notes: Some("Wrote 3 pages".to_string()),
        updated_at: "2026-09-14T22:05:00Z".to_string(),
        ..original.clone()
    };

    let result = db
        .update_task(updated.clone())
        .expect("Failed to update task");

    assert_eq!(result, updated);

    // Verify persistence via a fresh query.
    let tasks = db
        .get_tasks_for_day(original.day_date)
        .expect("Failed to re-query tasks");

    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0], updated);
}

#[test]
fn update_task_does_not_modify_other_tasks() {
    let db = setup_test_db();

    create_day(&db, "2026-09-14");

    let task_1 = sample_task("2026-09-14", "task_1", "First task");
    let task_2 = sample_task("2026-09-14", "task_2", "Second task");

    db.create_task(task_1.clone()).expect("Failed to create task 1");
    db.create_task(task_2.clone()).expect("Failed to create task 2");

    let updated_task_1 = crate::models::Task {
        title: "Updated first task".to_string(),
        updated_at: "2026-09-14T10:00:00Z".to_string(),
        ..task_1.clone()
    };

    db.update_task(updated_task_1.clone())
        .expect("Failed to update task 1");

    let tasks = db
        .get_tasks_for_day(task_1.day_date)
        .expect("Failed to re-query tasks");

    assert_eq!(tasks.len(), 2);
    assert!(tasks.contains(&updated_task_1));
    assert!(tasks.contains(&task_2));
    assert!(!tasks.contains(&task_1));
}

#[test]
fn delete_task_removes_task_from_database() {
    let db = setup_test_db();

    create_day(&db, "2026-09-14");

    let task = sample_task("2026-09-14", "task_1", "Write journal entry");

    db.create_task(task.clone()).expect("Failed to create task");

    db.delete_task(task.id.clone())
        .expect("Failed to delete task");

    let tasks = db
        .get_tasks_for_day(task.day_date)
        .expect("Failed to re-query tasks");

    assert!(tasks.is_empty());
}

#[test]
fn delete_task_does_not_remove_other_tasks() {
    let db = setup_test_db();

    create_day(&db, "2026-09-14");

    let task_1 = sample_task("2026-09-14", "task_1", "First task");
    let task_2 = sample_task("2026-09-14", "task_2", "Second task");

    db.create_task(task_1.clone()).expect("Failed to create task 1");
    db.create_task(task_2.clone()).expect("Failed to create task 2");

    db.delete_task(task_1.id.clone())
        .expect("Failed to delete task 1");

    let tasks = db
        .get_tasks_for_day(task_1.day_date)
        .expect("Failed to re-query tasks");

    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0], task_2);
}
