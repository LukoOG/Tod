use chrono::NaiveDate;
use rusqlite::{params, Transaction};
use serde::{Deserialize, Serialize};

use crate::{
    db::Database,
    models::{Day, Routine},
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Activity {
    pub id: String,
    pub day_date: NaiveDate,
    pub title: String,
    pub start_time: Option<String>,
    pub duration_minutes: Option<i64>,
    pub completed: bool,
    pub completed_at: Option<String>,
    pub notes: Option<String>,
    pub source_routine_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl Activity {
    pub fn from_routine(routine: &Routine, day: &Day) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            day_date: day.date,
            title: routine.title.clone(),
            start_time: routine.start_time.clone(),
            duration_minutes: routine.duration_minutes,
            completed: false,
            completed_at: None,
            notes: None,
            source_routine_id: Some(routine.id.clone()),
            created_at: now.clone(),
            updated_at: now,
        }
    }
}

impl Database {
    pub fn insert_activity_if_missing(
        tx: &Transaction,
        activity: &Activity,
    ) -> Result<Activity, String> {
        let day_date = activity.day_date.format("%Y-%m-%d").to_string();

        tx.execute(
            "
            INSERT INTO activities (
                id,
                day_date,
                title,
                start_time,
                duration_minutes,
                completed,
                completed_at,
                notes,
                source_routine_id,
                created_at,
                updated_at
            )
            VALUES (
                ?1, ?2, ?3, ?4, ?5,
                ?6, ?7, ?8, ?9, ?10, ?11
            )
            ON CONFLICT(day_date, source_routine_id) DO NOTHING
            ",
            params![
                activity.id,
                day_date,
                activity.title,
                activity.start_time,
                activity.duration_minutes,
                activity.completed,
                activity.completed_at,
                activity.notes,
                activity.source_routine_id,
                activity.created_at,
                activity.updated_at,
            ],
        )
        .map_err(|e| format!("Failed to create activity: {}", e))?;

        // Return the existing activity if it already existed,
        // otherwise return the activity we just inserted.
        let existing = tx
            .query_row(
                "
                SELECT
                    id,
                    day_date,
                    title,
                    start_time,
                    duration_minutes,
                    completed,
                    completed_at,
                    notes,
                    source_routine_id,
                    created_at,
                    updated_at
                FROM activities
                WHERE day_date = ?1
                  AND source_routine_id = ?2
                ",
                params![day_date, activity.source_routine_id,],
                |row| {
                    let date_string: String = row.get(1)?;

                    let day_date =
                        NaiveDate::parse_from_str(&date_string, "%Y-%m-%d").map_err(|e| {
                            rusqlite::Error::FromSqlConversionFailure(
                                1,
                                rusqlite::types::Type::Text,
                                Box::new(e),
                            )
                        })?;

                    Ok(Activity {
                        id: row.get(0)?,
                        day_date,
                        title: row.get(2)?,
                        start_time: row.get(3)?,
                        duration_minutes: row.get(4)?,
                        completed: row.get(5)?,
                        completed_at: row.get(6)?,
                        notes: row.get(7)?,
                        source_routine_id: row.get(8)?,
                        created_at: row.get(9)?,
                        updated_at: row.get(10)?,
                    })
                },
            )
            .map_err(|e| format!("Failed to retrieve activity: {}", e))?;

        Ok(existing)
    }

    pub fn update_activity(&self, activity: Activity) -> Result<Activity, String> {
        let conn = self.connection.lock().unwrap();
        let date_string = activity.day_date.format("%Y-%m-%d").to_string();

        let rows_affected = conn
            .execute(
                "
            UPDATE activities
            SET
                day_date = ?2,
                title = ?3,
                start_time = ?4,
                duration_minutes = ?5,
                completed = ?6,
                completed_at = ?7,
                notes = ?8,
                updated_at = ?9
            WHERE id = ?1
            ",
                params![
                    activity.id,
                    date_string,
                    activity.title,
                    activity.start_time,
                    activity.duration_minutes,
                    activity.completed,
                    activity.completed_at,
                    activity.notes,
                    activity.updated_at,
                ],
            )
            .map_err(|e| format!("Failed to update activity: {}", e))?;
        if rows_affected == 0 {
            return Err(format!("No activity found with id: {}", activity.id));
        }

        Ok(activity)
    }

    pub fn delete_activity(&self, activity_id: String) -> Result<(), String> {
        let conn = self.connection.lock().unwrap();

        let rows_affected = conn
            .execute(
                "
            DELETE FROM activities
            WHERE id = ?1
            ",
                params![activity_id],
            )
            .map_err(|e| format!("Failed to delete activity: {}", e))?;
        if rows_affected == 0 {
            return Err(format!("No activity found with id: {}", activity_id));
        }

        Ok(())
    }

    pub fn get_activities_for_day(
        // tx: &Transaction,
        &self,
        date: NaiveDate,
    ) -> Result<Vec<Activity>, String> {
        let conn = self.connection.lock().unwrap();
        let date_string = date.format("%Y-%m-%d").to_string();
        let mut stmt = conn
            .prepare(
                "
                SELECT
                    id,
                    day_date,
                    title,
                    start_time,
                    duration_minutes,
                    completed,
                    completed_at,
                    notes,
                    source_routine_id,
                    created_at,
                    updated_at
                FROM activities
                WHERE day_date = ?1
                ",
            )
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let activity_iter = stmt
            .query_map([date_string], |row| {
                let date_string: String = row.get(1)?;
                let day_date =
                    NaiveDate::parse_from_str(&date_string, "%Y-%m-%d").map_err(|e| {
                        rusqlite::Error::FromSqlConversionFailure(
                            1,
                            rusqlite::types::Type::Text,
                            Box::new(e),
                        )
                    })?;
                Ok(Activity {
                    id: row.get(0)?,
                    day_date,
                    title: row.get(2)?,
                    start_time: row.get(3)?,
                    duration_minutes: row.get(4)?,
                    completed: row.get(5)?,
                    completed_at: row.get(6)?,
                    notes: row.get(7)?,
                    source_routine_id: row.get(8)?,
                    created_at: row.get(9)?,
                    updated_at: row.get(10)?,
                })
            })
            .map_err(|e| format!("Failed to query activities for day {}: {}", date, e))?;

        let mut activities = Vec::new();
        for activity in activity_iter {
            activities.push(activity.map_err(|e| format!("Failed to map activity: {}", e))?);
        }

        Ok(activities)
    }
}
