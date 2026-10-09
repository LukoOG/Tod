use rusqlite::{Result, Transaction, params};
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

use crate::db::Database;

#[derive(Debug, Serialize, Deserialize)]
pub struct Day {
    pub date: NaiveDate,
    pub mood: Option<i64>,
    pub reflection: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl Database {
    pub fn get_or_create_day(
        tx: &Transaction,
        date: &str,
    ) -> Result<Day, String> {
        // Create the row if it doesn't already exist.
        tx.execute(
            "
            INSERT INTO days (
                date,
                created_at,
                updated_at
            )
            VALUES (?1, ?2, ?2)
            ON CONFLICT(date) DO NOTHING
            ",
            params![
                date,
                chrono::Utc::now().to_rfc3339(),
            ],
        )
        .map_err(|e| format!("Failed to create day: {}", e))?;

        // Fetch the existing/newly-created row.
        tx.query_row(
            "
            SELECT
                date,
                mood,
                reflection,
                created_at,
                updated_at
            FROM days
            WHERE date = ?1
            ",
            params![date],
            |row| {
                let date_string: String = row.get(0)?;

                let date = NaiveDate::parse_from_str(
                    &date_string,
                    "%Y-%m-%d",
                )
                .map_err(|e| {
                    rusqlite::Error::FromSqlConversionFailure(
                        0,
                        rusqlite::types::Type::Text,
                        Box::new(e),
                    )
                })?;

                Ok(Day {
                    date,
                    mood: row.get(1)?,
                    reflection: row.get(2)?,
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                })
            },
        )
        .map_err(|e| format!("Failed to retrieve day: {}", e))
    }

    pub fn update_day_reflection(
        &self,
        date: NaiveDate,
        reflection: Option<String>,
        mood: Option<i64>,
    ) -> Result<Day, String> {
        let date_string = date.format("%Y-%m-%d").to_string();
        let conn = self.connection.lock().unwrap();

        // Upsert: create the day if it doesn't yet exist, then update reflection/mood.
        conn.execute(
            "
            INSERT INTO days (date, created_at, updated_at)
            VALUES (?1, datetime('now'), datetime('now'))
            ON CONFLICT(date) DO NOTHING
            ",
            params![date_string],
        )
        .map_err(|e| format!("Failed to ensure day exists: {}", e))?;

        conn.execute(
            "
            UPDATE days
            SET
                reflection = ?2,
                mood = ?3,
                updated_at = datetime('now')
            WHERE date = ?1
            ",
            params![date_string, reflection, mood],
        )
        .map_err(|e| format!("Failed to update day reflection: {}", e))?;

        conn.query_row(
            "
            SELECT
                date,
                mood,
                reflection,
                created_at,
                updated_at
            FROM days
            WHERE date = ?1
            ",
            params![date_string],
            |row| {
                let date_str: String = row.get(0)?;
                let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").map_err(|e| {
                    rusqlite::Error::FromSqlConversionFailure(
                        0,
                        rusqlite::types::Type::Text,
                        Box::new(e),
                    )
                })?;
                Ok(Day {
                    date,
                    mood: row.get(1)?,
                    reflection: row.get(2)?,
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                })
            },
        )
        .map_err(|e| format!("Failed to retrieve day after reflection update: {}", e))
    }
}