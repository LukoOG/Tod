use chrono::{Datelike, NaiveDate};

use crate::{
    db::Database, models::Activity, views::DayView,
};

pub fn generate_day(db: &Database, date: NaiveDate) -> Result<DayView, String> {
    let day;
    {
        let mut conn = db.connection.lock().unwrap();

        let tx = conn.transaction().map_err(|e| e.to_string())?;
        let date_string = date.format("%Y-%m-%d").to_string();

        day = Database::get_or_create_day(&tx, &date_string)?;
        let weekday = date.weekday().number_from_monday() as u8;

        let routines = db.get_routines_for_day(&tx, weekday)?;

        for routine in routines {
            let activity = Activity::from_routine(&routine, &day);
            Database::insert_activity_if_missing(&tx, &activity)?;
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

