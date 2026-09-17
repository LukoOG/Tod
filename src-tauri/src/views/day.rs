use serde::Serialize;

use crate::models::{Activity, Day, Task};

#[derive(Debug, Serialize)]
pub struct DayView {
    pub day: Day,
    pub activities: Vec<Activity>,
    pub tasks: Vec<Task>,
}
