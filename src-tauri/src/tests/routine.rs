use chrono::NaiveDate;

use crate::domain::generate_day;
use crate::models::Routine;
use crate::test_utils::{fixtures::*, setup_test_db};

// ── Routine CRUD ─────────────────────────────────────────────────────────────

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
fn update_nonexistent_routine_returns_error() {
    let db = setup_test_db();

    let routine = sample_routine();

    let result = db.update_routine(routine);

    assert!(result.is_err());

    let routines = db.list_routines().expect("Failed to list routines");

    assert!(routines.is_empty());
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
fn delete_nonexistent_routine_returns_error() {
    let db = setup_test_db();

    let result = db.delete_routine("does_not_exist".to_string());

    assert!(result.is_err());

    let routines = db.list_routines().expect("Failed to list routines");

    assert!(routines.is_empty());
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

// ── get_routines_for_day ──────────────────────────────────────────────────────

#[test]
fn get_routines_for_day_returns_matching_routines() {
    let db = setup_test_db();

    let monday_routine = named_routine("routine_mon", "Morning Workout", vec![1]);

    db.create_routine(monday_routine.clone())
        .expect("Failed to create routine");

    let mut conn = db.connection.lock().unwrap();
    let tx = conn.transaction().unwrap();

    let routines = db
        .get_routines_for_day(&tx, 1) // Monday
        .expect("Failed to get routines for day");

    assert_eq!(routines.len(), 1);
    assert_eq!(routines[0].id, monday_routine.id);
}

#[test]
fn get_routines_for_day_excludes_non_matching_weekdays() {
    let db = setup_test_db();

    // Routine is only for Tuesday (2)
    let tuesday_routine = named_routine("routine_tue", "Evening Yoga", vec![2]);

    db.create_routine(tuesday_routine)
        .expect("Failed to create routine");

    let mut conn = db.connection.lock().unwrap();
    let tx = conn.transaction().unwrap();

    let routines = db
        .get_routines_for_day(&tx, 1) // Monday — should not match
        .expect("Failed to get routines for day");

    assert!(routines.is_empty());
}

#[test]
fn get_routines_for_day_excludes_inactive_routines() {
    let db = setup_test_db();

    let mut inactive = named_routine("routine_inactive", "Inactive Routine", vec![1]);
    inactive.active = false;

    db.create_routine(inactive)
        .expect("Failed to create routine");

    let mut conn = db.connection.lock().unwrap();
    let tx = conn.transaction().unwrap();

    let routines = db
        .get_routines_for_day(&tx, 1) // Monday
        .expect("Failed to get routines for day");

    assert!(routines.is_empty());
}

#[test]
fn get_routines_for_day_returns_only_active_matching_routines() {
    let db = setup_test_db();

    let active = named_routine("routine_active", "Active Routine", vec![1]);
    let mut inactive = named_routine("routine_inactive", "Inactive Routine", vec![1]);
    inactive.active = false;

    db.create_routine(active.clone())
        .expect("Failed to create active routine");
    db.create_routine(inactive)
        .expect("Failed to create inactive routine");

    let mut conn = db.connection.lock().unwrap();
    let tx = conn.transaction().unwrap();

    let routines = db
        .get_routines_for_day(&tx, 1)
        .expect("Failed to get routines for day");

    assert_eq!(routines.len(), 1);
    assert_eq!(routines[0].id, active.id);
}

// ── set_routine_active ────────────────────────────────────────────────────────

#[test]
fn set_routine_active_deactivates_an_active_routine() {
    let db = setup_test_db();
    let routine = sample_routine();
    db.create_routine(routine).expect("Failed to create routine");

    let result = db
        .set_routine_active("routine_123", false)
        .expect("Failed to deactivate");

    assert!(!result.active);

    let from_db = db.list_routines().expect("list");
    assert!(!from_db[0].active);
}

#[test]
fn set_routine_active_activates_an_inactive_routine() {
    let db = setup_test_db();
    let mut routine = sample_routine();
    routine.active = false;
    db.create_routine(routine).expect("Failed to create routine");

    let result = db
        .set_routine_active("routine_123", true)
        .expect("Failed to activate");

    assert!(result.active);

    let from_db = db.list_routines().expect("list");
    assert!(from_db[0].active);
}

#[test]
fn set_routine_active_returns_error_for_missing_id() {
    let db = setup_test_db();

    let result = db.set_routine_active("no_such_id", false);

    assert!(result.is_err());
}

#[test]
fn set_routine_active_updates_updated_at() {
    let db = setup_test_db();
    let routine = sample_routine();
    let original_updated_at = routine.updated_at.clone();
    db.create_routine(routine).expect("Failed to create routine");

    let result = db
        .set_routine_active("routine_123", false)
        .expect("Failed to deactivate");

    assert_ne!(result.updated_at, original_updated_at);
}

#[test]
fn deactivated_routine_does_not_generate_activity() {
    let db = setup_test_db();
    // Monday
    let date = NaiveDate::from_ymd_opt(2026, 9, 14).unwrap();

    let routine = named_routine("routine_1", "Morning Run", vec![1]);
    db.create_routine(routine).expect("create");

    // Deactivate before the day is ever generated.
    db.set_routine_active("routine_1", false)
        .expect("deactivate");

    let day = generate_day(&db, date, date).expect("generate_day");

    assert!(day.activities.is_empty());
}

#[test]
fn reactivated_routine_resumes_activity_generation() {
    let db = setup_test_db();
    // Monday
    let date = NaiveDate::from_ymd_opt(2026, 9, 14).unwrap();

    let mut routine = named_routine("routine_1", "Morning Run", vec![1]);
    routine.active = false;
    db.create_routine(routine).expect("create inactive");

    // Inactive — no activity generated.
    let day = generate_day(&db, date, date).expect("generate_day (inactive)");
    assert!(day.activities.is_empty());

    // Reactivate.
    db.set_routine_active("routine_1", true)
        .expect("reactivate");

    // Same date — no prior activity exists for this routine+date, so one is created.
    let day = generate_day(&db, date, date).expect("generate_day (active)");
    assert_eq!(day.activities.len(), 1);
    assert_eq!(
        day.activities[0].source_routine_id,
        Some("routine_1".to_string())
    );
}

// ── Historical activity preservation ─────────────────────────────────────────

#[test]
fn editing_routine_does_not_modify_historical_activities() {
    let db = setup_test_db();
    // Monday
    let date = NaiveDate::from_ymd_opt(2026, 9, 14).unwrap();

    let routine = named_routine("routine_1", "Morning Workout", vec![1]);
    db.create_routine(routine.clone()).expect("create");

    // Generate the day to materialise the activity.
    let day = generate_day(&db, date, date).expect("generate_day");
    assert_eq!(day.activities.len(), 1);
    let activity_id = day.activities[0].id.clone();
    assert_eq!(day.activities[0].title, "Morning Workout");

    // Simulate the user completing the activity and adding a note.
    {
        let conn = db.connection.lock().unwrap();
        conn.execute(
            "UPDATE activities SET completed = 1, notes = 'Done!' WHERE id = ?1",
            rusqlite::params![&activity_id],
        )
        .expect("mark completed");
    }

    // Edit the routine — change title, days and time.
    let updated = Routine {
        id: "routine_1".to_string(),
        title: "Evening Run".to_string(),
        start_time: Some("18:00".to_string()),
        duration_minutes: Some(30),
        days_of_week: vec![2, 4],
        active: true,
        created_at: routine.created_at.clone(),
        updated_at: chrono::Utc::now().to_rfc3339(),
    };
    db.update_routine(updated).expect("update");

    // Historical activity must be completely unchanged.
    let conn = db.connection.lock().unwrap();
    let (title, completed, notes): (String, bool, Option<String>) = conn
        .query_row(
            "SELECT title, completed, notes FROM activities WHERE id = ?1",
            rusqlite::params![activity_id],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )
        .expect("fetch activity");

    assert_eq!(title, "Morning Workout");
    assert!(completed);
    assert_eq!(notes, Some("Done!".to_string()));
}

#[test]
fn deleting_routine_preserves_historical_activities_with_null_source() {
    let db = setup_test_db();
    // Monday
    let date = NaiveDate::from_ymd_opt(2026, 9, 14).unwrap();

    let routine = named_routine("routine_1", "Morning Workout", vec![1]);
    db.create_routine(routine).expect("create");

    let day = generate_day(&db, date, date).expect("generate_day");
    assert_eq!(day.activities.len(), 1);
    let activity_id = day.activities[0].id.clone();

    // Delete the routine.
    db.delete_routine("routine_1".to_string()).expect("delete");

    // The activity must still exist.
    let conn = db.connection.lock().unwrap();
    let (title, source_id): (String, Option<String>) = conn
        .query_row(
            "SELECT title, source_routine_id FROM activities WHERE id = ?1",
            rusqlite::params![activity_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .expect("activity must survive routine deletion");

    assert_eq!(title, "Morning Workout");
    // ON DELETE SET NULL: link is cleared, activity row is preserved.
    assert_eq!(source_id, None);
}

#[test]
fn get_routine_by_id_returns_correct_routine() {
    let db = setup_test_db();
    let routine = sample_routine();
    db.create_routine(routine.clone()).expect("create");

    let fetched = db.get_routine_by_id("routine_123").expect("get by id");
    assert_eq!(fetched, routine);
}

#[test]
fn get_routine_by_id_returns_error_for_missing_id() {
    let db = setup_test_db();
    assert!(db.get_routine_by_id("no_such_id").is_err());
}

// ── days_of_week serialization ────────────────────────────────────────────────

/// Rows written by the API are now stored as JSON TEXT. Verify they round-trip
/// correctly through all three read paths.
#[test]
fn days_of_week_api_roundtrip_reads_correct_values() {
    let db = setup_test_db();
    let routine = named_routine("r1", "API Round-trip", vec![1, 3, 5]);
    db.create_routine(routine).expect("create");

    // list_routines
    let listed = db.list_routines().expect("list");
    assert_eq!(listed[0].days_of_week, vec![1u8, 3, 5]);

    // get_routine_by_id
    let fetched = db.get_routine_by_id("r1").expect("get by id");
    assert_eq!(fetched.days_of_week, vec![1u8, 3, 5]);
}

/// The write side must store JSON TEXT (not a BLOB of raw byte values) so that
/// SQLite's `json_each(days_of_week)` works correctly in `get_routines_for_day`.
#[test]
fn create_routine_stores_days_of_week_as_json_text() {
    let db = setup_test_db();
    let routine = named_routine("r1", "Storage Check", vec![2, 4, 6]);
    db.create_routine(routine).expect("create");

    let raw: String = {
        let conn = db.connection.lock().unwrap();
        conn.query_row(
            "SELECT days_of_week FROM routines WHERE id = 'r1'",
            [],
            |row| row.get(0),
        )
        .expect("fetch raw column")
    };

    // Must be valid JSON parseable back to the original vec.
    let parsed: Vec<u8> = serde_json::from_str(&raw).expect("stored value must be JSON text");
    assert_eq!(parsed, vec![2u8, 4, 6]);
}

/// `X'5b312c335d'` is the legacy seed format: a BLOB whose bytes are the
/// ASCII characters of the JSON string `"[1,3]"`. The helper must decode it to
/// `[1, 3]` even though rusqlite surfaces it as a `Blob` value ref.
#[test]
fn days_of_week_legacy_blob_json_bytes_are_decoded_correctly() {
    let db = setup_test_db();
    {
        let conn = db.connection.lock().unwrap();
        // X'5b312c335d' == bytes [0x5b,0x31,0x2c,0x33,0x5d] == ASCII chars of "[1,3]"
        conn.execute(
            "INSERT INTO routines (id, title, start_time, duration_minutes, days_of_week, active, created_at, updated_at)
             VALUES ('legacy', 'Seed Row', NULL, NULL, X'5b312c335d', 1,
                     '2026-01-01T00:00:00Z', '2026-01-01T00:00:00Z')",
            [],
        )
        .expect("raw insert");
    }

    let listed = db.list_routines().expect("list");
    assert_eq!(listed[0].days_of_week, vec![1u8, 3]);

    let fetched = db.get_routine_by_id("legacy").expect("get by id");
    assert_eq!(fetched.days_of_week, vec![1u8, 3]);
}

/// After the fix, `create_routine` stores JSON TEXT so `json_each` can parse
/// it; `get_routines_for_day` must therefore find the routine on its scheduled days.
#[test]
fn get_routines_for_day_works_after_api_create() {
    let db = setup_test_db();
    // Weekdays 1 (Mon) and 3 (Wed)
    let routine = named_routine("r1", "MW Class", vec![1, 3]);
    db.create_routine(routine).expect("create");

    let date_mon = NaiveDate::from_ymd_opt(2026, 11, 2).unwrap(); // Monday
    let date_tue = NaiveDate::from_ymd_opt(2026, 11, 3).unwrap(); // Tuesday

    let day_mon = generate_day(&db, date_mon, date_mon).expect("generate monday");
    let day_tue = generate_day(&db, date_tue, date_tue).expect("generate tuesday");

    assert_eq!(day_mon.activities.len(), 1, "Monday should have 1 activity");
    assert_eq!(day_tue.activities.len(), 0, "Tuesday should have no activities");
}
