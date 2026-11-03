use chrono::{Datelike, NaiveDate};

use crate::{db::Database, models::Activity, views::DayView};

/// `today` is the caller's notion of the current date. Activities are only
/// generated from routines when `date >= today`; past days are locked — the
/// `days` row is still created and existing data is still returned, but no new
/// activities are back-filled from the current routine set.
pub fn generate_day(db: &Database, date: NaiveDate, today: NaiveDate) -> Result<DayView, String> {
    let day;
    {
        let mut conn = db.connection.lock().unwrap();

        let tx = conn.transaction().map_err(|e| e.to_string())?;
        let date_string = date.format("%Y-%m-%d").to_string();

        day = Database::get_or_create_day(&tx, &date_string)?;

        if date >= today {
            if date > today {
                // Future day: wipe uncompleted routine-sourced activities so the
                // preview always reflects the current routine set.
                tx.execute(
                    "DELETE FROM activities
                     WHERE day_date = ?1 AND source_routine_id IS NOT NULL AND completed = 0",
                    rusqlite::params![date_string],
                )
                .map_err(|e| format!("Failed to clear future activities: {}", e))?;
            }
            let weekday = date.weekday().number_from_monday() as u8;
            let routines = db.get_routines_for_day(&tx, weekday)?;
            for routine in routines {
                let activity = Activity::from_routine(&routine, &day);
                Database::insert_activity_if_missing(&tx, &activity)?;
            }
        }

        tx.commit().map_err(|e| e.to_string())?;
    }

    let activities = db.get_activities_for_day(date)?;
    let tasks = db.get_tasks_for_day(date)?;

    Ok(DayView {
        day,
        activities,
        tasks,
    })
}
