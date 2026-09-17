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

use commands::{get_day, create_task};

#[tauri::command]
fn greet(name: &str) -> String {
    let s = format!("Hello, {}! You greeted from Rust!", name);
    s
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_data_dir = app.path().data_dir()?;
            // println!("{}",app_data_dir.display());
            std::fs::create_dir_all(&app_data_dir)?;
            let db_path = app_data_dir.join("planner.db");
            let connection =
                initialize_database(&db_path).map_err(|e| tauri::Error::Anyhow(e.into()))?;
            let db = Database {
                connection: Mutex::new(connection),
            };
            app.manage(db);

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_day, create_task])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
