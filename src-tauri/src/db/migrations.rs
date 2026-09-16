use rusqlite::{Connection, Result};

const MIGRATIONS: &[(i32, &str)] = &[
    (1, include_str!("../../migrations/001_initial.sql")),
    (2, include_str!("../../migrations/002_seed_routines.sql")),
];

pub fn run_migrations(conn: &mut Connection) -> Result<()> {
    let current_version: i32 = conn.pragma_query_value(None, "user_version", |row| row.get(0))?;

    for &(version, sql) in MIGRATIONS {
        //prevents already applied migrations from being reapplied
        if version <= current_version {
            continue;
        }

        let tx = conn.transaction()?;
        tx.execute_batch(sql)?;
        tx.pragma_update(None, "user_version", &version)?;
        tx.commit()?;
    }
    Ok(())
}
