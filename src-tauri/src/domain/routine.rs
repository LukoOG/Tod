use crate::views::{CreateRoutineInput, UpdateRoutineInput};

pub fn validate_create(input: &CreateRoutineInput) -> Result<(), String> {
    validate_fields(&input.title, &input.start_time, &input.duration_minutes, &input.days_of_week)
}

pub fn validate_update(input: &UpdateRoutineInput) -> Result<(), String> {
    validate_fields(&input.title, &input.start_time, &input.duration_minutes, &input.days_of_week)
}

fn validate_fields(
    title: &str,
    start_time: &Option<String>,
    duration_minutes: &Option<i64>,
    days_of_week: &[u8],
) -> Result<(), String> {
    if title.trim().is_empty() {
        return Err("title must not be empty".to_string());
    }
    if let Some(st) = start_time {
        validate_start_time(st)?;
    }
    if let Some(&d) = duration_minutes.as_ref() {
        if d <= 0 {
            return Err(format!(
                "duration_minutes must be a positive integer, got {}",
                d
            ));
        }
    }
    validate_days_of_week(days_of_week)
}

fn validate_start_time(s: &str) -> Result<(), String> {
    let parts: Vec<&str> = s.splitn(3, ':').collect();
    if parts.len() != 2 {
        return Err(format!(
            "start_time '{}' is invalid: expected HH:MM format",
            s
        ));
    }
    let h: u32 = parts[0]
        .parse()
        .map_err(|_| format!("start_time '{}' is invalid: hours must be 0-23", s))?;
    let m: u32 = parts[1]
        .parse()
        .map_err(|_| format!("start_time '{}' is invalid: minutes must be 0-59", s))?;
    if h > 23 {
        return Err(format!(
            "start_time '{}' is invalid: hours must be 0-23",
            s
        ));
    }
    if m > 59 {
        return Err(format!(
            "start_time '{}' is invalid: minutes must be 0-59",
            s
        ));
    }
    Ok(())
}

fn validate_days_of_week(days: &[u8]) -> Result<(), String> {
    if days.is_empty() {
        return Err("at least one weekday must be selected".to_string());
    }
    for &d in days {
        if !(1..=7).contains(&d) {
            return Err(format!(
                "weekday {} is invalid: must be 1 (Monday) through 7 (Sunday)",
                d
            ));
        }
    }
    let mut seen = std::collections::HashSet::new();
    for &d in days {
        if !seen.insert(d) {
            return Err(format!(
                "weekday {} appears more than once in days_of_week",
                d
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::views::{CreateRoutineInput, UpdateRoutineInput};

    fn make_create(
        title: &str,
        start_time: Option<&str>,
        duration_minutes: Option<i64>,
        days_of_week: Vec<u8>,
    ) -> CreateRoutineInput {
        CreateRoutineInput {
            title: title.to_string(),
            start_time: start_time.map(|s| s.to_string()),
            duration_minutes,
            days_of_week,
        }
    }

    fn make_update(
        title: &str,
        start_time: Option<&str>,
        duration_minutes: Option<i64>,
        days_of_week: Vec<u8>,
    ) -> UpdateRoutineInput {
        UpdateRoutineInput {
            id: "any".to_string(),
            title: title.to_string(),
            start_time: start_time.map(|s| s.to_string()),
            duration_minutes,
            days_of_week,
        }
    }

    // ── Title ─────────────────────────────────────────────────────────────────

    #[test]
    fn rejects_empty_title() {
        assert!(validate_create(&make_create("", None, None, vec![1])).is_err());
        assert!(validate_update(&make_update("", None, None, vec![1])).is_err());
    }

    #[test]
    fn rejects_whitespace_only_title() {
        assert!(validate_create(&make_create("   ", None, None, vec![1])).is_err());
        assert!(validate_update(&make_update("\t\n", None, None, vec![1])).is_err());
    }

    #[test]
    fn accepts_non_empty_title() {
        assert!(validate_create(&make_create("Morning run", None, None, vec![1])).is_ok());
    }

    // ── Start time ────────────────────────────────────────────────────────────

    #[test]
    fn accepts_none_start_time() {
        assert!(validate_create(&make_create("A", None, None, vec![1])).is_ok());
    }

    #[test]
    fn accepts_valid_start_times() {
        for t in &["00:00", "09:30", "12:00", "23:59"] {
            let r = validate_create(&make_create("A", Some(t), None, vec![1]));
            assert!(r.is_ok(), "Expected {} to be valid", t);
        }
    }

    #[test]
    fn rejects_start_time_missing_colon() {
        assert!(validate_create(&make_create("A", Some("0900"), None, vec![1])).is_err());
    }

    #[test]
    fn rejects_start_time_too_many_parts() {
        assert!(validate_create(&make_create("A", Some("09:30:00"), None, vec![1])).is_err());
    }

    #[test]
    fn rejects_start_time_invalid_hours() {
        assert!(validate_create(&make_create("A", Some("24:00"), None, vec![1])).is_err());
        assert!(validate_create(&make_create("A", Some("99:00"), None, vec![1])).is_err());
    }

    #[test]
    fn rejects_start_time_invalid_minutes() {
        assert!(validate_create(&make_create("A", Some("12:60"), None, vec![1])).is_err());
        assert!(validate_create(&make_create("A", Some("12:99"), None, vec![1])).is_err());
    }

    #[test]
    fn rejects_start_time_non_numeric() {
        assert!(validate_create(&make_create("A", Some("ab:cd"), None, vec![1])).is_err());
    }

    // ── Duration ──────────────────────────────────────────────────────────────

    #[test]
    fn accepts_none_duration() {
        assert!(validate_create(&make_create("A", None, None, vec![1])).is_ok());
    }

    #[test]
    fn accepts_positive_duration() {
        assert!(validate_create(&make_create("A", None, Some(1), vec![1])).is_ok());
        assert!(validate_create(&make_create("A", None, Some(90), vec![1])).is_ok());
    }

    #[test]
    fn rejects_zero_duration() {
        assert!(validate_create(&make_create("A", None, Some(0), vec![1])).is_err());
    }

    #[test]
    fn rejects_negative_duration() {
        assert!(validate_create(&make_create("A", None, Some(-1), vec![1])).is_err());
        assert!(validate_create(&make_create("A", None, Some(-100), vec![1])).is_err());
    }

    // ── Days of week ──────────────────────────────────────────────────────────

    #[test]
    fn rejects_empty_days_of_week() {
        assert!(validate_create(&make_create("A", None, None, vec![])).is_err());
    }

    #[test]
    fn rejects_weekday_zero() {
        assert!(validate_create(&make_create("A", None, None, vec![0])).is_err());
    }

    #[test]
    fn rejects_weekday_eight() {
        assert!(validate_create(&make_create("A", None, None, vec![8])).is_err());
    }

    #[test]
    fn rejects_duplicate_weekdays() {
        assert!(validate_create(&make_create("A", None, None, vec![1, 2, 1])).is_err());
        assert!(validate_create(&make_create("A", None, None, vec![3, 3])).is_err());
    }

    #[test]
    fn accepts_all_valid_weekdays() {
        assert!(validate_create(&make_create("A", None, None, vec![1, 2, 3, 4, 5, 6, 7])).is_ok());
    }

    #[test]
    fn accepts_single_weekday() {
        for d in 1u8..=7 {
            assert!(validate_create(&make_create("A", None, None, vec![d])).is_ok());
        }
    }
}
