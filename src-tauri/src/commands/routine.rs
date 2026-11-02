use crate::db::Database;
use crate::models::routine::Routine;

#[allow(dead_code)]
pub fn create_routine(
    routine: Routine,
    state: tauri::State<'_, Database>,
) -> Result<Routine, String> {
    state.create_routine(routine)
}

#[allow(dead_code)]
pub fn list_routines(state: tauri::State<'_, Database>) -> Result<Vec<Routine>, String> {
    state.list_routines()
}

#[allow(dead_code)]
pub fn update_routine(
    routine: Routine,
    state: tauri::State<'_, Database>,
) -> Result<Routine, String> {
    state.update_routine(routine)
}

#[allow(dead_code)]
pub fn delete_routine(routine_id: String, state: tauri::State<'_, Database>) -> Result<(), String> {
    state.delete_routine(routine_id)
}