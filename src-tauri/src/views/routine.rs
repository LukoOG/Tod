use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateRoutineInput {
    pub title: String,
    pub start_time: Option<String>,
    pub duration_minutes: Option<i64>,
    pub days_of_week: Vec<u8>,
}

/// Fields the frontend supplies when editing an existing routine.
/// `id` identifies the record; the backend preserves `created_at` and `active`.
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateRoutineInput {
    pub id: String,
    pub title: String,
    pub start_time: Option<String>,
    pub duration_minutes: Option<i64>,
    pub days_of_week: Vec<u8>,
}
