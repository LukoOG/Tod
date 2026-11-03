use chrono::NaiveDate;

use crate::{db::Database, domain::generate_day, models::Day, views::DayView};

#[tauri::command]
pub fn get_day(date: NaiveDate, state: tauri::State<'_, Database>) -> Result<DayView, String> {
    let today = chrono::Local::now().date_naive();
    generate_day(&state, date, today)
}

#[tauri::command]
pub fn update_day_reflection(
    date: NaiveDate,
    reflection: Option<String>,
    mood: Option<i64>,
    state: tauri::State<'_, Database>,
) -> Result<Day, String> {
    state.update_day_reflection(date, reflection, mood)
}
