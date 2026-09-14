use crate::models::Routine;
use rusqlite::{params, Connection};
use std::sync::Mutex;
pub struct Database {
    pub connection: Mutex<Connection>,
}