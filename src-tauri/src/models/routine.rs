use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Routine {
    pub id: String,
    pub title: String,
    pub start_time: Option<String>,
    pub duration_minutes: Option<i64>,
    pub days_of_week: String,
    pub active: bool,
    pub created_at: String,
    pub updated_at: String,
}