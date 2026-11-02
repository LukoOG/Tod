use chrono::NaiveDate;

use crate::domain::generate_day;
use crate::models::Activity;
use crate::test_utils::{fixtures::*, setup_test_db};

// ── Activity persistence ──────────────────────────────────────────────────────

#[test]
fn get_activities_for_day_returns_empty_for_new_day() {
    let db = setup_test_db();

    let date = NaiveDate::from_ymd_opt(2026, 9, 14).unwrap();

    let activities = db
        .get_activities_for_day(date)
        .expect("Failed to get activities");

    assert!(activities.is_empty());
}

#[test]
fn update_activity_updates_all_mutable_fields() {
    let db = setup_test_db();

    let date = NaiveDate::from_ymd_opt(2026, 9, 14).unwrap();

    // Create a day and a routine so generate_day produces an Activity.
    let routine = named_routine("routine_1", "Morning Workout", vec![1]); // Monday
    db.create_routine(routine.clone())
        .expect("Failed to create routine");

    let day_view = generate_day(&db, date).expect("Failed to generate day");
    assert_eq!(day_view.activities.len(), 1);

    let original = day_view.activities[0].clone();

    let updated = Activity {
        completed: true,
        completed_at: Some("2026-09-14T08:00:00Z".to_string()),
        notes: Some("Felt great".to_string()),
        updated_at: "2026-09-14T08:05:00Z".to_string(),
        ..original.clone()
    };

    let result = db
        .update_activity(updated.clone())
        .expect("Failed to update activity");

    assert_eq!(result, updated);

    // Verify persistence via a fresh query.
    let activities = db
        .get_activities_for_day(date)
        .expect("Failed to re-query activities");

    assert_eq!(activities.len(), 1);
    assert!(activities[0].completed);
    assert_eq!(
        activities[0].completed_at,
        Some("2026-09-14T08:00:00Z".to_string())
    );
    assert_eq!(activities[0].notes, Some("Felt great".to_string()));
}

#[test]
fn delete_activity_removes_activity_from_database() {
    let db = setup_test_db();

    let date = NaiveDate::from_ymd_opt(2026, 9, 14).unwrap();

    let routine = named_routine("routine_1", "Morning Workout", vec![1]);
    db.create_routine(routine)
        .expect("Failed to create routine");

    let day_view = generate_day(&db, date).expect("Failed to generate day");
    assert_eq!(day_view.activities.len(), 1);

    let activity_id = day_view.activities[0].id.clone();

    db.delete_activity(activity_id)
        .expect("Failed to delete activity");

    let activities = db
        .get_activities_for_day(date)
        .expect("Failed to re-query activities");

    assert!(activities.is_empty());
}

#[test]
fn delete_activity_does_not_remove_other_activities() {
    let db = setup_test_db();

    // Monday
    let date = NaiveDate::from_ymd_opt(2026, 9, 14).unwrap();

    let routine_1 = named_routine("routine_1", "Morning Workout", vec![1]);
    let routine_2 = named_routine("routine_2", "Morning Reading", vec![1]);

    db.create_routine(routine_1)
        .expect("Failed to create routine 1");
    db.create_routine(routine_2)
        .expect("Failed to create routine 2");

    let day_view = generate_day(&db, date).expect("Failed to generate day");
    assert_eq!(day_view.activities.len(), 2);

    // Delete only the first activity.
    db.delete_activity(day_view.activities[0].id.clone())
        .expect("Failed to delete activity");

    let activities = db
        .get_activities_for_day(date)
        .expect("Failed to re-query activities");

    assert_eq!(activities.len(), 1);
    assert_eq!(activities[0].id, day_view.activities[1].id);
}

#[test]
fn insert_activity_if_missing_does_not_overwrite_existing_activity() {
    let db = setup_test_db();

    // Monday
    let date = NaiveDate::from_ymd_opt(2026, 9, 14).unwrap();

    let routine = named_routine("routine_1", "Morning Workout", vec![1]);
    db.create_routine(routine.clone())
        .expect("Failed to create routine");

    // First generate: creates the activity.
    let first = generate_day(&db, date).expect("Failed to generate day (first)");
    let activity_id = first.activities[0].id.clone();

    // Manually mark activity as completed.
    {
        let conn = db.connection.lock().unwrap();
        conn.execute(
            "UPDATE activities SET completed = 1, completed_at = '2026-09-14T08:00:00Z' WHERE id = ?1",
            rusqlite::params![activity_id],
        )
        .expect("Failed to mark activity as completed");
    }

    // Second generate: should NOT overwrite the existing completed activity.
    let second = generate_day(&db, date).expect("Failed to generate day (second)");

    assert_eq!(second.activities.len(), 1);
    assert_eq!(second.activities[0].id, activity_id);
    assert!(second.activities[0].completed);
    assert_eq!(
        second.activities[0].completed_at,
        Some("2026-09-14T08:00:00Z".to_string())
    );
}

// ── Activity notes ────────────────────────────────────────────────────────────

#[test]
fn update_activity_notes_saves_notes() {
    let db = setup_test_db();

    let date = NaiveDate::from_ymd_opt(2026, 9, 14).unwrap();

    let routine = named_routine("routine_1", "Morning Workout", vec![1]);
    db.create_routine(routine)
        .expect("Failed to create routine");

    let day_view = generate_day(&db, date).expect("Failed to generate day");
    let activity_id = day_view.activities[0].id.clone();

    let result = db
        .update_activity_notes(activity_id.clone(), Some("Felt great".to_string()))
        .expect("Failed to update activity notes");

    assert_eq!(result.id, activity_id);
    assert_eq!(result.notes, Some("Felt great".to_string()));

    // Verify persistence.
    let activities = db
        .get_activities_for_day(date)
        .expect("Failed to re-query activities");
    assert_eq!(activities[0].notes, Some("Felt great".to_string()));
}

#[test]
fn update_activity_notes_overwrites_existing_notes() {
    let db = setup_test_db();

    let date = NaiveDate::from_ymd_opt(2026, 9, 14).unwrap();

    let routine = named_routine("routine_1", "Morning Workout", vec![1]);
    db.create_routine(routine)
        .expect("Failed to create routine");

    let day_view = generate_day(&db, date).expect("Failed to generate day");
    let activity_id = day_view.activities[0].id.clone();

    db.update_activity_notes(activity_id.clone(), Some("First note".to_string()))
        .expect("Failed to set initial notes");

    let result = db
        .update_activity_notes(activity_id.clone(), Some("Updated note".to_string()))
        .expect("Failed to update notes again");

    assert_eq!(result.notes, Some("Updated note".to_string()));

    let activities = db
        .get_activities_for_day(date)
        .expect("Failed to re-query activities");
    assert_eq!(activities[0].notes, Some("Updated note".to_string()));
}

#[test]
fn update_activity_notes_clears_notes_when_none() {
    let db = setup_test_db();

    let date = NaiveDate::from_ymd_opt(2026, 9, 14).unwrap();

    let routine = named_routine("routine_1", "Morning Workout", vec![1]);
    db.create_routine(routine)
        .expect("Failed to create routine");

    let day_view = generate_day(&db, date).expect("Failed to generate day");
    let activity_id = day_view.activities[0].id.clone();

    db.update_activity_notes(activity_id.clone(), Some("Some note".to_string()))
        .expect("Failed to set notes");

    let result = db
        .update_activity_notes(activity_id.clone(), None)
        .expect("Failed to clear notes");

    assert_eq!(result.notes, None);

    let activities = db
        .get_activities_for_day(date)
        .expect("Failed to re-query activities");
    assert_eq!(activities[0].notes, None);
}

#[test]
fn update_activity_notes_returns_error_for_missing_id() {
    let db = setup_test_db();

    let result = db.update_activity_notes("nonexistent-id".to_string(), Some("Note".to_string()));

    assert!(
        result.is_err(),
        "Expected an error when updating notes for a non-existent activity"
    );
    assert!(result.unwrap_err().contains("nonexistent-id"));
}
