use crate::{db::Database, models::Activity};

#[tauri::command]
pub fn complete_activity(id: String, state: tauri::State<'_, Database>) -> Result<Activity, String> {
    state.complete_activity(id)
}
