use chrono::NaiveDate;

use crate::{db::Database, domain::{DayView, generate_day}};

#[tauri::command]
pub fn get_day(date: NaiveDate, state: tauri::State<'_, Database>) -> Result<DayView, String> {
    generate_day(&state, date)
}
