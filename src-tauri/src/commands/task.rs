use crate::db::Database;
use crate::models::task::Task;
use crate::views::CreateTaskInput;

#[tauri::command]
pub fn create_task(
    input: CreateTaskInput,
    state: tauri::State<'_, Database>,
) -> Result<Task, String> {
    let id = uuid::Uuid::new_v4().to_string();
    let task = Task::from(id, input.day_date, input.title);
    state.create_task(task)
}


#[tauri::command]
pub fn toggle_task(id: String, state: tauri::State<'_, Database>) -> Result<Task, String> {
    state.toggle_task(id)
}