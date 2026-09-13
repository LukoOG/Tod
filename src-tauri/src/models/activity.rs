use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Activity {
    pub id: String,
    pub day_date: String,
    pub title: String,
    pub start_time: Option<String>,
    pub duration_minutes: Option<i64>,
    pub completed: bool,
    pub completed_at: Option<String>,
    pub notes: Option<String>,
    pub source_routine_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}