use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTaskInput {
    pub day_date: NaiveDate,
    pub title: String,
}