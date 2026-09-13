use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Day {
    pub date: String,
    pub mood: Option<i64>,
    pub reflection: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}