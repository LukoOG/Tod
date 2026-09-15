use rusqlite::{Connection};
use std::sync::Mutex;
pub struct Database {
    pub connection: Mutex<Connection>,
}