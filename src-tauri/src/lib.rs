// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;
mod db;
mod domain;
mod models;
mod views;

#[cfg(test)]
mod test_utils;

#[cfg(test)]
mod tests;

use db::{initialize_database, Database};
use std::sync::Mutex;
use tauri::Manager;

use commands::{
    complete_activity, create_task, get_day, get_platform, toggle_task, update_activity_notes,
    update_day_reflection, update_task_notes,
    list_routines, create_routine, update_routine, set_routine_active, delete_routine,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_data_dir = app.path().data_dir()?;
            std::fs::create_dir_all(&app_data_dir)?;
            let db_path = app_data_dir.join("planner.db");
            println!("{}", db_path.display());
            let connection =
                initialize_database(&db_path).map_err(|e| tauri::Error::Anyhow(e.into()))?;
            let db = Database {
                connection: Mutex::new(connection),
            };
            app.manage(db);

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_day,
            update_day_reflection,
            complete_activity,
            update_activity_notes,
            create_task,
            toggle_task,
            update_task_notes,
            list_routines,
            create_routine,
            update_routine,
            set_routine_active,
            delete_routine,
            get_platform,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
