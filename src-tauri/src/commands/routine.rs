use crate::db::Database;
use crate::domain::routine as domain;
use crate::models::routine::Routine;
use crate::views::{CreateRoutineInput, UpdateRoutineInput};

#[tauri::command]
pub fn list_routines(state: tauri::State<'_, Database>) -> Result<Vec<Routine>, String> {
    state.list_routines()
}

#[tauri::command]
pub fn create_routine(
    input: CreateRoutineInput,
    state: tauri::State<'_, Database>,
) -> Result<Routine, String> {
    domain::validate_create(&input)?;
    let routine: Routine = CreateRoutineInput {
        title: input.title.trim().to_string(),
        ..input
    }
    .into();
    state.create_routine(routine)
}

#[tauri::command]
pub fn update_routine(
    input: UpdateRoutineInput,
    state: tauri::State<'_, Database>,
) -> Result<Routine, String> {
    domain::validate_update(&input)?;
    let existing = state.get_routine_by_id(&input.id)?;
    let updated = Routine {
        id: existing.id,
        title: input.title.trim().to_string(),
        start_time: input.start_time,
        duration_minutes: input.duration_minutes,
        days_of_week: input.days_of_week,
        active: existing.active,
        created_at: existing.created_at,
        updated_at: chrono::Utc::now().to_rfc3339(),
    };
    state.update_routine(updated)
}

#[tauri::command]
pub fn set_routine_active(
    id: String,
    active: bool,
    state: tauri::State<'_, Database>,
) -> Result<Routine, String> {
    state.set_routine_active(&id, active)
}

#[tauri::command]
pub fn delete_routine(id: String, state: tauri::State<'_, Database>) -> Result<(), String> {
    state.delete_routine(id)
}
