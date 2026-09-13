mod migrations;

use rusqlite::{Connection, Result};
use std::path::Path;
use std::sync::Mutex;

pub struct Database {
    pub connection: Mutex<Connection>,
}

pub fn initialize_database(path: &Path) -> Result<Connection> {
    let mut conn = Connection::open(path)?;
    conn.execute_batch(
        "
        PRAGMA foreign_keys = ON;
        ",
    )?;
    migrations::run_migrations(&mut conn)?;
    Ok(conn)
}
