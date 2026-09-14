#[cfg(test)]
use std::sync::Mutex;
#[cfg(test)]
use crate::db::{Database, migrations::run_migrations};

#[cfg(test)]
pub fn setup_test_db() -> Database {
    use rusqlite::Connection;

    let mut conn = Connection::open_in_memory().unwrap();

    run_migrations(&mut conn).unwrap();

    Database {
        connection: Mutex::new(conn),
    }
}