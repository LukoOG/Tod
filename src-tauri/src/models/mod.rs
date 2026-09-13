use std::time::Duration;

enum ActivityType {
    Work,
}

enum Status {
    Completed,
}
pub struct Activity {
    name: String,
    r#type: ActivityType,
    planned_start: Duration,
    planned_end: Duration,
    actual_start: Duration,
    actual_end: Duration,
    status: Status,
    notes: Vec<String>,
}
